//! 手写扩展 trait（§9.9 K4-2a，`__XHw{N}`）的按名登记数据。
//!
//! 伴生文件的固有 impl 已改写为 `impl __XHw{N} for X`（`scripts/diag/k4_2a_rewrite_hw_impls.py`），
//! 声明侧由发射层镜像成声明 crate 里的扩展 trait（supertrait = `X__Methods`）。调用点（生成文件）
//! 解析这些方法需要 trait 在作用域：按 Java 成员名匹配到具体块（条目 rust 名 = java 名或
//! `名_<重载后缀>`；`__impl_` 前缀的虚方法体形态同样匹配），文件作用域登记 (binary, 块序号)，
//! 导入行 `use …::__XHw{N} as _;` 由 `imports::fill` 统一发射（与 `X__Methods` 同机制）。
//!
//! 数据由发射层从 `input::handwritten` 的扫描结果构建，[`crate::TyCtx`] 携带。

use std::collections::{BTreeMap, BTreeSet};

/// binary → 各块的条目名集（块序 = Vec 下标；序号 1 起，名字不带后缀）
#[derive(Debug, Default)]
pub struct HwNames {
    map: BTreeMap<String, Vec<BTreeSet<String>>>,
}

fn matches(rust: &str, member: &str) -> bool {
    let f = rust.strip_prefix("__impl_").unwrap_or(rust);
    f == member || f.strip_prefix(member).is_some_and(|r| r.starts_with('_'))
}

impl HwNames {
    pub fn build(entries: impl IntoIterator<Item = (String, Vec<BTreeSet<String>>)>) -> HwNames {
        HwNames { map: entries.into_iter().collect() }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Java 成员名命中的块序号（升序）。`<init>` 按 rust 构造器名 `new` / `new_*` 匹配
    pub fn ordinals_matching(&self, binary: &str, member: &str) -> Vec<u32> {
        let Some(blocks) = self.map.get(binary) else { return Vec::new() };
        let name = if member == "<init>" { "new" } else { member };
        blocks
            .iter()
            .enumerate()
            .filter(|(_, names)| names.iter().any(|n| matches(n, name)))
            .map(|(i, _)| i as u32 + 1)
            .collect()
    }

    /// 该类全部块的序号（升序）
    pub fn ordinals(&self, binary: &str) -> Vec<u32> {
        let n = self.map.get(binary).map_or(0, Vec::len);
        (1..=n as u32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> HwNames {
        HwNames::build([(
            "java/lang/String".to_string(),
            vec![
                ["of", "from_owned", "units"].into_iter().map(str::to_string).collect(),
                ["intern", "__impl_fetch"].into_iter().map(str::to_string).collect(),
            ],
        )])
    }

    #[test]
    fn matches_by_java_name_and_overload_suffix() {
        let n = names();
        assert_eq!(n.ordinals_matching("java/lang/String", "of"), vec![1]);
        assert_eq!(n.ordinals_matching("java/lang/String", "intern"), vec![2]);
        // 重载后缀（`intern_ljava_lang_String`）与虚体前缀（`__impl_fetch`）都命中
        assert_eq!(n.ordinals_matching("java/lang/String", "intern_ljava_lang_Object"), vec![2]);
        assert_eq!(n.ordinals_matching("java/lang/String", "fetch"), vec![2]);
        assert!(n.ordinals_matching("java/lang/String", "nope").is_empty());
        assert!(n.ordinals_matching("java/lang/Other", "of").is_empty());
        assert_eq!(n.ordinals("java/lang/String"), vec![1, 2]);
    }

    #[test]
    fn ctor_matches_new() {
        let n = names();
        assert!(n.ordinals_matching("java/lang/String", "<init>").is_empty());
        let n2 = HwNames::build([("a/C".to_string(), vec![["new_i"].into_iter().map(str::to_string).collect()])]);
        assert_eq!(n2.ordinals_matching("a/C", "<init>"), vec![1]);
    }
}
