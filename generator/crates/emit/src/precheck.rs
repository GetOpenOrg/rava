//! 编译前预检（← `callchain.precheck_from_tree` / `print_precheck`）。
//!
//! java_runtime 生成类文本里的存根调用（方法体存根 [`stub_call`]：`__stub("stub: …")` / `__stub("native: …")`；
//! 调用点存根 `panic!("stub: …")`）且签名在调用链上（分析器方法节点 id）者，即编译前可知的缺口：
//! - `native-missing`：调用链上的 native 方法缺手写实现；
//! - `boundary-stub`：调用链上的方法落为存根（缺手写或未补译）。
//!
//! 只看 java_runtime crate 的生成类（带生成标记）。扫描点在第二阶段收尾之后、物理拆层之前
//! （[`write_project`](crate::project::write_project) / [`scan_gaps`](crate::project::scan_gaps)）：
//! 拆层后声明层的方法体已省略、体在实现层 crate，拆层前的整块文本才是完整的方法体集合。

use std::collections::BTreeSet;
use std::sync::LazyLock;

use regex::Regex;

use crate::emission::ClassEmission;
use crate::project::fs::GEN_MARKER;

/// 方法体存根调用的运行时函数（`java_runtime::__stub`，共享冷路径）
pub const STUB_FN: &str = "__stub";

/// 方法体存根调用文本：全部方法体存根经此生成，预检据同一形态识别
pub fn stub_call(kind: &str, sig: &str) -> String {
    format!("{STUB_FN}(\"{kind}: {sig}\")")
}

static PANIC_STUB: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r#"(?:{STUB_FN}|panic!)\("(stub|native): ([^"]+)"\)"#)).expect("预检正则"));

/// 缺省明细上限（`--full-precheck` 不设限）
pub const DEFAULT_LIMIT: usize = 40;

/// 预检结果（各自排序去重）
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Precheck {
    pub native_missing: Vec<String>,
    pub boundary_stub: Vec<String>,
}

impl Precheck {
    /// `visited`：分析器方法节点 id（`类.方法:描述符`）
    pub fn scan<'e>(emissions: impl IntoIterator<Item = &'e ClassEmission>, visited: &BTreeSet<String>) -> Precheck {
        let (mut natives, mut stubs) = (BTreeSet::new(), BTreeSet::new());
        let texts = emissions
            .into_iter()
            .filter(|e| e.crate_name == "java_runtime" && !e.handwritten)
            .map(|e| e.text.as_str())
            .filter(|t| t.contains(GEN_MARKER) && (t.contains(STUB_FN) || t.contains("panic!(\"")));
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
        let (a, b) = (stub_call("stub", "p/A.a:()V"), stub_call("native", "p/A.b:()V"));
        let body = format!(
            "rava_macros::java_class! {{\n fn a() {{ {a} }}\n fn b() {{ {b} }}\n fn c() {{ panic!(\"stub: p/A.c:()V\") }}\n \
             fn d() {{ panic!(\"stub: p/A.d:()V\") }}\n}}"
        );
        let body = body.as_str();
        let ems = vec![em("java_runtime", false, body), em("user", false, body), em("java_runtime", true, body)];
        let visited: BTreeSet<String> = ["p/A.a:()V", "p/A.b:()V", "p/A.d:()V"].into_iter().map(String::from).collect();
        let p = Precheck::scan(&ems, &visited);
        assert_eq!(p.native_missing, vec!["p/A.b:()V"]);
        assert_eq!(p.boundary_stub, vec!["p/A.a:()V", "p/A.d:()V"]);
        assert_eq!(
            p.lines(40),
            vec![
                "[precheck] native-missing=1 boundary-stub=2",
                "[precheck] native-missing: p/A.b:()V",
                "[precheck] boundary-stub: p/A.a:()V",
                "[precheck] boundary-stub: p/A.d:()V"
            ]
        );
        let many = Precheck { native_missing: vec!["x".into(), "y".into(), "z".into()], boundary_stub: vec![] };
        assert_eq!(many.lines(2).last().unwrap(), "[precheck] native-missing: … 其余 1 条");
    }
}
