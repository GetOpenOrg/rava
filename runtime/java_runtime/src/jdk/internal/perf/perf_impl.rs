//! `jdk/internal/perf/Perf` 的 native 方法（准入类别①：ACC_NATIVE）。
//!
//! HotSpot 把计数器放在 jvmstat 共享内存（hsperfdata），供 jstat 等外部进程读取。原生二进制
//! 无 jvmstat：计数器只需进程内可读写，外部不可观测。`PerfCounter` 的 get / set / add 等语义
//! 由其字节码经 `ByteBuffer.asLongBuffer()` 承载，这里只提供存储。

use crate::prelude::*;
use super::perf::Perf;
use crate::java::nio::{ByteBuffer, ByteOrder};

impl Perf {
    /// native registerNatives：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `createLong(name, variability, units, value)`：分配一个 long 计数器的存储并以
    /// `value` 初始化（本机字节序写入，与 HotSpot 一致），返回覆盖该 8 字节的缓冲（默认
    /// BIG_ENDIAN 序，调用方 PerfCounter 自行改为 nativeOrder）。名字 / 可变性 / 单位只影响
    /// jvmstat 元数据，进程内不可观测。
    #[jvm_native(upcalls = "java/nio/ByteBuffer.allocate:(I)Ljava/nio/ByteBuffer; java/nio/ByteOrder.nativeOrder:()Ljava/nio/ByteOrder; java/nio/ByteBuffer.order:(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer; java/nio/ByteBuffer.putLong:(IJ)Ljava/nio/ByteBuffer;")]
    pub fn createLong(&self, _name: String, _variability: i32, _units: i32, value: i64) -> Result<ByteBuffer> {
        let bb = ByteBuffer::allocate(8)?;
        if value != 0 {
            bb.order_byteorder(ByteOrder::nativeOrder()?)?;
            bb.putLong_i_l(0, value)?;
            bb.order_byteorder(ByteOrder::BIG_ENDIAN()?)?;
        }
        Ok(bb)
    }
}
