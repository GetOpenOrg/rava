use crate::prelude::*;
use super::builtin_class_loader::BuiltinClassLoader;
use super::EmbeddedClassPath;

// jdk.internal.loader.BuiltinClassLoader 伴生：类路径运行模型（vm_intrinsics.toml kind = class_path，手写类 ②；
// 计划 docs/plans/2026-10-01-c1d-closure-bloat.md §30.15）。原生二进制无运行期应用类路径：类路径上没有可
// 定义的字节码，类路径资源取自构建期嵌入表（VM 支持类 EmbeddedClassPath）。`hasClassPath()`（ucp 非 null）
// 的判定照旧：只有应用加载器带类路径。
impl BuiltinClassLoader {
    /// `findClassOnClassPathOrNull(String)`：类全集构建期静态链接（闭包内的类已由 `findLoadedClass0` 命中），
    /// 类路径上无可定义的字节码 → null。
    #[jvm_native]
    pub fn findClassOnClassPathOrNull(&self, cn: String) -> Result<crate::java::lang::Class> {
        let _ = cn;
        Ok(Default::default())
    }

    /// `findResourceOnClassPath(String)`：带类路径的加载器 → 嵌入表首个同名资源的 URL；否则 null。
    #[jvm_native]
    pub fn findResourceOnClassPath(&self, name: String) -> Result<crate::java::net::URL> {
        if self.__get_ucp().is_jvm_null() {
            return Ok(Default::default());
        }
        EmbeddedClassPath::findResource(name)
    }

    /// `findResourcesOnClassPath(String)`：带类路径的加载器 → 嵌入表同名资源的 URL 枚举（类路径序）；否则空枚举。
    #[jvm_native]
    pub fn findResourcesOnClassPath(&self, name: String) -> Result<crate::java::util::Enumeration<Object>> {
        if self.__get_ucp().is_jvm_null() {
            return crate::java::util::Collections::emptyEnumeration();
        }
        EmbeddedClassPath::findResources(name)
    }
}
