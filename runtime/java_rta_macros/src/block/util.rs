//! 类型分类与属性工具函数。

use syn::{Attribute, Expr, Signature, Token, Type};

use super::parse::lit_str;

// ══════════════════════════════════════════════════════════════════════════════
// 类型分类与属性工具
// ══════════════════════════════════════════════════════════════════════════════

pub(crate) const BASIC_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "bool", "char",
];

pub(crate) fn is_basic(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if tp.qself.is_none() && tp.path.segments.len() == 1 {
            let seg = &tp.path.segments[0];
            if seg.arguments.is_empty() {
                return BASIC_TYPES.contains(&seg.ident.to_string().as_str());
            }
        }
    }
    false
}

/// 字段类型是否为裸 `i64`（Java `long` 的非擦除形态）——Unsafe 实例字段
/// long 原子协议（`__unsafe_long_cell`）的臂生成条件。
pub(crate) fn type_is_long(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if tp.qself.is_none() && tp.path.segments.len() == 1 {
            let seg = &tp.path.segments[0];
            return seg.arguments.is_empty() && seg.ident == "i64";
        }
    }
    false
}

/// 字段类型是否为裸 `i32`（Java `int` 的非擦除形态）——Unsafe 实例字段
/// int 原子协议（`__unsafe_int_cell`）的臂生成条件。
pub(crate) fn type_is_int(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if tp.qself.is_none() && tp.path.segments.len() == 1 {
            let seg = &tp.path.segments[0];
            return seg.arguments.is_empty() && seg.ident == "i32";
        }
    }
    false
}

pub(crate) const META_ATTRS: &[&str] = &[
    "descriptor",
    "generic_signature",
    "native",
    "readonly",
    "field_sig",
    "java_method",
    "java_native",
    "jvm_native",
];

pub(crate) fn strip_meta_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|a| {
            let p = a.path();
            !META_ATTRS.iter().any(|n| p.is_ident(n))
        })
        .collect()
}

pub(crate) fn attr_str(attrs: &[Attribute], name: &str) -> Option<String> {
    for a in attrs {
        if a.path().is_ident(name) {
            if let Ok(s) = lit_str(a) {
                return Some(s);
            }
        }
        if let syn::Meta::List(ml) = &a.meta {
            let mut found: Option<String> = None;
            let _ = ml.parse_nested_meta(|m| {
                if m.path.is_ident(name) {
                    if let Ok(v) = m.value().and_then(|v| v.parse::<syn::LitStr>()) {
                        found = Some(v.value());
                    }
                } else if m.input.peek(Token![=]) {
                    let _ = m.value().and_then(|v| v.parse::<Expr>());
                }
                Ok(())
            });
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

// ══════════════════════════════════════════════════════════════════════════════
// 方法分类
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub(crate) enum MethodKind {
    /// 新定义的虚方法：进 trait default impl + 生成 base 自由函数
    VirtualDefine,
    /// 覆盖祖先虚方法：进 `impl AncestorVTable for __inner` + 生成 base 自由函数
    /// vtable_class = 祖先 Rust 类名（不带 __VTable 后缀）
    VirtualOverride { vtable_class: String },
    /// 构造器或静态方法：留在 wrapper impl block
    Constructor,
    /// 非虚实例方法：留在 wrapper impl block
    NonVirtual,
    /// 继承成员声明（无方法体）：方法由祖先 `owner` 声明、本类未覆盖。
    /// 宏在 wrapper 上生成同名 inherent 方法，使调用点与 Java 一致（`obj.method(args)`）：
    ///   - `vtable_owner = Some(T)`：虚方法，经 `T__VTable` supertrait 分派（保持多态）
    ///   - `vtable_owner = None`：祖先的非虚方法（native 等），向上转型后调用
    ///   - `owner_kind = "interface"`：owner 是本类（经超类 / 超接口）实现的接口，本类及祖先类
    ///     均未声明该方法（抽象类隐式继承的接口抽象方法，JVMS §5.4.3.3 的 miranda 方法）：
    ///     经接口载体分派，由对象运行时类的接口 vtable 命中具体实现
    /// owner / vtable_owner 均为本类视角下的 Rust 类型（含类型实参，如 `AbstractList<E>`）。
    Inherited { owner: String, vtable_owner: Option<String>, interface_owner: bool },
}

pub(crate) fn classify_method(attrs: &[Attribute], sig: &Signature, self_name: &str) -> MethodKind {
    let mname = sig.ident.to_string();
    if let Some(owner) = attr_str(attrs, "inherited_from") {
        return MethodKind::Inherited {
            owner,
            vtable_owner: attr_str(attrs, "vtable_owner"),
            interface_owner: attr_str(attrs, "owner_kind").as_deref() == Some("interface"),
        };
    }
    if let Some(virtual_in) = attr_str(attrs, "virtual_in") {
        if virtual_in == self_name {
            MethodKind::VirtualDefine
        } else {
            MethodKind::VirtualOverride { vtable_class: virtual_in }
        }
    } else if mname == "new" || mname.starts_with("new_") || mname == "main" {
        MethodKind::Constructor
    } else {
        MethodKind::NonVirtual
    }
}
