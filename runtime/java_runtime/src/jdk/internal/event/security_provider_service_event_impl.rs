use crate::prelude::*;
use super::SecurityProviderServiceEvent;

// jdk.internal.event.SecurityProviderServiceEvent（JFR 安全服务查找事件的 java.base 占位层）：
// 事件系统未启用（原生二进制无 JFR），按调用链按需实现，其余保持存根。

impl SecurityProviderServiceEvent {
    /// `isTurnedOn()Z`：事件是否启用。java.base 占位实现恒 false——`Provider.getService`
    /// 据此跳过事件构造与 commit。
    #[jvm_boundary]
    pub fn isTurnedOn() -> Result<bool> {
        Ok(false)
    }
}
