//! 字段：类型解析（← `emitter/field_gen.py`）、继承链字段展平与 struct 行
//! （← `_gen_class_rs` 对应段）、static 字段声明（← `clinit_extract._gen_static_field_blocks`）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::{Const, Field};
use ty::ident::safe_ident;
use ty::{ClassInfo, RsType};

use super::attrs::{constant_value_str, field_attr};
use super::head::superclass_chain;
use crate::ctx::EmitCtx;
use crate::text::contains_word;

/// 不需要注册表校验的内建类型名（`JArray` 为 Rust 端数组包装）
const BUILTIN_TYPES: [&str; 20] = [
    "Object", "String", "i32", "i64", "f32", "f64", "bool", "u16", "i8", "i16", "u32", "u64", "()", "Rc",
    "__Shared", "Vec", "RefCell", "usize", "u8", "JArray",
];
/// Rust 结构符号（非类型名）
const RUST_TOKENS: [&str; 5] = ["", "mut", "dyn", "static", "impl"];
/// 基本类型的 Rust 名（`constants.PRIMITIVE_RUST_TYPES`）
pub const PRIMITIVE_RUST_TYPES: [&str; 13] =
    ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize", "()"];

/// 类型文本中的全部类型名（按 `<>, &'[]:` 切分）
fn type_names(text: &str) -> Vec<&str> {
    text.split(['<', '>', ',', ' ', '&', '\'', '[', ']', ':'])
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .collect()
}

/// 类型文本中的类型名是否全部可用（内建 / 类型形参 / 注册表短名）
pub fn validate_field_type(ctx: &EmitCtx<'_>, text: &str, type_params: &[String]) -> bool {
    type_names(text).into_iter().all(|n| {
        RUST_TOKENS.contains(&n)
            || BUILTIN_TYPES.contains(&n)
            || type_params.iter().any(|p| p == n)
            || ctx.ty.is_registry_short(n)
    })
}

/// 字段 Rust 类型：外部引用字段 → 有效的字段签名类型 → 描述符
pub fn resolve_field_rust(ctx: &EmitCtx<'_>, f: &Field, tps: &[String]) -> RsType {
    if let Some(t) = ctx.ty.outer_ref_field_type(&f.name, &f.desc, tps) {
        return t;
    }
    let sig = f.signature.as_deref().unwrap_or("");
    if let Some(g) = ctx.ty.parse_field_type(sig, tps) {
        if g != RsType::Object && validate_field_type(ctx, &g.render(&ctx.ty), tps) {
            return g;
        }
    }
    ctx.ty.jvm_to_rust(&f.desc)
}

/// 祖先字段 Rust 类型：按祖先自身形参解析签名，再按 `anc_map`（祖先形参 → 本类视角实参）代入
fn resolve_anc_field_rust(
    ctx: &EmitCtx<'_>,
    f: &Field,
    anc_params: &[String],
    anc_map: Option<&BTreeMap<String, RsType>>,
) -> RsType {
    let sig = f.signature.as_deref().unwrap_or("");
    let g = ctx
        .ty
        .outer_ref_field_type(&f.name, &f.desc, anc_params)
        .or_else(|| ctx.ty.parse_field_type(sig, anc_params));
    if let Some(g) = g {
        if g != RsType::Object && validate_field_type(ctx, &g.render(&ctx.ty), anc_params) {
            return match anc_map {
                Some(m) if !m.is_empty() => g.substitute(&|n| m.get(n).cloned()),
                _ => g,
            };
        }
    }
    ctx.ty.jvm_to_rust(&f.desc)
}

/// 继承链展平结果（宏输入）
#[derive(Debug, Default)]
pub struct SuperFields {
    /// (字段 Rust 名, 本类视角类型文本)，父类字段在前
    pub fields: Vec<(String, String)>,
    /// 祖先按类型变量声明、本类视角代入为基本类型的字段
    pub reference: Vec<String>,
    /// 祖先按自身类型形参声明的字段（声明方已 Object 化）
    pub erased: Vec<String>,
    /// 祖先声明为 volatile 的字段（存储访问器取顺序一致原子序，普通字段取 relaxed）
    pub volatile: Vec<String>,
    /// Rust 名与 Java 名不同的继承字段：`声明类.Java 名=Rust 名`（宏属性 `field_slots`）
    pub slots: Vec<String>,
    /// 与 `fields` 逐项对应的（声明类, Java 字段名, 描述符）：引导映像按声明类与字段名定位存储槽
    pub origin: Vec<(String, String, String)>,
}

/// `field_slots` 项：Rust 名与 Java 名不同时给出 `声明类.Java 名=Rust 名`
fn field_slot(decl: &str, java: &str, rust: &str) -> Option<String> {
    (java != rust).then(|| format!("{decl}.{java}={rust}"))
}

/// 本类实例字段中 Rust 名与 Java 名不同者（`field_slots` 的本类部分，继承部分见 [`SuperFields::slots`]）
pub fn own_field_slots(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    ci.fields()
        .iter()
        .filter(|f| !f.is_static())
        .filter_map(|f| field_slot(ci.name(), &f.name, &ctx.ty.instance_field_rust_name(ci.name(), &safe_ident(&f.name))))
        .collect()
}

/// 继承链字段展平（祖先形参按 `ancestor_type_args` 代入）
pub fn flatten_super_fields(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> SuperFields {
    let mut out = SuperFields::default();
    let anc_args: BTreeMap<String, Vec<RsType>> = ctx.ty.ancestor_type_args(ci, None).into_iter().collect();
    let names = &ctx.ty;
    let mut declared = BTreeSet::new();
    for anc in superclass_chain(ctx, ci).into_iter().rev() {
        let params = ctx.ty.effective_class_type_params(anc);
        let mut sub = anc_args.get(anc.name()).cloned().unwrap_or_default();
        while sub.len() < params.len() {
            sub.push(RsType::Object);
        }
        let anc_map: BTreeMap<String, RsType> = params.iter().cloned().zip(sub).collect();
        for f in anc.fields().iter().filter(|f| !f.is_static()) {
            let name = ctx.ty.instance_field_rust_name(anc.name(), &safe_ident(&f.name));
            if !declared.insert(name.clone()) {
                continue;
            }
            out.slots.extend(field_slot(anc.name(), &f.name, &name));
            let view = resolve_anc_field_rust(ctx, f, &params, Some(&anc_map)).render(names);
            // 恒等映射 = 不代入：声明方视角类型
            let declared_ty = resolve_anc_field_rust(ctx, f, &params, None).render(names);
            if PRIMITIVE_RUST_TYPES.contains(&view.as_str()) && !PRIMITIVE_RUST_TYPES.contains(&declared_ty.as_str()) {
                out.reference.push(name.clone());
            }
            if params.iter().any(|p| contains_word(&declared_ty, p)) {
                out.erased.push(name.clone());
            }
            if f.access & super::attrs::ACC_VOLATILE != 0 {
                out.volatile.push(name.clone());
            }
            out.origin.push((anc.name().to_string(), f.name.clone(), f.desc.clone()));
            out.fields.push((name, view));
        }
    }
    out
}

/// 本类实例字段的 struct 行（父类已展平的同名字段跳过）；`java_vis`：lib crate 的 Java 可见性映射
pub fn struct_lines(ctx: &EmitCtx<'_>, ci: &ClassInfo, tps: &[String], sup: &SuperFields, java_vis: bool) -> Vec<String> {
    let super_names: BTreeSet<&str> = sup.fields.iter().map(|(n, _)| n.as_str()).collect();
    let ex = ctx.extras(ci.name());
    let mut lines = Vec::new();
    for (i, f) in ci.fields().iter().enumerate() {
        if f.is_static() {
            continue;
        }
        let name = ctx.ty.instance_field_rust_name(ci.name(), &safe_ident(&f.name));
        if super_names.contains(name.as_str()) {
            continue;
        }
        lines.push(format!("    {}", field_attr(f, ex.fields.get(i), false)));
        let vis = if java_vis { super::visibility::java_member_vis(f.access) } else { "pub" };
        lines.push(format!("    {vis} {name}: {},", resolve_field_rust(ctx, f, tps).render(&ctx.ty)));
    }
    lines
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// `[A-Za-z_][A-Za-z0-9_]*` 标识符序列
fn idents(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if is_ident_start(b[i] as char) {
            let s = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push(&text[s..i]);
        } else {
            i += 1;
        }
    }
    out
}

/// static 字段的 Rust 类型文本（签名优先；引用类型形参 / 无效时回退描述符）
pub(crate) fn static_field_rust(ctx: &EmitCtx<'_>, f: &Field, tps: &[String]) -> String {
    let names = &ctx.ty;
    let sig = f.signature.as_deref().unwrap_or("");
    let gs = ctx.ty.parse_field_type(sig, tps).map(|t| t.render(names)).filter(|g| {
        !g.is_empty()
            && (g == "Object" || validate_field_type(ctx, g, tps))
            && !tps.iter().any(|tp| idents(g).contains(&tp.as_str()))
    });
    gs.unwrap_or_else(|| ctx.ty.jvm_to_rust(&f.desc).render(names))
}

/// static 字段声明块：ConstantValue → `pub const`；类型存根 → panic 存根访问器
/// （共置手写覆盖 / 核心转发除外）；其余 → `pub static`（初值由 `<clinit>` 翻译写入）
pub fn static_field_blocks(ctx: &EmitCtx<'_>, ci: &ClassInfo, tps: &[String], type_only: bool) -> Vec<String> {
    let hw = ctx.input.handwritten.get(ci.name());
    let ex = ctx.extras(ci.name());
    // 带映像常量值的静态字段：存储由根门面的映像模块定义，本层声明外部静态（计划 2026-10-05 §5.10）
    let image = crate::project::boot_image::image_statics_of(ctx, ci.name());
    let mut blocks = Vec::new();
    for (i, sf) in ci.fields().iter().enumerate() {
        if !sf.is_static() {
            continue;
        }
        // 与方法 / 其它 static 字段的访问器同名时加 `_field` 后缀（与访问端 instr `StaticField.accessor` 同口径）
        let fname = ci.static_accessor(&sf.name);
        let ty = static_field_rust(ctx, sf, tps);
        let meta = field_attr(sf, ex.fields.get(i), crate::phase2::dispatch::static_reflected(ctx, ci.name(), &sf.name));
        let head = format!("{meta}\n// static field: {}:{}\n", sf.name, sf.desc);
        let cv = sf.constant_value.as_ref().map(constant_value_str).unwrap_or_default();
        let (cls, fnm, fd) = (ci.name(), &sf.name, &sf.desc);
        if let Some(expr) = ctx.manifest.vm_injected_statics.get(&format!("{cls}.{fnm}")) {
            if !hw.is_some_and(|h| h.methods.contains(&fname)) {
                blocks.push(injected_static_block(&head, &fname, &ty, expr));
            }
        } else if let Some(Const::StringUtf16(units)) = &sf.constant_value {
            blocks.push(format!("{head}pub const {fname}: {ty} = {};", utf16_const_literal(units)));
        } else if !cv.is_empty() {
            blocks.push(format!("{head}pub const {fname}: {ty} = {};", const_literal(&ty, &cv)));
        } else if type_only {
            if hw.is_some_and(|h| h.methods.contains(&fname)) {
                continue;
            }
            let setter = format!("pub fn set_{fname}(v: {ty}) -> Result<()> {{\n    __stub(\"stub-set: {cls}.{fnm}:{fd}\")\n}}");
            if let Some((core, core_ret)) = hw.and_then(|h| h.method_cores.get(&fname)) {
                let cr = core_ret.strip_prefix("Result<").and_then(|s| s.strip_suffix('>'));
                let cast = match cr {
                    Some(cr) if cr != ty && PRIMITIVE_RUST_TYPES.contains(&cr) && PRIMITIVE_RUST_TYPES.contains(&ty.as_str()) => {
                        format!(" as {ty}")
                    }
                    _ => String::new(),
                };
                blocks.push(format!(
                    "{head}pub fn {fname}() -> Result<{ty}> {{\n    Ok(Self::{core}()?{cast})\n}}\n{setter}"
                ));
                continue;
            }
            let stub = crate::precheck::stub_call("stub", &format!("{cls}.{fnm}:{fd}"));
            blocks.push(format!("{head}pub fn {fname}() -> Result<{ty}> {{\n    {stub}\n}}\n{setter}"));
        } else if let Some(sym) = image.get(&sf.name) {
            blocks.push(format!("{head}#[image_static = \"{sym}\"]\npub static {fname}: {ty};"));
        } else {
            blocks.push(format!("{head}pub static {fname}: {ty};"));
        }
    }
    blocks
}

/// VM 注入的静态字段（`[vm_constants.injected_statics]`）：访问器读清单给出的取值表达式
/// （返回 i64 / bool，按字段类型做数值宽度转换）；字节码写入被 VM 值覆盖，setter 不落存储。
fn injected_static_block(head: &str, fname: &str, ty: &str, expr: &str) -> String {
    let cast = if ty == "bool" { String::new() } else { format!(" as {ty}") };
    format!(
        "{head}pub fn {fname}() -> Result<{ty}> {{\n    Ok({expr}{cast})\n}}\n\
         // VM 注入值覆盖字节码写入（同 HotSpot 在 <clinit> 之后改写）\n\
         pub fn set_{fname}(_v: {ty}) -> Result<()> {{\n    Ok(())\n}}"
    )
}

/// ConstantValue 的 Rust 字面量
/// 含孤立代理项的 String 常量（Rust `&str` 无法表示）：UTF-16 码元数组形态，与 ldc 同形
fn utf16_const_literal(units: &[u16]) -> String {
    let body: Vec<String> = units.iter().map(|u| format!("0x{u:04X}")).collect();
    format!("String::from_utf16_lit(&[{}])", body.join(", "))
}

fn const_literal(ty: &str, cv: &str) -> String {
    let float = |suffix: &str| match cv {
        "inf" => format!("{suffix}::INFINITY"),
        "-inf" => format!("{suffix}::NEG_INFINITY"),
        "NaN" => format!("{suffix}::NAN"),
        _ => format!("{cv}{suffix}"),
    };
    match ty {
        "String" => format!("String::from(\"{cv}\")"),
        "f32" => float("f32"),
        "f64" => float("f64"),
        "i64" => format!("{cv}i64"),
        "bool" => (if cv == "1" { "true" } else { "false" }).to_string(),
        _ => cv.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_literals() {
        assert_eq!(type_names("Rc<RefCell<Vec<T>>>, &'a [X]"), vec!["Rc", "RefCell", "Vec", "T", "a", "X"]);
        assert_eq!(idents("Foo<T1, _x>"), vec!["Foo", "T1", "_x"]);
        assert_eq!(const_literal("f32", "1e-05"), "1e-05f32");
        assert_eq!(utf16_const_literal(&[0x61, 0xD83D]), "String::from_utf16_lit(&[0x0061, 0xD83D])");
        assert_eq!(const_literal("f64", "-inf"), "f64::NEG_INFINITY");
        assert_eq!(const_literal("bool", "1"), "true");
        assert_eq!(const_literal("String", "a"), "String::from(\"a\")");
    }

    #[test]
    fn injected_static_accessors() {
        let b = injected_static_block("// h\n", "PAGE_SIZE", "i32", "crate::vm_constants::page_size()");
        assert!(b.contains("pub fn PAGE_SIZE() -> Result<i32> {\n    Ok(crate::vm_constants::page_size() as i32)\n}"));
        assert!(b.contains("pub fn set_PAGE_SIZE(_v: i32) -> Result<()> {\n    Ok(())\n}"));
        let b = injected_static_block("", "BIG_ENDIAN", "bool", "crate::vm_constants::big_endian()");
        assert!(b.contains("Ok(crate::vm_constants::big_endian())"));
        let b = injected_static_block("", "allowSecurityManager", "i32", "1i64");
        assert!(b.contains("Ok(1i64 as i32)"));
    }
}
