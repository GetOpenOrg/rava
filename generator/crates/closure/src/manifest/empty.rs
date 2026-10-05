//! `[facts.empty_collections]`：空的不可修改集合。
//!
//! `factories` 的返回值按 JDK 规范是不含元素、不可改写的集合（分析器给结果打 `Obj::Empty` 标签）；
//! `queries` 给出接收者带该标签时查询方法（`名:描述符`，与声明类无关——标签只来自工厂）的结果：
//! 布尔 / 整数 / `"null"`。

use std::collections::{HashMap, HashSet};

use super::Fact;

#[derive(Debug, Default)]
pub struct EmptyCollections {
    factories: HashSet<String>,
    queries: HashMap<String, Fact>,
}

impl EmptyCollections {
    pub fn from_toml(sec: Option<&toml::Value>) -> Result<Self, String> {
        let mut out = EmptyCollections::default();
        let Some(sec) = sec.and_then(|v| v.as_table()) else { return Ok(out) };
        let err = |k: &str, what: &str| format!("vm_intrinsics.toml [facts.empty_collections]：{k} {what}");
        for f in sec.get("factories").and_then(|v| v.as_array()).into_iter().flatten() {
            let s = f.as_str().ok_or_else(|| err("factories", "须为字符串数组"))?;
            if !s.ends_with(';') {
                return Err(err(s, "须返回引用"));
            }
            out.factories.insert(s.to_string());
        }
        for (k, v) in sec.get("queries").and_then(|v| v.as_table()).into_iter().flatten() {
            if k.contains('.') || !k.contains(':') {
                return Err(err(k, "须写成 名:描述符"));
            }
            let f = match v {
                toml::Value::Boolean(b) => Fact::Int(*b as i32),
                toml::Value::Integer(i) => Fact::Int(*i as i32),
                toml::Value::String(s) if s == "null" => Fact::Null,
                _ => return Err(err(k, "的值须为 null / 整数 / 布尔")),
            };
            out.queries.insert(k.clone(), f);
        }
        Ok(out)
    }

    /// 成员（`类.方法:描述符`）返回空的不可修改集合
    pub fn is_factory(&self, member: &str) -> bool {
        self.factories.contains(member)
    }

    /// 接收者为空集合时调用 `名:描述符` 的结果
    pub fn query(&self, name: &str, desc: &str) -> Option<&Fact> {
        if self.queries.is_empty() {
            return None;
        }
        self.queries.get(&format!("{name}:{desc}"))
    }
}
