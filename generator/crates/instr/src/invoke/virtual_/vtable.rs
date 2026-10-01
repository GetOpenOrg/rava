//! 类 vtable 分派（`_emit_class_vtable_dispatch`）：`__virtual_view` 视图重建 + wrapper
//! 方法调用；继承成员登记（本类 + 闭包子类，`_virtually_dispatched` / `_chain_has_bridge`）
//! 与形参 / 返回值的擦除边界对齐（`_erased_view_sig`）。

use std::collections::BTreeSet;

use ir::{Expr};
use sim::StackSim;
use ty::{ClassInfo, JvmType, RsType};

use super::bare::boxed_text;
use super::{is_object, is_prim, member_name, raw, same_text, Site};
use crate::build::{ir_ty, text, ty_text};
use crate::coerce;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::{sig, CallRef};
use crate::log::InstrLog;
use crate::owner;

const ACC_PRIVATE: u16 = 0x0002;
const ACC_FINAL: u16 = 0x0010;
const ACC_BRIDGE: u16 = 0x0040;

#[allow(clippy::too_many_arguments)]
pub(super) fn emit_class_vtable_dispatch(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    site: &Site,
    cls_ci: &ClassInfo,
    cls_rust: &RsType,
    rust_ret: &RsType,
) -> InstrResult<()> {
    let cls_bin = cls_ci.name();
    let pdesc = call.param_desc();
    // 名字视角 = 接收者（本类覆盖与继承成员都在其 wrapper 上，名字按本类重载态）
    let mname_r = member_name(env, cls_bin, call)?;
    // 继承成员登记：本类未声明时补转发成员；闭包子类逐一登记（按链上最近声明者填槽）
    log.inherited(cls_bin, &call.name, pdesc);
    if virtually_dispatched(env, cls_ci, &call.name, pdesc, None) {
        for sub in env.ctx.facts.subclasses.get(cls_bin).into_iter().flatten() {
            // K-6b：类型变量签名的方法逐子类判定（有 bridge 才登记）
            if virtually_dispatched(env, cls_ci, &call.name, pdesc, Some(sub)) {
                log.inherited(sub, &call.name, pdesc);
            }
        }
    }
    let view_sig = erased_view_sig(env, cls_ci, call);
    let wargs = align_args(env, sim, log, call, site, view_sig.as_ref().map(|(p, _)| p.as_slice()))?;
    let barg_str = wargs.join(", ");
    // 接收者静态类型可能已带实参：turbofish 统一在裸基名上追加擦除实参
    let base = cls_rust.head_name(env.ctx.ty.names).unwrap_or_default();
    let n_tps = env.ctx.ty.effective_class_type_params(cls_ci).len();
    let erased_targs = if n_tps == 0 { String::new() } else { format!("::<{}>", vec![super::O; n_tps].join(", ")) };
    // invokevirtual 语义：null 接收者先抛 NullPointerException（`__nn()?`，与 getfield / putfield
    // 判空同一路径）；非 null 而运行时类不在本类视图中只可能是生成器缺陷（静态类型已由 javac 校验），
    // 以精确 panic 报出被调方法，不以默认值静默继续
    let view_recv = view_query(&format!("{base}{erased_targs}"), &site.obj_e);
    let miss = view_miss_panic(call);
    let call_expr = format!("_d.{mname_r}({barg_str})?");
    if *rust_ret == RsType::Unit {
        raw(sim, format!("if let Some(_d) = {view_recv} {{ {call_expr}; }} else {{ {miss}; }}"))?;
        return Ok(());
    }
    let v = sim.fresh("_vdispatch")?;
    let r = ty_text(env, rust_ret);
    // 返回对齐（与 emit_call_result 同规则）：擦除返回 Object 而发射签名给出具体类型 → 装箱；
    // raw 形态对精确实例化 → 记录精确形态；其余按描述符类型记录
    let sig_r = view_sig.map(|(_, r)| r);
    let pushed = match sig_r {
        Some(s) if is_object(env, rust_ret) && !is_object(env, &s) => {
            let boxed = boxed_text(env, &call_expr, &s)?;
            raw(sim, format!("let {v}: {r} = if let Some(_d) = {view_recv} {{ {boxed} }} else {{ {miss} }};"))?;
            rust_ret.clone()
        }
        Some(s) if !same_text(env, &s, rust_ret) && !is_prim(rust_ret) => {
            raw(sim, format!("let {v} = if let Some(_d) = {view_recv} {{ {call_expr} }} else {{ {miss} }};"))?;
            s
        }
        _ => {
            raw(sim, format!("let {v}: {r} = if let Some(_d) = {view_recv} {{ {call_expr} }} else {{ {miss} }};"))?;
            rust_ret.clone()
        }
    };
    sim.push(Expr::Var(v), pushed);
    Ok(())
}

/// 视图查询 `Cls::<..>::__virtual_view(recv.__nn()?)`：先判空（null → NullPointerException）
fn view_query(view_ty: &str, obj_e: &str) -> String {
    format!("{view_ty}::__virtual_view({}.__nn()?)", postfix_recv(obj_e))
}

/// 视图查询落空（非 null 接收者的运行时类不是本类或其子类）的精确 panic：视图类 + 被调方法
fn view_miss_panic(call: &CallRef) -> String {
    format!("panic!(\"vtable-view-miss: {}.{}:{}\")", call.owner, call.name, call.desc)
}

/// 接收者文本作后缀调用（`.m()`）的接收方：前缀运算 / `as` / 块表达式加括号，其余原样
fn postfix_recv(e: &str) -> String {
    let needs = e.starts_with(['&', '*', '!', '-']) || e.contains(" as ") || e.starts_with("if ") || e.starts_with("match ");
    if needs { format!("({e})") } else { e.to_string() }
}

/// 形参边界：wrapper 方法（擦除实例化）签名 vs 调用点实参逐位对齐——类型变量位装箱为
/// Object、桥接的具体形参按 checkcast 语义还原
fn align_args(
    env: &InstrEnv,
    sim: &StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    site: &Site,
    view_params: Option<&[RsType]>,
) -> InstrResult<Vec<String>> {
    let mut wargs = site.args.clone();
    let Some(ps) = view_params.filter(|ps| ps.len() == site.args.len()) else {
        return Ok(wargs);
    };
    for (i, a) in site.args.iter().enumerate() {
        let expected = &ps[i];
        let actual = match site.sig_params.as_ref().and_then(|s| s.get(i)) {
            Some(Some(t)) => t.clone(),
            _ => env.ctx.ty.jvm_to_rust(&call.params[i]),
        };
        if same_text(env, expected, &actual) || *expected == RsType::Unit {
            continue;
        }
        let leaf = Expr::raw(a.clone());
        let e = if is_object(env, expected) {
            sig::coerce_arg(env, sim, log, leaf, &actual, &RsType::Object)?
        } else if is_object(env, &actual) {
            // 桥接的真实形参是具体类型：等价 bridge 方法内的 checkcast（binary 不可解析 → 视图转换）
            let bin = match env.ctx.ty.from_rs_type(expected, &Default::default()) {
                JvmType::Class { binary, .. } if env.ctx.reg().contains(&binary) => binary,
                _ => String::new(),
            };
            coerce::cast_node(leaf, ir_ty(env, expected)?, &bin, !bin.is_empty(), false)
        } else {
            sig::coerce_arg(env, sim, log, leaf, &actual, expected)?
        };
        wargs[i] = text(env, &e);
    }
    Ok(wargs)
}

/// 调用目标在接收者擦除实例化（`Cls<Object, ..>` wrapper 视角）下的（形参类型, 返回类型）：
/// 沿超类链找最近的非合成声明取发射签名，声明者类型变量按「接收者全 Object 实参」的祖先
/// 实参代入；只命中合成桥时按被桥接的真实方法解析。链上无声明 → None
fn erased_view_sig(env: &InstrEnv, cls_ci: &ClassInfo, call: &CallRef) -> Option<(Vec<RsType>, RsType)> {
    let ty = &env.ctx.ty;
    let reg = env.ctx.reg();
    let pdesc = call.param_desc();
    let subst_owner = |view_ci: &ClassInfo, m: &classfile::Method| -> (Vec<RsType>, RsType) {
        let owner_tps = ty.effective_class_type_params(view_ci);
        let es = ty.emitted_method_sig_types(view_ci, m, &owner_tps);
        let mapping: Vec<(String, RsType)> = if view_ci.name() == cls_ci.name() {
            owner_tps.iter().map(|p| (p.clone(), RsType::Object)).collect()
        } else {
            let recv_args = vec![RsType::Object; ty.effective_class_type_params(cls_ci).len()];
            let anc = ty.ancestor_type_args(cls_ci, Some(&recv_args));
            let anc_args = anc.into_iter().find(|(b, _)| b == view_ci.name()).map(|(_, a)| a).unwrap_or_default();
            owner_tps
                .iter()
                .enumerate()
                .map(|(i, p)| (p.clone(), anc_args.get(i).cloned().unwrap_or(RsType::Object)))
                .collect()
        };
        let f = |n: &str| mapping.iter().find(|(p, _)| p == n).map(|(_, t)| t.clone());
        (es.params.iter().map(|p| p.substitute(&f)).collect(), es.ret.substitute(&f))
    };
    let mut cur = Some(cls_ci);
    let mut seen = BTreeSet::new();
    while let Some(c) = cur {
        if c.is_interface() || !seen.insert(c.name().to_string()) {
            break;
        }
        if let Some(m) = c.methods().iter().find(|m| !m.is_synthetic() && m.name == call.name && m.desc.starts_with(pdesc)) {
            return Some(subst_owner(c, m));
        }
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    // 桥接（形参擦除）：真实方法的发射签名按其声明者解析
    let (real_bin, real_desc) = owner::resolve_bridge_target(reg, cls_ci, &call.name, &call.desc)?;
    let real_ci = reg.get(&real_bin).filter(|ci| !ci.is_interface())?;
    let m = real_ci.methods().iter().find(|m| !m.is_synthetic() && m.name == call.name && m.desc == real_desc)?;
    Some(subst_owner(real_ci, m))
}

/// 泛型签名里是否含类型变量 token（`TP_IN;` / `TV;`；排除 `Lfoo/Type;` 内的 `T..;`）
fn has_type_var_token(sig: &str) -> bool {
    let b = sig.as_bytes();
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c == b'$';
    (0..b.len()).any(|i| {
        if b[i] != b'T' || (i > 0 && (word(b[i - 1]) || b[i - 1] == b'/')) {
            return false;
        }
        let n = b[i + 1..].iter().take_while(|&&c| word(c)).count();
        n > 0 && b.get(i + 1 + n) == Some(&b';')
    })
}

/// (mname, param_desc) 沿接收者超类链的最近声明是否可安全按子类槽位登记
/// （`_virtually_dispatched`）：private / final → 否；签名提及类型变量 → 族级放行、
/// 逐子类按 bridge 见证判定
fn virtually_dispatched(env: &InstrEnv, recv_ci: &ClassInfo, mname: &str, pdesc: &str, sub_bin: Option<&str>) -> bool {
    let reg = env.ctx.reg();
    let mut cur = Some(recv_ci);
    let mut seen = BTreeSet::new();
    while let Some(c) = cur {
        if !seen.insert(c.name().to_string()) {
            break;
        }
        if let Some(m) = c.methods().iter().find(|m| !m.is_synthetic() && m.name == mname && m.desc.starts_with(pdesc)) {
            if m.access & (ACC_PRIVATE | ACC_FINAL) != 0 {
                return false;
            }
            if has_type_var_token(ty::registry::method_signature(m)) {
                return match sub_bin {
                    None => true,
                    Some(sub) => chain_has_bridge(env, sub, mname, pdesc),
                };
            }
            return true;
        }
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    true
}

/// sub_bin 自身及其超类链上的 (mname, 参数描述符) 合成桥存在性（K-6b）
fn chain_has_bridge(env: &InstrEnv, sub_bin: &str, mname: &str, pdesc: &str) -> bool {
    let reg = env.ctx.reg();
    let mut cur = reg.get(sub_bin);
    let mut seen = BTreeSet::new();
    while let Some(c) = cur {
        if !seen.insert(c.name().to_string()) {
            break;
        }
        let hit = c.methods().iter().any(|b| {
            b.is_synthetic() && b.access & ACC_BRIDGE != 0 && !b.is_static() && b.name == mname && owner::param_part(&b.desc) == pdesc
        });
        if hit {
            return true;
        }
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{has_type_var_token, postfix_recv, view_miss_panic, view_query};
    use crate::invoke::CallRef;

    #[test]
    fn view_query_null_checks_receiver() {
        assert_eq!(view_query("Animal", "a"), "Animal::__virtual_view(a.__nn()?)");
        assert_eq!(view_query("Box::<Object>", "&r"), "Box::<Object>::__virtual_view((&r).__nn()?)");
        assert_eq!(
            view_query("Animal", "Into::<Object>::into(Clone::clone(&t))"),
            "Animal::__virtual_view(Into::<Object>::into(Clone::clone(&t)).__nn()?)"
        );
        assert_eq!(postfix_recv("x as Object"), "(x as Object)");
        assert_eq!(postfix_recv("if c { a } else { b }"), "(if c { a } else { b })");
    }

    #[test]
    fn view_miss_is_precise_panic() {
        let call = CallRef::with("p/Animal", "speak", "()Ljava/lang/String;");
        assert_eq!(view_miss_panic(&call), "panic!(\"vtable-view-miss: p/Animal.speak:()Ljava/lang/String;\")");
    }

    #[test]
    fn type_var_token() {
        assert!(has_type_var_token("(TP_IN;)V"));
        assert!(has_type_var_token("<T:Lp/Obj;>(TT;)V"));
        assert!(!has_type_var_token("(Lfoo/Type;)V"));
        assert!(!has_type_var_token("(Lp/List<Lp/TX;>;)V"));
        assert!(!has_type_var_token("()V"));
    }
}
