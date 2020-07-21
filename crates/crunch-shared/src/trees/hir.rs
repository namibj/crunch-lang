pub use crate::trees::{
    ast::{
        BinaryOp, CompOp, Float, Integer, Literal as AstLiteral, LiteralVal as AstLiteralVal, Rune,
        Text, Type as AstType, Vis,
    },
    ItemPath, Signedness,
};
use crate::{
    error::{Locatable, Location, Span},
    strings::{StrInterner, StrT},
    trees::{CallConv, Sided},
};
#[cfg(feature = "no-std")]
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TypeId(usize);

impl TypeId {
    #[inline]
    pub(crate) const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Item<'ctx> {
    Function(Function<'ctx>),
    ExternFunc(ExternFunc),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Function<'ctx> {
    pub name: ItemPath,
    pub vis: Vis,
    pub args: Locatable<Vec<FuncArg>>,
    pub body: Block<&'ctx Stmt<'ctx>>,
    pub ret: TypeId,
    pub loc: Location,
    pub sig: Location,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct FuncArg {
    pub name: Var,
    pub kind: TypeId,
    pub loc: Location,
}

impl FuncArg {
    #[inline]
    pub const fn location(&self) -> Location {
        self.loc
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ExternFunc {
    pub name: ItemPath,
    pub vis: Vis,
    pub args: Locatable<Vec<FuncArg>>,
    pub ret: TypeId,
    pub callconv: CallConv,
    pub loc: Location,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Stmt<'ctx> {
    Item(&'ctx Item<'ctx>),
    Expr(&'ctx Expr<'ctx>),
    VarDecl(VarDecl<'ctx>),
}

impl<'ctx> From<&'ctx Item<'ctx>> for Stmt<'ctx> {
    #[inline]
    fn from(item: &'ctx Item<'ctx>) -> Self {
        Self::Item(item)
    }
}

impl<'ctx> From<&'ctx Expr<'ctx>> for Stmt<'ctx> {
    #[inline]
    fn from(expr: &'ctx Expr<'ctx>) -> Self {
        Self::Expr(expr)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Expr<'ctx> {
    pub kind: ExprKind<'ctx>,
    pub loc: Location,
}

impl<'ctx> Expr<'ctx> {
    #[inline]
    pub const fn location(&self) -> Location {
        self.loc
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ExprKind<'ctx> {
    Match(Match<'ctx>),
    Scope(Block<&'ctx Stmt<'ctx>>),
    Loop(Block<&'ctx Stmt<'ctx>>),
    Return(Return<'ctx>),
    Continue,
    Break(Break<'ctx>),
    FnCall(FuncCall<'ctx>),
    Literal(Literal),
    Comparison(Sided<CompOp, &'ctx Expr<'ctx>>),
    Variable(Var, TypeId),
    Assign(Var, &'ctx Expr<'ctx>),
    BinOp(Sided<BinaryOp, &'ctx Expr<'ctx>>),
    Cast(Cast<'ctx>),
    Reference(Reference<'ctx>),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Var {
    User(StrT),
    Auto(usize),
}

impl Var {
    #[inline]
    pub fn to_string(&self, interner: &StrInterner) -> String {
        match *self {
            Self::User(var) => interner.resolve(var).as_ref().to_owned(),
            Self::Auto(var) => var.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct VarDecl<'ctx> {
    pub name: Var,
    pub value: &'ctx Expr<'ctx>,
    pub mutable: bool,
    pub ty: TypeId,
    pub loc: Location,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FuncCall<'ctx> {
    pub func: ItemPath,
    pub args: Vec<&'ctx Expr<'ctx>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Match<'ctx> {
    pub cond: &'ctx Expr<'ctx>,
    pub arms: Vec<MatchArm<'ctx>>,
    pub ty: TypeId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MatchArm<'ctx> {
    pub bind: Binding,
    pub guard: Option<&'ctx Expr<'ctx>>,
    pub body: Block<&'ctx Stmt<'ctx>>,
    pub ty: TypeId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Binding {
    // TODO: Enum for mutability/referential status?
    pub reference: bool,
    pub mutable: bool,
    pub pattern: Pattern,
    pub ty: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Pattern {
    Literal(Literal),
    Ident(StrT),
    ItemPath(ItemPath),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Return<'ctx> {
    pub val: Option<&'ctx Expr<'ctx>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Break<'ctx> {
    pub val: Option<&'ctx Expr<'ctx>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Block<T> {
    pub block: Vec<T>,
    pub loc: Location,
}

impl<T> Block<T> {
    #[inline]
    pub fn new(loc: Location) -> Self {
        Self {
            block: Vec::new(),
            loc,
        }
    }

    #[inline]
    pub fn with_capacity(loc: Location, capacity: usize) -> Self {
        Self {
            block: Vec::with_capacity(capacity),
            loc,
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        self.block.push(item);
    }

    #[inline]
    pub fn insert(&mut self, idx: usize, item: T) {
        self.block.insert(idx, item);
    }

    #[inline]
    pub fn location(&self) -> Location {
        self.loc
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.loc.span()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.block.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.block.is_empty()
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.block.iter()
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'a {
        self.block.iter_mut()
    }

    #[inline]
    pub fn from_iter<I: IntoIterator<Item = T>>(loc: Location, iter: I) -> Self {
        let mut block = Vec::with_capacity(10);
        for item in iter {
            block.push(item);
        }

        Self { block, loc }
    }
}

impl<T> Block<T>
where
    T: Clone,
{
    #[inline]
    pub fn extend_from_slice<S>(&mut self, slice: S)
    where
        S: AsRef<[T]>,
    {
        self.block.extend_from_slice(slice.as_ref())
    }
}

impl<T> Extend<T> for Block<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.block.extend(iter)
    }
}

/// A type
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Type {
    /// The kind of type this type is
    pub kind: TypeKind,
    /// The type's source location
    pub loc: Location,
}

impl Type {
    /// Creates a new `Type`
    #[inline]
    pub const fn new(kind: TypeKind, loc: Location) -> Self {
        Self { kind, loc }
    }

    /// Returns the type's location
    #[inline]
    pub const fn location(&self) -> Location {
        self.loc
    }

    /// Returns `true` if the type is `Unknown`
    #[inline]
    pub fn is_unknown(&self) -> bool {
        self.kind.is_unknown()
    }
}

/// The type that a type actually is
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum TypeKind {
    /// An unknown type
    Unknown,
    /// An integer of potentially unknown width & sign
    Integer {
        /// Whether the integer is signed or not, `None` for unknown sign
        signed: Option<bool>,
        /// The integer's width, `None` for an unknown width
        width: Option<u16>,
    },
    /// A string
    String,
    /// A boolean
    Bool,
    /// The unit type
    Unit,
    /// The absurd type
    Absurd,
    /// An array type, arr[_; _]
    Array {
        /// The type of the array's elements
        element: TypeId,
        /// The length of the array
        length: u64,
    },
    /// A slice type, slice[_]
    Slice {
        /// The type of the slice's elements
        element: TypeId,
    },
    /// A reference type, &_ or &mut _
    Reference {
        /// The type the reference points to
        referee: TypeId,
        /// Whether the reference is mutable or not
        mutable: bool,
    },
    /// A pointer type, *const _ or *mut _
    Pointer {
        /// The type the pointer points to
        pointee: TypeId,
        /// Whether the pointer is mutable or not
        mutable: bool,
    },
    /// A type with the type of another type
    Variable(TypeId),
}

impl TypeKind {
    /// Returns `true` if the type is `Unknown`
    #[inline]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Cast<'ctx> {
    pub casted: &'ctx Expr<'ctx>,
    pub ty: TypeId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Reference<'ctx> {
    pub mutable: bool,
    pub reference: &'ctx Expr<'ctx>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Literal {
    pub val: LiteralVal,
    pub ty: TypeId,
    pub loc: Location,
}

impl Literal {
    #[inline]
    pub const fn location(&self) -> Location {
        self.loc
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum LiteralVal {
    Integer(Integer),
    Bool(bool),
    String(Text),
    Rune(Rune),
    Float(Float),
    Array { elements: Vec<Literal> },
    // TODO: Tuples, slices, others?
}
