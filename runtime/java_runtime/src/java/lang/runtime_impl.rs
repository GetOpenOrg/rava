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

    /// native `maxMemory()`：原生二进制无 Java 堆上限——按 JVM 缺省 MaxHeapSize（物理内存 1/4）
    /// 报告；物理内存不可得时 Long.MAX_VALUE（JDK「无上限」约定）。
    #[jvm_native]
    pub fn maxMemory(&self) -> Result<i64> {
        Ok(physical_memory().map(|m| (m / 4) as i64).unwrap_or(i64::MAX))
    }

    /// native `totalMemory()`：当前已向 OS 申请的内存（进程常驻集，/proc/self/statm）。
    #[jvm_native]
    pub fn totalMemory(&self) -> Result<i64> {
        Ok(resident_bytes().unwrap_or(0) as i64)
    }

    /// native `freeMemory()`：原生内存由分配器即时归还，已申请而空闲的部分不可观察，恒 0
    /// （`totalMemory() - freeMemory()` 即已用内存，与 JVM 的用法语义一致）。
    #[jvm_native]
    pub fn freeMemory(&self) -> Result<i64> {
        Ok(0)
    }

    /// native `gc()`：引用计数即时回收，无可触发的收集周期——no-op。
    #[jvm_native]
    pub fn gc(&self) -> Result<()> {
        Ok(())
    }
}

/// 物理内存字节数（sysconf(_SC_PHYS_PAGES) × 页大小）。
fn physical_memory() -> Option<u64> {
    // SAFETY: sysconf 只查询系统常量
    let (pages, page) = unsafe { (libc::sysconf(libc::_SC_PHYS_PAGES), libc::sysconf(libc::_SC_PAGESIZE)) };
    if pages > 0 && page > 0 { Some(pages as u64 * page as u64) } else { None }
}

/// 进程常驻集字节数（Linux /proc/self/statm 第 2 列 × 页大小；其余平台不可得）。
fn resident_bytes() -> Option<u64> {
    let statm = std::fs::read_to_string("/proc/self/statm").ok()?;
    let pages: u64 = statm.split_whitespace().nth(1)?.parse().ok()?;
    // SAFETY: sysconf 只查询系统常量
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page > 0 { Some(pages * page as u64) } else { None }
}
