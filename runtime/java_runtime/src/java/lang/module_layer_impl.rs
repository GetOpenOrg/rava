use crate::prelude::*;
use super::module_layer::ModuleLayer;

// java.lang.ModuleLayer 伴生（VM 耦合边界类）：单二进制运行时的模块层语义，按调用链按需实现。
//
// 产物无 JPMS 运行时，全部类归属无名模块（module_impl.rs）。JVM 中 boot layer 恒存在，无名模块
// 不属于任何层（`Module.getLayer()` 为 null）——`boot()` 以进程内唯一的空层对象承载，使
// 「无名模块 ∉ boot layer」的判定与 JVM 一致（StackTraceElement.isHashedInJavaBase 等）。

crate::__process_static! {
    // 进程内唯一的 boot layer
    static BOOT_LAYER: ModuleLayer = {
        let mut l = ModuleLayer::default();
        l._init_not_null();
        l
    };
}

impl ModuleLayer {
    /// `boot()`：boot layer（进程内唯一空层对象；层内无命名模块，configuration 等查询面未建模）。
    /// 服务目录由 `__vm_services_catalog` 钩子落地。
    #[jvm_boundary]
    pub fn boot() -> Result<ModuleLayer> {
        Ok(BOOT_LAYER.with(Clone::clone))
    }

    /// VM 注入状态 `servicesCatalog` 的落地（vm_intrinsics.toml `[vm_state.field_hooks]`，准入 ③）：
    /// HotSpot 引导期 initPhase2 经 `Module.defineModules` → `initServices` 把引导层各模块的 provides
    /// 登记进层目录。本运行时全部模块 provider 已登记在引导服务目录（`BootLoader.getServicesCatalog`，
    /// 按分析器服务事实装填；seeds.toml [services] lookups 含按层查找的
    /// `ServiceLoader(Class, ModuleLayer, Class)`），boot layer 的目录即该目录，首次访问前写入——
    /// `ServiceLoader.load(ModuleLayer.boot(), S)` 与引导加载器路径看到同一 provider 集合。
    /// 其他层（`defineModules*` 建立）的目录按字节码由 `nameToModule` 惰性建立，钩子不介入。
    pub fn __vm_services_catalog(&self) -> Result<&Self> {
        if !self.__get_servicesCatalog().is_jvm_null() {
            return Ok(self);
        }
        let is_boot = BOOT_LAYER.with(|l| Object::from(Clone::clone(l)) == Object::from(Clone::clone(self)));
        if is_boot {
            self.__set_servicesCatalog(crate::jdk::internal::loader::BootLoader::getServicesCatalog()?);
        }
        Ok(self)
    }

    /// `parents()`：boot 层的父层列表。产物无 JPMS 层级（boot 层即唯一空层），返回空列表——
    /// ServiceLoader 的 LayerLookupIterator 遍历到此即止（boot 层无 provider）。
    #[jvm_boundary]
    pub fn parents(&self) -> Result<crate::java::util::List<Object>> {
        let empty = crate::java::util::ArrayList::<Object>::new()?;
        Ok(<crate::java::util::List<Object> as ::std::convert::From<_>>::from(empty))
    }
}
