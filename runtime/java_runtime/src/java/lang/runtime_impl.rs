use crate::prelude::*;
use super::*;

impl Runtime {
    /// native availableProcessors()：可用处理器数。
    ///
    /// 对象模型是单线程协作调度（S-11，`Rc` 非 Send、无 OS 线程，见
    /// java/lang/thread_impl.rs 模块注释）——本运行时可实际利用的处理器数
    /// 恒为 1，如实报告 1（等价真机以 `-XX:ActiveProcessorCount=1` / 单核
    /// 环境启动的 JVM 配置档）。此前返回
    /// `std::thread::available_parallelism()` 的真实核数并断言「该值不进入
    /// 任何输出」：该断言被 CompletableFuture 打破——
    /// `USE_COMMON_POOL = ForkJoinPool.getCommonPoolParallelism() > 1` 在
    /// 多核报告下为 true，异步任务选入 commonPool（`ForkJoinPool.execute`
    /// 的 work-stealing 执行体在单线程档位不可承载，存根 panic）；单核报告
    /// 下为 false，异步任务走 `ThreadPerTaskExecutor`（每任务一线程，由既有
    /// 协作调度泵推进）——与单核 JVM 行为一致。j.u.c 其余消费
    /// （ConcurrentHashMap / Striped64 的 NCPU 批次切分阈值）在单线程语义
    /// 下不改变结果。
    #[jvm_native]
    pub fn availableProcessors(&self) -> Result<i32> {
        Ok(1)
    }
}
