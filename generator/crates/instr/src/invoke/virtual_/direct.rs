//! 非 bare 接收者：调用形态解析（`_resolve_direct_call_sig`）与调用发射 / 结果记录
//! （`_emit_call_result` / `_call_node` / `_chain_declares`）。

use std::collections::BTreeSet;

use ir::{Expr, LetStmt, Stmt};
use sim::StackSim;
use ty::RsType;

use super::bare::boxed_text;
use super::cs::CsCtx;
use super::{erased_head, is_object, is_prim, raw, same_text, Site, O};
use crate::build::{id, text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy;
use crate::invoke::{sig, CallRef};
use crate::log::{Audit, InstrLog};
use crate::owner;

/// 解析后的调用形态
pub(super) struct DirectSig {
    /// 调用（形参 / 返回按接收者真实声明或桥接目标重解析；owner 不变）
    call: CallRef,
    rust_ret: RsType,
    /// 返回类型解析所用的声明类 binary 与接收者形态
    sig_owner: String,
    sig_recv_ty: RsType,
    /// 接收者文本（根路由时为 `Object::from(Clone::clone(&obj))`）
    recv: String,
    /// 调用路由到根 vtable（返回类型由调用点描述符决定）
    root_routed: bool,
}

impl DirectSig {
    fn redesc(&mut self, env: &InstrEnv, desc: &str) {
        self.call = CallRef::with(&self.call.owner, &self.call.name, desc);
        self.rust_ret = env.ctx.ty.jvm_to_rust(&self.call.ret);
    }
}

/// 接收者类型的实参化祖先形态 `{Owner}<..>`（`ancestor_vtable_args_by_short` 查不到 → 无实参）
fn owner_view(env: &InstrEnv, recv_ci: &ty::ClassInfo, obj_ty: &RsType, owner_bin: &str) -> RsType {
    let args = env.ctx.ty.ancestor_vtable_args_by_short(recv_ci, obj_ty).remove(&env.ctx.short(owner_bin)).unwrap_or_default();
    RsType::class(owner_bin.to_string(), args)
}

/// vtable body 内 this 直调 / 继承成员登记 / 根 vtable 路由 / bridge 真实方法重解析
pub(super) fn resolve_direct_call_sig(
    env: &InstrEnv,
    sim: &StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    rust_ret: RsType,
    obj_base: &str,
    site: &Site,
) -> InstrResult<DirectSig> {
    let class_name = env.ctx.class_name;
    let reg = env.ctx.reg();
    let mut d = DirectSig {
        call: call.clone(),
        rust_ret,
        sig_owner: call.owner.clone(),
        sig_recv_ty: site.obj_ty.clone(),
        recv: site.obj_e.clone(),
        root_routed: false,
    };
    if reg.is_empty() {
        return Ok(d);
    }
    // this 接收者：取当前类 binary（短名可能跨包重名）
    let this_recv = sim.cfg.in_vtable_body
        && !class_name.is_empty()
        && obj_base == env.ctx.short(class_name)
        && matches!(site.obj_e.as_str(), "this" | "self")
        && reg.contains(class_name);
    let obj_jvm = if this_recv { Some(class_name.to_string()) } else { hierarchy::short_binary(&env.ctx, obj_base) };
    let Some((obj_jvm, ci_recv)) = obj_jvm.and_then(|b| reg.get(&b).map(|ci| (b, ci))) else {
        return Ok(d);
    };
    let mname = call.name.as_str();
    let mut pdesc = call.param_desc().to_string();
    let real = ci_recv.methods().iter().find(|m| !m.is_synthetic() && m.name == mname && m.desc.starts_with(&pdesc));
    if let Some(real_m) = real {
        // 协变返回：调用描述符命中合成桥，生成的 Rust 方法是真实方法 → 返回类型按真实方法
        if real_m.desc != format!("{pdesc}{}", call.ret) {
            d.redesc(env, &real_m.desc);
            d.sig_owner = obj_jvm.clone();
            d.sig_recv_ty = site.obj_ty.clone();
        }
        return Ok(d);
    }
    let jvm_desc = format!("{pdesc}{}", call.ret);
    let owner_bin = owner::resolve_method_owner(reg, &obj_jvm, mname, &jvm_desc).map(|(o, _)| o);
    let key = (mname.to_string(), pdesc.clone());
    if let Some(ob) = owner_bin.as_ref().filter(|o| **o != obj_jvm) {
        // 返回类型按 owner 在接收者静态类型下的实参化形态解析
        d.sig_recv_ty = owner_view(env, ci_recv, &site.obj_ty, ob);
        d.sig_owner = ob.clone();
        log.inherited(&obj_jvm, mname, &pdesc);
    } else if owner_bin.is_none()
        && (env.ctx.facts.root_virtual.contains(&key)
            || (env.ctx.facts.root_protected_void.contains(&key) && env.ctx.facts.root_api.contains(mname)))
    {
        // 整条祖先链未声明、由根类声明 → 装箱后走根 vtable（Object::from 保持接收者 vtable）
        // [equiv-audit] identity-hash：链上无人声明 hashCode → 恒走根 vtable 默认实现
        if mname == "hashCode" {
            log.audit(Audit::IdentityHash);
        }
        d.recv = format!("{O}::from(Clone::clone(&{}))", site.obj_e);
        d.root_routed = true;
    } else {
        // 超类链上无字节码声明：只命中合成桥 → 按被桥接真实方法的参数登记；否则为接口方法
        let bridged = owner::resolve_bridge_target(reg, ci_recv, mname, &jvm_desc);
        if let Some((b_bin, b_desc)) = &bridged {
            pdesc = owner::param_part(b_desc).to_string();
            let b_iface = reg.get(b_bin).is_some_and(|c| c.is_interface());
            if !b_iface {
                // 返回类型 / 签名查找按被桥接的真实方法（与生成的 Rust 方法同源）
                d.redesc(env, b_desc);
                d.sig_owner = b_bin.clone();
                if *b_bin != obj_jvm {
                    d.sig_recv_ty = owner_view(env, ci_recv, &site.obj_ty, b_bin);
                }
            }
        }
        if bridged.as_ref().is_none_or(|(b, _)| *b != obj_jvm) {
            log.inherited(&obj_jvm, mname, &pdesc);
        }
    }
    Ok(d)
}

/// 接收者静态类型的类链（本类及超类，注册表内）是否声明实例方法 mname
fn chain_declares(env: &InstrEnv, obj_ty: &RsType, mname: &str) -> bool {
    let reg = env.ctx.reg();
    if reg.is_empty() {
        return false;
    }
    let head = erased_head(env, obj_ty);
    let mut cur = if head.is_empty() { None } else { hierarchy::short_binary(&env.ctx, &head) };
    let mut seen = BTreeSet::new();
    while let Some(c) = cur.and_then(|b| reg.get(&b)) {
        if !seen.insert(c.name().to_string()) {
            break;
        }
        if c.methods().iter().any(|m| m.name == mname && !m.is_static()) {
            return true;
        }
        cur = Some(c.super_class().to_string()).filter(|s| !s.is_empty());
    }
    false
}

/// 调用节点 `recv.m(args)`；caller-sensitive 包装生效时 None（沿用字符串形态）
fn call_node(env: &InstrEnv, cs: &CsCtx, rust_mname: &str, recv: &str, site: &Site) -> InstrResult<Option<Expr>> {
    if cs.applies(env) {
        return Ok(None);
    }
    let is_ident = recv.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && recv.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    let rn = if let Some(i) = is_ident.then(|| ir::Ident::new(recv).ok()).flatten() {
        Expr::Var(i)
    } else if is_ident {
        Expr::raw(recv.to_string())
    } else if text(env, &site.obj_node) == recv {
        site.obj_node.clone()
    } else {
        Expr::raw(recv.to_string())
    };
    Ok(Some(Expr::method(rn, id(rust_mname)?, site.arg_nodes.clone())))
}

/// `let v = call?;`：节点可用时结构化，否则 Raw
fn let_call(sim: &mut StackSim, v: ir::Ident, node: Option<Expr>, call_str: &str) {
    match node {
        Some(n) => sim.emit(Stmt::Let(LetStmt::new(v, None, Some(Expr::try_(n))))),
        None => raw(sim, format!("let {v} = {call_str}?;")),
    }
}

/// 调用发射与结果记录：void 直发 / 根路由 / clone 特例 / 签名真实返回类型与擦除类型的对齐
#[allow(clippy::too_many_arguments)]
pub(super) fn emit_call_result(
    env: &InstrEnv,
    sim: &mut StackSim,
    cs: &CsCtx,
    orig: &CallRef,
    site: &Site,
    obj_is_bare: bool,
    rust_mname: &str,
    d: DirectSig,
) -> InstrResult<()> {
    let arg_str = site.arg_str();
    let rust_ret = &d.rust_ret;
    // bare Object 接收者只在注册表为空时走到这里（有注册表时已由 bare 多态分派接管）：
    // 无可分派实现的 void / 具体引用返回形 → 精确存根
    if obj_is_bare && (*rust_ret == RsType::Unit || (!is_object(env, rust_ret) && !is_prim(rust_ret))) {
        return super::bare::unresolved_stub(env, sim, orig, rust_ret);
    }
    if *rust_ret == RsType::Unit {
        match call_node(env, cs, rust_mname, &d.recv, site)? {
            Some(n) => sim.emit(Stmt::Expr(Expr::try_(n))),
            None => raw(sim, format!("{}?;", cs.build_call(env, rust_mname, &d.recv, &arg_str))),
        }
        return Ok(());
    }
    let v = sim.fresh("_t")?;
    let r = ty_text(env, rust_ret);
    let mname = d.call.name.as_str();
    if d.root_routed {
        raw(sim, format!("let {v}: {r} = {}?;", cs.build_call(env, rust_mname, &d.recv, &arg_str)));
        sim.push(Expr::Var(v), rust_ret.clone());
        return Ok(());
    }
    let obj_ty = &site.obj_ty;
    if rust_mname == "clone" && !is_object(env, obj_ty) && *obj_ty != RsType::Unit && !chain_declares(env, obj_ty, mname) {
        // 解析到根类 Object.clone（数组 / 类链无人声明）：Java 浅拷贝经 __shallow_copy 派发；
        // `this` 已是 &Self
        let src = if site.obj_e == "this" { "this".to_string() } else { format!("&{}", site.obj_e) };
        raw(sim, format!("let {v}: {O} = {O}__clone_base({src})?;"));
        sim.push(Expr::Var(v), RsType::Object);
        return Ok(());
    }
    let sig_call = CallRef::with(&d.sig_owner, mname, &d.call.desc);
    let sig_ret = sig::lookup_method_sig_ret(
        env,
        &sig_call,
        Some(env.ctx.class_name),
        &sim.cfg.class_type_params,
        Some(&d.sig_recv_ty),
    )?;
    let call_str = cs.build_call(env, rust_mname, &d.recv, &arg_str);
    // 类型变量返回判定按常量池类（Python 以常量池短名 `cls` 查）
    let erased_tv = || sig::erased_ret_is_type_var(&env.ctx, &orig.owner, mname, &d.call.desc);
    match sig_ret {
        Some(s) if is_object(env, rust_ret) && !is_object(env, &s) => {
            // 签名真实返回类型装箱（S-3.1）
            raw(sim, format!("let {v} = {};", boxed_text(env, &format!("{call_str}?"), &s)?));
            sim.push(Expr::Var(v), rust_ret.clone());
        }
        Some(s) if !same_text(env, &s, rust_ret) && !is_prim(rust_ret) => {
            let node = call_node(env, cs, rust_mname, &d.recv, site)?;
            let_call(sim, v.clone(), node, &call_str);
            sim.push(Expr::Var(v), s);
        }
        None if !is_prim(rust_ret) && !is_object(env, rust_ret) && env.ctx.ty.is_carrier(rust_ret) && erased_tv() => {
            // T-2：返回裸类型变量、描述符擦除为接口载体 → 经 Object 边界取回描述符载体
            raw(
                sim,
                format!("let {v}: {r} = <{r} as ::std::convert::From<{O}>>::from(::std::convert::Into::<{O}>::into({call_str}?));"),
            );
            sim.push(Expr::Var(v), rust_ret.clone());
        }
        None if is_object(env, rust_ret) && erased_tv() => {
            // 返回裸类型变量且无法按接收者实例化：幂等装箱与 sim 记录的 Object 一致
            raw(sim, format!("let {v} = {O}::from_any({call_str}?);"));
            sim.push(Expr::Var(v), rust_ret.clone());
        }
        _ => {
            let node = call_node(env, cs, rust_mname, &d.recv, site)?;
            let_call(sim, v.clone(), node, &call_str);
            sim.push(Expr::Var(v), rust_ret.clone());
        }
    }
    Ok(())
}
