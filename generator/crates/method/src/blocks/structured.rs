//! 可归约路径（← `BlockSimulator._run_structured` / `_inline_loop_header_temps` /
//! `_promote_cross_block_temps`）。

use std::collections::{BTreeMap, BTreeSet};

use cfg::{analyze, JumpKind, NodeId};
use ir::{Expr, LetStmt, Stmt, VarOrigin};

use super::{Blocks, SimOutcome};
use crate::cond_text::render_cond;
use crate::error::{cfg_err, MethodResult};
use crate::node::{Graph, Kind};
use crate::text;
use crate::types::ir_type_of;
use crate::unify::{inline_temps, is_temp_name, parse_temp_let, same_stack};

impl Blocks<'_, '_> {
    pub(super) fn run_structured(mut self) -> MethodResult<SimOutcome> {
        let entry = self.entry;
        let mut live: BTreeSet<NodeId> = BTreeSet::from([entry]);
        for nid in self.rpo.clone() {
            if !live.contains(&nid) {
                let pcs = self.nodes.node(nid).pcs.clone();
                self.consume(&pcs, JumpKind::Dead);
                self.nodes.node_mut(nid).removed = true;
                continue;
            }
            if nid != entry {
                self.try_ternary(nid)?;
            }
            self.set_entry_state(nid)?;
            self.simulate(nid)?;
            self.fold_const(nid);
            let node = self.nodes.node(nid);
            for s in node.successors() {
                live.insert(s);
                let target = self.nodes.node(s);
                if target.processed && !target.removed && s != nid && !same_stack(self.env, &node.exit_stack, &target.entry_stack) {
                    return cfg_err(format!("回边 pc={}→{} 两端操作数栈不一致", node.start_pc, target.start_pc));
                }
            }
            if node.successors().contains(&nid) && !same_stack(self.env, &node.exit_stack, &node.entry_stack) {
                return cfg_err(format!("自环 pc={} 两端操作数栈不一致", node.start_pc));
            }
            let fused = self.try_fuse(nid);
            self.try_short_circuit(fused);
        }

        self.inline_loop_header_temps();
        self.promote_cross_block_temps()?;
        let mut kept = Graph::default();
        for n in self.nodes.iter().filter(|n| n.processed && !n.removed) {
            kept.insert(n.clone());
        }
        for (h, hb) in &self.handler_bind {
            if let Some(hn) = kept.get(*h) {
                if kept.iter().any(|n| n.successors().contains(h) && n.id != hb.try_node) {
                    return cfg_err(format!("异常处理器 pc={} 同时是正常控制流的目标", hn.start_pc));
                }
            }
        }
        Ok(SimOutcome { nodes: kept, entry, dispatch: false, top_decls: Vec::new(), plan: self.plan })
    }

    /// 入口状态：方法入口 / 处理器入口 / 单前驱复制 / 多前驱汇合
    fn set_entry_state(&mut self, nid: NodeId) -> MethodResult<()> {
        let preds = self.live_preds(nid);
        let start_pc = self.nodes.node(nid).start_pc;
        if nid == self.entry {
            let locals = self.sim.state.locals.clone();
            let n = self.nodes.node_mut(nid);
            n.entry_stack = Vec::new();
            n.entry_locals = locals;
        } else if let Some(hb) = self.handler_bind.get(&nid).cloned() {
            // 处理器入口：操作数栈只有被捕获的异常对象；局部变量表取 try 入口处的状态
            if preds != [hb.try_node] {
                return cfg_err(format!("异常处理器 pc={start_pc} 同时是正常控制流的目标"));
            }
            let e = self.new_entry(Expr::Var(hb.bind), hb.ty);
            let locals = self.nodes.node(preds[0]).exit_locals.clone();
            let n = self.nodes.node_mut(nid);
            n.entry_stack = vec![e];
            n.entry_locals = locals;
        } else if preds.is_empty() {
            return cfg_err(format!("活块 pc={start_pc} 没有已处理的前驱"));
        } else if preds.len() == 1 {
            let p = self.nodes.node(preds[0]);
            let (stack, locals) = (p.exit_stack.clone(), p.exit_locals.clone());
            let n = self.nodes.node_mut(nid);
            n.entry_stack = stack;
            n.entry_locals = locals;
        } else {
            self.merge_entry(nid, &preds)?;
        }
        Ok(())
    }

    /// 循环头若只由可回填的临时变量构成，回填进条件（使 `while cond {` 形态成立）
    fn inline_loop_header_temps(&mut self) {
        let kept: Vec<NodeId> = self.nodes.iter().filter(|n| n.processed && !n.removed).map(|n| n.id).collect();
        let succs = kept.iter().map(|&i| (i, self.nodes.node(i).successors())).collect();
        let flow = analyze(self.entry, &succs);
        for h in flow.loops.keys() {
            let node = self.nodes.node(*h);
            if node.kind != Kind::Cond || node.stmts.is_empty() || !node.decls.is_empty() {
                continue;
            }
            let Some(c) = node.cond.as_ref() else { continue };
            if let Some(cond) = inline_temps(self.env, &node.stmts, c, &node.exit_stack) {
                let n = self.nodes.node_mut(*h);
                n.cond = Some(cond);
                n.stmts.clear();
            }
        }
    }

    /// 在某节点声明、被其他节点引用的临时变量：文本声明 → LetStmt，交给变量提升管理作用域
    pub(super) fn promote_cross_block_temps(&mut self) -> MethodResult<()> {
        let kept: Vec<NodeId> = self.nodes.iter().filter(|n| n.processed && !n.removed).map(|n| n.id).collect();
        let mut declared: BTreeMap<String, (NodeId, usize)> = BTreeMap::new();
        for &id in &kept {
            for (k, s) in self.nodes.node(id).stmts.iter().enumerate() {
                if let Some(t) = parse_temp_let(self.env, s) {
                    declared.insert(t.name, (id, k));
                }
            }
        }
        if declared.is_empty() {
            return Ok(());
        }
        for &id in &kept {
            let n = self.nodes.node(id);
            let mut parts: Vec<String> = n.stmts.iter().map(|s| text::stmt(self.env, s)).collect();
            if let Some(c) = &n.cond {
                parts.push(render_cond(c));
            }
            parts.push(n.key.clone());
            let joined = parts.join("\n");
            let names: BTreeSet<&str> = word_tokens(&joined).into_iter().filter(|w| is_temp_name(w)).collect();
            let mut todo = Vec::new();
            for name in names {
                match declared.get(name) {
                    Some(&(o, k)) if o != id => todo.push((name.to_string(), o, k)),
                    _ => {}
                }
            }
            for (name, o, k) in todo {
                let Some(t) = parse_temp_let(self.env, &self.nodes.node(o).stmts[k]) else { continue };
                let ty = match &t.ty {
                    Some(s) => Some(ir_type_of(s)?),
                    None => None,
                };
                let decl = LetStmt {
                    name: ir::Ident::new(name)?,
                    ty,
                    mutable: t.mutable,
                    value: Some(Expr::raw(t.value)),
                    origin: VarOrigin::default(),
                };
                self.nodes.node_mut(o).stmts[k] = Stmt::Let(decl);
            }
        }
        Ok(())
    }
}

/// 极大单词记号（`\b\w+\b` 的全部匹配）
fn word_tokens(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in s.char_indices() {
        match (start, text::is_word_char(c)) {
            (None, true) => start = Some(i),
            (Some(st), false) => {
                out.push(&s[st..i]);
                start = None;
            }
            _ => {}
        }
    }
    if let Some(st) = start {
        out.push(&s[st..]);
    }
    out
}
