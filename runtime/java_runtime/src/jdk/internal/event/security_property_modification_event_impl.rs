use crate::prelude::*;
use super::SecurityPropertyModificationEvent;

// jdk.internal.event.SecurityPropertyModificationEvent（JFR 安全属性修改事件的 java.base 占位层）：
// 事件系统未启用（原生二进制无 JFR），按调用链按需实现，其余保持存根。

impl SecurityPropertyModificationEvent {
    /// `<init>()V`：`Security.setProperty` 先构造事件再查 `shouldCommit()`（继承自 Event，
    /// 占位实现恒 false，事件字段与 commit 均不触达），缺省构造即可。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }
}
