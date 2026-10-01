//! `--api-package`：公开 API 包为调用链入口（`scripts/gap_scan.py` api 模式，← Python `JDK_SEEDS`）。
//!
//! 指定包内全部 public 类的 public / protected 方法（`<clinit>` 除外）作为闭包根，一次 BFS 得出
//! 「从这些公开 API 可达」的全部缺口（`--precheck-only` 明细）。C1d 终态无包前缀截断，边界只剩 VM 契约类，
//! 按方法划分，照常纳入。包名斜线或点形态均可；`recursive` 含子包。

use classfile::{acc, MemberRef};
use resolve::{ClassPath, Origin};

/// 类所在包是否在入口包集内
fn in_packages(cls: &str, pkgs: &[String], recursive: bool) -> bool {
    let pkg = cls.rsplit_once('/').map_or("", |(p, _)| p);
    pkgs.iter().any(|p| pkg == p || (recursive && pkg.strip_prefix(p.as_str()).is_some_and(|r| r.starts_with('/'))))
}

/// 入口方法（类名序、类内声明序）与入口类数
pub fn api_roots(cp: &ClassPath, packages: &[String], recursive: bool) -> (Vec<MemberRef>, usize) {
    let pkgs: Vec<String> = packages.iter().map(|p| p.replace('.', "/").trim_end_matches('/').to_string()).collect();
    let (mut roots, mut n_cls) = (Vec::new(), 0);
    for name in cp.names_of(Origin::Jdk) {
        if !in_packages(&name, &pkgs, recursive) || name.ends_with("module-info") || name.ends_with("package-info") {
            continue;
        }
        let Some(cf) = cp.get(&name) else { continue };
        if cf.access & acc::PUBLIC == 0 {
            continue;
        }
        n_cls += 1;
        for m in cf.methods.iter().filter(|m| m.name != "<clinit>" && m.access & (acc::PUBLIC | acc::PROTECTED) != 0) {
            roots.push(MemberRef { owner: name.clone(), name: m.name.clone(), desc: m.desc.clone() });
        }
    }
    (roots, n_cls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use closure::manifest::Manifest;

    #[test]
    fn package_membership() {
        let p = vec!["java/util".to_string()];
        assert!(in_packages("java/util/List", &p, false));
        assert!(!in_packages("java/util/concurrent/Future", &p, false));
        assert!(in_packages("java/util/concurrent/Future", &p, true));
        assert!(!in_packages("java/utilx/A", &p, true));
        assert!(!in_packages("java/lang/String", &p, true));
    }

    /// 真 JDK：只取 public 类的 public / protected 非 `<clinit>` 方法；内部包（无包前缀截断）与
    /// VM 耦合边界类（按方法划分）照常纳入
    #[test]
    fn real_jdk_roots() {
        let Some(home) = resolve::jdk::find_major(21) else { return };
        let rt = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime");
        let mut cp = ClassPath::new();
        cp.add_jdk(&home).unwrap();
        let man = Manifest::load(&rt).unwrap();
        let (roots, n_cls) = api_roots(&cp, &["java.util.function".to_string()], false);
        assert!(n_cls > 30 && !roots.is_empty());
        for r in &roots {
            let cf = cp.get(&r.owner).unwrap();
            assert!(cf.access & acc::PUBLIC != 0 && r.name != "<clinit>", "{r:?}");
            let m = cf.methods.iter().find(|m| m.name == r.name && m.desc == r.desc).unwrap();
            assert!(m.access & (acc::PUBLIC | acc::PROTECTED) != 0, "{r:?}");
        }
        assert!(api_roots(&cp, &["jdk/internal/misc".to_string()], true).1 > 0, "内部包照常纳入");
        let (lang, _) = api_roots(&cp, &["java/lang".to_string()], false);
        let vm: Vec<&MemberRef> = lang.iter().filter(|r| man.is_vm_boundary(&r.owner)).collect();
        assert!(!vm.is_empty(), "VM 耦合边界类照常纳入");
        let (rec, _) = api_roots(&cp, &["java/util".to_string()], true);
        assert!(rec.iter().any(|r| r.owner.starts_with("java/util/function/")));
    }
}
