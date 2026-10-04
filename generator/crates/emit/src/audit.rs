//! 转译审计行（`scripts/run_tests.py` 按行解析汇总），输出序：
//!
//! 1. `[cfg-audit]`：控制流审计（跳转消费 / try 区域 / instanceof 折叠 / 存根兜底位点分解）；
//!    `debug` 时逐条列存根兜底 `[cfg-audit] stub fallback ({site}): {method}: {reason}`；
//! 2. `[readability-audit]`：生成文件（含 `rava_macros::java_class` 标记）中可读层禁用调用形态计数，终态全 0；
//! 3. `[equiv-audit]`：近似 / 条件等价发射点计数，只列非零项；
//! 4. `[fallback-audit]`：静默兜底触发计数（[`crate::fallback`]），只列非零项；
//! 5. `[shortname-audit]`：因遮蔽 Rust prelude 名而限定改名的类；
//! 6. `[raw-audit]`：Raw 逃生舱构造事件（`raw_expr` / `raw_stmt`，[`ir::raw_audit`]，终态 0）+ 手写审计三项
//!    （FS-H0：越界覆盖 / VM 内建 / VM 边界方法，按成员去重）。生成器源码的静态卫生约束
//!    由测试守护（生成器源码无 JDK 类名字面量、类型查询全部经类型层），不输出；
//! 7. `[override-audit]` / `[vm-boundary-audit]`：非空时的逐成员明细；
//! 8. `[module-audit]`：发射类覆盖的模块数与模块可读性越界条数（[`crate::module_audit`]，终态越界 0）。

use std::collections::BTreeSet;
use std::path::Path;

use instr::Audit;
use ir::raw_audit::{self, RawKind};

use crate::ctx::HwAudit;
use crate::fallback::FallbackAudit;
use crate::method_bodies::BodyAudit;

/// 生成文件标记（手写 / 基础设施文件不含）
const GENERATED_MARKER: &str = "rava_macros::java_class";

/// 可读层禁用调用形态（`java-rust-translation-reference.md §16`）：(标签, 文本形态)
const READABILITY_PATTERNS: [(&str, &str); 5] = [
    ("from_any", "Object::from_any"),
    ("downcast", ".downcast::<"),
    ("downcast_ref", "downcast_ref"),
    ("rc_new", "Rc::new("),
    ("borrow", ".borrow()"),
];

/// 可读层禁用形态计数（与 [`READABILITY_PATTERNS`] 同序）
pub type ReadabilityCounts = [(&'static str, usize); 5];

/// 审计行的全部输入
pub struct AuditInputs<'a> {
    pub body: &'a BodyAudit,
    pub hw: &'a [(HwAudit, String)],
    pub fallback: &'a FallbackAudit,
    /// 可读层禁用形态计数（[`readability_counts`]，发射期在拆层前统计）
    pub readability: &'a ReadabilityCounts,
    /// 因 prelude 冲突限定改名的类（binary，已排序）
    pub prelude_disambiguated: &'a [String],
    /// `[module-audit]` 行（[`crate::module_audit::ModuleAudit::lines`]）
    pub module: Vec<String>,
    pub debug: bool,
}

/// 可读层禁用形态计数：含生成标记的类文本逐形态计次。
///
/// 输入取物理拆层（声明层 `java_runtime` / 实现层 `java_body_k`）之前的类发射文本：每个类的
/// 每个方法体恰好计一次——落盘后声明层与实现层各持一份块文本，按磁盘扫描会漏计实现层或重计。
pub fn readability_counts<'t>(texts: impl IntoIterator<Item = &'t str>) -> ReadabilityCounts {
    let mut counts = READABILITY_PATTERNS.map(|(label, _)| (label, 0usize));
    for text in texts {
        if !text.contains(GENERATED_MARKER) {
            continue;
        }
        for (i, (_, pat)) in READABILITY_PATTERNS.iter().enumerate() {
            counts[i].1 += text.matches(pat).count();
        }
    }
    counts
}

/// 审计行
pub fn audit_lines(a: &AuditInputs<'_>) -> Vec<String> {
    let mut lines = vec![a.body.cfg.summary()];
    if a.debug {
        lines.extend(
            a.body.cfg.stub_fallbacks.iter().map(|f| format!("[cfg-audit] stub fallback ({}): {}: {}", f.site, f.method_id, f.reason)),
        );
    }
    lines.push(format!("[readability-audit] {}", a.readability.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")));
    let equiv: Vec<String> = Audit::REPORT_ORDER
        .iter()
        .filter_map(|x| a.body.equiv.get(x).filter(|n| **n > 0).map(|n| format!("{}={n}", x.as_str())))
        .collect();
    lines.push(format!("[equiv-audit] {}", if equiv.is_empty() { "none".to_string() } else { equiv.join(" ") }));
    lines.push(a.fallback.summary());
    let pd = a.prelude_disambiguated;
    lines.push(format!(
        "[shortname-audit] prelude-disambig={}{}",
        pd.len(),
        if pd.is_empty() { String::new() } else { format!(" ({})", pd.join(", ")) }
    ));
    let of = |k: HwAudit| a.hw.iter().filter(|(x, _)| *x == k).map(|(_, m)| m.as_str()).collect::<BTreeSet<_>>();
    let (overrides, intrinsics, vm) = (of(HwAudit::Override), of(HwAudit::Intrinsic), of(HwAudit::VmBoundary));
    lines.push(format!(
        "[raw-audit] raw_expr={} raw_stmt={} non_native_overrides={} intrinsics={} vm_boundary_methods={}",
        raw_audit::count(RawKind::Expr),
        raw_audit::count(RawKind::Stmt),
        overrides.len(),
        intrinsics.len(),
        vm.len()
    ));
    if !overrides.is_empty() {
        lines.push(format!("[override-audit] {}", overrides.into_iter().collect::<Vec<_>>().join(" ")));
    }
    if !vm.is_empty() {
        lines.push(format!("[vm-boundary-audit] {}", vm.into_iter().collect::<Vec<_>>().join(" ")));
    }
    lines.extend(a.module.iter().cloned());
    lines
}

/// `--raw-sites FILE`：位点剖面按次数降序追加写入（`{n}\t{kind}\t{site}`）
pub fn append_raw_sites(path: &Path) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    for (n, kind, site) in raw_audit::sites() {
        writeln!(f, "{n}\t{}\t{site}", kind.as_str())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readability_counts_only_generated_files() {
        let texts = ["rava_macros::java_class! { x.borrow(); Rc::new(1); Object::from_any(v) }", "x.borrow(); x.borrow();"];
        let c = readability_counts(texts);
        assert_eq!(c, [("from_any", 1), ("downcast", 0), ("downcast_ref", 0), ("rc_new", 1), ("borrow", 1)]);
    }

    #[test]
    fn lines_follow_python_order() {
        let body = BodyAudit::default();
        let fb = FallbackAudit::new(false);
        let pd = vec!["p/Option".to_string()];
        let hw = vec![(HwAudit::Override, "p/A.m:()V".to_string())];
        let rd = readability_counts([]);
        let lines = audit_lines(&AuditInputs {
            body: &body,
            hw: &hw,
            fallback: &fb,
            readability: &rd,
            prelude_disambiguated: &pd,
            module: vec!["[module-audit] modules=0 out_of_reads=0".to_string()],
            debug: false,
        });
        let heads: Vec<&str> = lines.iter().map(|l| l.split(' ').next().unwrap()).collect();
        assert_eq!(
            heads,
            [
                "[cfg-audit]",
                "[readability-audit]",
                "[equiv-audit]",
                "[fallback-audit]",
                "[shortname-audit]",
                "[raw-audit]",
                "[override-audit]",
                "[module-audit]"
            ]
        );
        assert_eq!(lines[4], "[shortname-audit] prelude-disambig=1 (p/Option)");
        assert!(lines[5].starts_with("[raw-audit] raw_expr=") && lines[5].ends_with("non_native_overrides=1 intrinsics=0 vm_boundary_methods=0"));
    }
}
