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

    /// `hasClassPath()`：`-Xbootclasspath/a` 追加的 boot class path 是否存在。原生单二进制
    /// 无 class path → false；消费方 `ServiceLoader.LazyClassPathLookupIterator` 据此对平台
    /// 加载器取空的 `META-INF/services` 配置枚举（扩展 charset provider 查找退空，
    /// `Charset.forName` 未知名最终 UnsupportedCharsetException，与 JDK 默认安装一致）。
    #[jvm_boundary]
    pub fn hasClassPath() -> Result<bool> {
        Ok(false)
    }
}
