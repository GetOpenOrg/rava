//! 模块资源（jmod / jar 内的非类数据文件）由调用链上的字节码推导。
//!
//! JDK 读数据文件的形态是 `X.class.getResourceAsStream("<名>")` / `getClass().getResourceAsStream(..)`
//! 等，资源名为方法体里的 ldc 字符串常量（相对名如 `"x.dat"`、绝对名如 `"/p/q/y.data"`）。
//! 调用链上方法体的每个路径形字符串按
//! `Class.resolveName` 的规则解析（`/` 开头为绝对名，否则相对所在类的包；另按原样试一次，
//! 对应 `ClassLoader.getResource` 的绝对名形态），类路径上存在的非类文件即嵌入。
//! 只看调用链上的方法：资源随读取它的代码进出闭包，不在链上的资源不进二进制。

use std::collections::{BTreeMap, BTreeSet};

use resolve::classpath::ClassPath;

/// 路径形字符串：非空、含 `.`、只含路径字符、无 `..` 段、不是类文件
fn path_like(s: &str) -> bool {
    !s.is_empty()
        && s.contains('.')
        && !s.ends_with(".class")
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

/// 由 (所在类, ldc 字符串) 推导要嵌入的资源：(资源路径, 字节)，按路径排序
pub(crate) fn derive<'s>(cp: &ClassPath, strings: impl IntoIterator<Item = (&'s str, &'s str)>) -> Vec<(String, Vec<u8>)> {
    let names: BTreeSet<String> = strings
        .into_iter()
        .filter(|(_, s)| path_like(s))
        .flat_map(|(c, s)| candidates(c, s))
        .collect();
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
        assert!(!path_like("Name"));
        assert!(!path_like("a b.txt"));
        assert!(!path_like("../x.dat"));
        assert!(!path_like("p/q/A.class"));
        assert_eq!(candidates("p/q/Reader$1", "names.dat"), vec!["p/q/names.dat", "names.dat"]);
        assert_eq!(candidates("p/q/Table$1", "/p/q/table.data"), vec!["p/q/table.data"]);
        assert_eq!(candidates("Main", "x.dat"), vec!["x.dat"]);
    }
}
