//! `java.lang.ProcessImpl`（Unix）的 native 层。

use crate::prelude::*;
use super::*;

impl ProcessImpl {
    /// native `init()`（`<clinit>` 末尾调用）：JDK ProcessImpl_md.c 在此缓存 PATH 拆分
    /// 结果（parentPathv）与 JNI 字段 ID，供 forkAndExec 解析可执行文件。原生二进制无 JNI
    /// 字段缓存；PATH 在 forkAndExec 落地时按调用时刻的环境解析（与 JDK 在 <clinit> 快照
    /// 的差异仅在运行期修改 PATH 时可观察，而 Java 无修改进程环境的 API）。无可观察状态。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }
}
