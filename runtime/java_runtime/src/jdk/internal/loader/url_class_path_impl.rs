use crate::prelude::*;
use super::url_class_path::URLClassPath;

// jdk.internal.loader.URLClassPath 伴生：类路径运行模型（vm_intrinsics.toml kind = class_path，手写类 ②；
// 计划 docs/plans/2026-10-01-c1d-closure-bloat.md §30.15）。原生二进制无运行期应用类路径（模块模式
// cp = null 的等价）：类全集构建期静态链接，类路径资源构建期嵌入（EmbeddedClassPath）。
impl URLClassPath {
    /// `toFileURL(String)`：类路径字符串元素 → 运行期加载源。无运行期类路径 → null（调用方 String 构造器 /
    /// `addFile` 跳过 null），应用 ucp 恒空。
    #[jvm_native]
    pub fn toFileURL(s: String) -> Result<crate::java::net::URL> {
        let _ = s;
        Ok(Default::default())
    }
}
