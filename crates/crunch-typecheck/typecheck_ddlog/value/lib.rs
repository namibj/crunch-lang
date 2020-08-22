#![allow(
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::unknown_clippy_lints,
    clippy::missing_safety_doc
)]

use std::convert::TryFrom;
use std::ffi;
use std::fmt;
use std::hash::Hash;
use std::result;

use serde::Deserialize;
use serde::Serialize;

use differential_datalog::ddval::*;
use differential_datalog::decl_ddval_convert;
use differential_datalog::int::*;
use differential_datalog::program::*;
use differential_datalog::record;
use differential_datalog::record::FromRecord;
use differential_datalog::record::IntoRecord;
use differential_datalog::record::RelIdentifier;
use differential_datalog::uint::*;

use fnv::FnvHashMap;
use lazy_static::lazy_static;
use ordered_float::OrderedFloat;

use types::*;

/* FlatBuffers bindings generated by `ddlog` */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

impl TryFrom<&RelIdentifier> for Relations {
    type Error = ();

    fn try_from(rel_id: &RelIdentifier) -> result::Result<Self, Self::Error> {
        match rel_id {
            RelIdentifier::RelName(rname) => Relations::try_from(rname.as_ref()),
            RelIdentifier::RelId(id) => Relations::try_from(*id),
        }
    }
}


pub mod Value
{
    use super::*;
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Bitval64 (pub u64);
    impl abomonation::Abomonation for __Bitval64 {}
    impl fmt::Display for __Bitval64 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Bitval64 {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Bitval64> for record::Record {
        fn mutate(&self, v: &mut __Bitval64 ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Bitval64}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple0__ (pub ());
    impl abomonation::Abomonation for __Tuple0__ {}
    impl fmt::Display for __Tuple0__ {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple0__ {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple0__> for record::Record {
        fn mutate(&self, v: &mut __Tuple0__ ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple0__}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple3____Stringval_hir_Item_hir_Function (pub (String, super::hir_Item, super::hir_Function));
    impl abomonation::Abomonation for __Tuple3____Stringval_hir_Item_hir_Function {}
    impl fmt::Display for __Tuple3____Stringval_hir_Item_hir_Function {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple3____Stringval_hir_Item_hir_Function {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple3____Stringval_hir_Item_hir_Function> for record::Record {
        fn mutate(&self, v: &mut __Tuple3____Stringval_hir_Item_hir_Function ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple3____Stringval_hir_Item_hir_Function}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple2____Bitval64_internment_Intern__hir_ExprKind (pub (u64, super::internment_Intern<super::hir_ExprKind>));
    impl abomonation::Abomonation for __Tuple2____Bitval64_internment_Intern__hir_ExprKind {}
    impl fmt::Display for __Tuple2____Bitval64_internment_Intern__hir_ExprKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple2____Bitval64_internment_Intern__hir_ExprKind {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple2____Bitval64_internment_Intern__hir_ExprKind> for record::Record {
        fn mutate(&self, v: &mut __Tuple2____Bitval64_internment_Intern__hir_ExprKind ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple2____Bitval64_internment_Intern__hir_ExprKind}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64 (pub (u64, super::internment_Intern<super::hir_ExprKind>, u64));
    impl abomonation::Abomonation for __Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64 {}
    impl fmt::Display for __Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64 {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64> for record::Record {
        fn mutate(&self, v: &mut __Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64 ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple3____Bitval64_internment_Intern__hir_ExprKind___Bitval64}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt (pub (super::internment_Intern<super::hir_Scope>, super::internment_Intern<super::hir_Stmt>, super::internment_Intern<super::hir_Stmt>));
    impl abomonation::Abomonation for __Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt {}
    impl fmt::Display for __Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt> for record::Record {
        fn mutate(&self, v: &mut __Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function (pub (super::internment_Intern<super::std_Vec<u32>>, super::hir_Signature, String, super::hir_Function));
    impl abomonation::Abomonation for __Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function {}
    impl fmt::Display for __Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function> for record::Record {
        fn mutate(&self, v: &mut __Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct __Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function (pub (super::internment_Intern<super::std_Vec<u32>>, super::hir_Signature, super::hir_Function));
    impl abomonation::Abomonation for __Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function {}
    impl fmt::Display for __Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for __Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<__Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function> for record::Record {
        fn mutate(&self, v: &mut __Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{__Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct ClampUnknownInt (pub super::ClampUnknownInt);
    impl abomonation::Abomonation for ClampUnknownInt {}
    impl fmt::Display for ClampUnknownInt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for ClampUnknownInt {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<ClampUnknownInt> for record::Record {
        fn mutate(&self, v: &mut ClampUnknownInt ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{ClampUnknownInt}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct Errors (pub super::Errors);
    impl abomonation::Abomonation for Errors {}
    impl fmt::Display for Errors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for Errors {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<Errors> for record::Record {
        fn mutate(&self, v: &mut Errors ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{Errors}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct Expr (pub super::Expr);
    impl abomonation::Abomonation for Expr {}
    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for Expr {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<Expr> for record::Record {
        fn mutate(&self, v: &mut Expr ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{Expr}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct Functions (pub super::Functions);
    impl abomonation::Abomonation for Functions {}
    impl fmt::Display for Functions {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for Functions {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<Functions> for record::Record {
        fn mutate(&self, v: &mut Functions ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{Functions}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct InputItems (pub super::InputItems);
    impl abomonation::Abomonation for InputItems {}
    impl fmt::Display for InputItems {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for InputItems {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<InputItems> for record::Record {
        fn mutate(&self, v: &mut InputItems ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{InputItems}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct PropagateExprType (pub super::PropagateExprType);
    impl abomonation::Abomonation for PropagateExprType {}
    impl fmt::Display for PropagateExprType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for PropagateExprType {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<PropagateExprType> for record::Record {
        fn mutate(&self, v: &mut PropagateExprType ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{PropagateExprType}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct Statements (pub super::Statements);
    impl abomonation::Abomonation for Statements {}
    impl fmt::Display for Statements {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for Statements {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<Statements> for record::Record {
        fn mutate(&self, v: &mut Statements ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{Statements}
    #[derive(Default, Eq, Ord, Clone, Hash, PartialEq, PartialOrd, Serialize, Deserialize, Debug)]
    pub struct SymbolTable (pub super::SymbolTable);
    impl abomonation::Abomonation for SymbolTable {}
    impl fmt::Display for SymbolTable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.clone().into_record().fmt(f)
        }
    }
    impl record::IntoRecord for SymbolTable {
        fn into_record(self) -> record::Record {
            self.0.into_record()
        }
    }
    impl record::Mutator<SymbolTable> for record::Record {
        fn mutate(&self, v: &mut SymbolTable ) -> result::Result<(), std::string::String> {
            self.mutate(&mut v.0)
        }
    }
    //#[typetag::serde]
    decl_ddval_convert!{SymbolTable}
}
impl TryFrom<&str> for Relations {
    type Error = ();
    fn try_from(rname: &str) -> result::Result<Self, Self::Error> {
         match rname {
        "ClampUnknownInt" => Ok(Relations::ClampUnknownInt),
        "Errors" => Ok(Relations::Errors),
        "Expr" => Ok(Relations::Expr),
        "Functions" => Ok(Relations::Functions),
        "InputItems" => Ok(Relations::InputItems),
        "PropagateExprType" => Ok(Relations::PropagateExprType),
        "Statements" => Ok(Relations::Statements),
        "SymbolTable" => Ok(Relations::SymbolTable),
        "__MultiHead_0" => Ok(Relations::__MultiHead_0),
        "__MultiHead_1" => Ok(Relations::__MultiHead_1),
        "__MultiHead_3" => Ok(Relations::__MultiHead_3),
        "__Null" => Ok(Relations::__Null),
             _  => Err(())
         }
    }
}
impl Relations {
    pub fn is_output(&self) -> bool {
        match self {
        Relations::ClampUnknownInt => true,
        Relations::Errors => true,
        Relations::Functions => true,
        Relations::Statements => true,
        Relations::SymbolTable => true,
            _  => false
        }
    }
}
impl Relations {
    pub fn is_input(&self) -> bool {
        match self {
        Relations::Expr => true,
        Relations::InputItems => true,
            _  => false
        }
    }
}
impl TryFrom<RelId> for Relations {
    type Error = ();
    fn try_from(rid: RelId) -> result::Result<Self, Self::Error> {
         match rid {
        0 => Ok(Relations::ClampUnknownInt),
        1 => Ok(Relations::Errors),
        2 => Ok(Relations::Expr),
        3 => Ok(Relations::Functions),
        4 => Ok(Relations::InputItems),
        5 => Ok(Relations::PropagateExprType),
        6 => Ok(Relations::Statements),
        7 => Ok(Relations::SymbolTable),
        8 => Ok(Relations::__MultiHead_0),
        9 => Ok(Relations::__MultiHead_1),
        10 => Ok(Relations::__MultiHead_3),
        11 => Ok(Relations::__Null),
             _  => Err(())
         }
    }
}
pub fn relid2name(rid: RelId) -> Option<&'static str> {
   match rid {
        0 => Some(&"ClampUnknownInt"),
        1 => Some(&"Errors"),
        2 => Some(&"Expr"),
        3 => Some(&"Functions"),
        4 => Some(&"InputItems"),
        5 => Some(&"PropagateExprType"),
        6 => Some(&"Statements"),
        7 => Some(&"SymbolTable"),
        8 => Some(&"__MultiHead_0"),
        9 => Some(&"__MultiHead_1"),
        10 => Some(&"__MultiHead_3"),
        11 => Some(&"__Null"),
       _  => None
   }
}
pub fn relid2cname(rid: RelId) -> Option<&'static ffi::CStr> {
    RELIDMAPC.get(&rid).map(|c: &'static ffi::CString|c.as_ref())
}
lazy_static! {
    pub static ref RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::ClampUnknownInt, "ClampUnknownInt");
        m.insert(Relations::Errors, "Errors");
        m.insert(Relations::Expr, "Expr");
        m.insert(Relations::Functions, "Functions");
        m.insert(Relations::InputItems, "InputItems");
        m.insert(Relations::PropagateExprType, "PropagateExprType");
        m.insert(Relations::Statements, "Statements");
        m.insert(Relations::SymbolTable, "SymbolTable");
        m.insert(Relations::__MultiHead_0, "__MultiHead_0");
        m.insert(Relations::__MultiHead_1, "__MultiHead_1");
        m.insert(Relations::__MultiHead_3, "__MultiHead_3");
        m.insert(Relations::__Null, "__Null");
        m
   };
}
lazy_static! {
    pub static ref RELIDMAPC: FnvHashMap<RelId, ffi::CString> = {
        let mut m = FnvHashMap::default();
        m.insert(0, ffi::CString::new("ClampUnknownInt").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(1, ffi::CString::new("Errors").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(2, ffi::CString::new("Expr").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(3, ffi::CString::new("Functions").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(4, ffi::CString::new("InputItems").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(5, ffi::CString::new("PropagateExprType").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(6, ffi::CString::new("Statements").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(7, ffi::CString::new("SymbolTable").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(8, ffi::CString::new("__MultiHead_0").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(9, ffi::CString::new("__MultiHead_1").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(10, ffi::CString::new("__MultiHead_3").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m.insert(11, ffi::CString::new("__Null").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert relation name to C string").unwrap()));
        m
   };
}
lazy_static! {
    pub static ref INPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::Expr, "Expr");
        m.insert(Relations::InputItems, "InputItems");
        m
    };
}
lazy_static! {
    pub static ref OUTPUT_RELIDMAP: FnvHashMap<Relations, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Relations::ClampUnknownInt, "ClampUnknownInt");
        m.insert(Relations::Errors, "Errors");
        m.insert(Relations::Functions, "Functions");
        m.insert(Relations::Statements, "Statements");
        m.insert(Relations::SymbolTable, "SymbolTable");
        m
    };
}
impl TryFrom<&str> for Indexes {
    type Error = ();
    fn try_from(iname: &str) -> result::Result<Self, Self::Error> {
         match iname {
        "__Null_by_none" => Ok(Indexes::__Null_by_none),
             _  => Err(())
         }
    }
}
impl TryFrom<IdxId> for Indexes {
    type Error = ();
    fn try_from(iid: IdxId) -> result::Result<Self, Self::Error> {
         match iid {
        0 => Ok(Indexes::__Null_by_none),
             _  => Err(())
         }
    }
}
pub fn indexid2name(iid: IdxId) -> Option<&'static str> {
   match iid {
        0 => Some(&"__Null_by_none"),
       _  => None
   }
}
pub fn indexid2cname(iid: IdxId) -> Option<&'static ffi::CStr> {
    IDXIDMAPC.get(&iid).map(|c: &'static ffi::CString|c.as_ref())
}
lazy_static! {
    pub static ref IDXIDMAP: FnvHashMap<Indexes, &'static str> = {
        let mut m = FnvHashMap::default();
        m.insert(Indexes::__Null_by_none, "__Null_by_none");
        m
   };
}
lazy_static! {
    pub static ref IDXIDMAPC: FnvHashMap<IdxId, ffi::CString> = {
        let mut m = FnvHashMap::default();
        m.insert(0, ffi::CString::new("__Null_by_none").unwrap_or_else(|_|ffi::CString::new(r"Cannot convert index name to C string").unwrap()));
        m
   };
}
pub fn relval_from_record(rel: Relations, _rec: &record::Record) -> result::Result<DDValue, String> {
    match rel {
        Relations::ClampUnknownInt => {
            Ok(Value::ClampUnknownInt(<ClampUnknownInt>::from_record(_rec)?).into_ddvalue())
        },
        Relations::Errors => {
            Ok(Value::Errors(<Errors>::from_record(_rec)?).into_ddvalue())
        },
        Relations::Expr => {
            Ok(Value::Expr(<Expr>::from_record(_rec)?).into_ddvalue())
        },
        Relations::Functions => {
            Ok(Value::Functions(<Functions>::from_record(_rec)?).into_ddvalue())
        },
        Relations::InputItems => {
            Ok(Value::InputItems(<InputItems>::from_record(_rec)?).into_ddvalue())
        },
        Relations::PropagateExprType => {
            Ok(Value::PropagateExprType(<PropagateExprType>::from_record(_rec)?).into_ddvalue())
        },
        Relations::Statements => {
            Ok(Value::Statements(<Statements>::from_record(_rec)?).into_ddvalue())
        },
        Relations::SymbolTable => {
            Ok(Value::SymbolTable(<SymbolTable>::from_record(_rec)?).into_ddvalue())
        },
        Relations::__MultiHead_0 => {
            Ok(Value::__Tuple3__internment_Intern__std_Vec____Bitval32_hir_Signature_hir_Function(<(internment_Intern<std_Vec<u32>>, hir_Signature, hir_Function)>::from_record(_rec)?).into_ddvalue())
        },
        Relations::__MultiHead_1 => {
            Ok(Value::__Tuple4__internment_Intern__std_Vec____Bitval32_hir_Signature___Stringval_hir_Function(<(internment_Intern<std_Vec<u32>>, hir_Signature, String, hir_Function)>::from_record(_rec)?).into_ddvalue())
        },
        Relations::__MultiHead_3 => {
            Ok(Value::__Tuple3__internment_Intern__hir_Scope_internment_Intern__hir_Stmt_internment_Intern__hir_Stmt(<(internment_Intern<hir_Scope>, internment_Intern<hir_Stmt>, internment_Intern<hir_Stmt>)>::from_record(_rec)?).into_ddvalue())
        },
        Relations::__Null => {
            Ok(Value::__Tuple0__(<()>::from_record(_rec)?).into_ddvalue())
        }
    }
}
pub fn relkey_from_record(rel: Relations, _rec: &record::Record) -> result::Result<DDValue, String> {
    match rel {
        _ => Err(format!("relation {:?} does not have a primary key", rel))
    }
}
pub fn idxkey_from_record(idx: Indexes, _rec: &record::Record) -> result::Result<DDValue, String> {
    match idx {
        Indexes::__Null_by_none => {
            Ok(Value::__Tuple0__(<()>::from_record(_rec)?).into_ddvalue())
        }
    }
}
pub fn indexes2arrid(idx: Indexes) -> ArrId {
    match idx {
        Indexes::__Null_by_none => ( 11, 0),
    }
}
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub enum Relations {
    ClampUnknownInt = 0,
    Errors = 1,
    Expr = 2,
    Functions = 3,
    InputItems = 4,
    PropagateExprType = 5,
    Statements = 6,
    SymbolTable = 7,
    __MultiHead_0 = 8,
    __MultiHead_1 = 9,
    __MultiHead_3 = 10,
    __Null = 11
}
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub enum Indexes {
    __Null_by_none = 0
}