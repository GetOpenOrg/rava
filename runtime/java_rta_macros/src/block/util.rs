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
}

pub(crate) fn classify_method(attrs: &[Attribute], sig: &Signature, self_name: &str) -> MethodKind {
    let mname = sig.ident.to_string();
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
