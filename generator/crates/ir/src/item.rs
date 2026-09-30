//! 顶层条目（← `RsFn` / `RsStruct` / `RsImpl` / `RsUse` / `RsMod` / `RsTypeAlias` / `RsRawItem`）。

use crate::{Ident, Path, Raw, Stmt, Type};

/// 可见性（← `vis: str` 的 `'pub'` / `''` / `'pub(crate)'`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Vis {
    #[default]
    Pub,
    PubCrate,
    Private,
}

impl Vis {
    /// 渲染前缀（含尾随空格；私有为空串）。
    pub fn prefix(self) -> &'static str {
        match self {
            Vis::Pub => "pub ",
            Vis::PubCrate => "pub(crate) ",
            Vis::Private => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnItem {
    pub name: Ident,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Vec<Stmt>,
    pub vis: Vis,
    pub is_unsafe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    pub name: Ident,
    pub ty: Type,
    pub vis: Vis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructItem {
    pub name: Ident,
    pub fields: Vec<StructField>,
    pub vis: Vis,
    /// `#[derive(..)]` 的 trait 路径
    pub derives: Vec<Path>,
}

/// `impl [Trait for] SelfTy { fns }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplItem {
    pub self_ty: Type,
    pub trait_: Option<Path>,
    pub items: Vec<FnItem>,
}

/// use 树：`a::b`、`a::b as c`、`a::*`、`a::{b, c}`（← `RsUse.path: str`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UseTree {
    Path { prefix: Path, tail: Box<UseTree> },
    Name(Ident),
    Rename { name: Ident, alias: Ident },
    Glob,
    Group(Vec<UseTree>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModItem {
    pub name: Ident,
    pub vis: Vis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAlias {
    pub name: Ident,
    pub ty: Type,
    pub vis: Vis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Fn(FnItem),
    Struct(StructItem),
    Impl(ImplItem),
    Use(UseTree),
    Mod(ModItem),
    TypeAlias(TypeAlias),
    Raw(Raw),
}

impl Item {
    /// 文本逃生舱条目（raw-audit `raw_item` 计数，位点取调用者）
    #[track_caller]
    pub fn raw(text: impl Into<String>) -> Item {
        crate::raw_audit::record(crate::raw_audit::RawKind::Item, std::panic::Location::caller());
        Item::Raw(Raw::from_text(text.into()))
    }
}
