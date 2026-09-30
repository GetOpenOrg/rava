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

    /// 层服务目录（`JavaLangAccess.getServicesCatalog(ModuleLayer)` 的返回值）：进程内唯一的空目录。
    /// JDK 的 `ServiceLoader.LayerLookupIterator.providers` 不对目录判空，直接 `findServices`，
    /// 故层目录须为非 null 对象（boot 层无命名模块 → 目录为空）。
    pub fn __empty_layer_catalog() -> ServicesCatalog {
        crate::__process_static! {
            static EMPTY: ServicesCatalog = {
                let mut c = ServicesCatalog::default();
                c._init_not_null();
                c
            };
        }
        EMPTY.with(Clone::clone)
    }

    /// `findServices(String)`：单二进制无命名模块的 provides 声明——恒空列表
    /// （JDK：`map.getOrDefault(service, List.of())` 在空目录上的结果）。
    #[jvm_boundary(upcalls = "java/util/ArrayList.<init>:()V")]
    pub fn findServices(&self, _service: String) -> Result<crate::java::util::List<Object>> {
        let empty = crate::java::util::ArrayList::<Object>::new()?;
        Ok(<crate::java::util::List<Object> as ::std::convert::From<_>>::from(empty))
    }
}
