use crate::prelude::*;
use super::ThreadSleepEvent;

// jdk.internal.event.ThreadSleepEvent（JFR 线程休眠事件的 java.base 占位层）：
// 事件系统未启用（原生二进制无 JFR），按调用链按需实现，其余保持存根。

impl ThreadSleepEvent {
    /// `<init>()V`：JDK25 的 `Thread.beforeSleep` 先构造事件再查 `isEnabled()`
    ///（JDK21 为静态 `isTurnedOn()` 前置短路，不构造）。事件无运行期状态可观测
    ///（isEnabled 恒 false，time / begin / commit 均不触达），缺省构造即可。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `isTurnedOn()Z`：事件是否启用。java.base 层的占位实现恒 false
    ///（JFR 启用时会重定义本类；无 JFR 即不可用）——`Thread.sleep` 的
    /// beforeSleep/afterSleep 路径据此整体短路。
    #[jvm_boundary]
    pub fn isTurnedOn() -> Result<bool> {
        Ok(false)
    }
}
