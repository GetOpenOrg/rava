//! `[facts.keyed_lookups]` 按键取对象的查找入口：返回值的「键」等于调用点实参 `key` 的值。
//!
//! 返回类型即键类；键类对象的键由其构造器的某个 String 形参给出（`ctors`：键类构造器 `名:描述符` → 形参序号，
//! 按描述符 0 起、不含接收者）。键类的子类经构造器链把自身构造器的形参（或常量）传给键类构造器，分析器按字节码追溯，
//! 不必登记。`fold_case` = 键按不区分大小写比较。分析器据此只让键可能匹配的对象类型流出调用点（见 `engine/keyed.rs`）。

use std::collections::HashMap;

/// 一个按键查找入口
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyedLookup {
    /// 键实参序号（按描述符，0 起，不含接收者）
    pub key: usize,
    /// 键类构造器（`名:描述符`）→ 键形参序号
    pub ctors: HashMap<String, usize>,
    /// 键不区分大小写比较
    pub fold_case: bool,
}

/// 按成员（`类.方法:描述符`）登记的按键查找入口
#[derive(Debug, Default, Clone)]
pub struct KeyedLookups(HashMap<String, KeyedLookup>);

impl KeyedLookups {
    pub fn from_toml(sec: Option<&toml::Value>) -> Result<Self, String> {
        let mut out = HashMap::new();
        let Some(t) = sec.and_then(|v| v.as_table()) else { return Ok(KeyedLookups(out)) };
        for (k, v) in t {
            let err = |what: &str| format!("vm_intrinsics.toml [facts.keyed_lookups] {k}：{what}");
            let (head, desc) = k.split_once(':').ok_or_else(|| err("键须为 `类.方法:描述符`"))?;
            if !head.contains('.') || !desc.starts_with('(') {
                return Err(err("键须为 `类.方法:描述符`"));
            }
            let e = v.as_table().ok_or_else(|| err("须为 { key = 序号, ctors = { ... } }"))?;
            let idx = |x: &toml::Value| x.as_integer().filter(|i| *i >= 0).map(|i| i as usize);
            let key = e.get("key").and_then(idx).ok_or_else(|| err("缺 key（键实参序号）"))?;
            let mut ctors = HashMap::new();
            for (c, j) in e.get("ctors").and_then(|x| x.as_table()).ok_or_else(|| err("缺 ctors（键类构造器 → 键形参序号）"))? {
                if !c.starts_with("<init>:(") {
                    return Err(err(&format!("ctors 的键须为 `<init>:描述符`：{c}")));
                }
                ctors.insert(c.clone(), idx(j).ok_or_else(|| err(&format!("ctors.{c} 须为形参序号")))?);
            }
            let fold_case = e.get("fold_case").and_then(|x| x.as_bool()).unwrap_or(false);
            out.insert(k.clone(), KeyedLookup { key, ctors, fold_case });
        }
        Ok(KeyedLookups(out))
    }

    /// 成员（`类.方法:描述符`）登记的按键查找入口
    pub fn get(&self, member: &str) -> Option<&KeyedLookup> {
        self.0.get(member)
    }

    /// 全部登记成员（`类.方法:描述符`，有序）
    pub fn members(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.0.keys().map(|k| k.as_str()).collect();
        v.sort();
        v
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sec(s: &str) -> toml::Value {
        toml::Value::Table(s.parse().unwrap())
    }

    #[test]
    fn parses_entries() {
        let s = sec("\"a/P.get:(La/S;)La/V;\" = { key = 0, fold_case = true, ctors = { \"<init>:(La/S;)V\" = 0 } }\n");
        let k = KeyedLookups::from_toml(Some(&s)).unwrap();
        let e = k.get("a/P.get:(La/S;)La/V;").unwrap();
        assert_eq!(e.key, 0);
        assert!(e.fold_case);
        assert_eq!(e.ctors.get("<init>:(La/S;)V"), Some(&0));
        assert!(KeyedLookups::from_toml(None).unwrap().is_empty());
    }

    #[test]
    fn rejects_malformed() {
        for bad in [
            "\"bad\" = { key = 0, ctors = {} }\n",
            "\"a/P.get:(La/S;)La/V;\" = { ctors = {} }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0 }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0, ctors = { \"m:()V\" = 0 } }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = -1, ctors = {} }\n",
        ] {
            assert!(KeyedLookups::from_toml(Some(&sec(bad))).is_err(), "{bad}");
        }
    }
}
