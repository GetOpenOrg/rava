//! cfg golden 支撑：Python 转储 JSON ↔ Rust 输入 / 输出的转换。

use std::collections::{BTreeMap, BTreeSet};

use cfg::{Block, BreakLabel, Cond, FlowAnalysis, FlowNode, Item, NodeId, Terminal, Terminator};
use classfile::{ExceptionEntry, Insn, Operand};
use ir::{Expr, Raw};
use serde_json::{json, Value};

pub type R<T> = Result<T, String>;

pub fn u(v: &Value) -> R<u32> {
    v.as_u64().map(|x| x as u32).ok_or_else(|| format!("非整数 {v}"))
}

pub fn i(v: &Value) -> R<i32> {
    v.as_i64().map(|x| x as i32).ok_or_else(|| format!("非整数 {v}"))
}

pub fn arr(v: &Value) -> R<&Vec<Value>> {
    v.as_array().ok_or_else(|| format!("非数组 {v}"))
}

fn raw(v: &Value) -> R<Expr> {
    Ok(Expr::Raw(Raw(v.as_str().ok_or_else(|| format!("非文本 {v}"))?.to_string())))
}

pub fn opcode_by_name(name: &str) -> R<u8> {
    (0..=255u8)
        .find(|op| classfile::insn::opcode_name(*op) == name)
        .ok_or_else(|| format!("未知操作码 {name}"))
}

/// Python `parse_switch_operand` 的文本格式 → 结构化操作数（仅测试用）。
fn switch_operand(text: &str) -> R<Operand> {
    let (mut default, mut low, mut high, mut offs, mut pairs) = (None, None, None, None, Vec::new());
    for part in text.split_whitespace() {
        let Some((k, v)) = part.split_once(':') else { continue };
        let num = |s: &str| s.parse::<i64>().map_err(|e| format!("{e}: {s}"));
        match k {
            "default" => default = Some(num(v)? as u32),
            "low" => low = Some(num(v)? as i32),
            "high" => high = Some(num(v)? as i32),
            "offs" => {
                offs = Some(v.split(',').filter(|x| !x.is_empty()).map(|x| num(x).map(|n| n as u32)).collect::<R<Vec<_>>>()?)
            }
            _ => pairs.push((num(k)? as i32, num(v)? as u32)),
        }
    }
    let default = default.ok_or("switch 缺少 default")?;
    Ok(match offs {
        Some(targets) => {
            let low = low.ok_or("缺少 low")?;
            Operand::TableSwitch { default, low, high: high.unwrap_or(low + targets.len() as i32 - 1), targets }
        }
        None => Operand::LookupSwitch { default, pairs },
    })
}

pub fn insn(v: &Value) -> R<Insn> {
    let a = arr(v)?;
    let name = a[1].as_str().ok_or("操作码名")?;
    // Python 闭包常量折叠的伪指令（替换原 invoke / getfield）：对切块而言是非控制指令（GOLDEN_DIFF §2）
    let opcode = if name == "fold_const" { 0x00 } else { opcode_by_name(name)? };
    let text = a[2].as_str();
    let operand = if cfg::opcodes::is_cond_branch(opcode) || cfg::opcodes::is_goto(opcode) {
        Operand::Branch(text.ok_or("缺操作数")?.trim().parse::<u32>().map_err(|e| e.to_string())?)
    } else if cfg::opcodes::is_switch(opcode) {
        switch_operand(text.unwrap_or(""))?
    } else {
        Operand::None
    };
    Ok(Insn { offset: u(&a[0])?, opcode, operand })
}

pub fn exc(v: &Value) -> R<ExceptionEntry> {
    let a = arr(v)?;
    Ok(ExceptionEntry {
        start: u(&a[0])?,
        end: u(&a[1])?,
        handler: u(&a[2])?,
        catch_type: a[3].as_str().map(str::to_string),
    })
}

pub fn block_json(b: &Block) -> Value {
    let (kind, pc, target, fall, cases, default) = match &b.term {
        Terminator::Exit => ("exit", None, None, None, vec![], None),
        Terminator::Fall { target } => ("fall", None, Some(*target), None, vec![], None),
        Terminator::Goto { pc, target } => ("goto", Some(*pc), Some(*target), None, vec![], None),
        Terminator::Cond { pc, target, fallthrough } => {
            ("cond", Some(*pc), Some(*target), Some(*fallthrough), vec![], None)
        }
        Terminator::Switch { pc, cases, default } => {
            ("switch", Some(*pc), None, None, cases.iter().map(|(v, t)| json!([v, t])).collect(), Some(*default))
        }
    };
    json!({
        "id": b.id, "start_idx": b.start_idx, "end_idx": b.end_idx, "start_pc": b.start_pc,
        "handler": b.is_handler_entry, "kind": kind, "pc": pc, "target": target,
        "fallthrough": fall, "cases": cases, "default": default,
    })
}

pub fn succs(v: &Value) -> R<BTreeMap<NodeId, Vec<NodeId>>> {
    let mut out = BTreeMap::new();
    for e in arr(v)? {
        let e = arr(e)?;
        out.insert(u(&e[0])?, arr(&e[1])?.iter().map(u).collect::<R<Vec<_>>>()?);
    }
    Ok(out)
}

pub fn flow_json(f: &FlowAnalysis) -> Value {
    json!({
        "rpo": f.rpo,
        "idom": f.idom.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
        "back_edges": f.back_edges.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(),
        "loops": f.loops.iter().map(|(h, b)| json!([h, b.iter().collect::<Vec<_>>()])).collect::<Vec<_>>(),
        "reducible": f.reducible,
    })
}

pub fn cond(v: &Value) -> R<Cond> {
    Ok(match v["k"].as_str().ok_or("cond.k")? {
        "atom" => Cond::Atom { pos: raw(&v["pos"])?, neg: raw(&v["neg"])? },
        "const" => Cond::Const(v["v"].as_bool().ok_or("cond.v")?),
        "and" => Cond::And(arr(&v["items"])?.iter().map(cond).collect::<R<_>>()?),
        "or" => Cond::Or(arr(&v["items"])?.iter().map(cond).collect::<R<_>>()?),
        k => return Err(format!("未知 cond {k}")),
    })
}

fn raw_text(e: &Expr) -> Value {
    match e {
        Expr::Raw(r) => json!(r.0),
        other => json!(format!("<non-raw {other:?}>")),
    }
}

pub fn cond_json(c: &Cond) -> Value {
    match c {
        Cond::Atom { pos, neg } => json!({"k": "atom", "pos": raw_text(pos), "neg": raw_text(neg)}),
        Cond::Const(v) => json!({"k": "const", "v": v}),
        Cond::And(items) => json!({"k": "and", "items": items.iter().map(cond_json).collect::<Vec<_>>()}),
        Cond::Or(items) => json!({"k": "or", "items": items.iter().map(cond_json).collect::<Vec<_>>()}),
    }
}

/// 节点的旁路信息：汇合声明行、catch 描述（结构树按编号 / 序号引用）。
#[derive(Default)]
pub struct Side {
    pub decls: BTreeMap<NodeId, Vec<Value>>,
    pub catches: BTreeMap<NodeId, Vec<Value>>,
}

fn opt_u(v: &Value) -> R<u32> {
    u(v)
}

pub fn nodes(v: &Value) -> R<(BTreeMap<NodeId, FlowNode>, Side)> {
    let mut out = BTreeMap::new();
    let mut side = Side::default();
    for n in arr(v)? {
        let id = u(&n["id"])?;
        let term = match n["kind"].as_str().ok_or("kind")? {
            "exit" => Terminal::Exit,
            "goto" => Terminal::Goto { target: opt_u(&n["target"])? },
            "cond" => Terminal::Cond {
                cond: cond(&n["cond"])?,
                target: u(&n["target"])?,
                fallthrough: u(&n["fallthrough"])?,
            },
            "switch" => Terminal::Switch {
                key: raw(&n["key"])?,
                cases: arr(&n["cases"])?
                    .iter()
                    .map(|c| Ok((arr(&c[0])?.iter().map(i).collect::<R<Vec<_>>>()?, u(&c[1])?)))
                    .collect::<R<_>>()?,
                default: u(&n["default"])?,
            },
            "try" => Terminal::Try {
                body: u(&n["target"])?,
                handlers: arr(&n["handlers"])?.iter().map(u).collect::<R<_>>()?,
                group: u(&n["group"])?,
                catch_ends: arr(&n["catch_ends"])?.iter().map(|x| x.as_u64().map(|y| y as u32)).collect(),
            },
            k => return Err(format!("未知节点 kind {k}")),
        };
        let decls = arr(&n["decls"])?.clone();
        let node = FlowNode {
            start_pc: u(&n["start_pc"])?,
            term,
            has_stmts: n["has_stmts"].as_bool().ok_or("has_stmts")?,
            has_decls: !decls.is_empty(),
            ctx: arr(&n["ctx"])?.iter().map(u).collect::<R<BTreeSet<_>>>()?,
            jump_pcs: arr(&n["pcs"])?.iter().map(u).collect::<R<_>>()?,
        };
        side.decls.insert(id, decls);
        side.catches.insert(id, arr(&n["catches"])?.clone());
        out.insert(id, node);
    }
    Ok((out, side))
}

fn label_json(l: &BreakLabel) -> Value {
    match l {
        BreakLabel::Block(n) => json!(n),
        BreakLabel::LoopExit(h) => json!(["loop-exit", h]),
    }
}

pub fn tree_json(seq: &[Item], side: &Side) -> Value {
    let items: Vec<Value> = seq
        .iter()
        .map(|it| match it {
            Item::Code { block, exits, empty } => json!(["code", block, exits, empty]),
            Item::Decl(ids) => {
                let lines: Vec<Value> =
                    ids.iter().flat_map(|n| side.decls.get(n).cloned().unwrap_or_default()).collect();
                json!(["decl", lines])
            }
            Item::Block { label, body } => json!(["block", label, tree_json(body, side)]),
            Item::Loop(l) => json!([
                "loop",
                l.header,
                tree_json(&l.body, side),
                l.exit_label.as_ref().map(label_json),
                l.while_cond.as_ref().map(cond_json),
                l.cond_origin
            ]),
            Item::If(x) => json!([
                "if",
                cond_json(&x.cond),
                tree_json(&x.then, side),
                tree_json(&x.else_, side),
                x.origin
            ]),
            Item::Switch(s) => {
                let arms: Vec<Value> =
                    s.arms.iter().map(|a| json!([a.values, tree_json(&a.body, side)])).collect();
                json!(["switch", raw_text(&s.key), arms, s.origin])
            }
            Item::Try(t) => {
                let descs = side.catches.get(&t.origin).cloned().unwrap_or_default();
                let catches: Vec<Value> = t
                    .catches
                    .iter()
                    .map(|c| json!([descs.get(c.clause).cloned().unwrap_or(Value::Null), tree_json(&c.body, side)]))
                    .collect();
                json!(["try", tree_json(&t.body, side), catches, t.origin])
            }
            Item::Break(l) => json!(["break", label_json(l)]),
            Item::Continue(h) => json!(["continue", h]),
        })
        .collect();
    Value::Array(items)
}
