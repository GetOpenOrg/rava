//! 编译前预检（← `callchain.precheck_from_tree` / `print_precheck`）。
//!
//! java_runtime 生成类文本里方法体为 `panic!("stub: …")` / `panic!("native: …")` 且签名在调用链上
//! （分析器方法节点 id）者，即编译前可知的缺口：
//! - `native-missing`：调用链上的 native 方法缺手写实现；
//! - `boundary-stub`：调用链上的方法落为存根（缺手写或未补译）。
//!
//! 口径与 Python 一致：只看 java_runtime crate 的生成文件（带生成标记）。Rust 路径直接取本轮发射
//! 记录的最终文本（与落盘内容同源，免重扫磁盘；同 scratch 上轮幸存文件已由 mod 树清扫删除）。

use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use crate::emission::ClassEmission;
use crate::project::fs::GEN_MARKER;

static PANIC_STUB: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"panic!\("(stub|native): ([^"]+)"\)"#).expect("预检正则"));

/// 缺省明细上限（`--precheck-only` 不设限）
pub const DEFAULT_LIMIT: usize = 40;

/// 预检结果（各自排序去重）
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Precheck {
    pub native_missing: Vec<String>,
    pub boundary_stub: Vec<String>,
}

impl Precheck {
    /// `visited`：分析器方法节点 id（`类.方法:描述符`）
    pub fn scan(emissions: &[ClassEmission], visited: &BTreeSet<String>) -> Precheck {
        let (mut natives, mut stubs) = (BTreeSet::new(), BTreeSet::new());
        let texts = emissions
            .iter()
            .filter(|e| e.crate_name == "java_runtime" && !e.handwritten)
            .map(|e| e.text.as_str())
            .filter(|t| t.contains(GEN_MARKER) && t.contains("panic!(\""));
        for t in texts {
            for c in PANIC_STUB.captures_iter(t) {
                let sig = &c[2];
                if visited.contains(sig) {
                    let set = if &c[1] == "native" { &mut natives } else { &mut stubs };
                    set.insert(sig.to_string());
                }
            }
        }
        Precheck { native_missing: natives.into_iter().collect(), boundary_stub: stubs.into_iter().collect() }
    }

    /// `[precheck]` 汇总行 + 逐条明细（每类封顶 `limit` 行）
    pub fn lines(&self, limit: usize) -> Vec<String> {
        let mut out = vec![format!(
            "[precheck] native-missing={} boundary-stub={}",
            self.native_missing.len(),
            self.boundary_stub.len()
        )];
        for (kind, items) in [("native-missing", &self.native_missing), ("boundary-stub", &self.boundary_stub)] {
            out.extend(items.iter().take(limit).map(|it| format!("[precheck] {kind}: {it}")));
            if items.len() > limit {
                out.push(format!("[precheck] {kind}: … 其余 {} 条", items.len() - limit));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn em(crate_name: &str, handwritten: bool, text: &str) -> ClassEmission {
        ClassEmission { crate_name: crate_name.into(), handwritten, text: text.into(), ..ClassEmission::default() }
    }

    #[test]
    fn scan_and_lines() {
        let body = "rava_macros::java_class! {\n fn a() { panic!(\"stub: p/A.a:()V\") }\n fn b() { panic!(\"native: p/A.b:()V\") }\n \
                    fn c() { panic!(\"stub: p/A.c:()V\") }\n}";
        let ems = vec![em("java_runtime", false, body), em("user", false, body), em("java_runtime", true, body)];
        let visited: BTreeSet<String> = ["p/A.a:()V", "p/A.b:()V"].into_iter().map(String::from).collect();
        let p = Precheck::scan(&ems, &visited);
        assert_eq!(p.native_missing, vec!["p/A.b:()V"]);
        assert_eq!(p.boundary_stub, vec!["p/A.a:()V"]);
        assert_eq!(
            p.lines(40),
            vec![
                "[precheck] native-missing=1 boundary-stub=1",
                "[precheck] native-missing: p/A.b:()V",
                "[precheck] boundary-stub: p/A.a:()V"
            ]
        );
        let many = Precheck { native_missing: vec!["x".into(), "y".into(), "z".into()], boundary_stub: vec![] };
        assert_eq!(many.lines(2).last().unwrap(), "[precheck] native-missing: … 其余 1 条");
    }
}
