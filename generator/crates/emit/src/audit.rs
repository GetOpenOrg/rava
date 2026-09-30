//! 转译审计行（← `scripts/main.py _python_codegen` 的审计汇总段）。
//!
//! Rust 路径可得的口径：
//! - `[cfg-audit]`：控制流审计（跳转消费 / try 区域 / instanceof 折叠 / 存根兜底）；
//! - `[equiv-audit]`：近似 / 条件等价发射点计数，只列非零项；
//! - `[raw-audit]`：手写审计三项（FS-H0：越界覆盖 / VM 内建 / VM 边界方法，按成员去重）；
//!   Python 行的 `raw_expr` / `raw_stmt`（Python IR 构造事件）与 `type_surgery_*` / `jdk_literals`
//!   （Python 源码静态卫生）是 Python 实现的度量，Rust 路径不输出；
//! - `[override-audit]` / `[vm-boundary-audit]`：非空时的逐成员明细。

use std::collections::BTreeSet;

use instr::Audit;

use crate::ctx::HwAudit;
use crate::method_bodies::BodyAudit;

/// 审计行（输出序与 Python 一致）
pub fn audit_lines(body: &BodyAudit, hw: &[(HwAudit, String)]) -> Vec<String> {
    let mut lines = vec![body.cfg.summary()];
    let equiv: Vec<String> = Audit::REPORT_ORDER
        .iter()
        .filter_map(|a| body.equiv.get(a).filter(|n| **n > 0).map(|n| format!("{}={n}", a.as_str())))
        .collect();
    lines.push(format!("[equiv-audit] {}", if equiv.is_empty() { "none".to_string() } else { equiv.join(" ") }));
    let of = |k: HwAudit| hw.iter().filter(|(x, _)| *x == k).map(|(_, m)| m.as_str()).collect::<BTreeSet<_>>();
    let (overrides, intrinsics, vm) = (of(HwAudit::Override), of(HwAudit::Intrinsic), of(HwAudit::VmBoundary));
    lines.push(format!(
        "[raw-audit] non_native_overrides={} intrinsics={} vm_boundary_methods={}",
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
    lines
}
