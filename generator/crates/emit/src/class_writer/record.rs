//! Record 类的 toString / hashCode / equals 字段级实现（← `class_writer._patch_record_method_blocks`）。
//!
//! 这三个方法的字节码经 `ObjectMethods` 引导的 invokedynamic 实现；方法体生成器对该引导
//! 输出 `/* TODO: invokedynamic` 占位，本模块按文本识别后以字段访问器实现整体替换。
//! 这是 Python 的文本补丁形态（照搬，见 GOLDEN_DIFF.md）；终态由方法体生成器直接翻译
//! `ObjectMethods` 引导点，本模块随之删除。

use ty::ident::safe_ident;
use ty::ClassInfo;

use super::methods::PRIMITIVE_RUST_TYPES;
use crate::ctx::EmitCtx;
use crate::lang;

const INDY_TODO: &str = "/* TODO: invokedynamic";

fn getter(name: &str) -> String {
    format!("this.__get_{}()", safe_ident(name))
}

/// toString 分量文本：基本类型 / 字符串直接格式化（浮点按 Java 表示），其余经 Object.toString()
fn component_text(ctx: &EmitCtx<'_>, desc: &str, name: &str) -> String {
    let ty = ctx.ty.jvm_to_rust(desc).render(ctx.ty.names);
    let string_ty = ctx.ty.jvm_to_rust(&format!("L{};", lang::STRING)).render(ctx.ty.names);
    let get = getter(name);
    if PRIMITIVE_RUST_TYPES.contains(&ty.as_str()) || ty == string_ty {
        return match ty.as_str() {
            "f64" => format!("java_fmt_f64({get})"),
            "f32" => format!("java_fmt_f32({get})"),
            _ => get,
        };
    }
    format!("Into::<Object>::into({get}).toString()?")
}

/// hashCode 分量（`ObjectMethods.makeHashCode` 语义）
fn component_hash(desc: &str, name: &str) -> String {
    let get = getter(name);
    match desc {
        "I" | "S" | "B" | "C" => format!("({get} as i32)"),
        "Z" => format!("(if {get} {{ 1231i32 }} else {{ 1237i32 }})"),
        "J" => format!("{{ let v = {get}; (v ^ ((v as u64) >> 32) as i64) as i32 }}"),
        "D" => format!(
            "{{ let v = {get}; let b = if v.is_nan() {{ 0x7ff8000000000000u64 }} else {{ v.to_bits() }}; (b ^ (b >> 32)) as i32 }}"
        ),
        "F" => format!("{{ let v = {get}; (if v.is_nan() {{ 0x7fc00000u32 }} else {{ v.to_bits() }}) as i32 }}"),
        _ => format!("{{ let c = Into::<Object>::into({get}); if _is_jnull(&c) {{ 0i32 }} else {{ c.hashCode()? }} }}"),
    }
}

/// equals 分量比较：基本类型 `==`；String 值等；其余引用经擦除 Object.equals 虚分派
fn component_eq(desc: &str, name: &str) -> String {
    let f = safe_ident(name);
    let primitive = desc.len() == 1 && "ZBCSIJFD".contains(desc);
    if primitive {
        format!("this.__get_{f}() == other.__get_{f}()")
    } else if desc == format!("L{};", lang::STRING) {
        format!("this.__get_{f}().to_string() == other.__get_{f}().to_string()")
    } else {
        format!("Into::<Object>::into(this.__get_{f}()).equals(Into::<Object>::into(other.__get_{f}()))?")
    }
}

/// Record 类：替换经 invokedynamic 占位的 toString / hashCode / equals 方法块
pub fn patch_record_method_blocks(ctx: &EmitCtx<'_>, ci: &ClassInfo, record_ty: &str, blocks: Vec<String>) -> Vec<String> {
    if ci.super_class() != lang::RECORD || ci.is_interface() {
        return blocks;
    }
    let fields: Vec<(&str, &str)> =
        ci.fields().iter().filter(|f| !f.is_static()).map(|f| (f.name.as_str(), f.desc.as_str())).collect();
    let simple = ci.name().rsplit('$').next().unwrap_or("").rsplit('/').next().unwrap_or("");
    let fmt_parts: Vec<String> = fields.iter().map(|(n, _)| format!("{n}={{}}")).collect();
    let fmt_str = format!("{simple}[{}]", fmt_parts.join(", "));
    let fmt_args: Vec<String> = fields.iter().map(|(n, d)| component_text(ctx, d, n)).collect();
    let fmt_args = fmt_args.join(", ");
    blocks
        .into_iter()
        .map(|block| {
            if !block.contains(INDY_TODO) {
                return block;
            }
            let head = |n: &str| block.find(&format!("pub fn {n}(")).map(|i| block[..i].to_string());
            if let Some(attr) = head("toString") {
                let call = if fmt_args.is_empty() {
                    format!("String::from(\"{fmt_str}\")")
                } else {
                    format!("String::from(format!(\"{fmt_str}\", {fmt_args}).as_str())")
                };
                format!("{attr}pub fn toString(&self) -> Result<String> {{\n    let this = self;\n    Ok({call})\n}}")
            } else if let Some(attr) = head("hashCode") {
                let lines: String = fields
                    .iter()
                    .map(|(n, d)| format!("    h = h.wrapping_mul(31).wrapping_add({});\n", component_hash(d, n)))
                    .collect();
                format!("{attr}pub fn hashCode(&self) -> Result<i32> {{\n    let this = self;\n    let mut h: i32 = 0;\n{lines}    Ok(h)\n}}")
            } else if let Some(attr) = head("equals") {
                let cmps: Vec<String> = fields.iter().map(|(n, d)| component_eq(d, n)).collect();
                let cmp = if cmps.is_empty() { "true".to_string() } else { cmps.join(" && ") };
                format!(
                    "{attr}pub fn equals(&self, mut o: Object) -> Result<bool> {{\n    let this = self;\n    if !o.is_instance_of(\"{}\") {{\n        return Ok(false);\n    }}\n    let other = Into::<{record_ty}>::into(Clone::clone(&o));\n    Ok({cmp})\n}}",
                    ci.name()
                )
            } else {
                block
            }
        })
        .collect()
}
