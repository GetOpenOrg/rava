//! `rava audit` 报告：缺口表（api / corpus，按类汇总）与 native / 边界缺口表（native）。

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::audit_cmd::Gaps;

/// 预检两类缺口（报告段序）
pub const GAP_KINDS: [&str; 2] = ["native-missing", "boundary-stub"];

/// native 模式收录的闭包种类（手写层应提供而未提供）
const NATIVE_KINDS: [&str; 2] = ["handwritten:native", "handwritten:boundary"];

/// hits[kind][member] = 触达来源（测试名 / `api`）
#[derive(Debug, Default)]
pub struct GapHits(BTreeMap<String, BTreeMap<String, BTreeSet<String>>>);

impl GapHits {
    pub fn add(&mut self, kind: &str, member: &str, src: &str) {
        self.0.entry(kind.into()).or_default().entry(member.into()).or_default().insert(src.into());
    }

    pub fn len(&self, kind: &str) -> usize {
        self.0.get(kind).map_or(0, BTreeMap::len)
    }
}

/// 成员 id `类.方法:描述符` 的类部分
fn class_of(member: &str) -> &str {
    let head = member.split_once(':').map_or(member, |(h, _)| h);
    head.rsplit_once('.').map_or(head, |(c, _)| c)
}

fn write_file(path: &Path, text: &str) -> Result<(), String> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("{}：{e}", path.display()))
}

/// 缺口报告：每类缺口一段，按类汇总（类按其成员最大触达数降序，类内成员按触达数降序）；
/// `total` 为语料测试数（api 模式 None：不列触达数与示例）
pub fn render_gap_report(title: &str, meta: &[String], hits: &GapHits, total: Option<usize>) -> String {
    let mut f = format!("# {title}\n\n");
    for line in meta {
        let _ = writeln!(f, "- {line}");
    }
    f.push('\n');
    let empty = BTreeMap::new();
    for kind in GAP_KINDS {
        let items = hits.0.get(kind).unwrap_or(&empty);
        let _ = write!(f, "## {kind}（{}）\n\n", items.len());
        if items.is_empty() {
            f.push_str("无\n\n");
            continue;
        }
        let mut by_cls: BTreeMap<&str, Vec<(&String, &BTreeSet<String>)>> = BTreeMap::new();
        for (m, srcs) in items {
            by_cls.entry(class_of(m)).or_default().push((m, srcs));
        }
        let mut order: Vec<(&str, Vec<(&String, &BTreeSet<String>)>)> = by_cls.into_iter().collect();
        order.sort_by_key(|(c, ms)| (std::cmp::Reverse(ms.iter().map(|(_, s)| s.len()).max().unwrap_or(0)), *c));
        f.push_str(if total.is_some() { "| 类 | 成员 | 触达测试数 | 示例 |\n|----|------|-----:|------|\n" } else { "| 类 | 成员 |\n|----|------|\n" });
        for (cls, mut members) in order {
            members.sort_by_key(|(m, s)| (std::cmp::Reverse(s.len()), *m));
            for (m, srcs) in members {
                let short = m.strip_prefix(cls).and_then(|r| r.strip_prefix('.')).unwrap_or(m);
                if total.is_some() {
                    let ex: Vec<&str> = srcs.iter().take(3).map(String::as_str).collect();
                    let _ = writeln!(f, "| `{cls}` | `{short}` | {} | {} |", srcs.len(), ex.join(", "));
                } else {
                    let _ = writeln!(f, "| `{cls}` | `{short}` |");
                }
            }
        }
        f.push('\n');
    }
    f
}

pub fn write_gap_report(path: &Path, title: &str, meta: &[String], hits: &GapHits, total: Option<usize>) -> Result<(), String> {
    write_file(path, &render_gap_report(title, meta, hits, total))
}

/// native / 边界缺口行：(触达用例数, 种类, 方法, 示例)，按用例数降序、方法升序；`<clinit>` 不计
pub fn native_rows(results: &[(String, Gaps, Option<String>)]) -> Vec<(usize, String, String, Vec<String>)> {
    let mut hits: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    for (name, g, _) in results {
        for m in g.precheck.native_missing.iter().chain(&g.precheck.boundary_stub) {
            let Some(kind) = g.kinds.get(m).filter(|k| NATIVE_KINDS.contains(&k.as_str())) else { continue };
            if !m.contains("<clinit>") {
                hits.entry((kind.clone(), m.clone())).or_default().insert(name.clone());
            }
        }
    }
    let mut rows: Vec<_> = hits.into_iter().map(|((k, m), ts)| (ts.len(), k, m, ts.into_iter().take(3).collect())).collect();
    rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.2.cmp(&b.2)));
    rows
}

/// 写 `docs/reports/native-gap-scan.md`；返回 (路径, 汇总行)
pub fn native_report(repo: &Path, n_tests: usize, results: &[(String, Gaps, Option<String>)]) -> Result<(PathBuf, String), String> {
    let rows = native_rows(results);
    let mut f = format!("# 语料缺口（{n_tests} 例）\n\n| 用例数 | kind | 方法 | 示例 |\n|---:|---|---|---|\n");
    for (n, k, m, ex) in &rows {
        let _ = writeln!(f, "| {n} | {k} | `{m}` | {} |", ex.join(", "));
    }
    let path = repo.join("docs/reports/native-gap-scan.md");
    write_file(&path, &f)?;
    let native = rows.iter().filter(|r| r.1 == NATIVE_KINDS[0]).count();
    Ok((path, format!("{} missing; native: {native}", rows.len())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use emit::precheck::Precheck;

    #[test]
    fn gap_report_groups_by_class() {
        let mut h = GapHits::default();
        h.add("boundary-stub", "p/A.x:()V", "T1");
        h.add("boundary-stub", "p/A.y:()V", "T1");
        h.add("boundary-stub", "p/A.y:()V", "T2");
        h.add("boundary-stub", "p/B$C.z:(I)V", "T3");
        let text = render_gap_report("t", &["m".into()], &h, Some(3));
        let want = "## boundary-stub（3）\n\n| 类 | 成员 | 触达测试数 | 示例 |\n|----|------|-----:|------|\n\
                    | `p/A` | `y:()V` | 2 | T1, T2 |\n| `p/A` | `x:()V` | 1 | T1 |\n| `p/B$C` | `z:(I)V` | 1 | T3 |\n";
        assert!(text.contains(want), "{text}");
        assert!(text.contains("## native-missing（0）\n\n无\n"), "{text}");
        let api = render_gap_report("t", &[], &h, None);
        assert!(api.contains("| 类 | 成员 |\n|----|------|\n| `p/A` | `y:()V` |\n| `p/A` | `x:()V` |"), "{api}");
    }

    #[test]
    fn native_rows_filter_kind_and_clinit() {
        let g = |stubs: &[&str], kinds: &[(&str, &str)]| Gaps {
            precheck: Precheck { native_missing: vec![], boundary_stub: stubs.iter().map(|s| s.to_string()).collect() },
            kinds: kinds.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(),
        };
        let r = vec![
            ("T1".to_string(), g(&["p/A.a:()V", "p/A.<clinit>:()V", "p/A.b:()V"], &[("p/A.a:()V", "handwritten:boundary"), ("p/A.<clinit>:()V", "handwritten:boundary"), ("p/A.b:()V", "bytecode")]), None),
            ("T2".to_string(), g(&["p/A.a:()V"], &[("p/A.a:()V", "handwritten:boundary")]), None),
        ];
        let rows = native_rows(&r);
        assert_eq!(rows, vec![(2, "handwritten:boundary".to_string(), "p/A.a:()V".to_string(), vec!["T1".to_string(), "T2".to_string()])]);
    }
}
