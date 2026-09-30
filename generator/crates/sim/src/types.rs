//! 类型判定（← `stack.py` 散布的 `isinstance(ty, RsPrimitive/RsNamed)` + 名字集合判定、
//! `is_jvm_array`、`coerce._same_generic_family`、菱形构造正则 `^(\w+)<(_(?:, _)*)>$`）。
//!
//! Python 的 `RsPrimitive` / `RsNamed` 区分在 [`RsType`] 上对应「标量（`Prim` / `Unit`）/
//! 引用（其余）」；以基本类型名构造的 `RsNamed('i32')` 在这里同为标量（见 GOLDEN_DIFF.md）。

use crate::env::{erased_base, type_text, SimEnv, INFER_PARAM};
use ty::{consts, Prim, RsType};

/// Rust 原生标量（`PRIMITIVE_RUST_TYPES`：基本类型与 `()`）
pub fn is_scalar(t: &RsType) -> bool {
    matches!(t, RsType::Prim(_) | RsType::Unit)
}

/// 根类 `Object`
pub fn is_object(t: &RsType) -> bool {
    match t {
        RsType::Object => true,
        RsType::Class { binary, args } => binary == consts::OBJECT && args.is_empty(),
        _ => false,
    }
}

/// 引用类型且不是根类（`_is_ref_ty`）
pub fn is_ref_non_object(t: &RsType) -> bool {
    !is_scalar(t) && !is_object(t)
}

/// JVM 操作数栈上以 int 承载的类型族（`_INT_FAMILY`：i32 / bool / u16 / i8 / i16）
pub fn int_family(t: &RsType) -> Option<Prim> {
    match t {
        RsType::Prim(p @ (Prim::I32 | Prim::Bool | Prim::U16 | Prim::I8 | Prim::I16)) => Some(*p),
        _ => None,
    }
}

/// JVM 数组形态 `JArray<..>`
pub fn is_jvm_array(t: &RsType) -> bool {
    matches!(t, RsType::Array(_))
}

/// 渲染文本含 `<`（带实参的类或数组）
fn has_args(t: &RsType) -> bool {
    !t.type_args().is_empty()
}

fn mentions_infer(t: &RsType) -> bool {
    match t {
        RsType::Param(n) => n == INFER_PARAM,
        RsType::Class { args, .. } => args.iter().any(mentions_infer),
        RsType::Array(e) => mentions_infer(e),
        _ => false,
    }
}

/// 同一泛型类的不同实例化（基名相同、实参不同；实参含推断占位 `_` 的不算）
pub fn same_generic_family(actual: &RsType, expected: &RsType, env: &dyn SimEnv) -> bool {
    if !has_args(actual) || !has_args(expected) || type_text(actual, env) == type_text(expected, env) {
        return false;
    }
    erased_base(actual, env) == erased_base(expected, env) && !mentions_infer(actual)
}

/// 菱形构造结果 `X<_, ..>`（全部实参为推断占位）→ 实参个数
pub fn diamond_arity(t: &RsType) -> Option<usize> {
    match t {
        RsType::Class { args, .. } if !args.is_empty() && args.iter().all(|a| matches!(a, RsType::Param(n) if n == INFER_PARAM)) => {
            Some(args.len())
        }
        _ => None,
    }
}

/// 类型变量名（`Param`）
pub fn param_name(t: &RsType) -> Option<&str> {
    match t {
        RsType::Param(n) => Some(n),
        _ => None,
    }
}
