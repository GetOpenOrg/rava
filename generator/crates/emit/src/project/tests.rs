//! overlay / 落盘 / mod 树规则单元测试（临时目录）

use std::path::{Path, PathBuf};

use super::fs::Writer;
use super::mod_tree::{complete_lib_rs, sweep_user_crate, write_mod_tree};
use super::overlay::{prepare_scratch, JdkDirs};

/// 单 crate 目录表（声明层目录 = java_runtime，与改造前同一落位）
fn single() -> JdkDirs {
    JdkDirs::single("java_runtime")
}

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
    put(&rt.join("src/jdk_resources/mod.rs"), "pub mod generated_table;\n");
    put(&rt.join("build.rs"), "fn main() {}\n");
    put(
        &rt.join("Cargo.toml"),
        "[package]\nname = \"java_runtime\"\nversion = \"0.1.0\"\n[dependencies]\nrava_macros = { path = \"../rava_macros\" }\nrava_coro = { path = \"../rava_coro\" }\n",
    );
    let meta = root.join("runtime").join("java_meta");
    put(&meta.join("Cargo.toml"), "[package]\nname = \"java_meta\"\nversion = \"0.1.0\"\n");
    put(&meta.join("build_script/main.rs"), "fn main() {}\n");
    put(&meta.join("src/lib.rs"), "// 表\n");
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
    prepare_scratch(&out, &rt, &macros, &single(), false).unwrap();
    let src = out.join("java_runtime/src");
    assert_eq!(read(&src.join("java/lang/object.rs")), "// 手写\n");
    assert!(src.join("java/lang/gone.rs").exists(), "陈旧手写文件留待 mod 树阶段清扫");
    assert!(!src.join("lib.rs").exists(), "lib.rs 由 mod 树阶段写出");
    assert!(src.join("java/lang/string.rs").exists(), "生成文件保留（mod 树阶段清扫）");
    let cargo = read(&out.join("java_runtime/Cargo.toml"));
    assert!(cargo.contains(&format!("path = \"{}\"", macros.display())));
    let coro = root.join("runtime").join("rava_coro");
    assert!(cargo.contains(&format!("path = \"{}\"", coro.display())), "rava_coro 依赖改绝对路径");
    assert!(!cargo.contains("0.1.0"));
    assert!(src.join("sun/mod.rs").exists(), "顶层占位 mod.rs");
    assert!(out.join("java_runtime/build.rs").exists());
    // java_meta 整体镜像：版本唯一化，真源已无的文件删除
    let meta = out.join("java_meta");
    assert_eq!(read(&meta.join("build_script/main.rs")), "fn main() {}\n");
    assert!(!read(&meta.join("Cargo.toml")).contains("0.1.0"));
    put(&meta.join("build_script/gone.rs"), "// 旧\n");
    put(&meta.join("old_dir/gone.rs"), "// 旧\n");
    prepare_scratch(&out, &rt, &macros, &single(), false).unwrap();
    assert!(!meta.join("build_script/gone.rs").exists());
    assert!(!meta.join("old_dir").exists());
    assert!(meta.join("build_script/main.rs").exists());
    assert!(meta.join("src/lib.rs").exists());
    prepare_scratch(&out, &rt, &macros, &single(), true).unwrap();
    assert!(!src.join("java/lang/string.rs").exists(), "--clean 清空");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn writer_never_overwrites_handwritten() {
    let root = tmp("writer");
    let rt = runtime(&root);
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
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
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let src = out.join("java_runtime/src");
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&src.join("java/lang/string.rs"), "rava_macros::java_class! {}\n").unwrap();
    w.write(&src.join("java/lang/r#ref/reference.rs"), "x").unwrap();
    w.write(&src.join("java/util/stream/collectors_collector_impl.rs"), "rava_macros::java_class! {}\n").unwrap();
    put(&src.join("java/util/stale.rs"), "rava_macros::java_class! {}\n");
    w.write(&src.join("jdk_resources/generated_table.rs"), "pub fn lookup() {}\n").unwrap();
    put(&src.join("java/lang/gone.rs"), "// 旧手写\n");
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    assert!(!src.join("java/util/stale.rs").exists(), "本轮未写的生成文件清扫");
    assert!(!src.join("java/lang/gone.rs").exists(), "手写真源已删除的无标记文件清扫");
    assert!(src.join("jdk_resources/generated_table.rs").exists(), "本轮写出的无标记生成文件保留");
    assert_eq!(
        read(&src.join("java/lang/mod.rs")),
        "#![allow(ambiguous_glob_reexports)]\npub mod object;\npub use object::*;\npub mod r#ref;\n\
         pub mod string;\npub use string::*;\nmod object_impl;\n"
    );
    assert_eq!(
        read(&src.join("java/util/stream/mod.rs")),
        "#![allow(ambiguous_glob_reexports)]\npub mod collectors_collector_impl;\npub use collectors_collector_impl::*;\n"
    );
    assert_eq!(read(&src.join("jdk_resources/mod.rs")), "pub mod generated_table;\n", "手写模块目录不重建");
    std::fs::create_dir_all(src.join("javax")).unwrap();
    put(&src.join("javax/mod.rs"), "");
    complete_lib_rs(&src, &rt.join("src"), &mut w).unwrap();
    let lib = read(&src.join("lib.rs"));
    assert!(lib.starts_with("pub mod java;\n") && lib.ends_with("非手写清单成员）\npub mod javax;\n"));
    let mtime = |p: &Path| std::fs::metadata(p).unwrap().modified().unwrap();
    let before = mtime(&src.join("lib.rs"));
    std::thread::sleep(std::time::Duration::from_millis(20));
    complete_lib_rs(&src, &rt.join("src"), &mut w).unwrap();
    assert_eq!(mtime(&src.join("lib.rs")), before, "内容不变不重写");
    let _ = std::fs::remove_dir_all(&root);
}

/// 复用 scratch：包内类全部离开闭包后，陈旧包目录的生成 mod.rs 与空目录清除（E0583 / E0761）；
/// 手写模块目录整棵子树、手写 lib.rs 声明的顶层目录、只剩共置手写的目录保留
#[test]
fn mod_tree_prunes_stale_package_dirs() {
    let root = tmp("modprune");
    let rt = runtime(&root);
    put(&rt.join("src/java/nio/buffer_impl.rs"), "// 手写 impl\n");
    let out = root.join("build").join("t");
    let src = out.join("java_runtime/src");
    // 上轮：java/lang/module/ 包（本轮其类全部离开闭包）、未声明的顶层包 javax/、
    // 手写模块目录下的子目录
    put(&src.join("java/lang/module/mod.rs"), "pub mod descriptor;\n");
    put(&src.join("java/lang/module/descriptor.rs"), "rava_macros::java_class! {}\n");
    put(&src.join("javax/crypto/mod.rs"), "pub mod cipher;\n");
    put(&src.join("javax/mod.rs"), "pub mod crypto;\n");
    put(&src.join("jdk_resources/sub/mod.rs"), "// 手写子树\n");
    put(&src.join("java/nio/mod.rs"), "pub mod buffer;\n");
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&src.join("java/lang/module.rs"), "rava_macros::java_class! {}\n").unwrap();
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    assert!(!src.join("java/lang/module").exists(), "陈旧包目录删除（与 module.rs 并存即 E0761）");
    assert!(read(&src.join("java/lang/mod.rs")).contains("pub mod module;\npub use module::*;"));
    assert!(!src.join("javax").exists(), "lib.rs 未声明的顶层陈旧包删除");
    assert!(src.join("sun/mod.rs").is_file(), "lib.rs 声明的顶层目录保留");
    assert!(src.join("jdk_resources/sub/mod.rs").is_file(), "手写模块目录子树保留");
    assert!(src.join("java/nio/buffer_impl.rs").is_file() && !src.join("java/nio/mod.rs").exists(), "只剩共置手写：删 mod.rs 留文件");
    let _ = std::fs::remove_dir_all(&root);
}

/// 复用 scratch：overlay 按清单与 runtime/ 同步——runtime/ 中删除 / 改名的手写文件与私有辅助目录、
/// 非 `.rs` 资源、改定向到别的 crate 的手写目录，其上轮落盘文件删除、空目录移除；
/// 同路径已由生成器接管（带生成标记）的文件与清单外文件保留
#[test]
fn overlay_ledger_syncs_with_runtime() {
    let root = tmp("ledger");
    let rt = runtime(&root);
    let macros = root.join("runtime").join("rava_macros");
    put(&rt.join("src/java/lang/class_impl.rs"), "mod members;\nuse members::*;\n");
    put(&rt.join("src/java/lang/class_impl/members.rs"), "use super::*;\n");
    put(&rt.join("src/jdk_resources/locale.properties"), "k=v\n");
    put(&rt.join("src/java/nio/buffer_impl.rs"), "// 手写\n");
    put(&rt.join("src/java/util/zip/crc_impl.rs"), "// 手写\n");
    let out = root.join("build").join("t");
    let dirs = |zip: &str| JdkDirs::with_dirs("java_runtime", &[("java/util/zip", Some(zip))]);
    prepare_scratch(&out, &rt, &macros, &dirs("java_runtime"), false).unwrap();
    let src = out.join("java_runtime/src");
    assert!(src.join("java/lang/class_impl/members.rs").is_file() && src.join("jdk_resources/locale.properties").is_file());
    assert!(src.join("java/util/zip/crc_impl.rs").is_file());
    // runtime/ 改动：辅助目录并回宿主、宿主改名、资源删除、手写文件删除（同路径改由生成器写出）、目录改定向
    std::fs::remove_dir_all(rt.join("src/java/lang/class_impl")).unwrap();
    std::fs::rename(rt.join("src/java/lang/class_impl.rs"), rt.join("src/java/lang/klass_impl.rs")).unwrap();
    std::fs::remove_file(rt.join("src/jdk_resources/locale.properties")).unwrap();
    std::fs::remove_file(rt.join("src/java/nio/buffer_impl.rs")).unwrap();
    put(&src.join("java/nio/buffer_impl.rs"), "rava_macros::java_class! {}\n");
    put(&src.join("java/lang/notes.txt"), "清单外\n");
    prepare_scratch(&out, &rt, &macros, &dirs("java_base_zip"), false).unwrap();
    assert!(!src.join("java/lang/class_impl.rs").exists() && !src.join("java/lang/class_impl").exists(), "改名 / 删除的手写文件与辅助目录清除");
    assert!(src.join("java/lang/klass_impl.rs").is_file());
    assert!(!src.join("jdk_resources/locale.properties").exists() && src.join("jdk_resources/mod.rs").is_file(), "非 .rs 资源同步");
    assert_eq!(read(&src.join("java/nio/buffer_impl.rs")), "rava_macros::java_class! {}\n", "带生成标记不删");
    assert!(src.join("java/lang/notes.txt").is_file(), "清单外文件不动");
    assert!(!src.join("java/util").exists(), "改定向：原 crate 的落盘文件删除、空目录移除");
    assert!(out.join("java_base_zip/src/java/util/zip/crc_impl.rs").is_file());
    assert!(src.join("java/lang/object.rs").is_file() && src.join("java/lang/object_impl.rs").is_file());
    let ledger = read(&out.join(super::overlay::LEDGER));
    assert!(ledger.contains("java_base_zip/src/java/util/zip/crc_impl.rs\n") && !ledger.contains("class_impl"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn companion_skipped_when_used_module_absent() {
    let root = tmp("companion");
    let rt = runtime(&root);
    put(
        &rt.join("src/java/lang/invoke/natives_impl.rs"),
        "use crate::prelude::*;\nuse super::member_name::MemberName;\n\
         impl Natives { pub fn f(x: MemberName) {} }\n",
    );
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let src = out.join("java_runtime/src");
    let dir = src.join("java/lang/invoke");
    let gen = "rava_macros::java_class! {}\n";
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&dir.join("natives.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    assert!(!read(&dir.join("mod.rs")).contains("mod natives_impl;"), "依赖模块缺席 → 不声明");
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&dir.join("natives.rs"), gen).unwrap();
    w.write(&dir.join("member_name.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    assert!(read(&dir.join("mod.rs")).ends_with("mod natives_impl;\n"), "依赖齐 → 声明");
    let _ = std::fs::remove_dir_all(&root);
}

/// 包路径下非模块文件段的在场判定（d7f482e1 场景）：手写 pub item（类型 / pub mod）恒在场；
/// 大写生成类按 snake 化文件名探测（`Proxy_Dyn` → `proxy_dyn.rs`）；「目录 + 同名 .rs」并存时条目
/// 可能在 .rs 模块体里；其余缺席（`super::member_name` 而无 member_name.rs）
#[test]
fn companion_item_segments_probed_precisely() {
    let root = tmp("companion_items");
    let rt = runtime(&root);
    put(&rt.join("src/p/hw_ext.rs"), "pub struct HwType;\npub mod hw_mod { pub struct G; }\n");
    // 路径引用只记到首个大写段：小写段恒为中间的模块段
    put(
        &rt.join("src/p/a_impl.rs"),
        "use crate::p::HwType;\nuse crate::p::hw_mod::G;\nuse super::Proxy_Dyn;\n\
         use super::arr::inner::Helper;\nuse crate::p::__Hidden;\n\
         impl A { pub fn f(_a: HwType, _b: Proxy_Dyn, _c: __Hidden, _g: G, _h: Helper) {} }\n",
    );
    put(&rt.join("src/p/b_impl.rs"), "use super::missing_mod::Thing;\nimpl B { pub fn f(_t: Thing) {} }\n");
    put(&rt.join("src/p/c_impl.rs"), "use super::Absent_Gen;\nimpl C { pub fn f(_t: Absent_Gen) {} }\n");
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let src = out.join("java_runtime/src");
    let dir = src.join("p");
    let gen = "rava_macros::java_class! {}\n";
    let write = |extra: &[&str]| {
        let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
        for f in ["a.rs", "b.rs", "c.rs"].iter().chain(extra) {
            w.write(&dir.join(f), gen).unwrap();
        }
        write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
        read(&dir.join("mod.rs"))
    };
    // arr/ 与 arr.rs 并存（inner 在 arr.rs 模块体里）；Proxy_Dyn 未生成 → a_impl 缺依赖
    let m = write(&["arr.rs", "arr/x.rs"]);
    assert!(!m.contains("mod a_impl;"), "snake 生成类缺席 → 不声明：{m}");
    let m = write(&["arr.rs", "arr/x.rs", "proxy_dyn.rs"]);
    assert!(m.contains("mod a_impl;"), "手写 item / snake 生成类 / 目录+同名 .rs / `__` 内部条目均在场 → 声明：{m}");
    assert!(!m.contains("mod b_impl;"), "缺席的小写段 → 不声明：{m}");
    assert!(!m.contains("mod c_impl;"), "缺席的大写段 → 不声明：{m}");
    // 仅有 arr/ 目录、无 arr.rs：inner 无处承载 → a_impl 不声明
    std::fs::remove_file(dir.join("arr.rs")).unwrap();
    let m = write(&["arr/x.rs", "proxy_dyn.rs"]);
    assert!(!m.contains("mod a_impl;"), "目录无同名 .rs 时小写模块段缺席：{m}");
    let _ = std::fs::remove_dir_all(&root);
}

/// 叠层共置：`foo_impl.rs` 是类 Foo 的共置手写，`foo_impl_impl.rs` 是类 FooImpl 的共置手写（FooImpl
/// 生成于让出路径后的 `foo_impl_t.rs`）。两个类都不在本轮时一律不声明——`foo_impl.rs` 不能被当作
/// FooImpl 的宿主而以 `pub mod` 引入；FooImpl 在本轮（`foo_impl_t.rs`）时才声明 `foo_impl_impl`
#[test]
fn stacked_companion_needs_generated_host() {
    let root = tmp("stacked");
    let rt = runtime(&root);
    put(&rt.join("src/java/net/foo_impl.rs"), "use super::foo::Foo;\nimpl Foo {}\n");
    put(&rt.join("src/java/net/foo_impl_impl.rs"), "use super::FooImpl;\nimpl FooImpl {}\n");
    let out = root.join("build").join("t");
    prepare_scratch(&out, &rt, &root.join("m"), &single(), false).unwrap();
    let src = out.join("java_runtime/src");
    let dir = src.join("java/net");
    let gen = "rava_macros::java_class! {}\n";
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&dir.join("bar.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    let m = read(&dir.join("mod.rs"));
    assert!(!m.contains("foo_impl"), "Foo / FooImpl 均缺席 → 不声明：{m}");
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&dir.join("foo_impl_t.rs"), gen).unwrap();
    write_mod_tree(&src, Some(&rt), 2, &mut w).unwrap();
    let m = read(&dir.join("mod.rs"));
    assert!(m.contains("mod foo_impl_impl;") && !m.contains("mod foo_impl;"), "仅 FooImpl 在 → 只声明其共置：{m}");
    let _ = std::fs::remove_dir_all(&root);
}

/// 复用 scratch 换测试：user crate 上轮的生成类文件与陈旧包目录清除，本轮写出与无标记文件保留
#[test]
fn user_crate_sweeps_previous_test() {
    let root = tmp("user_sweep");
    let rt = runtime(&root);
    let out = root.join("build").join("t");
    let src = out.join("user/src");
    let gen = "rava_macros::java_class! {}\n";
    // 上轮（另一测试）遗留
    put(&src.join("deep_copy.rs"), gen);
    put(&src.join("deep_copy_person.rs"), gen);
    put(&src.join("com/acme/old.rs"), gen);
    put(&src.join("com/acme/mod.rs"), "pub mod old;\n");
    put(&src.join("com/mod.rs"), "pub mod acme;\n");
    put(&src.join("org/x/gone.rs"), gen);
    put(&src.join("notes.rs"), "// 非生成\n");
    // 本轮写出
    let mut w = Writer::new(&[out.join("java_runtime/src")], &rt.join("src"));
    w.write(&src.join("digester.rs"), gen).unwrap();
    w.write(&src.join("org/x/y.rs"), gen).unwrap();
    w.write(&src.join("org/x/mod.rs"), "pub mod y;\n").unwrap();
    w.write(&src.join("org/mod.rs"), "pub mod x;\n").unwrap();
    w.write(&src.join("main.rs"), "mod digester;\nmod org;\n").unwrap();
    let mut dirs = std::collections::BTreeMap::new();
    for (d, c) in [(src.clone(), ["digester", "org"].as_slice()), (src.join("org"), &["x"]), (src.join("org/x"), &["y"])] {
        dirs.insert(d, c.iter().map(|s| s.to_string()).collect::<std::collections::BTreeSet<_>>());
    }
    sweep_user_crate(&src, &dirs, &w, 2).unwrap();
    let mut left: Vec<String> = super::fs::walk(&src)
        .into_iter()
        .flat_map(|(d, _, fs)| {
            let rel = d.strip_prefix(&src).unwrap().to_path_buf();
            fs.into_iter().map(move |f| rel.join(f).to_string_lossy().into_owned())
        })
        .collect();
    left.sort();
    assert_eq!(left, ["digester.rs", "main.rs", "notes.rs", "org/mod.rs", "org/x/mod.rs", "org/x/y.rs"]);
    assert!(!src.join("com").exists(), "陈旧包目录整棵删除");
    let _ = std::fs::remove_dir_all(&root);
}
