//! 文本层的值强转（← `vars._coerce_icmp_operand` / `_coerce_acmp_operand` 与
//! `coerce._coerce_to_object` 的字符串入口）。块级合并值与条件原子在 Python 里是文本，
//! 这里按同一口径拼接；装箱分类与形态走 [`instr::coerce::to_object_text`]。

use instr::InstrEnv;
use ir::anchors::OBJECT;
use ty::{JvmType, Prim, RsType};

use crate::error::MethodResult;

/// `_coerce_to_object(val, ty, clone)`：值文本 → Object 引用文本（对象身份保持）；
/// 分类与形态见 [`instr::coerce::to_object_text`]
pub fn to_object(env: &InstrEnv, val: &str, t: &RsType, clone: bool) -> MethodResult<String> {
    Ok(instr::coerce::to_object_text(env, val, t, clone)?)
}

/// if_icmpX 操作数：u16 / i8 / i16 / bool → i32（bool 非原子先整体加括号）
pub fn icmp_operand(val: &str, t: &RsType) -> String {
    match t {
        RsType::Prim(Prim::U16 | Prim::I8 | Prim::I16) => format!("({val} as i32)"),
        RsType::Prim(Prim::Bool) => {
            if instr::text::is_atomic(val) {
                format!("({val} as i32)")
            } else {
                format!("(({val}) as i32)")
            }
        }
        _ => val.to_string(),
    }
}

/// if_acmpX 操作数：非 Object 类型统一上转为 Object（`Object::from` 保持对象标识）
pub fn acmp_operand(env: &InstrEnv, val: &str, t: &RsType) -> MethodResult<String> {
    if crate::text::ty(env, t) == OBJECT {
        return Ok(val.to_string());
    }
    let jt = env.ctx.ty.from_rs_type(t, &env.tparam_set());
    if matches!(jt, JvmType::Primitive(_) | JvmType::HostPrim(_)) {
        return Ok(val.to_string());
    }
    let clean = val.strip_prefix('&').unwrap_or(val);
    if let JvmType::Class { binary, .. } = &jt {
        if env.ctx.reg().contains(binary) {
            return Ok(format!("Object::from(Clone::clone(&{clean}))"));
        }
    }
    to_object(env, clean, t, true)
}
