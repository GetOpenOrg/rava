//! S-17：`typeSwitch` 类引导方法调用点 → 运行时标签判定链（`dynamic._gen_type_switch`）。
//!
//! 调用点描述符 `(Ljava/lang/Object;I)I`：栈上 (selector, restart) → 命中下标。JVM 语义：
//! - selector 为 null → -1（`case null` 由 javac 在 switch 前用 ifnull 单独编译）；
//! - 自 restart 起首个匹配标签的下标；未命中 → 标签数（tableswitch default 消化）；
//! - Class 标签 = 运行时 instanceof；连续相同标签由「首个下标先判」等价去重。
//!
//! guarded pattern（`case X when g`）无需感知：guard 失败路径把 restart 置为下一 case 下标
//! 后跳回本调用点（CFG 回边），`restart <= i` 守卫跳过已否决标签。

use std::fmt::Write as _;

use classfile::Const;
use ir::{Expr, Raw};
use sim::StackSim;
use ty::{Prim, RsType};

use super::boxing::obj_text;
use super::{numeric_const_text, raw, raw_stmt, wrapper_of, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;

/// case 标签（`classfile` 的 switch_labels 口径）：`'c'` Class 标签（基本类型描述符的 Class
/// 常量归一为装箱类）/ `'s'` String 常量 / `'i'` 数值常量 / `'?'` 其余形态（EnumDesc 的
/// CONSTANT_Dynamic 等）
fn labels(site: &IndySite) -> Vec<(char, String)> {
    let Some(bm) = site.bsm else {
        return Vec::new();
    };
    bm.args
        .iter()
        .map(|a| match a {
            Const::Class(name) => ('c', wrapper_of(name).map_or_else(|| name.clone(), str::to_string)),
            Const::String(s) => ('s', s.clone()),
            other => match numeric_const_text(other) {
                Some(t) => ('i', t),
                None => ('?', String::new()),
            },
        })
        .collect()
}

/// Python `json.dumps(str)`（ensure_ascii：非 ASCII 以 `\uXXXX` 转义，BMP 外拆代理对）
fn json_dumps(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    let _ = write!(out, "\\u{u:04x}");
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// typeSwitch 调用点：弹出 (selector, restart)，压入命中下标
pub(super) fn gen_type_switch(env: &InstrEnv, sim: &mut StackSim, site: &IndySite) -> InstrResult<()> {
    let labels = labels(site);
    let restart = sim.pop()?;
    let sel = sim.pop()?;
    let restart_s = text(env, &restart.expr);
    // selector 求值一次（JVM 语义），统一到 Object 后由 ObjectVTable::is_instance_of 按运行时类
    // 判定（G-9）；具体类型经 Object 边界装箱保持对象身份
    let obj_expr = if ty_text(env, &sel.ty) == ir::anchors::OBJECT {
        if matches!(sel.expr, Expr::Var(_) | Expr::Lit(_)) {
            sel.expr
        } else {
            sim.fresh_let("__ts_sel", sel.expr, &sel.ty)?
        }
    } else {
        let coerced = obj_text(env, &text(env, &sel.expr), &sel.ty);
        sim.fresh_let("__ts_sel", Expr::Raw(Raw(coerced)), &RsType::Object)?
    };
    let obj_s = text(env, &obj_expr);
    let unsupported: Vec<String> =
        labels.iter().filter(|(k, _)| !matches!(k, 'c' | 's' | 'i')).map(|(k, _)| k.to_string()).collect();
    if !unsupported.is_empty() {
        // EnumDesc 等（CONSTANT_Dynamic 形态）暂未落地——保持可见的占位失败，不静默给错值
        sim.emit(raw_stmt(format!("/* TODO S-17: typeSwitch 含未支持的标签形态（{}），退化为占位 */", unsupported.join(","))));
        sim.push(raw(format!("{}::default()", ir::anchors::OBJECT)), RsType::Object);
        return Ok(());
    }
    // null → -1；否则按标签序判定（`restart <= i` 守卫实现 restart 语义）；未命中 → 标签数。
    // 标签谓词：'c' 运行时 instanceof；'s' label.equals(selector)；'i' selector 是 Integer
    // 且值相等（判定桥 _ts_str_label_eq / _ts_int_label_eq 经 prelude 导出）
    let mut chain = format!("if _is_jnull(&{obj_s}) {{ -1 }} else ");
    for (i, (kind, val)) in labels.iter().enumerate() {
        let pred = match kind {
            'c' => format!("{obj_s}.is_instance_of(\"{val}\")"),
            's' => format!("_ts_str_label_eq({}, &{obj_s})", json_dumps(val)),
            _ => format!("_ts_int_label_eq({val}, &{obj_s})"),
        };
        let _ = write!(chain, "if {restart_s} <= {i} && {pred} {{ {i} }} else ");
    }
    let _ = write!(chain, "{{ {} }}", labels.len());
    let i32_t = RsType::Prim(Prim::I32);
    let idx = sim.fresh_let("__ts_idx", Expr::Raw(Raw(chain)), &i32_t)?;
    sim.push(idx, i32_t);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::json_dumps;

    #[test]
    fn json_dumps_matches_python() {
        assert_eq!(json_dumps("a\"b\\c"), r#""a\"b\\c""#);
        assert_eq!(json_dumps("x\ny\u{1}"), r#""x\ny\u0001""#);
        assert_eq!(json_dumps("\u{e9}\u{1f600}"), r#""\u00e9\ud83d\ude00""#);
        assert_eq!(json_dumps("\u{7f}"), r#""\u007f""#);
    }
}
