use crate::prelude::*;
use super::boot_loader::BootLoader;

// jdk.internal.loader.BootLoader 伴生：bootstrap 加载器（null 层级顶）的
// 服务面。运行期全部类由 boot 定义：模块 provider 全部登记在 boot 服务目录
// （`boot_catalog`，按分析器导出的服务事实装填）。引导层的模块定义与服务登记由 VM / 启动器
// 在 main 之前完成（handwritten-boundary.md 类 ③ VM 注入状态），目录本身与登记动作走字节码。
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
        boot_catalog()
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

    /// native `setBootLoaderUnnamedModule0(Module)`：`<clinit>`（字节码翻译）把引导加载器的无名模块
    /// 交给 VM，VM 据此给 `-Xbootclasspath/a` 追加路径上加载的类定模块。原生单二进制无引导类路径
    /// （`hasClassPath` 恒 false），没有类归属该模块——无需登记，no-op。模块对象本身仍由字节码建立，
    /// `BootLoader.getUnnamedModule()` 照常返回它。
    #[jvm_native]
    pub fn setBootLoaderUnnamedModule0(_module: crate::java::lang::Module) -> Result<()> {
        Ok(())
    }
}

/// 引导服务目录：进程内唯一，首次请求时按分析器导出的服务事实装填
/// （closure.json seeds.services 的模块 provider，经 java_meta 的 MODULE_SERVICES 表读取）。
/// 装填走字节码翻译的 `create()` + `addProvider(provider 所在模块, 服务, provider)`，
/// 与 JDK 引导期 `ServicesCatalog.register(Module)` 按模块描述符 provides 登记的结果同构。
fn boot_catalog() -> Result<crate::jdk::internal::module::ServicesCatalog> {
    use crate::jdk::internal::module::ServicesCatalog;
    crate::__process_static! {
        static BOOT: RefCell<Option<ServicesCatalog>> = const { RefCell::new(None) };
    }
    if let Some(c) = BOOT.with(|b| b.borrow().clone()) {
        return Ok(c);
    }
    let catalog = ServicesCatalog::create()?;
    for (service, provider) in crate::meta::module_services() {
        let service = crate::java::lang::Class::for_class(String::from(*service));
        let provider = crate::java::lang::Class::for_class(String::from(*provider));
        catalog.addProvider(provider.getModule()?, service, provider)?;
    }
    BOOT.with(|b| *b.borrow_mut() = Some(catalog.clone()));
    Ok(catalog)
}
