use crate::prelude::*;
use super::services_catalog::ServicesCatalog;

// jdk.internal.module.ServicesCatalog 伴生：命名模块层的服务目录。
// 单二进制无模块层——加载器永远没有已登记的服务目录：
// getServicesCatalogOrNull 恒 null（消费方 ServiceLoader 的
// ModuleServicesLookupIterator.iteratorFor 对 null 目录退空 provider 列表，
// 即「无命名模块 provider」的单二进制语义）。
impl ServicesCatalog {
    #[jvm_boundary]
    pub fn getServicesCatalogOrNull(_loader: crate::java::lang::ClassLoader) -> Result<ServicesCatalog> {
        Ok(Default::default())
    }
}
