//! `jdk/internal/vm/ContinuationSupport` 手写伴生：VM 能力探测面的静态回答。

use crate::prelude::*;
use super::continuation_support::ContinuationSupport;

impl ContinuationSupport {
    /// `isSupported()Z`：JVM 是否支持 continuation（虚拟线程的承载机制，
    /// VirtualThread 创建路径的档位开关）。单线程协作调度下虚拟线程任务经
    /// 就绪队列泵分派（S-11 等价档位），continuation 语义由调度器承载 →
    /// 恒真（与 availableProcessors=1 的档位报告一致）。
    #[jvm_boundary]
    pub fn isSupported() -> Result<bool> {
        Ok(true)
    }
}
