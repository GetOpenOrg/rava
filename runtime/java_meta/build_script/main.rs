//! java_meta 构建脚本：扫描整个 workspace 的 java_class! 属性，生成反射元数据表
//!（OUT_DIR/*_table.rs，由 src/lib.rs 包含）。
//!
//! 类宇宙 = 运行时 crate 树（../java_runtime/src）+ 用户 crate 树（../user/src）+ lib crate 树
//!（jar 输入模式的兄弟 crate，如 junit4/hamcrest）。用户类、lib 类与生成 JDK 类同一属性协议。
//!
//! 表：类层次（Class.isAssignableFrom）、直接父类、字段（Class.getDeclaredField /
//! Field.get/set）、方法（Class.getDeclaredMethod、MethodHandleNatives.resolve；身份键为
//! (name, descriptor)）、类修饰符、record、<clinit> 类集与 sealed 许可子类型、嵌套、直接超接口、
//! 类级注解、模块服务（来自 closure.json 的服务事实，非 java_class! 属性）。表元素类型定义在 java_runtime::meta（手写）；每个表 static 以
//! `__java_meta_<表名>` 符号导出，java_runtime 以同名 extern 声明读取——用户类变化只重编
//! 本 crate，不重编 java_runtime（方案 docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S1）。

mod anno_table;
mod class_tables;
mod member_tables;
mod closure_tables;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anno_table::*;
use class_tables::*;
use member_tables::*;

fn main() {
    let mut meta_roots: Vec<PathBuf> = vec![PathBuf::from("../java_runtime/src"), PathBuf::from("../user/src")];
    for lib_root in discover_lib_crate_roots() {
        meta_roots.push(lib_root);
    }
    for root in &meta_roots {
        println!("cargo:rerun-if-changed={}", root.display());
    }
    let meta_roots: Vec<&Path> = meta_roots.iter()
        .filter(|p| p.is_dir())
        .map(|p| p.as_path())
        .collect();
    write_hierarchy_table(&scan_class_hierarchy(&meta_roots));
    write_direct_super_table(&scan_direct_super(&meta_roots));
    write_field_table(&scan_class_fields(&meta_roots));
    write_method_table(&with_object_ctor_row(scan_class_methods(&meta_roots)));
    write_modifiers_table(&scan_class_modifiers(&meta_roots));
    write_record_table(&scan_record_classes(&meta_roots), &scan_record_components(&meta_roots));
    write_class_meta_table(&scan_clinit_classes(&meta_roots), &scan_class_attr(&meta_roots, "permitted_subclasses"),
        &scan_class_attr(&meta_roots, "nest_members"), &scan_class_attr(&meta_roots, "class_access_flags"),
        &scan_class_attr(&meta_roots, "source"));
    write_nest_table(&scan_nest_meta(&meta_roots));
    write_interfaces_table(&scan_class_interfaces(&meta_roots));
    write_class_anno_table(&scan_class_annos(&meta_roots));
    // 分析器导出的闭包事实（scratch 根下 closure_input/，与本 crate 同级）
    closure_tables::write_closure_tables(Path::new("../closure_input/closure.json"));
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

/// 兄弟 lib crate 的 src 树发现（jar 输入模式：junit4/hamcrest 等）。判据 =
/// 兄弟目录含 Cargo.toml + src/（排除自身/用户/target）。确定序（排序）保证
/// 表生成稳定。
fn discover_lib_crate_roots() -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir("..") else { return Vec::new() };
    let mut roots: Vec<PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir()
            && p.join("Cargo.toml").is_file()
            && p.join("src").is_dir()
            && !p.file_name().and_then(|n| n.to_str())
                .is_some_and(|n| ["java_runtime", "java_meta", "user"].contains(&n)
                    // 实现层 crate（拆 crate S4）是声明层类块的副本，类宇宙已由 java_runtime 覆盖
                    || n.starts_with("java_body_")))
        .map(|p| p.join("src"))
        .collect();
    roots.sort();
    roots
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
