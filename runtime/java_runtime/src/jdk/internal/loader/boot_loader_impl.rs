use crate::prelude::*;
use super::boot_loader::BootLoader;

// jdk.internal.loader.BootLoader 伴生：只含 ACC_NATIVE 方法（手写边界 ①）。服务目录（getServicesCatalog）
// 与资源定位（findResource / findResourceAsStream）走字节码：引导层由 Module.defineModules 按 jimage
// 系统模块登记服务目录，资源经 SystemModuleReader 读本程序 jimage（boot-image §5.7）。
impl BootLoader {
    /// native `getSystemPackageNames()`：boot 层已定义包名（Package.getPackages / ClassLoader
    /// .getPackages 的 boot 部分）。单二进制无模块层包登记——空数组（BootLoader.packages()
    /// 的其余部分由 Java 侧按已加载类补齐）。
    #[jvm_native]
    pub fn getSystemPackageNames() -> Result<JArray<String>> {
        Ok(JArray::new(0))
    }

    /// native `getSystemPackageLocation(String)`：引导加载器的包（内部形式）所在位置——HotSpot 取包所属
    /// 模块的 location（引导层模块为 `jrt:/<模块名>`，`PackageHelper.definePackage` 据此按模块定义
    /// Package）。引导层模块登记于 VM 模块表（映像初值）；未登记的包 → null。
    #[jvm_native]
    pub fn getSystemPackageLocation(name: String) -> Result<String> {
        if name.is_jvm_null() {
            return Ok(Default::default());
        }
        let pkg = format!("{}", name);
        Ok(match crate::java::lang::Module::__vm_boot_package_location(&pkg) {
            Some(loc) => String::from(loc.as_str()),
            None => Default::default(),
        })
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
