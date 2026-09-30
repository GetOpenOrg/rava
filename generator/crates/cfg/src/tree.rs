//! 结构树节点（← `structure.py` 的 Code / Decl / Block / Loop / If / Switch / Try / Break /
//! Continue）与遍历。
//!
//! 与 Python 的差异：
//! - `Decl` 记录「哪些节点的合并变量声明在此输出」（节点编号，按输出序），声明语句本身
//!   仍由节点持有（Python 复制声明行文本）；
//! - `Try` 的 catch 臂以 catch 子句序号（[`crate::Terminal::Try`] 的 `handlers` 下标）
//!   引用子句描述（Python 复制描述元组）；
//! - 循环出口标签 `('loop-exit', header)` 元组结构化为 [`BreakLabel::LoopExit`]。

use ir::Expr;

use crate::cond::Cond;
use crate::graph::NodeId;

/// break 的目标：带标签块（follower 节点编号）或循环自身的出口。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BreakLabel {
    Block(NodeId),
    LoopExit(NodeId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopItem {
    pub header: NodeId,
    pub body: Vec<Item>,
    /// 合并进来的出口标签：`Break(exit_label)` = 跳出本循环
    pub exit_label: Option<BreakLabel>,
    /// 非 None → while 形态
    pub while_cond: Option<Cond>,
    /// while 条件来源块
    pub cond_origin: Option<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfItem {
    pub cond: Cond,
    pub then: Vec<Item>,
    pub else_: Vec<Item>,
    pub origin: NodeId,
}

/// switch 臂：`values` 为 None 即 default 臂。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchArm {
    pub values: Option<Vec<i32>>,
    pub body: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchItem {
    pub key: Expr,
    pub arms: Vec<SwitchArm>,
    pub origin: NodeId,
}

/// catch 臂：`clause` 为 try 节点的 catch 子句序号。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchArm {
    pub clause: usize,
    pub body: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryItem {
    pub body: Vec<Item>,
    pub catches: Vec<CatchArm>,
    pub origin: NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// 节点的直线代码：`exits` = 以 return / athrow 结束，`empty` = 无语句
    Code { block: NodeId, exits: bool, empty: bool },
    /// 这些节点的入口合并变量声明
    Decl(Vec<NodeId>),
    /// 带标签块 `'bN: { body }`
    Block { label: NodeId, body: Vec<Item> },
    Loop(LoopItem),
    If(IfItem),
    Switch(SwitchItem),
    Try(TryItem),
    Break(BreakLabel),
    Continue(NodeId),
}

impl Item {
    /// 空且不退出的代码块（可读性规整中不计入「有效项」）
    pub fn is_insignificant(&self) -> bool {
        matches!(self, Item::Code { empty: true, exits: false, .. })
    }

    /// 直接子序列（先序遍历用）
    pub fn children(&self) -> Vec<&[Item]> {
        match self {
            Item::Block { body, .. } => vec![body],
            Item::Loop(l) => vec![&l.body],
            Item::If(i) => vec![&i.then, &i.else_],
            Item::Switch(s) => s.arms.iter().map(|a| a.body.as_slice()).collect(),
            Item::Try(t) => {
                let mut v: Vec<&[Item]> = vec![&t.body];
                v.extend(t.catches.iter().map(|c| c.body.as_slice()));
                v
            }
            _ => Vec::new(),
        }
    }
}

/// 有效项（去掉空且不退出的代码块）
pub fn significant(seq: &[Item]) -> Vec<&Item> {
    seq.iter().filter(|it| !it.is_insignificant()).collect()
}

/// 先序遍历全部节点（← `simplify.walk`）。
pub fn walk<'a>(seq: &'a [Item], out: &mut Vec<&'a Item>) {
    for it in seq {
        out.push(it);
        for child in it.children() {
            walk(child, out);
        }
    }
}
