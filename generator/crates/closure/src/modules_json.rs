//! closure.json / profile.json 的模块事实（T1 第 2 步 M1）：每个类条目的 `module` 字段与顶层 `modules` 段。
//!
//! - 类条目 `module`：所属模块名（[`resolve::ModuleGraph::module_of`]）；无名模块（用户类、无描述符档案）不写该字段；
//! - `modules`：类条目覆盖的模块，按 requires 拓扑序（上游在前、同层名序，即 VM 登记序），每项
//!   `{"name", "automatic", "classes", "upstream"}`；`upstream` 为该模块完整的 requires 传递闭包（名序），
//!   与本段模块集的交即按模块切 crate（M2）后的 crate 依赖上界。档案把各入口的行取并（[`ModuleRows`]）。

use std::collections::{BTreeMap, BTreeSet};

use resolve::ModuleGraph;
use serde_json::{json, Value};

/// 一个模块行（不含类数：类数由类条目重算）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleRow {
    pub automatic: bool,
    pub upstream: BTreeSet<String>,
}

/// 模块名 → 行
pub type ModuleRows = BTreeMap<String, ModuleRow>;

/// 给类条目（`{"name", ...}`）补 `module` 字段，返回 `modules` 段
pub fn annotate(g: &ModuleGraph<'_>, classes: &mut [Value]) -> Value {
    let mut rows = ModuleRows::new();
    for c in classes.iter_mut() {
        let Some(m) = c.get("name").and_then(Value::as_str).and_then(|n| g.module_of(n)).map(str::to_string) else { continue };
        if !rows.contains_key(&m) {
            let node = g.node(&m);
            let row = ModuleRow {
                automatic: node.is_some_and(|n| n.automatic),
                upstream: node.map(|n| n.upstream.clone()).unwrap_or_default(),
            };
            rows.insert(m.clone(), row);
        }
        c["module"] = json!(m);
    }
    rows_json(&rows, classes)
}

/// `modules` 段：只列类条目实际覆盖的模块（类数由类条目的 `module` 字段计）
pub fn rows_json(rows: &ModuleRows, classes: &[Value]) -> Value {
    let mut count: BTreeMap<&str, usize> = BTreeMap::new();
    for m in classes.iter().filter_map(|c| c.get("module").and_then(Value::as_str)) {
        *count.entry(m).or_default() += 1;
    }
    let deps: BTreeMap<String, BTreeSet<String>> =
        rows.iter().filter(|(m, _)| count.contains_key(m.as_str())).map(|(m, r)| (m.clone(), r.upstream.clone())).collect();
    let order = resolve::modules::topo_order(&deps);
    json!(order
        .iter()
        .map(|m| {
            let r = &rows[m];
            json!({"name": m, "automatic": r.automatic, "classes": count[m.as_str()], "upstream": r.upstream})
        })
        .collect::<Vec<_>>())
}

/// 把一份 closure 的 `modules` 段并入 `rows`；同名模块的行须一致（同一 JDK / 类路径）
pub fn merge_rows(rows: &mut ModuleRows, section: Option<&Value>, entry: &str) -> Result<(), String> {
    for v in section.and_then(Value::as_array).into_iter().flatten() {
        let name = v.get("name").and_then(Value::as_str).ok_or_else(|| format!("modules 条目缺 name：{v}"))?;
        let row = ModuleRow {
            automatic: v.get("automatic").and_then(Value::as_bool).unwrap_or(false),
            upstream: v.get("upstream").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(String::from).collect(),
        };
        match rows.get(name) {
            None => {
                rows.insert(name.to_string(), row);
            }
            Some(r) if *r != row => return Err(format!("模块 {name} 的 requires 闭包在入口 {entry} 与其他入口不一致（类路径不同）")),
            Some(_) => {}
        }
    }
    Ok(())
}
