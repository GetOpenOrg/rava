//! `sun/security/jca/ProviderList` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//! provider 集合见 `providers_impl.rs` 模块说明。

use crate::prelude::*;
use super::provider_list::implref::ProviderList;
use crate::java::security::Provider;
use crate::java::util::ArrayList;
use crate::java::util::List;

impl ProviderList {
    /// `providers()`：按优先序的全部 provider（不可变视图语义；调用方只遍历）。
    #[jvm_boundary]
    pub fn __impl_providers(&self) -> Result<List<Object>> {
        let out = ArrayList::<Object>::new()?;
        for p in crate::jca::all_providers()? {
            out.add_obj(p)?;
        }
        Ok(<List<Object> as ::std::convert::From<Object>>::from(Object::from(out)))
    }

    /// `getProvider(String name)`：按名取 provider，未登记 → null。
    #[jvm_boundary]
    pub fn __impl_getProvider(&self, name: String) -> Result<Provider> {
        Ok(match crate::jca::provider(&format!("{}", name))? {
            Some(p) => <Provider as ::std::convert::From<Object>>::from(p),
            None => Provider::default(),
        })
    }
}
