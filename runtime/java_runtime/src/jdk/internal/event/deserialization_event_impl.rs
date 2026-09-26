use crate::prelude::*;
use super::DeserializationEvent;

// jdk.internal.event.DeserializationEvent（JFR 反序列化过滤事件的 java.base 占位层）：
// 事件系统未启用（原生二进制无 JFR），按调用链按需实现，其余保持存根。

impl DeserializationEvent {
    /// `<init>()V`：`ObjectInputStream.filterCheck` 每次过滤检查先构造事件再查
    /// `shouldCommit()`（继承自 `Event`，占位层恒 false）——字段赋值与 commit 整体短路，
    /// 事件无运行期状态可观测，缺省构造即可。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }
}
