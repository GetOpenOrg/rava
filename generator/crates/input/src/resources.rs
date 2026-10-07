//! 模块资源（jmod / jar 内的非类数据文件）由调用链上的字节码推导。
//!
//! JDK 读数据文件的形态是 `X.class.getResourceAsStream("<名>")` / `getClass().getResourceAsStream(..)`
//! 等，资源名为方法体里的 ldc 字符串常量（相对名如 `"x.dat"`、绝对名如 `"/p/q/y.data"`）；
//! 资源名也可以无扩展名（`BreakIteratorResourceBundle` 以「所在类包名 + '/' + 信息束里的字符串」
//! 拼出 `WordBreakIteratorData` 之类的路径）。调用链上方法体的每个路径形字符串按
//! `Class.resolveName` 的规则解析（`/` 开头为绝对名，否则相对所在类的包；另按原样试一次，
//! 对应 `ClassLoader.getResource` 的绝对名形态），类路径上存在的非类文件即嵌入。
//! 资源名也可以由拼接得出（ICU `Norm2AllModesSingleton(name)` 以「目录前缀 + 名 + `.nrm`」拼出
//! `nfc.nrm`）：同一方法体里相邻的两个 ldc 常量为「`/` 结尾的目录前缀」与「`.` 开头的扩展名后缀」时
//! 记为拼接模板，中间的动态段取调用链上方法体里的单段字符串常量（拼接实参的常量本身也是调用链上
//! 某个方法的 ldc，如 `NFCSingleton.<clinit>` 的 `"nfc"`）。
//! 只看调用链上的方法：资源随读取它的代码进出闭包，不在链上的资源不进二进制。
//! 名字经计算得出的资源由闭包分析器按名求值后作为种子事实 `named_resources` 给出，在此一并嵌入：
//! 按名装载的资源束（`ResourceBundle.getBundle` 的 `.properties` 束，`closure/src/engine/bundles.rs`）与
//! 按名读取入口上拼接 / 字段得出的资源名（`closure/src/engine/res_lookups.rs`）。

use std::collections::{BTreeMap, BTreeSet};

use resolve::classpath::ClassPath;

/// 路径形字符串：非空、只含路径字符、无 `..` 段、不是类文件（是否为资源由类路径上是否存在决定）
fn path_like(s: &str) -> bool {
    !s.is_empty()
        && !s.ends_with(".class")
        && !s.ends_with('/')
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'.' | b'_' | b'-' | b'$'))
        && !s.split('/').any(|seg| seg == "..")
}

/// 资源名候选（`Class.resolveName` 语义 + 原样绝对名）
fn candidates(class: &str, s: &str) -> Vec<String> {
    if let Some(abs) = s.strip_prefix('/') {
        return vec![abs.to_string()];
    }
    match class.rsplit_once('/') {
        Some((pkg, _)) => vec![format!("{pkg}/{s}"), s.to_string()],
        None => vec![s.to_string()],
    }
}

/// 调用链上方法体的字符串常量：(所在类, 字符串) 与拼接模板 (所在类, 目录前缀, 扩展名后缀)
#[derive(Default)]
pub(crate) struct Strings<'s> {
    lits: BTreeSet<(&'s str, &'s str)>,
    templates: BTreeSet<(&'s str, &'s str, &'s str)>,
}

impl<'s> Strings<'s> {
    /// 登记一个方法体的 ldc 字符串常量（按指令序）
    pub(crate) fn add_method(&mut self, class: &'s str, lits: impl IntoIterator<Item = &'s str>) {
        let mut prev: Option<&'s str> = None;
        for s in lits {
            if let Some(p) = prev.filter(|p| dir_prefix(p) && ext_suffix(s)) {
                self.templates.insert((class, p, s));
            }
            self.lits.insert((class, s));
            prev = Some(s);
        }
    }
}

/// 拼接模板的目录前缀：`/` 结尾，去掉结尾 `/` 后为路径形
fn dir_prefix(s: &str) -> bool {
    s.len() > 1 && s.ends_with('/') && path_like(&s[..s.len() - 1])
}

/// 拼接模板的扩展名后缀：`.` 开头的单段路径形（如 `.nrm`）
fn ext_suffix(s: &str) -> bool {
    s.len() > 1 && s.starts_with('.') && !s.contains('/') && path_like(s)
}

/// 由调用链上的字符串常量推导要嵌入的资源，并入闭包分析器按名求出的资源（`named`）：
/// (资源路径, 字节)，按路径排序
pub(crate) fn derive(cp: &ClassPath, strings: &Strings<'_>, named: &BTreeSet<String>) -> Vec<(String, Vec<u8>)> {
    let mut names: BTreeSet<String> = named.clone();
    names.extend(strings
        .lits
        .iter()
        .filter(|(_, s)| path_like(s))
        .flat_map(|(c, s)| candidates(c, s)),
    );
    if !strings.templates.is_empty() {
        let segs: BTreeSet<&str> = strings.lits.iter().map(|(_, s)| *s).filter(|s| path_like(s) && !s.contains('/')).collect();
        for (c, p, x) in &strings.templates {
            for g in &segs {
                names.extend(candidates(c, &format!("{p}{g}{x}")));
            }
        }
    }
    let found: BTreeMap<String, Vec<u8>> = names.into_iter().filter_map(|p| cp.resource(&p).map(|b| (p, b))).collect();
    found.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_names_follow_resolve_name() {
        assert!(path_like("names.dat"));
        assert!(path_like("/p/q/table.data"));
        assert!(path_like("WordBreakIteratorData"));
        assert!(!path_like("a b.txt"));
        assert!(!path_like("../x.dat"));
        assert!(!path_like("p/q/A.class"));
        assert_eq!(candidates("p/q/Reader$1", "names.dat"), vec!["p/q/names.dat", "names.dat"]);
        assert_eq!(candidates("p/q/Table$1", "/p/q/table.data"), vec!["p/q/table.data"]);
        assert_eq!(candidates("Main", "x.dat"), vec!["x.dat"]);
    }

    #[test]
    fn concat_templates_pair_dir_prefix_with_extension() {
        let mut s = Strings::default();
        s.add_method("p/q/Single", ["/p/q/data/", ".nrm", "other"]);
        s.add_method("p/q/Holder", ["nfc"]);
        s.add_method("p/q/Plain", ["dir/", "x", ".nrm"]);
        assert_eq!(s.templates.iter().map(|t| (t.1, t.2)).collect::<Vec<_>>(), vec![("/p/q/data/", ".nrm")]);
        assert!(!dir_prefix("/") && !ext_suffix(".") && !ext_suffix(".a/b"));
    }
}
