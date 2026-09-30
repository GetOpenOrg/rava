//! `coerce_arg_node`：实参 → 形参类型的强转（分支顺序与 Python 一致）。

use std::collections::BTreeSet;

use ir::{Expr, Type, UpcastWrap};
use sim::exprs::{clone_plain, clone_ref, from_call, into_call, object_from, object_type, qualified_from};
use sim::StackSim;
use ty::{JvmType, RsType};

use super::{downcast_target_valid, exact_ancestor_type, is_generated_concrete_class, is_prim, jvm};
use crate::build::{cast, ir_ty, str_leaf, text, ty_text};
use crate::coerce;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::log::InstrLog;

/// 实参叶子的克隆：`this`（`&Self`）直接克隆本体，其余 `Clone::clone(&e)`
fn clone_arg(e: Expr, is_this: bool) -> InstrResult<Expr> {
    Ok(if is_this { clone_plain(e)? } else { clone_ref(e)? })
}

fn is_obj_or_unit(t: &str) -> bool {
    t == ir::anchors::OBJECT || t == "()"
}

/// 子类值上转到祖先类的「另一实例化」（`_upcast_to_ancestor_instantiation`）：
/// 先上转到精确祖先，再经 Object 边界重新实例化；目标就是精确祖先 → None
fn upcast_to_ancestor_instantiation(env: &InstrEnv, sim: &StackSim, src: &Expr, actual: &RsType, expected: &RsType) -> InstrResult<Option<Expr>> {
    let ctx = &env.ctx;
    let JvmType::Class { binary, args, .. } = jvm(ctx, expected) else {
        return Ok(None);
    };
    let Some(exact) = exact_ancestor_type(ctx, actual, &binary) else {
        return Ok(None);
    };
    if args.is_empty()
        || ty_text(env, &exact) == ty_text(env, expected)
        || !downcast_target_valid(env, sim, expected)
        || !downcast_target_valid(env, sim, &exact)
    {
        return Ok(None);
    }
    let inner = object_from(into_call(ir_ty(env, &exact)?, src.clone())?)?;
    Ok(Some(qualified_from(ir_ty(env, expected)?, object_type()?, inner)?))
}

/// 接口载体形参（A-4）：同载体 Clone；Object → 非受检载体包装；其余 → 经 Object 包装
fn to_carrier(env: &InstrEnv, e: Expr, is_this: bool, actual: &RsType, expected: &RsType) -> InstrResult<Expr> {
    let (at, et) = (ty_text(env, actual), ty_text(env, expected));
    if at == et && !is_prim(actual) {
        return clone_arg(e, is_this);
    }
    let target = ir_ty(env, expected)?;
    if at == ir::anchors::OBJECT {
        return Ok(qualified_from(target, Type::Infer, clone_ref(e)?)?);
    }
    let src = clone_arg(e, is_this)?;
    Ok(qualified_from(target, Type::Infer, coerce::to_object(env, src, actual, false)?)?)
}

pub(super) fn coerce(env: &InstrEnv, sim: &StackSim, log: &mut InstrLog, e: Expr, actual: &RsType, expected: &RsType) -> InstrResult<Expr> {
    let ctx = &env.ctx;
    let is_this = text(env, &e) == "this";
    let tparams: BTreeSet<String> = sim.cfg.class_type_params.iter().cloned().collect();
    let act_tv = matches!(ctx.ty.from_rs_type(actual, &tparams), JvmType::TypeVar { .. });
    let exp_tv = matches!(ctx.ty.from_rs_type(expected, &tparams), JvmType::TypeVar { .. });
    let (at, et) = (ty_text(env, actual), ty_text(env, expected));
    if let Some(d) = coerce::from_null(env, log, &e, expected)? {
        return Ok(d);
    }
    if ctx.ty.is_carrier(expected) {
        return to_carrier(env, e, is_this, actual, expected);
    }
    if et == ir::anchors::OBJECT && !is_obj_or_unit(&at) {
        if !act_tv && is_this && is_generated_concrete_class(env, sim, actual) {
            return Ok(object_from(clone_plain(e)?)?);
        }
        return coerce::to_object(env, e, actual, false);
    }
    if ["bool", "i8", "i16", "u16"].contains(&et.as_str()) && at != et {
        return Ok(coerce::value(e, actual, expected));
    }
    if et == "i32" && ["i8", "i16", "u16", "bool"].contains(&at.as_str()) {
        return Ok(cast(e, Type::I32, true));
    }
    let cast_leaf = |e: Expr| str_leaf(e);
    if downcast_target_valid(env, sim, expected) && coerce::same_generic_family(env, actual, expected) {
        return Ok(coerce::cast_node(cast_leaf(e), ir_ty(env, expected)?, "", false, true));
    }
    let (act_t, exp_t) = (jvm(ctx, actual), jvm(ctx, expected));
    let both_ref = !is_prim(expected) && !is_prim(actual);
    if both_ref && !is_obj_or_unit(&et) && et != at && ty::jvm_type::subtype::strict_erased_subtype(&act_t, &exp_t, ctx.reg()) {
        // 子类实参传给类祖先形参：按值上转（宏 From<Self> for Ancestor）
        let src = clone_arg(e, is_this)?;
        if let Some(r) = upcast_to_ancestor_instantiation(env, sim, &src, actual, expected)? {
            return Ok(r);
        }
        return Ok(Expr::upcast(src, UpcastWrap::Bare));
    }
    let act_obj = at == ir::anchors::OBJECT;
    if !is_prim(expected) && act_obj && !is_obj_or_unit(&et) && !exp_tv && downcast_target_valid(env, sim, expected) {
        // 调用点隐式 checkcast（Fix 18）
        let target = ir_ty(env, expected)?;
        return Ok(match &exp_t {
            JvmType::Class { binary, .. } if ctx.reg().contains(binary) => coerce::cast_node(cast_leaf(e), target, binary, true, false),
            _ => coerce::cast_node(cast_leaf(e), target, "", false, false),
        });
    }
    if act_obj && exp_tv {
        return Ok(from_call(clone_arg(e, is_this)?)?);
    }
    if exp_tv && at != et && !is_prim(actual) && !is_obj_or_unit(&at) && !act_tv {
        return Ok(from_call(coerce::to_object(env, e, actual, false)?)?);
    }
    if act_tv && et != at && !is_prim(expected) && !is_obj_or_unit(&et) && !exp_tv && downcast_target_valid(env, sim, expected) {
        return Ok(from_call(coerce::to_object(env, e, actual, false)?)?);
    }
    let (act_arr, exp_arr) = (matches!(act_t, JvmType::Array(_)), matches!(exp_t, JvmType::Array(_)));
    if act_obj && exp_arr {
        return Ok(from_call(clone_ref(e)?)?);
    }
    if act_arr && exp_arr && at != et {
        return Ok(from_call(coerce::to_object(env, e, actual, false)?)?);
    }
    if !is_prim(actual) {
        return clone_arg(e, is_this);
    }
    Ok(e)
}
