//! 状态机路径（不可归约 CFG；← `BlockSimulator._run_dispatch`）：块间栈值经 `__s{块}_{k}`
//! 溢出变量传递，块内 let 声明提升到函数顶部。

use std::collections::BTreeMap;

use cfg::{JumpKind, NodeId};
use ir::{AssignStmt, Expr, Raw, Stmt, VarOrigin};
use sim::{Local, StackEntry};

use super::{Blocks, SimOutcome};
use crate::error::{cfg_err, MethodResult};
use crate::text;
use crate::unify::arm_value;

impl Blocks<'_, '_> {
    pub(super) fn run_dispatch(mut self) -> MethodResult<SimOutcome> {
        let mut spill: BTreeMap<NodeId, Vec<StackEntry>> = BTreeMap::new();
        let mut top_decls: Vec<String> = Vec::new();
        let mut entry_locals: BTreeMap<NodeId, BTreeMap<u16, Local>> = BTreeMap::from([(0, self.sim.state.locals.clone())]);
        for nid in self.rpo.clone() {
            let n = self.nodes.node_mut(nid);
            n.entry_stack = spill.get(&nid).cloned().unwrap_or_default();
            n.entry_locals = entry_locals.get(&nid).cloned().unwrap_or_default();
            self.simulate(nid)?;
            let node = self.nodes.node(nid).clone();
            let succs = node.successors();
            let mut values = Vec::new();
            let mut extra = Vec::new();
            for e in &node.exit_stack {
                let mut e = e.clone();
                if succs.len() > 1 && !matches!(e.expr, Expr::Var(_)) {
                    let tmp = self.sim.fresh("_spill")?;
                    extra.push(Stmt::Raw(Raw(format!("let {tmp} = {};", text::expr(self.env, &e.expr)))));
                    e.expr = Expr::Var(tmp);
                }
                values.push(e);
            }
            for s in succs {
                if let std::collections::btree_map::Entry::Vacant(slot) = spill.entry(s) {
                    let mut vars = Vec::new();
                    for (k, v) in values.iter().enumerate() {
                        let var = ir::Ident::new(format!("__s{s}_{k}"))?;
                        top_decls.push(format!("let mut {var}: {} = Default::default();", text::ty(self.env, &v.ty)));
                        vars.push(self.new_entry(Expr::Var(var), v.ty.clone()));
                    }
                    slot.insert(vars);
                    entry_locals.insert(s, node.exit_locals.clone());
                }
                let vars = &spill[&s];
                if vars.len() != values.len() {
                    return cfg_err(format!("状态机：pc={} 各前驱栈深不一致", self.nodes.node(s).start_pc));
                }
                for (var, v) in vars.iter().zip(&values) {
                    extra.push(Stmt::Raw(Raw(format!("{} = {};", text::expr(self.env, &var.expr), arm_value(self.env, v)?))));
                }
            }
            self.nodes.node_mut(nid).stmts.extend(extra);
            self.consume(&node.pcs, JumpKind::Dispatch);
        }

        self.promote_cross_block_temps()?;
        let mut hoisted: Vec<(String, Option<String>)> = Vec::new();
        let order = self.nodes.order.clone();
        for id in order {
            let node = self.nodes.node(id);
            let mut new_stmts = Vec::new();
            for s in &node.stmts {
                let Stmt::Let(l) = s else {
                    new_stmts.push(s.clone());
                    continue;
                };
                let mut ty_s = l.ty.as_ref().map(ir::render::render_type);
                if ty_s.is_none() {
                    ty_s = node.exit_locals.values().find(|x| x.name == l.name).map(|x| text::ty(self.env, &x.ty));
                }
                let name = l.name.as_str().to_string();
                let pos = hoisted.iter().position(|(n, _)| *n == name);
                let prev = pos.and_then(|p| hoisted[p].1.clone());
                if let (Some(p), Some(t)) = (&prev, &ty_s) {
                    if p != t {
                        return cfg_err(format!("状态机：变量 {name} 类型冲突 {p} / {t}"));
                    }
                }
                let merged = prev.or(ty_s);
                match pos {
                    Some(p) => hoisted[p].1 = merged,
                    None => hoisted.push((name, merged)),
                }
                if let Some(v) = &l.value {
                    new_stmts.push(Stmt::Assign(AssignStmt {
                        target: Expr::Var(l.name.clone()),
                        value: v.clone(),
                        origin: VarOrigin::default(),
                    }));
                }
            }
            self.nodes.node_mut(id).stmts = new_stmts;
        }
        for (name, ty_s) in hoisted {
            let ann = ty_s.map(|t| format!(": {t}")).unwrap_or_default();
            top_decls.push(format!("let mut {name}{ann} = Default::default();"));
        }
        Ok(SimOutcome { nodes: self.nodes, entry: 0, dispatch: true, top_decls, plan: self.plan })
    }
}
