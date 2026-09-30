//! `java/net/Inet4Address` 的 native 方法（与生成的同名文件共置）。

use crate::prelude::*;
use super::inet4_address::Inet4Address;

impl Inet4Address {
    /// native `init()`：缓存 JNI 字段 ID——无需。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }
}
