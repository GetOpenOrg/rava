use crate::prelude::*;
use super::boot_loader::BootLoader;

// jdk.internal.loader.BootLoader 伴生：bootstrap 加载器（null 层级顶）的
// 服务面。运行期全部类由 boot 定义：模块 provider 全部登记在 boot 服务目录
// （ServicesCatalog::__boot_catalog，按分析器导出的服务事实装填）。
impl BootLoader {
    /// `loadLibrary(String)`：JDK 以 System.loadLibrary 加载 libzip / libnio 等本地库。
    /// 原生二进制的 native 方法全部静态链接在 java_runtime 内（`*_impl.rs`），无动态库
    /// 可加载——no-op。消费方：ZipUtils.loadLibrary（Adler32 / CRC32 的 <clinit>）。
    #[jvm_boundary]
    pub fn loadLibrary(_name: String) -> Result<()> {
        Ok(())
    }

    /// `getServicesCatalog()`：boot 加载器定义的模块的服务目录（JDK：引导期逐模块
    /// `ServicesCatalog.register` 装填）。运行期全部类由 boot 定义 → 全部模块 provider 在此；
    /// 进程唯一目录，首次请求时按服务事实装填（addProvider 在有模块 provider 时由
    /// seeds.toml [services] population 作根，与此处只在服务表非空时调用一致）。
    #[jvm_boundary(upcalls = "jdk/internal/module/ServicesCatalog.create:()Ljdk/internal/module/ServicesCatalog;")]
    pub fn getServicesCatalog() -> Result<crate::jdk::internal::module::ServicesCatalog> {
        crate::jdk::internal::module::ServicesCatalog::__boot_catalog()
    }

    /// `loadClass(Module, String)`：在 boot 加载器中按名加载 module 内的类，未找到或
    /// 不属于该模块 → null（与字节码同：`loadClassOrNull(name)` 后比对 `getModule()`）。
    /// 消费方：`Class.forName(Module, String)`（模块加载器为 null 时），即
    /// `ServiceLoader.loadProvider` 按服务目录的 provider 名加载实现类。
    #[jvm_boundary(upcalls = "java/lang/ClassNotFoundException.<init>:(Ljava/lang/String;)V")]
    pub fn loadClass(module: crate::java::lang::Module, name: String) -> Result<crate::java::lang::Class> {
        let c = BootLoader::loadClassOrNull(name)?;
        if !c.is_jvm_null() && c.getModule()? == module {
            return Ok(c);
        }
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
