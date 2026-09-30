use crate::prelude::*;
use super::services_catalog::ServicesCatalog;

// jdk.internal.module.ServicesCatalog 伴生（jdk/internal/module 已放行，其余成员按字节码翻译）。
//
// 本文件三项均为过渡类（handwritten-boundary.md §三，策略截断 / 因截断而补的手写），终态删除，
// 当前被 jdk/internal/access、jdk/internal/loader 的截断阻塞（C1d 计划 §6.8 第 7 步起随之删除）：
// - getServicesCatalogOrNull：字节码 `CLV.get(loader)` 经 jdk/internal/loader/AbstractClassLoaderValue
//   （边界内，get / map 为存根）→ JavaLangAccess.createOrGetClassLoaderValueMap（整体手写的实现对象未实现）。
// - findServices / __empty_layer_catalog：层目录由整体手写的 JavaLangAccess.getServicesCatalog(ModuleLayer)
//   返回；该实现对象对闭包分析不透明（接口分派无 Java 实现类可达），其中对 ServicesCatalog.create 的调用
//   不入闭包、map 字段无写入来源——按字节码的 findServices 会被折叠为对 null 调 getOrDefault。
//
// 语义：单二进制无模块层——加载器没有已登记的服务目录（getServicesCatalogOrNull 恒 null，消费方
// ServiceLoader 的 ModuleServicesLookupIterator.iteratorFor 对 null 目录退空 provider 列表）；boot 层无命名模块，
// 层目录为空目录。
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
