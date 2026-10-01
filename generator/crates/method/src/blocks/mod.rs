//! 逐块栈模拟 + CFG 级归约（← `method/blocks.py`）。
//!
//! 输入：方法指令序列的基本块 CFG 与一个 [`StackSim`]。
//! 输出：归约后的节点图（每个节点 = 一段 Rust 语句 + 一个终结：cond / goto / switch / exit /
//! 合成 try），交给 `cfg::structure` 结构化（不可归约时走状态机）。
//!
//! - 每个块以「入口状态」（操作数栈 + 局部变量表）启动模拟；入口状态由已处理的前向前驱汇合
//! - 汇合点上各前驱留下的不同栈值 → 合并变量 `_mergedN`（前驱末尾赋值）
//! - 跳转指令由本层解释（弹操作数、构造条件），不经过 `sim_instr`
//!
//! 归约（全部保语义，且每条被吸收的跳转都记入账本）：fuse / short-circuit / ternary /
//! const-fold（见 [`fusion`]）。

mod dispatch;
mod fusion;
mod merge;
mod structured;
mod trynodes;

use std::collections::{BTreeMap, BTreeSet};

use cfg::opcodes::{is_cond_branch, is_jump, is_switch};
use cfg::{analyze, build_blocks, reachable, Block, JumpKind, JumpLedger, NodeId, Succs, Terminator};
use classfile::extras::LocalVar;
use classfile::{ExceptionEntry, Insn, Operand};
use input::NInsn;
use instr::{sim_instr, InstrEnv, InstrLog};
use ir::{Expr, Ident, Stmt};
use sim::{StackEntry, StackSim};
use ty::RsType;

use crate::cond_text::render_cond;
use crate::error::{cfg_err, MethodResult};
use crate::node::{Graph, Kind, Node};
use crate::text;
use crate::try_plan::TryPlan;
use crate::unify::{jump_condition, CondValues};

/// 处理器入口的绑定：所属 try 节点、异常变量名、绑定类型
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerBind {
    pub try_node: NodeId,
    pub bind: Ident,
    pub ty: RsType,
}

/// 模拟结果
#[derive(Debug, Clone)]
pub struct SimOutcome {
    /// 保留的节点（结构化路径：已处理且未移除；状态机路径：全部活块）
    pub nodes: Graph,
    pub entry: NodeId,
    pub dispatch: bool,
    /// 状态机模式下提升到函数顶部的声明行
    pub top_decls: Vec<String>,
    pub plan: TryPlan,
}

/// 块模拟器
pub struct Blocks<'s, 'e> {
    pub(crate) env: &'e InstrEnv<'e>,
    pub(crate) sim: &'s mut StackSim<'e>,
    pub(crate) log: &'s mut InstrLog,
    pub(crate) ledger: &'s mut JumpLedger,
    code: &'s [NInsn],
    /// CFG 视图：FoldCall → 调用指令，FoldField → nop，NullRecv / NoReturn → athrow
    insns: Vec<Insn>,
    exception_table: &'s [ExceptionEntry],
    pub(crate) plan: TryPlan,
    blocks: Vec<Block>,
    pub(crate) nodes: Graph,
    pub(crate) entry: NodeId,
    pub(crate) handler_bind: BTreeMap<NodeId, HandlerBind>,
    try_entries: BTreeSet<NodeId>,
    pub(crate) catch_exits: BTreeSet<NodeId>,
    rpo: Vec<NodeId>,
    idom: BTreeMap<NodeId, NodeId>,
    pub(crate) conds: CondValues,
}

/// 规范化指令的 CFG 视图：控制流终点（null_recv / noreturn 调用）视同 athrow——无正常后继，
/// 只经覆盖它的异常处理器转移
pub fn cfg_view(code: &[NInsn]) -> Vec<Insn> {
    code.iter()
        .map(|n| match n.insn() {
            _ if n.is_abrupt() => Insn { offset: n.offset(), opcode: classfile::insn::op::ATHROW, operand: Operand::None },
            Some(i) => i.clone(),
            None => Insn { offset: n.offset(), opcode: 0x00, operand: Operand::None },
        })
        .collect()
}

fn is_return_op(opc: u8) -> bool {
    (classfile::insn::op::IRETURN..=classfile::insn::op::RETURN).contains(&opc)
}

impl<'s, 'e> Blocks<'s, 'e> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        env: &'e InstrEnv<'e>,
        sim: &'s mut StackSim<'e>,
        log: &'s mut InstrLog,
        ledger: &'s mut JumpLedger,
        code: &'s [NInsn],
        exception_table: &'s [ExceptionEntry],
        local_vars: &[LocalVar],
        conds: CondValues,
    ) -> MethodResult<Blocks<'s, 'e>> {
        let insns = cfg_view(code);
        let plan = TryPlan::new(exception_table, &insns, local_vars);
        let blocks = build_blocks(&insns, exception_table, &plan.catch_body_ends())?;
        Ok(Blocks {
            env,
            sim,
            log,
            ledger,
            code,
            insns,
            exception_table,
            plan,
            blocks,
            nodes: Graph::default(),
            entry: 0,
            handler_bind: BTreeMap::new(),
            try_entries: BTreeSet::new(),
            catch_exits: BTreeSet::new(),
            rpo: Vec::new(),
            idom: BTreeMap::new(),
            conds,
        })
    }

    /// 运行模拟（`run`）
    pub fn run(mut self) -> MethodResult<SimOutcome> {
        let mut staged = Graph::default();
        for b in &self.blocks {
            staged.insert(staged_node(b));
        }
        self.install_try_nodes(&mut staged)?;

        let raw_reach = reachable(self.entry, &succs_of(&staged));
        self.register_jumps(&raw_reach);
        self.thread_jumps(&mut staged);

        let flow0 = analyze(self.entry, &succs_of(&staged));
        self.rpo = flow0.rpo.clone();
        self.idom = flow0.idom.clone();
        let live: BTreeSet<NodeId> = flow0.rpo.iter().copied().collect();
        for &nid in &flow0.rpo {
            self.nodes.insert(staged.node(nid).clone());
        }
        for n in staged.iter() {
            // 线程化后失去全部前驱的纯 goto 块：其跳转已由前驱的边承载
            if raw_reach.contains(&n.id) && !live.contains(&n.id) {
                for &pc in &n.pcs {
                    self.ledger.consume(pc, JumpKind::Structured);
                }
            }
        }
        if !flow0.reducible {
            if !self.handler_bind.is_empty() {
                return cfg_err("含异常处理器的不可归约 CFG");
            }
            return self.run_dispatch();
        }
        self.run_structured()
    }

    // ── 图查询 ──────────────────────────────────────────────────────────────

    /// 全部未移除的前驱（节点插入序）
    pub(crate) fn all_preds(&self, nid: NodeId) -> Vec<NodeId> {
        self.nodes.iter().filter(|p| !p.removed && p.successors().contains(&nid)).map(|p| p.id).collect()
    }

    /// 已处理且未移除的前驱（初始 rpo 序）
    pub(crate) fn live_preds(&self, nid: NodeId) -> Vec<NodeId> {
        self.rpo
            .iter()
            .map(|p| self.nodes.node(*p))
            .filter(|p| p.processed && !p.removed && p.successors().contains(&nid))
            .map(|p| p.id)
            .collect()
    }

    pub(crate) fn consume(&mut self, pcs: &[u32], kind: JumpKind) {
        for &pc in pcs {
            self.ledger.consume(pc, kind);
        }
    }

    /// 新身份的栈条目
    pub(crate) fn new_entry(&mut self, expr: Expr, ty: RsType) -> StackEntry {
        let id = self.sim.state.next_id;
        self.sim.state.next_id += 1;
        StackEntry { expr, ty, id }
    }

    // ── 单块模拟 ────────────────────────────────────────────────────────────

    pub(crate) fn simulate(&mut self, nid: NodeId) -> MethodResult<()> {
        let node = self.nodes.node(nid);
        if node.is_try() {
            // 合成节点没有指令：出口状态 = 入口状态
            let n = self.nodes.node_mut(nid);
            n.exit_stack = n.entry_stack.clone();
            n.exit_locals = n.entry_locals.clone();
            n.processed = true;
            return Ok(());
        }
        let start_pc = node.start_pc;
        let blk = &self.blocks[nid as usize];
        let (start, end) = (blk.start_idx, blk.end_idx);
        self.sim.state.stack = node.entry_stack.clone();
        self.sim.state.locals = node.entry_locals.clone();
        self.sim.state.stmts = Vec::new();
        let mut cond = None;
        let mut key = None;
        for i in start..end {
            let ins = &self.code[i];
            self.sim.state.current_offset = ins.offset();
            self.sim.state.next_offset = self.code.get(i + 1).map_or(0, NInsn::offset);
            let opc = self.insns[i].opcode;
            if i == end - 1 && is_jump(opc) {
                if is_cond_branch(opc) {
                    cond = Some(jump_condition(self.env, self.sim, &self.conds, opc)?);
                } else if is_switch(opc) {
                    let k = self.sim.pop()?;
                    let ks = text::expr(self.env, &k.expr);
                    key = Some(if text::ty(self.env, &k.ty) == "i32" { ks } else { format!("(({ks}) as i32)") });
                }
                break;
            }
            sim_instr(self.env, self.sim, self.log, ins)?;
        }
        if self.sim.state.underflow {
            return cfg_err(format!("操作数栈下溢（块 pc={start_pc}）"));
        }
        let stmts = std::mem::take(&mut self.sim.state.stmts);
        let n = self.nodes.node_mut(nid);
        if cond.is_some() {
            n.cond = cond;
        }
        if let Some(k) = key {
            n.key = k;
        }
        n.stmts = stmts;
        n.exit_stack = self.sim.state.stack.clone();
        n.exit_locals = self.sim.state.locals.clone();
        n.processed = true;
        Ok(())
    }

    // ── 常量条件折叠 ────────────────────────────────────────────────────────

    pub(crate) fn fold_const(&mut self, nid: NodeId) {
        let n = self.nodes.node(nid);
        if n.kind != Kind::Cond {
            return;
        }
        let Some(cond) = n.cond.clone() else { return };
        let pcs = n.pcs.clone();
        if let cfg::Cond::Const(v) = cond {
            let n = self.nodes.node_mut(nid);
            n.target = if v { n.target } else { n.fallthrough };
            (n.kind, n.cond, n.fallthrough) = (Kind::Goto, None, None);
            n.pcs.clear();
            self.consume(&pcs, JumpKind::ConstFold);
        } else if n.target == n.fallthrough {
            // 两臂同一目标：条件只为副作用求值
            let n = self.nodes.node_mut(nid);
            n.stmts.push(Stmt::raw(format!("let _ = {};", render_cond(&cond))));
            (n.kind, n.cond, n.fallthrough) = (Kind::Goto, None, None);
            n.pcs.clear();
            self.consume(&pcs, JumpKind::Structured);
        }
    }
}

/// 块 → 待装节点（fall 视为 goto）
fn staged_node(b: &Block) -> Node {
    let kind = match b.term {
        Terminator::Cond { .. } => Kind::Cond,
        Terminator::Goto { .. } | Terminator::Fall { .. } => Kind::Goto,
        Terminator::Switch { .. } => Kind::Switch,
        Terminator::Exit => Kind::Exit,
    };
    let mut n = Node::new(b.id, b.start_pc, kind);
    match &b.term {
        Terminator::Cond { target, fallthrough, .. } => {
            n.target = Some(*target);
            n.fallthrough = Some(*fallthrough);
        }
        Terminator::Goto { target, .. } | Terminator::Fall { target } => n.target = Some(*target),
        Terminator::Switch { cases, default, .. } => {
            n.cases = cases.clone();
            n.default = Some(*default);
        }
        Terminator::Exit => {}
    }
    n.pcs = b.term.pc().into_iter().collect();
    n
}

pub(crate) fn succs_of(g: &Graph) -> Succs {
    g.iter().map(|n| (n.id, n.successors())).collect()
}
