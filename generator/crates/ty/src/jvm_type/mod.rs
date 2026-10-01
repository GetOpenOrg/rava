//! JVM 类型代数：擦除 / 代入 / 子类型 / 构造入口。
//!
//! 与 Python 的差异：registry 参数改为 [`Registry`]（空注册表 = Python 的 `None` /
//! 空 dict，二者在 Python 侧同为假值、行为一致）；闭包短名回退 `_closure_hit`
//! 不移植（binary 即身份，见 [`subtype`]）。

mod bridge;
mod parse;
pub mod subtype;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

pub use parse::TypeParseError;

use crate::consts;
use crate::registry::Registry;

/// JVM 基本类型（含 void）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimKind {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void,
}

impl PrimKind {
    /// 描述符字符（含 `V`）→ 基本类型
    pub fn from_desc(c: u8) -> Option<PrimKind> {
        Some(match c {
            b'Z' => PrimKind::Boolean,
            b'B' => PrimKind::Byte,
            b'C' => PrimKind::Char,
            b'S' => PrimKind::Short,
            b'I' => PrimKind::Int,
            b'J' => PrimKind::Long,
            b'F' => PrimKind::Float,
            b'D' => PrimKind::Double,
            b'V' => PrimKind::Void,
            _ => return None,
        })
    }

    /// Java 拼写（`int` / `void` …）
    pub fn java_name(self) -> &'static str {
        match self {
            PrimKind::Boolean => "boolean",
            PrimKind::Byte => "byte",
            PrimKind::Char => "char",
            PrimKind::Short => "short",
            PrimKind::Int => "int",
            PrimKind::Long => "long",
            PrimKind::Float => "float",
            PrimKind::Double => "double",
            PrimKind::Void => "void",
        }
    }

    /// Rust 拼写（void → `()`）
    pub fn rust_name(self) -> &'static str {
        match self {
            PrimKind::Boolean => "bool",
            PrimKind::Byte => "i8",
            PrimKind::Char => "u16",
            PrimKind::Short => "i16",
            PrimKind::Int => "i32",
            PrimKind::Long => "i64",
            PrimKind::Float => "f32",
            PrimKind::Double => "f64",
            PrimKind::Void => "()",
        }
    }
}

/// 通配符种类：`*` / `+`（extends）/ `-`（super）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WildKind {
    Unbounded,
    Extends,
    Super,
}

/// Rust 宿主基本类型（生成代码内部的索引 / 字节值，无 JVM 对应物）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HostPrim {
    U8,
    U32,
    U64,
    Usize,
}

impl HostPrim {
    pub fn rust_name(self) -> &'static str {
        match self {
            HostPrim::U8 => "u8",
            HostPrim::U32 => "u32",
            HostPrim::U64 => "u64",
            HostPrim::Usize => "usize",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JvmType {
    Primitive(PrimKind),
    /// 类 / 接口引用；raw 类型 `args` 为空
    Class {
        binary: String,
        args: Vec<JvmType>,
        is_interface: bool,
    },
    Array(Box<JvmType>),
    /// 类型变量；`bound = None` 为无界
    TypeVar {
        name: String,
        bound: Option<Box<JvmType>>,
    },
    Wildcard {
        kind: WildKind,
        bound: Option<Box<JvmType>>,
    },
    /// null 底类型
    Null,
    HostPrim(HostPrim),
}

impl JvmType {
    pub fn object_type() -> JvmType {
        JvmType::class(consts::OBJECT)
    }

    /// 非接口 raw 类引用
    pub fn class(binary: impl Into<String>) -> JvmType {
        JvmType::Class {
            binary: binary.into(),
            args: Vec::new(),
            is_interface: false,
        }
    }

    pub fn class_with(
        binary: impl Into<String>,
        args: Vec<JvmType>,
        is_interface: bool,
    ) -> JvmType {
        JvmType::Class {
            binary: binary.into(),
            args,
            is_interface,
        }
    }

    pub fn array(elem: JvmType) -> JvmType {
        JvmType::Array(Box::new(elem))
    }

    pub fn type_var(name: impl Into<String>, bound: Option<JvmType>) -> JvmType {
        JvmType::TypeVar {
            name: name.into(),
            bound: bound.map(Box::new),
        }
    }

    pub fn wildcard(kind: WildKind, bound: Option<JvmType>) -> JvmType {
        JvmType::Wildcard {
            kind,
            bound: bound.map(Box::new),
        }
    }

    /// binary name → 类引用；注册表可查时补全 is_interface
    pub fn class_of(binary: &str, reg: &Registry) -> JvmType {
        let is_interface = reg.get(binary).is_some_and(|c| c.is_interface());
        JvmType::class_with(binary, Vec::new(), is_interface)
    }

    pub fn is_interface(&self) -> bool {
        matches!(
            self,
            JvmType::Class {
                is_interface: true,
                ..
            }
        )
    }

    /// 擦除到 raw：实参 → 擦除；TypeVar / Wildcard → 上界擦除（无界 → Object）；
    /// 数组擦除元素。幂等
    pub fn erasure(&self) -> JvmType {
        match self {
            JvmType::Class {
                binary,
                is_interface,
                ..
            } => JvmType::Class {
                binary: binary.clone(),
                args: Vec::new(),
                is_interface: *is_interface,
            },
            JvmType::Array(e) => JvmType::array(e.erasure()),
            JvmType::TypeVar { bound, .. } | JvmType::Wildcard { bound, .. } => match bound {
                Some(b) => b.erasure(),
                None => JvmType::object_type(),
            },
            _ => self.clone(),
        }
    }

    /// 类型变量代入（单遍：命中值不再代入；命中的 TypeVar 连同上界整体替换）
    pub fn substitute(&self, mapping: &BTreeMap<String, JvmType>) -> JvmType {
        let sub_bound =
            |b: &Option<Box<JvmType>>| b.as_ref().map(|b| Box::new(b.substitute(mapping)));
        match self {
            JvmType::TypeVar { name, bound } => match mapping.get(name) {
                Some(t) => t.clone(),
                None => JvmType::TypeVar {
                    name: name.clone(),
                    bound: sub_bound(bound),
                },
            },
            JvmType::Class {
                binary,
                args,
                is_interface,
            } => JvmType::Class {
                binary: binary.clone(),
                args: args.iter().map(|a| a.substitute(mapping)).collect(),
                is_interface: *is_interface,
            },
            JvmType::Array(e) => JvmType::array(e.substitute(mapping)),
            JvmType::Wildcard { kind, bound } => JvmType::Wildcard {
                kind: *kind,
                bound: sub_bound(bound),
            },
            _ => self.clone(),
        }
    }

    /// 人读串（诊断用）
    pub fn to_display(&self) -> String {
        let bound_or_object =
            |b: &Option<Box<JvmType>>| b.as_ref().map_or("Object".to_string(), |b| b.to_display());
        match self {
            JvmType::Primitive(k) => k.java_name().to_string(),
            JvmType::Class { binary, args, .. } => {
                let base = binary.replace(['/', '$'], ".");
                if args.is_empty() {
                    base
                } else {
                    let inner: Vec<String> = args.iter().map(JvmType::to_display).collect();
                    format!("{base}<{}>", inner.join(", "))
                }
            }
            JvmType::Array(e) => format!("{}[]", e.to_display()),
            JvmType::TypeVar {
                name,
                bound: Some(b),
            } => format!("{name} extends {}", b.to_display()),
            JvmType::TypeVar { name, bound: None } => name.clone(),
            JvmType::Wildcard {
                kind: WildKind::Extends,
                bound,
            } => format!("? extends {}", bound_or_object(bound)),
            JvmType::Wildcard {
                kind: WildKind::Super,
                bound,
            } => format!("? super {}", bound_or_object(bound)),
            JvmType::Wildcard {
                kind: WildKind::Unbounded,
                ..
            } => "?".to_string(),
            JvmType::Null => "null".to_string(),
            // Python 基类回落分支：非上列变体一律 'null'
            JvmType::HostPrim(_) => "null".to_string(),
        }
    }
}
