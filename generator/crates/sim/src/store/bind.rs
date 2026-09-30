//! store 阶段 4（this / 装箱）、阶段 5（槽位复用与形参重绑定）、阶段 6（发射 let / 赋值）。

use super::StoreCtx;
use crate::env::{ident, to_ir_type, type_text};
use crate::error::SimResult;
use crate::exprs::{
    clone_moved_var, clone_plain, default_value, is_default, is_null, maybe_downcast, object_from, object_type,
    opaque_let_value, qualified_from,
};
use crate::names::safe_name;
use crate::state::{Local, StackSim};
use crate::types::{is_object, is_ref_non_object, is_scalar};
use ir::{AssignStmt, Expr, Ident, LetStmt, Stmt, VarOrigin};
use ty::{consts, RsType};

/// 局部变量 `this` 的名字
const THIS: &str = "this";

fn is_var_named(e: &Expr, name: &str) -> bool {
    matches!(e, Expr::Var(v) if v.as_str() == name)
}

impl StackSim<'_> {
    fn origin(&self, slot: u16, value_ty: Option<&RsType>) -> SimResult<VarOrigin> {
        let value_ty = match value_ty {
            Some(t) => Some(to_ir_type(t, self.env)?),
            None => None,
        };
        Ok(VarOrigin { value_ty, slot: Some(slot), bind_off: Some(self.state.current_offset) })
    }

    fn emit_assign(&mut self, slot: u16, name: Ident, value: Expr) -> SimResult<()> {
        let origin = self.origin(slot, None)?;
        self.state.stmts.push(Stmt::Assign(AssignStmt { target: Expr::Var(name), value, origin }));
        Ok(())
    }

    /// `<Declared as ::std::convert::From<Object>>::from(Object::from(src))`：经 Object 边界按声明类型重建
    fn rebuild_via_object(&self, declared: &RsType, src: Expr) -> SimResult<Expr> {
        qualified_from(to_ir_type(declared, self.env)?, object_type()?, object_from(src)?)
    }

    /// 阶段 4：null → Default、this 克隆 / 监视对象装箱、Object 声明变量再赋具体值的装箱
    pub(super) fn store_this_and_boxing(&mut self, slot: u16, c: &mut StoreCtx) -> SimResult<()> {
        if c.src_is_object && c.hint.is_some() && is_null(&c.expr) {
            // null 赋给非 Object 提示类型：由显式类型注解决定具体类型
            c.expr = default_value()?;
            c.src_is_object = false;
        }
        let is_this = is_var_named(&c.expr, THIS);
        c.is_ref_ty = is_ref_non_object(&c.ty);
        if is_this && c.is_ref_ty {
            // synchronized(this)：dup; astore N; monitorenter —— 栈顶仍是 this 时存身份等价的 Object 装箱
            let top_is_this = self.state.stack.last().is_some_and(|e| is_var_named(&e.expr, THIS));
            if top_is_this {
                c.expr = object_from(clone_plain(c.expr.clone())?)?;
                c.ty = RsType::Object;
            } else {
                // Clone::clone 而非 this.clone()：类的 Java clone() 方法会遮蔽 std Clone
                c.expr = clone_plain(c.expr.clone())?;
            }
        }
        if self.needs_object_boxing(slot, c) {
            // 声明为 Object 的变量再赋入具体类值：Java 隐式上转 → 装箱后赋值，不按值类型 let 阴影
            let src = clone_moved_var(c.expr.clone(), &c.ty)?;
            c.expr = self.env.box_object(src, &c.ty)?;
            c.ty = RsType::Object;
        }
        Ok(())
    }

    fn needs_object_boxing(&self, slot: u16, c: &StoreCtx) -> bool {
        let Some(decl) = c.decl.as_ref().filter(|d| d.ty.is_none()) else {
            return false;
        };
        if c.src_is_object || self.state.param_slots.contains(&slot) {
            return false;
        }
        let decl_name = c.decl_name.as_deref().unwrap_or_default();
        let local = self.state.locals.get(&slot);
        let same_var = local.is_some_and(|l| l.name.as_str() == decl_name);
        let decl_is_root = decl.desc == format!("L{};", consts::OBJECT);
        let rebind_object = same_var && local.is_some_and(|l| is_object(&l.ty));
        // 首次绑定：LVT 声明类型恰为根类——变量静态类型以声明为准，不随首个值收窄
        let first_root = decl_is_root && !is_object(&c.ty) && !same_var;
        (rebind_object || first_root) && !is_scalar(&c.ty)
    }

    /// 阶段 5：槽位复用判定与形参重绑定。返回 true = 已发射，调用方结束
    pub(super) fn store_rebind(&mut self, slot: u16, c: &StoreCtx) -> SimResult<bool> {
        if let Some(local_name) = self.state.locals.get(&slot).map(|l| l.name.as_str().to_string()) {
            if let Some(decl_name) = c.decl_name.as_deref() {
                if local_name != decl_name {
                    // slot 被另一个 Java 变量复用：按新变量的声明名重新 let 声明
                    self.state.locals.remove(&slot);
                }
            } else if c.decl.is_none()
                && !self.state.param_slots.contains(&slot)
                && local_name != self.undeclared_slot_name(slot, Some(&c.ty))
            {
                // slot 被编译器合成的临时变量复用：与此前的 Java 变量是两个变量
                self.state.locals.remove(&slot);
            }
        }
        if !self.state.param_slots.contains(&slot) {
            return Ok(false);
        }
        let Some(local) = self.state.locals.get(&slot).cloned() else {
            return Ok(false);
        };
        let local_ref = is_ref_non_object(&local.ty);
        if is_null(&c.expr) && local_ref {
            // 形参（具体引用类型）被置 null：保持形参类型，赋默认值
            self.emit_assign(slot, local.name, default_value()?)?;
            return Ok(true);
        }
        if is_object(&local.ty) && c.is_ref_ty {
            // 形参声明为 Object，方法体内重新赋入具体类值：装箱后赋回原形参（let 阴影只在块内可见）
            let src = clone_moved_var(c.expr.clone(), &c.ty)?;
            let boxed = self.env.box_object(src, &c.ty)?;
            self.emit_assign(slot, local.name, boxed)?;
            return Ok(true);
        }
        if local_ref && !is_scalar(&c.ty) && type_text(&local.ty, self.env) != type_text(&c.ty, self.env) {
            // 形参声明为具体引用类型、赋入异型引用值：经 Object 边界按形参声明类型重建后赋回
            let src = clone_moved_var(c.expr.clone(), &c.ty)?;
            let value = self.rebuild_via_object(&local.ty, src)?;
            self.emit_assign(slot, local.name, value)?;
            return Ok(true);
        }
        Ok(false)
    }

    /// 阶段 6：发射 let / 赋值。同变量跨实例化漂移经 Object 边界落回原变量时返回 true（跳过栈清理）
    pub(super) fn store_emit(&mut self, slot: u16, c: &mut StoreCtx) -> SimResult<bool> {
        let Some(local) = self.state.locals.get(&slot).cloned() else {
            let name = match c.decl_name.clone() {
                Some(n) => n,
                None => self.undeclared_slot_name(slot, Some(&c.ty)),
            };
            self.emit_let(slot, ident(&name)?, c)?;
            return Ok(false);
        };
        let decl_depth = self.state.slot_decl_depth.get(&slot).copied().unwrap_or(0);
        let retyped = type_text(&local.ty, self.env) != type_text(&c.ty, self.env);
        if !retyped && decl_depth <= self.state.depth {
            // hint 升级后赋给已声明局部：Object 值按局部精确类型还原视图
            if c.src_is_object {
                c.expr = maybe_downcast(c.expr.clone(), &c.ty, self.env)?;
            }
            let value = clone_moved_var(c.expr.clone(), &c.ty)?;
            self.emit_assign(slot, local.name.clone(), value)?;
            c.name = Some(local.name);
            return Ok(false);
        }
        // 类型变化或内层作用域首次声明：let 阴影
        if self.is_same_var_drift(slot, c, &local, decl_depth) {
            // 同一 Java 变量在原声明可见处存入渲染不同的引用类型：经 Object 边界按原类型重建后赋回
            let src = clone_moved_var(c.expr.clone(), &c.ty)?;
            let value = self.rebuild_via_object(&local.ty, src)?;
            self.emit_assign(slot, local.name, value)?;
            return Ok(true);
        }
        let mut name = local.name.clone();
        let off = self.state.current_offset;
        let reused_by_synth = c.decl.is_none()
            && retyped
            && self.cfg.slot_decls.get(&slot).is_some_and(|ds| {
                ds.iter().any(|d| safe_name(&d.name) == name.as_str() && !(d.start <= off && off < d.end))
            });
        if reused_by_synth {
            // 存储点不在该槽任何同名声明的作用域内且类型不同：javac 合成变量复用槽位 → 按槽位另行命名
            name = ident(&self.synth_slot_name(slot, Some(&c.ty)))?;
        }
        self.emit_let(slot, name, c)?;
        Ok(false)
    }

    /// 同变量漂移判定：当前绑定与本次存储落在同一 LVT 声明区间，且新旧均为引用类型
    fn is_same_var_drift(&self, slot: u16, c: &StoreCtx, local: &Local, decl_depth: u32) -> bool {
        if c.decl.is_none() || c.decl_name.as_deref() != Some(local.name.as_str()) {
            return false;
        }
        let off = self.state.current_offset;
        let decls = self.cfg.slot_decls.get(&slot).map(Vec::as_slice).unwrap_or_default();
        let Some(same) = decls.iter().find(|d| d.start <= off && off < d.end && safe_name(&d.name) == local.name.as_str())
        else {
            return false;
        };
        let Some(bind_pos) = self.state.slot_bind_pos.get(&slot).copied() else {
            return false;
        };
        // 绑定点不落入本槽其他 LVT 声明区间（同名跨区间是 javac 槽位复用的另一个变量），且先于本区间终点
        let in_other = decls
            .iter()
            .any(|d| d.start <= bind_pos && bind_pos < d.end && (d.start, d.end) != (same.start, same.end));
        !in_other
            && bind_pos < same.end
            && decl_depth <= self.state.depth
            && !is_scalar(&local.ty)
            && !is_scalar(&c.ty)
    }

    /// 绑定并发射 `let mut name[: ty] = value;`
    fn emit_let(&mut self, slot: u16, name: Ident, c: &mut StoreCtx) -> SimResult<()> {
        self.state.locals.insert(slot, Local { name: name.clone(), ty: c.ty.clone(), is_new: true });
        self.state.slot_decl_depth.insert(slot, self.state.depth);
        self.state.slot_bind_pos.insert(slot, self.state.current_offset);
        let value = if c.src_is_object { maybe_downcast(c.expr.clone(), &c.ty, self.env)? } else { c.expr.clone() };
        // Default::default() 需保留类型注解；不透明值（downcast 等）交给 Rust 推断
        let let_ty = if is_default(&value) || c.force_let_ty || !opaque_let_value(&value) {
            Some(to_ir_type(&c.ty, self.env)?)
        } else {
            None
        };
        // Java 局部变量间赋值在 Rust 中是 move，包 Clone 保活源变量（E0382）
        let value = clone_moved_var(value, &c.ty)?;
        let origin = self.origin(slot, Some(&c.ty))?;
        self.state.stmts.push(Stmt::Let(LetStmt { name: name.clone(), ty: let_ty, mutable: true, value: Some(value), origin }));
        c.name = Some(name);
        Ok(())
    }
}
