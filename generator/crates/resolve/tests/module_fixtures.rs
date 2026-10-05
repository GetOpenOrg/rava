//! V12-0 合成夹具：分裂包、重名具名 / 自动模块、JDK 包遮蔽、重复类、多版本覆盖、
//! crate 名冲突后缀、命名兜底链（坐标 / 显式 module）。全部经 `ModuleFacts::build` +
//! `ClassPath` 公共面验证。

use std::io::Write;
use std::path::{Path, PathBuf};

use resolve::classpath::{ClassPath, LibMeta, Origin};
use resolve::modules::{ModuleFacts, ModuleKind};

/// 夹具目录（唯一名，进程退出由系统回收；同进程重复跑先清）
fn dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("rava_modules_fixture_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// 写一个 jar：清单属性（可缺省）+ 条目（名 → 字节）
fn jar(dir: &Path, name: &str, manifest_attrs: &[(&str, &str)], entries: &[(&str, Vec<u8>)]) -> PathBuf {
    let p = dir.join(name);
    let f = std::fs::File::create(&p).unwrap();
    let mut w = zip::ZipWriter::new(f);
    let opt: zip::write::SimpleFileOptions = Default::default();
    if !manifest_attrs.is_empty() {
        let mf = manifest_attrs.iter().map(|(k, v)| format!("{k}: {v}\n")).collect::<String>();
        w.start_file("META-INF/MANIFEST.MF", opt).unwrap();
        w.write_all(mf.as_bytes()).unwrap();
    }
    for (n, data) in entries {
        w.start_file(n.to_string(), opt).unwrap();
        w.write_all(data).unwrap();
    }
    w.finish().unwrap();
    p
}

/// 手工组装的最小 module-info.class（具名模块；requires 仅 java.base mandated + 额外项）。
/// 常量池：1 Utf8"Module" 2 Utf8<名> 3 Module#2 4 Utf8"java.base" 5 Module#4，其后每个
/// requires 追加 Utf8 + Module 对
fn module_info(name: &str, requires: &[&str]) -> Vec<u8> {
    let utf = |b: &mut Vec<u8>, s: &str| {
        b.push(1);
        b.extend((s.len() as u16).to_be_bytes());
        b.extend(s.as_bytes());
    };
    let mut b: Vec<u8> = vec![0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 53, 0, 0]; // 末两位 cp 计数占位
    utf(&mut b, "Module");
    utf(&mut b, name);
    b.extend([19, 0, 2]);
    utf(&mut b, "java.base");
    b.extend([19, 0, 4]);
    let mut req_module_idx = Vec::new();
    for r in requires {
        utf(&mut b, r);
        let utf_idx = 6 + req_module_idx.len() * 2;
        b.push(19);
        b.extend(((utf_idx + 1) as u16).to_be_bytes());
        req_module_idx.push(utf_idx as u16);
    }
    let count = (6 + req_module_idx.len() * 2) as u16;
    b[8..10].copy_from_slice(&count.to_be_bytes());
    // access（ACC_MODULE）this super interfaces fields methods 全零
    b.extend([0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    b.extend(1u16.to_be_bytes()); // class 属性 1 个
    let mut body: Vec<u16> = vec![3, 0, 0, 1 + requires.len() as u16, 5, 0x8000, 0];
    for idx in req_module_idx {
        body.extend([idx + 1, 0, 0]);
    }
    body.extend([0, 0, 0, 0]); // exports / opens / uses / provides 计数
    b.extend(1u16.to_be_bytes()); // 属性名 #1 = "Module"
    b.extend(((body.len() * 2) as u32).to_be_bytes());
    for x in body {
        b.extend(x.to_be_bytes());
    }
    b
}

#[test]
fn split_package_between_automatic_modules_is_recorded_not_fatal() {
    let d = dir("split");
    let a = jar(&d, "liba-1.jar", &[("Automatic-Module-Name", "lib.a")], &[("p/A.class", vec![1])]);
    let b = jar(&d, "libb-1.jar", &[("Automatic-Module-Name", "lib.b")], &[("p/B.class", vec![1])]);
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &a).unwrap();
    cp.add(Origin::Lib, &b).unwrap();
    let f = ModuleFacts::build(&cp);
    assert!(f.errors.is_empty(), "{}", f.errors.join("; "));
    let g = f.graph(&cp);
    assert_eq!(
        g.split_packages().get("p").map(|s| s.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["lib.a", "lib.b"])
    );
}

#[test]
fn duplicate_named_modules_are_an_error() {
    let d = dir("dupnamed");
    let a = jar(&d, "a-1.jar", &[], &[("module-info.class", module_info("same.mod", &[])), ("q/A.class", vec![1])]);
    let b = jar(&d, "b-1.jar", &[], &[("module-info.class", module_info("same.mod", &[])), ("r/B.class", vec![1])]);
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &a).unwrap();
    cp.add(Origin::Lib, &b).unwrap();
    let f = ModuleFacts::build(&cp);
    assert!(f.errors.iter().any(|e| e.contains("same.mod") && e.contains("具名模块")), "{:?}", f.errors);
    assert!(resolve::modules::check(&cp).is_err());
}

#[test]
fn duplicate_automatic_modules_merge_into_one() {
    let d = dir("dupauto");
    let a = jar(&d, "a-1.jar", &[("Automatic-Module-Name", "merges.here")], &[("p/A.class", vec![1])]);
    let b = jar(&d, "b-1.jar", &[("Automatic-Module-Name", "merges.here")], &[("q/B.class", vec![1])]);
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &a).unwrap();
    cp.add(Origin::Lib, &b).unwrap();
    let f = ModuleFacts::build(&cp);
    assert!(f.errors.is_empty(), "{}", f.errors.join("; "));
    assert!(!f.warnings.is_empty());
    let g = f.graph(&cp);
    let n = g.node("merges.here").unwrap();
    assert!(n.automatic && n.kind == ModuleKind::Lib && n.jars.len() == 2);
}

/// 真机 JDK：库 jar 中属于 JDK 具名模块所拥有包的类被遮蔽，其余保留（JVM 类路径同语义）
#[test]
fn jdk_owned_packages_shadow_lib_classes() {
    let Some(home) = resolve::jdk::find_major(21) else { return };
    let d = dir("shadow");
    let j = jar(
        &d,
        "evil-1.jar",
        &[("Automatic-Module-Name", "evil.lib")],
        &[("java/lang/Evil.class", vec![1]), ("org/evil/Ok.class", vec![1])],
    );
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &j).unwrap();
    cp.add_jdk(&home).unwrap();
    cp.shadow_jdk_owned_packages();
    assert!(cp.contains("org/evil/Ok"), "非 JDK 包的库类保留");
    assert!(!cp.contains("java/lang/Evil"), "JDK 包的库类被遮蔽（java.base 无同名类则不可见）");
    let sh = cp.shadowed();
    assert_eq!(sh.len(), 1);
    assert_eq!(sh[0].class, "java/lang/Evil");
    assert!(sh[0].owner.starts_with("java."), "拥有者是 JDK 模块：{}", sh[0].owner);
    let _ = std::fs::remove_dir_all(&d);
}

#[test]
fn duplicate_classes_first_classpath_entry_wins() {
    let d = dir("dupclass");
    let a = jar(&d, "first-1.jar", &[("Automatic-Module-Name", "first.lib")], &[("p/Dup.class", vec![1])]);
    let b = jar(&d, "second-1.jar", &[("Automatic-Module-Name", "second.lib")], &[("p/Dup.class", vec![2])]);
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &a).unwrap();
    cp.add(Origin::Lib, &b).unwrap();
    assert_eq!(cp.duplicates().len(), 1);
    assert_eq!(cp.duplicates()[0].class, "p/Dup");
    assert!(cp.duplicates()[0].winner.ends_with("first-1.jar"));
    assert!(cp.duplicates()[0].loser.ends_with("second-1.jar"));
    assert_eq!(cp.bytes("p/Dup"), Some(vec![1]));
}

/// 多版本 jar：描述符只在 META-INF/versions/9 下 → 具名模块（不是自动模块），
/// 版本化类覆盖基础条目且不以 META-INF/versions 形态入索引
#[test]
fn multi_release_descriptor_makes_named_module() {
    let d = dir("mrmod");
    let j = jar(
        &d,
        "mr-1.jar",
        &[("Multi-Release", "true")],
        &[
            ("p/Base.class", vec![1]),
            ("META-INF/versions/9/module-info.class", module_info("mr.named", &[])),
            ("META-INF/versions/11/p/Base.class", vec![2]),
        ],
    );
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &j).unwrap();
    let f = ModuleFacts::build(&cp);
    assert!(f.errors.is_empty(), "{}", f.errors.join("; "));
    let g = f.graph(&cp);
    let n = g.node("mr.named").expect("版本化描述符应给出具名模块");
    assert!(!n.automatic && n.kind == ModuleKind::Lib);
    assert_eq!(g.module_of("p/Base"), Some("mr.named"));
    assert_eq!(cp.bytes("p/Base"), Some(vec![2]));
    assert!(!cp.contains("META-INF/versions/11/p/Base"));
}

/// crate 名冲突：`a.b_c` 与 `a_b.c` 都映射到 `a_b_c`，名序靠后者带 FNV 4 位十六进制后缀
#[test]
fn crate_name_collision_gets_fnv_suffix() {
    let d = dir("cratecoll");
    let a = jar(&d, "a-1.jar", &[("Automatic-Module-Name", "a.b_c")], &[("p1/A.class", vec![1])]);
    let b = jar(&d, "b-1.jar", &[("Automatic-Module-Name", "a_b.c")], &[("p2/B.class", vec![1])]);
    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &a).unwrap();
    cp.add(Origin::Lib, &b).unwrap();
    let f = ModuleFacts::build(&cp);
    let g = f.graph(&cp);
    let first = g.crate_name("a.b_c").unwrap().to_string();
    let second = g.crate_name("a_b.c").unwrap().to_string();
    let (plain, suffixed) = if first == "a_b_c" { (&first, &second) } else { (&second, &first) };
    assert_eq!(plain, "a_b_c", "名序在前者保持原名");
    assert!(suffixed.starts_with("a_b_c_") && suffixed.len() == "a_b_c_".len() + 4, "{suffixed}");
}

/// 命名兜底链：文件名不可推导时用坐标 artifactId；再不行用锁条目显式 module；全无则报错
#[test]
fn naming_fallback_coordinate_then_explicit_module() {
    let d = dir("fallback");
    let coord = jar(&d, "-1.jar", &[], &[("p/A.class", vec![1])]);
    let explicit = jar(&d, "-2.jar", &[], &[("q/B.class", vec![1])]);

    let mut cp = ClassPath::new(21);
    cp.add(Origin::Lib, &coord).unwrap();
    cp.add(Origin::Lib, &explicit).unwrap();
    assert!(ModuleFacts::build(&cp).errors.iter().any(|e| e.contains("-1.jar")), "无任何命名来源应报错");

    cp.set_lib_meta(&coord, LibMeta { coordinate: Some("org.example:mylib:1.0".into()), module: None });
    cp.set_lib_meta(&explicit, LibMeta { coordinate: None, module: Some("explicit.name".into()) });
    let f = ModuleFacts::build(&cp);
    assert!(f.errors.is_empty(), "{}", f.errors.join("; "));
    let g = f.graph(&cp);
    assert!(g.node("mylib").is_some(), "坐标 artifactId 推导");
    assert!(g.node("explicit.name").is_some(), "锁条目显式 module 兜底");
}
