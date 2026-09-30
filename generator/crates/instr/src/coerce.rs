//! 值强转（← `codegen/instr/coerce.py`）：栈值 → 目标类型的表达式构造（基本类型互转 /
//! Object 装箱 / null 还原 / 跨实例化视图转换）。
//!
//! Python 的字符串入口（`_coerce_to_object` / `_render_cast` / `_coerce_value`）与节点版
//! （`coerce_to_object_node` / `cast_node` / `coerce_value_node`）在这里合一为节点构造；
//! 字符串入口对 `this` 叶子的特殊形态由调用方经 [`crate::build::str_leaf`] 保持。

use ir::{BinOp, CastExpr, CastMode, Expr, Type};
use ty::{JvmType, PrimKind, RsType};

use crate::build::{call, cast, mcall, paren, text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::log::{Audit, InstrLog};

/// 装箱分类（`_object_coercion_kind`）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
    /// JVM 基本类型（void 除外）：`.into()`
    Prim,
    /// 作用域类型形参：`Into::<Object>::into(..)`
    TypeVar,
    /// 数组 / 注册表内类 / 接口载体：`Object::from(..)`（对象身份保持）
    Ref,
    /// 其余（无运行时类的值）：`Object::from_any(..)`
    Opaque,
}

pub fn object_kind(env: &InstrEnv, t: &RsType) -> ObjectKind {
    match env.ctx.ty.from_rs_type(t, &env.tparam_set()) {
        JvmType::Primitive(PrimKind::Void) => ObjectKind::Opaque,
        JvmType::Primitive(_) => ObjectKind::Prim,
        JvmType::TypeVar { .. } => ObjectKind::TypeVar,
        JvmType::Array(_) => ObjectKind::Ref,
        JvmType::Class { binary, .. } if env.ctx.reg().contains(&binary) => ObjectKind::Ref,
        _ => ObjectKind::Opaque,
    }
}

/// 值 → Object 引用（`coerce_to_object_node`；Java 的隐式上转，对象身份保持）。
/// `clone`：先 `Clone::clone(&v)`（值仍被后续使用时）
pub fn to_object(env: &InstrEnv, val: Expr, t: &RsType, clone: bool) -> InstrResult<Expr> {
    let kind = object_kind(env, t);
    if kind == ObjectKind::Prim {
        // 负数字面量补外层括号：`-1i32.into()` 解析为 `-(1i32.into())`
        let recv = if text(env, &val).trim_start().starts_with('-') { paren(val) } else { val };
        return mcall(recv, "into", Vec::new());
    }
    let src = if clone { sim::exprs::clone_ref(val)? } else { val };
    Ok(match kind {
        ObjectKind::TypeVar => sim::exprs::into_call(sim::exprs::object_type()?, src)?,
        ObjectKind::Ref => sim::exprs::object_from(src)?,
        _ => call(&[ir::anchors::OBJECT, "from_any"], vec![src])?,
    })
}

/// checkcast / 跨实例化转换节点（`cast_node`）：`checked` → `try_cast::<T>("binary")?`；
/// 否则 `<T as From<Object>>::from(..)`
pub fn cast_node(e: Expr, target: Type, binary: &str, checked: bool, box_first: bool) -> Expr {
    let mode = if checked { CastMode::Checked { binary: binary.to_string() } } else { CastMode::Unchecked };
    Expr::CheckCast(CastExpr { expr: Box::new(e), target, mode, box_first })
}

/// null 字面量流入具体引用类型槽位 → `Default::default()`（`_coerce_from_null`；
/// 记 `boxed-null` 审计）。非 null / 目标为 Object、`()`、基本类型 → None
pub fn from_null(env: &InstrEnv, log: &mut InstrLog, val: &Expr, expected: &RsType) -> InstrResult<Option<Expr>> {
    let t = text(env, val);
    if t != "Object::default()" && t != "Object::default().clone()" {
        return Ok(None);
    }
    if matches!(expected, RsType::Object | RsType::Unit | RsType::Prim(_)) {
        return Ok(None);
    }
    log.audit(Audit::BoxedNull);
    Ok(Some(sim::exprs::default_value()?))
}

/// 窄 / 宽整型与 bool 对齐（`coerce_value_node`）：类型已匹配原样返回
pub fn value(val: Expr, val_ty: &RsType, target: &RsType) -> Expr {
    use ty::Prim as P;
    let src = match val_ty {
        RsType::Prim(p) => Some(*p),
        _ => None,
    };
    let RsType::Prim(tp) = target else {
        return val;
    };
    if src == Some(*tp) {
        return val;
    }
    match tp {
        P::I32 if matches!(src, Some(P::Bool | P::U16 | P::I8 | P::I16)) => cast(paren(val), Type::I32, false),
        P::Bool => paren(Expr::binary(BinOp::Ne, val, crate::build::i32_lit(0))),
        P::I8 | P::I16 | P::U16 => {
            let inner = if src == Some(P::I32) { val } else { cast(val, Type::I32, true) };
            cast(paren(inner), Type::Prim(sim_prim(*tp)), true)
        }
        _ => val,
    }
}

/// 窄类型（i8/i16/u16/bool）上转 i32（`_to_i32`；非原子表达式先整体加括号）
pub fn to_i32(env: &InstrEnv, e: Expr, t: &RsType) -> Expr {
    use ty::Prim as P;
    if !matches!(t, RsType::Prim(P::I8 | P::I16 | P::U16 | P::Bool)) {
        return e;
    }
    let e = if crate::text::is_atomic(&text(env, &e)) { e } else { paren(e) };
    cast(e, Type::I32, true)
}

pub fn sim_prim(p: ty::Prim) -> ir::Prim {
    use ty::Prim as P;
    match p {
        P::I8 => ir::Prim::I8,
        P::I16 => ir::Prim::I16,
        P::I32 => ir::Prim::I32,
        P::I64 => ir::Prim::I64,
        P::F32 => ir::Prim::F32,
        P::F64 => ir::Prim::F64,
        P::Bool => ir::Prim::Bool,
        P::U16 => ir::Prim::U16,
    }
}

/// 同一泛型类的不同实例化（`_same_generic_family`）
pub fn same_generic_family(env: &InstrEnv, actual: &RsType, expected: &RsType) -> bool {
    sim::types::same_generic_family(actual, expected, env)
}
