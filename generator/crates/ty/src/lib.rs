//! 发射层类型层（类型映射 / 签名解析 / 类型实参 / 签名类型 / JVM 类型代数）。
//!
//! 与 Python 的结构差异：
//! - 注册表 [`Registry`]、短名 [`ShortNames`]、清单 [`Manifest`] 是显式上下文，
//!   经 [`TyCtx`] 传递；无模块级全局状态；
//! - Rust 类型是结构化的 [`RsType`]，文本只在 [`RsType::render`] 出口产生；
//! - JVM 侧类型代数是 [`JvmType`]。

pub mod carrier;
pub mod class_params;
pub mod consts;
pub mod fn_sig;
pub mod ident;
pub mod jvm_type;
pub mod manifest;
pub mod name_scope;
pub mod registry;
pub mod rs_type;
pub mod short_names;
pub mod sig_parse;
pub mod sig_types;
pub mod type_args;
pub mod type_map;

#[cfg(test)]
pub(crate) mod testutil;

pub use fn_sig::FnSig;
pub use jvm_type::{HostPrim, JvmType, PrimKind, TypeParseError, WildKind};
pub use manifest::Manifest;
pub use registry::{ClassInfo, Registry};
pub use rs_type::{Prim, RsType};
pub use name_scope::{NameScope, Names, ScopeState};
pub use short_names::ShortNames;
pub use sig_parse::MethodSigTypes;
pub use sig_types::{EmittedSig, SigTypes};

/// 类型层查询上下文：注册表 + 名字来源 + runtime 清单（均不可变）
///
/// 名字一律经 [`TyCtx::short`] / [`Names`] 取得：带文件作用域时在该文件内认领，
/// 无作用域（分析期查询）时取全局表。
#[derive(Clone, Copy)]
pub struct TyCtx<'a> {
    pub reg: &'a Registry,
    global: &'a ShortNames,
    scope: Option<&'a NameScope>,
    pub manifest: &'a Manifest,
}

impl<'a> TyCtx<'a> {
    pub fn new(reg: &'a Registry, names: &'a ShortNames, manifest: &'a Manifest) -> TyCtx<'a> {
        TyCtx {
            reg,
            global: names,
            scope: None,
            manifest,
        }
    }

    /// 同一上下文换上文件作用域
    pub fn scoped<'b>(&self, scope: &'b NameScope) -> TyCtx<'b>
    where
        'a: 'b,
    {
        TyCtx { reg: self.reg, global: self.global, scope: Some(scope), manifest: self.manifest }
    }

    /// 全局名字表（建作用域、分析期统计用；发射文本不得经此取名）
    pub fn global_names(&self) -> &'a ShortNames {
        self.global
    }

    /// 文件作用域（无作用域 = 分析期查询）
    pub fn scope(&self) -> Option<&'a NameScope> {
        self.scope
    }

    /// binary → 本处的 Rust 类型名
    pub fn short(&self, binary: &str) -> String {
        match self.scope {
            Some(s) => s.claim(self.global, binary),
            None => self.global.short(binary).into_owned(),
        }
    }

    /// 派生名 `<类型名><后缀>`（`__VTable`、`__m_base` 等），带作用域时登记以便导入
    pub fn derived(&self, binary: &str, suffix: &str) -> String {
        match self.scope {
            Some(s) => s.derived(self.global, binary, suffix),
            None => format!("{}{suffix}", self.global.short(binary)),
        }
    }

    /// 结构化引用集按 binary 序预认领（无作用域时无操作）
    pub fn preclaim<'s>(&self, bins: impl IntoIterator<Item = &'s str>) {
        if let Some(s) = self.scope {
            s.preclaim(self.global, bins);
        }
    }

    /// 显示名 → binary（先本作用域，再全局表）
    pub fn binary_of(&self, name: &str) -> Option<String> {
        self.scope
            .and_then(|s| s.lookup(name))
            .or_else(|| self.global.binary_of(name).map(str::to_string))
    }
}

impl TyCtx<'_> {
    /// 显示名是否指注册表内的类
    pub fn is_registry_short(&self, name: &str) -> bool {
        self.binary_of(name).is_some_and(|b| self.reg.contains(&b))
    }

    /// 显示名是否指注册表内的接口
    pub fn is_iface_short(&self, name: &str) -> bool {
        self.binary_of(name).is_some_and(|b| self.reg.get(&b).is_some_and(ClassInfo::is_interface))
    }
}

impl Names for TyCtx<'_> {
    fn short(&self, binary: &str) -> String {
        TyCtx::short(self, binary)
    }
    fn binary_of(&self, name: &str) -> Option<String> {
        TyCtx::binary_of(self, name)
    }
    fn is_registry_short(&self, name: &str) -> bool {
        TyCtx::is_registry_short(self, name)
    }
    fn is_iface_short(&self, name: &str) -> bool {
        TyCtx::is_iface_short(self, name)
    }
}
