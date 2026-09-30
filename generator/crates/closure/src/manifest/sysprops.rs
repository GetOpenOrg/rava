//! `[facts.system_properties]`：VM 初始系统属性表与属性读写锚点（分析器按此折叠属性读取）。
//!
//! 表即原生二进制启动时 `System.props` 的内容：`values` 是取值恒定的键，`dynamic` 是存在、但取值
//! 由宿主环境 / 语料 JDK 在启动期决定的键（不折叠）；两者之外的键启动时不存在（读取为 null——原生
//! 二进制无 `-D` 注入机制）。锚点给出系统属性表对象从哪里来（`holders`：静态字段 / 返回它的方法）、
//! 从哪里读（`readers`）、按键改写的入口（`writers`）、只读查询入口（`queries`：接收者为表对象时
//! 既不改写表、结果也不持有表的引用）；实参序号含接收者。

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// 读取入口：`receiver` = 接收者须为系统属性表对象；`key` / `default` = 键 / 缺省值的实参序号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropRead {
    pub receiver: bool,
    pub key: usize,
    pub default: Option<usize>,
}

/// 表中键的取值
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropValue<'a> {
    Const(&'a str),
    /// 存在，取值启动期才定
    Dynamic,
    /// 启动时不存在
    Absent,
}

#[derive(Debug, Default)]
pub struct SysProps {
    values: BTreeMap<String, String>,
    dynamic: BTreeSet<String>,
    holders: HashSet<String>,
    readers: HashMap<String, PropRead>,
    /// 接收者为系统属性表对象时只改写一个键的入口：成员 → 键的实参序号
    writers: HashMap<String, usize>,
    /// 只读查询入口（接收者为系统属性表对象时不算逃逸）
    queries: HashSet<String>,
}

fn idx(t: &toml::Table, k: &str) -> Option<usize> {
    t.get(k).and_then(|v| v.as_integer()).and_then(|i| usize::try_from(i).ok())
}

impl SysProps {
    pub fn from_toml(sec: Option<&toml::Value>) -> Result<Self, String> {
        let mut out = SysProps::default();
        let Some(sec) = sec.and_then(|v| v.as_table()) else { return Ok(out) };
        let err = |k: &str, what: &str| format!("vm_intrinsics.toml [facts.system_properties]：{k} {what}");
        for (k, v) in sec.get("values").and_then(|v| v.as_table()).into_iter().flatten() {
            let s = v.as_str().ok_or_else(|| err(k, "的值须为字符串"))?;
            out.values.insert(k.clone(), s.to_string());
        }
        let list = |key: &str| -> Vec<String> {
            sec.get(key).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
        };
        out.dynamic = list("dynamic").into_iter().collect();
        if let Some(k) = out.dynamic.iter().find(|k| out.values.contains_key(*k)) {
            return Err(err(k, "不能同时列在 values 与 dynamic"));
        }
        out.holders = list("holders").into_iter().collect();
        out.queries = list("queries").into_iter().collect();
        for (k, v) in sec.get("readers").and_then(|v| v.as_table()).into_iter().flatten() {
            let t = v.as_table().ok_or_else(|| err(k, "须为 { key = 序号, receiver = 布尔, default = 序号 }"))?;
            let key = idx(t, "key").ok_or_else(|| err(k, "缺 key"))?;
            let receiver = t.get("receiver").and_then(|v| v.as_bool()).unwrap_or(false);
            out.readers.insert(k.clone(), PropRead { receiver, key, default: idx(t, "default") });
        }
        for (k, v) in sec.get("writers").and_then(|v| v.as_table()).into_iter().flatten() {
            let key = v.as_table().and_then(|t| idx(t, "key")).ok_or_else(|| err(k, "须为 { key = 序号 }"))?;
            out.writers.insert(k.clone(), key);
        }
        Ok(out)
    }

    /// 启动时键 k 的取值
    pub fn lookup(&self, k: &str) -> PropValue<'_> {
        match self.values.get(k) {
            Some(v) => PropValue::Const(v),
            None if self.dynamic.contains(k) => PropValue::Dynamic,
            None => PropValue::Absent,
        }
    }

    /// 成员（字段 `类.名:描述符` / 方法）持有系统属性表对象
    pub fn is_holder(&self, member: &str) -> bool {
        self.holders.contains(member)
    }

    pub fn reader(&self, member: &str) -> Option<PropRead> {
        self.readers.get(member).copied()
    }

    pub fn writer(&self, member: &str) -> Option<usize> {
        self.writers.get(member).copied()
    }

    pub fn is_query(&self, member: &str) -> bool {
        self.queries.contains(member)
    }

    pub fn is_empty(&self) -> bool {
        self.holders.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Result<SysProps, String> {
        let v: toml::Value = toml::from_str(src).unwrap();
        SysProps::from_toml(v.get("s"))
    }

    #[test]
    fn table_and_anchors() {
        let p = parse(
            r#"
            [s]
            holders = ["a/B.props:Lx/P;"]
            queries = ["x/P.names:()Ljava/util/Set;"]
            dynamic = ["user.dir"]
            [s.values]
            "k.on" = "true"
            [s.readers]
            "x/P.get:(Ljava/lang/String;)Ljava/lang/String;" = { receiver = true, key = 1 }
            "x/Q.get:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;" = { key = 0, default = 1 }
            [s.writers]
            "x/P.set:(Ljava/lang/String;Ljava/lang/String;)V" = { key = 1 }
            "#,
        )
        .unwrap();
        assert_eq!(p.lookup("k.on"), PropValue::Const("true"));
        assert_eq!(p.lookup("user.dir"), PropValue::Dynamic);
        assert_eq!(p.lookup("nope"), PropValue::Absent);
        assert!(p.is_holder("a/B.props:Lx/P;") && !p.is_empty());
        assert_eq!(p.reader("x/P.get:(Ljava/lang/String;)Ljava/lang/String;"), Some(PropRead { receiver: true, key: 1, default: None }));
        assert_eq!(
            p.reader("x/Q.get:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;"),
            Some(PropRead { receiver: false, key: 0, default: Some(1) })
        );
        assert!(p.is_query("x/P.names:()Ljava/util/Set;") && !p.is_query("x/P.set:(Ljava/lang/String;Ljava/lang/String;)V"));
        assert_eq!(p.writer("x/P.set:(Ljava/lang/String;Ljava/lang/String;)V"), Some(1));
    }

    #[test]
    fn value_and_dynamic_conflict() {
        let e = parse("[s]\ndynamic = [\"a\"]\n[s.values]\na = \"1\"\n").unwrap_err();
        assert!(e.contains("a"));
        assert!(parse("").unwrap().is_empty());
    }
}
