//! 同一 try 组的不相连受护区间（← `codegen._split_disjoint_try_ranges`）。
//!
//! javac 对 record pattern 的解构访问器逐个发卫兵：每个访问器调用一个短 try 区间，全部指向
//! 同一个处理器——一个 try 组因此携带多个互不相连的区间。块模拟只为首区间装 try 节点并改指
//! 入边；本 pass 在模拟完成后、结构化前为后续每个区间补装合成 try 节点 T_j：
//! - 体入口 = 区间首块，group / catches / catch_ends 沿用本组；
//! - 处理器入口的**私有子图**（全部前驱都在子图内的节点）整体克隆挂到 T_j，克隆链汇回
//!   正常流的边保持指向原节点；
//! - 组外（ctx 不含本组）指向区间首块的边改指 T_j。

use std::collections::{BTreeMap, BTreeSet};

use cfg::NodeId;

use crate::error::{cfg_err, MethodResult};
use crate::node::{Graph, Kind, Node};
use crate::try_plan::TryPlan;

struct Splitter<'g> {
    nodes: &'g mut Graph,
    preds: BTreeMap<NodeId, BTreeSet<NodeId>>,
    next_id: NodeId,
}

impl Splitter<'_> {
    fn fresh_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// 处理器入口 h 的私有子图克隆（返回克隆入口）
    fn clone_handler_chain(&mut self, h: NodeId) -> NodeId {
        let mut private = BTreeSet::from([h]);
        let mut changed = true;
        while changed {
            changed = false;
            for x in self.nodes.iter() {
                if private.contains(&x.id) || x.is_try() {
                    continue;
                }
                if let Some(ps) = self.preds.get(&x.id) {
                    if !ps.is_empty() && ps.is_subset(&private) {
                        private.insert(x.id);
                        changed = true;
                    }
                }
            }
        }
        let mut reach = BTreeSet::from([h]);
        let mut work = vec![h];
        while let Some(w) = work.pop() {
            for s in self.nodes.node(w).successors() {
                if private.contains(&s) && reach.insert(s) {
                    work.push(s);
                }
            }
        }
        let mut clone = BTreeMap::new();
        self.clone_of(h, &reach, &mut clone)
    }

    fn clone_of(&mut self, x: NodeId, reach: &BTreeSet<NodeId>, clone: &mut BTreeMap<NodeId, NodeId>) -> NodeId {
        if !reach.contains(&x) {
            // 汇回共享流：两路在原节点汇合
            return x;
        }
        if let Some(&c) = clone.get(&x) {
            return c;
        }
        let src = self.nodes.node(x).clone();
        let mut c = src.clone();
        c.id = self.fresh_id();
        let cid = c.id;
        clone.insert(x, cid);
        self.nodes.insert(c);
        let mut opt = |s: &mut Self, v: Option<NodeId>| v.map(|v| s.clone_of(v, reach, clone));
        let target = opt(self, src.target);
        let fallthrough = opt(self, src.fallthrough);
        let default = opt(self, src.default);
        let mut cases = Vec::new();
        for (v, tg) in &src.cases {
            cases.push((v.clone(), self.clone_of(*tg, reach, clone)));
        }
        let c = self.nodes.node_mut(cid);
        (c.target, c.fallthrough, c.default, c.cases) = (target, fallthrough, default, cases);
        cid
    }
}

pub fn split_disjoint_try_ranges(plan: &TryPlan, nodes: &mut Graph) -> MethodResult<()> {
    let mut try_of_group: BTreeMap<u32, NodeId> = BTreeMap::new();
    for n in nodes.iter() {
        if let (Kind::Try, Some(g)) = (n.kind, n.group) {
            try_of_group.entry(g).or_insert(n.id);
        }
    }
    let multi: Vec<(u32, usize)> = (0u32..)
        .zip(0..plan.groups.len())
        .filter(|&(gid, k)| plan.groups[k].ranges.len() > 1 && try_of_group.contains_key(&gid))
        .collect();
    if multi.is_empty() {
        return Ok(());
    }
    // try 节点与体入口块同 start_pc，只登记普通块（后者覆盖前者）
    let mut by_start_pc: BTreeMap<u32, NodeId> = BTreeMap::new();
    let mut preds: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for n in nodes.iter() {
        if !n.is_try() {
            by_start_pc.insert(n.start_pc, n.id);
        }
        for s in n.successors() {
            preds.entry(s).or_default().insert(n.id);
        }
    }
    let next_id = nodes.nodes.keys().max().map_or(0, |m| m + 1);
    let mut sp = Splitter { nodes, preds, next_id };
    for (gid, k) in multi {
        let t_id = try_of_group[&gid];
        for &(start_pc, end) in &plan.groups[k].ranges[1..] {
            // 每区间重读：前一区间的改指可能改写了本组 try 节点的处理器边
            let t = sp.nodes.node(t_id).clone();
            let entry = match by_start_pc.get(&start_pc) {
                Some(&e) => e,
                None => {
                    // 区间首块已被折叠进前驱边：取区间内起点最小的存活块；无则该区间不装 try 节点
                    let in_range = by_start_pc
                        .iter()
                        .filter(|(pc, id)| start_pc <= **pc && **pc < end && !sp.nodes.node(**id).is_try())
                        .min_by_key(|(pc, _)| **pc);
                    match in_range {
                        Some((_, &id)) => id,
                        None => continue,
                    }
                }
            };
            let entry_node = sp.nodes.node(entry);
            if entry_node.is_try() {
                return cfg_err(format!("try 区间 pc={start_pc} 缺少可作体入口的块"));
            }
            let ctx = entry_node.ctx.iter().copied().filter(|x| Some(*x) != t.group).collect();
            let tj_id = sp.fresh_id();
            let handlers: Vec<NodeId> = t.handlers.clone().into_iter().map(|h| sp.clone_handler_chain(h)).collect();
            let mut tj = Node::new(tj_id, start_pc, Kind::Try);
            tj.target = Some(entry);
            tj.handlers = handlers;
            tj.catches = t.catches.clone();
            tj.catch_ends = t.catch_ends.clone();
            tj.group = t.group;
            tj.ctx = ctx;
            sp.nodes.insert(tj);
            retarget_into(sp.nodes, entry, tj_id, t.group);
        }
    }
    Ok(())
}

/// 组外指向区间首块的边改指 T_j（try 节点自身的目标是它自己区间的体入口，不受影响）
fn retarget_into(nodes: &mut Graph, entry: NodeId, tj: NodeId, group: Option<u32>) {
    let order = nodes.order.clone();
    for id in order {
        if id == tj {
            continue;
        }
        let n = nodes.node_mut(id);
        let outside = !group.is_some_and(|g| n.ctx.contains(&g));
        let src = n.id;
        let rt = |slot: NodeId| if slot == entry && slot != src && outside { tj } else { slot };
        if n.is_try() {
            n.handlers = n.handlers.iter().map(|&h| rt(h)).collect();
        } else {
            n.target = n.target.map(rt);
            n.fallthrough = n.fallthrough.map(rt);
            n.default = n.default.map(rt);
            for (_, tg) in &mut n.cases {
                *tg = rt(*tg);
            }
        }
    }
}
