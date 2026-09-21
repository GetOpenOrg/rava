use crate::prelude::*;
use super::*;

// 内部边界类 java.lang.StringUTF16：按调用链按需实现，其余保持 panic 存根。

impl StringUTF16 {
    /// `StringUTF16.isBigEndian:()Z`：宿主平台字节序（JDK native 同义——
    /// `BIG_ENDIAN` 编译期常量的运行时查询形态）。消费方 `<clinit>` 据此设置
    /// HI/LO_BYTE_SHIFT（compact strings 的 UTF-16 通道字节序），值不进入
    /// 可观测输出；小端平台（x86-64 / aarch64 Linux 与 macOS）为 false。
    #[jvm_native]
    pub fn isBigEndian() -> Result<bool> {
        Ok(cfg!(target_endian = "big"))
    }
}
