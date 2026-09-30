//! 字段写入值转换（← `sim/fields.py` 的 `_coerce_stored_value`）：栈顶值 → 字段声明类型的存储表达式。
//!
//! 沿用 Python 字符串管线的文本产出：各分支以 Raw 叶子承载已渲染的值，结果为存储表达式文本，
//! 由调用方在「未经转换」时换回原节点（`_value_node`）。

use ir::{CastExpr, Expr, Raw, UpcastWrap};
use ty::RsType;

use crate::build::{ir_ty, text, ty_text};
use crate::coerce::{cast_node, from_null, same_generic_family, to_object, value};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy::is_subtype;
use crate::log::InstrLog;

const O: &str = ir::anchors::OBJECT;

/// 存储值的上下文：写入接收者文本（putstatic 为空）、字段声明是否为类型变量、调用方类型形参
pub struct StoreCtx<'a> {
    pub obj_text: &'a str,
    pub slot_is_type_var: bool,
    pub class_tps: &'a [String],
}

fn leaf(s: &str) -> Expr {
    Expr::Raw(Raw(s.to_string()))
}

/// 擦除数组还原（CastExpr 目标 `JArray<Object>`）赋给参数化 Vec 字段：转换目标重定向为字段类型
fn redirect_vec_cast(env: &InstrEnv, val: &Expr, vt: &str, ft: &str, ftype: &RsType) -> InstrResult<Option<Expr>> {
    if ft == vt || !ft.contains("Vec<") || !vt.contains(&format!("Vec<{O}>")) {
        return Ok(None);
    }
    let Expr::CheckCast(c) = val else {
        return Ok(None);
    };
    if c.target != ir_ty(env, &RsType::array(RsType::Object))? {
        return Ok(None);
    }
    Ok(Some(Expr::CheckCast(CastExpr { expr: c.expr.clone(), target: ir_ty(env, ftype)?, mode: c.mode.clone(), box_first: c.box_first })))
}

/// putfield / putstatic 共用：把栈顶值转换为字段声明类型 `ftype` 的存储表达式文本
pub fn coerce_stored_value(env: &InstrEnv, log: &mut InstrLog, val: &Expr, val_ty: &RsType, ftype: &RsType, sc: &StoreCtx<'_>) -> InstrResult<String> {
    let ft = ty_text(env, ftype);
    let mut vt = ty_text(env, val_ty);
    let mut val = val.clone();
    if let Some(v) = redirect_vec_cast(env, &val, &vt, &ft, ftype)? {
        val = v;
        vt = ft.clone();
    }
    let raw = text(env, &val);
    let is_cast = matches!(val, Expr::CheckCast(_));
    let carrier = |t: &RsType| env.ctx.ty.carrier_type_for_ident(t).map(|c| ty_text(env, &c));
    let non_value = |s: &str| s == O || s == "()";
    let val_str = if let Some(n) = from_null(env, log, &val, ftype)? {
        // null 值赋给具体类型字段
        text(env, &n)
    } else if carrier(ftype).as_deref() == Some(ft.as_str()) && vt != ft {
        // A-4：字段声明类型是接口载体。Object 直接 From<_>；其余先经 Object 保持对象身份
        if vt == O {
            format!("<{ft} as ::std::convert::From<_>>::from(Clone::clone(&{raw}))")
        } else {
            let o = to_object(env, leaf(&raw), val_ty, true)?;
            format!("<{ft} as ::std::convert::From<_>>::from({})", text(env, &o))
        }
    } else if sc.class_tps.contains(&ft) && !non_value(&vt) && carrier(val_ty).as_deref() == Some(vt.as_str()) {
        // A-4：字段声明是类型变量、值是接口载体 → 经 Object 边界按类型变量的 From<Object> 取回
        format!("<{ft} as ::std::convert::From<{O}>>::from({O}::from({raw}))")
    } else if ft == O && sc.slot_is_type_var && !non_value(&vt) {
        // 字段声明为类型变量但声明类形参名在调用方不可见：值按原类型直接存入
        raw.clone()
    } else if ft == O && !non_value(&vt) {
        // 字段声明为 Object（真实多态边界）：身份保持的向上转型
        text(env, &to_object(env, leaf(&raw), val_ty, true)?)
    } else if same_generic_family(env, val_ty, ftype) {
        // raw type / 通配符字段接收精确实例化的值：CastExpr 的擦除路径（A-3）
        text(env, &cast_node(leaf(&raw), ir_ty(env, ftype)?, "", false, true))
    } else if !sim::types::is_scalar(ftype)
        && !sim::types::is_scalar(val_ty)
        && !non_value(&ft)
        && ft != vt
        && is_subtype(&env.ctx, &sim::erase(val_ty), &sim::erase(ftype))
    {
        // 子类型赋给祖先类型字段：先 clone 再 .into()（E0382）
        text(env, &Expr::upcast(leaf(&raw), UpcastWrap::Clone))
    } else if vt == O && !sim::types::is_scalar(ftype) && !non_value(&ft) && !ft.starts_with("Rc<") && !ft.starts_with("__Shared<") {
        // 值经擦除边界退化为 Object：经 From<Object> 还原（隐式 checkcast）
        format!("From::from(Clone::clone(&{raw}))")
    } else {
        text(env, &value(leaf(&raw), val_ty, ftype))
    };
    // 引用类型赋值加 Clone::clone()（E0382）；CastExpr 的渲染形态自带 clone
    if sim::types::is_scalar(val_ty) || val_str.starts_with("Default::") || val_str.contains(".clone()") || val_str.contains("Clone::clone(") || is_cast {
        return Ok(val_str);
    }
    Ok(if val_str == "this" && sc.obj_text.contains("this") {
        "Clone::clone(&this)".to_string()
    } else if val_str == "this" {
        // 写入他对象的字段（`res.root = this`）：与其余值位置同一约定
        "Clone::clone(this)".to_string()
    } else {
        format!("Clone::clone(&{val_str})")
    })
}
