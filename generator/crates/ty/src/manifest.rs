//! 类型层读取的 runtime 清单（`runtime/java_runtime/*.txt`，库知识的唯一来源）。
//!
//! - `signature_erased_interfaces.txt`：签名解析直接擦为 `Object` 的接口；
//! - `overload_abbrev.txt`：重载后缀的类名缩写（`小写简单名 缩写`）。
//!
//! 格式：每行一条，
//! 首个非空白字符为 `#` 的行是注释，文件缺失视为空表。runtime 目录由调用方显式传入。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Manifest {
    pub erased_interfaces: BTreeSet<String>,
    pub overload_abbrev: BTreeMap<String, String>,
}

/// 清单行格式错误（映射清单缺值）
#[derive(Debug)]
pub struct ManifestError {
    pub file: String,
    pub line: String,
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: 映射条目缺值：{}", self.file, self.line)
    }
}

impl std::error::Error for ManifestError {}

impl Manifest {
    /// 从 `runtime/java_runtime` 目录读取
    pub fn load(runtime_dir: &Path) -> Result<Manifest, ManifestError> {
        let erased = read_list(runtime_dir, "signature_erased_interfaces.txt")
            .into_iter()
            .collect();
        let abbrev = read_map(runtime_dir, "overload_abbrev.txt")?;
        Ok(Manifest {
            erased_interfaces: erased,
            overload_abbrev: abbrev,
        })
    }
}

fn read_list(dir: &Path, name: &str) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(dir.join(name)) else {
        return Vec::new();
    };
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

fn read_map(dir: &Path, name: &str) -> Result<BTreeMap<String, String>, ManifestError> {
    let mut out = BTreeMap::new();
    for line in read_list(dir, name) {
        let mut it = line.splitn(2, char::is_whitespace);
        let key = it.next().unwrap_or_default();
        let Some(val) = it.next().map(str::trim).filter(|v| !v.is_empty()) else {
            return Err(ManifestError {
                file: name.to_string(),
                line,
            });
        };
        out.insert(key.to_string(), val.to_string());
    }
    Ok(out)
}
