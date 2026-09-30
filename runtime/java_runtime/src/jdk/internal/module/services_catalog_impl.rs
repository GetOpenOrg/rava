use crate::prelude::*;
use super::services_catalog::ServicesCatalog;

// jdk.internal.module.ServicesCatalog 伴生（jdk/internal/module 已放行，其余成员按字节码翻译，
// findServices 亦然：`map.getOrDefault(service, List.of())`）。
//
// getServicesCatalogOrNull 为过渡手写（handwritten-boundary.md §三，因截断而补的手写），终态删除：
// 字节码 `CLV.get(loader)` 经 jdk/internal/loader/AbstractClassLoaderValue（边界内，get / map 为存根）
// → JavaLangAccess.createOrGetClassLoaderValueMap（整体手写的实现对象未实现）。
//
// 运行期加载器模型：全部类由 boot 定义（Class.getClassLoader 恒 null，Class.getModule 为进程唯一的
// 模块且其加载器为 null），故模块 provider 全部登记在 boot 目录（BootLoader.getServicesCatalog）；
// app / platform 加载器没有自己定义的模块 → 无目录（null；消费方 ServiceLoader 的
// ModuleServicesLookupIterator.iteratorFor 对 null 目录退空列表，继续沿父链走到 boot）。
// JDK 上 platform 模块（如 jdk.charsets）的 provider 在 platform 一步命中；此处在 boot 一步命中，
// 同一 provider、同一迭代结果。
impl ServicesCatalog {
    #[jvm_boundary]
    pub fn getServicesCatalogOrNull(_loader: crate::java::lang::ClassLoader) -> Result<ServicesCatalog> {
        Ok(Default::default())
    }

    /// 引导服务目录：进程内唯一，首次请求时按分析器导出的服务事实装填
    /// （closure.json seeds.services 的模块 provider，经 build.rs 生成 services_table.rs）。
    /// 装填走字节码翻译的 `create()` + `addProvider(provider 所在模块, 服务, provider)`，
    /// 与 JDK 引导期 `ServicesCatalog.register(Module)` 按模块描述符 provides 登记的结果同构。
    /// 同时充当 boot 层的层目录（`JavaLangAccess.getServicesCatalog(ModuleLayer)`：
    /// JDK 引导层目录即全部模块 provides 的汇总）。
    pub fn __boot_catalog() -> Result<ServicesCatalog> {
        crate::__process_static! {
            static BOOT: RefCell<Option<ServicesCatalog>> = const { RefCell::new(None) };
        }
        if let Some(c) = BOOT.with(|b| b.borrow().clone()) {
            return Ok(c);
        }
        let catalog = ServicesCatalog::create()?;
        for (service, provider) in services_table::MODULE_SERVICES {
            let service = crate::java::lang::Class::for_class(String::from(*service));
            let provider = crate::java::lang::Class::for_class(String::from(*provider));
            catalog.addProvider(provider.getModule()?, service, provider)?;
        }
        BOOT.with(|b| *b.borrow_mut() = Some(catalog.clone()));
        Ok(catalog)
    }
}

mod services_table {
    include!(concat!(env!("OUT_DIR"), "/services_table.rs"));
}
