//! VTableBodyKind 分类：判断方法体是否可安全放入 vtable 上下文。

use quote::quote;
use syn::Block;

/// 判断方法体是否可以安全放入 vtable 上下文（__inner impl）。
/// vtable 上下文中 self 是 &dyn Trait，禁止出现：
///   - Self:: 引用
///   - Clone::clone（可能依赖 wrapper 类型的 Clone impl）
///   - this.non_accessor_method()（调用 wrapper 上的方法，__inner 无法访问）
#[derive(Debug)]
pub(crate) enum VTableBodyKind {
    Safe,         // 直接放入 vtable impl
    NeedsWrapper, // 需要 wrapper 重建（含 Clone::clone，this 需要是 wrapper）
    Skip,         // 跳过（含 Self::，wrapper 重建也无法解决）
}

pub(crate) fn classify_vtable_body(block: &Block) -> VTableBodyKind {
    // 折叠空白：proc-macro 上下文的 to_string 保留源码换行 / 缩进，子串匹配须在规范化文本上做
    let s: String = quote!(#block).to_string().split_whitespace().collect::<Vec<_>>().join(" ");
    if s.contains("Self ::") || s.contains("Self::") {
        return VTableBodyKind::Skip;
    }
    let clone_bare_this = s.contains("Clone :: clone (this)")
        || s.contains("Clone :: clone(this)")
        || s.contains("Clone::clone (this)")
        || s.contains("Clone::clone(this)");
    if clone_bare_this {
        return VTableBodyKind::NeedsWrapper;
    }
    // this.method() 调用 non-__ 方法：可能是 native 方法（仅在 wrapper 上有实现）
    let mut parts: Vec<&str> = Vec::new();
    parts.extend(s.split("this .").skip(1));
    parts.extend(s.split("this.").skip(1));
    for part in parts {
        let trimmed = part.trim_start();
        let mname: String =
            trimmed.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if !mname.is_empty() && !mname.starts_with("__") {
            let rest = &trimmed[mname.len()..];
            if rest.trim_start().starts_with('(') {
                return VTableBodyKind::NeedsWrapper;
            }
        }
    }
    VTableBodyKind::Safe
}

/// VirtualOverride 安全性检查：委托给 classify_vtable_body。
pub(crate) fn is_vtable_safe_body(block: &Block) -> bool {
    matches!(classify_vtable_body(block), VTableBodyKind::Safe)
}

/// A-1 存储层擦除后的门控分类：泛型类（带类型形参）的方法体不走 vtable 直连（Safe）
/// 路径 —— `impl<P..> X__VTable<P..> for X__inner` 覆盖全部实例化，inner 上下文对
/// `this.__get_x()` / `this.vtable_method()` 的解析无法定实例化（E0283）。统一改走
/// wrapper 钩子（NeedsWrapper）：wrapper 的 vtable 槽位类型已知，签名不歧义。
pub(crate) fn vtable_body_kind_gated(block: &Block, class_is_generic: bool) -> VTableBodyKind {
    match classify_vtable_body(block) {
        VTableBodyKind::Safe if class_is_generic => VTableBodyKind::NeedsWrapper,
        other => other,
    }
}
