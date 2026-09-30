//! 结构化的输入节点（← `method/blocks.py` 的 `Node` 中被 cfg 读取的部分）。
//!
//! 逐块栈模拟（P4b）产出的归约节点图经 [`StructNode`] 交给 [`crate::structure()`]；
//! 节点的语句、栈状态等由 P4b 自行持有，cfg 只读取形状与 try 区域信息。

use std::collections::BTreeSet;

use ir::Expr;

use crate::cond::Cond;
use crate::graph::{switch_successors, NodeId};

/// 归约节点的终结。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Terminal {
    /// return / athrow
    Exit,
    Goto { target: NodeId },
    /// 条件成立 → `target`，否则 → `fallthrough`
    Cond { cond: Cond, target: NodeId, fallthrough: NodeId },
    /// `key` 为 match 的判定表达式
    Switch { key: Expr, cases: Vec<(Vec<i32>, NodeId)>, default: NodeId },
    /// 合成的 try 节点：`body` = try 体入口，`handlers` = 各 catch 子句的处理器入口（下标即
    /// catch 子句序号），`group` = try 组编号，`catch_ends` = 各 catch 体的文本终点 pc
    Try { body: NodeId, handlers: Vec<NodeId>, group: u32, catch_ends: Vec<Option<u32>> },
}

impl Terminal {
    pub fn successors(&self) -> Vec<NodeId> {
        match self {
            Terminal::Exit => Vec::new(),
            Terminal::Goto { target } => vec![*target],
            Terminal::Cond { target, fallthrough, .. } => {
                if target == fallthrough {
                    vec![*target]
                } else {
                    vec![*target, *fallthrough]
                }
            }
            Terminal::Switch { cases, default, .. } => switch_successors(cases, *default),
            Terminal::Try { body, handlers, .. } => {
                let mut out = vec![*body];
                for h in handlers {
                    if !out.contains(h) {
                        out.push(*h);
                    }
                }
                out
            }
        }
    }

    pub fn is_exit(&self) -> bool {
        matches!(self, Terminal::Exit)
    }

    pub fn is_try(&self) -> bool {
        matches!(self, Terminal::Try { .. })
    }
}

/// 结构化读取的节点视图。
pub trait StructNode {
    fn start_pc(&self) -> u32;
    fn terminal(&self) -> &Terminal;
    /// 节点有无直线语句（无 → 结构树的 `Code` 标记 empty）
    fn has_stmts(&self) -> bool;
    /// 节点有无入口合并变量声明（有 → 随 follower 标签块输出 `Decl`）
    fn has_decls(&self) -> bool;
    /// 覆盖本节点的 try 组编号集合
    fn ctx(&self) -> &BTreeSet<u32>;
    /// 结构化可能补齐外层 try 组（只以退出结束的处理器子树，见 `structure` 模块文档）
    fn ctx_mut(&mut self) -> &mut BTreeSet<u32>;
    /// 块终结承载的跳转指令 pc（结构树自检通过后记为 structured）
    fn jump_pcs(&self) -> &[u32];
}

/// [`StructNode`] 的朴素实现（测试与不需要额外状态的调用方）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowNode {
    pub start_pc: u32,
    pub term: Terminal,
    pub has_stmts: bool,
    pub has_decls: bool,
    pub ctx: BTreeSet<u32>,
    pub jump_pcs: Vec<u32>,
}

impl StructNode for FlowNode {
    fn start_pc(&self) -> u32 {
        self.start_pc
    }
    fn terminal(&self) -> &Terminal {
        &self.term
    }
    fn has_stmts(&self) -> bool {
        self.has_stmts
    }
    fn has_decls(&self) -> bool {
        self.has_decls
    }
    fn ctx(&self) -> &BTreeSet<u32> {
        &self.ctx
    }
    fn ctx_mut(&mut self) -> &mut BTreeSet<u32> {
        &mut self.ctx
    }
    fn jump_pcs(&self) -> &[u32] {
        &self.jump_pcs
    }
}
