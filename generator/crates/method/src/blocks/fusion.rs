//! 窥孔融合（← `method/fusion.py`）：
//! - [`Blocks::try_fuse`]：单前驱直线块并入前驱
//! - [`Blocks::try_short_circuit`]：`a && b` / `a || b` 级联条件块并入前驱条件
//! - [`Blocks::try_ternary`]：两臂各留一值的菱形 → 条件表达式（0/1 两臂 → 布尔表达式）
//!
//! 每条被吸收的跳转记入账本。

use cfg::{Cond, JumpKind, NodeId};
use ir::{Expr};
use sim::StackEntry;
use ty::{Prim, RsType};

use super::Blocks;
use crate::cond_text::render_cond;
use crate::error::MethodResult;
use crate::node::{same_value, Kind};
use crate::text;
use crate::unify::{arm_value, inline_temps, unify_pair};

fn stack_prefix_same(p: &[StackEntry], n: &[StackEntry]) -> bool {
    p.iter().zip(n).all(|(x, y)| same_value(x, y))
}

impl Blocks<'_, '_> {
    /// 单前驱直线块并入前驱，返回并入后的当前节点
    pub(super) fn try_fuse(&mut self, nid: NodeId) -> NodeId {
        let node = self.nodes.node(nid);
        if nid == self.entry || !node.decls.is_empty() || node.is_try() || self.catch_exits.contains(&nid) {
            return nid;
        }
        let preds = self.all_preds(nid);
        let [pid] = preds[..] else { return nid };
        let p = self.nodes.node(pid);
        if pid == nid || !p.processed || p.kind != Kind::Goto || p.ctx != node.ctx {
            return nid;
        }
        let node = self.nodes.node(nid).clone();
        let old_pcs = self.nodes.node(pid).pcs.clone();
        self.consume(&old_pcs, JumpKind::Structured);
        let p = self.nodes.node_mut(pid);
        p.append_stmts(node.stmts, node.stmt_pcs);
        (p.kind, p.cond, p.key) = (node.kind, node.cond, node.key);
        (p.target, p.fallthrough) = (node.target, node.fallthrough);
        (p.cases, p.default) = (node.cases, node.default);
        (p.exit_stack, p.exit_locals) = (node.exit_stack, node.exit_locals);
        p.pcs = node.pcs;
        self.nodes.node_mut(nid).removed = true;
        pid
    }

    /// 条件块 B 并入其唯一前驱条件块 P（&& / ||），可级联
    pub(super) fn try_short_circuit(&mut self, mut nid: NodeId) {
        loop {
            let node = self.nodes.node(nid);
            if node.kind != Kind::Cond || nid == self.entry || !node.decls.is_empty() || self.catch_exits.contains(&nid) {
                break;
            }
            let preds = self.all_preds(nid);
            let [pid] = preds[..] else { break };
            let p = self.nodes.node(pid);
            if pid == nid || !p.processed || p.kind != Kind::Cond || p.ctx != node.ctx {
                break;
            }
            if !stack_prefix_same(&p.exit_stack, &node.exit_stack) || p.exit_stack.len() != node.exit_stack.len() {
                break;
            }
            let (Some(n_cond), Some(cp)) = (node.cond.as_ref(), p.cond.clone()) else { break };
            let Some(cb) = inline_temps(self.env, &node.stmts, n_cond, &node.exit_stack) else { break };
            let (pt, pf, nt, nf) = (p.target, p.fallthrough, node.target, node.fallthrough);
            let me = Some(nid);
            let (cond, tgt, fall) = if pf == me && pt != me {
                if nt == pt {
                    // if (P || B) goto T
                    (Cond::or(cp, cb), pt, nf)
                } else if nf == pt {
                    // if (!P && B) goto Bt
                    (Cond::and(cp.negate(), cb), nt, pt)
                } else {
                    break;
                }
            } else if pt == me && pf != me {
                if nt == pf {
                    // if (P && !B) goto Bf
                    (Cond::and(cp, cb.negate()), nf, pf)
                } else if nf == pf {
                    // if (P && B) goto Bt
                    (Cond::and(cp, cb), nt, pf)
                } else {
                    break;
                }
            } else {
                break;
            };
            let (n_locals, n_pcs) = (node.exit_locals.clone(), node.pcs.clone());
            self.consume(&n_pcs, JumpKind::ShortCircuit);
            let p = self.nodes.node_mut(pid);
            (p.cond, p.target, p.fallthrough) = (Some(cond), tgt, fall);
            p.exit_locals = n_locals;
            p.pcs.extend(n_pcs);
            self.nodes.node_mut(nid).removed = true;
            self.fold_const(pid);
            nid = pid;
        }
    }

    /// 汇合点前的菱形：两臂各留一个值 → 条件表达式
    pub(super) fn try_ternary(&mut self, merge_id: NodeId) -> MethodResult<()> {
        'again: loop {
            for a in self.live_preds(merge_id) {
                if !self.is_value_arm(a, merge_id) {
                    continue;
                }
                let pid = self.all_preds(a)[0];
                let p = self.nodes.node(pid);
                if p.kind != Kind::Cond || !p.processed || p.target == p.fallthrough {
                    continue;
                }
                let Some(other) = (if p.target == Some(a) { p.fallthrough } else { p.target }) else { continue };
                let Some(b) = self.nodes.get(other) else { continue };
                if other == a || b.removed || !b.processed || !self.is_value_arm(other, merge_id) {
                    continue;
                }
                if self.all_preds(other)[0] != pid {
                    continue;
                }
                let an = self.nodes.node(a);
                let merge_ctx = &self.nodes.node(merge_id).ctx;
                if !(p.ctx == an.ctx && an.ctx == b.ctx && b.ctx == *merge_ctx) {
                    continue;
                }
                let base = p.exit_stack.len();
                let arm_ok = |arm: &crate::node::Node| {
                    arm.exit_stack.len() == base + 1 && stack_prefix_same(&p.exit_stack, &arm.exit_stack)
                };
                if !(arm_ok(an) && arm_ok(b)) {
                    continue;
                }
                let (jump_arm, fall_arm) = if p.target == Some(a) { (a, other) } else { (other, a) };
                let Some(jump_cond) = p.cond.clone() else { continue };
                let fall_e = self.nodes.node(fall_arm).exit_stack[base].clone();
                let jump_e = self.nodes.node(jump_arm).exit_stack[base].clone();
                let v = self.ternary_value(&jump_cond, &fall_e, &jump_e)?;
                let mut pcs = self.nodes.node(pid).pcs.clone();
                pcs.extend(self.nodes.node(a).pcs.iter().copied());
                pcs.extend(self.nodes.node(other).pcs.iter().copied());
                self.consume(&pcs, JumpKind::Ternary);
                let p = self.nodes.node_mut(pid);
                p.exit_stack.push(v);
                (p.kind, p.cond, p.target, p.fallthrough) = (Kind::Goto, None, Some(merge_id), None);
                p.pcs.clear();
                self.nodes.node_mut(a).removed = true;
                self.nodes.node_mut(other).removed = true;
                continue 'again;
            }
            return Ok(());
        }
    }

    fn is_value_arm(&self, arm: NodeId, merge_id: NodeId) -> bool {
        let n = self.nodes.node(arm);
        if arm == self.entry
            || self.handler_bind.contains_key(&arm)
            || n.kind != Kind::Goto
            || n.target != Some(merge_id)
            || !n.stmts.is_empty()
            || !n.decls.is_empty()
        {
            return false;
        }
        self.all_preds(arm).len() == 1
    }

    fn ternary_value(&mut self, jump_cond: &Cond, fall: &StackEntry, jump: &StackEntry) -> MethodResult<StackEntry> {
        let (fs, js) = (text::expr(self.env, &fall.expr), text::expr(self.env, &jump.expr));
        let lits = (fs.as_str(), js.as_str());
        if matches!(lits, ("0i32", "1i32") | ("1i32", "0i32"))
            && text::ty(self.env, &fall.ty) == "i32"
            && text::ty(self.env, &jump.ty) == "i32"
        {
            let truth = if js == "1i32" { jump_cond.clone() } else { jump_cond.negate() };
            let expr = Expr::raw(format!("({})", render_cond(&truth)));
            let e = self.new_entry(expr.clone(), RsType::Prim(Prim::Bool));
            self.conds.insert(e.id, expr, truth);
            return Ok(e);
        }
        let (tv, ev, ty) = unify_pair(self.env, arm_value(self.env, fall)?, &fall.ty, arm_value(self.env, jump)?, &jump.ty)?;
        let fall_cond = render_cond(&jump_cond.negate());
        let expr = Expr::raw(format!("(if {fall_cond} {{ {tv} }} else {{ {ev} }})"));
        Ok(self.new_entry(expr, ty))
    }
}
