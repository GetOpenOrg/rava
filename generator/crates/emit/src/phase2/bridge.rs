//! synthetic 桥接方法的解析与转发成员（← `instr.member_owner._resolve_bridge_target` /
//! `_bridge_call_target` / `_resolve_method_owner`、`inherited_gen.resolve_bridge_member` /
//! `_covariant_bridge_pending` / `_bridge_override_member`）。
//!
//! `_resolve_bridge_target` 一族是方法体层（instr）的函数；方法体层尚未移植，这里是发射层
//! 自用的私有等价实现，方法体 crate 落地后统一到其实现。

use std::collections::{BTreeSet, VecDeque};

use classfile::{acc, Method, Operand};
use ty::ident::safe_ident;
use ty::type_map::{parse_descriptor_params, parse_descriptor_return};
use ty::ClassInfo;

use super::sig::{param_mapping, param_part, result_inner, sig_param_types, substitute_type_params};
use super::{anc_args, class_params, Emissions};
use crate::class_writer::inherit::interface_special_member_name;
use crate::class_writer::slot::override_vtable_erasure;
use crate::ctx::EmitCtx;
use crate::emission::{ClassEmission, EmittedMethod};
use crate::lang;

fn ret_part(desc: &str) -> &str {
    desc.find(')').map_or("", |i| &desc[i + 1..])
}

/// 沿继承链解析 mname 的声明类（参数部分前缀匹配，排除 synthetic）；整条链没有 → None
fn resolve_method_owner<'c>(ctx: &EmitCtx<'c>, cls: &str, mname: &str, desc: &str) -> Option<&'c ClassInfo> {
    let pp = if desc.is_empty() { "" } else { param_part(desc) };
    let mut ci = ctx.ty.reg.get(cls);
    while let Some(c) = ci {
        let found =
            c.methods().iter().any(|m| !m.is_synthetic() && m.name == mname && (pp.is_empty() || m.desc.starts_with(pp)));
        if found {
            return Some(c);
        }
        let sc = c.super_class();
        if sc.is_empty() || sc == lang::OBJECT {
            break;
        }
        ci = ctx.ty.reg.get(sc);
    }
    None
}

/// bridge 方法体里被桥接的真实方法：(声明类, 真实描述符)
fn bridge_call_target<'c>(ctx: &EmitCtx<'c>, owner: &ClassInfo, bridge: &Method, mname: &str) -> Option<(&'c ClassInfo, String)> {
    let code = ctx.input.code_ops(owner.name(), bridge)?;
    for insn in code.ops().rev() {
        let Operand::Method(r, _) = &insn.operand else { continue };
        if r.name != mname {
            continue;
        }
        let real_params = param_part(&r.desc);
        let declared = |c: &'c ClassInfo| {
            c.methods().iter().find(|m| !m.is_synthetic() && m.name == mname && m.desc.starts_with(real_params))
        };
        let mut owner_ci = ctx.ty.reg.get(&r.owner);
        let mut real = owner_ci.and_then(declared);
        if real.is_none() {
            owner_ci = resolve_method_owner(ctx, &r.owner, mname, &r.desc);
            real = owner_ci.and_then(declared);
        }
        return real.map(|m| (owner_ci.expect("声明类"), m.desc.clone()));
    }
    None
}

/// 找描述符精确命中的 synthetic bridge（先父类链、再接口闭包），返回被桥接的 (声明类, 真实描述符)
pub fn resolve_bridge_target<'c>(ctx: &EmitCtx<'c>, ci: &'c ClassInfo, mname: &str, desc: &str) -> Option<(&'c ClassInfo, String)> {
    let reg = ctx.ty.reg;
    let bridge_in = |c: &'c ClassInfo| c.methods().iter().find(|m| m.is_synthetic() && m.name == mname && m.desc == desc);
    let mut seen = BTreeSet::new();
    let mut pending: VecDeque<String> = VecDeque::new();
    let mut cur = Some(ci);
    while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
        seen.insert(c.name().to_string());
        if let Some(b) = bridge_in(c) {
            return bridge_call_target(ctx, c, b, mname);
        }
        pending.extend(c.interfaces().iter().cloned());
        cur = (!c.super_class().is_empty()).then(|| reg.get(c.super_class())).flatten();
    }
    while let Some(i) = pending.pop_front() {
        let Some(ici) = reg.get(&i) else { continue };
        if !seen.insert(ici.name().to_string()) {
            continue;
        }
        if let Some(b) = bridge_in(ici) {
            return bridge_call_target(ctx, ici, b, mname);
        }
        pending.extend(ici.interfaces().iter().cloned());
    }
    None
}

/// 桥接解析结果（生成侧与接口 impl 的 target 预测共用，名字必须同源）
pub struct BridgeMember<'c> {
    /// wrapper 成员名（槽位声明者名；纯接口桥接按接收者视角）
    pub member_name: String,
    /// vtable 槽位声明类 binary（纯接口桥接为空）
    pub vt_bin: String,
    /// 真实方法签名与调用名（接收者视角）
    pub real_sig: String,
    pub real_rust: String,
    /// 真实方法在祖先时的补充需求 (名, 参数描述符)
    pub real_want: Option<(String, String)>,
    pub bridge: &'c Method,
    /// 协变桥的槽位名（wrapper 取唯一名时指回声明者）
    pub vtable_name: String,
}

fn find_slot<'e>(em: &'e ClassEmission, name: &str, param_desc: &str, covariant_desc: Option<&str>) -> Option<&'e EmittedMethod> {
    match covariant_desc {
        Some(d) => em.methods.iter().find(|m| m.name == name && m.descriptor == d),
        None => em.find(name, param_desc),
    }
}

/// 槽位归属沿声明链上溯到根声明者（K-6a 覆盖传递性）：返回 (槽位类, 槽位声明)
fn climb_slot<'e>(
    ctx: &EmitCtx<'_>,
    ems: &'e Emissions,
    start: &str,
    found: &'e EmittedMethod,
    name: &str,
    param_desc: &str,
    cov: Option<&str>,
) -> (String, &'e EmittedMethod) {
    let reg = ctx.ty.reg;
    let (mut slot_cur, mut slot_m) = (start.to_string(), found);
    while !slot_m.virtual_in.is_empty() && slot_m.virtual_in != slot_cur {
        let mut up = slot_cur.clone();
        let mut up_seen = BTreeSet::new();
        let mut nxt = None;
        while !up.is_empty() && up != lang::OBJECT && reg.contains(&up) && up_seen.insert(up.clone()) {
            if up == slot_m.virtual_in {
                nxt = Some(up.clone());
                break;
            }
            up = reg.get(&up).map(|c| c.super_class().to_string()).unwrap_or_default();
        }
        let Some(nxt) = nxt else { break };
        let up_found = ems.get(&nxt).filter(|e| !e.handwritten).and_then(|e| find_slot(e, name, param_desc, cov));
        let Some(uf) = up_found else { break };
        slot_cur = nxt;
        slot_m = uf;
    }
    (slot_cur, slot_m)
}

/// (name, param_desc) 的 synthetic 桥接解析核心（JVM 方法解析顺序：桥接优先）；
/// 无桥接 / 不可解析 → None（回落普通继承路径）
pub fn resolve_bridge_member<'c>(
    ctx: &EmitCtx<'c>,
    recv_ci: &'c ClassInfo,
    name: &str,
    param_desc: &str,
    ems: &Emissions,
    recv: Option<&ClassEmission>,
    allow_covariant: bool,
) -> Option<BridgeMember<'c>> {
    let reg = ctx.ty.reg;
    let mut seen = BTreeSet::new();
    let mut cur = Some(recv_ci);
    let mut hit: Option<(&'c ClassInfo, &'c Method)> = None;
    while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
        seen.insert(c.name().to_string());
        if let Some(b) = c.methods().iter().find(|m| m.is_synthetic() && m.name == name && m.desc.starts_with(param_desc)) {
            hit = Some((c, b));
            break;
        }
        cur = (!c.super_class().is_empty()).then(|| reg.get(c.super_class())).flatten();
    }
    let (bridge_cls, bridge) = hit?;
    let bridge_in_recv = std::ptr::eq(bridge_cls, recv_ci);
    let mut target = resolve_bridge_target(ctx, recv_ci, name, &bridge.desc);
    if !bridge_in_recv && allow_covariant {
        let own_real = recv_ci.methods().iter().find(|m| {
            !m.is_synthetic() && !m.is_static() && m.name == name && m.desc.starts_with(param_desc) && m.desc != bridge.desc
        });
        if let Some(o) = own_real {
            target = Some((recv_ci, o.desc.clone()));
        }
    }
    let (real_owner_ci, real_desc) = target?;
    let real_owner_is_recv = std::ptr::eq(real_owner_ci, recv_ci) || real_owner_ci.name() == recv_ci.name();
    let real_param = param_part(&real_desc).to_string();
    let covariant = real_param == param_desc && ret_part(&real_desc) != ret_part(&bridge.desc);
    if covariant {
        if !allow_covariant || !real_owner_is_recv {
            return None;
        }
    } else if real_param == param_desc
        || (ret_part(&real_desc) != ret_part(&bridge.desc) && (!allow_covariant || !bridge_in_recv || !real_owner_is_recv))
    {
        return None;
    }

    let (real_sig, real_rust, real_want) = match recv.and_then(|r| r.find(name, &real_param)) {
        Some(m) => (m.signature(ctx.ty.names), m.rust_name.clone(), None),
        None => {
            let owner_em = ems.get(real_owner_ci.name()).filter(|e| !e.handwritten)?;
            let found = owner_em.find(name, &real_param)?;
            let owner_args = anc_args(ctx, recv_ci).into_iter().find(|(b, _)| b == real_owner_ci.name()).map(|(_, a)| a);
            let mapping = param_mapping(&class_params(ctx, real_owner_ci), &owner_args.unwrap_or_default());
            let sig = substitute_type_params(&found.signature(ctx.ty.names), &mapping);
            let rust = ctx.ty.receiver_member_name(&found.name, &found.descriptor, recv_ci);
            (sig, rust, Some((name.to_string(), real_param.clone())))
        }
    };

    let cov = covariant.then_some(bridge.desc.as_str());
    let (mut vt_bin, mut member_name) = (String::new(), String::new());
    let mut cur = recv_ci.super_class().to_string();
    let mut seen = BTreeSet::new();
    while !cur.is_empty() && cur != lang::OBJECT && reg.contains(&cur) && seen.insert(cur.clone()) {
        if let Some(found) = ems.get(&cur).filter(|e| !e.handwritten).and_then(|e| find_slot(e, name, param_desc, cov)) {
            let (slot_cur, slot_m) = climb_slot(ctx, ems, &cur, found, name, param_desc, cov);
            member_name = if slot_m.vtable_name.is_empty() { slot_m.rust_name.clone() } else { slot_m.vtable_name.clone() };
            vt_bin = slot_cur;
            break;
        }
        cur = reg.get(&cur).map(|c| c.super_class().to_string()).unwrap_or_default();
    }
    if member_name.is_empty() {
        member_name = ctx.ty.interface_member_local_name(recv_ci, name, &bridge.desc);
    }
    let mut vtable_name = String::new();
    if covariant && vt_bin.is_empty() {
        return None;
    }
    if member_name == real_rust {
        if !covariant {
            return None;
        }
        vtable_name = member_name;
        let slot_ci = reg.get(&vt_bin)?;
        member_name = safe_ident(&interface_special_member_name(ctx, slot_ci, name, &bridge.desc));
        if member_name == real_rust {
            return None;
        }
    }
    Some(BridgeMember { member_name, vt_bin, real_sig, real_rust, real_want, bridge, vtable_name })
}

/// 本类的协变返回覆盖已声明 (name, param_desc)，但同参异返回的 ACC_BRIDGE 桥承担的是另一个
/// 祖先槽位——该槽位仍待桥成员填充
pub fn covariant_bridge_pending(ctx: &EmitCtx<'_>, recv_ci: &ClassInfo, recv_bin: &str, own: &EmittedMethod, name: &str, param_desc: &str) -> bool {
    let reg = ctx.ty.reg;
    let own_desc = own.descriptor.as_str();
    if !recv_ci.methods().iter().any(|m| !m.is_synthetic() && m.name == name && m.desc == own_desc) {
        return false;
    }
    let bridges_of = |c: &ClassInfo| -> Vec<String> {
        c.methods()
            .iter()
            .filter(|m| m.access & acc::BRIDGE != 0 && m.name == name && !m.is_static() && m.desc.starts_with(param_desc) && m.desc != own_desc)
            .map(|m| m.desc.clone())
            .collect()
    };
    let mut bridges = bridges_of(recv_ci);
    if !bridges.is_empty() && own.virtual_in == recv_bin {
        return true;
    }
    if bridges.is_empty() {
        let mut cur = recv_ci.super_class().to_string();
        let mut seen = BTreeSet::new();
        while bridges.is_empty() && !cur.is_empty() && seen.insert(cur.clone()) {
            let Some(c) = reg.get(&cur) else { break };
            bridges = bridges_of(c);
            cur = c.super_class().to_string();
        }
        if bridges.is_empty() {
            return false;
        }
    }
    let slot_root = |desc: &str| -> String {
        let mut root = String::new();
        let mut cur = recv_ci.super_class().to_string();
        let mut seen = BTreeSet::new();
        while !cur.is_empty() && seen.insert(cur.clone()) {
            let Some(sci) = reg.get(&cur) else { break };
            if sci.methods().iter().any(|m| m.name == name && m.desc == desc && !m.is_static()) {
                root = cur.clone();
            }
            cur = sci.super_class().to_string();
        }
        root
    };
    bridges.iter().any(|b| {
        let r = slot_root(b);
        !r.is_empty() && r != own.virtual_in
    })
}

/// 桥接转发成员声明：(声明文本, 导入扫描用签名文本, 真实方法的补充需求)
pub struct BridgeDecl {
    pub decl: String,
    pub sig_text: String,
    pub real_want: Option<(String, String)>,
}

/// 接收者类（或其超类链）的 synthetic bridge → bridge 转发成员声明（体在 self 上调用真实方法，
/// 以 `virtual_in` 填声明类的 vtable 槽位）；无 bridge / 不可解析 → None
pub fn bridge_override_member(
    ctx: &EmitCtx<'_>,
    name: &str,
    param_desc: &str,
    recv: &ClassEmission,
    recv_ci: &ClassInfo,
    ems: &Emissions,
) -> Option<BridgeDecl> {
    let r = resolve_bridge_member(ctx, recv_ci, name, param_desc, ems, Some(recv), true)?;
    let names = ctx.ty.names;
    let bridge = r.bridge;
    let eff = class_params(ctx, recv_ci);
    let es = ctx.ty.emitted_method_sig_types(recv_ci, bridge, &eff);
    let param_tys: Vec<String> = if es.params.is_empty() {
        parse_descriptor_params(&bridge.desc).iter().map(|p| ctx.ty.jvm_to_rust(p).render(names)).collect()
    } else {
        es.params.iter().map(|t| t.render(names)).collect()
    };
    let ret_desc = parse_descriptor_return(&bridge.desc);
    let sig_ret = es.ret.render(names);
    let ret_ty = if ret_desc == "V" {
        "()".to_string()
    } else if !sig_ret.is_empty() && sig_ret != "()" {
        sig_ret
    } else {
        ctx.ty.jvm_to_rust(ret_desc).render(names)
    };
    let params: Vec<String> = param_tys.iter().enumerate().map(|(i, t)| format!("arg{i}: {t}")).collect();
    let signature = format!("pub fn {}(&self, {}) -> Result<{ret_ty}>", r.member_name, params.join(", "));

    let (real_param_tys, real_ret) = sig_param_types(&r.real_sig);
    let call_args: Vec<String> = param_tys
        .iter()
        .zip(&real_param_tys)
        .enumerate()
        .map(|(i, (bt, rt))| {
            let arg = format!("arg{i}");
            if bt == rt {
                arg
            } else if bt == "Object" {
                format!("<{rt} as ::std::convert::From<Object>>::from({arg})")
            } else {
                format!("<{rt} as ::std::convert::From<Object>>::from(<Object as ::std::convert::From<{bt}>>::from({arg}))")
            }
        })
        .collect();
    let call = format!("this.{}({})", r.real_rust, call_args.join(", "));
    let body = if ret_desc == "V" {
        format!("let this = self; {call}?; Ok(())")
    } else if result_inner(&real_ret) == Some(ret_ty.as_str()) {
        format!("let this = self; Ok({call}?)")
    } else {
        format!("let this = self; Ok(::std::convert::Into::<{ret_ty}>::into({call}?))")
    };

    let mut parts = vec![format!("name = \"{name}\""), format!("descriptor = \"{}\"", bridge.desc)];
    if bridge.access & acc::PUBLIC != 0 {
        parts.push("access = \"public\"".into());
    } else if bridge.access & acc::PROTECTED != 0 {
        parts.push("access = \"protected\"".into());
    }
    if !r.vt_bin.is_empty() {
        parts.push(format!("virtual_in = \"{}\"", ctx.short(&r.vt_bin)));
        if !r.vtable_name.is_empty() {
            parts.push(format!("vtable_name = \"{}\"", r.vtable_name));
        }
        let erasure = override_vtable_erasure(ctx, recv_ci, bridge, &r.vt_bin);
        if !erasure.is_empty() {
            parts.push(format!("vtable_erasure = \"{}\"", erasure.join(";")));
        }
    }
    let decl = format!("#[java_method({})]\n{signature} {{ {body} }}", parts.join(", "));
    Some(BridgeDecl { sig_text: decl.clone(), decl, real_want: r.real_want })
}
