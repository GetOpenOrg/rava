use crate::prelude::*;
use super::*;

impl Runtime {
    /// native availableProcessors()：可用处理器数（FS-T3 最终态）。
    ///
    /// 如实报告宿主可用并行度（`std::thread::available_parallelism`，与 HotSpot
    /// 读取 CPU 亲和集 / cgroup 配额同一来源）。消费面：ForkJoinPool 公共池并行度
    /// （`ncpu - 1`）、CompletableFuture 的 `USE_COMMON_POOL`（> 1 时异步任务走公共池）、
    /// 并行流拆分、ConcurrentHashMap / Striped64 的 NCPU 阈值。线程是真实 OS 线程
    /// （#42），公共池工作线程为 OS 线程真并行执行。
    #[jvm_native]
    pub fn availableProcessors(&self) -> Result<i32> {
        Ok(std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(1))
    }
}
