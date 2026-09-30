//! overlay / 落盘 / mod 树规则单元测试（临时目录）

use std::path::{Path, PathBuf};

use super::fs::Writer;
use super::mod_tree::{complete_lib_rs, write_mod_tree};
use super::overlay::prepare_scratch;

fn tmp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("rava_emit_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn put(p: &Path, s: &str) {
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, s).unwrap();
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap()
}

/// 手写真源：runtime/java_runtime + runtime/rava_macros
fn runtime(root: &Path) -> PathBuf {
    let rt = root.join("runtime").join("java_runtime");
    put(&rt.join("src/lib.rs"), "pub mod java;\npub mod jdk;\npub mod jdk_resources;\npub mod sun;\n");
    put(&rt.join("src/java/lang/object.rs"), "// 手写\n");
    put(&rt.join("src/java/lang/object_impl.rs"), "// 手写 impl\n");
    put(&rt.join("src/jdk_resources/mod.rs"), "pub mod module_resources;\n");
    put(&rt.join("build.rs"), "fn main() {}\n");
    put(
        &rt.join("Cargo.toml"),
        "[package]\nname = \"java_runtime\"\nversion = \"0.1.0\"\n[dependencies]\nrava_macros = { path = \"../rava_macros\" }\n",
    );
    rt
}

#[test]
fn overlay_copies_rewrites_and_prunes() {
    let root = tmp("overlay");
    let rt = runtime(&root);
    let out = root.join("build").join("t");
    let macros = root.join("runtime").join("rava_macros");
    // 上轮遗留：已从 runtime/ 删除的手写文件 + 生成文件
    put(&out.join("java_runtime/src/java/lang/gone.rs"), "// 旧手写\n");
    put(&out.join("java_runtime/src/java/lang/string.rs"), "rava_macros::java_class! {}\n");
    prepare_scratch(&out, &rt, &macros, false).unwrap();
    let src = out.join("java_runtime/src");
    assert_eq!(read(&src.join("java/lang/object.rs")), "// 手写\n");
    assert!(!src.join("java/lang/gone.rs").exists(), "陈旧手写文件清除");
    assert!(src.join("java/lang/string.rs").exists(), "生成文件保留（mod 树阶段清扫）");
    let cargo = read(&out.join("java_runtime/Cargo.toml"));
    assert!(cargo.contains(&format!("path = \"{}\"", macros.display())));
    assert!(!cargo.contains("0.1.0"));
    assert!(src.join("sun/mod.rs").exists(), "顶层占位 mod.rs");
    assert!(out.join("java_runtime/build.rs").exists());
    prepare_scratch(&out, &rt, &macros, true).unwrap();
    assert!(!src.join("java/lang/string.rs").exists(), "--clean 清空");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn writer_never_overwrites_handwritten() {
    let root = tmp("writer");
    let rt = runtime(&root);
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), false).unwrap();
    let mut w = Writer::new(&out, &rt.join("src"));
    let obj = out.join("java_runtime/src/java/lang/object.rs");
    assert!(w.is_handwritten(&obj));
    w.write(&obj, "rava_macros::java_class! {}\n").unwrap();
    assert_eq!(read(&obj), "// 手写\n");
    let lib = out.join("java_runtime/src/lib.rs");
    assert!(!w.is_handwritten(&lib), "lib.rs / mod.rs 不按手写判定");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn mod_tree_declares_disk_contents() {
    let root = tmp("modtree");
    let rt = runtime(&root);
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), false).unwrap();
    let src = out.join("java_runtime/src");
    let mut w = Writer::new(&out, &rt.join("src"));
    w.write(&src.join("java/lang/string.rs"), "rava_macros::java_class! {}\n").unwrap();
    w.write(&src.join("java/lang/r#ref/reference.rs"), "x").unwrap();
    w.write(&src.join("java/util/stream/collectors_collector_impl.rs"), "rava_macros::java_class! {}\n").unwrap();
    put(&src.join("java/util/stale.rs"), "rava_macros::java_class! {}\n");
    put(&src.join("jdk_resources/module_resources.rs"), "pub fn lookup() {}\n");
    write_mod_tree(&src, Some(&rt), &mut w).unwrap();
    assert!(!src.join("java/util/stale.rs").exists(), "本轮未写的生成文件清扫");
    assert_eq!(
        read(&src.join("java/lang/mod.rs")),
        "#![allow(ambiguous_glob_reexports)]\npub mod object;\npub use object::*;\npub mod r#ref;\n\
         pub mod string;\npub use string::*;\nmod object_impl;\n"
    );
    assert_eq!(
        read(&src.join("java/util/stream/mod.rs")),
        "#![allow(ambiguous_glob_reexports)]\npub mod collectors_collector_impl;\npub use collectors_collector_impl::*;\n"
    );
    assert_eq!(read(&src.join("jdk_resources/mod.rs")), "pub mod module_resources;\n", "手写模块目录不重建");
    std::fs::create_dir_all(src.join("javax")).unwrap();
    put(&src.join("javax/mod.rs"), "");
    complete_lib_rs(&src).unwrap();
    assert!(read(&src.join("lib.rs")).ends_with("非手写清单成员）\npub mod javax;\n"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn companion_skipped_when_used_module_absent() {
    let root = tmp("companion");
    let rt = runtime(&root);
    put(
        &rt.join("src/java/lang/invoke/natives_impl.rs"),
        "use crate::prelude::*;\nuse crate::java::lang::Class;\nuse super::member_name::MemberName;\n\
         impl Natives { pub fn f(x: MemberName) {} }\n",
    );
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), false).unwrap();
    let src = out.join("java_runtime/src");
    let dir = src.join("java/lang/invoke");
    let gen = "rava_macros::java_class! {}\n";
    let mut w = Writer::new(&out, &rt.join("src"));
    w.write(&dir.join("natives.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), &mut w).unwrap();
    assert!(!read(&dir.join("mod.rs")).contains("mod natives_impl;"), "依赖模块缺席 → 不声明");
    let mut w = Writer::new(&out, &rt.join("src"));
    w.write(&dir.join("natives.rs"), gen).unwrap();
    w.write(&dir.join("member_name.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), &mut w).unwrap();
    assert!(read(&dir.join("mod.rs")).ends_with("mod natives_impl;\n"), "依赖齐 → 声明");
    let _ = std::fs::remove_dir_all(&root);
}
