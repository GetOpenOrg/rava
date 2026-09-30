//! 结构树 → 扁平 entries（← `method/emit.py`）。
//!
//! 标签策略：循环 `'lN`、带标签块 `'bN`，按先序编号；只有被 break / continue 显式引用时
//! 才输出循环标签。break / continue 指向最内层循环且中间没有隔着带标签块时省略标签
//! （Rust E0695：带标签块内部不允许出现无标签的 break / continue）。
//!
//! try 区域输出为 `java_try! { try { .. } catch (e: T) { .. } }`。跨越 try 边界的
//! break / continue 一律带标签（宏对带标签跳转原样放行）。
//!
//! Python 先产出 `BlockStmt` 结构块再 `flatten`；这里直接按同一顺序产出扁平序列
//! （`emit_tree = flatten ∘ emit_tree_structured`）。

use std::collections::{BTreeMap, BTreeSet};

use cfg::tree::significant;
use cfg::{BreakLabel, Dispatch, IfItem, Item, NodeId, TryItem, PC_VAR};
use instr::InstrEnv;
use ir::Stmt;

use crate::cond_text::render_cond;
use crate::entry::{Entry, Tag};
use crate::node::{Graph, Kind, Node};
use crate::text;
use crate::try_plan::catch_head;

const STEP: &str = "    ";

/// 跳转上下文：由外到内的循环 / 带标签块（try 边界记为无标签块）
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ctx {
    Loop(NodeId),
    Block,
}

struct TreeEmitter<'a> {
    env: &'a InstrEnv<'a>,
    nodes: &'a Graph,
    loop_names: BTreeMap<NodeId, String>,
    block_names: BTreeMap<NodeId, String>,
    exit_owner: BTreeMap<BreakLabel, NodeId>,
    /// 被显式引用的循环标签
    used: BTreeSet<NodeId>,
}

/// 结构树 → entries（缺省缩进 4 空格）
pub fn emit_tree(env: &InstrEnv, tree: &[Item], nodes: &Graph) -> Vec<Entry> {
    let mut em = TreeEmitter {
        env,
        nodes,
        loop_names: BTreeMap::new(),
        block_names: BTreeMap::new(),
        exit_owner: BTreeMap::new(),
        used: BTreeSet::new(),
    };
    em.number(tree);
    let mut out = Vec::new();
    em.seq(tree, STEP, &[], &mut out);
    out
}

/// 状态机兜底 → entries：`let mut __pc` 前导 + `loop { match __pc { .. } }`
pub fn emit_dispatch(d: &Dispatch, nodes: &Graph) -> Vec<Entry> {
    let ind = STEP;
    let mut out = vec![Entry::line(format!("{ind}let mut {PC_VAR}: i32 = {};", d.entry))];
    out.push(Entry::structure(format!("{ind}loop {{"), 1, Tag::Loop));
    out.push(Entry::structure(format!("{ind}{STEP}match {PC_VAR} {{"), 1, Tag::Plain));
    let arm_ind = format!("{ind}{STEP}{STEP}");
    for &bid in &d.blocks {
        let node = nodes.node(bid);
        out.push(Entry::structure(format!("{arm_ind}{bid} => {{"), 1, Tag::Arm));
        let body_ind = format!("{arm_ind}{STEP}");
        for s in &node.stmts {
            out.push(Entry::stmt(&body_ind, s.clone()));
        }
        for line in next_pc_lines(node) {
            out.push(Entry::line(format!("{body_ind}{line}")));
        }
        out.push(Entry::structure(format!("{arm_ind}}}"), -1, Tag::Plain));
    }
    out.push(Entry::structure(format!("{arm_ind}_ => unreachable!(),"), 0, Tag::Plain));
    out.push(Entry::structure(format!("{ind}{STEP}}}"), -1, Tag::Plain));
    out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
    out
}

/// 块的终结 → 对 `__pc` 赋值的文本行（← `cfg.dispatch.next_pc_lines`；exit 块无后继）
fn next_pc_lines(node: &Node) -> Vec<String> {
    let t = |x: Option<NodeId>| x.map_or_else(|| "None".to_string(), |v| v.to_string());
    match node.kind {
        Kind::Exit | Kind::Try => Vec::new(),
        Kind::Goto => vec![format!("{PC_VAR} = {};", t(node.target))],
        Kind::Cond => {
            let c = node.cond.as_ref().map(render_cond).unwrap_or_default();
            vec![format!("{PC_VAR} = if {c} {{ {} }} else {{ {} }};", t(node.target), t(node.fallthrough))]
        }
        Kind::Switch => {
            let mut out = vec![format!("{PC_VAR} = match {} {{", node.key)];
            for (vals, tgt) in &node.cases {
                let vs: Vec<String> = vals.iter().map(i32::to_string).collect();
                out.push(format!("    {} => {tgt},", vs.join(" | ")));
            }
            out.push(format!("    _ => {},", t(node.default)));
            out.push("};".to_string());
            out
        }
    }
}

impl TreeEmitter<'_> {
    // ── 标签编号（先序）──

    fn number(&mut self, seq: &[Item]) {
        for it in seq {
            match it {
                Item::Loop(l) => {
                    self.loop_names.insert(l.header, format!("'l{}", self.loop_names.len()));
                    if let Some(x) = l.exit_label {
                        self.exit_owner.insert(x, l.header);
                    }
                    self.number(&l.body);
                }
                Item::Block { label, body } => {
                    self.block_names.insert(*label, format!("'b{}", self.block_names.len()));
                    self.number(body);
                }
                Item::If(i) => {
                    self.number(&i.then);
                    self.number(&i.else_);
                }
                Item::Switch(s) => {
                    for a in &s.arms {
                        self.number(&a.body);
                    }
                }
                Item::Try(t) => {
                    self.number(&t.body);
                    for c in &t.catches {
                        self.number(&c.body);
                    }
                }
                _ => {}
            }
        }
    }

    // ── 输出 ──

    fn jump(&mut self, keyword: &str, header: NodeId, ctx: &[Ctx]) -> String {
        // 最内层是本循环（且未隔着带标签块 / try 边界）→ 省略标签
        if ctx.last() == Some(&Ctx::Loop(header)) {
            return format!("{keyword};");
        }
        self.used.insert(header);
        format!("{keyword} {};", self.loop_names[&header])
    }

    fn block_label(&self, label: BreakLabel) -> &str {
        let id = match label {
            BreakLabel::Block(n) | BreakLabel::LoopExit(n) => n,
        };
        &self.block_names[&id]
    }

    fn seq(&mut self, seq: &[Item], ind: &str, ctx: &[Ctx], out: &mut Vec<Entry>) {
        for it in seq {
            match it {
                Item::Code { block, .. } => {
                    for s in &self.nodes.node(*block).stmts {
                        out.push(Entry::stmt(ind, s.clone()));
                    }
                }
                Item::Decl(ids) => {
                    for id in ids {
                        for d in &self.nodes.node(*id).decls {
                            out.push(Entry::stmt(ind, d.clone()));
                        }
                    }
                }
                Item::Break(label) => {
                    let line = match self.exit_owner.get(label) {
                        Some(&h) => self.jump("break", h, ctx),
                        None => format!("break {};", self.block_label(*label)),
                    };
                    out.push(Entry::line(format!("{ind}{line}")));
                }
                Item::Continue(h) => {
                    let line = self.jump("continue", *h, ctx);
                    out.push(Entry::line(format!("{ind}{line}")));
                }
                Item::Block { label, body } => {
                    let mut inner = Vec::new();
                    self.seq(body, &format!("{ind}{STEP}"), &with(ctx, Ctx::Block), &mut inner);
                    let name = &self.block_names[label];
                    out.push(Entry::structure(format!("{ind}{name}: {{"), 1, Tag::Plain));
                    out.append(&mut inner);
                    out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
                }
                Item::Loop(l) => {
                    let mut inner = Vec::new();
                    self.seq(&l.body, &format!("{ind}{STEP}"), &with(ctx, Ctx::Loop(l.header)), &mut inner);
                    let label =
                        if self.used.contains(&l.header) { format!("{}: ", self.loop_names[&l.header]) } else { String::new() };
                    let (head, tag) = match &l.while_cond {
                        Some(c) => (format!("while {} {{", render_cond(c)), Tag::Plain),
                        None => ("loop {".to_string(), Tag::Loop),
                    };
                    out.push(Entry::structure(format!("{ind}{label}{head}"), 1, tag));
                    out.append(&mut inner);
                    out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
                }
                Item::If(i) => self.if_(i, ind, ctx, out, ""),
                Item::Switch(s) => {
                    let key = &self.nodes.node(s.origin).key;
                    out.push(Entry::structure(format!("{ind}match {key} {{"), 1, Tag::Plain));
                    for arm in &s.arms {
                        let pattern = match &arm.values {
                            None => "_".to_string(),
                            Some(vs) => vs.iter().map(i32::to_string).collect::<Vec<_>>().join(" | "),
                        };
                        out.push(Entry::structure(format!("{ind}{STEP}{pattern} => {{"), 1, Tag::Arm));
                        self.seq(&arm.body, &format!("{ind}{STEP}{STEP}"), ctx, out);
                        out.push(Entry::structure(format!("{ind}{STEP}}}"), -1, Tag::Plain));
                    }
                    out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
                }
                Item::Try(t) => self.try_(t, ind, ctx, out),
            }
        }
    }

    /// `prefix` 非空：else-if 链的后继臂（`} else if c {` 直接接在前一臂子序列之后）
    fn if_(&mut self, it: &IfItem, ind: &str, ctx: &[Ctx], out: &mut Vec<Entry>, prefix: &str) {
        let then_sig = significant(&it.then);
        let else_sig = significant(&it.else_);
        if then_sig.is_empty() && else_sig.is_empty() {
            // 两臂皆空：条件只为副作用求值
            out.push(Entry::stmt(ind, Stmt::Raw(ir::Raw(format!("let _ = {};", render_cond(&it.cond))))));
            return;
        }
        let (delta, tag) = if prefix.is_empty() { (1, Tag::Plain) } else { (0, Tag::Else) };
        out.push(Entry::structure(format!("{ind}{prefix}if {} {{", render_cond(&it.cond)), delta, tag));
        self.seq(&it.then, &format!("{ind}{STEP}"), ctx, out);
        if !else_sig.is_empty() {
            if let [Item::If(inner)] = else_sig.as_slice() {
                if !significant(&inner.then).is_empty() || !significant(&inner.else_).is_empty() {
                    self.if_(inner, ind, ctx, out, "} else ");
                    return;
                }
            }
            out.push(Entry::structure(format!("{ind}}} else {{"), 0, Tag::Else));
            self.seq(&it.else_, &format!("{ind}{STEP}"), ctx, out);
        }
        out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
    }

    fn try_(&mut self, it: &TryItem, ind: &str, ctx: &[Ctx], out: &mut Vec<Entry>) {
        // try 边界：其内的 break / continue 必须带标签
        let inner_ctx = with(ctx, Ctx::Block);
        let body_ind = format!("{ind}{STEP}{STEP}");
        out.push(Entry::structure(format!("{ind}java_try! {{"), 1, Tag::Plain));
        out.push(Entry::structure(format!("{ind}{STEP}try {{"), 1, Tag::Try));
        self.seq(&it.body, &body_ind, &inner_ctx, out);
        let try_node = self.nodes.node(it.origin);
        for arm in &it.catches {
            let c = &try_node.catches[arm.clause];
            let mut bind = c.bind.as_str().to_string();
            let bind_ty = text::ty(self.env, &c.bind_ty);
            let mut handler = Vec::new();
            self.seq(&arm.body, &body_ind, &inner_ctx, &mut handler);
            // 处理器首条 astore 生成的 `let e: T = _caughtN;` 并入 catch 头
            if let Some(Stmt::Let(first)) = handler.first().and_then(Entry::as_stmt) {
                if let (Some(v), Some(t)) = (&first.value, &first.ty) {
                    let vs = text::expr(self.env, v);
                    if (vs == bind || vs == format!("Clone::clone(&{bind})")) && ir::render::render_type(t) == bind_ty {
                        bind = first.name.as_str().to_string();
                        handler.remove(0);
                    }
                }
            }
            let head = catch_head(&self.env.ctx, &c.clause, &bind, &bind_ty);
            out.push(Entry::structure(format!("{ind}{STEP}}} {head} {{"), 0, Tag::Catch));
            out.append(&mut handler);
        }
        out.push(Entry::structure(format!("{ind}{STEP}}}"), -1, Tag::Plain));
        out.push(Entry::structure(format!("{ind}}}"), -1, Tag::Plain));
    }
}

fn with(ctx: &[Ctx], c: Ctx) -> Vec<Ctx> {
    let mut v = ctx.to_vec();
    v.push(c);
    v
}
