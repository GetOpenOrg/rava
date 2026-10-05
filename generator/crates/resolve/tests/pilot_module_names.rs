//! pilot jar 模块名验收：本机 pilot-libs 的全部 jar 与参考 JDK `ModuleFinder`
//! 给出的名字 / 种类逐一对账。期望表由 `scripts/pilot_module_expected.sh` 生成
//! （pom 取包变更后重跑刷新）；jar 目录缺失时跳过（新检出 / 未取包环境）。

use std::path::PathBuf;

use resolve::classpath::{ClassPath, Origin};
use resolve::modules::ModuleFacts;

#[test]
fn pilot_jar_module_names_match_reference_jdk() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let libs = manifest.join("../../../tests/lib_pilot/deps/target/pilot-libs");
    if !libs.is_dir() {
        eprintln!("跳过：pilot-libs 不存在（{}）", libs.display());
        return;
    }
    let tsv = std::fs::read_to_string(manifest.join("tests/pilot_module_names.tsv"))
        .expect("期望表缺失：跑 scripts/pilot_module_expected.sh 生成");
    let expected: Vec<(String, String, String)> = tsv
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.split('\t');
            (it.next().unwrap().into(), it.next().unwrap().into(), it.next().unwrap().into())
        })
        .collect();
    assert!(!expected.is_empty(), "期望表为空");

    // 目录里出现期望表没有的 jar = 取包后未刷新期望表
    let on_disk: Vec<String> = std::fs::read_dir(&libs)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "jar"))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    for j in &on_disk {
        assert!(expected.iter().any(|(n, _, _)| n == j), "jar {j} 不在期望表：重跑 scripts/pilot_module_expected.sh");
    }

    let mut mismatches = Vec::new();
    for (jar, want_name, want_kind) in &expected {
        let p = libs.join(jar);
        assert!(p.is_file(), "期望表中的 jar 不存在：{jar}");
        let mut cp = ClassPath::new(21);
        cp.add(Origin::Lib, &p).unwrap();
        let f = ModuleFacts::build(&cp);
        assert!(f.errors.is_empty(), "{jar}：{}", f.errors.join("; "));
        let (name, node) = f.graph(&cp).nodes().iter().next().unwrap_or_else(|| panic!("{jar}：无模块节点"));
        let got_kind = if node.automatic { "automatic" } else { "named" };
        if name != want_name || got_kind != want_kind {
            mismatches.push(format!("{jar}：得 {name}/{got_kind}，期望 {want_name}/{want_kind}"));
        }
        // 版本化条目不以 META-INF/versions 形态进入类索引（此前 byte-buddy 等以 3,078 条错误类名入索引）
        if let Some(bad) = cp.names_of(Origin::Lib).iter().find(|n| n.starts_with("META-INF/versions/")) {
            mismatches.push(format!("{jar}：版本化条目入索引：{bad}"));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} / {} 不符：\n{}",
        mismatches.len(),
        expected.len(),
        mismatches.join("\n")
    );
}
