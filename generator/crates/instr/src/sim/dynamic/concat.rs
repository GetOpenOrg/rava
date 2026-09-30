//! 字符串拼接 invokedynamic（← `invoke._gen_string_concat`，`[indy] concat` 类引导方法）：
//! 结果为 java.lang.String，在 UTF-16 层构造：`String::of("首段") + 片段 + ..`（运行时 `Add`
//! 逐段追加码元）。配方常量、常量位值、char / String 实参中的孤立代理项原样保留。
//!
//! 拼接配方（`makeConcatWithConstants`）：`\u{1}` = 动态实参位，`\u{2}` = 常量位（值为配方
//! 之后的静态实参，按序）。配方与常量值取自引导方法静态实参（`classfile` 的
//! `template` / `template_consts` 口径）。

use classfile::Const;
use ir::{ConcatPart, Expr, Lit};
use sim::StackSim;
use ty::RsType;

use super::boxing::obj_text;
use super::{numeric_const_text, raw_stmt, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;

/// 实参位 / 常量位标记码元
const ARG_SLOT: u16 = 0x0001;
const CONST_SLOT: u16 = 0x0002;

/// 拼接配方：`template` 为配方的 UTF-16 码元，`consts` 为常量位值（按序）
pub(super) struct Recipe {
    pub template: Vec<u16>,
    pub consts: Vec<Vec<u16>>,
}

impl Recipe {
    /// 无常量位的配方（record toString 等由生成器合成的模板）
    pub fn text(template: &str) -> Recipe {
        Recipe { template: template.encode_utf16().collect(), consts: Vec::new() }
    }
}

/// 字符串常量 → UTF-16 码元（含孤立代理项的常量原样保留）
fn const_units(c: &Const) -> Option<Vec<u16>> {
    match c {
        Const::String(s) => Some(s.encode_utf16().collect()),
        Const::StringUtf16(u) => Some(u.clone()),
        _ => None,
    }
}

/// 拼接配方：首个静态实参为字符串时取之；含常量位时携带常量值
/// （字符串 → 码元，数值 → Python `str`；出现其它形态 → 不携带）
fn recipe(site: &IndySite) -> Option<Recipe> {
    let args = &site.bsm?.args;
    let template = const_units(args.first()?)?;
    let mut consts = Vec::new();
    if template.contains(&CONST_SLOT) {
        for c in &args[1..] {
            match const_units(c).or_else(|| numeric_const_text(c).map(|t| t.encode_utf16().collect())) {
                Some(u) => consts.push(u),
                None => {
                    consts.clear();
                    break;
                }
            }
        }
    }
    Some(Recipe { template, consts })
}

/// 常量码元段 → 拼接片段：合法 UTF-16 为文本段，含孤立代理项时为码元段
fn const_part(units: Vec<u16>) -> ConcatPart {
    match std::string::String::from_utf16(&units) {
        Ok(t) => ConcatPart::Text(t),
        Err(_) => ConcatPart::Units(units),
    }
}

/// 按配方把常量段与实参交织成片段序列（实参位数须与 `args` 相符）；常量位按序取值
/// （常量值内的 `\u{1}` 是普通字符，不参与实参位切分），配方未携带常量时 `\u{2}` 保持原样
fn recipe_parts(recipe: &Recipe, args: Vec<Expr>) -> Option<Vec<ConcatPart>> {
    let segs: Vec<&[u16]> = recipe.template.split(|u| *u == ARG_SLOT).collect();
    if segs.len() != args.len() + 1 {
        return None;
    }
    let mut parts = Vec::new();
    let mut buf: Vec<u16> = Vec::new();
    let mut next = 0usize;
    let mut args = args.into_iter();
    for (i, seg) in segs.iter().enumerate() {
        if recipe.consts.is_empty() {
            buf.extend_from_slice(seg);
        } else {
            for (k, chunk) in seg.split(|u| *u == CONST_SLOT).enumerate() {
                if k > 0 {
                    buf.extend(recipe.consts.get(next).into_iter().flatten());
                    next += 1;
                }
                buf.extend_from_slice(chunk);
            }
        }
        if i + 1 < segs.len() {
            if !buf.is_empty() {
                parts.push(const_part(std::mem::take(&mut buf)));
            }
            parts.push(ConcatPart::Arg(args.next()?));
        }
    }
    if !buf.is_empty() {
        parts.push(const_part(buf));
    }
    Some(parts)
}

/// `+` 右操作数是否无需括号（标识符 / 路径 / 调用链等无空白的原子形态）
fn is_atomic(s: &str) -> bool {
    !s.is_empty() && !s.contains(char::is_whitespace) && !s.contains(['+', '*', '/', '%', '<', '>', '=', '|', '^', '{'])
}

fn paren(s: String) -> String {
    if is_atomic(&s) {
        s
    } else {
        format!("({s})")
    }
}

/// 无后缀的整数 / 浮点字面量（`Add` 右操作数有多个数值实现，须带后缀定型）
fn is_bare_number(s: &str) -> bool {
    let d = s.strip_prefix('-').unwrap_or(s);
    d.starts_with(|c: char| c.is_ascii_digit()) && d.chars().all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-'))
}

/// 基本类型实参按形参描述符定型为 `Add` 右操作数：栈类型不符时显式转换，裸字面量补后缀
fn prim_operand(raw_s: String, have: &str, want: &str) -> String {
    if have != want {
        format!("({raw_s} as {want})")
    } else if is_bare_number(&raw_s) {
        format!("{raw_s}{want}")
    } else {
        paren(raw_s)
    }
}

/// 一个拼接实参按 Java 字符串化语义整形为 `Add` 右操作数（`p` 为形参描述符）；
/// 引用实参的 toString 物化为临时变量
fn concat_arg(env: &InstrEnv, sim: &mut StackSim, e: sim::StackEntry, p: &str) -> InstrResult<String> {
    let raw_s = text(env, &e.expr);
    let have = ty_text(env, &e.ty);
    let string_desc = format!("L{};", ty::consts::STRING);
    Ok(match p {
        // Java 浮点数格式化（整数值显示 .0）由运行时 Add<f64> / Add<f32> 承担
        "D" => prim_operand(raw_s, &have, "f64"),
        "F" => prim_operand(raw_s, &have, "f32"),
        // Java char 以 u16 码元追加（孤立代理项原样保留）
        "C" => prim_operand(raw_s, &have, "u16"),
        "B" => prim_operand(raw_s, &have, "i8"),
        "S" => prim_operand(raw_s, &have, "i16"),
        "I" => prim_operand(raw_s, &have, "i32"),
        "J" => prim_operand(raw_s, &have, "i64"),
        // Java 布尔拼接（JLS §5.1.11）呈现 true/false：布尔短路表达式经分支合并以 i32（1/0）
        // 流动，栈类型仍为 bool 时保持原样
        "Z" if have != "bool" => format!("({raw_s} != 0)"),
        "Z" => paren(raw_s),
        // String 实参（栈类型即 String）：按引用追加码元，null → "null" 由运行时承担
        _ if p == string_desc && have == ir::anchors::STRING => format!("&{}", paren(raw_s)),
        _ => {
            // 引用类型实参：Java 语义是 String.valueOf(x)（虚 toString 分派）。预物化为临时变量
            // （toString 返回 Result，需在语句层传播 ?）；Object::toString 经 vtable 桥接，
            // null 给出 "null"
            let boxed = obj_text(env, &raw_s, &e.ty);
            let sv = sim.fresh("_t")?;
            sim.emit(raw_stmt(format!("let {}: {} = {boxed}.toString()?;", sv.as_str(), ir::anchors::STRING)));
            format!("&{}", sv.as_str())
        }
    })
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
    recipe: Option<Recipe>,
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
        args.push(Expr::raw(concat_arg(env, sim, e, p)?));
    }
    // 无配方 / 配方实参位与实参数不符：逐个实参直接拼接
    let parts = match recipe.as_ref().and_then(|r| recipe_parts(r, args.clone())) {
        Some(parts) => parts,
        None => args.into_iter().map(ConcatPart::Arg).collect(),
    };
    sim.push(Expr::Lit(Lit::JStringConcat(parts)), string_t);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arg(s: &str) -> Expr {
        Expr::raw(s)
    }

    #[test]
    fn recipe_keeps_lone_surrogates() {
        // 配方 "\uD800=\u0001;\u0002"，常量位值含孤立低代理项
        let r = Recipe { template: vec![0xD800, 0x3D, ARG_SLOT, 0x3B, CONST_SLOT], consts: vec![vec![0xDC00, 0x41]] };
        let parts = recipe_parts(&r, vec![arg("c")]).unwrap();
        assert_eq!(
            parts,
            vec![ConcatPart::Units(vec![0xD800, 0x3D]), ConcatPart::Arg(arg("c")), ConcatPart::Units(vec![0x3B, 0xDC00, 0x41])]
        );
    }

    #[test]
    fn recipe_text_segments_and_arity() {
        let r = Recipe::text("a{\u{1}}b\u{1}");
        let parts = recipe_parts(&r, vec![arg("x"), arg("y")]).unwrap();
        assert_eq!(
            parts,
            vec![ConcatPart::Text("a{".into()), ConcatPart::Arg(arg("x")), ConcatPart::Text("}b".into()), ConcatPart::Arg(arg("y"))]
        );
        assert!(recipe_parts(&r, vec![arg("x")]).is_none());
        // 常量值内的 \u{1} 不参与实参位切分
        let r = Recipe { template: vec![CONST_SLOT, ARG_SLOT], consts: vec![vec![ARG_SLOT]] };
        assert_eq!(recipe_parts(&r, vec![arg("x")]).unwrap(), vec![ConcatPart::Text("\u{1}".into()), ConcatPart::Arg(arg("x"))]);
    }

    #[test]
    fn prim_operands_are_typed() {
        assert_eq!(prim_operand("5".into(), "i32", "i32"), "5i32");
        assert_eq!(prim_operand("-1.5".into(), "f64", "f64"), "-1.5f64");
        assert_eq!(prim_operand("1.0E10".into(), "f64", "f64"), "1.0E10f64");
        assert_eq!(prim_operand("7i64".into(), "i64", "i64"), "7i64");
        assert_eq!(prim_operand("c".into(), "i32", "u16"), "(c as u16)");
        assert_eq!(prim_operand("a + b".into(), "i64", "i64"), "(a + b)");
        assert_eq!(prim_operand("self.n".into(), "i16", "i16"), "self.n");
    }
}
