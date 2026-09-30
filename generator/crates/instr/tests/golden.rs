//! instr golden 对照：`scripts/golden/dump_instr.py` 在一次正常转译中截获 Python
//! `sim_instr` 每次调用的（指令视图, 配置, 前状态, 被引用的 let）→（新增语句, 后状态,
//! let 回写, 副作用），这里按 meta 行重建真实注册表 / 类型上下文，以前状态回放同一条
//! 指令并逐行比对。分类口径与规范化规则见 crate 根 `GOLDEN_DIFF.md`。
//!
//! - 目录：`INSTR_GOLDEN_DIR`（缺省 `<repo>/build/golden/instr`）；文件缺失时跳过并提示生成命令
//! - 逐行流式读取（CompletableFuture 转储约 1.5GB）
//! - 采样：`INSTR_GOLDEN_SAMPLE=N` 每 N 条记录回放 1 条；缺省 CompletableFuture 取 10、其余 1
//! - `INSTR_GOLDEN_SHOW=N`：已移植指令打印前 N 条不一致（缺省 20），其余指令前 N/2 条
//! - `INSTR_GOLDEN_TRACE=<op>`：打印该指令前 3 条记录的两侧全部行（核对比对口径）
//! - 断言：[`PORTED_OPS`] 内的指令 mismatch = 0；其余只报告

#[path = "../../ir/tests/golden_support/mod.rs"]
#[allow(dead_code)]
mod ir_golden;
#[path = "../../sim/tests/support/mod.rs"]
#[allow(dead_code)]
mod sim_support;
mod support;

use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use serde_json::Value;
use support::env::build_env;
use support::insn::{op_key, resolve};
use support::replay::{replay, Replayed};
use support::tally::{classify, is_ported, Class, Tally, PORTED_OPS};

fn golden_dir() -> PathBuf {
    match std::env::var_os("INSTR_GOLDEN_DIR") {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../build/golden/instr"),
    }
}

fn env_usize(key: &str) -> Option<usize> {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).filter(|&n| n > 0)
}

fn check(test: &str, default_stride: usize) {
    let path = golden_dir().join(format!("{test}.jsonl"));
    let Ok(file) = std::fs::File::open(&path) else {
        eprintln!(
            "[instr-golden] 跳过 {test}：无 {}。生成：python3 scripts/golden/dump_instr.py <{test}.java> --jdk 21 --clean",
            path.display()
        );
        return;
    };
    let stride = env_usize("INSTR_GOLDEN_SAMPLE").unwrap_or(default_stride);
    let show = env_usize("INSTR_GOLDEN_SHOW").unwrap_or(20);
    let trace = std::env::var("INSTR_GOLDEN_TRACE").unwrap_or_default();
    let mut traced = 0;
    let mut rd = BufReader::with_capacity(1 << 22, file);
    let mut line = String::new();
    rd.read_line(&mut line).expect("读取 meta 行");
    let meta: Value = serde_json::from_str(&line).expect("meta 行");
    let env = build_env(&meta).expect("重建环境");
    let mut tally = Tally::default();
    let (mut idx, mut shown_ported, mut shown_other) = (0usize, 0usize, 0usize);
    loop {
        line.clear();
        if rd.read_line(&mut line).expect("读取记录") == 0 {
            break;
        }
        idx += 1;
        if (idx - 1) % stride != 0 || line.trim().is_empty() {
            continue;
        }
        let rec: Value = serde_json::from_str(&line).expect("jsonl 行");
        let op = op_key(&rec);
        let count = rec["count"].as_u64().unwrap_or(1);
        let (class, lines) = match resolve(&env, &rec).and_then(|ni| replay(&env, &rec, &ni)) {
            Ok(Replayed::Unported(s)) => (Class::Unported(s), None),
            Ok(Replayed::Lines { py, rs }) => (classify(&py, &rs), Some((py, rs))),
            Err(e) => (Class::Mismatch, Some((vec![], vec![format!("回放失败：{e}")]))),
        };
        if let (true, Some((py, rs))) = (op == trace && traced < 3, &lines) {
            traced += 1;
            eprintln!("  trace [{op}]\n    py: {}\n    rs: {}", py.join("\n    py: "), rs.join("\n    rs: "));
        }
        if let (Class::Mismatch, Some((py, rs))) = (&class, &lines) {
            let (shown, cap) = if is_ported(&op) { (&mut shown_ported, show) } else { (&mut shown_other, show / 2) };
            if *shown < cap {
                *shown += 1;
                print_diff(&rec, &op, py, rs);
            }
        }
        tally.add(&op, count, &class);
    }
    let sampled = if stride == 1 { format!("全量 {idx} 条") } else { format!("每 {stride} 条取 1，共 {idx} 条") };
    tally.report(test, &sampled);
    assert_eq!(
        tally.ported_mismatch, 0,
        "{test}：已移植指令（{} 种）有 {} 条记录不一致",
        PORTED_OPS.len(),
        tally.ported_mismatch
    );
}

fn print_diff(rec: &Value, op: &str, py: &[String], rs: &[String]) {
    let ins = &rec["ins"];
    eprintln!(
        "  ≠ [{op}] {}.{}{} @{} operand={} comment={}",
        rec["cls"].as_str().unwrap_or(""),
        rec["m"].as_str().unwrap_or(""),
        rec["d"].as_str().unwrap_or(""),
        ins["off"],
        ins["operand"],
        ins["comment"]
    );
    for i in 0..py.len().max(rs.len()) {
        let (p, r) = (py.get(i).map_or("", String::as_str), rs.get(i).map_or("", String::as_str));
        if p != r {
            eprintln!("    py: {p}\n    rs: {r}");
        }
    }
}

fn big_stack(f: fn()) {
    std::thread::Builder::new().stack_size(256 << 20).spawn(f).expect("线程").join().expect("golden 失败");
}

#[test]
fn golden_hash_map_ops() {
    big_stack(|| check("TestHashMapOps", 1));
}

#[test]
fn golden_stream_basic() {
    big_stack(|| check("TestStreamBasic", 1));
}

#[test]
fn golden_completable_future() {
    big_stack(|| check("TestCompletableFuture", 10));
}
