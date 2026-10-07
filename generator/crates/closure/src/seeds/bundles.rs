//! 按名装载的资源束（seeds.toml `[bundles]`）：`lookups` 成员按基名装载资源束（束类经 `Class.forName` +
//! `newInstance` 反射构造，`.properties` 束经模块资源查询读入），没有静态调用边。
//!
//! 基名由调用点实参求值（常量 / 构造点键值）；推不出时回退到调用链上的束形字面量。基名按入选 locale
//! 的父链展开候选：束类 `B` / `B_<后缀>`，属性文件 `B.properties` / `B_<后缀>.properties`。
//! 引擎侧补种见 `engine/bundles.rs`。

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BundleCfg {
    /// 资源束根类（束类候选须是其子类）
    pub root: String,
    /// 按名装载入口 `类.方法:描述符` → 基名形参序号（不含接收者，0 起）
    pub lookups: HashMap<String, usize>,
}

impl BundleCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let Some(sec) = sec else { return Self::default() };
        let lookups = sec
            .get("lookups")
            .and_then(|v| v.as_table())
            .map(|t| t.iter().filter_map(|(k, v)| Some((k.clone(), usize::try_from(v.as_integer()?).ok()?))).collect())
            .unwrap_or_default();
        let root = sec.get("root").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        BundleCfg { root, lookups }
    }
}

/// 束形名：两段及以上、每段为 Java 标识符的点分名（`ResourceBundle` 基名即类的二进制名）
pub fn bundle_shaped(s: &str) -> bool {
    let mut n = 0;
    for seg in s.split('.') {
        let mut cs = seg.chars();
        let Some(f) = cs.next() else { return false };
        if !(f.is_ascii_alphabetic() || f == '_' || f == '$') || !cs.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
            return false;
        }
        n += 1;
    }
    n >= 2
}

/// 基名的候选：(束类内部名, 属性文件路径)，根束在前、后缀按给定顺序
pub fn candidates(base: &str, suffixes: &[String]) -> (Vec<String>, Vec<String>) {
    let b = base.replace('.', "/");
    let mut names = vec![b.clone()];
    names.extend(suffixes.iter().map(|s| format!("{b}_{s}")));
    let props = names.iter().map(|n| format!("{n}.properties")).collect();
    (names, props)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_shape_is_dotted_identifiers() {
        assert!(bundle_shaped("p.q.Messages"));
        assert!(bundle_shaped("a.b$C"));
        assert!(!bundle_shaped("Messages"));
        assert!(!bundle_shaped("p/q/Messages"));
        assert!(!bundle_shaped("p..q"));
        assert!(!bundle_shaped("x.properties.1a"));
        assert!(!bundle_shaped("a b.c"));
        assert!(!bundle_shaped(""));
    }

    #[test]
    fn candidates_expand_root_then_suffixes() {
        let (c, p) = candidates("p.q.Msg", &["en_US".into(), "en".into()]);
        assert_eq!(c, vec!["p/q/Msg", "p/q/Msg_en_US", "p/q/Msg_en"]);
        assert_eq!(p, vec!["p/q/Msg.properties", "p/q/Msg_en_US.properties", "p/q/Msg_en.properties"]);
    }
}
