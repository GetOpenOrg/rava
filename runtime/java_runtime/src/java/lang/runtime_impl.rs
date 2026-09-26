use crate::prelude::*;
use super::*;

impl Runtime {
    /// native availableProcessors()：可用处理器数。
    ///
    /// #42 第一档（OS 线程 + GIL）下同一时刻只有一个线程执行 Java 代码，可实际利用的
    /// 处理器数为 1，如实报告 1（等价以 `-XX:ActiveProcessorCount=1` 启动的 JVM）。
    /// 消费面：CompletableFuture 的 `USE_COMMON_POOL = getCommonPoolParallelism() > 1`
    /// 为 false → 异步任务走 ThreadPerTaskExecutor（每任务一条 OS 线程）；
    /// ConcurrentHashMap / Striped64 的 NCPU 阈值不改变结果。
    /// 最终态（#42 第二档并行后端，FS-T3）返回真实核数，commonPool 按并行度运行。
    #[jvm_native]
    pub fn availableProcessors(&self) -> Result<i32> {
        Ok(1)
    }
}
