//! store 阶段 2（对齐描述符声明类型）与阶段 3（类型变量上界上转 + LVTT hint 精化）。

use super::StoreCtx;
use crate::env::{erase, erased_base, to_ir_type, type_text};
use crate::error::SimResult;
use crate::exprs::{
    clone_moved_var, default_value, from_call, int_family_cast, into_call, is_default, is_default_or_null, is_null,
    object_from, object_type, qualified_from, unchecked_cast,
};
use crate::state::StackSim;
use crate::types::{int_family, is_jvm_array, is_object, is_scalar, param_name, same_generic_family};
use ir::{Expr, Type, UpcastWrap};
use ty::{Prim, RsType};

impl StackSim<'_> {
    fn is_class_tparam(&self, t: &RsType) -> bool {
        param_name(t).is_some_and(|n| self.class_tparams.contains(n))
    }

    /// 阶段 2：值对齐到描述符声明类型（int 族收窄、接口载体 / Object / 子类上转、包装类 null）
    pub(super) fn store_align_decl(&mut self, slot: u16, c: &mut StoreCtx) -> SimResult<()> {
        if let Some(src) = int_family(&c.ty) {
            // 操作数栈上 boolean/byte/char/short 都是 int；局部变量类型以 LVT 声明为准，无声明按 int
            let mut target = c.decl_ty.as_ref().and_then(int_family);
            if self.state.param_slots.contains(&slot) {
                if let Some(local) = self.state.locals.get(&slot) {
                    // 形参的 Rust 类型由方法签名决定，回写时对齐
                    target = int_family(&local.ty);
                }
            }
            let target = target.unwrap_or(if src == Prim::Bool { Prim::I32 } else { src });
            if target != src {
                c.expr = int_family_cast(c.expr.clone(), target);
                c.ty = RsType::Prim(target);
                c.force_let_ty = true;
            }
        } else if let Some(decl_ty) = c.decl_ty.clone() {
            if !is_scalar(&c.ty) && !is_scalar(&decl_ty) && type_text(&decl_ty, self.env) != type_text(&c.ty, self.env) {
                self.align_ref_decl(c, &decl_ty)?;
            }
        }
        if matches!(c.decl_ty, Some(RsType::Prim(_) | RsType::Unit)) && is_null(&c.expr) {
            // 声明为包装类（按基本类型建模）的局部赋 null：变量类型以声明为准
            c.expr = default_value()?;
            c.ty = c.decl_ty.clone().unwrap_or(RsType::Unit);
            c.force_let_ty = true;
        }
        Ok(())
    }

    /// 阶段 2 的引用类型分支：接口载体 / 栈类型退化为 Object / 子类值存入父类声明
    fn align_ref_decl(&mut self, c: &mut StoreCtx, decl_ty: &RsType) -> SimResult<()> {
        let env = self.env;
        let null = is_null(&c.expr);
        let carrier = env.carrier_type(decl_ty);
        if let Some(carrier) = carrier.filter(|k| type_text(k, env) == type_text(decl_ty, env)) {
            // 局部声明类型是接口载体：值 → 载体经 Object 边界的非受检包装（Clone 保活）
            if type_text(&c.ty, env) != type_text(&carrier, env) {
                let mut src = clone_moved_var(c.expr.clone(), &c.ty)?;
                if self.is_class_tparam(&c.ty) {
                    // 类型变量值（`L extends System.Logger`）无到载体的直接 From：经 Object 边界
                    src = into_call(object_type()?, src)?;
                }
                c.expr = if null { default_value()? } else { qualified_from(to_ir_type(&carrier, env)?, Type::Infer, src)? };
                c.force_let_ty = true;
            }
            c.ty = decl_ty.clone();
        } else if is_object(&c.ty) {
            // 声明为具体类但栈类型退化为 Object（类型推断缺口）：对齐到声明类型
            if null {
                c.expr = default_value()?;
            } else if decl_ty.type_args().is_empty() {
                c.expr = unchecked_cast(c.expr.clone(), to_ir_type(decl_ty, env)?, false);
                c.force_let_ty = true;
            }
            c.hint = Some(decl_ty.clone());
        } else if erased_base(decl_ty, env) != erased_base(&c.ty, env) && env.is_subtype(&erase(&c.ty), &erase(decl_ty)) {
            // 声明为父类、赋入子类值：From 上转保留运行时类型
            c.expr = Expr::upcast(c.expr.clone(), UpcastWrap::Owned);
            c.ty = decl_ty.clone();
            c.force_let_ty = true;
        }
        Ok(())
    }

    /// 阶段 3：类型变量上界上转 + LVTT hint 精化（Object 取回、类型变量视图、同族实例化、父类上转）
    pub(super) fn store_refine_hint(&mut self, c: &mut StoreCtx) -> SimResult<()> {
        let env = self.env;
        // 类型变量值赋给声明为其上界类型的局部：经 Object 的视图转换为上界类型
        let bound = param_name(&c.ty).and_then(|n| self.state.type_var_bounds.get(n)).cloned();
        let target = c.hint.clone().or_else(|| c.decl_ty.clone());
        if let (Some(bound), Some(target)) = (bound, target) {
            if !is_scalar(&target) && erased_base(&target, env) == erased_base(&bound, env) {
                let src = clone_moved_var(c.expr.clone(), &c.ty)?;
                c.expr = into_call(to_ir_type(&bound, env)?, into_call(object_type()?, src)?)?;
                c.ty = bound;
                c.hint = None;
            }
        }
        c.src_is_object = is_object(&c.ty);
        let Some(hint) = c.hint.clone() else {
            return Ok(());
        };
        if c.src_is_object {
            self.refine_from_object(c, &hint)?;
        } else if !is_scalar(&c.ty) && self.is_class_tparam(&hint) && type_text(&c.ty, env) != type_text(&hint, env) {
            // 声明类型是类型变量（checkcast 落在上界类）：经 Object 边界按对象标识取回视图
            let from_cast = match &c.expr {
                Expr::CheckCast(k) if !k.box_first && k.target == to_ir_type(&c.ty, env)? => Some((*k.expr).clone()),
                _ => None,
            };
            c.expr = match from_cast {
                Some(inner) => from_call(inner)?,
                None => from_call(env.box_object(clone_moved_var(c.expr.clone(), &c.ty)?, &c.ty)?)?,
            };
            c.ty = hint;
            c.force_let_ty = true;
        } else if !is_scalar(&c.ty) && !is_scalar(&hint) {
            self.refine_same_family_or_upcast(c, &hint)?;
        }
        Ok(())
    }

    /// 阶段 3：栈类型为 Object 时按 hint 还原（类型变量 From 取回 / 数组视图）
    fn refine_from_object(&mut self, c: &mut StoreCtx, hint: &RsType) -> SimResult<()> {
        let simple = matches!(c.expr, Expr::Var(_) | Expr::Lit(_) | Expr::NewPending { .. });
        if self.is_class_tparam(hint) && !simple && !is_default(&c.expr) {
            // `E v = (E) es[i]`：擦除后无 checkcast → 宏补的 From<Object> 取回类型变量视图
            c.expr = from_call(c.expr.clone())?;
            c.force_let_ty = true;
        } else if !matches!(c.expr, Expr::Var(_)) && is_jvm_array(hint) && !is_default_or_null(&c.expr) {
            // 声明为数组、元素经 Object 流转：非 Var 值按数组目标还原视图
            c.expr = unchecked_cast(c.expr.clone(), to_ir_type(hint, self.env)?, false);
            c.force_let_ty = true;
        }
        c.ty = hint.clone();
        Ok(())
    }

    /// 再赋值的接口声明变量存入非 null 值、且值不是该接口本身：接口擦除载体；否则 None。
    /// 值为子接口（如 `Collection` 声明存入 `Set`）同样取声明接口——区间内后续存入的值
    /// 可能不实现该子接口，变量按子接口定型会把它们包成子接口视图，派发落空
    fn reassigned_iface_carrier(&self, c: &StoreCtx, hint: &RsType) -> Option<RsType> {
        let env = self.env;
        if !c.decl.as_ref().is_some_and(|d| d.reassigned) || is_null(&c.expr) {
            return None;
        }
        if !env.is_interface(&erase(hint)) {
            return None;
        }
        if env.is_interface(&erase(&c.ty)) && erased_base(&c.ty, env) == erased_base(hint, env) {
            return None;
        }
        env.carrier_type(hint).filter(|k| type_text(k, env) != type_text(&c.ty, env))
    }

    /// 阶段 3：同一泛型类的不同实例化（经 Object 边界重建目标实例化）或类祖先上转
    fn refine_same_family_or_upcast(&mut self, c: &mut StoreCtx, hint: &RsType) -> SimResult<()> {
        let env = self.env;
        let (hint_base, ty_base) = (erased_base(hint, env), erased_base(&c.ty, env));
        if hint_base == ty_base {
            if !is_null(&c.expr) && same_generic_family(&c.ty, hint, env) {
                c.expr = unchecked_cast(c.expr.clone(), to_ir_type(hint, env)?, true);
                c.force_let_ty = true;
            }
            c.ty = hint.clone();
        } else if let Some(carrier) = self.reassigned_iface_carrier(c, hint) {
            // 声明为接口且区间内再赋值：变量先后可持有不同运行时类（如 ArrayList 后接 subList 视图），
            // 静态类型取接口擦除载体，不收窄为首个值的具体类
            let mut src = clone_moved_var(c.expr.clone(), &c.ty)?;
            if env.is_interface(&erase(&c.ty)) {
                // 接口载体之间无直接 From：经 Object 边界
                src = object_from(src)?;
            }
            c.expr = qualified_from(to_ir_type(&carrier, env)?, Type::Infer, src)?;
            c.ty = carrier;
            c.force_let_ty = true;
        } else if !env.is_interface(&erase(hint))
            && hint_base != ir::anchors::OBJECT
            && hint_base != "()"
            && env.is_subtype(&erase(&c.ty), &erase(hint))
        {
            // 声明类型是值类型的父类（Java 隐式拓宽，字节码无 checkcast）：From 上转
            c.expr = Expr::upcast(c.expr.clone(), UpcastWrap::Owned);
            c.ty = hint.clone();
            c.force_let_ty = true;
        }
        // Python 的 `Vec<Object>` ↔ `Vec<E>` 重定向分支：RsType 无 Vec 形态，不可达
        Ok(())
    }
}
