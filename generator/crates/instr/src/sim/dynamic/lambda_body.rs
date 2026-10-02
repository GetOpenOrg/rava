//! lambda 闭包体（实现方法调用 + 返回值适配）与闭包装箱（SAM 合成对象 / 不透明闭包）。

use classfile::Const;
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
use crate::error::{InstrError, InstrResult};
use crate::invoke::turbofish::static_call_turbofish;
use crate::log::{Effect, InstrLog};

/// 实现方法是接口实例方法（`action::accept`）：接收者是擦除的接口引用，经与接口同名的
/// 载体分派（与 invokeinterface 同形态）
fn iface_call(env: &InstrEnv, lam: &Lam, ci: &ty::ClassInfo, call: &CallArgs) -> InstrResult<String> {
    let all: Vec<&String> = call.cap.iter().chain(&call.sam).collect();
    let Some(first) = all.first() else {
        // 实例实现方法的接收者只能来自捕获首值或 SAM 首参；两者皆空的调用点在 JVM 链接期即抛
        // LambdaConversionException（metafactory 实参不相容），javac 不产出
        return Err(InstrError::BadInsn(format!("接口实现方法 {}.{} 无接收者实参", lam.impl_cls, lam.impl_mname)));
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
            recv_src = obj_text(env, recv, cap_t)?;
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
            None => format!("Ok({})", obj_text(env, &tried, &env.ctx.ty.jvm_to_rust(&impl_ret))?),
        });
    }
    if erased_sam && impl_ret != "V" {
        let ret_t = env.ctx.ty.jvm_to_rust(&impl_ret);
        let ret_rust = ty_text(env, &ret_t);
        let is_carrier = env.ctx.ty.carrier_type_for_ident(&ret_t).is_some_and(|c| ty_text(env, &c) == ret_rust);
        if !lam.is_erased_ref(env, &impl_ret) || lam.has_generic_sig || is_carrier {
            // 按实现方法的返回类型装箱（S-3.1）：注册表类走 Object::from——对象身份、运行时类
            // 与接口 vtable 全部可达；返回是已铺设载体时 Object::from 解包 __ref 装箱
            return Ok(format!("Ok({})", obj_text(env, &format!("{body}?"), &ret_t)?));
        }
    }
    Ok(body)
}

/// 绑定接收者（首个捕获值）的静态类型是实现类的真子类，且实现方法可被继承（非 private）：
/// javac 把 `super::m` 编成本类合成方法，故此形态只来自 invokevirtual
fn bound_subclass_recv(env: &InstrEnv, lam: &Lam, ci: &ty::ClassInfo) -> bool {
    if !lam.is_instance || lam.is_ctor || lam.cap_names.is_empty() {
        return false;
    }
    if !ci.methods().iter().any(|m| m.name == lam.impl_mname && m.desc == lam.impl_desc && !m.is_private()) {
        return false;
    }
    let RsType::Class { binary, .. } = &lam.cap_tys[0] else { return false };
    let reg = env.ctx.reg();
    let mut cur = reg.get(binary).filter(|c| !c.is_interface()).map(|c| c.super_class());
    let mut hops = 0;
    while let Some(sup) = cur.filter(|s| !s.is_empty()) {
        if sup == lam.impl_cls {
            return true;
        }
        hops += 1;
        if hops > 64 {
            break;
        }
        cur = reg.get(sup).map(|c| c.super_class());
    }
    false
}

/// 闭包体：`Impl::<..>::name(args)`（接口实例方法走载体分派），再做返回值适配
pub(super) fn closure_body(env: &InstrEnv, sim: &StackSim, lam: &Lam, call: &CallArgs) -> InstrResult<String> {
    let body = match lam.ci {
        Some(ci) if lam.is_instance && ci.is_interface() => iface_call(env, lam, ci, call)?,
        Some(ci) if bound_subclass_recv(env, lam, ci) => {
            // 绑定接收者的方法引用（`sdf::getTimeZone`），捕获值静态类型是实现类的子类：
            // invokevirtual 语义，与普通虚调用同形态 `recv.m(args)`（子类无 Deref 到父类，
            // UFCS `Impl::m(&recv)` 既类型不符也绕过子类重写）
            let rest: Vec<&str> = call.cap[1..].iter().chain(&call.sam).map(String::as_str).collect();
            format!("{}.{}({})", lam.cap_names[0], lam.impl_rust_name, rest.join(", "))
        }
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

/// 闭包装箱为 Object：经 samtype 的 SAM 合成对象
/// （`Object::from(I__Lambda::new(__Shared::new(closure), "<隐藏类名>"))`，站点登记账本）。
/// JVM 为每个 lambda 调用点定义实现 samtype 的隐藏类，对象恒有类身份；samtype 不可合成
/// （不在注册表 / 非函数式接口 / 预扫描漏登）是生成器内部错误，不退化为无类身份的裸闭包
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
    let hidden = ctor.as_ref().and_then(|_| env.ctx.hooks.lambda_class_name(site.pc));
    let (Some(path), Some(hidden)) = (ctor, hidden) else {
        return Err(InstrError::BadInsn(format!(
            "lambda 调用点 {}@{} 的 samtype `{iface}` 无 SAM 合成对象",
            env.ctx.class_name, site.pc
        )));
    };
    log.push(Effect::SamSite {
        iface: iface.to_string(),
        sam_desc: lam.sam_desc.clone(),
        class: env.ctx.class_name.to_string(),
        hidden: hidden.clone(),
        interfaces: lambda_interfaces(env, site, iface),
    });
    Ok(sim::exprs::object_from(Expr::call(path, vec![raw(closure), raw(format!("{hidden:?}"))]))?)
}

/// lambda 隐藏类的直接超接口（`InnerClassLambdaMetafactory` 同序）：samtype，altMetafactory 的
/// 标记接口（FLAG_MARKERS，去重），FLAG_SERIALIZABLE 且已列接口均非 Serializable 子类型时追加
/// Serializable（`altMetafactory` 的 foundSerializableSupertype 判定）。altMetafactory 的
/// 静态实参为 (samMethodType, implMethod, instantiatedMethodType, flags, [markerCount, markers..],
/// [bridgeCount, bridges..])；metafactory 只有前三项
fn lambda_interfaces(env: &InstrEnv, site: &IndySite, iface: &str) -> Vec<String> {
    const FLAG_SERIALIZABLE: i32 = 1;
    const FLAG_MARKERS: i32 = 2;
    let mut out = vec![iface.to_string()];
    let args = site.bsm.map(|b| b.args.as_slice()).unwrap_or_default();
    let Some(Const::Int(flags)) = args.get(3) else { return out };
    if flags & FLAG_MARKERS != 0 {
        if let Some(Const::Int(n)) = args.get(4) {
            for a in args.iter().skip(5).take(usize::try_from(*n).unwrap_or(0)) {
                if let Const::Class(c) = a {
                    if !out.contains(c) {
                        out.push(c.clone());
                    }
                }
            }
        }
    }
    if flags & FLAG_SERIALIZABLE != 0 && !out.iter().any(|i| extends_serializable(env, i)) {
        out.push(ty::consts::SERIALIZABLE.to_string());
    }
    out
}

/// 接口 `i` 是否为 Serializable 或其传递子接口（注册表内展开）
fn extends_serializable(env: &InstrEnv, i: &str) -> bool {
    let mut stack = vec![i.to_string()];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(c) = stack.pop() {
        if c == ty::consts::SERIALIZABLE {
            return true;
        }
        if seen.insert(c.clone()) {
            if let Some(ci) = env.ctx.reg().get(&c) {
                stack.extend(ci.interfaces().iter().cloned());
            }
        }
    }
    false
}
