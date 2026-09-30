//! 超类虚方法继承（← `class_writer._emit_superclass_virtual_inheritance` / `_slot_demanded_on_chain`）。
//!
//! 子类未覆盖祖先虚方法时补继承实现，使子类的祖先 vtable impl 含真实函数体：
//! - **用户祖先段**：祖先方法体按本类重发射（槽位归属经协变模型解析）；本类精确同签名的
//!   synthetic 桥是槽位覆盖的语义载体，优先翻译桥体；
//! - **JDK 祖先段**：不重发射字节码，按 (本类, 方法名, 参数描述符) 登记继承成员需求，
//!   由第二阶段（inherited_gen）统一生成转发成员。
//!
//! 链模式逐祖先判定（用户超类链可经用户祖先进入 JDK 祖先）。用户类直接继承 JDK 类时，
//! JDK 祖先段按 JDK 调用链门控（Python `_JDK_INHERIT_CHAIN`）。

use std::borrow::Cow;
use std::collections::BTreeSet;

use classfile::{acc, Method};
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use super::attrs::MethodAttrExtra;
use super::inherit::{chain_all, interface_special_member_name};
use super::methods::{BodySpec, Cx, Emitted};
use super::slot::override_vtable_erasure;
use crate::body::MethodBodyEmitter;
use crate::ctx::{EmitCtx, ProjectState};
use crate::error::Result;
use crate::vtable::param_part;

const ACC_BRIDGE: u16 = 0x0040;

/// 签名里的类型变量记号（`(?<![A-Za-z0-9_$/])T[A-Za-z0-9_$]+;`）
fn has_type_var_token(sig: &str) -> bool {
    let b = sig.as_bytes();
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c == b'$';
    (0..b.len()).any(|i| {
        if b[i] != b'T' || (i > 0 && (word(b[i - 1]) || b[i - 1] == b'/')) {
            return false;
        }
        let mut j = i + 1;
        while j < b.len() && word(b[j]) {
            j += 1;
        }
        j > i + 1 && j < b.len() && b[j] == b';'
    })
}

/// 本类及超类链上的 (名, 参数描述符) synthetic bridge 存在性（K-6b 见证）
fn chain_has_bridge(ctx: &EmitCtx<'_>, sub: &str, name: &str, pp: &str) -> bool {
    let mut seen = BTreeSet::new();
    let mut cur = ctx.ty.reg.get(sub);
    while let Some(c) = cur.filter(|c| seen.insert(c.name().to_string())) {
        if c.methods().iter().any(|b| {
            b.is_synthetic() && b.access & ACC_BRIDGE != 0 && !b.is_static() && b.name == name && param_part(&b.desc) == pp
        }) {
            return true;
        }
        cur = ctx.ty.reg.get(c.super_class());
    }
    false
}

/// (名, 参数描述符) 沿接收者超类链的最近声明能否按子类 `sub` 的槽位登记
/// （private / final 不登记；类型变量签名须有 bridge 见证）
fn virtually_dispatched(ctx: &EmitCtx<'_>, recv: &ClassInfo, name: &str, pp: &str, sub: &str) -> bool {
    let mut seen = BTreeSet::new();
    let mut cur = Some(recv);
    while let Some(c) = cur.filter(|c| seen.insert(c.name().to_string())) {
        if let Some(m) = c.methods().iter().find(|x| !x.is_synthetic() && x.name == name && x.desc.starts_with(pp)) {
            if m.access & (acc::PRIVATE | acc::FINAL) != 0 {
                return false;
            }
            if m.signature.as_deref().is_some_and(has_type_var_token) {
                return chain_has_bridge(ctx, sub, name, pp);
            }
            return true;
        }
        cur = if c.super_class().is_empty() { None } else { ctx.ty.reg.get(c.super_class()) };
    }
    true
}

/// 槽位需求门控：(方法名, 参数描述符) 在 ci 超类链任一声明层被调用链索要过，且可按 ci 登记
fn slot_demanded_on_chain(ctx: &EmitCtx<'_>, ci: &ClassInfo, vm: &Method) -> bool {
    let index = ctx.chain_slots();
    let slot = (vm.name.clone(), param_part(&vm.desc).to_string());
    let mut seen = BTreeSet::new();
    let mut cur = ci.super_class();
    while !cur.is_empty() && cur != ty::consts::OBJECT && seen.insert(cur) {
        let Some(c) = ctx.ty.reg.get(cur) else { break };
        if index.get(cur).is_some_and(|s| s.contains(&slot)) {
            return virtually_dispatched(ctx, c, &slot.0, &slot.1, ci.name());
        }
        cur = c.super_class();
    }
    false
}

/// 单个祖先方法的判定上下文
struct Anc<'c> {
    sci: &'c ClassInfo,
    index: usize,
    vm: &'c Method,
    user: bool,
    in_cc: bool,
}

/// 超类虚方法继承段
pub(super) fn superclass_virtual_inheritance(
    cx: &Cx<'_, '_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    visible: &[&Method],
    out: &mut Vec<String>,
) -> Result<()> {
    let (ctx, ci) = (cx.ctx, cx.ci);
    let sup0 = ci.super_class();
    if ci.is_interface() || sup0.is_empty() {
        return Ok(());
    }
    // 用户类且超类为用户类：全量（Python call_chain None）；否则按调用链门控
    let cc_all = chain_all(ctx, ci) && !sup0.contains('/');
    let mut existing: BTreeSet<(String, String)> =
        visible.iter().map(|m| (m.name.clone(), param_part(&m.desc).to_string())).collect();
    let mut done: BTreeSet<(String, String)> = BTreeSet::new();
    let mut sup = sup0;
    while !sup.is_empty() && sup != ty::consts::OBJECT {
        let Some(sci) = ctx.ty.reg.get(sup) else { break };
        let user = !sup.contains('/');
        let anc_hw = ctx.input.handwritten.get(sup).map(|h| &h.methods);
        for (index, vm) in sci.methods().iter().enumerate() {
            let virt = !vm.is_static() && vm.name != "<init>" && vm.name != "<clinit>";
            let has_exact = visible.iter().any(|m| m.name == vm.name && m.desc == vm.desc);
            let bridge = if virt && !has_exact {
                ci.methods().iter().position(|b| b.access & ACC_BRIDGE != 0 && !b.is_static() && b.name == vm.name && b.desc == vm.desc)
            } else {
                None
            };
            let anc_bridge_slot = bridge.is_none() && !user && vm.access & ACC_BRIDGE != 0 && !vm.is_static() && !has_exact;
            let key = (vm.name.clone(), param_part(&vm.desc).to_string());
            if existing.contains(&key) && ((bridge.is_none() && !anc_bridge_slot) || done.contains(&key)) {
                continue;
            }
            if !virt {
                continue;
            }
            let in_cc = cc_all || ctx.in_chain(ci.name(), &vm.name, &vm.desc) || ctx.in_chain(sup, &vm.name, &vm.desc);
            let impl_name = |n: &str| format!("__impl_{}", safe_ident(n));
            let hw_has = |n: &str| anc_hw.is_some_and(|h| h.contains(n));
            let mangled = mangle_name(&ctx.manifest.ty, &vm.name, &vm.desc);
            let hw_override = !vm.is_abstract() && (hw_has(&impl_name(&vm.name)) || hw_has(&impl_name(&mangled)));
            if !user && !in_cc && !hw_override && !slot_demanded_on_chain(ctx, ci, vm) {
                continue;
            }
            existing.insert(key.clone());
            if !done.insert(key.clone()) {
                continue;
            }
            if !user {
                // JDK 链：登记继承成员需求（第二阶段生成转发成员）
                let hand_hit = hw_has(&safe_ident(&vm.name)) || hw_has(&safe_ident(&mangled)) || hw_override;
                let register = if vm.is_abstract() { bridge.is_some() } else { !vm.is_native() || hand_hit };
                if register {
                    state.inherited_requests.insert((ci.name().to_string(), key.0, key.1));
                }
                continue;
            }
            let a = Anc { sci, index, vm, user, in_cc };
            if let Some(block) = user_ancestor_block(cx, state, bodies, visible, &a, bridge)? {
                out.push(block);
            }
        }
        sup = sci.super_class();
    }
    Ok(())
}

/// 用户祖先段：祖先虚方法（或本类桥）按本类重发射
fn user_ancestor_block(
    cx: &Cx<'_, '_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    visible: &[&Method],
    a: &Anc<'_>,
    bridge: Option<usize>,
) -> Result<Option<String>> {
    let (ctx, ci, vm) = (cx.ctx, cx.ci, a.vm);
    let virt_in = ctx.resolve_virtual_slot(vm, a.sci);
    if virt_in.is_empty() {
        return Ok(None);
    }
    if bridge.is_some()
        && visible
            .iter()
            .any(|m| !m.is_static() && m.name != "<init>" && m.name == vm.name && ctx.resolve_virtual_slot(m, ci) == virt_in)
    {
        return Ok(None); // 槽位已由本类可见覆盖经协变落位填充
    }
    let vm_is_bridge = vm.access & ACC_BRIDGE != 0;
    if vm_is_bridge && a.user && virt_in == ctx.short(a.sci.name()) {
        return Ok(None); // 祖先自身的桥只承载接口槽位
    }
    let own_short = ctx.short(ci.name());
    let mangled = mangle_name(&ctx.manifest.ty, &vm.name, &vm.desc);
    let mut extra = MethodAttrExtra { virtual_in: virt_in.clone(), ..Default::default() };
    if vm_is_bridge && virt_in != own_short {
        let slot = ctx.slot_member_rust_name(vm, ci);
        if !slot.is_empty() && slot != mangled {
            extra.vtable_name = slot;
        }
        extra.vtable_erasure = override_vtable_erasure(ctx, ci, vm, &virt_in);
    }
    let e = Emitted { method: Cow::Borrowed(vm), owner: a.sci, index: a.index };
    let attr = cx.attr(&e, &extra);
    if let Some(bi) = bridge.filter(|_| a.in_cc) {
        let b = &ci.methods()[bi];
        let base = bridge_wrapper_name(ctx, visible, b, &virt_in, a.sci.name());
        let mut bx = MethodAttrExtra { virtual_in: virt_in.clone(), ..Default::default() };
        if virt_in != own_short {
            let slot = ctx.slot_member_rust_name(vm, ci);
            if !slot.is_empty() && slot != base {
                bx.vtable_name = slot;
            }
            bx.vtable_erasure = override_vtable_erasure(ctx, ci, vm, &virt_in);
        }
        let be = Emitted::declared(ci, bi);
        let spec = BodySpec { ctparams: cx.tps, rust_name: Some(&base), in_vtable_body: true, view: None };
        if let Some(text) = cx.body_with(state, bodies, &be, &spec)? {
            return Ok(Some(format!("{}\n{text}", cx.attr(&be, &bx))));
        }
        if visible.iter().any(|m| m.name == vm.name && param_part(&m.desc) == param_part(&vm.desc)) {
            return Ok(None);
        }
    }
    let text = if vm.is_native() || vm.is_abstract() || !a.in_cc {
        None
    } else {
        let spec = BodySpec { ctparams: cx.tps, rust_name: None, in_vtable_body: true, view: None };
        cx.body_with(state, bodies, &e, &spec)?
    };
    let text = text.unwrap_or_else(|| cx.stub(&e, "", cx.tps).text);
    Ok(Some(format!("{attr}\n{text}")))
}

/// 桥 wrapper 名：与可见方法同名即 mangle；同名同参（仅返回不同）取 `Iface_super_m` 形态唯一名
fn bridge_wrapper_name(ctx: &EmitCtx<'_>, visible: &[&Method], b: &Method, virt_in: &str, sup: &str) -> String {
    if !visible.iter().any(|m| m.name == b.name) {
        return b.name.clone();
    }
    if visible.iter().any(|m| m.name == b.name && param_part(&m.desc) == param_part(&b.desc)) {
        let owner_bin = ctx.ty.names.binary_of(virt_in).unwrap_or(sup);
        if let Some(owner) = ctx.ty.reg.get(owner_bin) {
            return safe_ident(&interface_special_member_name(ctx, owner, &b.name, &b.desc));
        }
        return safe_ident(&format!("{}_super_{}", ctx.short(owner_bin), b.name));
    }
    mangle_name(&ctx.manifest.ty, &b.name, &b.desc)
}

#[cfg(test)]
mod tests {
    use super::has_type_var_token;

    #[test]
    fn type_var_token() {
        assert!(has_type_var_token("(TP_IN;)V"));
        assert!(!has_type_var_token("(Ljava/lang/Test;)V"));
        assert!(has_type_var_token("(Ljava/util/List<TT;>;)V"));
        assert!(!has_type_var_token("()V"));
    }
}
