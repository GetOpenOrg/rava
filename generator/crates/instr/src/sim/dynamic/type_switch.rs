//! S-17：`typeSwitch` / `enumSwitch` 类引导方法调用点 → 运行时标签判定链（`dynamic._gen_type_switch`）。
//!
//! 调用点描述符 `(Ljava/lang/Object;I)I`：栈上 (selector, restart) → 命中下标。JVM 语义：
//! - selector 为 null → -1（`case null` 由 javac 在 switch 前用 ifnull 单独编译）；
//! - 自 restart 起首个匹配标签的下标；未命中 → 标签数（tableswitch default 消化）；
//! - Class 标签 = 运行时 instanceof；连续相同标签由「首个下标先判」等价去重；
//! - 枚举常量标签（typeSwitch 的 EnumDesc 动态常量 / enumSwitch 的常量名）= 与该常量同一对象。
//!
//! guarded pattern（`case X when g`）无需感知：guard 失败路径把 restart 置为下一 case 下标
//! 后跳回本调用点（CFG 回边），`restart <= i` 守卫跳过已否决标签。

use std::fmt::Write as _;

use classfile::Const;
use ir::{Expr};
use sim::StackSim;
use ty::{Prim, RsType};

use ty::type_map::parse_descriptor_params;

use super::boxing::obj_text;
use super::{numeric_const_text, wrapper_of, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::sim::fields::static_field_read;

/// case 标签
#[derive(Debug, Clone, PartialEq, Eq)]
enum Label {
    /// Class 标签（基本类型描述符的 Class 常量归一为装箱类）：运行时 instanceof
    Class(String),
    /// String 常量：`label.equals(selector)`
    Str(String),
    /// 数值常量（文本）：selector 是 Integer 且值相等
    Int(String),
    /// 枚举常量（类 binary 名, 常量名）：selector 与该常量同一对象
    Enum(String, String),
}

/// switch 引导方法的形态（`SwitchBootstraps.typeSwitch` / `enumSwitch`）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SwitchKind {
    Type,
    /// 选择子为枚举类型：String 标签是该枚举的常量名
    Enum,
}

/// 动态常量 → 枚举常量标签（`EnumDesc.of(ClassDesc.of("p.C"), "NAME")`，经常量引导方法调用工厂）。
/// 按结构识别，不依赖引导方法类名：外层静态实参 `[工厂句柄, 类描述动态常量, 常量名]`，
/// 内层 `[工厂句柄, 点分类名]`
fn enum_desc(cf: &classfile::ClassFile, c: &Const) -> Option<(String, String)> {
    let Const::Dynamic(outer, _, _) = c else {
        return None;
    };
    let [Const::MethodHandle(_), Const::Dynamic(inner, _, _), Const::String(name)] =
        cf.bootstrap_methods.get(usize::from(*outer))?.args.as_slice()
    else {
        return None;
    };
    let [Const::MethodHandle(_), Const::String(dotted)] = cf.bootstrap_methods.get(usize::from(*inner))?.args.as_slice()
    else {
        return None;
    };
    Some((dotted.replace('.', "/"), name.clone()))
}

/// 标签序列；无法识别的静态实参形态 → OutOfScope（javac 只产出上述四种）
fn labels(env: &InstrEnv, site: &IndySite, kind: SwitchKind) -> InstrResult<Vec<Label>> {
    let Some(bm) = site.bsm else {
        return Err(InstrError::BadInsn(format!("switch 调用点 {} 缺引导方法", site.name)));
    };
    // enumSwitch：String 标签属于选择子（首个动态实参）的枚举类
    let enum_cls = parse_descriptor_params(site.desc)
        .first()
        .and_then(|d| d.strip_prefix('L').and_then(|r| r.strip_suffix(';')).map(str::to_string));
    bm.args
        .iter()
        .map(|a| match a {
            Const::Class(name) => Ok(Label::Class(wrapper_of(name).map_or_else(|| name.clone(), str::to_string))),
            Const::String(s) if kind == SwitchKind::Enum => match &enum_cls {
                Some(c) => Ok(Label::Enum(c.clone(), s.clone())),
                None => Err(InstrError::BadInsn(format!("enumSwitch 选择子不是类类型：{}", site.desc))),
            },
            Const::String(s) => Ok(Label::Str(s.clone())),
            other => {
                if let Some(t) = numeric_const_text(other) {
                    return Ok(Label::Int(t));
                }
                env.ctx
                    .class_file()
                    .and_then(|cf| enum_desc(cf, other))
                    .map(|(c, n)| Label::Enum(c, n))
                    .ok_or_else(|| InstrError::OutOfScope(format!("switch 标签形态 {other:?}")))
            }
        })
        .collect()
}

/// 枚举常量标签的判定：先按类型判定（常量类未初始化时不触发其读取），再比对象同一
fn enum_pred(env: &InstrEnv, obj_s: &str, cls: &str, name: &str) -> InstrResult<String> {
    if !env.ctx.reg().contains(cls) {
        // 枚举类不在闭包：不存在其实例，标签恒不命中
        return Ok("false".to_string());
    }
    let (e, t) = static_field_read(env, cls, name, &format!("L{cls};"))?;
    let konst = obj_text(env, &text(env, &e), &t)?;
    Ok(format!("({obj_s}.is_instance_of(\"{cls}\") && {obj_s} == {konst})"))
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

/// typeSwitch / enumSwitch 调用点：弹出 (selector, restart)，压入命中下标
pub(super) fn gen_type_switch(env: &InstrEnv, sim: &mut StackSim, site: &IndySite, kind: SwitchKind) -> InstrResult<()> {
    let labels = labels(env, site, kind)?;
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
        let coerced = obj_text(env, &text(env, &sel.expr), &sel.ty)?;
        sim.fresh_let("__ts_sel", Expr::raw(coerced), &RsType::Object)?
    };
    let obj_s = text(env, &obj_expr);
    // null → -1；否则按标签序判定（`restart <= i` 守卫实现 restart 语义）；未命中 → 标签数。
    // 标签谓词：Class 运行时 instanceof；String label.equals(selector)；数值 selector 是 Integer
    // 且值相等（判定桥 _ts_str_label_eq / _ts_int_label_eq 经 prelude 导出）；枚举常量同一对象
    let mut chain = format!("if _is_jnull(&{obj_s}) {{ -1 }} else ");
    for (i, label) in labels.iter().enumerate() {
        let pred = match label {
            Label::Class(c) => format!("{obj_s}.is_instance_of(\"{c}\")"),
            Label::Str(s) => format!("_ts_str_label_eq({}, &{obj_s})", json_dumps(s)),
            Label::Int(v) => format!("_ts_int_label_eq({v}, &{obj_s})"),
            Label::Enum(c, n) => enum_pred(env, &obj_s, c, n)?,
        };
        let _ = write!(chain, "if {restart_s} <= {i} && {pred} {{ {i} }} else ");
    }
    let _ = write!(chain, "{{ {} }}", labels.len());
    let i32_t = RsType::Prim(Prim::I32);
    let idx = sim.fresh_let("__ts_idx", Expr::raw(chain), &i32_t)?;
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
