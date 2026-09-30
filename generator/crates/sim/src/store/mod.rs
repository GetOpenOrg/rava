//! 局部变量存储（← `StackSim.store_local` / `_store_local` 七阶段）。
//!
//! 阶段划分与 Python 一致：① 声明查询 + 冻结栈上旧值 + 菱形实参求解（本文件）；
//! ② 对齐描述符声明类型、③ 类型变量上界 + LVTT hint 精化（[`align`]）；
//! ④ this / 装箱、⑤ 槽位复用与形参重绑定、⑥ 发射 let / 赋值（[`bind`]）；
//! ⑦ 栈上同名副本改指目标变量（本文件）。

mod align;
mod bind;

use crate::env::{erased_base, to_ir_type, type_text};
use crate::error::SimResult;
use crate::exprs::{clone_moved_var, is_trivial};
use crate::names::safe_name;
use crate::state::{SlotDecl, StackEntry, StackSim};
use crate::types::diamond_arity;
use ir::{Expr, FnPath, Ident, Type};
use std::collections::BTreeMap;
use ty::RsType;

/// 阶段间传递的局部状态（`_StoreState`）
pub(crate) struct StoreCtx {
    pub expr: Expr,
    pub ty: RsType,
    pub decl: Option<SlotDecl>,
    pub decl_name: Option<String>,
    pub hint: Option<RsType>,
    pub decl_ty: Option<RsType>,
    pub force_let_ty: bool,
    pub src_is_object: bool,
    pub is_ref_ty: bool,
    pub name: Option<Ident>,
}

impl StackSim<'_> {
    /// 存储到局部变量槽；栈上与被存值同身份的 dup 副本改为引用该局部变量。
    pub fn store_local(&mut self, slot: u16, value: StackEntry) -> SimResult<()> {
        let source_id = value.id;
        let source_trivial = is_trivial(&value.expr);
        self.store_inner(slot, value.expr, value.ty)?;
        if source_trivial {
            return Ok(());
        }
        if let Some(local) = self.state.locals.get(&slot) {
            let (name, ty) = (local.name.clone(), local.ty.clone());
            for e in self.state.stack.iter_mut().filter(|e| e.id == source_id) {
                e.expr = Expr::Var(name.clone());
                e.ty = ty.clone();
            }
        }
        Ok(())
    }

    fn store_inner(&mut self, slot: u16, expr: Expr, ty: RsType) -> SimResult<()> {
        let mut c = StoreCtx {
            expr,
            ty,
            decl: None,
            decl_name: None,
            hint: None,
            decl_ty: None,
            force_let_ty: false,
            src_is_object: false,
            is_ref_ty: false,
            name: None,
        };
        self.store_prepare(slot, &mut c)?;
        self.store_align_decl(slot, &mut c)?;
        self.store_refine_hint(&mut c)?;
        self.store_this_and_boxing(slot, &mut c)?;
        if self.store_rebind(slot, &c)? || self.store_emit(slot, &mut c)? {
            return Ok(());
        }
        self.store_rename_stack_copies(&c);
        Ok(())
    }

    /// 阶段 1：声明表查询（LVT 区间）、冻结栈上旧值、菱形构造的类型实参求解
    fn store_prepare(&mut self, slot: u16, c: &mut StoreCtx) -> SimResult<()> {
        c.decl = if self.state.param_slots.contains(&slot) { None } else { self.decl_at(slot, true).cloned() };
        c.decl_name = c.decl.as_ref().map(|d| safe_name(&d.name));
        c.hint = c.decl.as_ref().filter(|d| d.from_sig).and_then(|d| d.ty.clone());
        c.decl_ty = c.decl.as_ref().filter(|d| !d.from_sig).and_then(|d| d.ty.clone());
        if let Some(local) = self.state.locals.get(&slot) {
            let name = local.name.clone();
            self.freeze_stack_var_copies(&name)?;
        }
        // 菱形构造结果（`X<_>`）存入有泛型声明的局部：按声明签名解出类型实参
        let Some(arity) = diamond_arity(&c.ty) else {
            return Ok(());
        };
        let Some(sig) = c.decl.as_ref().map(|d| d.raw_sig.clone()).filter(|s| !s.is_empty()) else {
            return Ok(());
        };
        let Some(solved) = self.env.infer_type_args(&c.ty, &sig).filter(|v| !v.is_empty()) else {
            return Ok(());
        };
        let head = erased_base(&c.ty, self.env);
        let solved_ir = solved.iter().map(|t| to_ir_type(t, self.env)).collect::<SimResult<Vec<_>>>()?;
        if !rewrite_leading_turbofish(&mut c.expr, &head, arity, &solved_ir) {
            c.force_let_ty = true;
        }
        if let RsType::Class { binary, .. } = &c.ty {
            c.ty = RsType::class(binary.clone(), solved);
        }
        Ok(())
    }

    /// 写入局部变量前，把栈上残留的同名 Var 快照到 `_name_preN`（按渲染类型各一份）：
    /// 字节码加载先于存储，dup 留在栈上的副本是存储前的旧值
    fn freeze_stack_var_copies(&mut self, name: &Ident) -> SimResult<()> {
        let copies: Vec<usize> =
            (0..self.state.stack.len()).filter(|&j| matches!(&self.state.stack[j].expr, Expr::Var(v) if v == name)).collect();
        let mut snap_by_ty: BTreeMap<String, Expr> = BTreeMap::new();
        for j in copies {
            let sty = self.state.stack[j].ty.clone();
            let key = type_text(&sty, self.env);
            let snap = match snap_by_ty.get(&key) {
                Some(s) => s.clone(),
                None => {
                    let value = clone_moved_var(Expr::Var(name.clone()), &sty)?;
                    let s = self.fresh_let(&format!("_{name}_pre"), value, &sty)?;
                    snap_by_ty.insert(key, s.clone());
                    s
                }
            };
            self.state.stack[j].expr = snap;
        }
        Ok(())
    }

    /// 阶段 7：dup 后 astore——栈上残留的同名 Var（已被 move）改指目标变量（E0382 防护）
    fn store_rename_stack_copies(&mut self, c: &StoreCtx) {
        let (Expr::Var(src), Some(name)) = (&c.expr, &c.name) else {
            return;
        };
        if src == name {
            return;
        }
        for e in self.state.stack.iter_mut() {
            if matches!(&e.expr, Expr::Var(v) if v == src) {
                e.expr = Expr::Var(name.clone());
            }
        }
    }
}

/// 渲染文本以 `Head::<_, ..>::` 开头时把占位实参替换为解出的实参（Python 按渲染前缀替换；
/// 这里沿渲染最左路径——调用接收者 / `?` / 字段 / 下标 / 无括号 as / 二元左操作数——找到首个调用路径）
fn rewrite_leading_turbofish(e: &mut Expr, head: &str, arity: usize, solved: &[Type]) -> bool {
    match e {
        Expr::Try(inner) => rewrite_leading_turbofish(inner, head, arity, solved),
        Expr::MethodCall { recv, .. } | Expr::Field { recv, .. } | Expr::Index { recv, .. } => {
            rewrite_leading_turbofish(recv, head, arity, solved)
        }
        Expr::Cast { expr, outer_paren: false, .. } => rewrite_leading_turbofish(expr, head, arity, solved),
        Expr::Binary { lhs, .. } => rewrite_leading_turbofish(lhs, head, arity, solved),
        Expr::Call { func: FnPath::Path(p), .. } if !p.global && p.segments.len() >= 2 => {
            let first = &mut p.segments[0];
            let open = first.ident.as_str() == head
                && first.generics.len() == arity
                && first.generics.iter().all(|g| matches!(g, Type::Infer));
            if open {
                first.generics = solved.to_vec();
            }
            open
        }
        _ => false,
    }
}
