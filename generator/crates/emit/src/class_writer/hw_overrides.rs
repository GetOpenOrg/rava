//! 边界类手写覆盖的继承虚方法（← `class_writer._handwritten_inherited_overrides`）。
//!
//! 共置 `_impl.rs` 提供 `__impl_<m>`、而本类字节码未声明 m（声明在祖先，常为 abstract）时，
//! 按祖先声明合成本类的覆盖声明（去 abstract），使槽位走手写体而非祖先 trait default。
//! 仅限内部边界类（公开 API 命名空间的手写只许 native）；按名唯一匹配，重载歧义不合成。

use std::collections::BTreeSet;

use classfile::Method;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::lang;

const IMPL_PREFIX: &str = "__impl_";
const ACC_ABSTRACT: u16 = 0x0400;

/// 合成的覆盖声明：祖先声明的副本（access 去 abstract）；`owner` / `index` 指向祖先原方法
/// （局部变量表、Deprecated 等补充属性随副本取自祖先）
pub struct HwOverride<'c> {
    pub owner: &'c ClassInfo,
    pub index: usize,
    pub method: Method,
}

/// `visible` 为本类非 synthetic 方法
pub fn handwritten_inherited_overrides<'c>(ctx: &EmitCtx<'c>, ci: &ClassInfo, visible: &[&Method]) -> Vec<HwOverride<'c>> {
    let Some(hw) = ctx.input.handwritten.get(ci.name()) else { return Vec::new() };
    if ci.is_interface() || ctx.ty.reg.is_empty() || lang::in_public_api(ci.name()) {
        return Vec::new();
    }
    let declared: BTreeSet<&str> = visible.iter().map(|m| m.name.as_str()).collect();
    let mut wanted: Vec<&str> = hw
        .methods
        .iter()
        .filter_map(|n| n.strip_prefix(IMPL_PREFIX))
        .filter(|n| !declared.contains(n))
        .collect();
    wanted.sort_unstable();
    let mut out = Vec::new();
    for name in wanted {
        let mut found: Vec<(&ClassInfo, usize)> = Vec::new();
        let mut sup = ci.super_class();
        while let Some(sci) = ctx.ty.reg.get(sup) {
            found.extend(
                sci.methods()
                    .iter()
                    .enumerate()
                    .filter(|(_, m)| m.name == name && !m.is_static() && !m.is_synthetic() && !m.is_private())
                    .map(|(i, _)| (sci, i)),
            );
            if !found.is_empty() {
                break;
            }
            sup = sci.super_class();
        }
        if let [(owner, index)] = found.as_slice() {
            let mut method = owner.methods()[*index].clone();
            method.access &= !ACC_ABSTRACT;
            out.push(HwOverride { owner, index: *index, method });
        }
    }
    out
}
