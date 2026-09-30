//! 不可归约 CFG 的通用兜底：程序计数器状态机（← `cfg/dispatch.py`）。
//!
//! ```text
//! let mut __pc: i32 = <entry>;
//! loop { match __pc { <block> => { <stmts>; __pc = <next>; } ... _ => unreachable!(), } }
//! ```
//! 每个基本块是一个 match 臂，终结翻译为对 `__pc` 的赋值；javac 产物恒为可归约 CFG，
//! 本路径只服务于其他编译器 / 混淆器产出的字节码。
//!
//! 与 Python 的差异：switch 终结产出 `match key { vals => { __pc = t; } .. }` 语句，
//! Python 为 `__pc = match key { vals => t, .. };` 赋值表达式（ir 无 match 表达式节点），
//! 两者语义相同。

use ir::{ArmBody, AssignStmt, BlockExpr, Expr, Ident, IfExpr, Lit, MatchArm, MatchStmt, Pattern, Stmt, VarOrigin};

use crate::error::CfgError;
use crate::graph::NodeId;
use crate::node::Terminal;

/// 状态机变量名
pub const PC_VAR: &str = "__pc";

/// 结构树之外的状态机形态：`blocks` 按节点编号（字节码顺序）升序。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispatch {
    pub entry: NodeId,
    pub blocks: Vec<NodeId>,
}

pub fn build_dispatch(entry: NodeId, live: &[NodeId]) -> Dispatch {
    let mut blocks = live.to_vec();
    blocks.sort_unstable();
    Dispatch { entry, blocks }
}

/// 状态机块号字面量（无后缀，`__pc: i32` 定型）
pub fn pc_lit(n: NodeId) -> Expr {
    Expr::Lit(Lit::Int { value: i128::from(n), ty: None })
}

fn pc_var() -> Result<Expr, CfgError> {
    Ident::new(PC_VAR).map(Expr::Var).map_err(|e| CfgError::new(format!("{e}")))
}

fn assign_pc(value: Expr) -> Result<Stmt, CfgError> {
    Ok(Stmt::Assign(AssignStmt { target: pc_var()?, value, origin: VarOrigin::default() }))
}

fn tail_block(e: Expr) -> BlockExpr {
    BlockExpr { stmts: Vec::new(), tail: Some(Box::new(e)) }
}

/// 块的终结 → 对 `__pc` 赋值的语句（exit 块无后继，返回空）。try 节点不进入状态机。
pub fn next_pc_stmts(term: &Terminal) -> Result<Vec<Stmt>, CfgError> {
    Ok(match term {
        Terminal::Exit => Vec::new(),
        Terminal::Goto { target } => vec![assign_pc(pc_lit(*target))?],
        Terminal::Cond { cond, target, fallthrough } => {
            let value = Expr::If(IfExpr {
                cond: Box::new(cond.to_expr()),
                then: tail_block(pc_lit(*target)),
                else_: Some(tail_block(pc_lit(*fallthrough))),
            });
            vec![assign_pc(value)?]
        }
        Terminal::Switch { key, cases, default } => {
            let mut arms = Vec::with_capacity(cases.len() + 1);
            for (vals, tgt) in cases {
                let lits = vals.iter().map(|v| Lit::Int { value: i128::from(*v), ty: None }).collect();
                arms.push(MatchArm {
                    pattern: Pattern::Alts(lits),
                    body: ArmBody::Block(vec![assign_pc(pc_lit(*tgt))?]),
                });
            }
            arms.push(MatchArm {
                pattern: Pattern::Wildcard,
                body: ArmBody::Block(vec![assign_pc(pc_lit(*default))?]),
            });
            vec![Stmt::Match(MatchStmt { scrutinee: key.clone(), arms })]
        }
        Terminal::Try { .. } => return Err(CfgError::new("状态机兜底不承载 try 节点")),
    })
}
