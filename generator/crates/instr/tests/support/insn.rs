//! 记录里的 Python 指令视图 → [`NInsn`]。
//!
//! 优先按（类, 方法, 描述符, 偏移）取类文件里已解码的同一条指令；Python 规范化改写过的
//! 位置（getstatic 常量替换为装载指令、合成 pop / goto）与折叠点的装载指令按视图合成：
//! `fold_const` → [`NInsn::FoldField`]，带 `fold` 的调用 → [`NInsn::FoldCall`]。

use std::collections::HashMap;
use std::sync::OnceLock;

use classfile::insn::opcode_name;
use classfile::{Const, Insn, Operand};
use input::NInsn;
use serde_json::Value;

use crate::sim_support::R;
use crate::support::env::Env;

/// 记录的分类键：折叠点单列（`fold_const` / `fold`），其余为 Python 操作码名
pub fn op_key(rec: &Value) -> String {
    let ins = &rec["ins"];
    if !ins["fold"].is_null() {
        return "fold".to_string();
    }
    ins["op"].as_str().unwrap_or("?").to_string()
}

pub fn resolve(env: &Env, rec: &Value) -> R<NInsn> {
    let ins = &rec["ins"];
    let op = ins["op"].as_str().ok_or("ins 缺 op")?;
    let off = ins["off"].as_u64().ok_or("ins 缺 off")? as u32;
    if op == "fold_const" {
        // comment = "装载操作码\t操作数\t注释"（closure_folds.decode_fold_const）
        let c = ins["comment"].as_str().unwrap_or("");
        let mut parts = c.splitn(3, '\t');
        let (lop, operand, comment) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        let load = synth(lop, off, non_empty(operand), non_empty(comment))?;
        return Ok(NInsn::FoldField { offset: off, load });
    }
    let main = match real_insn(env, rec, off, op) {
        Some(r) => r,
        None => synth(op, off, ins["operand"].as_str(), ins["comment"].as_str())?,
    };
    let fold = &ins["fold"];
    if fold.is_null() {
        return Ok(NInsn::Op(main));
    }
    let lop = fold["op"].as_str().ok_or("fold 缺 op")?;
    let load = synth(lop, off, fold["operand"].as_str(), fold["comment"].as_str())?;
    Ok(NInsn::FoldCall { call: main, load })
}

fn non_empty(s: &str) -> Option<&str> {
    (!s.is_empty()).then_some(s)
}

/// 类文件里的同一条指令：沿记录类 → 超类 / 接口广度优先找同名同描述符、且该偏移上操作码
/// 与视图一致的方法体。记录类是发射所在类，而继承展开会把祖先 / 接口 default 方法体
/// （含 `super.m()` 展开的接口 default 体）发射进子类，此时记录类自身的同名方法不是出处
fn real_insn(env: &Env, rec: &Value, off: u32, op: &str) -> Option<Insn> {
    let (cls, m, d) = (rec["cls"].as_str()?, rec["m"].as_str()?, rec["d"].as_str()?);
    let mut queue = std::collections::VecDeque::from([cls.to_string()]);
    let mut seen = std::collections::BTreeSet::new();
    while let Some(c) = queue.pop_front() {
        if !seen.insert(c.clone()) {
            continue;
        }
        let Some(ci) = env.reg.get(&c) else { continue };
        let code = ci.class_file().methods.iter().find(|x| x.name == m && x.desc == d).and_then(|x| x.code.as_ref());
        if let Some(i) = code.and_then(|code| code.insns.iter().find(|i| i.offset == off && i.name() == op)) {
            return Some(i.clone());
        }
        if !ci.super_class().is_empty() {
            queue.push_back(ci.super_class().to_string());
        }
        queue.extend(ci.interfaces().iter().cloned());
    }
    None
}

fn opcode_of(name: &str) -> Option<u8> {
    static TABLE: OnceLock<HashMap<&'static str, u8>> = OnceLock::new();
    let t = TABLE.get_or_init(|| {
        let mut m = HashMap::new();
        for op in 0..=255u8 {
            m.entry(opcode_name(op)).or_insert(op);
        }
        m.remove("unknown");
        m
    });
    t.get(name).copied()
}

/// 按 Python 视图合成指令（只覆盖规范化 / 折叠会产生的形态）
fn synth(op: &str, off: u32, operand: Option<&str>, comment: Option<&str>) -> R<Insn> {
    let opcode = opcode_of(op).ok_or_else(|| format!("未知操作码 {op}"))?;
    let num = |s: Option<&str>| s.and_then(|x| x.trim().parse::<i64>().ok()).ok_or_else(|| format!("{op} 操作数不是整数：{s:?}"));
    let operand = match op {
        "ldc" | "ldc_w" | "ldc2_w" => Operand::Ldc(ldc_const(comment.unwrap_or(""))?),
        "bipush" | "sipush" => Operand::Int(num(operand)? as i32),
        "iload" | "lload" | "fload" | "dload" | "aload" | "istore" | "lstore" | "fstore" | "dstore" | "astore" | "ret" => {
            Operand::Local(num(operand)? as u16)
        }
        "iinc" => {
            let mut it = operand.unwrap_or("").split_whitespace();
            let index = num(it.next())? as u16;
            let delta = num(it.next())? as i16;
            Operand::Iinc { index, delta }
        }
        "goto" | "goto_w" => Operand::Branch(num(operand)? as u32),
        _ if operand.is_none() => Operand::None,
        _ => return Err(format!("无法按视图合成 {op} {operand:?} {comment:?}")),
    };
    Ok(Insn { offset: off, opcode, operand })
}

/// javap 风格常量注释 → 常量（`int 1` / `long 5` / `float 0.75` / `double -0.0` / `String s` / `class a/B`）
fn ldc_const(comment: &str) -> R<Const> {
    let (kind, text) = comment.split_once(' ').unwrap_or((comment, ""));
    let bad = || format!("ldc 注释无法解析：{comment:?}");
    Ok(match kind {
        "int" => Const::Int(text.parse().map_err(|_| bad())?),
        "long" => Const::Long(text.trim_end_matches(['l', 'L']).parse().map_err(|_| bad())?),
        "float" => Const::Float((parse_f64(text).ok_or_else(bad)? as f32).to_bits()),
        "double" => Const::Double(parse_f64(text).ok_or_else(bad)?.to_bits()),
        "String" => Const::String(text.to_string()),
        "class" => Const::Class(text.to_string()),
        _ => return Err(bad()),
    })
}

fn parse_f64(s: &str) -> Option<f64> {
    let t = s.trim().trim_end_matches(['f', 'F', 'd', 'D']);
    match t {
        "NaN" | "nan" => Some(f64::NAN),
        "Infinity" | "inf" => Some(f64::INFINITY),
        "-Infinity" | "-inf" => Some(f64::NEG_INFINITY),
        _ => t.parse().ok(),
    }
}
