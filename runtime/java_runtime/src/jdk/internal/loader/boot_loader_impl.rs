use crate::prelude::*;
use super::boot_loader::BootLoader;

// jdk.internal.loader.BootLoader 伴生：bootstrap 加载器（null 层级顶）的
// 服务面。单二进制无模块层——boot 服务目录恒 null（消费方
// ServiceLoader.ModuleServicesLookupIterator.iteratorFor 对 null 目录退空
// provider 列表）。
impl BootLoader {
    /// `loadLibrary(String)`：JDK 以 System.loadLibrary 加载 libzip / libnio 等本地库。
    /// 原生二进制的 native 方法全部静态链接在 java_runtime 内（`*_impl.rs`），无动态库
    /// 可加载——no-op。消费方：ZipUtils.loadLibrary（Adler32 / CRC32 的 <clinit>）。
    #[jvm_boundary]
    pub fn loadLibrary(_name: String) -> Result<()> {
        Ok(())
    }

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

    /// `loadClassOrNull(String name)`：boot 层按名加载，未找到 → null。原生单二进制的类宇宙
    /// 编译期定死，经 `Class.forName0` 同一元数据表判定存在性（ClassNotFoundException → null）。
    /// 消费方：ClassSpecializer 按类名先查预生成的 BMH 物种类（MH-native）。
    #[jvm_boundary(upcalls = "java/lang/ClassNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn loadClassOrNull(name: String) -> Result<crate::java::lang::Class> {
        match crate::java::lang::Class::forName0(name, false, Default::default(), Default::default()) {
            Ok(c) => Ok(c),
            Err(_) => Ok(Default::default()),
        }
    }

    /// native `getSystemPackageNames()`：boot 层已定义包名（Package.getPackages / ClassLoader
    /// .getPackages 的 boot 部分）。单二进制无模块层包登记——空数组（BootLoader.packages()
    /// 的其余部分由 Java 侧按已加载类补齐）。
    #[jvm_native]
    pub fn getSystemPackageNames() -> Result<JArray<String>> {
        Ok(JArray::new(0))
    }
}
