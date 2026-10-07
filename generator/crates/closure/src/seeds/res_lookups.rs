//! 按名读取的资源（seeds.toml `[resource_lookups]`）：`类.方法:描述符` 按资源名读取模块 / 类路径资源，
//! 资源名可由拼接、字段等计算得出（不只是 ldc 字面量）。调用点的资源名由闭包分析器按名求值，
//! 类路径上存在的资源进输出事实 `named_resources`。引擎侧见 `engine/res_lookups.rs`。

use std::collections::{HashMap, HashSet};

/// 一个读取入口
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lookup {
    /// 资源名形参序号（不含接收者，0 起）
    pub arg: usize,
    /// 相对名按所在类的包解析（`Class.resolveName` 语义：`/` 开头为绝对名）
    pub relative: bool,
}

#[derive(Debug, Default)]
pub struct ResLookupCfg {
    /// 读取入口（按声明处的成员键）
    pub lookups: HashMap<String, Lookup>,
    /// 入口方法名（扫描时先按名筛，命中再解析声明处）
    pub names: HashSet<String>,
}

impl ResLookupCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let Some(t) = sec.and_then(|v| v.as_table()) else { return Self::default() };
        let lookups = t
            .iter()
            .filter_map(|(k, v)| {
                let arg = usize::try_from(v.get("arg")?.as_integer()?).ok()?;
                let relative = v.get("relative").and_then(|r| r.as_bool()).unwrap_or(false);
                Some((k.clone(), Lookup { arg, relative }))
            })
            .collect::<HashMap<_, _>>();
        let names = lookups.keys().filter_map(|k| Some(k.split_once(':')?.0.rsplit_once('.')?.1.to_string())).collect();
        ResLookupCfg { lookups, names }
    }
}

/// 资源名 s 在类 class 的调用点上的候选路径（relative：`Class.resolveName` 语义）
pub fn candidates(class: &str, s: &str, relative: bool) -> Vec<String> {
    if let Some(abs) = s.strip_prefix('/') {
        return if relative { vec![abs.to_string()] } else { vec![] };
    }
    match class.rsplit_once('/').filter(|_| relative) {
        Some((pkg, _)) => vec![format!("{pkg}/{s}")],
        None => vec![s.to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lookups() {
        let t: toml::Table = toml::from_str(
            r#"
            [resource_lookups]
            "p/M.get:(Ljava/lang/String;)V" = { arg = 0 }
            "p/C.get:(Ljava/lang/String;)V" = { arg = 1, relative = true }
            "#,
        )
        .unwrap();
        let c = ResLookupCfg::from_toml(t.get("resource_lookups"));
        assert_eq!(c.lookups["p/M.get:(Ljava/lang/String;)V"], Lookup { arg: 0, relative: false });
        assert_eq!(c.lookups["p/C.get:(Ljava/lang/String;)V"], Lookup { arg: 1, relative: true });
        assert!(c.names.contains("get") && c.names.len() == 1);
    }

    #[test]
    fn candidate_resolution() {
        assert_eq!(candidates("p/q/C", "x.dat", true), vec!["p/q/x.dat"]);
        assert_eq!(candidates("p/q/C", "/a/b.dat", true), vec!["a/b.dat"]);
        assert_eq!(candidates("p/q/C", "a/b.dat", false), vec!["a/b.dat"]);
        // 类加载器 / 模块的资源名不以 `/` 开头，`/` 开头的名字查不到
        assert!(candidates("p/q/C", "/a/b.dat", false).is_empty());
    }
}
