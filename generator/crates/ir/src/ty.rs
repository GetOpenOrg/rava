//! 类型节点（← `RsPrimitive` / `RsNamed` / `RsGeneric` / `RsRef` / `RsSlice` / `RsTuple` / `RsInfer`）。
//!
//! Python 的 `RsNamed(name)` 把整段类型文本（`HashMap_Node<K, V>`、`JArray<i8>`）塞进
//! name，`RsGeneric(outer, params)` 只在少数位置使用；这里二者统一为 [`Type::Path`]：
//! 路径段 + 每段的泛型实参，均为结构。

use crate::{anchors, Ident};

/// Rust 基本类型（含单元类型 `()`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prim {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Usize,
    Isize,
    F32,
    F64,
    Bool,
    Char,
    Unit,
}

impl Prim {
    pub fn as_str(self) -> &'static str {
        match self {
            Prim::I8 => "i8",
            Prim::I16 => "i16",
            Prim::I32 => "i32",
            Prim::I64 => "i64",
            Prim::U8 => "u8",
            Prim::U16 => "u16",
            Prim::U32 => "u32",
            Prim::U64 => "u64",
            Prim::Usize => "usize",
            Prim::Isize => "isize",
            Prim::F32 => "f32",
            Prim::F64 => "f64",
            Prim::Bool => "bool",
            Prim::Char => "char",
            Prim::Unit => "()",
        }
    }

    /// 按 Rust 拼写查找（`"i32"` → `Prim::I32`）。
    pub fn from_name(name: &str) -> Option<Prim> {
        const ALL: [Prim; 15] = [
            Prim::I8,
            Prim::I16,
            Prim::I32,
            Prim::I64,
            Prim::U8,
            Prim::U16,
            Prim::U32,
            Prim::U64,
            Prim::Usize,
            Prim::Isize,
            Prim::F32,
            Prim::F64,
            Prim::Bool,
            Prim::Char,
            Prim::Unit,
        ];
        ALL.into_iter().find(|p| p.as_str() == name)
    }
}

/// 路径段：标识符 + 泛型实参（类型位置渲染 `Name<A, B>`，表达式位置渲染 `Name::<A, B>`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub ident: Ident,
    pub generics: Vec<Type>,
}

impl PathSegment {
    pub fn new(ident: Ident) -> PathSegment {
        PathSegment { ident, generics: Vec::new() }
    }

    pub fn with_generics(ident: Ident, generics: Vec<Type>) -> PathSegment {
        PathSegment { ident, generics }
    }
}

/// 路径：`a::b::C<T>`；`global` 为真时带前导 `::`（`::std::convert::From<_>`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub global: bool,
    pub segments: Vec<PathSegment>,
}

impl Path {
    pub fn new(segments: Vec<PathSegment>) -> Path {
        Path { global: false, segments }
    }

    /// 无泛型实参的多段路径。
    pub fn from_idents(idents: impl IntoIterator<Item = Ident>) -> Path {
        Path::new(idents.into_iter().map(PathSegment::new).collect())
    }

    pub fn last(&self) -> Option<&PathSegment> {
        self.segments.last()
    }
}

/// 类型节点。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Prim(Prim),
    Path(Path),
    /// `&'lt mut T`
    Ref { inner: Box<Type>, mutable: bool, lifetime: Option<Ident> },
    /// `&[T]`（Python `RsSlice` 即带引用的切片）
    Slice(Box<Type>),
    /// `(A, B)`；空元组渲染为 `()`
    Tuple(Vec<Type>),
    /// `_`
    Infer,
}

impl Type {
    pub const I32: Type = Type::Prim(Prim::I32);
    pub const I64: Type = Type::Prim(Prim::I64);
    pub const F32: Type = Type::Prim(Prim::F32);
    pub const F64: Type = Type::Prim(Prim::F64);
    pub const BOOL: Type = Type::Prim(Prim::Bool);
    pub const USIZE: Type = Type::Prim(Prim::Usize);
    pub const UNIT: Type = Type::Prim(Prim::Unit);

    /// 单段具名类型 `Name<generics..>`。
    pub fn named(ident: Ident, generics: Vec<Type>) -> Type {
        Type::Path(Path::new(vec![PathSegment::with_generics(ident, generics)]))
    }

    /// JVM 数组载体 `JArray<E>` 的元素类型（其余类型返回 None；
    /// 无实参的裸 `JArray` 同样返回 None，与 Python `split_rust_type_args` 为空一致）。
    pub fn array_elem(&self) -> Option<&Type> {
        match self {
            Type::Path(p) if !p.global && p.segments.len() == 1 => {
                let seg = &p.segments[0];
                if seg.ident.as_str() == anchors::ARRAY {
                    seg.generics.first()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
