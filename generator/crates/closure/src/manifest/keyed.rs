//! `[facts.keyed_lookups]` 按键取对象的查找入口：返回值的「键」等于调用点实参 `key` 的值。
//!
//! 返回类型即键类；键类对象的键由其构造器的某个 String 形参给出（`ctors`：键类构造器 `名:描述符` → 形参序号，
//! 按描述符 0 起、不含接收者）。键类的子类经构造器链把自身构造器的形参（或常量）传给键类构造器，分析器按字节码追溯，
//! 不必登记。`fold_case` = 键按不区分大小写比较。分析器据此只让键可能匹配的对象类型流出调用点（见 `engine/keyed.rs`）。
//!
//! 另两种键来源（均为清单事实，分析器不含类名 / 包名）：
//! - `class_pattern`：键类子类的键由其类名按命名约定给出（`前缀{}后缀`，内部名；`{}` 段的 `/` 换成 `.` 即键），
//!   用于键类对象无构造器键形参、而查找入口按约定拼类名加载的情形；不符合约定的子类仍按 `ctors` 追溯（追溯不到即任意）；
//! - `scheme_sites`：所列方法（`类.方法:描述符`）内的查找调用点，键取该方法某个 String 形参的 URL scheme
//!   （按 `java.net.URL` 的解析规则，见 `engine/keyed_scheme.rs`）而不取键实参；无 scheme 的实参不经该调用点。

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
    /// 类名命名约定（前缀, 后缀）：键类子类名为 `前缀 + 段 + 后缀` 时键为段（`/` 换成 `.`）
    pub class_pattern: Option<(String, String)>,
    /// 方法（`类.方法:描述符`）→ String 形参序号（按描述符，0 起，不含接收者）：该方法内查找调用点的键取此形参的 URL scheme
    pub scheme_sites: HashMap<String, usize>,
}

impl KeyedLookup {
    /// 类（内部名）按命名约定的键：None = 无约定或不符合约定
    pub fn pattern_key(&self, cls: &str) -> Option<String> {
        let (pre, suf) = self.class_pattern.as_ref()?;
        let mid = cls.strip_prefix(pre.as_str())?.strip_suffix(suf.as_str())?;
        (!mid.is_empty()).then(|| mid.replace('/', "."))
    }
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
            let class_pattern = match e.get("class_pattern") {
                None => None,
                Some(x) => {
                    let p = x.as_str().ok_or_else(|| err("class_pattern 须为字符串"))?;
                    let (pre, suf) = p.split_once("{}").ok_or_else(|| err("class_pattern 须含一个 `{}`"))?;
                    if suf.contains("{}") || pre.contains('.') || suf.contains('.') {
                        return Err(err("class_pattern 须为内部名形式、恰含一个 `{}`"));
                    }
                    Some((pre.to_string(), suf.to_string()))
                }
            };
            let ctors_t = match e.get("ctors") {
                Some(x) => x.as_table().ok_or_else(|| err("ctors 须为表"))?.clone(),
                None if class_pattern.is_some() => toml::value::Table::new(),
                None => return Err(err("缺 ctors（键类构造器 → 键形参序号）")),
            };
            let mut scheme_sites = HashMap::new();
            if let Some(x) = e.get("scheme_sites") {
                for (mk, j) in x.as_table().ok_or_else(|| err("scheme_sites 须为表"))? {
                    let ok = mk.split_once(':').is_some_and(|(h, d)| h.contains('.') && d.starts_with('('));
                    if !ok {
                        return Err(err(&format!("scheme_sites 的键须为 `类.方法:描述符`：{mk}")));
                    }
                    scheme_sites.insert(mk.clone(), idx(j).ok_or_else(|| err(&format!("scheme_sites.{mk} 须为形参序号")))?);
                }
            }
            let mut ctors = HashMap::new();
            for (c, j) in &ctors_t {
                if !c.starts_with("<init>:(") {
                    return Err(err(&format!("ctors 的键须为 `<init>:描述符`：{c}")));
                }
                ctors.insert(c.clone(), idx(j).ok_or_else(|| err(&format!("ctors.{c} 须为形参序号")))?);
            }
            let fold_case = e.get("fold_case").and_then(|x| x.as_bool()).unwrap_or(false);
            out.insert(k.clone(), KeyedLookup { key, ctors, fold_case, class_pattern, scheme_sites });
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
        assert_eq!(e.pattern_key("a/x/Handler"), None);
    }

    #[test]
    fn parses_class_pattern_and_scheme_sites() {
        let s = sec("\"a/U.get:(La/S;)La/H;\" = { key = 0, class_pattern = \"p/q/{}/Handler\", scheme_sites = { \"a/U.<init>:(La/U;La/S;)V\" = 1 } }\n");
        let k = KeyedLookups::from_toml(Some(&s)).unwrap();
        let e = k.get("a/U.get:(La/S;)La/H;").unwrap();
        assert!(e.ctors.is_empty());
        assert_eq!(e.scheme_sites.get("a/U.<init>:(La/U;La/S;)V"), Some(&1));
        assert_eq!(e.pattern_key("p/q/jar/Handler").as_deref(), Some("jar"));
        // 段内的 `/` 换成 `.`（协议名可含 `.`，按 `前缀 + 协议 + 后缀` 拼出的类名在包层级里展开）
        assert_eq!(e.pattern_key("p/q/a/b/Handler").as_deref(), Some("a.b"));
        assert_eq!(e.pattern_key("p/q//Handler"), None);
        assert_eq!(e.pattern_key("p/q/jar/Other"), None);
        assert_eq!(e.pattern_key("x/Handler"), None);
    }

    #[test]
    fn rejects_malformed() {
        for bad in [
            "\"bad\" = { key = 0, ctors = {} }\n",
            "\"a/P.get:(La/S;)La/V;\" = { ctors = {} }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0 }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0, ctors = { \"m:()V\" = 0 } }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = -1, ctors = {} }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0, class_pattern = \"p/Handler\" }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0, class_pattern = \"p.{}.Handler\" }\n",
            "\"a/P.get:(La/S;)La/V;\" = { key = 0, class_pattern = \"p/{}/H\", scheme_sites = { \"bad\" = 1 } }\n",
        ] {
            assert!(KeyedLookups::from_toml(Some(&sec(bad))).is_err(), "{bad}");
        }
    }
}
