//! 库专属运行时清单（V12 §3.6：`runtime/lib_runtime/<模块名>/`）。
//!
//! 与 `runtime/java_runtime/` 的三份清单共用解析口径（缺省文件容忍）；每个模块目录的
//! `manifest.toml` 以 `versions = "[a,b)"` 给出版本区间（缺省 = 全版本），`seeds.toml` /
//! `closure.toml` 只写本库的补种与边界。**命中多个区间报错**（V12 §3.6）；模块名不含版本，
//! 库升级不改目录名。清单摘要只进该库的生成输入（V12-3 的 K(c)），不波及 JDK 侧。

use std::path::{Path, PathBuf};

/// 一个库模块的 lib_runtime 目录
#[derive(Debug, Clone)]
pub struct LibRuntime {
    /// 模块名（目录名；与模块图同名）
    pub module: String,
    /// 版本区间 `[lo, hi)`（下含上不含；空串 = 无下界，None 上界 = 无上界；None = 全版本）
    bounds: Option<(String, Option<String>)>,
    pub dir: PathBuf,
}

/// 版本比较：按 `.` 分段，数值段按数值比，其余按字典序（段数不足者补 0——`4` < `4.1`）
fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    let num = |s: &str| s.parse::<u64>().ok();
    let mut ai = a.split('.');
    let mut bi = b.split('.');
    loop {
        match (ai.next(), bi.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (x, y) => {
                let (x, y) = (x.unwrap_or("0"), y.unwrap_or("0"));
                let ord = match (num(x), num(y)) {
                    (Some(m), Some(n)) => m.cmp(&n),
                    _ => x.cmp(y),
                };
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
        }
    }
}

/// 解析 `"[a,b)"`（下含上不含；开界省略：`"[a,)"` / `"[,b)"`）；非法格式报错
fn parse_interval(s: &str) -> Result<(String, Option<String>), String> {
    let s = s.trim();
    let inner = s
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(')'))
        .ok_or_else(|| format!("versions 区间须形如 \"[a,b)\"（下含上不含，开界省略）：{s}"))?;
    let (lo, hi) = inner.split_once(',').ok_or_else(|| format!("versions 区间缺逗号：{s}"))?;
    let lo = lo.trim().to_string();
    let hi = hi.trim().to_string();
    if lo.is_empty() && hi.is_empty() {
        return Err(format!("versions 区间两侧都空（全版本应缺省 versions 字段）：{s}"));
    }
    Ok((lo, if hi.is_empty() { None } else { Some(hi) }))
}

impl LibRuntime {
    /// 扫描 `runtime/lib_runtime/`：子目录 = 模块名；`manifest.toml` 坏语法报错
    pub fn scan(root: &Path) -> Result<Vec<LibRuntime>, String> {
        let mut out = Vec::new();
        let rd = match std::fs::read_dir(root) {
            Ok(rd) => rd,
            Err(_) => return Ok(out),
        };
        for e in rd.flatten() {
            if !e.path().is_dir() {
                continue;
            }
            let module = e.file_name().into_string().unwrap_or_default();
            if module.is_empty() || module.starts_with('.') {
                continue;
            }
            let text = std::fs::read_to_string(e.path().join("manifest.toml")).unwrap_or_default();
            let t: toml::Table = if text.trim().is_empty() {
                Default::default()
            } else {
                text.parse().map_err(|err| format!("lib_runtime/{module}/manifest.toml：{err}"))?
            };
            // versions：单区间字符串或区间数组（数组时同目录展开为多条——命中多个才可能报错）
            let at = format!("lib_runtime/{module}/manifest.toml");
            match t.get("versions") {
                None => out.push(LibRuntime { module, bounds: None, dir: e.path() }),
                Some(toml::Value::String(v)) => {
                    let bounds = parse_interval(v).map_err(|err| format!("{at}：{err}"))?;
                    out.push(LibRuntime { module, bounds: Some(bounds), dir: e.path() });
                }
                Some(toml::Value::Array(vs)) => {
                    if vs.is_empty() {
                        return Err(format!("{at}：versions 数组为空"));
                    }
                    for v in vs {
                        let v = v.as_str().ok_or_else(|| format!("{at}：versions 数组项须为字符串"))?;
                        let bounds = parse_interval(v).map_err(|err| format!("{at}：{err}"))?;
                        out.push(LibRuntime { module: module.clone(), bounds: Some(bounds), dir: e.path() });
                    }
                }
                Some(_) => return Err(format!("{at}：versions 须为区间字符串或字符串数组")),
            }
        }
        out.sort_by(|a, b| a.module.cmp(&b.module));
        Ok(out)
    }

    /// 某版本是否落在本库的区间内（无区间 = 全版本）
    pub fn matches(&self, version: &str) -> bool {
        let Some((lo, hi)) = &self.bounds else { return true };
        if !lo.is_empty() && cmp_version(version, lo) == std::cmp::Ordering::Less {
            return false;
        }
        if let Some(hi) = hi {
            if hi.is_empty() {
                return true; // 上界开放
            }
            if cmp_version(version, hi) != std::cmp::Ordering::Less {
                return false;
            }
        }
        true
    }
}

/// 选出某模块某版本的清单目录：命中多个区间报错（V12 §3.6），未命中 None
pub fn select<'a>(all: &'a [LibRuntime], module: &str, version: &str) -> Result<Option<&'a LibRuntime>, String> {
    let hits: Vec<&LibRuntime> = all.iter().filter(|l| l.module == module && l.matches(version)).collect();
    match hits.len() {
        0 => Ok(None),
        1 => Ok(Some(hits[0])),
        _ => Err(format!(
            "模块 {module} 版本 {version} 命中多个 lib_runtime 区间：{}",
            hits.iter().map(|h| h.dir.display().to_string()).collect::<Vec<_>>().join("、")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("rava_lib_runtime_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn interval_select_and_errors() {
        let d = dir("sel");
        let mk = |name: &str, versions: &str| {
            std::fs::create_dir_all(d.join(name)).unwrap();
            std::fs::write(d.join(name).join("manifest.toml"), format!("versions = \"{versions}\"\n")).unwrap();
        };
        mk("junit", "[4.12,5)");
        // 同目录数组区间（重叠示例）：4.13.2 同时命中两段 → 报错
        let multi = d.join("junit4");
        std::fs::create_dir_all(&multi).unwrap();
        std::fs::write(multi.join("manifest.toml"), "versions = [\"[4,5)\", \"[4.13,5)\"]\n").unwrap();
        std::fs::create_dir_all(d.join("junit5")).unwrap(); // 无 manifest.toml = 全版本
        let all = LibRuntime::scan(&d).unwrap();
        assert_eq!(all.len(), 4, "junit×1 + junit4×2 + junit5×1");
        assert!(select(&all, "junit", "4.13.2").unwrap().is_some());
        assert!(select(&all, "junit", "5.0").unwrap().is_none(), "5.0 不在 [4.12,5)");
        assert!(select(&all, "junit4", "4.13.2").is_err(), "4.13.2 命中 junit4 的两段区间");
        assert!(select(&all, "junit4", "3.9").unwrap().is_none());
        assert!(select(&all, "junit4", "5.0").unwrap().is_none(), "5.0 超出两段");
        assert!(select(&all, "junit5", "1.0").unwrap().is_some(), "无区间目录全版本命中");
        assert!(select(&all, "nope", "1").unwrap().is_none());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn version_compare_and_bad_interval() {
        assert_eq!(cmp_version("4", "4.0"), std::cmp::Ordering::Equal);
        assert_eq!(cmp_version("4.13.2", "4.9"), std::cmp::Ordering::Greater, "数值段不按字典序");
        assert_eq!(cmp_version("4.13.2", "4.13.10"), std::cmp::Ordering::Less);
        assert!(parse_interval("[4,5)").is_ok());
        assert!(parse_interval("[4,)").is_ok());
        assert!(parse_interval("4,5").is_err());
        assert!(parse_interval("[,]").is_err());
        assert!(parse_interval("[4.13.2,4.14)").is_ok());
    }
}
