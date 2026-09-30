//! 结构化：可归约 CFG → 结构树（← `cfg/structure.py`）。
//!
//! 算法：支配树驱动的结构化（Ramsey, "Beyond Relooper", ICFP 2022）的 Rust 适配。
//! - 每个节点 Y 有唯一的「放置父节点」：
//!   * 若存在包含 idom(Y) 而不包含 Y 的循环，取最外层循环头 H：Y 是 H 的 out follower，
//!     放在 `loop` 之后（循环出口后继必须在 loop 之外）；
//!   * 否则父节点为 idom(Y)：前向入边 ≥ 2 → in follower（汇合节点）；否则在分支点内联。
//! - follower 用带标签块包裹其前驱所在的子树，到它的跳转即 `break 'label`；回边即
//!   `continue 'header`。
//! - try 区域（[`Terminal::Try`] 合成节点 T）与循环同构：T 的后继内联为 `Try` 的 try 体与
//!   catch 体；离开受保护区间的节点、try 体与处理器之后的汇合点、越过 catch 体文本终点的
//!   节点是 T 的 try follower，放在 `Try` 之后。循环与 try 同时满足时取更外层者。
//! - 放置完成后逐节点校验「词法所处的 try 组集合 == 节点 ctx」，不一致 → [`CfgError`]。
//!
//! 只以退出结束、却落在外层 try 区域之外的处理器子树（javac 对 synchronized 内记录模式
//! switch 的 MatchException 处理器）会被补齐外层 try 组（S-67），这是本函数唯一的输入改写。

use std::collections::{BTreeMap, BTreeSet};

use crate::error::CfgError;
use crate::flow::{dominates, FlowAnalysis};
use crate::graph::NodeId;
use crate::node::{StructNode, Terminal};
use crate::tree::{BreakLabel, CatchArm, IfItem, Item, LoopItem, SwitchArm, SwitchItem, TryItem};

fn get<N: StructNode>(nodes: &BTreeMap<NodeId, N>, x: NodeId) -> Result<&N, CfgError> {
    nodes.get(&x).ok_or_else(|| CfgError::new(format!("结构化：节点 {x} 不存在")))
}

/// y 支配的子图不流回子图之外（所有路径以 return / athrow 结束）。
fn exit_only_subtree(y: NodeId, flow: &FlowAnalysis) -> bool {
    let sub: BTreeSet<NodeId> =
        flow.rpo.iter().copied().filter(|&n| dominates(&flow.idom, y, n)).collect();
    sub.iter().all(|n| flow.succs_of(*n).iter().all(|s| sub.contains(s)))
}

/// 三类 follower 的放置结果。
#[derive(Default)]
struct Placement {
    in_followers: BTreeMap<NodeId, Vec<NodeId>>,
    out_followers: BTreeMap<NodeId, Vec<NodeId>>,
    try_followers: BTreeMap<NodeId, Vec<NodeId>>,
    follower_set: BTreeSet<NodeId>,
}

struct TryInfo {
    node: NodeId,
    body: NodeId,
    slots: Vec<NodeId>,
    handlers: Vec<NodeId>,
    group: u32,
    catch_ends: Vec<Option<u32>>,
}

fn try_infos<N: StructNode>(
    nodes: &BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
) -> Result<Vec<TryInfo>, CfgError> {
    let mut out = Vec::new();
    for &x in &flow.rpo {
        if let Terminal::Try { body, handlers, group, catch_ends } = get(nodes, x)?.terminal() {
            let mut slots = vec![*body];
            slots.extend(handlers.iter().copied());
            out.push(TryInfo {
                node: x,
                body: *body,
                slots,
                handlers: handlers.clone(),
                group: *group,
                catch_ends: catch_ends.clone(),
            });
        }
    }
    Ok(out)
}

fn nesting_error<N: StructNode>(n: &N) -> CfgError {
    CfgError::new(format!("块 pc={} 的 try 区域与控制流不成嵌套结构", n.start_pc()))
}

fn ctx_of<N: StructNode>(nodes: &BTreeMap<NodeId, N>, x: NodeId) -> Result<BTreeSet<u32>, CfgError> {
    Ok(get(nodes, x)?.ctx().clone())
}

/// try 体入口 / 处理器入口：恒内联为 Try 的 try 体 / catch 体（含 S-67 补齐）。
fn place_try_slot<N: StructNode>(
    nodes: &mut BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    t: &TryInfo,
    y: NodeId,
) -> Result<(), CfgError> {
    let mut lexical = ctx_of(nodes, t.node)?;
    if y == t.body {
        lexical.insert(t.group);
    }
    let cy = ctx_of(nodes, y)?;
    if lexical != cy && y != t.body && cy.is_subset(&lexical) && exit_only_subtree(y, flow) {
        let missing: BTreeSet<u32> = lexical.difference(&cy).copied().collect();
        for &n in &flow.rpo {
            if dominates(&flow.idom, y, n) {
                if let Some(node) = nodes.get_mut(&n) {
                    node.ctx_mut().extend(missing.iter().copied());
                }
            }
        }
    }
    if lexical != ctx_of(nodes, y)? {
        return Err(nesting_error(get(nodes, y)?));
    }
    Ok(())
}

fn find_parent_loop<N: StructNode>(
    nodes: &BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    loops_outer_first: &[(NodeId, &BTreeSet<NodeId>)],
    y: NodeId,
    d: NodeId,
) -> Result<Option<NodeId>, CfgError> {
    let cy = get(nodes, y)?.ctx();
    for &(h, body) in loops_outer_first {
        if body.contains(&d) && !body.contains(&y) && get(nodes, h)?.ctx() == cy {
            // 单前驱的 return / athrow 块留在循环体内，随其唯一前驱的分支臂内联
            let single_pred = flow.preds.get(&y).map_or(0, Vec::len) == 1;
            if get(nodes, y)?.terminal().is_exit() && single_pred {
                continue;
            }
            return Ok(Some(h));
        }
    }
    Ok(None)
}

/// y 位于 t 的 catch 体文本终点之后（try 体不正常落出时的 try 语句后续代码）。
fn leaves_catch<N: StructNode>(
    nodes: &BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    t: &TryInfo,
    d: NodeId,
    y: NodeId,
) -> Result<bool, CfgError> {
    if get(nodes, t.node)?.ctx() != get(nodes, y)?.ctx() {
        return Ok(false);
    }
    let (y_pc, d_pc) = (get(nodes, y)?.start_pc(), get(nodes, d)?.start_pc());
    for (h, end_pc) in t.handlers.iter().zip(t.catch_ends.iter()) {
        if let Some(end) = end_pc {
            if y_pc >= *end && *end > d_pc && dominates(&flow.idom, *h, d) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn find_parent_try<N: StructNode>(
    nodes: &BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    tries: &[TryInfo],
    past_catch: &mut BTreeMap<NodeId, BTreeSet<NodeId>>,
    y: NodeId,
    d: NodeId,
) -> Result<Option<NodeId>, CfgError> {
    let mut parent: Option<NodeId> = None;
    let mut matched = false;
    let (cd, cy) = (ctx_of(nodes, d)?, ctx_of(nodes, y)?);
    for t in tries {
        if t.slots.contains(&y) {
            continue;
        }
        let g = t.group;
        if (d == t.node || cd.contains(&g)) && !cy.contains(&g) && dominates(&flow.idom, t.node, d) {
            matched = true;
            // 同词法层候选取支配链最深者（RPO 最大）
            let deeper = parent.is_none_or(|p| flow.rpo_index[&t.node] > flow.rpo_index[&p]);
            if *get(nodes, t.node)?.ctx() == cy && deeper {
                parent = Some(t.node);
            }
            continue;
        }
        if matched {
            continue;
        }
        let pc = past_catch.entry(t.node).or_default();
        if pc.contains(&d) {
            pc.insert(y);
        } else if leaves_catch(nodes, flow, t, d, y)? {
            past_catch.entry(t.node).or_default().insert(y);
            parent = Some(t.node);
            break;
        }
    }
    Ok(parent)
}

fn place<N: StructNode>(
    nodes: &mut BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    tries: &[TryInfo],
) -> Result<Placement, CfgError> {
    // 循环按体大小降序（外层在前）；同尺寸按循环头升序（稳定排序）
    let mut loops_outer_first: Vec<(NodeId, &BTreeSet<NodeId>)> =
        flow.loops.iter().map(|(h, b)| (*h, b)).collect();
    loops_outer_first.sort_by_key(|(_, b)| std::cmp::Reverse(b.len()));
    let mut pl = Placement::default();
    let mut past_catch: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for &y in &flow.rpo {
        if y == flow.entry {
            continue;
        }
        let d = *flow.idom.get(&y).ok_or_else(|| CfgError::new(format!("节点 {y} 无 idom")))?;
        if let Some(t) = tries.iter().find(|t| t.node == d && t.slots.contains(&y)) {
            place_try_slot(nodes, flow, t, y)?;
            continue;
        }
        let parent_loop = find_parent_loop(nodes, flow, &loops_outer_first, y, d)?;
        let mut parent_try = find_parent_try(nodes, flow, tries, &mut past_catch, y, d)?;
        if let (Some(pt), Some(plp)) = (parent_try, parent_loop) {
            if flow.rpo_index[&plp] <= flow.rpo_index[&pt] {
                parent_try = None; // 循环更外层（或同一节点：loop 包住 try）
            }
        }
        let lexical = if let Some(pt) = parent_try {
            pl.try_followers.entry(pt).or_default().push(y);
            pl.follower_set.insert(y);
            ctx_of(nodes, pt)?
        } else if let Some(h) = parent_loop {
            pl.out_followers.entry(h).or_default().push(y);
            pl.follower_set.insert(y);
            ctx_of(nodes, h)?
        } else {
            if flow.forward_in_degree(y) >= 2 {
                pl.in_followers.entry(d).or_default().push(y);
                pl.follower_set.insert(y);
            }
            if get(nodes, d)?.terminal().is_try() {
                return Err(CfgError::new(format!(
                    "控制流绕过 try 入口进入受保护区间（pc={}）",
                    get(nodes, y)?.start_pc()
                )));
            }
            ctx_of(nodes, d)?
        };
        let cy = ctx_of(nodes, y)?;
        if lexical != cy {
            return Err(CfgError::new(format!(
                "块 pc={} 的 try 区域与控制流不成嵌套结构（词法={:?} 实际={:?}）",
                get(nodes, y)?.start_pc(),
                lexical,
                cy
            )));
        }
    }
    Ok(pl)
}

/// 结构树构建（放置完成后只读）。
struct Builder<'a, N> {
    nodes: &'a BTreeMap<NodeId, N>,
    flow: &'a FlowAnalysis,
    pl: &'a Placement,
    emitted: Vec<NodeId>,
}

impl<N: StructNode> Builder<'_, N> {
    fn branch(&mut self, src: NodeId, tgt: NodeId) -> Result<Vec<Item>, CfgError> {
        if self.flow.is_back_edge(src, tgt) {
            return Ok(vec![Item::Continue(tgt)]);
        }
        if self.pl.follower_set.contains(&tgt) {
            return Ok(vec![Item::Break(BreakLabel::Block(tgt))]);
        }
        self.tree(tgt)
    }

    fn try_arm(&mut self, src: NodeId, tgt: NodeId) -> Result<Vec<Item>, CfgError> {
        if self.pl.follower_set.contains(&tgt) || self.flow.is_back_edge(src, tgt) {
            return Err(CfgError::new(format!(
                "try 区域 pc={} 的体 / 处理器入口 pc={} 同时是其他控制流的汇合点",
                get(self.nodes, src)?.start_pc(),
                get(self.nodes, tgt)?.start_pc()
            )));
        }
        self.tree(tgt)
    }

    fn terminator(&mut self, x: NodeId) -> Result<Vec<Item>, CfgError> {
        let term = get(self.nodes, x)?.terminal();
        Ok(match term {
            Terminal::Exit => Vec::new(),
            Terminal::Try { body, handlers, .. } => {
                let body = self.try_arm(x, *body)?;
                let mut catches = Vec::with_capacity(handlers.len());
                for (clause, h) in handlers.iter().enumerate() {
                    catches.push(CatchArm { clause, body: self.try_arm(x, *h)? });
                }
                vec![Item::Try(TryItem { body, catches, origin: x })]
            }
            Terminal::Goto { target } => self.branch(x, *target)?,
            Terminal::Cond { cond, target, fallthrough } => {
                // 源码顺序：不跳转（fall-through）臂在前
                let then = self.branch(x, *fallthrough)?;
                let else_ = self.branch(x, *target)?;
                vec![Item::If(IfItem { cond: cond.negate(), then, else_, origin: x })]
            }
            Terminal::Switch { key, cases, default } => {
                // 同一目标的 case 合并为一个臂（首次出现序）；与 default 同目标的 case 由 `_` 臂覆盖
                let mut grouped: Vec<(NodeId, Vec<i32>)> = Vec::new();
                for (vals, tgt) in cases {
                    if tgt == default {
                        continue;
                    }
                    match grouped.iter_mut().find(|(t, _)| t == tgt) {
                        Some((_, v)) => v.extend(vals.iter().copied()),
                        None => grouped.push((*tgt, vals.clone())),
                    }
                }
                let mut arms = Vec::with_capacity(grouped.len() + 1);
                for (tgt, vals) in grouped {
                    arms.push(SwitchArm { values: Some(vals), body: self.branch(x, tgt)? });
                }
                arms.push(SwitchArm { values: None, body: self.branch(x, *default)? });
                vec![Item::Switch(SwitchItem { key: key.clone(), arms, origin: x })]
            }
        })
    }

    /// followers 按 RPO 升序依次输出；最先输出者的块在最内层。返回（声明项, 包好的序列）。
    fn wrap(
        &mut self,
        inner: Vec<Item>,
        followers: Option<&Vec<NodeId>>,
    ) -> Result<(Vec<Item>, Vec<Item>), CfgError> {
        let mut ordered: Vec<NodeId> = followers.cloned().unwrap_or_default();
        ordered.sort_by_key(|f| self.flow.rpo_index.get(f).copied().unwrap_or(usize::MAX));
        let mut decls = Vec::new();
        for f in &ordered {
            if get(self.nodes, *f)?.has_decls() {
                decls.push(*f);
            }
        }
        let mut body = inner;
        for f in ordered {
            let mut next = vec![Item::Block { label: f, body }];
            next.extend(self.tree(f)?);
            body = next;
        }
        let decl_items = if decls.is_empty() { Vec::new() } else { vec![Item::Decl(decls)] };
        Ok((decl_items, body))
    }

    fn tree(&mut self, x: NodeId) -> Result<Vec<Item>, CfgError> {
        self.emitted.push(x);
        let n = get(self.nodes, x)?;
        // 块自身的直线代码放在标签块之外，局部变量声明与 Java 同层可见
        let code = Item::Code { block: x, exits: n.terminal().is_exit(), empty: !n.has_stmts() };
        let term = self.terminator(x)?;
        let (in_decls, branch) = self.wrap(term, self.pl.in_followers.get(&x))?;
        let mut core = in_decls;
        core.push(code);
        core.extend(branch);
        if let Some(tf) = self.pl.try_followers.get(&x) {
            let (try_decls, wrapped) = self.wrap(core, Some(tf))?;
            core = try_decls;
            core.extend(wrapped);
        }
        if self.flow.loops.contains_key(&x) {
            core = vec![Item::Loop(LoopItem {
                header: x,
                body: core,
                exit_label: None,
                while_cond: None,
                cond_origin: None,
            })];
        }
        let (mut out, body) = self.wrap(core, self.pl.out_followers.get(&x))?;
        out.extend(body);
        Ok(out)
    }
}

/// 可归约 CFG → 结构树。`nodes` 须覆盖 `flow.rpo` 中全部节点。
pub fn structure<N: StructNode>(
    nodes: &mut BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
) -> Result<Vec<Item>, CfgError> {
    if !flow.reducible {
        return Err(CfgError::new("structure() 只接受可归约 CFG"));
    }
    let tries = try_infos(nodes, flow)?;
    let pl = place(nodes, flow, &tries)?;
    let mut b = Builder { nodes, flow, pl: &pl, emitted: Vec::new() };
    let mut tree = b.tree(flow.entry)?;
    // 归约后不是 follower 的节点，其汇合变量声明置于函数顶部
    let mut orphan = Vec::new();
    for &n in &flow.rpo {
        if !pl.follower_set.contains(&n) && get(nodes, n)?.has_decls() {
            orphan.push(n);
        }
    }
    if !orphan.is_empty() {
        tree.insert(0, Item::Decl(orphan));
    }
    let mut emitted = b.emitted;
    emitted.sort_unstable();
    let mut expected = flow.rpo.clone();
    expected.sort_unstable();
    if emitted != expected {
        let seen: BTreeSet<NodeId> = emitted.iter().copied().collect();
        let missing: Vec<NodeId> = expected.iter().copied().filter(|n| !seen.contains(n)).collect();
        let mut dup: Vec<NodeId> = emitted.windows(2).filter(|w| w[0] == w[1]).map(|w| w[0]).collect();
        dup.dedup();
        return Err(CfgError::new(format!("结构化未恰好输出每个块一次：缺失={missing:?} 重复={dup:?}")));
    }
    Ok(tree)
}
