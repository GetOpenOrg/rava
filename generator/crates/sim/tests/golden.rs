//! sim golden 对照：`scripts/golden/dump_sim.py` 录下 Python `StackSim` 公开入口的
//! （配置, 前状态, 实参, 钩子日志）→（返回值, 新增语句, 后状态），这里以前状态重建
//! [`StackSim`] 回放同一调用并逐项比对。golden 不在仓库里（`build/golden/sim/`），
//! 不存在时跳过并打印生成命令。已知差异登记在 `GOLDEN_DIFF.md`。

#[path = "../../ir/tests/golden_support/mod.rs"]
#[allow(dead_code)]
mod ir_golden;
mod support;

use ir::Renderer;
use ir_golden::convert::{Conv, Fallbacks};
use serde_json::Value;
use sim::{exprs::is_trivial, type_text, StackSim};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use support::*;

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../build/golden/sim")
}

/// 一条记录的比对结果：(Python 行, Rust 行)
fn replay(names: &Names, rec: &Value) -> R<(Vec<String>, Vec<String>)> {
    let env = ReplayEnv::new(names, &rec["hooks"]);
    let mut conv = Conv { rd: Renderer::new(names), fb: Fallbacks::default() };
    let kind = rec["kind"].as_str().unwrap_or("");
    let args = &rec["args"];
    let mut py = Vec::new();
    let mut rs = Vec::new();
    let mut sim = if kind == "init" {
        let params = args["params"]
            .as_array()
            .ok_or("init 缺 params")?
            .iter()
            .map(|p| ty_of(&env, p).and_then(|t| t.ok_or_else(|| "形参缺类型".to_string())))
            .collect::<R<Vec<_>>>()?;
        let mut cfg = config(&env, &rec["cfg"], &params)?;
        // cfg 里的类型形参按名排序（集合语义）；init 实参保留声明序（`this` 的实参序）
        cfg.class_type_params =
            args["class_type_params"].as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        StackSim::new(cfg, &env).map_err(|e| e.to_string())?
    } else {
        let cfg = config(&env, &rec["cfg"], &[])?;
        let st = state(&env, &mut conv, &rec["cfg"], &rec["pre"])?;
        StackSim::from_state(cfg, st, &env)
    };
    let rd = Renderer::new(names);
    let pair = |e: &ir::Expr, t: &ty::RsType| format!("ret {} : {}", rd.expr(e), type_text(t, &env));
    let py_pair = |r: &Value| format!("ret {} : {}", r[0]["text"].as_str().unwrap_or(""), r[1]["text"].as_str().unwrap_or(""));
    match kind {
        "init" => {
            let ps: Vec<u16> = sim.state.param_slots.iter().copied().collect();
            rs.push(format!("param_slots {ps:?}"));
            let py_ps: Vec<u64> = rec["cfg"]["param_slots"].as_array().map(|a| a.iter().filter_map(Value::as_u64).collect()).unwrap_or_default();
            py.push(format!("param_slots {py_ps:?}"));
        }
        "pop" | "pop_for_store" => {
            let e = if kind == "pop" { sim.pop() } else { sim.pop_for_store() }.map_err(|e| e.to_string())?;
            rs.push(pair(&e.expr, &e.ty));
            py.push(py_pair(&rec["ret"]));
        }
        "load_local" => {
            let (e, t) = sim.load_local(args["slot"].as_u64().ok_or("slot")? as u16).map_err(|e| e.to_string())?;
            rs.push(pair(&e, &t));
            py.push(py_pair(&rec["ret"]));
        }
        "store_local" => {
            let v = entry(&env, &mut conv, &args["expr"], &args["ty"], &args["id"])?;
            sim.store_local(args["slot"].as_u64().ok_or("slot")? as u16, v).map_err(|e| e.to_string())?;
        }
        "fresh_let" => {
            let value = expr_of(&mut conv, &args["value"])?;
            let t = ty_of(&env, &args["ty"])?.ok_or("fresh_let 缺类型")?;
            let e = sim.fresh_let(args["prefix"].as_str().unwrap_or(""), value, &t).map_err(|e| e.to_string())?;
            rs.push(format!("ret {}", rd.expr(&e)));
            py.push(format!("ret {}", rec["ret"]["text"].as_str().unwrap_or("")));
        }
        k => return Err(format!("未知记录 {k}")),
    }
    for s in rec["stmts"].as_array().cloned().unwrap_or_default() {
        py.push(stmt_line_py(&s));
    }
    for s in &sim.state.stmts {
        rs.push(stmt_line_rs(s, &env));
    }
    let trivial: Vec<bool> = sim.state.stack.iter().map(|e| is_trivial(&e.expr)).collect();
    py.extend(snapshot_py(&rec["post"], &trivial));
    rs.extend(snapshot_rs(&sim.state, &env));
    for m in env.misses.borrow().iter() {
        rs.push(format!("钩子未命中 {m}"));
    }
    Ok((py, rs))
}

#[derive(Default)]
struct Tally {
    records: usize,
    equal: usize,
    calls: u64,
    equal_calls: u64,
}

fn check(test: &str) {
    let dir = golden_dir();
    let path = dir.join(format!("{test}.jsonl"));
    let (Ok(text), Ok(names)) =
        (std::fs::read_to_string(&path), std::fs::read_to_string(dir.join(format!("{test}.names.json"))))
    else {
        println!(
            "[golden] 跳过 {test}：无 {}。生成：python3 scripts/golden/dump_sim.py <{test}.java> --jdk 21 --clean",
            path.display()
        );
        return;
    };
    let names = Names::load(&serde_json::from_str(&names).expect("names.json")).expect("names.json 格式");
    let mut by_kind: BTreeMap<String, Tally> = BTreeMap::new();
    let (mut shown, mut bad) = (0, 0);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let rec: Value = serde_json::from_str(line).expect("jsonl 行");
        let kind = rec["kind"].as_str().unwrap_or("?").to_string();
        let count = rec["count"].as_u64().unwrap_or(1);
        let t = by_kind.entry(kind.clone()).or_default();
        t.records += 1;
        t.calls += count;
        let (py, rs) = match replay(&names, &rec) {
            Ok(x) => x,
            Err(e) => (vec![], vec![format!("回放失败：{e}")]),
        };
        if py == rs {
            t.equal += 1;
            t.equal_calls += count;
            continue;
        }
        bad += 1;
        if shown < 15 {
            shown += 1;
            println!("  ≠ [{kind}] args={}", truncate(&rec["args"].to_string(), 400));
            let n = py.len().max(rs.len());
            for i in 0..n {
                let (p, r) = (py.get(i).map_or("", String::as_str), rs.get(i).map_or("", String::as_str));
                if p != r {
                    println!("    py: {p}\n    rs: {r}");
                }
            }
        }
    }
    for (k, t) in &by_kind {
        println!("[golden] {test} {k}: 记录全等 {}/{}，调用全等 {}/{}", t.equal, t.records, t.equal_calls, t.calls);
    }
    assert_eq!(bad, 0, "{test}: {bad} 条记录不一致");
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let mut end = n;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &s[..end])
}

fn big_stack(f: fn()) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).expect("线程").join().expect("golden 失败");
}

#[test]
fn golden_hash_map_ops() {
    big_stack(|| check("TestHashMapOps"));
}

#[test]
fn golden_stream_basic() {
    big_stack(|| check("TestStreamBasic"));
}
