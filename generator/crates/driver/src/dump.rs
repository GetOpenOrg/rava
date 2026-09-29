//! `rava dump-classes`：类解析结果的 golden 归一形态（每类一行 JSON）。
//! 归一规则与 scripts/classfile_golden.py 一一对应——两侧输出逐行相等即解析一致。

use std::io::Write;

use classfile::archive::Archive;
use classfile::{ClassFile, Const, Insn, Operand};
use serde_json::{json, Value};

use crate::Args;

const NEWARRAY_TYPES: [&str; 8] = ["boolean", "char", "float", "double", "byte", "short", "int", "long"];

fn ldc(c: &Const) -> String {
    match c {
        Const::String(s) => format!("S:{s}"),
        Const::Int(v) => format!("I:{v}"),
        Const::Long(v) => format!("J:{v}"),
        Const::Class(n) => format!("C:{n}"),
        Const::Float(_) => "F".into(),
        Const::Double(_) => "D".into(),
        _ => "O".into(),
    }
}

fn insn(i: &Insn) -> String {
    let operand = match &i.operand {
        Operand::None => String::new(),
        Operand::Int(v) => v.to_string(),
        Operand::Local(v) => v.to_string(),
        Operand::Iinc { index, delta } => format!("{index} {delta}"),
        Operand::Branch(t) => t.to_string(),
        Operand::TableSwitch { default, low, high, targets } => format!(
            "default:{default} low:{low} high:{high} offs:{}",
            targets.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",")
        ),
        Operand::LookupSwitch { default, pairs } => {
            let mut s = format!("default:{default} ");
            s.push_str(&pairs.iter().map(|(k, v)| format!("{k}:{v}")).collect::<Vec<_>>().join(" "));
            s
        }
        Operand::Field(m) | Operand::Method(m, _) => m.to_string(),
        Operand::InvokeDynamic { name, desc, .. } => format!("{name}:{desc}"),
        Operand::Class(c) => c.clone(),
        Operand::MultiANewArray(c, d) => format!("{c} {d}"),
        Operand::NewArray(t) => {
            NEWARRAY_TYPES.get((*t as usize).wrapping_sub(4)).map_or_else(|| t.to_string(), |s| s.to_string())
        }
        Operand::Ldc(c) => ldc(c),
    };
    format!("{} {} {}", i.offset, i.name(), operand)
}

pub fn class_json(c: &ClassFile) -> Value {
    json!({
        "name": c.name,
        "super": c.super_name.clone().unwrap_or_default(),
        "interfaces": c.interfaces,
        "access": c.access,
        "major": c.major,
        "fields": c.fields.iter().map(|f| json!([f.name, f.desc, f.access])).collect::<Vec<_>>(),
        "methods": c.methods.iter().map(|m| {
            let (insns, exc) = match &m.code {
                Some(code) => (
                    code.insns.iter().map(insn).collect::<Vec<_>>(),
                    code.exception_table.iter()
                        .map(|e| json!([e.start, e.end, e.handler, e.catch_type.clone().unwrap_or_default()]))
                        .collect::<Vec<_>>(),
                ),
                None => (vec![], vec![]),
            };
            json!([m.name, m.desc, m.access, insns, exc])
        }).collect::<Vec<_>>(),
    })
}

pub fn run(args: &Args) -> Result<(), String> {
    let home = crate::java_home(args)?;
    let module = args.opt("--module").unwrap_or_else(|| "java.base".into());
    let prefix = args.opt("--prefix").unwrap_or_default();
    let path = home.join("jmods").join(format!("{module}.jmod"));
    let mut a = Archive::open(&path).map_err(|e| e.to_string())?;
    let out = std::io::stdout();
    let mut out = out.lock();
    let mut failed = 0usize;
    for n in a.class_names() {
        if !n.starts_with(&prefix) {
            continue;
        }
        let bytes = a.read_class(&n).map_err(|e| e.to_string())?.unwrap_or_default();
        match classfile::parse(&bytes) {
            Ok(c) => writeln!(out, "{}", class_json(&c)).map_err(|e| e.to_string())?,
            Err(e) => {
                failed += 1;
                eprintln!("解析失败 {n}: {e}");
            }
        }
    }
    if failed > 0 {
        return Err(format!("{failed} 个类解析失败"));
    }
    Ok(())
}
