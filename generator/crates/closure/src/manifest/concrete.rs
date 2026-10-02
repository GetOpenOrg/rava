//! 具体求值（`vm_intrinsics.toml [concrete]`）：入口、native 白名单、内存缓存字段、VM 布局字段。
//!
//! 入口方法在调用点实参可枚举时按实参逐组具体执行（engine/concrete），轨迹上的方法入闭包、
//! 结果对象图物化为类型流；native 只认白名单（成员 → 操作名，操作的实现只按操作名分派）。

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ConcreteCfg {
    /// 入口方法（`类.方法:描述符`）
    pub entries: HashSet<String>,
    /// 手写承载方法的具体语义：成员 → 操作名（engine/concrete/natives.rs）
    pub natives: HashMap<String, String>,
    /// 内存缓存字段（`类.字段`）：首次求值写入、之后命中；轨迹按冷 / 热两次求值取并
    pub memo_fields: HashSet<String>,
    /// 发布后不再改写的类型（含子类型）：映像中这类对象的全部实例字段可读，经其字段取到的映像数组视为冻结。
    /// 依据是类的不可变契约（如正则模式及其节点图编译后只读），由清单逐类声明
    pub stable_types: Vec<String>,
    /// 隐式异常的类型：种类（`null` / `index` / `cast` / `arith` / `store` / `size`）→ 类
    pub implicit: HashMap<String, String>,
    /// VM 布局的字段（字符串字面量与类镜像由 VM 直接构造）：键为语义名（`string_value` / `string_coder` / `component_type`），值为 `类.字段`
    pub vm_fields: HashMap<String, String>,
}

fn strs(v: Option<&toml::Value>) -> Vec<String> {
    v.and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
}

fn table(v: Option<&toml::Value>, what: &str) -> Result<HashMap<String, String>, String> {
    let mut out = HashMap::new();
    for (k, v) in v.and_then(|v| v.as_table()).into_iter().flatten() {
        let Some(s) = v.as_str() else {
            return Err(format!("vm_intrinsics.toml [concrete.{what}]：{k} 的值须为字符串"));
        };
        out.insert(k.clone(), s.to_string());
    }
    Ok(out)
}

pub fn parse(t: Option<&toml::Value>) -> Result<ConcreteCfg, String> {
    let get = |k: &str| t.and_then(|t| t.get(k));
    Ok(ConcreteCfg {
        entries: strs(get("entries")).into_iter().collect(),
        natives: table(get("natives"), "natives")?,
        memo_fields: strs(get("memo_fields")).into_iter().collect(),
        stable_types: strs(get("stable_types")),
        vm_fields: table(get("vm_fields"), "vm_fields")?,
        implicit: table(get("implicit"), "implicit")?,
    })
}
