//! 栈模拟的基本行为（不依赖 golden）：形参绑定、dup 物化、store 的 let / 赋值 / int 族收窄、下溢。

use ir::{Expr, Lit, Renderer, ShortNames};
use sim::{SimConfig, SimEnv, SimResult, StackEntry, StackSim};
use std::collections::BTreeMap;
use ty::{Prim, RsType};

struct Env;

impl ShortNames for Env {
    fn short_cls(&self, binary: &str) -> String {
        binary.rsplit('/').next().unwrap_or(binary).to_string()
    }
}

impl SimEnv for Env {
    fn short_name(&self, binary: &str) -> String {
        self.short_cls(binary)
    }
    fn is_subtype(&self, _: &RsType, _: &RsType) -> bool {
        false
    }
    fn is_interface(&self, _: &RsType) -> bool {
        false
    }
    fn carrier_type(&self, _: &RsType) -> Option<RsType> {
        None
    }
    fn box_object(&self, value: Expr, _: &RsType) -> SimResult<Expr> {
        Ok(value)
    }
    fn infer_type_args(&self, _: &RsType, _: &str) -> Option<Vec<RsType>> {
        None
    }
}

fn cfg(params: Vec<RsType>) -> SimConfig {
    SimConfig {
        params,
        is_static: false,
        class_name: "pkg/Foo".to_string(),
        class_type_params: vec!["T".to_string()],
        local_names: BTreeMap::new(),
        slot_decls: BTreeMap::new(),
        return_type: RsType::Unit,
        is_constructor: false,
        in_vtable_body: false,
    }
}

fn stmts(sim: &StackSim) -> Vec<String> {
    let rd = Renderer::new(&Env);
    sim.state.stmts.iter().map(|s| rd.stmt(s, 0)).collect()
}

fn int(v: i128) -> Expr {
    Expr::Lit(Lit::Int { value: v, ty: None })
}

#[test]
fn binds_this_and_wide_params() {
    let sim = StackSim::new(cfg(vec![RsType::Prim(Prim::I64), RsType::class("pkg/Bar", vec![])]), &Env).unwrap();
    let names: Vec<(u16, String)> = sim.state.locals.iter().map(|(k, l)| (*k, l.name.as_str().to_string())).collect();
    assert_eq!(names, vec![(0, "this".into()), (1, "arg_0".into()), (3, "arg_1".into())]);
    assert_eq!(sim.state.locals[&0].ty, RsType::class("pkg/Foo", vec![RsType::Param("T".into())]));
    assert_eq!(sim.state.param_slots.iter().copied().collect::<Vec<_>>(), vec![1, 3]);
}

#[test]
fn dup_materializes_once() {
    let mut sim = StackSim::new(cfg(vec![]), &Env).unwrap();
    let call = Expr::MethodCall { recv: Box::new(Expr::Var(ir::Ident::new("this").unwrap())), method: ir::Ident::new("next").unwrap(), turbofish: vec![], args: vec![] };
    let id = sim.push(call, RsType::Prim(Prim::I32));
    sim.push_entry(StackEntry { expr: sim.state.stack[0].expr.clone(), ty: RsType::Prim(Prim::I32), id });
    let top = sim.pop().unwrap();
    assert_eq!(stmts(&sim), vec!["let _t0 = this.next();"]);
    assert!(matches!(top.expr, Expr::Var(_)));
    assert!(matches!(sim.state.stack[0].expr, Expr::Var(_)));
}

#[test]
fn store_emits_let_then_assign_and_narrows_int_family() {
    let mut sim = StackSim::new(cfg(vec![]), &Env).unwrap();
    sim.state.current_offset = 4;
    sim.push(int(1), RsType::Prim(Prim::Bool));
    let v = sim.pop_for_store().unwrap();
    sim.store_local(1, v).unwrap();
    sim.push(int(2), RsType::Prim(Prim::I32));
    let v = sim.pop_for_store().unwrap();
    sim.store_local(1, v).unwrap();
    assert_eq!(stmts(&sim), vec!["let mut local_1: i32 = (1) as i32;", "local_1 = 2;"]);
    assert_eq!(sim.state.slot_bind_pos.get(&1), Some(&4));
}

#[test]
fn underflow_is_flagged() {
    let mut sim = StackSim::new(cfg(vec![]), &Env).unwrap();
    let e = sim.pop().unwrap();
    assert!(sim.state.underflow);
    assert_eq!(Renderer::new(&Env).expr(&e.expr), "(panic!(\"stack underflow\") as i32)");
}

/// 语句发射前，栈上有状态的待求值条目按栈序物化（`f() + "-" + x + "-" + g()`：`x` 的读取不晚于 `g()`）；
/// 变量 / 字面量不物化，dup 副本只物化一次
#[test]
fn statement_spills_stateful_pending_entries_in_stack_order() {
    let mut sim = StackSim::new(cfg(vec![]), &Env).unwrap();
    sim.push(Expr::Var(ir::Ident::new("local_1").unwrap()), RsType::Prim(Prim::I32));
    let id = sim.push(Expr::raw("Foo::step()?"), RsType::Prim(Prim::I32));
    sim.push_entry(StackEntry { expr: Expr::raw("Foo::step()?"), ty: RsType::Prim(Prim::I32), id });
    sim.push(int(7), RsType::Prim(Prim::I32));
    sim.push(Expr::raw("(local_1 + 1)"), RsType::Prim(Prim::I32));
    sim.emit(ir::Stmt::raw("Foo::make()?;")).unwrap();
    assert_eq!(stmts(&sim), vec!["let _t0 = Foo::step()?;", "Foo::make()?;"]);
    let texts: Vec<String> = sim.state.stack.iter().map(|e| Renderer::new(&Env).expr(&e.expr)).collect();
    assert_eq!(texts, vec!["local_1", "_t0", "_t0", "7", "(local_1 + 1)"]);
}

#[test]
fn reads_state_classifies_pending_values() {
    use sim::exprs::reads_state;
    assert!(reads_state(&Expr::raw("this.__get_n()?")));
    assert!(reads_state(&Expr::raw("Foo::<T>(x)")));
    assert!(!reads_state(&Expr::raw("(local_1 + 1)")));
    assert!(!reads_state(&Expr::raw("Clone::clone(&a)")));
    assert!(!reads_state(&int(3)));
    let concat = |arg: &str| Expr::Lit(Lit::JStringConcat { fmt: Some("{}-".into()), args: vec![Expr::raw(arg)] });
    assert!(reads_state(&concat("Foo::step()?")), "拼接字面量的实参读状态");
    assert!(!reads_state(&concat("_t3")));
}
