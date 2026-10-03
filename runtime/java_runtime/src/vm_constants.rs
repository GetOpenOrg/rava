//! VM 注入常量的取值（handwritten-boundary.md 类 ③：VM 注入的状态）。
//!
//! HotSpot 在类初始化期间把平台 / 对象布局常量写入若干 static 字段（`UnsafeConstantsFixup`
//! 改写 `UnsafeConstants` 五个字段；`Unsafe.<clinit>` 经 VM 原语 `arrayBaseOffset0` /
//! `arrayIndexScale0` 取数组布局）。这些字段登记在 `vm_intrinsics.toml`
//! `[vm_constants.injected_statics]`：生成器把字段发为读本模块取值的访问器（按字段描述符做
//! 数值宽度转换，字节码对该字段的写入被 VM 值覆盖），分析器不按字节码初值折叠其读取。
//! 本模块只给出取值，字段清单与类型适配由清单与生成器承载。

/// 本地指针字节数（HotSpot `oopSize`）：64 位平台 8，32 位平台 4。
pub fn address_size() -> i64 {
    std::mem::size_of::<usize>() as i64
}

/// 内存页字节数（HotSpot `os::vm_page_size()`，即 `sysconf(_SC_PAGESIZE)`）。
pub fn page_size() -> i64 {
    // SAFETY: sysconf 只查询系统常量
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as i64 }
}

/// 宿主字节序是否大端。
pub fn big_endian() -> bool {
    cfg!(target_endian = "big")
}

/// 是否支持非对齐访问（HotSpot `UseUnalignedAccesses`）。本运行时的原始内存读写
/// （`native_memory::read` / `write`）按字节复制，任何平台上非对齐访问都成立。
pub fn unaligned_access() -> bool {
    true
}

/// 数据缓存行回写粒度（HotSpot `VM_Version::data_cache_line_flush_size()`）。本运行时不提供
/// `Unsafe.writeback0` 族回写原语，按「不支持」报 0（HotSpot 在无回写能力平台上同值）。
pub fn data_cache_line_flush_size() -> i64 {
    0
}

/// 数组首元素偏移（全部数组类同值，与 `native_memory` 的寻址解码同一常量）。
pub fn array_base_offset() -> i64 {
    crate::native_memory::ARRAY_BASE_OFFSET
}

/// 数组元素寻址 stride（字节），按元素描述符首字符：引用元素（`L` / `[`）取压缩指针 4。
pub fn array_index_scale(elem: char) -> i64 {
    match elem {
        'Z' | 'B' => 1,
        'C' | 'S' => 2,
        'J' | 'D' => 8,
        _ => 4,
    }
}
