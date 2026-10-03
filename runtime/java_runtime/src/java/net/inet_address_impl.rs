//! `java/net/InetAddress` 的 ACC_NATIVE（类 1，libnet `InetAddress.c` / `net_util.c`）。其余方法（含
//! `<clinit>`）按字节码翻译。
//!
//! 注意：生成类 `java/net/InetAddressImpl`（接口）的文件名主干与本文件相同，生成器把它落在
//! `inet_address_impl_t.rs`。

use crate::prelude::*;
use super::inet_address::InetAddress;

impl InetAddress {
    /// native `init()`：缓存 JNI 类与字段 ID——无需。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }

    /// native `isIPv4Available()`：能创建 AF_INET 套接字。
    #[jvm_native]
    pub fn isIPv4Available() -> Result<bool> {
        Ok(crate::net_posix::ipv4_supported())
    }

    /// native `isIPv6Supported()`：协议栈支持 IPv6（不看 preferIPv4Stack，Java 侧自行处理）。
    #[jvm_native]
    pub fn isIPv6Supported() -> Result<bool> {
        Ok(crate::net_posix::ipv6_supported())
    }
}
