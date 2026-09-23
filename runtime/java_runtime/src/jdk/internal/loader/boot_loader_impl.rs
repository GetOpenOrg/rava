use crate::prelude::*;
use super::boot_loader::BootLoader;

// jdk.internal.loader.BootLoader 伴生：bootstrap 加载器（null 层级顶）的
// 服务面。单二进制无模块层——boot 服务目录恒 null（消费方
// ServiceLoader.ModuleServicesLookupIterator.iteratorFor 对 null 目录退空
// provider 列表）。
impl BootLoader {
    #[jvm_boundary]
    pub fn getServicesCatalog() -> Result<crate::jdk::internal::module::ServicesCatalog> {
        Ok(Default::default())
    }
}
