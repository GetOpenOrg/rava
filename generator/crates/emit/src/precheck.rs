//! 编译前预检（← `callchain.precheck_from_tree` / `print_precheck`）。
//!
//! java_runtime 生成类文本里的方法体存根（[`stub_call`]：`__stub("stub: …")` / `__stub("native: …")`；
//! 旧形态 `panic!("stub: …")` 同样识别）且签名在调用链上（分析器方法节点 id）者，即编译前可知的缺口：
//! - `native-missing`：调用链上的 native 方法缺手写实现；
//! - `boundary-stub`：调用链上的方法落为存根（缺手写或未补译）。
//!
//! 只看 java_runtime crate 的生成类（带生成标记），且只认声明类自身文本里的存根：子类里的继承转发副本
//! 以声明者签名作标签，为存根时表示该槽未被派发（`slot_stub`），不代表方法体缺失。
//! 扫描点在第二阶段收尾之后、物理拆层之前
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
            // JDK 类（模块 crate 内，路径首段 `crate`）的生成文件
            .filter(|e| e.crate_prefix == "crate" && !e.handwritten)
            .filter(|e| e.text.contains(GEN_MARKER) && (e.text.contains(STUB_FN) || e.text.contains("panic!(\"")));
        for e in texts {
            for c in PANIC_STUB.captures_iter(&e.text) {
                let sig = &c[2];
                let own = sig.split_once(':').and_then(|(h, _)| h.rsplit_once('.')).is_some_and(|(cls, _)| cls == e.binary_name);
                if own && visited.contains(sig) {
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
        let binary_name = "p/A".into();
        let crate_prefix = if crate_name == "user" { "rt" } else { "crate" }.into();
        ClassEmission { binary_name, crate_name: crate_name.into(), crate_prefix, handwritten, text: text.into(), ..ClassEmission::default() }
    }

    #[test]
    fn scan_and_lines() {
        let (a, b) = (stub_call("stub", "p/A.a:()V"), stub_call("native", "p/A.b:()V"));
        let body = format!(
            "rava_macros::java_class! {{\n fn a() {{ {a} }}\n fn b() {{ {b} }}\n fn c() {{ panic!(\"stub: p/A.c:()V\") }}\n \
             fn d() {{ panic!(\"stub: p/A.d:()V\") }}\n}}"
        );
        let body = body.as_str();
        // 子类 p/B 里继承转发副本的槽存根（声明者签名）不计
        let sub = ClassEmission { binary_name: "p/B".into(), ..em("java_runtime", false, body) };
        let ems = vec![em("java_runtime", false, body), em("user", false, body), em("java_runtime", true, body), sub];
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

    #[test]
    fn over_default_limit_keeps_totals_and_reports_rest() {
        // 45 个可达 native 缺口 + 1 个存根：首行总数恒为全量，明细封顶 DEFAULT_LIMIT，截断行报出其余条数
        let n = DEFAULT_LIMIT + 5;
        let mut body = String::from("rava_macros::java_class! {\n");
        let mut visited = BTreeSet::new();
        for i in 0..n {
            let sig = format!("p/A.n{i:02}:()V");
            body.push_str(&format!(" fn n{i:02}() {{ {} }}\n", stub_call("native", &sig)));
            visited.insert(sig);
        }
        body.push_str(&format!(" fn s() {{ {} }}\n}}", stub_call("stub", "p/A.s:()V")));
        visited.insert("p/A.s:()V".to_string());
        let p = Precheck::scan(&[em("java_runtime", false, &body)], &visited);
        assert_eq!(p.native_missing.len(), n);
        let lines = p.lines(DEFAULT_LIMIT);
        assert_eq!(lines[0], format!("[precheck] native-missing={n} boundary-stub=1"));
        assert_eq!(lines.iter().filter(|l| l.starts_with("[precheck] native-missing: p/")).count(), DEFAULT_LIMIT);
        assert_eq!(lines[DEFAULT_LIMIT + 1], "[precheck] native-missing: … 其余 5 条");
        assert_eq!(lines.last().unwrap(), "[precheck] boundary-stub: p/A.s:()V");
        // 不设限（--full-precheck）：全部列出、无截断行
        let full = p.lines(usize::MAX);
        assert_eq!(full.len(), 1 + n + 1);
        assert!(!full.iter().any(|l| l.contains("其余")));
    }
}
