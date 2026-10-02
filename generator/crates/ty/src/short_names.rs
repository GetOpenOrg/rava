//! 类的 Rust 名：定义名（declared）与全局令牌（token）。
//!
//! - **定义名**：类在自己文件里的 struct 名，只由类自身决定——简单名（`$` → `_`）；简单名是
//!   Rust prelude 可见名（[`PRELUDE_CONFLICT_NAMES`]）时取限定名（`/`、`$` → `_`），
//!   `Object` / `String` 的本主除外（直映射即 prelude 名本身，同一实体）。
//! - **令牌**：全局唯一的类名——同简单名组（≥2 个成员）的每个成员都取限定名，其余同定义名。
//!   令牌用于无作用域的反查（显示名 → binary），并作为文件作用域里定义名被占用时的本地别名
//!   （`use path::Simple as 令牌;`）。各文件里一个类叫什么由文件名字作用域（[`crate::NameScope`]）决定。

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use crate::consts;
use crate::registry::Registry;

/// 发射面以裸名引用的 prelude 名（`java_runtime::prelude` 再导出 + std prelude）
pub const PRELUDE_CONFLICT_NAMES: [&str; 18] = [
    "JArray",
    "JvmError",
    "Result",
    "Object",
    "ObjectVTable",
    "String",
    "MonitorGuard",
    "Rc",
    "__Shared",
    "RefCell",
    "Option",
    "Some",
    "None",
    "Ok",
    "Err",
    "Vec",
    "Clone",
    "Default",
];

/// prelude 名的本主（不参与任何冲突组）
const CANONICAL_OWNERS: [&str; 2] = [consts::STRING, consts::OBJECT];

#[derive(Debug, Clone, Default)]
pub struct ShortNames {
    /// 令牌与定义名不同的类：binary → 令牌（同简单名组成员 + prelude 同名类）
    qualified: BTreeMap<String, String>,
    /// 定义名取限定名的类（prelude 同名，字典序）
    prelude_disambiguated: Vec<String>,
    /// 令牌 → binary
    index: BTreeMap<String, String>,
    /// 注册表内全部接口的令牌
    iface_shorts: BTreeSet<String>,
}

/// 未限定的短名：`/` 与 `.` 之后的末段，`$` → `_`
fn plain_short(binary: &str) -> String {
    let last = binary.rsplit('/').next().unwrap_or("");
    let last = last.rsplit('.').next().unwrap_or("");
    last.replace('$', "_")
}

/// 限定名：完整 binary（`/`、`$` → `_`）
pub fn qualify(binary: &str) -> String {
    binary.replace(['/', '$'], "_")
}

/// 定义名是否取限定名：简单名与 prelude 名同名（本主除外）
fn prelude_named(binary: &str) -> bool {
    binary.contains('/') && !CANONICAL_OWNERS.contains(&binary) && PRELUDE_CONFLICT_NAMES.contains(&plain_short(binary).as_str())
}

/// 类的定义名（只由类自身决定）
pub fn declared(binary: &str) -> String {
    if prelude_named(binary) {
        qualify(binary)
    } else {
        plain_short(binary)
    }
}

impl ShortNames {
    pub fn build(reg: &Registry) -> ShortNames {
        let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for ci in reg.iter() {
            let b = ci.name();
            if !b.contains('/') || CANONICAL_OWNERS.contains(&b) {
                continue;
            }
            groups.entry(plain_short(b)).or_default().push(b);
        }
        let mut qualified = BTreeMap::new();
        let mut prelude_disambiguated = Vec::new();
        for members in groups.values() {
            for b in members {
                if prelude_named(b) {
                    prelude_disambiguated.push(b.to_string());
                    qualified.insert(b.to_string(), qualify(b));
                } else if members.len() > 1 {
                    qualified.insert(b.to_string(), qualify(b));
                }
            }
        }
        prelude_disambiguated.sort();
        let mut names = ShortNames {
            qualified,
            prelude_disambiguated,
            index: BTreeMap::new(),
            iface_shorts: BTreeSet::new(),
        };
        for ci in reg.iter_insertion() {
            let short = names.short(ci.name()).into_owned();
            if ci.is_interface() {
                names.iface_shorts.insert(short.clone());
            }
            names.index.insert(short, ci.name().to_string());
        }
        names
    }

    /// 令牌：binary → 全局唯一的 Rust 类型名
    pub fn short<'a>(&'a self, binary: &str) -> Cow<'a, str> {
        match self.qualified.get(binary) {
            Some(q) => Cow::Borrowed(q),
            None => Cow::Owned(plain_short(binary)),
        }
    }

    /// 定义名：类在自己文件里的 struct 名（导入路径末段）
    pub fn declared(&self, binary: &str) -> String {
        declared(binary)
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
