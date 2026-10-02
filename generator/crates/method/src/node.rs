//! 归约节点（← `method/blocks.py` 的 `Node`）：一段 Rust 语句 + 一个终结
//! （cond / goto / switch / exit / 合成 try）。
//!
//! 终结字段按 Python 同名属性保存（逐块模拟期间逐步填写、归约期间改写）；结构化前经
//! [`Node::sync_term`] 生成 [`cfg::Terminal`] 视图供 [`StructNode`] 读取。

use std::collections::{BTreeMap, BTreeSet};

use cfg::{Cond, NodeId, StructNode, Terminal};
use ir::{Expr, Ident, Stmt};
use sim::{Local, StackEntry};
use ty::RsType;

use crate::cond_text::raw;
use crate::try_plan::CatchClause;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Cond,
    Goto,
    Switch,
    Exit,
    Try,
}

/// try 节点的一个 catch 子句：子句描述、异常绑定名、绑定类型
#[derive(Debug, Clone, PartialEq)]
pub struct Catch {
    pub clause: CatchClause,
    pub bind: Ident,
    pub bind_ty: RsType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub start_pc: u32,
    pub kind: Kind,
    pub target: Option<NodeId>,
    pub fallthrough: Option<NodeId>,
    pub cases: Vec<(Vec<i32>, NodeId)>,
    pub default: Option<NodeId>,
    pub cond: Option<Cond>,
    /// switch 判定表达式文本
    pub key: String,
    /// 本节点终结承载的跳转指令 pc
    pub pcs: Vec<u32>,
    pub stmts: Vec<Stmt>,
    /// 与 `stmts` 等长：语句来源指令的字节码偏移（合成语句为 None）；经 [`Node::push_stmt`] /
    /// [`Node::append_stmts`] 维护，发射时映射为 Java 行号
    pub stmt_pcs: Vec<Option<u32>>,
    /// 入口合并变量的声明
    pub decls: Vec<Stmt>,
    pub entry_stack: Vec<StackEntry>,
    pub entry_locals: BTreeMap<u16, Local>,
    pub exit_stack: Vec<StackEntry>,
    pub exit_locals: BTreeMap<u16, Local>,
    pub processed: bool,
    pub removed: bool,
    /// 覆盖本节点的 try 组编号集合
    pub ctx: BTreeSet<u32>,
    /// try 节点：组编号 / 各子句处理器入口 / 子句 / catch 体文本终点
    pub group: Option<u32>,
    pub handlers: Vec<NodeId>,
    pub catches: Vec<Catch>,
    pub catch_ends: Vec<Option<u32>>,
    /// 结构化视图（[`Node::sync_term`] 生成）
    pub term: Terminal,
}

impl Node {
    pub fn new(id: NodeId, start_pc: u32, kind: Kind) -> Node {
        Node {
            id,
            start_pc,
            kind,
            target: None,
            fallthrough: None,
            cases: Vec::new(),
            default: None,
            cond: None,
            key: String::new(),
            pcs: Vec::new(),
            stmts: Vec::new(),
            stmt_pcs: Vec::new(),
            decls: Vec::new(),
            entry_stack: Vec::new(),
            entry_locals: BTreeMap::new(),
            exit_stack: Vec::new(),
            exit_locals: BTreeMap::new(),
            processed: false,
            removed: false,
            ctx: BTreeSet::new(),
            group: None,
            handlers: Vec::new(),
            catches: Vec::new(),
            catch_ends: Vec::new(),
            term: Terminal::Exit,
        }
    }

    /// 后继（去重、保序；与 Python `Node.successors` 同序）
    pub fn successors(&self) -> Vec<NodeId> {
        let mut out: Vec<NodeId> = Vec::new();
        let mut add = |n: Option<NodeId>| {
            if let Some(n) = n {
                if !out.contains(&n) {
                    out.push(n);
                }
            }
        };
        match self.kind {
            Kind::Try => {
                add(self.target);
                self.handlers.iter().for_each(|&h| add(Some(h)));
            }
            Kind::Cond => {
                add(self.target);
                add(self.fallthrough);
            }
            Kind::Goto => add(self.target),
            Kind::Switch => {
                self.cases.iter().for_each(|&(_, t)| add(Some(t)));
                add(self.default);
            }
            Kind::Exit => {}
        }
        out
    }

    /// 追加一条语句（`pc` 为来源指令偏移，合成语句为 None）
    pub fn push_stmt(&mut self, stmt: Stmt, pc: Option<u32>) {
        self.stmts.push(stmt);
        self.stmt_pcs.push(pc);
    }

    /// 追加语句序列及其来源偏移（两者等长）
    pub fn append_stmts(&mut self, stmts: Vec<Stmt>, pcs: Vec<Option<u32>>) {
        debug_assert_eq!(stmts.len(), pcs.len());
        self.stmts.extend(stmts);
        self.stmt_pcs.extend(pcs);
    }

    /// 清空语句
    pub fn clear_stmts(&mut self) {
        self.stmts.clear();
        self.stmt_pcs.clear();
    }

    /// 第 k 条语句的来源偏移
    pub fn stmt_pc(&self, k: usize) -> Option<u32> {
        self.stmt_pcs.get(k).copied().flatten()
    }

    pub fn is_try(&self) -> bool {
        self.kind == Kind::Try
    }

    /// 全部目标槽位的改写（非 try 节点：target / fallthrough / default / cases）
    pub fn remap_targets(&mut self, f: &mut dyn FnMut(NodeId) -> NodeId) {
        self.target = self.target.map(&mut *f);
        self.fallthrough = self.fallthrough.map(&mut *f);
        self.default = self.default.map(&mut *f);
        for (_, t) in &mut self.cases {
            *t = f(*t);
        }
    }

    /// 由终结字段生成结构化视图
    pub fn sync_term(&mut self) {
        let t = |x: Option<NodeId>| x.unwrap_or(0);
        self.term = match self.kind {
            Kind::Exit => Terminal::Exit,
            Kind::Goto => Terminal::Goto { target: t(self.target) },
            Kind::Cond => Terminal::Cond {
                cond: self.cond.clone().unwrap_or(Cond::Const(true)),
                target: t(self.target),
                fallthrough: t(self.fallthrough),
            },
            Kind::Switch => Terminal::Switch { key: raw(self.key.clone()), cases: self.cases.clone(), default: t(self.default) },
            Kind::Try => Terminal::Try {
                body: t(self.target),
                handlers: self.handlers.clone(),
                group: self.group.unwrap_or(0),
                catch_ends: self.catch_ends.clone(),
            },
        };
    }
}

impl StructNode for Node {
    fn start_pc(&self) -> u32 {
        self.start_pc
    }
    fn terminal(&self) -> &Terminal {
        &self.term
    }
    fn has_stmts(&self) -> bool {
        !self.stmts.is_empty()
    }
    fn has_decls(&self) -> bool {
        !self.decls.is_empty()
    }
    fn ctx(&self) -> &BTreeSet<u32> {
        &self.ctx
    }
    fn ctx_mut(&mut self) -> &mut BTreeSet<u32> {
        &mut self.ctx
    }
    fn jump_pcs(&self) -> &[u32] {
        &self.pcs
    }
}

/// 节点图：编号 → 节点，另记 Python dict 的插入序（遍历序影响前驱顺序）
#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: BTreeMap<NodeId, Node>,
    pub order: Vec<NodeId>,
}

impl Graph {
    pub fn insert(&mut self, n: Node) {
        if !self.nodes.contains_key(&n.id) {
            self.order.push(n.id);
        }
        self.nodes.insert(n.id, n);
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[&id]
    }

    pub fn node_mut(&mut self, id: NodeId) -> &mut Node {
        self.nodes.get_mut(&id).expect("节点编号")
    }

    /// 按插入序遍历
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.order.iter().map(|id| &self.nodes[id])
    }

    pub fn retain(&mut self, keep: impl Fn(&Node) -> bool) {
        let drop: BTreeSet<NodeId> = self.iter().filter(|n| !keep(n)).map(|n| n.id).collect();
        self.order.retain(|id| !drop.contains(id));
        self.nodes.retain(|id, _| !drop.contains(id));
    }
}

/// 栈条目的「同一值」（Python `x is y or x[0] is y[0]`：同一身份且表达式未被改写）
pub fn same_value(a: &StackEntry, b: &StackEntry) -> bool {
    a.id == b.id && a.expr == b.expr
}

/// 条目表达式是否为某名字的变量
pub fn var_name(e: &Expr) -> Option<&str> {
    match e {
        Expr::Var(v) => Some(v.as_str()),
        _ => None,
    }
}
