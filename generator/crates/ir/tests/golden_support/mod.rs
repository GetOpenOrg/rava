//! golden 对照支撑：记录读取、短名表、Python IR → Rust IR 转换、逐条比对与统计。

pub mod convert;
pub mod parse;

use convert::{walk_exprs, Conv, Fallbacks};
use ir::{Renderer, ShortNames};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

/// dump_ir.py 落盘的短名表（binary 名 → 短名）。
pub struct NameTable(pub HashMap<String, String>);

impl ShortNames for NameTable {
    fn short_cls(&self, binary: &str) -> String {
        // 表外的名字不应出现（dump 时已登记全部查询）；出现即暴露为不等
        self.0.get(binary).cloned().unwrap_or_else(|| format!("<unregistered {binary}>"))
    }
}

pub fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../build/golden/ir")
}

/// 一条不等记录（输入 JSON 摘要、Python 输出、Rust 输出）。
pub struct Mismatch {
    pub kind: String,
    pub input: String,
    pub python: String,
    pub rust: String,
}

#[derive(Default)]
pub struct Report {
    pub total: usize,
    pub equal: usize,
    pub by_kind: BTreeMap<String, (usize, usize)>,
    pub mismatches: Vec<Mismatch>,
    pub errors: Vec<String>,
    pub fallbacks: Fallbacks,
    /// 结构化 is_atomic 与渲染文本扫描不一致的表达式
    pub atomic_disagree: Vec<String>,
}

/// Python `is_atomic_rs` 原样（渲染文本扫描），用作结构化判定的对照。
fn scan_atomic(text: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in text.trim().chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if depth == 0 && !(c.is_alphanumeric() || matches!(c, '_' | '.' | ':' | '?')) => return false,
            _ => {}
        }
    }
    true
}

fn check_atomic(rd: &Renderer, e: &ir::Expr, rep: &mut Report) {
    let mut all = Vec::new();
    walk_exprs(e, &mut all);
    for x in all {
        let text = rd.expr(x);
        if rd.is_atomic(x) != scan_atomic(&text) {
            rep.atomic_disagree.push(text);
        }
    }
}

/// 转换 + 渲染一条记录；返回 Rust 输出。
fn render_record(conv: &mut Conv, rec: &Value, rep: &mut Report) -> Result<String, String> {
    let kind = rec["kind"].as_str().unwrap_or("");
    let input = &rec["input"];
    let args = &rec["args"];
    let rd = conv.rd;
    Ok(match kind {
        "render_type" => rd.ty(&conv.ty(input)?),
        "render_expr" => {
            let e = conv.expr(input)?;
            check_atomic(&rd, &e, rep);
            rd.expr(&e)
        }
        "render_cast" => rd.expr(&ir::Expr::CheckCast(conv.cast(input)?)),
        "render_stmt" => {
            let indent = args.get("indent").and_then(Value::as_u64).unwrap_or(0) as usize;
            rd.stmt(&conv.stmt(input)?, indent)
        }
        "upcast_expr" => {
            let wrap = args.get("wrap").and_then(Value::as_str).unwrap_or("none");
            let e = conv.upcast(input.as_str().ok_or("upcast_expr 输入非字符串")?, wrap)?;
            check_atomic(&rd, &e, rep);
            rd.expr(&e)
        }
        "is_atomic_rs" => {
            let e = conv.expr_text("is_atomic_rs(字符串)", input.as_str().ok_or("is_atomic_rs 输入非字符串")?);
            rd.is_atomic(&e).to_string()
        }
        k => return Err(format!("未支持的记录类别 {k}")),
    })
}

/// 跑一个测试的 golden；返回 None 表示 golden 不存在。
pub fn run(test: &str) -> Option<Report> {
    let dir = golden_dir();
    let jsonl = std::fs::read_to_string(dir.join(format!("{test}.jsonl"))).ok()?;
    let names: HashMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string(dir.join(format!("{test}.short_names.json"))).ok()?).ok()?;
    let table = NameTable(names);
    let mut conv = Conv { rd: Renderer::new(&table), fb: Fallbacks::default() };
    let mut rep = Report::default();
    for line in jsonl.lines().filter(|l| !l.trim().is_empty()) {
        let rec: Value = serde_json::from_str(line).expect("jsonl 行解析失败");
        let kind = rec["kind"].as_str().unwrap_or("").to_string();
        let python = if rec["out"].is_boolean() {
            rec["out"].as_bool().unwrap().to_string()
        } else {
            rec["out"].as_str().unwrap_or("").to_string()
        };
        rep.total += 1;
        let slot = rep.by_kind.entry(kind.clone()).or_default();
        slot.0 += 1;
        match render_record(&mut conv, &rec, &mut rep) {
            Ok(rust) if rust == python => {
                rep.equal += 1;
                rep.by_kind.get_mut(&kind).unwrap().1 += 1;
            }
            Ok(rust) => rep.mismatches.push(Mismatch { kind, input: rec["input"].to_string(), python, rust }),
            Err(e) => rep.errors.push(format!("[{kind}] {e}")),
        }
    }
    rep.fallbacks = std::mem::take(&mut conv.fb);
    Some(rep)
}
