//! 文件名字作用域：一个生成文件内类型名的认领表。
//!
//! 文件里每个类的显示名都经本作用域取得：首次取名即认领，同一文件内此后恒用同一名字；
//! 认领记录（binary → 显示名、派生名后缀）就是该文件的导入来源。作用域只在本文件内
//! 生效，类在别的文件里叫什么与此无关。
//!
//! [`Names`] 是「名字来源」的统一接口：全局表 [`ShortNames`]（无作用域的分析期查询）、
//! 带作用域的 [`crate::TyCtx`] 都实现它，渲染函数只认这个接口。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use crate::short_names::ShortNames;

/// 类型名来源：binary → 显示名，显示名 → binary
pub trait Names {
    /// binary → 本处使用的 Rust 类型名（带作用域时即认领）
    fn short(&self, binary: &str) -> String;
    /// 显示名 → binary（先本作用域，再全局表）
    fn binary_of(&self, name: &str) -> Option<String>;
    /// 显示名是否指注册表内的类
    fn is_registry_short(&self, name: &str) -> bool {
        self.binary_of(name).is_some()
    }
    /// 显示名是否指注册表内的接口
    fn is_iface_short(&self, name: &str) -> bool;
}

impl Names for ShortNames {
    fn short(&self, binary: &str) -> String {
        ShortNames::short(self, binary).into_owned()
    }
    fn binary_of(&self, name: &str) -> Option<String> {
        ShortNames::binary_of(self, name).map(str::to_string)
    }
    fn is_registry_short(&self, name: &str) -> bool {
        ShortNames::is_registry_short(self, name)
    }
    fn is_iface_short(&self, name: &str) -> bool {
        ShortNames::is_iface_short(self, name)
    }
}

/// 作用域的可保存状态（跨发射阶段续用）
#[derive(Debug, Clone, Default)]
pub struct ScopeState {
    /// binary → 显示名
    by_bin: BTreeMap<String, String>,
    /// 显示名 → binary
    by_name: BTreeMap<String, String>,
    /// 用到的派生名（binary, 后缀），如 `__VTable`、`__m_base`
    derived: BTreeSet<(String, String)>,
}

impl ScopeState {
    /// 认领记录（binary 序）：(binary, 显示名)
    pub fn claims(&self) -> impl Iterator<Item = (&str, &str)> {
        self.by_bin.iter().map(|(b, n)| (b.as_str(), n.as_str()))
    }

    /// 派生名记录（binary 序）：(binary, 后缀)
    pub fn derived(&self) -> impl Iterator<Item = (&str, &str)> {
        self.derived.iter().map(|(b, s)| (b.as_str(), s.as_str()))
    }

    /// binary 的显示名（已认领）
    pub fn name_of(&self, binary: &str) -> Option<&str> {
        self.by_bin.get(binary).map(String::as_str)
    }
}

/// 一个生成文件的名字作用域
#[derive(Debug, Default)]
pub struct NameScope {
    self_bin: String,
    state: Mutex<ScopeState>,
}

impl NameScope {
    /// 新作用域：本文件的类先认领自己的名字
    pub fn new(self_bin: &str, global: &ShortNames) -> NameScope {
        let s = NameScope { self_bin: self_bin.to_string(), state: Mutex::new(ScopeState::default()) };
        s.claim(global, self_bin);
        s
    }

    /// 续用保存的状态
    pub fn resume(self_bin: &str, state: ScopeState) -> NameScope {
        NameScope { self_bin: self_bin.to_string(), state: Mutex::new(state) }
    }

    /// 本文件的类
    pub fn self_bin(&self) -> &str {
        &self.self_bin
    }

    /// 取出状态（阶段结束保存）
    pub fn into_state(self) -> ScopeState {
        self.state.into_inner().unwrap_or_else(|e| e.into_inner())
    }

    /// 状态快照
    pub fn snapshot(&self) -> ScopeState {
        self.lock().clone()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ScopeState> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// 结构化引用集按 binary 序预认领
    pub fn preclaim<'s>(&self, global: &ShortNames, bins: impl IntoIterator<Item = &'s str>) {
        for b in bins {
            self.claim(global, b);
        }
    }

    /// binary 的显示名：已认领直接返回，否则认领
    pub fn claim(&self, global: &ShortNames, binary: &str) -> String {
        let mut st = self.lock();
        if let Some(n) = st.by_bin.get(binary) {
            return n.clone();
        }
        let name = global.short(binary).into_owned();
        st.by_name.entry(name.clone()).or_insert_with(|| binary.to_string());
        st.by_bin.insert(binary.to_string(), name.clone());
        name
    }

    /// 派生名（`<显示名><后缀>`）：认领 binary 并登记后缀
    pub fn derived(&self, global: &ShortNames, binary: &str, suffix: &str) -> String {
        let n = self.claim(global, binary);
        self.lock().derived.insert((binary.to_string(), suffix.to_string()));
        format!("{n}{suffix}")
    }

    /// 显示名 → binary（仅本作用域）
    pub fn lookup(&self, name: &str) -> Option<String> {
        self.lock().by_name.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claims_are_stable_and_recorded() {
        let g = ShortNames::default();
        let s = NameScope::new("p/A", &g);
        assert_eq!(s.claim(&g, "q/B$C"), "B_C");
        assert_eq!(s.derived(&g, "q/D", "__VTable"), "D__VTable");
        assert_eq!(s.lookup("B_C").as_deref(), Some("q/B$C"));
        let st = s.into_state();
        let claims: Vec<_> = st.claims().collect();
        assert_eq!(claims, vec![("p/A", "A"), ("q/B$C", "B_C"), ("q/D", "D")]);
        assert_eq!(st.derived().collect::<Vec<_>>(), vec![("q/D", "__VTable")]);
    }
}
