//! `java/security/Provider` 手写伴生：边界类（java/security/ 前缀），按调用链按需实现（K-2 规则）。
//!
//! K-JCA：provider 对象只承担「身份」——名字（`getName`）与服务查找（`getService`，见
//! `provider_service_impl.rs`）。每个 provider 名对应线程内唯一对象（JDK ProviderList 中
//! provider 实例唯一，`==` 比较成立）。服务表来自 `crate::jca`（生成注册表）。

use crate::prelude::*;
use super::provider::implref::Provider;
use std::collections::HashMap;

std::thread_local! {
    static PROVIDERS: crate::sync_model::__RefSlot<HashMap<&'static str, Provider>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

impl Provider {
    /// provider 名 → 线程内唯一的 Provider 对象（首次取用时构造，仅设 name 字段）。
    pub fn __for_name(name: &'static str) -> Provider {
        PROVIDERS.with(|p| {
            let mut p = p.borrow_mut();
            let prov = p.entry(name).or_insert_with(|| {
                let mut prov = Provider::default();
                prov._init_not_null();
                prov.__set_name(String::from(name));
                prov
            });
            Clone::clone(&*prov)
        })
    }

    /// `getName()`：provider 名。
    pub fn __impl_getName(&self) -> Result<String> {
        Ok(self.__get_name())
    }
}
