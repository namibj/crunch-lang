use crunch_shared::{
    strings::StrInterner,
    trees::mir::{
        Block, BlockId, Constant, Function as MirFunction, Instruction, Mir, RightValue, Rval,
        Type as MirType, VarId,
    },
    utils::{HashMap, Timer},
    visitors::mir::MirVisitor,
};
use llvm::{
    module::{BuildingBlock, FunctionBuilder, Linkage, Module},
    types::{IntType, Type, VoidType},
    values::{BasicBlock, FunctionValue, InstructionValue, Value},
    Context, Error, ErrorKind, Result,
};

pub mod llvm;

pub struct CodeGenerator<'ctx> {
    module: &'ctx Module<'ctx>,
    ctx: &'ctx Context,
    values: HashMap<VarId, Value<'ctx>>,
    blocks: HashMap<BlockId, BasicBlock<'ctx>>,
    function_builder: Option<FunctionBuilder<'ctx>>,
    block_builder: Option<BuildingBlock<'ctx>>,
    interner: &'ctx StrInterner,
    raw_blocks: Vec<Option<Block>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(module: &'ctx Module<'ctx>, interner: &'ctx StrInterner) -> Self {
        Self {
            module,
            ctx: module.context(),
            values: HashMap::new(),
            blocks: HashMap::new(),
            function_builder: None,
            block_builder: None,
            interner,
            raw_blocks: Vec::new(),
        }
    }

    pub fn generate(mut self, mir: Mir) -> Result<()> {
        let _codegen_timer = Timer::start("codegen");

        for function in mir.iter() {
            self.visit_function(function)?;
        }

        Ok(())
    }

    fn value(&self, id: &VarId) -> Result<Value<'ctx>> {
        self.values.get(id).map_or_else(
            || {
                Err(Error::new(
                    "Attempted to get a value that doesn't exist",
                    ErrorKind::LLVMError,
                ))
            },
            |value| Ok(*value),
        )
    }
}

impl<'ctx> MirVisitor for CodeGenerator<'ctx> {
    type FunctionOutput = Result<FunctionValue<'ctx>>;
    type BlockOutput = Result<BasicBlock<'ctx>>;
    type InstructionOutput = Result<Option<InstructionValue<'ctx>>>;
    type RvalOutput = Result<Value<'ctx>>;
    type ConstantOutput = Result<Value<'ctx>>;
    type TypeOutput = Result<Type<'ctx>>;

    fn visit_function(&mut self, function: &MirFunction) -> Self::FunctionOutput {
        let args: Vec<Type<'ctx>> = function
            .args
            .iter()
            .map(|(_, arg)| self.visit_type(arg).unwrap())
            .collect();

        let sig =
            self.module
                .function_ty(self.visit_type(&function.ret)?, args.as_slice(), false)?;

        self.raw_blocks = function
            .blocks
            .clone()
            .into_iter()
            .filter_map(|b| if !b.is_empty() { Some(Some(b)) } else { None })
            .collect();

        let builder: FunctionBuilder<'ctx> = self.module.build_function(
            function
                .name
                .as_ref()
                .map_or_else(|| function.id.0.to_string(), |n| n.to_string(self.interner)),
            sig,
        )?;

        builder.with_linkage(Linkage::External);
        self.function_builder = Some(builder);

        while let Some(block) = self.raw_blocks.iter_mut().find_map(|b| b.take()) {
            self.visit_block(&block)?;
        }

        self.function_builder
            .take()
            .expect("there should be a finished function after a function finishes")
            .finish()
    }

    fn visit_block(&mut self, block: &Block) -> Self::BlockOutput {
        let block_builder: BuildingBlock<'ctx> = self
            .function_builder
            .as_mut()
            .expect("blocks should be inside builders")
            .append_block()?;

        let basic_block = block_builder.basic_block();
        self.blocks.insert(block.id, basic_block);
        self.block_builder = Some(block_builder);

        for instruction in block.iter() {
            self.visit_instruction(instruction)?;
        }

        Ok(basic_block)
    }

    fn visit_instruction(&mut self, instruction: &Instruction) -> Self::InstructionOutput {
        match instruction {
            Instruction::Return(ret) => {
                // If the return value is `None` then it's interpreted as returning void
                let return_value = if let Some(ret) = ret {
                    Some(self.visit_rval(ret)?)
                } else {
                    None
                };

                self.block_builder
                    .as_ref()
                    .expect("instructions should be in a block")
                    .ret(return_value)
                    .map(Some)
            }

            Instruction::Goto(bl) => self
                .block_builder
                .as_ref()
                .expect("instructions should be in a block")
                .branch(self.blocks.get(&bl).unwrap())
                .map(Some),

            Instruction::Assign(id, val) => {
                let val = self.visit_rval(val)?;
                self.values.insert(*id, val);

                Ok(None)
            }

            Instruction::Switch(cond, branches) => {
                let current_block = self
                    .block_builder
                    .as_ref()
                    .expect("instructions should be in a block")
                    .basic_block();
                let cond = self.visit_rval(cond)?;

                let true_branch = branches[0].clone();
                let true_branch = match self.blocks.get(&true_branch.1) {
                    Some(block) => *block,
                    None => {
                        let block = self.raw_blocks[(true_branch.1).0].take().unwrap();
                        self.visit_block(&block)?
                    }
                };

                let false_branch = branches[1].clone();
                let false_branch = match self.blocks.get(&false_branch.1) {
                    Some(block) => *block,
                    None => {
                        let block = self.raw_blocks[(false_branch.1).0].take().unwrap();
                        self.visit_block(&block)?
                    }
                };

                self.block_builder = Some(
                    self.function_builder
                        .as_ref()
                        .expect("instructions should be in a function")
                        .move_to_end(current_block)?,
                );

                let instruction = self
                    .block_builder
                    .as_ref()
                    .expect("instructions should be in a block")
                    .conditional_branch(cond, true_branch, false_branch)
                    .map(Some);

                instruction
            }
        }
    }

    fn visit_rval(&mut self, RightValue { ty: _ty, val }: &RightValue) -> Self::RvalOutput {
        match val {
            Rval::Const(constant) => self.visit_constant(constant),
            Rval::Var(id) => self.value(id),

            Rval::Add(lhs, rhs) => {
                let (lhs, rhs) = (self.visit_rval(lhs)?, self.visit_rval(rhs)?);
                self.block_builder
                    .as_ref()
                    .expect("rvals should be inside blocks")
                    .add(lhs, rhs)
            }

            Rval::Sub(lhs, rhs) => {
                let (lhs, rhs) = (self.visit_rval(lhs)?, self.visit_rval(rhs)?);
                self.block_builder
                    .as_ref()
                    .expect("rvals should be inside blocks")
                    .sub(lhs, rhs)
            }

            Rval::Mul(lhs, rhs) => {
                let (lhs, rhs) = (self.visit_rval(lhs)?, self.visit_rval(rhs)?);
                self.block_builder
                    .as_ref()
                    .expect("rvals should be inside blocks")
                    .mult(lhs, rhs)
            }

            Rval::Phi(_, _) => todo!(),
            Rval::Call(_, _) => todo!(),
            Rval::Div(_, _) => todo!(),
        }
    }

    fn visit_constant(&mut self, constant: &Constant) -> Self::ConstantOutput {
        match constant {
            Constant::I64(int) => IntType::i64(self.ctx)?
                .constant(*int as u64, true)
                .map(|i| i.into()),

            Constant::U8(int) => IntType::u8(self.ctx)?
                .constant(*int as u64, true)
                .map(|i| i.into()),

            Constant::Bool(boolean) => IntType::i1(self.ctx)?
                .constant(*boolean as u64, true)
                .map(|i| i.into()),
        }
    }

    fn visit_type(&mut self, ty: &MirType) -> Self::TypeOutput {
        match ty {
            MirType::I64 => IntType::i64(self.ctx).map(|i| i.into()),
            MirType::U8 => IntType::u8(self.ctx).map(|i| i.into()),
            MirType::Bool => IntType::i1(self.ctx).map(|i| i.into()),
            MirType::Unit => VoidType::new(self.ctx).map(|i| i.into()),
        }
    }
}

#[test]
fn mir() {
    use crunch_parser::Parser;
    use crunch_shared::{
        context::Context as ParseContext,
        files::{CurrentFile, FileId, Files},
        symbol_table::Resolver,
        trees::mir::MirBuilder,
        visitors::ast::ItemVisitor,
    };
    use ladder::Ladder;
    use llvm::{
        target_machine::{CodegenFileKind, Target, TargetConf, TargetMachine},
        Context,
    };
    use std::fs::File;

    simple_logger::init().ok();

    let source = r#"
    fn main() -> i64
        let mut greeting: i64 := 10
        greeting *= 10

        if false
            return greeting
        else
            return 0
        end
    end
    "#;

    let compilation = Timer::start("compilation");

    let parse_ctx = ParseContext::default();
    let mut files = Files::new();
    files.add("<test>", source);

    match Parser::new(
        source,
        CurrentFile::new(FileId::new(0), source.len()),
        parse_ctx.clone(),
    )
    .parse()
    {
        Ok((items, mut warnings)) => {
            warnings.emit(&files);

            let mut resolver = Resolver::new(vec![parse_ctx.strings().intern("<test>")].into());
            for item in items.iter() {
                resolver.visit_item(item);
            }
            resolver.finalize();

            let ladder = Ladder::new().lower(&items);
            let mir = MirBuilder::new().lower(&ladder).unwrap();

            let ctx = Context::new().unwrap();
            let module = ctx.module("crunch-module").unwrap();

            CodeGenerator::new(&module, &parse_ctx.strings)
                .generate(mir)
                .unwrap();

            module.verify().unwrap();

            let object_emission = Timer::start("object file emission");

            Target::init_native(TargetConf::all()).unwrap();
            let target = Target::from_triple("x86_64-pc-windows-msvc").unwrap();
            let target_machine = TargetMachine::new(
                &target,
                "x86_64-pc-windows-msvc",
                None,
                None,
                None,
                None,
                None,
            )
            .unwrap();

            target_machine
                .emit_to_file(&module, "crunch.o", CodegenFileKind::Object)
                .unwrap();

            object_emission.end();

            let linking = Timer::start("linking");

            std::process::Command::new("lld-link")
                .args(&["/ENTRY:main", "crunch.o"])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            linking.end();
            compilation.end();

            let exit_code = std::process::Command::new("crunch.exe")
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap()
                .status;
            println!(
                "Source Code:{}\n\nLLVM IR:\n{}\n\nExited with code {:?}\nFile Sizes:\n    Source: {:>4} bytes\n    Object: {:>4} bytes\n    Binary: {:>4} bytes",
                source,
                format!("{:?}", module).trim().lines().map(|l| "    ".to_string() + l).collect::<Vec<String>>().join("\n"),
                exit_code.code(),
                source.as_bytes().len(),
                File::open("crunch.o")
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .len(),
                File::open("crunch.exe")
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .len(),
            );
        }

        Err(mut err) => {
            err.emit(&files);
        }
    }
}
