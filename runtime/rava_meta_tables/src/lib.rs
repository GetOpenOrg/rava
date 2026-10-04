//! 反射元数据表的渲染（生成器发射层调用；T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）。
//!
//! 输入是一组生成文件文本，扫描其中 java_class! 块的属性，渲染表源码：
//! - [`Side::Archive`]：档案侧（`java_runtime` 声明层 + lib crate）的表，每个表 static 以
//!   `__java_meta_<表名>` 符号导出（JDK `java_meta` crate 承载），java_runtime::meta 以同名 extern 声明读取；
//!   含手写根类 Object 的成员行；
//! - [`Side::User`]：用户 crate 的表，渲染为同名 `const`，并聚合为 `USER_META: UserMeta`（运行时 `meta::UserMeta`，
//!   由调用方导入），入口启动时 `meta::register_user(&USER_META)` 登记，与档案侧表合并查询。
//!
//! 同一档案下档案侧文本与用户程序无关，表源码逐字节相同。
//!
//! 表：类层次（Class.isAssignableFrom）、直接父类、字段（Class.getDeclaredField /
//! Field.get/set）、方法（Class.getDeclaredMethod、MethodHandleNatives.resolve；身份键为
//! (name, descriptor)）、类修饰符、record、<clinit> 类集与 sealed 许可子类型、嵌套、直接超接口、
//! 类级注解。模块服务表、VM 初始系统属性表与栈帧行表由发射层另行渲染。

mod anno_table;
mod class_tables;
mod member_tables;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anno_table::*;
use class_tables::*;
use member_tables::*;

/// 表的归属侧
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// 档案侧：导出符号的 static
    Archive,
    /// 用户侧：`const` + `USER_META` 聚合
    User,
}

/// 用户侧聚合的字段（运行时 `meta::UserMeta` 字段名 ← 表名）
const USER_FIELDS: &[(&str, &str)] = &[
    ("class_hierarchy", "CLASS_HIERARCHY"),
    ("class_direct_super", "CLASS_DIRECT_SUPER"),
    ("class_fields", "CLASS_FIELDS"),
    ("class_methods", "CLASS_METHODS"),
    ("class_modifiers", "CLASS_MODIFIERS"),
    ("class_nest", "CLASS_NEST"),
    ("class_interfaces", "CLASS_INTERFACES"),
    ("class_anno", "CLASS_ANNO"),
    ("clinit_classes", "CLINIT_CLASSES"),
    ("hidden_classes", "HIDDEN_CLASSES"),
    ("permitted_subclasses", "PERMITTED_SUBCLASSES"),
    ("nest_members", "NEST_MEMBERS"),
    ("class_access_flags", "CLASS_ACCESS_FLAGS"),
    ("class_source_file", "CLASS_SOURCE_FILE"),
    ("class_defining_loader", "CLASS_DEFINING_LOADER"),
    ("record_classes", "RECORD_CLASSES"),
    ("record_components", "RECORD_COMPONENTS"),
    // 以下三表由调用方（发射层）在同一文件内以同名 `const` 给出
    ("module_services", "MODULE_SERVICES"),
    ("line_tables", "LINE_TABLES"),
    ("line_numbers", "LINE_NUMBERS"),
];

/// 扫描 `texts`（生成文件文本，顺序无关）渲染全部反射元数据表。用户侧另需调用方在同一文件给出
/// `MODULE_SERVICES` / `LINE_TABLES` / `LINE_NUMBERS` 三个 `const`（`USER_META` 引用之）
pub fn render(texts: &[&str], side: Side) -> String {
    let methods = scan_class_methods(texts);
    let methods = if side == Side::Archive { with_object_ctor_row(methods) } else { methods };
    let parts = [
        render_hierarchy_table(&scan_class_hierarchy(texts)),
        render_direct_super_table(&scan_direct_super(texts)),
        render_field_table(&scan_class_fields(texts)),
        render_method_table(&methods),
        render_modifiers_table(&scan_class_modifiers(texts)),
        render_record_table(&scan_record_classes(texts), &scan_record_components(texts)),
        render_class_meta_table(&scan_flag_classes(texts, "has_clinit"), &scan_flag_classes(texts, "is_hidden"),
            &scan_class_attr(texts, "permitted_subclasses"),
            &scan_class_attr(texts, "nest_members"), &scan_class_attr(texts, "class_access_flags"),
            &scan_class_attr(texts, "source"), &scan_class_attr(texts, "defining_loader")),
        render_nest_table(&scan_nest_meta(texts)),
        render_interfaces_table(&scan_class_interfaces(texts)),
        render_class_anno_table(&scan_class_annos(texts)),
    ];
    let mut out = String::from("// 生成：反射元数据表（rava_meta_tables）。请勿手改。\n\n");
    for p in &parts {
        out.push_str(p);
        out.push('\n');
    }
    match side {
        Side::Archive => out,
        Side::User => {
            let mut out = localize(&out);
            out.push_str("\n/// 用户类的反射元数据行：入口启动时登记（运行时 `meta::register_user`）\n");
            out.push_str("pub static USER_META: UserMeta = UserMeta {\n");
            for (field, table) in USER_FIELDS {
                out.push_str(&format!("    {field}: {table},\n"));
            }
            out.push_str("};\n");
            out
        }
    }
}

/// 导出 static（`#[export_name = "__java_meta_X"] pub static X:`）→ 本地 `pub const X:`
pub fn localize(text: &str) -> String {
    const OPEN: &str = "#[export_name = \"__java_meta_";
    const CLOSE: &str = "\"] pub static ";
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(i) = rest.find(OPEN) {
        out.push_str(&rest[..i]);
        let after = &rest[i + OPEN.len()..];
        let Some(j) = after.find(CLOSE) else {
            out.push_str(&rest[i..]);
            return out;
        };
        out.push_str("pub const ");
        rest = &after[j + CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

/// 目录树下全部 `.rs` 文件（按路径排序）
pub fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut v = walk_rs_files(dir);
    v.sort();
    v
}

/// 生成属性的键与 '=' 之间有对齐填充空格（`#[binary_name       = "..."]`），
/// 此提取器容忍空白；键须以 `#[` 前缀出现，避免子串误配。
fn extract_attr_padded(s: &str, key: &str) -> Option<String> {
    let start = s.find(&format!("#[{}", key))?;
    let rest = s[start + key.len() + 2..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

/// 键名精确匹配的属性取值（`raw_annotations` 不被 `annotations` 等后缀键误配）：
/// 键前一字符须非标识符字符。
fn extract_key(s: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    let mut from = 0usize;
    while let Some(off) = s[from..].find(&pattern) {
        let at = from + off;
        let prev_ok = at == 0 || {
            let c = s[..at].chars().last().unwrap_or(' ');
            !(c.is_ascii_alphanumeric() || c == '_')
        };
        if prev_ok {
            let start = at + pattern.len();
            let end = s[start..].find('"')? + start;
            return Some(s[start..end].to_owned());
        }
        from = at + pattern.len();
    }
    None
}

/// 十六进制文本 → 字节（注解原始属性体，FS-R R4b）。
fn hex_bytes(s: &str) -> Vec<u8> {
    (0..s.len() / 2).filter_map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()).collect()
}

fn extract_attr(s: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    let start = s.find(&pattern)? + pattern.len();
    let end = s[start..].find('"')? + start;
    Some(s[start..end].to_owned())
}

/// 访问标志 / 修饰符词串 → java.lang.reflect.Modifier 位集。
/// 未知 token（varargs 等）忽略。
fn modifier_bits(s: &str) -> i32 {
    let mut bits = 0i32;
    for tok in s.split_whitespace() {
        bits |= match tok {
            "public"       => 0x0001,
            "private"      => 0x0002,
            "protected"    => 0x0004,
            "static"       => 0x0008,
            "final"        => 0x0010,
            "synchronized" => 0x0020,
            "volatile"     => 0x0040,
            "transient"    => 0x0080,
            // 方法侧同位异名（JVMS access_flags：字段 volatile=0x40/方法
            // bridge=0x40、字段 transient=0x80/方法 varargs=0x80——
            // java.lang.reflect.Modifier 对方法读 varargs 位）
            "varargs"      => 0x0080,
            "native"       => 0x0100,
            // 类侧专有（java_class! 块 modifiers 属性，attrs._class_modifiers_str）：
            // 生成的接口块带 super_class=Object，class_modifier_bits 的「无父类即
            // 接口」推断对其不成立——INTERFACE/ANNOTATION 位以修饰词为准
            "interface"    => 0x0200,
            "abstract"     => 0x0400,
            "annotation"   => 0x2000,
            "strictfp"     => 0x0800,
            "synthetic"    => 0x1000,
            "enum"         => 0x4000,
            _ => 0,
        };
    }
    bits
}

/// 布尔属性提取（`key = true/false`，容忍对齐空格）；缺席 → None。
fn extract_flag(s: &str, key: &str) -> Option<bool> {
    let start = s.find(&format!("{} ", key))?;
    let rest = s[start + key.len()..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    if rest.starts_with("true") { Some(true) }
    else if rest.starts_with("false") { Some(false) }
    else { None }
}

fn walk_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    walk_dir(dir, &mut result);
    result
}

fn walk_dir(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() { walk_dir(&path, out); }
        else if path.extension().map(|e| e == "rs").unwrap_or(false) { out.push(path); }
    }
}
