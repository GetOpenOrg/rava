//! `java/net/Inet4Address` 的 native 方法（与生成的同名文件共置）。

use crate::java::net::Inet4Address__Methods as _;
use crate::prelude::*;
use super::inet4_address::Inet4Address;
use super::__Inet4AddressHw;

impl __Inet4AddressHw for Inet4Address {
    /// native `init()`：缓存 JNI 字段 ID——无需。
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn init() -> Result<()> {
        Ok(())
    }
}
