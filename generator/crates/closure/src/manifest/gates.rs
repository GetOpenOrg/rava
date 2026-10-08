//! 门排名的处理类别提示（closure.toml `[gates]`）：`rava closure --gates` 给每个门标处理类别时，
//! 分析器自身事实（派发扇出、溯源根种类、属性读取等，见 `engine/gates/classify.rs`）之外的逐类判定只来自本段。
//!
//! 条目形态：以 `/` 结尾为包前缀（覆盖包内全部类，含子包）；否则为类 binary name（覆盖自身与其 `$` 嵌套类）。
//! 多条命中取最长条目。

use std::fmt;

/// 门的处理类别（闭包构成报告 §六「三问」）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateCategory {
    /// ③ 翻译 + 精度缺口：分析器收窄（派发 / 反射 / 上下文精度）
    Precision,
    /// ② 构建期可求值：引导映像求值 / 清单定值后整块不再可达
    BuildTime,
    /// ① 运行模型替换：机制在原生映像下不成立（类加载、jar、类路径服务查找）
    RuntimeModel,
    /// VM 驱动：由 VM 直接驱动的行为（GC 引用处理、信号等），手写承载
    VmDriven,
}

impl GateCategory {
    pub const ALL: [GateCategory; 4] = [GateCategory::Precision, GateCategory::BuildTime, GateCategory::RuntimeModel, GateCategory::VmDriven];

    /// 机读名（JSON / 清单键）
    pub fn key(self) -> &'static str {
        match self {
            GateCategory::Precision => "precision",
            GateCategory::BuildTime => "build_time",
            GateCategory::RuntimeModel => "runtime_model",
            GateCategory::VmDriven => "vm_driven",
        }
    }

    /// 报告用中文名
    pub fn label(self) -> &'static str {
        match self {
            GateCategory::Precision => "精度缺口",
            GateCategory::BuildTime => "构建期可求值",
            GateCategory::RuntimeModel => "运行模型替换",
            GateCategory::VmDriven => "VM 驱动",
        }
    }
}

impl fmt::Display for GateCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.key())
    }
}

#[derive(Debug, Default, Clone)]
pub struct GateHints {
    entries: Vec<(String, GateCategory)>,
}

impl GateHints {
    pub fn from_toml(t: Option<&toml::Value>) -> Result<GateHints, String> {
        let mut entries = Vec::new();
        let Some(t) = t else { return Ok(GateHints::default()) };
        let t = t.as_table().ok_or("closure.toml [gates] 须为表")?;
        for (k, v) in t {
            let cat = GateCategory::ALL
                .into_iter()
                .find(|c| c.key() == k)
                .ok_or_else(|| format!("closure.toml [gates]：未知类别 {k}（应为 precision / build_time / runtime_model / vm_driven）"))?;
            let arr = v.as_array().ok_or_else(|| format!("closure.toml [gates] {k} 须为字符串数组"))?;
            for x in arr {
                let s = x.as_str().ok_or_else(|| format!("closure.toml [gates] {k} 须为字符串数组"))?;
                if let Some((_, c)) = entries.iter().find(|(e, _)| e == s) {
                    return Err(format!("closure.toml [gates]：{s} 同时列在 {c} 与 {cat}"));
                }
                entries.push((s.to_string(), cat));
            }
        }
        Ok(GateHints { entries })
    }

    /// 类的提示类别与命中条目（最长条目优先）
    pub fn category_of(&self, cls: &str) -> Option<(GateCategory, &str)> {
        self.entries
            .iter()
            .filter(|(e, _)| covers(e, cls))
            .max_by_key(|(e, _)| e.len())
            .map(|(e, c)| (*c, e.as_str()))
    }
}

fn covers(entry: &str, cls: &str) -> bool {
    if entry.ends_with('/') {
        return cls.starts_with(entry);
    }
    cls == entry || cls.strip_prefix(entry).is_some_and(|r| r.starts_with('$'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_entry_wins() {
        let t: toml::Table = "build_time = [\"a/b/\", \"a/b/S\"]\nruntime_model = [\"a/b/S$Lazy\"]".parse().unwrap();
        let h = GateHints::from_toml(Some(&toml::Value::Table(t))).unwrap();
        assert_eq!(h.category_of("a/b/X").map(|x| x.0), Some(GateCategory::BuildTime));
        assert_eq!(h.category_of("a/b/S$Lazy$1").map(|x| x.0), Some(GateCategory::RuntimeModel));
        assert_eq!(h.category_of("a/b/S$Other"), Some((GateCategory::BuildTime, "a/b/S")));
        // 类条目不作字符串前缀
        assert_eq!(h.category_of("a/b/Sx").map(|x| x.1), Some("a/b/"));
        assert_eq!(h.category_of("a/c/X"), None);
        let bad: toml::Table = "fast = [\"x\"]".parse().unwrap();
        assert!(GateHints::from_toml(Some(&toml::Value::Table(bad))).is_err());
    }
}
