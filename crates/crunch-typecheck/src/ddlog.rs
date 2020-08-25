use super::TypecheckDatabase;
use core::hash::Hash;
use crunch_shared::{
    tracing,
    trees::{
        hir::{
            BinaryOp as HirBinaryOp, Binding as HirBinding, Block as HirBlock, Expr as HirExpr,
            ExprKind as HirExprKind, FuncArg as HirFuncArg, Function as HirFunction, Integer,
            Item as HirItem, ItemPath as HirItemPath, Literal as HirLiteral,
            LiteralVal as HirLiteralVal, Match as HirMatch, MatchArm as HirMatchArm,
            Pattern as HirPattern, Stmt as HirStmt, Type as HirType, TypeId,
            TypeKind as HirTypeKind, Var as HirVar, VarDecl as HirVarDecl, Vis as HirVis,
        },
        Sided,
    },
    utils::HashMap,
};
use ddlog_types::{
    hir_BinOp as BinOp, hir_BinaryOp as BinaryOp, hir_Binding as Binding, hir_ExprId as ExprId,
    hir_ExprKind as ExprKind, hir_FuncArg as FuncArg, hir_Function as Function, hir_Item as Item,
    hir_ItemPath as ItemPath, hir_Literal as Literal, hir_Match as Match, hir_MatchArm as MatchArm,
    hir_Pattern as Pattern, hir_Stmt as Stmt, hir_TypeKind as TypeKind, hir_VarDecl as VarDecl,
    hir_Vis as Vis, internment_Intern as Interned, internment_intern as ddlog_intern,
    std_Vec as Vector, Expr, InputItems,
};
use ddlog_values::{relid2name, Relations, Value};
use differential_datalog::{
    ddval::{DDValConvert, DDValue},
    program::{RelId, Update},
    record::Record,
    DDlog, DeltaMap,
};
use typecheck_ddlog::api::HDDlog;

trait Visit<T> {
    type Output;

    fn visit(&mut self, data: &T) -> Self::Output;
}

pub struct DDlogEngine<'ctx> {
    variable_id: u64,
    variables: HashMap<HirVar, u64>,
    expression_id: u64,
    expressions: Vec<Expr>,
    db: &'ctx dyn TypecheckDatabase,
}

impl<'ctx> DDlogEngine<'ctx> {
    pub fn new(db: &'ctx dyn TypecheckDatabase) -> Self {
        crunch_shared::trace!("creating a new ddlog type engine");

        Self {
            variable_id: 0,
            variables: HashMap::default(),
            expression_id: 0,
            expressions: Vec::new(),
            db,
        }
    }

    #[crunch_shared::instrument(name = "type checking", skip(self, program, items))]
    pub fn walk(&mut self, program: &mut HDDlog, items: &[&HirItem<'_>]) -> Result<(), String> {
        crunch_shared::trace!("starting transaction");
        program.transaction_start()?;

        program.apply_valupdates(items.iter().copied().map(|item| {
            Update::Insert {
                relid: Relations::InputItems as RelId,
                v: Value::InputItems(InputItems {
                    item: self.visit(item),
                })
                .into_ddvalue(),
            }
        }))?;
        program.apply_valupdates(self.expressions.drain(..).map(|expr| Update::Insert {
            relid: Relations::Expr as RelId,
            v: Value::Expr(expr).into_ddvalue(),
        }))?;

        crunch_shared::trace!("committing transaction");
        let delta = program.transaction_commit_dump_changes()?;
        dump_delta(&delta);

        Ok(())
    }

    fn next_expression(&mut self) -> u64 {
        let expr = self.expression_id;
        self.expression_id += 1;
        expr
    }

    fn next_variable(&mut self) -> u64 {
        let variable = self.variable_id;
        self.variable_id += 1;
        variable
    }

    fn get_or_create_var(&mut self, var: HirVar) -> u64 {
        if let Some(&var) = self.variables.get(&var) {
            var
        } else {
            let id = self.next_variable();
            self.variables.insert(var, id);

            id
        }
    }

    fn intern<T>(&mut self, data: T) -> Interned<T>
    where
        T: Eq + Hash + Send + Sync + Clone + 'static,
    {
        ddlog_intern(&data)
    }
}

impl<'ctx> Visit<HirItem<'_>> for DDlogEngine<'ctx> {
    type Output = Item;

    #[crunch_shared::instrument(name = "item", skip(self, item))]
    fn visit(&mut self, item: &HirItem<'_>) -> Self::Output {
        match item {
            HirItem::Function(function) => {
                crunch_shared::trace!("item is a function, visiting");
                Item::hir_ItemFunc {
                    func: self.visit(function),
                }
            }

            HirItem::ExternFunc(_external_function) => {
                crunch_shared::trace!("item is an external function, visiting");
                // self.visit(external_function)
                todo!()
            }
        }
    }
}

impl<'ctx> Visit<HirFunction<'_>> for DDlogEngine<'ctx> {
    type Output = Function;

    #[crunch_shared::instrument(
        name = "function",
        skip(self, function),
        fields(name = ?function.name.to_string(self.db.context().strings())),
    )]
    fn visit(&mut self, function: &HirFunction<'_>) -> Self::Output {
        crunch_shared::trace!("visiting {} function arguments", function.args.len());
        let args = function.args.iter().map(|arg| self.visit(arg)).collect();

        Function {
            name: self.visit(&function.name),
            vis: self.visit(&function.vis),
            args: Vector { x: args },
            body: self.visit(&function.body),
            ret: self.visit(&function.ret),
        }
    }
}

impl<'ctx> Visit<HirFuncArg> for DDlogEngine<'ctx> {
    type Output = FuncArg;

    #[crunch_shared::instrument(
        name = "function argument",
        skip(self, arg),
        fields(name = ?arg.name.to_string(self.db.context().strings())),
    )]
    fn visit(&mut self, arg: &HirFuncArg) -> Self::Output {
        FuncArg {
            name: self.next_variable(),
            kind: self.visit(&arg.kind),
        }
    }
}

impl<'ctx> Visit<TypeId> for DDlogEngine<'ctx> {
    type Output = Interned<TypeKind>;

    #[crunch_shared::instrument(name = "type id", skip(self, id))]
    fn visit(&mut self, id: &TypeId) -> Self::Output {
        let ty = self
            .db
            .context()
            .get_hir_type(*id)
            .expect("got a hir type id that doesn't exist");

        self.visit(ty)
    }
}

impl<'ctx> Visit<HirType> for DDlogEngine<'ctx> {
    type Output = Interned<TypeKind>;

    #[crunch_shared::instrument(name = "type", skip(self, ty))]
    fn visit(&mut self, ty: &HirType) -> Self::Output {
        let ty = match ty.kind {
            HirTypeKind::Unknown => TypeKind::hir_Unknown,
            HirTypeKind::Integer { signed, width } => TypeKind::hir_Int {
                is_signed: signed.into(),
                width: width.into(),
            },
            HirTypeKind::Bool => TypeKind::hir_Bool,
            HirTypeKind::Unit => TypeKind::hir_Unit,
            HirTypeKind::Absurd => TypeKind::hir_Absurd,

            kind => {
                crunch_shared::warn!("unhandled type in ddlog: {:?}", kind);
                TypeKind::hir_Error
            }
        };

        self.intern(ty)
    }
}

impl<'ctx> Visit<HirItemPath> for DDlogEngine<'ctx> {
    type Output = ItemPath;

    #[crunch_shared::instrument(name = "item path", skip(self, path))]
    fn visit(&mut self, path: &HirItemPath) -> Self::Output {
        self.intern(path.iter().map(|s| s.as_u32()).collect::<Vec<_>>().into())
    }
}

impl<'ctx> Visit<HirVis> for DDlogEngine<'ctx> {
    type Output = Vis;

    #[crunch_shared::instrument(name = "visibility", skip(self, vis))]
    fn visit(&mut self, vis: &HirVis) -> Self::Output {
        match vis {
            HirVis::FileLocal => Vis::hir_FileLocal,
            HirVis::Package => Vis::hir_Package,
            HirVis::Exposed => Vis::hir_Exposed,
        }
    }
}

impl<'ctx> Visit<HirStmt<'_>> for DDlogEngine<'ctx> {
    type Output = Stmt;

    #[crunch_shared::instrument(name = "statement", skip(self, stmt))]
    fn visit(&mut self, stmt: &HirStmt<'_>) -> Self::Output {
        match stmt {
            &HirStmt::Item(item) => Stmt::hir_StmtItem {
                item: self.visit(item),
            },

            &HirStmt::Expr(expr) => Stmt::hir_StmtExpr {
                expr: self.visit(expr),
            },

            HirStmt::VarDecl(var_decl) => Stmt::hir_StmtDecl {
                decl: self.visit(var_decl),
            },
        }
    }
}

impl<'ctx> Visit<HirVarDecl<'_>> for DDlogEngine<'ctx> {
    type Output = VarDecl;

    #[crunch_shared::instrument(name = "variable declaration", skip(self, var_decl))]
    fn visit(&mut self, var_decl: &HirVarDecl<'_>) -> Self::Output {
        VarDecl {
            name: self.get_or_create_var(var_decl.name),
            value: self.visit(var_decl.value),
            mutable: var_decl.mutable,
            ty: self.visit(&var_decl.ty),
        }
    }
}

impl<'ctx> Visit<HirExpr<'_>> for DDlogEngine<'ctx> {
    type Output = ExprId;

    #[crunch_shared::instrument(name = "expression", skip(self, expr))]
    fn visit(&mut self, expr: &HirExpr<'_>) -> Self::Output {
        let id = self.next_expression();

        let (kind, ty) = match &expr.kind {
            HirExprKind::Literal(literal) => {
                let (literal, ty) = self.visit(literal);
                (
                    ExprKind::hir_ExprLit {
                        lit: self.intern(literal),
                    },
                    Some(ty),
                )
            }

            &HirExprKind::Variable(var, ty) => {
                let expr = ExprKind::hir_ExprVar {
                    variable: self.get_or_create_var(var),
                };
                let ty = self.visit(&ty);

                (expr, Some(ty))
            }

            &HirExprKind::Assign(var, rhs) => {
                let rhs = self.visit(rhs);
                let variable = self
                    .variables
                    .get(&var)
                    .copied()
                    .expect("got a variable that doesn't exist");

                let expr = ExprKind::hir_ExprAssign {
                    variable,
                    expr_id: rhs,
                };

                (expr, Some(self.intern(TypeKind::hir_Unit)))
            }

            HirExprKind::Match(match_) => {
                let match_ = self.visit(match_);
                let ty = match_.ty.clone();

                (ExprKind::hir_ExprMatch { match_ }, Some(ty))
            }

            HirExprKind::Scope(block) => (
                ExprKind::hir_ExprScope {
                    block: self.visit(block),
                },
                None,
            ),

            HirExprKind::Return(ret) => (
                ExprKind::hir_ExprReturn {
                    val: self.visit(&ret.val).into(),
                },
                Some(self.intern(TypeKind::hir_Absurd)),
            ),

            HirExprKind::BinOp(binary_op) => (
                ExprKind::hir_ExprBinOp {
                    op: self.visit(binary_op),
                },
                None,
            ),

            HirExprKind::Loop(_) => todo!(),
            HirExprKind::Continue => todo!(),
            HirExprKind::Break(_) => todo!(),
            HirExprKind::FnCall(_) => todo!(),
            HirExprKind::Comparison(_) => todo!(),
            HirExprKind::Cast(_) => todo!(),
            HirExprKind::Reference(_) => todo!(),
            HirExprKind::Index { var: _, index: _ } => todo!(),
        };
        let kind = self.intern(kind);
        let ty = ty.unwrap_or_else(|| self.intern(TypeKind::hir_Unknown));

        self.expressions.push(Expr { id, kind, ty });
        id
    }
}

impl<'ctx> Visit<HirLiteral> for DDlogEngine<'ctx> {
    type Output = (Literal, Interned<TypeKind>);

    #[crunch_shared::instrument(name = "literal", skip(self, lit))]
    fn visit(&mut self, lit: &HirLiteral) -> Self::Output {
        let val = self.visit(&lit.val);
        let ty = self
            .db
            .context()
            .get_hir_type(lit.ty)
            .expect("attempted to get a type that doesn't exist");

        (val, self.visit(ty))
    }
}

impl<'ctx> Visit<HirLiteralVal> for DDlogEngine<'ctx> {
    type Output = Literal;

    #[crunch_shared::instrument(name = "literal value", skip(self, val))]
    fn visit(&mut self, val: &HirLiteralVal) -> Self::Output {
        match val {
            &HirLiteralVal::Integer(Integer { bits: int, .. }) => {
                Literal::hir_Integer { int: int as u64 }
            }
            &HirLiteralVal::Bool(boolean) => Literal::hir_Boolean { boolean },
            HirLiteralVal::String(string) => Literal::hir_String {
                r#str: string.to_string(),
            },
            HirLiteralVal::Rune(_) => todo!(),
            HirLiteralVal::Float(_) => todo!(),
            HirLiteralVal::Array { elements: _ } => todo!(),
        }
    }
}

impl<'ctx> Visit<HirMatch<'_>> for DDlogEngine<'ctx> {
    type Output = Match;

    #[crunch_shared::instrument(name = "match", skip(self, match_))]
    fn visit(&mut self, match_: &HirMatch<'_>) -> Self::Output {
        let arms = match_
            .arms
            .iter()
            .map(|arm| self.visit(arm))
            .collect::<Vec<_>>()
            .into();

        Match {
            cond: self.visit(match_.cond),
            arms,
            ty: self.visit(&match_.ty),
        }
    }
}

impl<'ctx> Visit<HirMatchArm<'_>> for DDlogEngine<'ctx> {
    type Output = MatchArm;

    #[crunch_shared::instrument(name = "match", skip(self, arm))]
    fn visit(&mut self, arm: &HirMatchArm<'_>) -> Self::Output {
        MatchArm {
            bind: self.visit(&arm.bind),
            guard: self.visit(&arm.guard).into(),
            body: self.visit(&arm.body),
            ty: self.visit(&arm.ty),
        }
    }
}

impl<'ctx> Visit<HirBlock<&HirStmt<'_>>> for DDlogEngine<'ctx> {
    type Output = Interned<Stmt>;

    #[crunch_shared::instrument(name = "statement block", skip(self, block))]
    fn visit(&mut self, block: &HirBlock<&HirStmt<'_>>) -> Self::Output {
        crunch_shared::trace!("visiting {} block statements", block.len());

        block
            .iter()
            .copied()
            .rev()
            .fold(None, |prev, next| {
                let next = {
                    let next = self.visit(next);
                    self.intern(next)
                };
                match prev {
                    Some(prev) => Some(self.intern(Stmt::hir_StmtSeq {
                        first: prev,
                        second: next,
                    })),
                    None => Some(next),
                }
            })
            .unwrap_or_else(|| self.intern(Stmt::hir_Empty))
    }
}

impl<'ctx> Visit<HirBinding> for DDlogEngine<'ctx> {
    type Output = Binding;

    #[crunch_shared::instrument(name = "binding", skip(self, binding))]
    fn visit(&mut self, binding: &HirBinding) -> Self::Output {
        let &HirBinding {
            reference,
            mutable,
            ref pattern,
            ref ty,
        } = binding;

        Binding {
            reference,
            mutable,
            pattern: self.visit(pattern),
            ty: self.visit(ty).into(),
        }
    }
}

impl<'ctx> Visit<HirPattern> for DDlogEngine<'ctx> {
    type Output = Pattern;

    #[crunch_shared::instrument(name = "pattern", skip(self, pattern))]
    fn visit(&mut self, pattern: &HirPattern) -> Self::Output {
        match pattern {
            HirPattern::Literal(literal) => {
                let (lit, ty) = self.visit(literal);
                Pattern::hir_PatLit { lit, ty }
            }
            HirPattern::Ident(_) | HirPattern::ItemPath(_) | HirPattern::Wildcard => todo!(),
        }
    }
}

impl<'ctx> Visit<Sided<HirBinaryOp, &HirExpr<'_>>> for DDlogEngine<'ctx> {
    type Output = BinaryOp;

    #[crunch_shared::instrument(name = "binary operation", skip(self, binary_op))]
    fn visit(&mut self, binary_op: &Sided<HirBinaryOp, &HirExpr<'_>>) -> Self::Output {
        BinaryOp {
            lhs: self.visit(binary_op.lhs),
            op: self.visit(&binary_op.op),
            rhs: self.visit(binary_op.rhs),
        }
    }
}

impl<'ctx> Visit<HirBinaryOp> for DDlogEngine<'ctx> {
    type Output = BinOp;

    #[crunch_shared::instrument(name = "binary operand", skip(self, op))]
    fn visit(&mut self, op: &HirBinaryOp) -> Self::Output {
        match op {
            HirBinaryOp::Mult => BinOp::hir_Mult,
            HirBinaryOp::Div => BinOp::hir_Div,
            HirBinaryOp::Add => BinOp::hir_Add,
            HirBinaryOp::Sub => BinOp::hir_Sub,
            HirBinaryOp::Mod => BinOp::hir_Mod,
            HirBinaryOp::Pow => BinOp::hir_Pow,
            HirBinaryOp::BitAnd => BinOp::hir_BitAnd,
            HirBinaryOp::BitOr => BinOp::hir_BitOr,
            HirBinaryOp::BitXor => BinOp::hir_BitXor,
            HirBinaryOp::Shl => BinOp::hir_Shl,
            HirBinaryOp::Shr => BinOp::hir_Shr,
        }
    }
}

impl<'ctx, T> Visit<&'_ T> for DDlogEngine<'ctx>
where
    DDlogEngine<'ctx>: Visit<T>,
{
    type Output = <DDlogEngine<'ctx> as Visit<T>>::Output;

    #[crunch_shared::instrument(name = "reference", skip(self, reference))]
    fn visit(&mut self, reference: &&'_ T) -> Self::Output {
        self.visit(*reference)
    }
}

impl<'ctx, T> Visit<Option<T>> for DDlogEngine<'ctx>
where
    DDlogEngine<'ctx>: Visit<T>,
{
    type Output = Option<<DDlogEngine<'ctx> as Visit<T>>::Output>;

    #[crunch_shared::instrument(name = "option", skip(self, option))]
    fn visit(&mut self, option: &Option<T>) -> Self::Output {
        option.as_ref().map(|val| self.visit(val))
    }
}

#[test]
fn ddlog_test() {
    run_ddlog().unwrap();
}

pub fn ddlog_callback(_rel: usize, _rec: &Record, _w: isize) {
    /* Obsolete, will be removed in later ddlog releases */
}
pub const DDLOG_WORKER_THREADS: usize = 12;
// Used for `HDDlog::dump_table()`
pub const DDLOG_TRACK_SNAPSHOTS: bool = false;

#[cfg(test)]
fn run_ddlog() -> Result<(), String> {
    let (mut program, init_state) =
        HDDlog::run(DDLOG_WORKER_THREADS, DDLOG_TRACK_SNAPSHOTS, ddlog_callback)?;

    println!("Initial ddlog state:");
    dump_delta(&init_state);

    program.transaction_start()?;
    let updates = vec![
        Update::Insert {
            relid: Relations::Expr as RelId,
            v: Value::Expr(Expr {
                id: 0,
                kind: ddlog_intern(&ExprKind::hir_ExprLit {
                    lit: ddlog_intern(&Literal::hir_Integer { int: 10 }),
                }),
                ty: ddlog_intern(&TypeKind::hir_Unknown),
            })
            .into_ddvalue(),
        },
        Update::Insert {
            relid: Relations::Expr as RelId,
            v: Value::Expr(Expr {
                id: 1,
                kind: ddlog_intern(&ExprKind::hir_ExprLit {
                    lit: ddlog_intern(&Literal::hir_Integer { int: 10 }),
                }),
                ty: ddlog_intern(&TypeKind::hir_Unknown),
            })
            .into_ddvalue(),
        },
        Update::Insert {
            relid: Relations::Expr as RelId,
            v: Value::Expr(Expr {
                id: 2,
                kind: ddlog_intern(&ExprKind::hir_ExprVar { variable: 0 }),
                ty: ddlog_intern(&TypeKind::hir_Unknown),
            })
            .into_ddvalue(),
        },
        Update::Insert {
            relid: Relations::InputItems as RelId,
            v: Value::InputItems(InputItems {
                item: Item::hir_ItemFunc {
                    func: Function {
                        name: ddlog_intern(&Vector { x: vec![0u32] }),
                        vis: Vis::hir_FileLocal,
                        args: Vector {
                            x: vec![FuncArg {
                                name: 0,
                                kind: ddlog_intern(&TypeKind::hir_Unknown),
                            }],
                        },
                        body: ddlog_intern(&Stmt::hir_StmtSeq {
                            first: ddlog_intern(&Stmt::hir_StmtExpr { expr: 0 }),
                            second: ddlog_intern(&Stmt::hir_StmtExpr { expr: 2 }),
                        }),
                        ret: ddlog_intern(&TypeKind::hir_Unknown),
                    },
                },
            })
            .into_ddvalue(),
        },
    ];
    program.apply_valupdates(updates.into_iter())?;

    let delta = program.transaction_commit_dump_changes()?;
    println!("\nState after transaction 1");
    dump_delta(&delta);

    program.stop()
}

fn dump_delta(delta: &DeltaMap<DDValue>) {
    for (rel, changes) in delta.iter() {
        crunch_shared::debug!("Changes to relation {}", relid2name(*rel).unwrap());
        for (val, weight) in changes.iter() {
            crunch_shared::debug!("{} {:+}", val, weight);
        }
    }
}