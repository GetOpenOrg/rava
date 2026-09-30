//! Rust 类型短名消歧（`type_map.configure_short_names` / `short_cls` /
//! `_registry_short_index` 的移植）。Python 侧是模块级全局表，这里是由注册表
//! 显式构建的不可变上下文。
//!
//! Rust 类型名 = 类的简单名（`$` → `_`）。两类冲突源：
//! 1. 注册表内不同包同简单名：组内 binary 字典序最小者保留简单名，其余以完整
//!    binary（`/`、`$` → `_`）为名；
//! 2. Rust prelude 可见名（[`PRELUDE_CONFLICT_NAMES`]）：同名的类同样限定改名——
//!    `Object` / `String` 的本主除外（直映射即 prelude 名本身，同一实体）。

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use crate::consts;
use crate::registry::Registry;

/// 发射面以裸名引用的 prelude 名（`java_runtime::prelude` 再导出 + std prelude）
pub const PRELUDE_CONFLICT_NAMES: [&str; 18] = [
    "JArray", "JvmError", "Result", "Object", "ObjectVTable", "String", "MonitorGuard", "Rc", "__Shared", "RefCell",
    "Option", "Some", "None", "Ok", "Err", "Vec", "Clone", "Default",
];

/// prelude 名的本主（不参与任何冲突组）
const CANONICAL_OWNERS: [&str; 2] = [consts::STRING, consts::OBJECT];

#[derive(Debug, Clone, Default)]
pub struct ShortNames {
    qualified: BTreeMap<String, String>,
    prelude_disambiguated: Vec<String>,
    /// 短名 → binary（Python dict 推导式语义：按注册表插入序，同短名后者胜出）
    index: BTreeMap<String, String>,
    /// 注册表内全部接口的短名
    iface_shorts: BTreeSet<String>,
}

/// 未限定的短名：`/` 与 `.` 之后的末段，`$` → `_`
fn plain_short(binary: &str) -> String {
    let last = binary.rsplit('/').next().unwrap_or("");
    let last = last.rsplit('.').next().unwrap_or("");
    last.replace('$', "_")
}

fn qualify(binary: &str) -> String {
    binary.replace(['/', '$'], "_")
}

impl ShortNames {
    pub fn build(reg: &Registry) -> ShortNames {
        let canonical: BTreeSet<&str> = CANONICAL_OWNERS.iter().copied().filter(|b| reg.contains(b)).collect();
        let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for ci in reg.iter() {
            let b = ci.name();
            if !b.contains('/') || canonical.contains(b) {
                continue;
            }
            let simple = b.rsplit('/').next().unwrap_or(b).replace('$', "_");
            groups.entry(simple).or_default().push(b);
        }
        let mut qualified = BTreeMap::new();
        for members in groups.values() {
            // BTreeMap 迭代已按字典序：首个保留简单名
            for b in members.iter().skip(1) {
                qualified.insert(b.to_string(), qualify(b));
            }
        }
        let mut prelude_disambiguated = Vec::new();
        for (short, members) in &groups {
            if !PRELUDE_CONFLICT_NAMES.contains(&short.as_str()) {
                continue;
            }
            for b in members {
                if qualified.contains_key(*b) {
                    continue;
                }
                qualified.insert(b.to_string(), qualify(b));
                prelude_disambiguated.push(b.to_string());
            }
        }
        prelude_disambiguated.sort();
        let mut names =
            ShortNames { qualified, prelude_disambiguated, index: BTreeMap::new(), iface_shorts: BTreeSet::new() };
        for ci in reg.iter_insertion() {
            let short = names.short(ci.name()).into_owned();
            if ci.is_interface() {
                names.iface_shorts.insert(short.clone());
            }
            names.index.insert(short, ci.name().to_string());
        }
        names
    }

    /// `short_cls`：binary name → Rust 类型名
    pub fn short<'a>(&'a self, binary: &str) -> Cow<'a, str> {
        match self.qualified.get(binary) {
            Some(q) => Cow::Borrowed(q),
            None => Cow::Owned(plain_short(binary)),
        }
    }

    /// 限定改名表（binary → 限定名）
    pub fn qualified(&self) -> &BTreeMap<String, String> {
        &self.qualified
    }

    /// 本轮因 prelude 冲突改名的 binary（字典序）
    pub fn prelude_disambiguated(&self) -> &[String] {
        &self.prelude_disambiguated
    }

    /// 短名 → binary 反查（注册表域内）
    pub fn binary_of(&self, short: &str) -> Option<&str> {
        self.index.get(short).map(String::as_str)
    }

    /// 短名是否属于注册表内某个类
    pub fn is_registry_short(&self, short: &str) -> bool {
        self.index.contains_key(short)
    }

    /// 短名是否属于注册表内某个接口（`registry_iface_shorts`）
    pub fn is_iface_short(&self, short: &str) -> bool {
        self.iface_shorts.contains(short)
    }
}
