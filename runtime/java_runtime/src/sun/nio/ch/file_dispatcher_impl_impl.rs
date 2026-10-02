//! `sun/nio/ch/FileDispatcherImpl` 的 ACC_NATIVE（类 1，libnio 平台目录下的 `FileDispatcherImpl.c`）。
//! 其余方法按字节码翻译。
//!
//! 该类按平台各有一份字节码：Linux 版的 `<clinit>` 为 `IOUtil.load(); init0();`；macOS 版没有 `init0`。
//! 下面的 native 落点只在声明它的平台上被翻译体调用，另一平台上只是多出一个未被调用的固有函数。

use crate::prelude::*;
use super::file_dispatcher_impl::FileDispatcherImpl;

impl FileDispatcherImpl {
    /// native `init0()`（Linux）：JNI 侧用 `dlsym(RTLD_DEFAULT, "copy_file_range")` 取函数指针，
    /// 留给 transferTo0 / transferFrom0 的快速路径使用；取不到时退回 sendfile。原生二进制直接链接 libc，
    /// 不需要运行期查找函数指针，因此这里没有需要初始化的状态。
    #[jvm_native]
    pub fn init0() -> Result<()> {
        Ok(())
    }
}
