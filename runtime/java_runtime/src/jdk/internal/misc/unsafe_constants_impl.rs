//! `jdk/internal/misc/UnsafeConstants` 手写伴生：VM 注入的平台常量（准入第 ③ 类：VM 注入状态）。
//!
//! JDK 源码注释：「The JVM injects values into all the static fields of this class during class
//! initialization」——字节码里的 `<clinit>` 只写占位缺省值，真实值由 HotSpot 按宿主平台写入。
//! 原生二进制按编译目标平台给出同一组值；消费方：`Unsafe.pageSize` / `unalignedAccess` /
//! `isBigEndian` / `addressSize` / `dataCacheLineFlushSize`（Bits / DirectByteBuffer / ByteBuffer 视图）。

use crate::prelude::*;
use super::unsafe_constants::UnsafeConstants;

impl UnsafeConstants {
    /// 本机指针宽度（字节）：HotSpot `oopSize`。
    #[jvm_boundary]
    pub fn ADDRESS_SIZE0() -> Result<i32> {
        Ok(std::mem::size_of::<usize>() as i32)
    }

    /// 内存页大小：HotSpot `os::vm_page_size()`（sysconf(_SC_PAGESIZE)）。
    #[jvm_boundary]
    pub fn PAGE_SIZE() -> Result<i32> {
        // SAFETY: sysconf 只读系统配置
        Ok(unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as i32)
    }

    /// 本机字节序是否大端：HotSpot `Endian::is_Java_byte_ordering_different()` 的反面。
    #[jvm_boundary]
    pub fn BIG_ENDIAN() -> Result<bool> {
        Ok(cfg!(target_endian = "big"))
    }

    /// 平台是否允许非对齐访问：HotSpot `UseUnalignedAccesses` 的平台缺省（x86 / x86_64 / aarch64 为 true）。
    #[jvm_boundary]
    pub fn UNALIGNED_ACCESS() -> Result<bool> {
        Ok(cfg!(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))
    }

    /// 数据缓存行回写粒度：0 = 不支持回写（MappedByteBuffer 的 NVM 同步映射不可用）。原生二进制未实现
    /// `Unsafe.writeback0` 族，按「不支持」给出，与 HotSpot 在不支持 CLWB / DC CVAP 的平台上的取值一致。
    #[jvm_boundary]
    pub fn DATA_CACHE_LINE_FLUSH_SIZE() -> Result<i32> {
        Ok(0)
    }
}
