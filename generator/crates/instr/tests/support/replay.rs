//! 单条记录回放：以 pre 状态重建 [`StackSim`]（真实 [`InstrEnv`] 回答层次 / 装箱查询），
//! 语句表补到 Python 的长度（被栈上变量引用的 `let` 按原下标放回，其余为占位），执行
//! [`sim_instr`] 后生成与 Python 同形的逐行快照：新增语句、后状态、`let` 回写、副作用。

use input::NInsn;
use instr::{sim_instr, Effect, InstrCtx, InstrEnv, InstrError, InstrHooks, InstrLog};
use ir::{Ident, Path, PathSegment, Renderer, Stmt};
use serde_json::Value;
use sim::{exprs::is_trivial, StackSim};
use ty::TyCtx;

use crate::ir_golden::convert::{Conv, Fallbacks};
use crate::sim_support::{config, snapshot_py, snapshot_rs, state, stmt_line_py, stmt_line_rs, ReplayEnv, R};
use crate::support::env::Env;

/// 占位语句：Python 语句表中未转储的前序语句（指令翻译只回看被栈引用的 `let`）
const PLACEHOLDER: &str = "/* golden: 未转储的前序语句 */";

/// 回放结果
pub enum Replayed {
    /// Rust 侧走到未移植分支（按分支名计数）
    Unported(String),
    /// 两侧逐行快照（相等 → exact；否则交给版式规则 / 记为不一致）
    Lines { py: Vec<String>, rs: Vec<String> },
}

/// 按 fx 日志里的 `sam_ctor` 查询结果回答 [`InstrHooks`]
struct ReplayHooks {
    sam: Vec<(String, String, Option<String>)>,
}

impl ReplayHooks {
    fn from_rec(rec: &Value) -> ReplayHooks {
        ReplayHooks::from_fx(&rec["fx"])
    }

    fn from_fx(fx: &Value) -> ReplayHooks {
        let sam = fx
            .as_array()
            .into_iter()
            .flatten()
            .filter(|f| f[0] == "sam_ctor")
            .map(|f| {
                let a = |i: usize| f[1][i].as_str().unwrap_or("").to_string();
                (a(0), a(1), f[2].as_str().map(str::to_string))
            })
            .collect();
        ReplayHooks { sam }
    }
}

impl InstrHooks for ReplayHooks {
    fn sam_ctor_path(&self, iface: &str, current_class: &str) -> Option<Path> {
        let (_, _, out) = self.sam.iter().find(|(i, c, _)| i == iface && c == current_class)?;
        let segs = out.as_deref()?.split("::").map(|s| Ident::new(s).ok().map(PathSegment::new)).collect::<Option<Vec<_>>>()?;
        Some(Path::new(segs))
    }
}

pub fn replay(env: &Env, rec: &Value, (ninsn, owner): &(NInsn, Option<String>)) -> R<Replayed> {
    let renv = ReplayEnv::new(&env.rnames, &Value::Null);
    let mut conv = Conv { rd: Renderer::new(&env.rnames), fb: Fallbacks::default() };
    let cfg = config(&renv, &rec["cfg"], &[])?;
    let mut st = state(&renv, &mut conv, &rec["cfg"], &rec["pre"])?;
    let n_pre = rec["pre"]["n_stmts"].as_u64().ok_or("pre 缺 n_stmts")? as usize;
    st.stmts = vec![Stmt::raw(PLACEHOLDER.to_string()); n_pre];
    for l in rec["lets"].as_array().into_iter().flatten() {
        let i = l[0].as_u64().ok_or("lets 下标")? as usize;
        let s = conv.stmt(&l[1]["j"])?;
        *st.stmts.get_mut(i).ok_or("lets 下标越界")? = s;
    }
    let hooks = ReplayHooks::from_rec(rec);
    let cls = rec["cls"].as_str().unwrap_or("");
    let ctx = InstrCtx::new(TyCtx::new(&env.reg, &env.names, &env.manifest), &env.rt, &env.facts, &hooks, cls)
        .with_code_owner(owner.as_deref().unwrap_or(cls));
    let tparams = cfg.class_type_params.clone();
    let ienv = InstrEnv::new(ctx, &tparams);
    let mut sim = StackSim::from_state(cfg, st, &ienv);
    let mut log = InstrLog::default();
    let res = sim_instr(&ienv, &mut sim, &mut log, ninsn);
    let mut py = Vec::new();
    let mut rs = Vec::new();
    match res {
        Err(InstrError::OutOfScope(s)) => return Ok(Replayed::Unported(s)),
        Err(e) => rs.push(format!("err {e}")),
        Ok(()) => {}
    }
    if let Some(e) = rec["err"].as_str() {
        py.push(format!("err {e}"));
    }
    for s in rec["stmts"].as_array().into_iter().flatten() {
        py.push(stmt_line_py(s));
    }
    for s in sim.state.stmts.iter().skip(n_pre) {
        rs.push(stmt_line_rs(s, &renv));
    }
    let trivial: Vec<bool> = sim.state.stack.iter().map(|e| is_trivial(&e.expr)).collect();
    py.extend(snapshot_py(&rec["post"], &trivial));
    rs.extend(snapshot_rs(&sim.state, &renv));
    for l in rec["lets_post"].as_array().into_iter().flatten() {
        let i = l[0].as_u64().unwrap_or(0) as usize;
        py.push(format!("let[{i}] {}", stmt_line_py(&l[1])));
        let r = sim.state.stmts.get(i).map_or_else(|| "（已移除）".to_string(), |s| stmt_line_rs(s, &renv));
        rs.push(format!("let[{i}] {r}"));
    }
    py.extend(fx_lines_py(&rec["fx"]));
    rs.extend(log.effects.iter().map(fx_line_rs));
    Ok(Replayed::Lines { py, rs })
}

/// Python 副作用 → 行；`sam_ctor` 是查询（由 [`ReplayHooks`] 回答），不参与比对
fn fx_lines_py(fx: &Value) -> Vec<String> {
    let mut out = Vec::new();
    for f in fx.as_array().into_iter().flatten() {
        let name = f[0].as_str().unwrap_or("?");
        let args: Vec<String> = f[1].as_array().into_iter().flatten().map(|a| a.as_str().map_or_else(|| a.to_string(), str::to_string)).collect();
        match name {
            "sam_ctor" => {}
            "audit" => {
                // equiv_audit.record(equiv_id, count=1)：Rust 侧每次计数一条
                let n = args.get(1).and_then(|c| c.parse::<usize>().ok()).unwrap_or(1);
                out.extend(std::iter::repeat_n(format!("fx audit {}", args.first().map_or("", String::as_str)), n));
            }
            _ => out.push(format!("fx {name} {}", args.join(" | "))),
        }
    }
    out
}

fn fx_line_rs(e: &Effect) -> String {
    match e {
        Effect::Audit(a) => format!("fx audit {}", a.as_str()),
        Effect::InstanceofFold => "fx instanceof_fold ".to_string(),
        Effect::Inherited { receiver, method, param_desc } => format!("fx inherited {receiver} | {method} | {param_desc}"),
        Effect::LambdaRef { class, method, rust_name } => format!("fx lambda_ref {class} | {method} | {rust_name}"),
        Effect::SamSite { iface, sam_desc, class } => format!("fx sam_site {iface} | {sam_desc} | {class}"),
    }
}
