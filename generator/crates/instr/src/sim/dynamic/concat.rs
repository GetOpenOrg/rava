//! 字符串拼接 invokedynamic（← `invoke._gen_string_concat`，`[indy] concat` 类引导方法）：
//! 结果为 java.lang.String，经 `String::from_owned(format!(..))` 构造。
//!
//! 拼接配方（`makeConcatWithConstants`）：`\u{1}` = 动态实参位，`\u{2}` = 常量位（值为配方
//! 之后的静态实参，按序）。配方与常量值取自引导方法静态实参（`classfile` 的
//! `template` / `template_consts` 口径）。

use classfile::Const;
use ir::{Expr, Lit};
use sim::StackSim;
use ty::RsType;

use super::boxing::obj_text;
use super::{numeric_const_text, raw_stmt, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;

/// 拼接配方：首个静态实参为 String 时取之；含常量位时携带常量值
/// （String → 原文，数值 → Python `str`；出现其它形态 → 不携带）
fn recipe(site: &IndySite) -> Option<(String, Vec<String>)> {
    let args = &site.bsm?.args;
    let Some(Const::String(template)) = args.first() else {
        return None;
    };
    let mut consts = Vec::new();
    if template.contains('\u{2}') {
        for c in &args[1..] {
            match c {
                Const::String(s) => consts.push(s.clone()),
                other => match numeric_const_text(other) {
                    Some(t) => consts.push(t),
                    None => {
                        consts.clear();
                        break;
                    }
                },
            }
        }
    }
    Some((template.clone(), consts))
}

/// 配方文本段：按**原文**的花括号切段，各段按 Rust 字面量转义，再以双写花括号拼回
/// （不可对转义结果整体双写——控制字符转义自带花括号）
fn tmpl_seg(p: &str) -> String {
    p.split('}')
        .map(|part| part.split('{').map(ir::render::escape_str).collect::<Vec<_>>().join("{{"))
        .collect::<Vec<_>>()
        .join("}}")
}

/// 带常量位的配方段：每个 `\u{2}` 按序替换为对应常量的已转义段（常量值内的 `\u{1}` 是
/// 普通字符，不参与实参位切分）；`next` 是跨段共享的常量游标
fn tmpl_seg_consts(p: &str, consts: &[String], next: &mut usize) -> String {
    let mut chunks = p.split('\u{2}');
    let mut out = tmpl_seg(chunks.next().unwrap_or(""));
    for ch in chunks {
        let c = consts.get(*next).map_or("", String::as_str);
        *next += 1;
        out.push_str(&tmpl_seg(c));
        out.push_str(&tmpl_seg(ch));
    }
    out
}

/// 一个拼接实参按 Java 字符串化语义渲染（`p` 为形参描述符）；引用实参的 toString 物化为临时变量
fn concat_arg(env: &InstrEnv, sim: &mut StackSim, e: sim::StackEntry, p: &str) -> InstrResult<String> {
    let raw_s = text(env, &e.expr);
    let string_desc = format!("L{};", ty::consts::STRING);
    Ok(match p {
        // Java 浮点数格式化：整数值需显示 .0（如 5.0 而非 5）
        "D" => format!("java_fmt_f64({raw_s})"),
        "F" => format!("java_fmt_f32({raw_s})"),
        // Java char (u16) 必须转为 Rust char 才能以字符形式格式化
        "C" => format!("char::from_u32({raw_s} as u32).unwrap_or('?')"),
        // Java 布尔拼接（JLS §5.1.11）呈现 true/false：布尔短路表达式经分支合并以 i32（1/0）
        // 流动，栈类型仍为 bool 时保持原样
        "Z" if ty_text(env, &e.ty) != "bool" => format!("({raw_s} != 0)"),
        _ if (p.starts_with('L') || p.starts_with('[')) && p != string_desc => {
            // 引用类型实参：Java 语义是 String.valueOf(x)（虚 toString 分派）。预物化为临时变量
            // （toString 返回 Result，format! 内不能传播 ?）；Object::toString 经 vtable 桥接，
            // null 给出 "null"。String 自身走 Display 快速路径不变
            let boxed = obj_text(env, &raw_s, &e.ty);
            let sv = sim.fresh("_t")?;
            sim.emit(raw_stmt(format!("let {}: {} = {boxed}.toString()?;", sv.as_str(), ir::anchors::STRING)));
            sv.as_str().to_string()
        }
        _ => raw_s,
    })
}

/// 拼接结果值（← Python 以 `Lit` 承载：参与 trivial / opaque-let 判定）
fn concat_value(fmt: Option<String>, args: Vec<String>) -> Expr {
    Expr::Lit(Lit::JStringConcat { fmt, args: args.into_iter().map(|a| Expr::raw(a)).collect() })
}

/// 拼接调用点：弹出动态实参，按配方生成 `String::from_owned(format!(..))`
pub(super) fn string_concat(env: &InstrEnv, sim: &mut StackSim, site: &IndySite) -> InstrResult<()> {
    // Python 以 `^InvokeDynamic [^: ]+:(\([^)]*\))` 取形参段；取不到时按单个 String 实参
    let name_ok = !site.name.is_empty() && !site.name.contains([':', ' ']);
    let params = if name_ok && site.desc.starts_with('(') && site.desc.contains(')') {
        ty::type_map::parse_descriptor_params(site.desc)
    } else {
        vec![format!("L{};", ty::consts::STRING)]
    };
    concat_from_stack(env, sim, &params, recipe(site))
}

/// 按配方拼接栈顶 `params.len()` 个实参（`params` 为其描述符，声明序），压入 String 结果；
/// 配方缺省或实参位数不符时逐个实参直接拼接
pub(super) fn concat_from_stack(
    env: &InstrEnv,
    sim: &mut StackSim,
    params: &[String],
    recipe: Option<(String, Vec<String>)>,
) -> InstrResult<()> {
    let string_t = RsType::class(ty::consts::STRING.to_string(), Vec::new());
    // 先按栈序弹出全部实参，再按声明序字符串化：引用实参的 toString 物化次序与 Java 求值序（从左到右）一致
    let mut entries = Vec::with_capacity(params.len());
    for _ in params {
        entries.push(sim.pop()?);
    }
    entries.reverse();
    let mut args = Vec::with_capacity(params.len());
    for (e, p) in entries.into_iter().zip(params) {
        args.push(concat_arg(env, sim, e, p)?);
    }

    if let Some((template, consts)) = recipe {
        let parts: Vec<&str> = template.split('\u{1}').collect();
        if parts.len() == args.len() + 1 {
            let mut next = 0usize;
            let mut fmt_str = String::new();
            for (i, p) in parts.iter().enumerate() {
                fmt_str.push_str(&if consts.is_empty() { tmpl_seg(p) } else { tmpl_seg_consts(p, &consts, &mut next) });
                if i < args.len() {
                    fmt_str.push_str("{}");
                }
            }
            sim.push(concat_value(Some(fmt_str), args), string_t);
            return Ok(());
        }
    }

    // 无配方 / 配方实参位与实参数不符：逐个实参直接拼接
    let fmt = (!args.is_empty()).then(|| "{}".repeat(args.len()));
    sim.push(concat_value(fmt, args), string_t);
    Ok(())
}
