//! [`sim::SimEnv`] 的实现（← `method/codegen.py` 构造 `StackSim` 时注入的回调：
//! `_sim_is_subtype` / `_sim_is_interface` / `_sim_box_object` / `_sim_infer_type_args`）。
//!
//! 方法体生成（P4c）以 `InstrEnv::new(ctx, class_type_params)` 构造，再交给
//! `StackSim::new(cfg, &env)`；instr 的各处理函数也经它渲染判定文本。

use std::collections::BTreeSet;

use ir::Expr;
use sim::{SimEnv, SimError, SimResult};
use ty::RsType;

use crate::coerce;
use crate::ctx::InstrCtx;
use crate::hierarchy;

pub struct InstrEnv<'a> {
    pub ctx: InstrCtx<'a>,
    /// 当前方法作用域的类型形参（类的有效类型形参）
    pub tparams: Vec<String>,
}

impl<'a> InstrEnv<'a> {
    pub fn new(ctx: InstrCtx<'a>, tparams: &[String]) -> InstrEnv<'a> {
        InstrEnv { ctx, tparams: tparams.to_vec() }
    }

    pub fn tparam_set(&self) -> BTreeSet<String> {
        self.tparams.iter().cloned().collect()
    }
}

/// 类型文本中的标识符是否都在生成范围内（`_sim_infer_type_args._in_scope`）
fn in_scope(env: &InstrEnv, t: &RsType) -> bool {
    match t {
        RsType::Prim(_) | RsType::Unit | RsType::Object => true,
        RsType::Class { binary, args } => env.ctx.reg().contains(binary) && args.iter().all(|a| in_scope(env, a)),
        RsType::Array(e) => in_scope(env, e),
        RsType::Bare { binary } => env.ctx.reg().contains(binary),
        RsType::Param(n) => env.tparams.contains(n) || hierarchy::short_binary(&env.ctx, n).is_some(),
    }
}

impl SimEnv for InstrEnv<'_> {
    fn short_name(&self, binary: &str) -> String {
        self.ctx.short(binary)
    }

    fn is_subtype(&self, sub: &RsType, sup: &RsType) -> bool {
        hierarchy::is_subtype(&self.ctx, sub, sup)
    }

    fn is_interface(&self, ty: &RsType) -> bool {
        hierarchy::is_interface(&self.ctx, ty)
    }

    fn carrier_type(&self, ty: &RsType) -> Option<RsType> {
        self.ctx.ty.carrier_type_for_ident(ty)
    }

    fn box_object(&self, value: Expr, ty: &RsType) -> SimResult<Expr> {
        coerce::to_object(self, value, ty, false).map_err(|e| SimError::Env(e.to_string()))
    }

    fn infer_type_args(&self, ty: &RsType, declared_sig: &str) -> Option<Vec<RsType>> {
        let actual = hierarchy::type_binary(&self.ctx, &sim::erase(ty))?;
        let solved = self.ctx.ty.infer_type_args_from_declared(&actual, declared_sig, &self.tparams)?;
        // 声明实参引用了生成范围之外的类：该实参按擦除取 Object
        Some(solved.into_iter().map(|a| if in_scope(self, &a) { a } else { RsType::Object }).collect())
    }
}
