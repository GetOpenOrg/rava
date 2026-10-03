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

/// `os.name`（HotSpot SystemProps 的平台名）。
pub fn os_name() -> &'static str {
    if cfg!(target_os = "macos") { "Mac OS X" }
    else if cfg!(target_os = "linux") { "Linux" }
    else { std::env::consts::OS }
}

/// `os.arch`（HotSpot 的架构名：x86_64 → amd64）。
pub fn os_arch() -> &'static str {
    if cfg!(target_arch = "aarch64") { "aarch64" }
    else if cfg!(target_arch = "x86_64") { "amd64" }
    else { std::env::consts::ARCH }
}

/// `os.version`：内核发行号（HotSpot 取 uname(2).release；/proc 为同一内核数据源）。
pub fn os_release() -> std::string::String {
    if let Ok(s) = std::fs::read_to_string("/proc/sys/kernel/osrelease") {
        return s.trim().to_owned();
    }
    std::process::Command::new("uname").arg("-r").output().ok()
        .and_then(|o| std::string::String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_owned())
        .unwrap_or_default()
}

/// `user.language` / `user.country`：宿主区域（HotSpot initPhase1 的 SystemProps.Raw 取
/// LC_ALL / LC_MESSAGES / LANG，形如 `en_US.UTF-8`）；C / POSIX / 未设置时为 en / 空。
pub fn locale() -> (std::string::String, std::string::String) {
    let raw = ["LC_ALL", "LC_MESSAGES", "LANG"].iter()
        .filter_map(|name| std::env::var(name).ok())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    let tag = raw.split(['.', '@']).next().unwrap_or("");
    if tag.is_empty() || tag == "C" || tag == "POSIX" {
        return ("en".to_owned(), std::string::String::new());
    }
    let mut parts = tag.splitn(2, '_');
    let language = parts.next().unwrap_or("en").to_owned();
    let country = parts.next().unwrap_or("").to_owned();
    (language, country)
}

/// uid → passwd 条目的用户名（getpwuid_r）；无对应条目 → None。
pub fn passwd_name(uid: u32) -> Option<std::string::String> {
    let mut buf = vec![0 as libc::c_char; 4096];
    // SAFETY: passwd 全零为合法初值；getpwuid_r 只写入 pwd / buf，result 指向 pwd 或为 null
    let mut pwd: libc::passwd = unsafe { std::mem::zeroed() };
    let mut result: *mut libc::passwd = std::ptr::null_mut();
    let r = unsafe { libc::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut result) };
    if r == 0 && !result.is_null() && !pwd.pw_name.is_null() {
        // SAFETY: 成功时 pw_name 是 buf 内以 NUL 结尾的字符串
        let name = unsafe { std::ffi::CStr::from_ptr(pwd.pw_name) };
        return Some(name.to_string_lossy().into_owned());
    }
    None
}

/// 当前进程实际用户的用户名：`user.name` 系统属性（JDK java_props_md.c：`getpwuid(getuid())->pw_name`，
/// 无条目时为 `"?"`；不读 `USER` 环境变量）。
pub fn current_user_name() -> std::string::String {
    // SAFETY: getuid 无副作用
    passwd_name(unsafe { libc::getuid() }).unwrap_or_else(|| std::string::String::from("?"))
}

/// JVM 缺省 MaxHeapSize（物理内存 1/4；不可得时 Long.MAX_VALUE）：`Runtime.maxMemory` 与
/// `VM.directMemory`（未指定 MaxDirectMemorySize 时取 maxMemory）同源。
pub fn default_max_heap() -> i64 {
    // SAFETY: sysconf 只查询系统常量
    let (pages, page) = unsafe { (libc::sysconf(libc::_SC_PHYS_PAGES), libc::sysconf(libc::_SC_PAGESIZE)) };
    if pages > 0 && page > 0 { (pages as u64 * page as u64 / 4) as i64 } else { i64::MAX }
}
