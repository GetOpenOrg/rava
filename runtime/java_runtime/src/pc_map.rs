//! 地址 → Java 帧表（运行时基础设施；二进制体积 B2，`docs/plans/2026-10-04-binary-size.md`）。
//!
//! 原生二进制没有 HotSpot 的帧元数据（Method* + bci）。等价物是链接期预解析的地址表，对标 Go pclntab /
//! GraalVM CodeInfo：
//! - 链接器包装 `rava-link` 链接一次，读出 DWARF 行号与内联链，套用成帧规则（闭包帧不成帧，块外 / 序言
//!   不成帧，按发射层旁路行表 `closure_input/frame_lines.json` 对位），得出「代码地址区间 → Java 帧序列」；
//! - 编成表对象，带上它再链接一次，进本节（Mach-O `__DATA,__rava_pcmap`，ELF `rava_pcmap`），地址记为
//!   相对锚点 [`__rava_pc_anchor`] 的偏移（ASLR 下整体平移不变）。
//!
//! 运行时取帧只用 `_Unwind_Backtrace` 拿返回地址，再按地址查本表。不读 DWARF、符号表或构建目录，
//! 二进制 strip 后照常工作。
//!
//! 表字形（与 `rava-link` 的编码一一对应）：`RAVAPCM1` + u32 LE 池长 + 池（`meta_codec` 池格式）+
//! u32 LE 流长 + 流。流依次为：
//! - 方法项 `[类, 方法名, 描述符, 源文件, 标志字, 注解]`；
//! - 帧序列 `[[方法下标, Java 行码（0 native / 1 无行号 / 行 + 2）]]`，自内向外；
//! - 区间 `[起点, 帧序列码（0 = 不成帧，否则下标 + 1）]`，起点首项为相对锚点的 zigzag 偏移，其后为增量。

use crate::meta::LineMethod;

/// 本节的占位项：保证节在第一次链接中就存在（节边界符号可解析），全零、不含表头
#[used]
#[cfg_attr(target_vendor = "apple", link_section = "__DATA,__rava_pcmap")]
#[cfg_attr(not(target_vendor = "apple"), link_section = "rava_pcmap")]
static mut PLACEHOLDER: [u8; 8] = [0; 8];

#[cfg(target_vendor = "apple")]
extern "C" {
    #[link_name = "\x01section$start$__DATA$__rava_pcmap"]
    static SECTION_START: u8;
    #[link_name = "\x01section$end$__DATA$__rava_pcmap"]
    static SECTION_END: u8;
}

#[cfg(not(target_vendor = "apple"))]
extern "C" {
    #[link_name = "__start_rava_pcmap"]
    static SECTION_START: u8;
    #[link_name = "__stop_rava_pcmap"]
    static SECTION_END: u8;
}

/// 地址锚点：表内地址都是相对它的偏移。`rava-link` 按此符号名定位它，也凭它的存在识别 Java 程序
#[no_mangle]
#[inline(never)]
pub extern "C" fn __rava_pc_anchor() {}

const MAGIC: &[u8; 8] = b"RAVAPCM1";
/// Java 行码：native 方法帧（`StackTraceElement.lineNumber = -2`）
pub const LINE_NATIVE: i32 = -2;
/// Java 行码：无行号的手写方法帧（`-1`）
pub const LINE_UNKNOWN: i32 = -1;

/// 解码后的表
pub struct PcMap {
    pub methods: &'static [LineMethod],
    /// 帧序列：(方法下标, Java 行；[`LINE_NATIVE`] / [`LINE_UNKNOWN`] 为哨兵)，自内向外
    pub lists: &'static [&'static [(u32, i32)]],
    /// (相对锚点的起点, 帧序列下标；`u32::MAX` = 不成帧)，按起点升序
    pub ranges: &'static [(i64, u32)],
}

/// 节内容（占位项与表对象按链接器拼接序首尾相接）
fn section() -> &'static [u8] {
    // SAFETY：两个符号是链接器为本节合成的首尾边界，节在映像内常驻且运行期不写
    unsafe {
        let start = std::ptr::addr_of!(SECTION_START);
        let end = std::ptr::addr_of!(SECTION_END);
        std::slice::from_raw_parts(start, end as usize - start as usize)
    }
}

/// 节内的表字节（池, 流）；没有表（未经 `rava-link` 链接）→ None
fn blob() -> Option<(&'static [u8], &'static [u8])> {
    let s = section();
    let at = s.windows(MAGIC.len()).position(|w| w == MAGIC)? + MAGIC.len();
    let u32_at = |i: usize| u32::from_le_bytes([s[i], s[i + 1], s[i + 2], s[i + 3]]) as usize;
    let pool_len = u32_at(at);
    let pool = &s[at + 4..at + 4 + pool_len];
    let at = at + 4 + pool_len;
    let stream_len = u32_at(at);
    Some((pool, &s[at + 4..at + 4 + stream_len]))
}

/// 表（进程内首次取帧时解码一次）
pub fn table() -> &'static PcMap {
    static CELL: std::sync::OnceLock<PcMap> = std::sync::OnceLock::new();
    CELL.get_or_init(|| match blob() {
        Some((pool, stream)) => crate::meta_codec::pc_map(pool, stream),
        // 未经 rava-link 链接的二进制：构建流程缺陷，取帧即报错，不静默给出空栈
        None => panic!("pc_map: 二进制缺少 __rava_pcmap 地址表（须经 rava compile 的链接器包装 rava-link 链接）"),
    })
}

/// 返回地址 → 该物理帧上的 Java 帧序列（自内向外）
pub fn frames_at(return_address: usize) -> &'static [(u32, i32)] {
    let map = table();
    // 返回地址指向调用指令之后：减 1 落回调用指令本身（与 DWARF 符号化同口径）
    let offset = return_address as i64 - 1 - __rava_pc_anchor as usize as i64;
    let at = map.ranges.partition_point(|r| r.0 <= offset);
    match at.checked_sub(1).map(|i| map.ranges[i].1) {
        Some(list) if list != u32::MAX => map.lists[list as usize],
        _ => &[],
    }
}

/// 当前线程调用栈的返回地址（自栈顶向下），经系统 unwinder（`_Unwind_Backtrace`，按 unwind 表）
pub fn return_addresses() -> Vec<usize> {
    use std::ffi::c_void;

    #[repr(C)]
    struct UnwindContext {
        _opaque: [u8; 0],
    }
    type TraceFn = extern "C" fn(*mut UnwindContext, *mut c_void) -> i32;
    extern "C" {
        fn _Unwind_Backtrace(trace: TraceFn, arg: *mut c_void) -> i32;
        fn _Unwind_GetIP(ctx: *mut UnwindContext) -> usize;
    }
    extern "C" fn push(ctx: *mut UnwindContext, arg: *mut c_void) -> i32 {
        // SAFETY：arg 即下方传入的 Vec 指针，回调在 _Unwind_Backtrace 返回前同步执行
        let pcs = unsafe { &mut *(arg as *mut Vec<usize>) };
        let ip = unsafe { _Unwind_GetIP(ctx) };
        if ip != 0 {
            pcs.push(ip);
        }
        0 // _URC_NO_REASON：继续回溯
    }
    let mut pcs: Vec<usize> = Vec::with_capacity(64);
    // SAFETY：回调只把地址写入 pcs
    unsafe { _Unwind_Backtrace(push, &mut pcs as *mut Vec<usize> as *mut c_void) };
    pcs
}
