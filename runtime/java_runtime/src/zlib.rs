//! 系统 zlib 的运行期绑定（java.util.zip.Inflater / Deflater 的 native 层共用）。
//!
//! JDK 的 Inflater.c / Deflater.c 直接调用 zlib；压缩输出需与 JVM 逐字节一致，只能使用
//! 同一实现。首次使用时 `dlopen` 系统 zlib（Linux `libz.so.1`、macOS `libz.1.dylib`）——
//! 不要求构建机安装 zlib 开发包（`-lz` 链接需要 `libz.so` 符号链接）。库不可得时
//! 由调用方抛 InternalError（与 JVM 缺 libzip 时的失败形态同类）。

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void, CStr};
use std::sync::OnceLock;

pub const Z_OK: c_int = 0;
pub const Z_STREAM_END: c_int = 1;
pub const Z_NEED_DICT: c_int = 2;
pub const Z_STREAM_ERROR: c_int = -2;
pub const Z_DATA_ERROR: c_int = -3;
pub const Z_MEM_ERROR: c_int = -4;
pub const Z_BUF_ERROR: c_int = -5;
pub const Z_PARTIAL_FLUSH: c_int = 1;
pub const Z_DEFLATED: c_int = 8;
pub const MAX_WBITS: c_int = 15;
pub const DEF_MEM_LEVEL: c_int = 8;

/// zlib.h 的 `z_stream`（布局与 C 定义逐字段一致）。
#[repr(C)]
pub struct ZStream {
    pub next_in: *const u8,
    pub avail_in: c_uint,
    pub total_in: c_ulong,
    pub next_out: *mut u8,
    pub avail_out: c_uint,
    pub total_out: c_ulong,
    pub msg: *const c_char,
    pub state: *mut c_void,
    pub zalloc: *const c_void,
    pub zfree: *const c_void,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: c_ulong,
    pub reserved: c_ulong,
}

impl ZStream {
    /// 零初始化（zalloc / zfree / opaque 为 NULL → zlib 使用缺省分配器）。
    pub fn zeroed() -> Self {
        // SAFETY: 全零是 z_stream 的合法初值（zlib 文档要求调用 init 前置零 / 设 NULL）
        unsafe { std::mem::zeroed() }
    }

    /// `strm->msg`（可为 NULL）。
    pub fn msg(&self) -> Option<String> {
        if self.msg.is_null() {
            None
        } else {
            // SAFETY: zlib 保证 msg 非空时指向静态 NUL 结尾字符串
            Some(unsafe { CStr::from_ptr(self.msg) }.to_string_lossy().into_owned())
        }
    }
}

type InitInflate = unsafe extern "C" fn(*mut ZStream, c_int, *const c_char, c_int) -> c_int;
type InitDeflate = unsafe extern "C" fn(*mut ZStream, c_int, c_int, c_int, c_int, c_int, *const c_char, c_int) -> c_int;
type StreamOp = unsafe extern "C" fn(*mut ZStream, c_int) -> c_int;
type StreamOnly = unsafe extern "C" fn(*mut ZStream) -> c_int;
type SetDict = unsafe extern "C" fn(*mut ZStream, *const u8, c_uint) -> c_int;
type Params = unsafe extern "C" fn(*mut ZStream, c_int, c_int) -> c_int;
type Version = unsafe extern "C" fn() -> *const c_char;

/// zlib 函数表。
pub struct Zlib {
    pub inflate_init2: InitInflate,
    pub inflate: StreamOp,
    pub inflate_end: StreamOnly,
    pub inflate_reset: StreamOnly,
    pub inflate_set_dictionary: SetDict,
    pub deflate_init2: InitDeflate,
    pub deflate: StreamOp,
    pub deflate_end: StreamOnly,
    pub deflate_reset: StreamOnly,
    pub deflate_params: Params,
    pub deflate_set_dictionary: SetDict,
    pub version: *const c_char,
}

// 函数指针与静态版本串在进程内不变，可跨线程共享
unsafe impl Send for Zlib {}
unsafe impl Sync for Zlib {}

const LIB_NAMES: &[&str] = &["libz.so.1", "libz.1.dylib", "/usr/lib/libz.1.dylib", "libz.so", "libz.dylib"];

fn load() -> Option<Zlib> {
    // SAFETY: dlopen / dlsym 按名查找 zlib 导出符号，签名与 zlib.h 声明一致
    unsafe {
        let handle = LIB_NAMES.iter().find_map(|n| {
            let c = std::ffi::CString::new(*n).ok()?;
            let h = libc::dlopen(c.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL);
            if h.is_null() { None } else { Some(h) }
        })?;
        macro_rules! sym {
            ($name:literal, $ty:ty) => {{
                let p = libc::dlsym(handle, concat!($name, "\0").as_ptr() as *const c_char);
                if p.is_null() { return None; }
                std::mem::transmute::<*mut c_void, $ty>(p)
            }};
        }
        let version: Version = sym!("zlibVersion", Version);
        Some(Zlib {
            inflate_init2: sym!("inflateInit2_", InitInflate),
            inflate: sym!("inflate", StreamOp),
            inflate_end: sym!("inflateEnd", StreamOnly),
            inflate_reset: sym!("inflateReset", StreamOnly),
            inflate_set_dictionary: sym!("inflateSetDictionary", SetDict),
            deflate_init2: sym!("deflateInit2_", InitDeflate),
            deflate: sym!("deflate", StreamOp),
            deflate_end: sym!("deflateEnd", StreamOnly),
            deflate_reset: sym!("deflateReset", StreamOnly),
            deflate_params: sym!("deflateParams", Params),
            deflate_set_dictionary: sym!("deflateSetDictionary", SetDict),
            version: version(),
        })
    }
}

/// 进程内唯一的 zlib 绑定；库不可得返回 None。
pub fn zlib() -> Option<&'static Zlib> {
    static ZLIB: OnceLock<Option<Zlib>> = OnceLock::new();
    ZLIB.get_or_init(load).as_ref()
}

/// Java 侧持有的流句柄（`long address`）→ z_stream 指针。
pub fn stream(addr: i64) -> *mut ZStream {
    addr as usize as *mut ZStream
}

/// 新建堆上 z_stream，返回其地址（`init` 的返回值）。
pub fn alloc_stream() -> i64 {
    Box::into_raw(Box::new(ZStream::zeroed())) as usize as i64
}

/// 释放 `alloc_stream` 分配的 z_stream（`end` 在 zlib 的 *End 之后调用）。
pub fn free_stream(addr: i64) {
    if addr != 0 {
        // SAFETY: addr 由 alloc_stream 的 Box::into_raw 产生且只释放一次（Java 侧 end 后置零）
        unsafe { drop(Box::from_raw(stream(addr))) };
    }
}

/// 结果编码（Inflater.c / Deflater.c 同形）：inputUsed | outputUsed << 31 | flag62 << 62 | flag63 << 63。
pub fn pack(input_used: i32, output_used: i32, flag62: bool, flag63: bool) -> i64 {
    (input_used as i64) | ((output_used as i64) << 31) | ((flag62 as i64) << 62) | ((flag63 as i64) << 63)
}
