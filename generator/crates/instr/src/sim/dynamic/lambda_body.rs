//! lambda 闭包体（实现方法调用 + 返回值适配）与闭包装箱（SAM 合成对象 / 不透明闭包）。

use ir::Expr;
use sim::StackSim;
use ty::type_map::parse_descriptor_return;
use ty::RsType;

use super::boxing::{box_prim_via_valueof, is_prim_text, obj_text};
use super::lambda::{rust_text, Lam};
use super::lambda_args::CallArgs;
use super::{raw, wrapper_of, IndySite};
use crate::build::ty_text;
use crate::env::InstrEnv;
use crate::error::{unported, InstrResult};
use crate::invoke::turbofish::static_call_turbofish;
use crate::log::{Effect, InstrLog};

/// 实现方法是接口实例方法（`action::accept`）：接收者是擦除的接口引用，经与接口同名的
/// 载体分派（与 invokeinterface 同形态）
fn iface_call(env: &InstrEnv, lam: &Lam, ci: &ty::ClassInfo, call: &CallArgs) -> InstrResult<String> {
    let all: Vec<&String> = call.cap.iter().chain(&call.sam).collect();
    let Some(first) = all.first() else {
        // Python 在此对空实参表取下标（IndexError）
        return unported(format!("接口实现方法 {}.{} 无接收者实参", lam.impl_cls, lam.impl_mname));
    };
    let tps = env.ctx.ty.effective_class_type_params(ci);
    let targs =
        if tps.is_empty() { String::new() } else { format!("<{}>", vec![ir::anchors::OBJECT; tps.len()].join(", ")) };
    let recv = first.trim_start_matches('&');
    let mut recv_src = format!("Clone::clone(&{recv})");
    if !call.cap.is_empty() {
        // 绑定接收者是捕获值：静态类型为具体类（`list::add`）时先按对象标识上转为擦除的接口引用
        let cap_t = &lam.cap_tys[0];
        let cap_ty = ty_text(env, cap_t);
        if ![ir::anchors::OBJECT, "()", "_"].contains(&cap_ty.as_str()) && !is_prim_text(&cap_ty) {
            recv_src = obj_text(env, recv, cap_t);
        }
    }
    let rest: Vec<&str> = all[1..].iter().map(|s| s.as_str()).collect();
    Ok(format!(
        "Into::<{}{targs}>::into({recv_src}).{}({})",
        env.ctx.short(&lam.impl_cls),
        lam.impl_rust_name,
        rest.join(", ")
    ))
}

/// 返回值适配：SAM 返回 void → 丢弃实现方法返回值；SAM 返回擦除引用而实现方法返回具体
/// 类型 → 装箱
fn adapt_return(env: &InstrEnv, lam: &Lam, body: String) -> InstrResult<String> {
    let impl_ret = if lam.is_ctor { format!("L{};", lam.impl_cls) } else { parse_descriptor_return(&lam.impl_desc).to_string() };
    if lam.sam_ret == "V" {
        return Ok(if impl_ret != "V" { format!("{body}?; Ok(())") } else { body });
    }
    let erased_sam = lam.is_erased_ref(env, &lam.sam_ret);
    if erased_sam && wrapper_of(&impl_ret).is_some() {
        // 实现方法返回基本类型、SAM 返回擦除引用：经装箱类 valueOf 装成真实包装对象（S-3.1）
        let tried = format!("{body}?");
        return Ok(match box_prim_via_valueof(env, &tried, &impl_ret)? {
            Some(boxed) => format!("Ok({boxed})"),
            None => format!("Ok({})", obj_text(env, &tried, &env.ctx.ty.jvm_to_rust(&impl_ret))),
        });
    }
    if erased_sam && impl_ret != "V" {
        let ret_t = env.ctx.ty.jvm_to_rust(&impl_ret);
        let ret_rust = ty_text(env, &ret_t);
        let is_carrier = env.ctx.ty.carrier_type_for_ident(&ret_t).is_some_and(|c| ty_text(env, &c) == ret_rust);
        if !lam.is_erased_ref(env, &impl_ret) || lam.has_generic_sig || is_carrier {
            // 按实现方法的返回类型装箱（S-3.1）：注册表类走 Object::from——对象身份、运行时类
            // 与接口 vtable 全部可达；返回是已铺设载体时 Object::from 解包 __ref 装箱
            return Ok(format!("Ok({})", obj_text(env, &format!("{body}?"), &ret_t)));
        }
    }
    Ok(body)
}

/// 闭包体：`Impl::<..>::name(args)`（接口实例方法走载体分派），再做返回值适配
pub(super) fn closure_body(env: &InstrEnv, sim: &StackSim, lam: &Lam, call: &CallArgs) -> InstrResult<String> {
    let body = match lam.ci {
        Some(ci) if lam.is_instance && ci.is_interface() => iface_call(env, lam, ci, call)?,
        _ => {
            // 泛型类上的静态实现方法：闭包内无上下文可推断（E0283），与 invokestatic 同规则
            // 显式给出 turbofish；实例实现方法由接收者类型推断
            let tf = if lam.is_instance {
                Vec::new()
            } else {
                static_call_turbofish(&env.ctx, sim, &RsType::class(lam.impl_cls.clone(), Vec::new()), None)
            };
            let tf = if tf.is_empty() {
                String::new()
            } else {
                format!("::<{}>", tf.iter().map(|t| ty_text(env, t)).collect::<Vec<_>>().join(", "))
            };
            let parts = [call.cap.join(", "), call.sam.join(", ")];
            let args: Vec<&str> = parts.iter().filter(|s| !s.is_empty()).map(String::as_str).collect();
            format!("{}{tf}::{}({})", env.ctx.short(&lam.impl_cls), lam.impl_rust_name, args.join(", "))
        }
    };
    adapt_return(env, lam, body)
}

/// 闭包装箱为 Object：samtype 是可合成的函数式接口时经 SAM 合成对象
/// （`Object::from(I__Lambda::new(__Shared::new(closure)))`，站点登记账本）；否则不透明装箱
/// （`Object::from_any(__Shared::new(closure) as __Shared<__DynFn!(..)>)`）
pub(super) fn boxed_closure(env: &InstrEnv, log: &mut InstrLog, site: &IndySite, lam: &Lam, body: String) -> InstrResult<Expr> {
    let obj = ir::anchors::OBJECT;
    let ptypes: Vec<String> = lam.sam_params.iter().map(|p| rust_text(env, p)).collect();
    let rtype = if lam.sam_ret == "V" { "()".to_string() } else { rust_text(env, &lam.sam_ret) };
    // 闭包声明返回是接口载体（SAM 返回接口类型）而上方按擦除装箱成 Object：经 From<Object>
    // 还原为载体（未载体化时返回类型即 Object，不变）
    let mut body = body;
    if lam.sam_ret != "V" && lam.is_erased_ref(env, &lam.sam_ret) && rtype != obj && body.starts_with(&format!("Ok({obj}::")) {
        if let Some(inner) = body.get(3..body.len() - 1) {
            body = format!("Ok(From::from({inner}))");
        }
    }
    let sig: Vec<String> = lam.sam_names.iter().zip(&ptypes).map(|(a, t)| format!("{a}: {t}")).collect();
    let closure = format!("__Shared::new(move |{}| -> Result<{rtype}> {{ {body} }})", sig.join(", "));
    let iface = if lam.sam_desc.starts_with('(') {
        let r = parse_descriptor_return(site.desc);
        r.strip_prefix('L').and_then(|s| s.strip_suffix(';')).unwrap_or("")
    } else {
        ""
    };
    let ctor = if iface.is_empty() { None } else { env.ctx.hooks.sam_ctor_path(iface, env.ctx.class_name) };
    Ok(match ctor {
        Some(path) => {
            log.push(Effect::SamSite {
                iface: iface.to_string(),
                sam_desc: lam.sam_desc.clone(),
                class: env.ctx.class_name.to_string(),
            });
            sim::exprs::object_from(Expr::call(path, vec![raw(closure)]))?
        }
        None => {
            // Result 用裸名：user crate 里 crate::error 是 E0433，两边均经 prelude 引入
            let fn_type = format!("__Shared<__DynFn!(({}) -> Result<{rtype}>)>", ptypes.join(", "));
            raw(format!("{obj}::from_any({closure} as {fn_type})"))
        }
    })
}
