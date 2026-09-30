//! 入口状态汇合（← `BlockSimulator._merge_entry` / `_join_ref_type`）。

use std::collections::BTreeMap;

use cfg::NodeId;
use instr::hierarchy::{common_ref_type_widening, is_subtype};
use ir::{AssignStmt, Expr, LetStmt, Raw, Stmt, VarOrigin};
use sim::Local;
use ty::RsType;

use super::Blocks;
use crate::error::{cfg_err, MethodResult};
use crate::text;
use crate::unify::{same_entry, unify_values};

impl Blocks<'_, '_> {
    /// 多前驱汇合：栈列不同 → 合并变量；局部变量取各前驱一致的绑定
    pub(super) fn merge_entry(&mut self, nid: NodeId, preds: &[NodeId]) -> MethodResult<()> {
        let start_pc = self.nodes.node(nid).start_pc;
        let depth = self.nodes.node(preds[0]).exit_stack.len();
        if preds[1..].iter().any(|p| self.nodes.node(*p).exit_stack.len() != depth) {
            return cfg_err(format!("汇合点 pc={start_pc} 各前驱栈深不一致"));
        }
        let mut stack = Vec::new();
        for k in 0..depth {
            let column: Vec<_> = preds.iter().map(|p| self.nodes.node(*p).exit_stack[k].clone()).collect();
            if column[1..].iter().all(|c| same_entry(self.env, &column[0], c)) {
                stack.push(column[0].clone());
                continue;
            }
            let (values, ty) = unify_values(self.env, &column)?;
            let name = self.sim.fresh("_merged")?;
            for (p, v) in preds.iter().zip(values) {
                // 汇合赋值：值文本为标识符时以 Var 承载，其余为 Raw 叶子
                let value = match ir::Ident::new(v.clone()) {
                    Ok(id) if text::is_ident(&v) => Expr::Var(id),
                    _ => Expr::Raw(Raw(v)),
                };
                self.nodes.node_mut(*p).stmts.push(Stmt::Assign(AssignStmt {
                    target: Expr::Var(name.clone()),
                    value,
                    origin: VarOrigin::default(),
                }));
            }
            // 前置声明以 LetStmt 进入 entries：合并值在其声明所在块之外被消费时由变量提升移到外层
            let ir_ty = sim::to_ir_type(&ty, self.env)?;
            let decl = LetStmt { name: name.clone(), ty: Some(ir_ty), mutable: true, value: None, origin: VarOrigin::default() };
            self.nodes.node_mut(nid).decls.push(Stmt::Let(decl));
            let e = self.new_entry(Expr::Var(name), ty);
            stack.push(e);
        }
        self.nodes.node_mut(nid).entry_stack = stack;
        let locals = self.merge_locals(nid, preds);
        self.nodes.node_mut(nid).entry_locals = locals;
        Ok(())
    }

    fn merge_locals(&self, nid: NodeId, preds: &[NodeId]) -> BTreeMap<u16, Local> {
        let mut locals = self.nodes.node(preds[0]).exit_locals.clone();
        let idom_node = self.idom.get(&nid).and_then(|d| self.nodes.get(*d));
        let slots: Vec<u16> = locals.keys().copied().collect();
        for slot in slots {
            let Local { name, ty, .. } = locals[&slot].clone();
            let mut dom_used = false;
            let mut broke = false;
            for p in &preds[1..] {
                let other = self.nodes.node(*p).exit_locals.get(&slot);
                let Some(other) = other.filter(|o| o.name == name) else {
                    locals.remove(&slot);
                    broke = true;
                    break;
                };
                if text::ty(self.env, &other.ty) != text::ty(self.env, &ty) {
                    if let Some(d) = idom_node.filter(|d| d.processed) {
                        if let Some(dom_entry) = d.exit_locals.get(&slot).filter(|e| e.name == name) {
                            locals.insert(slot, dom_entry.clone());
                            dom_used = true;
                        }
                    }
                }
            }
            if broke || dom_used {
                continue;
            }
            // 兄弟分支各自首绑定同名局部的不同引用类型：按 JVM 校验器合并语义取公共祖先，
            // 无公共类祖先回退根类（与变量提升阶段的合并类型一致）
            if locals.contains_key(&slot) {
                if let Some(merged) = self.join_ref_type(slot, preds) {
                    if let Some(l) = locals.get_mut(&slot) {
                        l.ty = merged;
                    }
                }
            }
        }
        locals
    }

    /// 前驱出口同名局部的引用类型全等 → None（不变）；否则公共类祖先 / 根类。
    /// 任一侧为基本类型或无类型 → None
    fn join_ref_type(&self, slot: u16, preds: &[NodeId]) -> Option<RsType> {
        let mut seen: Vec<(String, RsType)> = Vec::new();
        for p in preds {
            let ent = self.nodes.node(*p).exit_locals.get(&slot)?;
            if matches!(ent.ty, RsType::Prim(_) | RsType::Unit) {
                return None;
            }
            let r = text::ty(self.env, &ent.ty);
            if !seen.iter().any(|(s, _)| *s == r) {
                seen.push((r, ent.ty.clone()));
            }
        }
        if seen.len() < 2 {
            return None;
        }
        // 其余类型均可按值侧对齐到首个类型（子类型）时维持首前驱类型
        let first = &seen[0].1;
        if seen[1..].iter().all(|(_, t)| is_subtype(&self.env.ctx, t, first)) {
            return None;
        }
        let mut common = first.clone();
        for (_, other) in &seen[1..] {
            match common_ref_type_widening(&self.env.ctx, &common, other) {
                Some(c) => common = c,
                None => return Some(RsType::Object),
            }
        }
        Some(common)
    }
}
