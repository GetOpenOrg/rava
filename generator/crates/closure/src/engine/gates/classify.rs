//! 门的处理类别：只看分析器自身事实与 `runtime/java_runtime/` 清单（`[gates]` 提示、`[vm_boundary]`），
//! 不出现任何类名判断。规则按证据强弱排序，第一条成立的定类别，其余成立的事实一并列为旁证：
//!
//! 1. 清单 `[gates]` 提示（门所在类，其次调用目标类）——逐类判定只在清单里；
//! 2. VM 驱动：门所在类是 VM 边界类，或首达链根为 VM 规则 / 链上经 VM 钩子；
//! 3. 构建期可求值：首达链根为 JCA / locale / 注解种子，下游经服务 / 资源束查找到达，或门方法读系统属性；
//! 4. 精度缺口：调用点派发目标数达枢纽阈值、接 open 接收者枢纽，或下游经反射建模到达；
//! 5. 弱事实：门在类初始化链上 → 构建期可求值；
//! 6. 缺省：精度缺口（按 ③ 查精度）。

use std::collections::BTreeMap;

use crate::manifest::GateCategory;

/// 首达链根中属于构建期种子的种类（`Via::root` 的 kind）
const BUILD_TIME_ROOTS: &[&str] = &["jca", "jca-provider", "locale", "annotation", "annotation-class", "annotation-enum"];
/// 下游经服务 / 资源束查找到达的溯源种类
const LOOKUP_KINDS: &[&str] = &["service-provider", "service-catalog", "bundle"];
/// 下游经反射建模到达的溯源种类
const REFLECT_KINDS: &[&str] = &["reflect", "reflect-newarray", "member", "method-handle", "field-name"];
/// VM 驱动的溯源种类（根 / 链上步）
const VM_KINDS: &[&str] = &["vm-rule", "vm-hook"];

/// 分类所需的事实（由 `tree.rs` 从引擎与清单收集）
#[derive(Debug, Default, Clone)]
pub struct Facts {
    /// 清单 `[gates]` 对门所在类的提示：(类别, 命中条目)
    pub hint_owner: Option<(GateCategory, String)>,
    /// 清单 `[gates]` 对调用目标类的提示（调用点门）
    pub hint_target: Option<(GateCategory, String)>,
    /// 门所在类是 VM 边界类（closure.toml `[vm_boundary]`）
    pub vm_boundary: bool,
    /// 自门向根的首达链溯源种类（最后一项为根的种类）
    pub chain_kinds: Vec<&'static str>,
    /// 经本门首达的下游节点按溯源种类计数
    pub child_kinds: BTreeMap<&'static str, usize>,
    /// 调用点派发目标数（按成员合并克隆；方法体门为 0）
    pub fanout: usize,
    /// 调用点接入 open 接收者枢纽
    pub open_hub: bool,
    /// 门方法体内有系统属性读取（清单读取锚点 / 摘要形态）
    pub reads_sysprop: bool,
    /// 门方法是类初始化，或首达链近端（3 步内）经类初始化
    pub in_clinit: bool,
    /// 枢纽阈值（`HUB_MIN`）
    pub hub_min: usize,
}

/// (类别, 证据)：证据首条为定类别的事实
pub fn classify(f: &Facts) -> (GateCategory, Vec<String>) {
    let mut found: Vec<(GateCategory, String)> = Vec::new();
    if let Some((c, e)) = &f.hint_owner {
        found.push((*c, format!("清单 [gates] {} 条目 `{e}`（门所在类）", c.key())));
    }
    if let Some((c, e)) = &f.hint_target {
        found.push((*c, format!("清单 [gates] {} 条目 `{e}`（调用目标类）", c.key())));
    }
    if f.vm_boundary {
        found.push((GateCategory::VmDriven, "门所在类是 VM 边界类（closure.toml [vm_boundary]）".into()));
    }
    if let Some(k) = f.chain_kinds.iter().find(|k| VM_KINDS.contains(k)) {
        found.push((GateCategory::VmDriven, format!("首达链经 VM 驱动入口（{k}）")));
    }
    if let Some(root) = f.chain_kinds.last().filter(|k| BUILD_TIME_ROOTS.contains(k)) {
        found.push((GateCategory::BuildTime, format!("首达链根为构建期种子（{root}）")));
    }
    let lookups: usize = LOOKUP_KINDS.iter().filter_map(|k| f.child_kinds.get(k)).sum();
    if lookups > 0 {
        found.push((GateCategory::BuildTime, format!("下游 {lookups} 个节点经服务 / 资源束查找首达")));
    }
    if f.reads_sysprop {
        found.push((GateCategory::BuildTime, "门方法读系统属性（构建期定值）".into()));
    }
    if f.fanout >= f.hub_min.max(1) {
        found.push((GateCategory::Precision, format!("调用点派发目标 {} 个（≥ 枢纽阈值 {}）", f.fanout, f.hub_min)));
    }
    if f.open_hub {
        found.push((GateCategory::Precision, "调用点接 open 接收者枢纽".into()));
    }
    let reflect: usize = REFLECT_KINDS.iter().filter_map(|k| f.child_kinds.get(k)).sum();
    if reflect > 0 {
        found.push((GateCategory::Precision, format!("下游 {reflect} 个节点经反射建模首达")));
    }
    if f.in_clinit {
        found.push((GateCategory::BuildTime, "门在类初始化链上（弱：初始化结果可构建期求值）".into()));
    }
    if found.is_empty() {
        return (GateCategory::Precision, vec!["无更强事实，按 ③ 查精度".into()]);
    }
    (found[0].0, found.into_iter().map(|(_, e)| e).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_in_order() {
        let base = Facts { hub_min: 8, ..Default::default() };
        assert_eq!(classify(&base).0, GateCategory::Precision);
        let fan = Facts { fanout: 300, ..base.clone() };
        let (c, ev) = classify(&fan);
        assert_eq!(c, GateCategory::Precision);
        assert!(ev[0].contains("派发目标 300"));
        // 清单提示压过分析器事实
        let hinted = Facts { hint_target: Some((GateCategory::RuntimeModel, "p/".into())), ..fan.clone() };
        assert_eq!(classify(&hinted).0, GateCategory::RuntimeModel);
        // 构建期事实压过派发扇出；VM 驱动压过构建期
        let sp = Facts { reads_sysprop: true, ..fan.clone() };
        assert_eq!(classify(&sp).0, GateCategory::BuildTime);
        let vm = Facts { chain_kinds: vec!["invoke", "vm-rule"], ..sp.clone() };
        assert_eq!(classify(&vm).0, GateCategory::VmDriven);
        // 弱事实只在无强事实时定类别
        let cl = Facts { in_clinit: true, ..base.clone() };
        assert_eq!(classify(&cl).0, GateCategory::BuildTime);
        let cl_fan = Facts { in_clinit: true, ..fan };
        assert_eq!(classify(&cl_fan).0, GateCategory::Precision);
        let mut ck = base;
        ck.child_kinds.insert("service-provider", 3);
        assert_eq!(classify(&ck).0, GateCategory::BuildTime);
    }
}
