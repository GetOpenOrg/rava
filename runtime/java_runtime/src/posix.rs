//! POSIX 系统调用的窄封装（std 无等价物的部分）：access(2)、statvfs(3)、pathconf(3)。
//! 消费方：`UnixFileSystem.checkAccess0 / getSpace0 / getNameMax0`（java.io.File）与
//! `UnixNativeDispatcher.access`（java.nio.file.Files.isReadable 等）。

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn c_path(path: &Path) -> Option<CString> {
    CString::new(path.as_os_str().as_bytes()).ok()
}

/// access(2)：0 = 允许，否则 errno（路径含 NUL → ENOENT）。
pub fn access(path: &Path, amode: i32) -> i32 {
    let Some(c) = c_path(path) else { return libc::ENOENT };
    if unsafe { libc::access(c.as_ptr(), amode) } == 0 {
        0
    } else {
        std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EACCES)
    }
}

/// statvfs(3) 的三种空间量（字节）：0 = 总量、1 = 空闲、2 = 非特权可用
/// （java.io.FileSystem SPACE_TOTAL / SPACE_FREE / SPACE_USABLE）。失败 → 0（JDK 同）。
pub fn space(path: &Path, kind: i32) -> i64 {
    let Some(c) = c_path(path) else { return 0 };
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut st) } != 0 {
        return 0;
    }
    let frsize = st.f_frsize as u64;
    let blocks = match kind {
        0 => st.f_blocks as u64,
        1 => st.f_bfree as u64,
        2 => st.f_bavail as u64,
        _ => return 0,
    };
    frsize.saturating_mul(blocks).min(i64::MAX as u64) as i64
}

/// pathconf(_PC_NAME_MAX)：失败或无限制时回落 255（JDK UnixFileSystem 同）。
pub fn name_max(path: &Path) -> i64 {
    let Some(c) = c_path(path) else { return 255 };
    let v = unsafe { libc::pathconf(c.as_ptr(), libc::_PC_NAME_MAX) };
    if v <= 0 { 255 } else { v as i64 }
}

/// `native.encoding` / `sun.jnu.encoding`：宿主区域的 codeset（HotSpot 取
/// nl_langinfo(CODESET)）。macOS 恒 UTF-8；Linux 按 LC_ALL / LC_CTYPE / LANG 的
/// codeset 段，C / POSIX 区域为 glibc 的 `ANSI_X3.4-1968`。
pub fn native_encoding() -> std::string::String {
    if cfg!(target_os = "macos") {
        return std::string::String::from("UTF-8");
    }
    let raw = ["LC_ALL", "LC_CTYPE", "LANG"].iter()
        .filter_map(|name| std::env::var(name).ok())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let codeset = raw.split('@').next().unwrap_or("").split('.').nth(1).unwrap_or("");
    match codeset.to_ascii_lowercase().replace('-', "").as_str() {
        "" => std::string::String::from("ANSI_X3.4-1968"),
        "utf8" => std::string::String::from("UTF-8"),
        _ => codeset.to_owned(),
    }
}
