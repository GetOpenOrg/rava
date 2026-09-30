//! 异常表 → CFG 的合成 try 节点、裸 return 收编、跳转线程化与跳转登记
//! （← `BlockSimulator._install_try_nodes` / `_adopt_bare_returns` / `_thread_jumps` /
//! `_register_jumps`）。

use std::collections::{BTreeMap, BTreeSet};

use cfg::opcodes::is_jump;
use cfg::{JumpKind, NodeId, Terminator};

use super::{is_return_op, Blocks, HandlerBind};
use crate::error::{cfg_err, MethodResult};
use crate::node::{Catch, Graph, Kind, Node};
use crate::try_plan::binding_type;

impl Blocks<'_, '_> {
    /// 每个 try 组一个合成节点 T（kind = try）：T 的后继 = try 体入口 + 各处理器入口。
    /// 进入受保护区间的边（源不在该区间内）改指 T；同一起点的多个组由外到内串成
    /// T_outer → T_inner → 体入口。
    pub(super) fn install_try_nodes(&mut self, staged: &mut Graph) -> MethodResult<()> {
        self.entry = 0;
        let planned: BTreeSet<u32> =
            self.plan.groups.iter().flat_map(|g| g.clauses.iter().map(|c| c.handler_pc)).collect();
        for e in self.exception_table {
            if e.start < e.handler && !planned.contains(&e.handler) {
                return cfg_err(format!("异常处理器 pc={} 无法归入任何 try 区域", e.handler));
            }
        }
        if self.plan.is_empty() {
            return Ok(());
        }
        for b in &self.blocks {
            let ctx: BTreeSet<u32> =
                (0u32..).zip(&self.plan.groups).filter(|(_, g)| g.covers(b.start_pc)).map(|(k, _)| k).collect();
            staged.node_mut(b.id).ctx = ctx;
        }
        self.adopt_bare_returns(staged);
        let by_start_idx: BTreeMap<usize, NodeId> = self.blocks.iter().map(|b| (b.start_idx, b.id)).collect();
        let by_start_pc: BTreeMap<u32, NodeId> = self.blocks.iter().map(|b| (b.start_pc, b.id)).collect();
        self.catch_exits = self.plan.catch_body_ends().iter().map(|pc| by_start_pc[pc]).collect();

        // 体入口块 → [T_outer, ..., T_inner]
        let mut chains: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
        let mut next_id = self.blocks.len() as NodeId;
        for (start_idx, groups) in self.plan.groups_by_start().to_vec() {
            let body = by_start_idx[&start_idx];
            self.try_entries.insert(body);
            let mut chain: Vec<Node> = Vec::new();
            let mut inner: BTreeSet<u32> = BTreeSet::new();
            // 内 → 外：ctx(T_g) 不含 g 及其内层同起点组
            for &g in groups.iter().rev() {
                let gid = g as u32;
                inner.insert(gid);
                let body_node = staged.node(body);
                let mut t = Node::new(0, body_node.start_pc, Kind::Try);
                t.ctx = body_node.ctx.difference(&inner).copied().collect();
                t.group = Some(gid);
                for clause in self.plan.groups[g].clauses.clone() {
                    t.handlers.push(by_start_idx[&clause.handler_idx]);
                    t.catch_ends.push(clause.body_end_pc);
                    let bind = self.sim.fresh("_caught")?;
                    let bind_ty = binding_type(&self.env.ctx, &clause)?;
                    t.catches.push(Catch { clause, bind, bind_ty });
                }
                chain.insert(0, t);
            }
            for t in &mut chain {
                t.id = next_id;
                next_id += 1;
            }
            let ids: Vec<NodeId> = chain.iter().map(|t| t.id).collect();
            for (k, mut t) in chain.into_iter().enumerate() {
                t.target = Some(ids.get(k + 1).copied().unwrap_or(body));
                staged.insert(t);
            }
            chains.insert(body, ids);
        }

        let redirect = |staged: &Graph, src: &Node, tgt: Option<NodeId>| -> Option<NodeId> {
            let tgt = tgt?;
            for &t in chains.get(&tgt).map(Vec::as_slice).unwrap_or(&[]) {
                let group = staged.node(t).group.unwrap_or(0);
                if !src.ctx.contains(&group) && t != src.id {
                    return Some(t);
                }
            }
            Some(tgt)
        };
        let order = staged.order.clone();
        for id in order {
            let mut n = staged.node(id).clone();
            if n.is_try() {
                n.handlers = n.handlers.iter().map(|&h| redirect(staged, &n, Some(h)).unwrap_or(h)).collect();
            } else {
                n.target = redirect(staged, &n, n.target);
                n.fallthrough = redirect(staged, &n, n.fallthrough);
                n.default = redirect(staged, &n, n.default);
                n.cases = n.cases.iter().map(|(v, tg)| (v.clone(), redirect(staged, &n, Some(*tg)).unwrap_or(*tg))).collect();
            }
            *staged.node_mut(id) = n;
        }
        for t in staged.iter().filter(|n| n.is_try()) {
            for (&h, c) in t.handlers.iter().zip(&t.catches) {
                if self.handler_bind.contains_key(&h) {
                    return cfg_err(format!("处理器 pc={} 被多个 try 区域共用", staged.node(h).start_pc));
                }
                self.handler_bind.insert(h, HandlerBind { try_node: t.id, bind: c.bind.clone(), ty: c.bind_ty.clone() });
            }
        }
        if let Some(chain) = chains.get(&0) {
            self.entry = chain[0];
        }
        Ok(())
    }

    /// 只含一条返回指令、且只有一个前驱的块随其前驱归入同一组 try 区域
    /// （javac 把 `try { return f(); }` 的受保护区间收在 xreturn 之前）
    fn adopt_bare_returns(&self, staged: &mut Graph) {
        let mut preds: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
        for n in staged.iter() {
            for s in n.successors() {
                preds.entry(s).or_default().push(n.id);
            }
        }
        for b in &self.blocks {
            if !matches!(b.term, Terminator::Exit)
                || b.end_idx - b.start_idx != 1
                || !is_return_op(self.insns[b.start_idx].opcode)
            {
                continue;
            }
            // 处理器入口此时尚无前驱（try 节点还未安装），不会被收编
            if let Some([src]) = preds.get(&b.id).map(Vec::as_slice) {
                let ctx = staged.node(*src).ctx.clone();
                staged.node_mut(b.id).ctx = ctx;
            }
        }
    }

    /// 跳转线程化：只含一条 goto 的块不产生任何代码，指向它的边直接改指其最终目标
    pub(super) fn thread_jumps(&self, staged: &mut Graph) {
        let mut trampoline: BTreeMap<NodeId, NodeId> = BTreeMap::new();
        for b in &self.blocks {
            let Terminator::Goto { target, .. } = b.term else { continue };
            if b.id != 0
                && !self.try_entries.contains(&b.id)
                && !self.handler_bind.contains_key(&b.id)
                && b.end_idx - b.start_idx == 1
                && target != b.id
            {
                // 已含 try 入口改指
                if let Some(t) = staged.node(b.id).target {
                    trampoline.insert(b.id, t);
                }
            }
        }
        let resolve = |mut nid: NodeId| -> Option<NodeId> {
            let mut seen = BTreeSet::new();
            while let Some(&t) = trampoline.get(&nid) {
                if !seen.insert(nid) {
                    return None;
                }
                nid = t;
            }
            Some(nid)
        };
        let fin: BTreeMap<NodeId, NodeId> = trampoline.keys().filter_map(|&n| resolve(n).map(|t| (n, t))).collect();
        if fin.is_empty() {
            return;
        }
        let order = staged.order.clone();
        for id in order {
            let n = staged.node_mut(id);
            let map = |x: Option<NodeId>| x.map(|v| fin.get(&v).copied().unwrap_or(v));
            n.target = map(n.target);
            n.fallthrough = map(n.fallthrough);
            n.default = map(n.default);
            for (_, t) in &mut n.cases {
                *t = fin.get(t).copied().unwrap_or(*t);
            }
        }
    }

    /// 登记全部跳转指令；不可达块记为 dead
    pub(super) fn register_jumps(&mut self, reach: &BTreeSet<NodeId>) {
        for b in &self.blocks {
            for ins in &self.insns[b.start_idx..b.end_idx] {
                if !is_jump(ins.opcode) {
                    continue;
                }
                self.ledger.expect(ins.offset);
                if !reach.contains(&b.id) {
                    self.ledger.consume(ins.offset, JumpKind::Dead);
                }
            }
        }
    }
}
