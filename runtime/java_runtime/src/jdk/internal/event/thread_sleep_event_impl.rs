use crate::prelude::*;
use super::ThreadSleepEvent;

// jdk.internal.event.ThreadSleepEvent（JFR 线程休眠事件的 java.base 占位层）：
// 事件系统未启用（原生二进制无 JFR），按调用链按需实现，其余保持存根。

impl ThreadSleepEvent {
    /// `isTurnedOn()Z`：事件是否启用。java.base 层的占位实现恒 false
    ///（JFR 启用时会重定义本类；无 JFR 即不可用）——`Thread.sleep` 的
    /// beforeSleep/afterSleep 路径据此整体短路。
    #[jvm_boundary]
    pub fn isTurnedOn() -> Result<bool> {
        Ok(false)
    }
}
