//! `_lookup_method_sig_ret`：被调方法泛型签名解析出的真实返回类型。

use std::collections::BTreeSet;

use ty::{ClassInfo, JvmType, RsType};

use super::{handwritten_boundary_method, ident_tokens, is_registry_short, jvm, resolve_cls};
use crate::build::ty_text;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::CallRef;
use crate::owner;

/// 返回类型文本中恒为已知类型的名字（Rust 容器 / 基本类型 / 运行时锚点）
const BUILTIN: [&str; 17] =
    ["i32", "i64", "f32", "f64", "bool", "u16", "i8", "i16", "u32", "u64", "Rc", "__Shared", "Vec", "RefCell", "usize", "u8", "()"];

fn is_builtin(n: &str) -> bool {
    BUILTIN.contains(&n) || [ir::anchors::OBJECT, ir::anchors::STRING, ir::anchors::ARRAY].contains(&n)
}

/// 类型文本的全部标识符是否为已知类型（内建 / 注册表短名 / `extra` 中的类型形参）
fn all_known(env: &InstrEnv, t: &RsType, extra: &[String]) -> bool {
    ident_tokens(&ty_text(env, t)).into_iter().all(|n| is_builtin(n) || extra.iter().any(|p| p == n) || is_registry_short(&env.ctx, n))
}

fn subst(t: &RsType, tparams: &[String], args: &[RsType]) -> RsType {
    t.substitute(&|n| tparams.iter().position(|p| p == n).and_then(|i| args.get(i).cloned()))
}

/// sig_ret 引用了 callee 类型变量时：同类同名形参原样返回；接收者是 callee 的参数化形态、
/// 子类或实现者（K-6b）→ 按接收者在声明者处的实参视图替换；其余 → None
fn resolve_type_vars(
    env: &InstrEnv,
    sig_ret: RsType,
    callee: &ClassInfo,
    callee_tparams: &[String],
    used: &BTreeSet<&str>,
    caller: (Option<&str>, &[String]),
    receiver_type: Option<&RsType>,
) -> Option<RsType> {
    let (caller_class, caller_tparams) = caller;
    let cls_bin = callee.name();
    if !caller_tparams.is_empty() && caller_class == Some(cls_bin) && used.iter().all(|v| caller_tparams.iter().any(|p| p == v)) {
        return Some(sig_ret);
    }
    let recv = receiver_type?;
    let ctx = &env.ctx;
    if let JvmType::Class { binary, args, .. } = jvm(ctx, recv) {
        if binary == cls_bin && !args.is_empty() {
            let rargs = recv.type_args();
            if rargs.len() == callee_tparams.len() {
                let sub = subst(&sig_ret, callee_tparams, rargs);
                if all_known(env, &sub, caller_tparams) {
                    return Some(sub);
                }
            }
        }
    }
    let targ = if callee.is_interface() {
        super::iface_view_targ_map(ctx, recv, callee)
    } else {
        super::receiver_type_arg_map(ctx, recv, cls_bin)
    }?;
    let sub = sig_ret.substitute(&|n| targ.get(n).cloned());
    all_known(env, &sub, caller_tparams).then_some(sub)
}

pub(super) fn lookup(
    env: &InstrEnv,
    call: &CallRef,
    caller_class: Option<&str>,
    caller_tparams: &[String],
    receiver_type: Option<&RsType>,
) -> InstrResult<Option<RsType>> {
    let ctx = &env.ctx;
    if call.owner.is_empty() || ctx.reg().is_empty() {
        return Ok(None);
    }
    let cls_bin = if call.owner.contains('/') { call.owner.clone() } else { resolve_cls(ctx, &call.owner).unwrap_or_else(|| call.owner.clone()) };
    let Some(ci0) = ctx.reg().get(&cls_bin) else {
        return Ok(None);
    };
    // 常量池类未声明 → 按 JVM 方法解析找继承来的声明者（超类链 → 最具体超接口）
    let declared = |c: &ClassInfo| c.methods().iter().any(|m| m.name == call.name && m.desc == call.desc);
    let cls_bin = if declared(ci0) {
        cls_bin
    } else {
        match owner::resolve_method_declarer(ctx.reg(), &cls_bin, &call.name, &call.desc) {
            Some(d) => d,
            None => return Ok(None),
        }
    };
    let Some(ci) = ctx.reg().get(&cls_bin) else {
        return Ok(None);
    };
    let Some(m) = ci.methods().iter().find(|m| m.name == call.name && m.desc == call.desc) else {
        return Ok(None);
    };
    let callee_tparams = ctx.ty.effective_class_type_params(ci);
    let Some(sig_ret) = ctx.ty.method_sig_types(ci, m, &callee_tparams).ret else {
        // 签名类型按描述符擦除（生成的 Rust 方法同源）→ 返回类型即描述符类型
        return Ok(Some(ctx.ty.jvm_to_rust(&call.ret)));
    };
    if !all_known(env, &sig_ret, &callee_tparams) {
        return Ok(None);
    }
    let text = ty_text(env, &sig_ret);
    let used: BTreeSet<&str> = ident_tokens(&text).into_iter().filter(|n| callee_tparams.iter().any(|p| p == n)).collect();
    if !used.is_empty() {
        return Ok(resolve_type_vars(env, sig_ret, ci, &callee_tparams, &used, (caller_class, caller_tparams), receiver_type));
    }
    if handwritten_boundary_method(ctx, &cls_bin, &call.name, &call.desc)?
        && matches!(jvm(ctx, &sig_ret), JvmType::Class { is_interface: true, .. })
    {
        return Ok(Some(RsType::Object));
    }
    Ok(Some(sig_ret))
}
