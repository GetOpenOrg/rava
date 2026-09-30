//! `_lookup_method_sig_params`：被调方法的真实形参类型。

use std::collections::BTreeSet;

use ty::{ClassInfo, JvmType, RsType};

use super::{handwritten_boundary_method, iface_view_targ_map, jvm, receiver_type_arg_map, resolve_cls, RecvView, TargMap};
use crate::ctx::InstrCtx;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::CallRef;

fn declares(ci: &ClassInfo, mname: &str, desc: &str) -> bool {
    ci.methods().iter().any(|m| m.name == mname && m.desc == desc)
}

/// 类型是否为 callee 的裸类型形参
fn tparam_of<'t>(t: &'t RsType, tparams: &[String]) -> Option<&'t str> {
    match t {
        RsType::Param(n) if tparams.contains(n) => Some(n.as_str()),
        _ => None,
    }
}

/// JVM 方法解析：常量池类未声明 → 超类链最近声明者；再查接口闭包（广度）。
/// 声明者变化时按接收者静态类型重算实参映射
fn resolve_declarer<'a>(
    ctx: &InstrCtx<'a>,
    mut ci: &'a ClassInfo,
    call: &CallRef,
    recv_ty: Option<&RsType>,
    targ: &mut Option<TargMap>,
) -> &'a ClassInfo {
    let reg = ctx.reg();
    let (mname, desc) = (call.name.as_str(), call.desc.as_str());
    if mname == "<init>" || declares(ci, mname, desc) {
        return ci;
    }
    let mut seen = BTreeSet::from([ci.name().to_string()]);
    let mut cur = if ci.super_class().is_empty() { None } else { reg.get(ci.super_class()) };
    while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
        seen.insert(c.name().to_string());
        if declares(c, mname, desc) {
            ci = c;
            if let Some(rt) = recv_ty {
                *targ = receiver_type_arg_map(ctx, rt, ci.name());
            }
            break;
        }
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    if declares(ci, mname, desc) {
        return ci;
    }
    let mut queue: std::collections::VecDeque<String> = ci.interfaces().iter().cloned().collect();
    while let Some(ifn) = queue.pop_front() {
        if seen.contains(&ifn) {
            continue;
        }
        let Some(ic) = reg.get(&ifn) else {
            continue;
        };
        seen.insert(ifn);
        if declares(ic, mname, desc) {
            if let Some(rt) = recv_ty {
                if let Some(m) = receiver_type_arg_map(ctx, rt, ic.name()).or_else(|| iface_view_targ_map(ctx, rt, ic)) {
                    *targ = Some(m);
                }
            }
            return ic;
        }
        queue.extend(ic.interfaces().iter().cloned());
    }
    ci
}

fn iface_head(ctx: &InstrCtx, t: &RsType) -> bool {
    matches!(jvm(ctx, t), JvmType::Class { is_interface: true, .. })
}

pub(super) fn lookup(env: &InstrEnv, call: &CallRef, caller_tparams: &[String], recv: RecvView) -> InstrResult<Option<Vec<Option<RsType>>>> {
    let ctx = &env.ctx;
    if call.owner.is_empty() || ctx.reg().is_empty() {
        return Ok(None);
    }
    let Some(cls_bin) = resolve_cls(ctx, &call.owner) else {
        return Ok(None);
    };
    let Some(ci0) = ctx.reg().get(&cls_bin) else {
        return Ok(None);
    };
    let mut targ = recv.targ_map.cloned();
    let ci = resolve_declarer(ctx, ci0, call, recv.ty, &mut targ);
    let Some(m) = ci.methods().iter().find(|m| m.name == call.name && m.desc == call.desc) else {
        return Ok(None);
    };
    let callee_tparams = ctx.ty.effective_class_type_params(ci);
    let hw = handwritten_boundary_method(ctx, &cls_bin, &call.name, &call.desc)?;
    let mut types = ctx.ty.method_sig_types(ci, m, &callee_tparams).params;
    if types.is_empty() && !hw {
        types = ctx.ty.emitted_method_sig_types(ci, m, &callee_tparams).params;
    }
    if types.is_empty() {
        return Ok(None);
    }
    let targ = targ.filter(|t| !t.is_empty());
    if let Some(map) = &targ {
        types = types.into_iter().map(|t| if tparam_of(&t, &callee_tparams).is_some() { t } else { t.substitute(&|n| map.get(n).cloned()) }).collect();
    } else if ci.is_interface() && !recv.is_this && !callee_tparams.is_empty() {
        types = types
            .into_iter()
            .map(|t| if tparam_of(&t, &callee_tparams).is_some() { t } else { t.substitute(&|n| callee_tparams.contains(&n.to_string()).then_some(RsType::Object)) })
            .collect();
    }
    let mut resolved: Vec<Option<RsType>> = Vec::with_capacity(types.len());
    let mut subst_pos = BTreeSet::new();
    for (i, t) in types.into_iter().enumerate() {
        let Some(name) = tparam_of(&t, &callee_tparams).map(str::to_string) else {
            resolved.push(Some(t));
            continue;
        };
        let mapped = targ.as_ref().and_then(|m| m.get(&name));
        if ci.is_interface() && !recv.is_this && mapped.is_none() {
            resolved.push(Some(RsType::Object));
        } else if let Some(a) = mapped {
            resolved.push(Some(a.clone()));
            subst_pos.insert(i);
        } else if caller_tparams.contains(&name) {
            resolved.push(Some(t));
        } else {
            resolved.push(None);
        }
    }
    let desc_rust = |i: usize| call.params.get(i).map(|p| ctx.ty.jvm_to_rust(p));
    let mut out = Vec::with_capacity(resolved.len());
    for (i, t) in resolved.into_iter().enumerate() {
        if hw && i < call.params.len() {
            let d = desc_rust(i);
            let hit = match &t {
                Some(t) => iface_head(ctx, t),
                None => d.as_ref().is_some_and(|d| iface_head(ctx, d)),
            };
            if hit {
                out.push(Some(RsType::Object));
                continue;
            }
        }
        match t {
            Some(t) if iface_head(ctx, &t) => {
                // 描述符擦除头与形参头同一（erasure 相等）；越界位按空串（Object 以外）处理
                let same_head = match desc_rust(i) {
                    Some(d) => jvm(ctx, &d).erasure() == jvm(ctx, &t).erasure(),
                    None => false,
                };
                let keep_carrier = ctx.ty.is_carrier(&t) && subst_pos.contains(&i) && !(recv.is_this && ci.is_interface());
                out.push((keep_carrier || same_head).then_some(t));
            }
            other => out.push(other),
        }
    }
    Ok(Some(out))
}
