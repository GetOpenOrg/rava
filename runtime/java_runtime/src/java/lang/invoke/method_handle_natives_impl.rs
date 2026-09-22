//! `java/lang/invoke/MethodHandleNatives` 手写伴生：公开 API 类的 ACC_NATIVE
//! 方法（K-3a 共置形态）。HotSpot 里本类是 method handle 机制与 VM 的 JNI
//! 边界；原生二进制没有 VM 元数据层——resolve 族按「Rust 侧静态注册表」
//! 承载（build.rs 方法/字段元数据表，与 Class.getDeclaredField 同源）。

use crate::prelude::*;
use super::method_handle_natives::MethodHandleNatives;

impl MethodHandleNatives {
    /// native `registerNatives()`：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }
}
