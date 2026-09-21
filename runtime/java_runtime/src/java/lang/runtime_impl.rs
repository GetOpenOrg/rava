use crate::prelude::*;
use super::*;

impl Runtime {
    /// native availableProcessors()：可用处理器数。HotSpot 的
    /// JVM_ActiveProcessorCount 返回 os::active_processor_count()（受 affinity /
    /// cgroup 配额约束的进程可用并行度），语义等价 Rust 标准库的
    /// std::thread::available_parallelism()。
    ///
    /// 可观测性：该值不进入任何输出——java.base 中唯一消费形态是
    /// j.u.c 类的 `static final int NCPU = Runtime.getRuntime()
    /// .availableProcessors()`（ConcurrentHashMap / Striped64 等），仅作
    /// 批次切分阈值，单线程语义下不影响结果。查询失败（affinity 不可得
    /// 等）按 JVM 对不确定环境的惯例回落 1。
    #[jvm_native]
    pub fn availableProcessors(&self) -> Result<i32> {
        Ok(std::thread::available_parallelism()
            .map(|n| n.get() as i32)
            .unwrap_or(1))
    }
}
