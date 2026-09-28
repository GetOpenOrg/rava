use crate::prelude::*;
use super::module_layer::ModuleLayer;

// java.lang.ModuleLayer 伴生（VM 耦合边界类）：单二进制运行时的模块层语义，按调用链按需实现。
//
// 产物无 JPMS 运行时，全部类归属无名模块（module_impl.rs）。JVM 中 boot layer 恒存在，无名模块
// 不属于任何层（`Module.getLayer()` 为 null）——`boot()` 以进程内唯一的空层对象承载，使
// 「无名模块 ∉ boot layer」的判定与 JVM 一致（StackTraceElement.isHashedInJavaBase 等）。

impl ModuleLayer {
    /// `boot()`：boot layer（进程内唯一空层对象；层内无命名模块，configuration 等查询面未建模）。
    #[jvm_boundary]
    pub fn boot() -> Result<ModuleLayer> {
        crate::__process_static! {
            static BOOT: ModuleLayer = {
                let mut l = ModuleLayer::default();
                l._init_not_null();
                l
            };
        }
        Ok(BOOT.with(Clone::clone))
    }
}
