//! `jdk/internal/perf/PerfCounter` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! HotSpot 性能计数器（jvmstat 共享内存 `Perf` 缓冲）。原生二进制无 jvmstat：计数器是
//! 不可观测的统计量，构造返回占位实例，递增 / 累加为空操作，读取恒 0。
//! 消费方：LambdaForm / ClassLoader 的统计点（MH-native）。

use crate::prelude::*;
use super::perf_counter::implref::PerfCounter;

impl PerfCounter {
    #[jvm_boundary]
    pub fn newPerfCounter(_name: String) -> Result<PerfCounter> {
        let mut c = PerfCounter::default();
        c._init_not_null();
        Ok(c)
    }

    #[jvm_boundary]
    pub fn newConstantPerfCounter(name: String) -> Result<PerfCounter> {
        Self::newPerfCounter(name)
    }

    /// 核心计数器取值族（JDK 经嵌套持有者 `CoreCounters` / `ZipFileCounters` 惰性创建）：
    /// 无 jvmstat 时统计不可观测，返回占位计数器（其递增 / 累加为空操作）。消费方：
    /// `ClassLoader.loadClass` 的委派 / findClass 计时、`ZipFile` 的打开计数。
    #[jvm_boundary]
    pub fn getFindClasses() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn getFindClassTime() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn getReadClassBytesTime() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn getParentDelegationTime() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn getZipFileCount() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn getZipFileOpenTime() -> Result<PerfCounter> {
        Self::newPerfCounter(String::default())
    }

    #[jvm_boundary]
    pub fn __impl_get(&self) -> Result<i64> {
        Ok(0)
    }

    #[jvm_boundary]
    pub fn __impl_set(&self, _newValue: i64) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_add(&self, _value: i64) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_increment(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_addTime(&self, _interval: i64) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_addElapsedTimeFrom(&self, _startTime: i64) -> Result<()> {
        Ok(())
    }
}
