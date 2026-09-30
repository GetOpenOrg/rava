//! cfg golden 对照：`scripts/golden/dump_cfg.py` 录下 Python `codegen/cfg/` 各入口的
//! （输入, 输出），这里逐条重放并比对。golden 不在仓库里（`build/golden/cfg/`），
//! 不存在时跳过并打印生成命令。已知差异登记在 `GOLDEN_DIFF.md`。

mod support;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use cfg::{analyze, build_blocks, simplify, structure, StructNode};
use ir::{Expr, Renderer, ShortNames};
use serde_json::{json, Value};
use support::*;

struct NoNames;

impl ShortNames for NoNames {
    fn short_cls(&self, binary: &str) -> String {
        format!("<unregistered {binary}>")
    }
}

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../build/golden/cfg")
}

fn replay(kind: &str, input: &Value) -> R<Value> {
    match kind {
        "blocks" => {
            let insns = arr(&input["instrs"])?.iter().map(insn).collect::<R<Vec<_>>>()?;
            let exc = arr(&input["exc"])?.iter().map(exc).collect::<R<Vec<_>>>()?;
            let bounds = arr(&input["boundaries"])?.iter().map(u).collect::<R<Vec<_>>>()?;
            Ok(match build_blocks(&insns, &exc, &bounds) {
                Ok(blocks) => Value::Array(blocks.iter().map(block_json).collect()),
                Err(e) => json!({"error": e.0}),
            })
        }
        "analyze" => Ok(flow_json(&analyze(u(&input["entry"])?, &succs(&input["succs"])?))),
        "structure" => {
            let (mut nodes, side) = nodes(&input["nodes"])?;
            let flow = analyze(u(&input["entry"])?, &succs(&input["succs"])?);
            match structure(&mut nodes, &flow) {
                Ok(tree) => {
                    let pre = tree_json(&tree, &side);
                    let ctx_after: Vec<Value> =
                        nodes.iter().map(|(k, n)| json!([k, n.ctx().iter().collect::<Vec<_>>()])).collect();
                    let post = tree_json(&simplify(tree), &side);
                    Ok(json!({"tree": pre, "ctx_after": ctx_after, "simplified": post}))
                }
                Err(e) => Ok(json!({"error": e.0})),
            }
        }
        "cmp_op" | "neg_cmp_op" => {
            let op = opcode_by_name(input["op"].as_str().ok_or("op")?)?;
            let a = Expr::raw(input["a"].as_str().ok_or("a")?.to_string());
            let b = Expr::raw(input["b"].as_str().unwrap_or("").to_string());
            let e = if kind == "cmp_op" { cfg::cmp_op(op, a, b) } else { cfg::neg_cmp_op(op, a, b) };
            Ok(json!(Renderer::new(&NoNames).expr(&e.map_err(|e| e.0)?)))
        }
        k => Err(format!("未知记录 {k}")),
    }
}

/// 错误记录只比对「是否出错」（消息文本随实现措辞不同）。
fn same(py: &Value, rs: &Value) -> bool {
    match (py.get("error"), rs.get("error")) {
        (Some(_), Some(_)) => true,
        _ => py == rs,
    }
}

/// 单操作数比较的结构化渲染差异：Python `(a==0)`，Rust `(a == 0)`（GOLDEN_DIFF §1）。
fn normalize_cmp(text: &str) -> String {
    let mut s = text.to_string();
    for op in ["==", "!=", "<=", ">=", "<", ">"] {
        s = s.replace(&format!("{op}0)"), &format!(" {op} 0)"));
    }
    s
}

fn check(test: &str) {
    let path = golden_dir().join(format!("{test}.jsonl"));
    let Ok(text) = std::fs::read_to_string(&path) else {
        println!(
            "[golden] 跳过 {test}：无 {}。生成：python3 scripts/golden/dump_cfg.py <{test}.java> --jdk 21 --clean",
            path.display()
        );
        return;
    };
    let mut by_kind: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    let mut shown = 0;
    let mut hard = 0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let rec: Value = serde_json::from_str(line).expect("jsonl 行");
        let kind = rec["kind"].as_str().unwrap_or("?").to_string();
        let slot = by_kind.entry(kind.clone()).or_default();
        slot.0 += 1;
        let rs = match replay(&kind, &rec["input"]) {
            Ok(v) => v,
            Err(e) => json!({"replay_error": e}),
        };
        if same(&rec["out"], &rs) {
            slot.1 += 1;
            continue;
        }
        // 已登记差异：单操作数比较的空格
        let (Some(p), Some(r)) = (rec["out"].as_str(), rs.as_str()) else {
            hard += 1;
            if shown < 10 {
                shown += 1;
                println!("  ≠ [{kind}] {}\n    py: {}\n    rs: {}", rec["input"], rec["out"], rs);
            }
            continue;
        };
        if normalize_cmp(p) == r {
            slot.2 += 1;
        } else {
            hard += 1;
            if shown < 10 {
                shown += 1;
                println!("  ≠ [{kind}] {}\n    py: {p}\n    rs: {r}", rec["input"]);
            }
        }
    }
    for (k, (n, eq, known)) in &by_kind {
        println!("[golden] {test} {k}: 全等 {eq}/{n}，已登记差异 {known}");
    }
    assert_eq!(hard, 0, "{test}: {hard} 条未登记的不一致");
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
