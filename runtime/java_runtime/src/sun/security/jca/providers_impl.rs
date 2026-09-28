//! `sun/security/jca/Providers` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 在此持有线程 / 全局 provider 列表（ProviderList，按 java.security 配置装载）。原生侧
//! 的 provider 列表 = 生成 main 登记的 provider（`crate::jca`，登记序即 security.provider.N
//! 优先序，provider 对象按需构造一次）。docs/plans/2026-09-28-jca-faithful-provider.md。

use crate::prelude::*;
use super::providers::implref::Providers;
use super::provider_list::implref::ProviderList;
use crate::java::security::Provider;

impl Providers {
    /// `getProviderList()`：全局 provider 列表视图（状态在 `crate::jca`，列表对象无自有字段）。
    #[jvm_boundary]
    pub fn getProviderList() -> Result<ProviderList> {
        let mut list = ProviderList::default();
        list._init_not_null();
        Ok(list)
    }

    /// `getSunProvider()`：SUN provider（JDK 在列表外另建 `new Sun()`；列表内实例语义等价）。
    #[jvm_boundary]
    pub fn getSunProvider() -> Result<Provider> {
        match crate::jca::provider("SUN")? {
            Some(p) => Ok(<Provider as ::std::convert::From<Object>>::from(p)),
            None => panic!("stub: sun/security/jca/Providers.getSunProvider:()Ljava/security/Provider;（SUN provider 未登记）"),
        }
    }
}
