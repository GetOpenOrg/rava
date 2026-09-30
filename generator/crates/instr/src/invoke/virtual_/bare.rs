//! bare Object 接收者的多态分派（`_dispatch_bare_object`）：接口经载体 / 根类方法经根
//! vtable 单次直调 / 类虚方法经类 vtable 视图重建；均不命中时为精确存根。

use ir::{Expr, Raw};
use sim::StackSim;
use ty::RsType;

use super::{erased_inst, is_object, is_prim, let_push, member_name, raw, vtable, Site, O};
use crate::build::{text, ty_text};
use crate::coerce;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::{sig, CallRef};
use crate::log::InstrLog;
use crate::owner;

pub(super) fn dispatch_bare_object(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    site: &Site,
    rust_mname: &str,
    rust_ret: &RsType,
) -> InstrResult<()> {
    let cls_bin = call.owner.as_str();
    let reg = env.ctx.reg();
    // 接口方法：经与接口同名的载体分派（itable 语义，不依赖对象的类型实参；不枚举实现类）
    if let Some(ci) = reg.get(cls_bin).filter(|ci| ci.is_interface()) {
        if let Some(decl_bin) = owner::declaring_interface(&env.ctx, ci, &call.name, &call.desc) {
            iface_dispatch(env, sim, log, call, site, &decl_bin, rust_ret)?;
            return Ok(());
        }
    }
    // 根类声明的方法：单次根 vtable 分派（声明 / 覆盖类在自身 ObjectVTable 上桥接）
    if env.ctx.facts.root_virtual.contains(&(call.name.clone(), call.param_desc().to_string())) {
        let root_call = format!("{}.{rust_mname}({})?", site.obj_e, site.arg_str());
        if *rust_ret == RsType::Unit {
            raw(sim, format!("{root_call};"));
        } else {
            let_push(env, sim, "_t", &root_call, rust_ret.clone())?;
        }
        return Ok(());
    }
    // 类虚方法：`Cls::<Object, ..>::__virtual_view(&obj)` 按运行时类查询本类擦除 vtable
    if let Some(ci) = reg.get(cls_bin) {
        let cls_rust = env.ctx.ty.jvm_to_rust(&format!("L{cls_bin};"));
        if !ci.is_interface() && !is_object(env, &cls_rust) {
            return vtable::emit_class_vtable_dispatch(env, sim, log, call, site, ci, &cls_rust, rust_ret);
        }
    }
    unresolved_stub(env, sim, call, rust_ret)
}

/// 生成范围内无可分派实现（方法所属类不在注册表 / 接口无声明者）：调用点以精确存根占据，
/// 命中即报出被调方法（与调用链外方法的存根同一口径）
pub(super) fn unresolved_stub(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, rust_ret: &RsType) -> InstrResult<()> {
    let stub = format!("panic!(\"stub: {}.{}:{}\")", call.owner, call.name, call.desc);
    if *rust_ret == RsType::Unit {
        raw(sim, format!("{stub};"));
        Ok(())
    } else {
        let_push(env, sim, "_vdispatch", &stub, rust_ret.clone())
    }
}

/// 接口载体分派：`Into::<I<Object, ..>>::into(Clone::clone(&obj)).m(args)?`
/// （Into 全限定：接口自身可能声明名为 from 的静态方法）
fn iface_dispatch(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    site: &Site,
    decl_bin: &str,
    rust_ret: &RsType,
) -> InstrResult<()> {
    let cls_bin = call.owner.as_str();
    if decl_bin != cls_bin {
        log.inherited(cls_bin, &call.name, call.param_desc());
    }
    // 名字视角 = 声明接口（载体的继承成员按声明接口重载态命名）
    let iface_mname = member_name(env, decl_bin, call)?;
    let view_ty = erased_inst(env, cls_bin);
    let iface_call = format!(
        "Into::<{}>::into(Clone::clone(&{})).{iface_mname}({})?",
        ty_text(env, &view_ty),
        site.obj_e,
        site.arg_str()
    );
    if *rust_ret == RsType::Unit {
        raw(sim, format!("{iface_call};"));
        return Ok(());
    }
    let v = sim.fresh("_t")?;
    // 擦除描述符返回 Object 而接口签名给出具体引用类型：装箱为 Object 记录；
    // 方法可在祖先接口上声明，按声明者解析
    let decl_call = CallRef::with(decl_bin, &call.name, &call.desc);
    let sig_ret =
        sig::lookup_method_sig_ret(env, &decl_call, Some(env.ctx.class_name), &sim.cfg.class_type_params, Some(&view_ty))?;
    let r = ty_text(env, rust_ret);
    let value = match &sig_ret {
        Some(s) if is_object(env, rust_ret) && !is_object(env, s) => boxed_text(env, &iface_call, s)?,
        _ if !is_prim(rust_ret)
            && !is_object(env, rust_ret)
            && (sig_ret.as_ref().is_some_and(|s| is_object(env, s))
                || (sig_ret.is_none() && sig::erased_ret_is_type_var(&env.ctx, decl_bin, &call.name, &call.desc))) =>
        {
            // 接口签名返回裸类型变量：擦除载体接收者上 Rust 方法返回 Object，
            // 经 Object 边界取回描述符类型（unchecked，与擦除返回 + checkcast 同义）
            format!("<{r} as ::std::convert::From<{O}>>::from(::std::convert::Into::<{O}>::into({iface_call}))")
        }
        _ => iface_call,
    };
    raw(sim, format!("let {v}: {r} = {value};"));
    sim.push(Expr::Var(v), rust_ret.clone());
    Ok(())
}

/// `_coerce_to_object(call_text, t)`（clone 缺省开启）的渲染文本
pub(super) fn boxed_text(env: &InstrEnv, value: &str, t: &RsType) -> InstrResult<String> {
    let e = coerce::to_object(env, Expr::Raw(Raw(value.to_string())), t, true)?;
    Ok(text(env, &e))
}
