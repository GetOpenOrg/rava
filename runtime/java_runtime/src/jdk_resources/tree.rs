//! `${java.home}` 虚拟树的结构：文件清单、目录、stat 与目录流（opendir / fdopendir / readdir / closedir）。
//!
//! 树由 [`FILES`] 定死（构建期嵌入，只读）；目录是文件路径的各级前缀。stat 的形态：目录
//! `S_IFDIR | 0555`、文件 `S_IFREG | 0444`、`st_size` 为内容长度、时间戳为 0（嵌入数据无宿主时间）。
//! 目录流句柄为负的 i64（真实 `DIR*` 是正地址，两空间不碰撞）。

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Mutex;

use super::JAVA_RUNTIME_HOME;

/// 虚拟树中的文件（相对 java.home）。增删须同步 [`content`]。
const FILES: &[&str] = &[
    "conf/security/java.security",
    "conf/security/policy/limited/default_US_export.policy",
    "conf/security/policy/limited/default_local.policy",
    "conf/security/policy/limited/exempt_local.policy",
    "conf/security/policy/unlimited/default_US_export.policy",
    "conf/security/policy/unlimited/default_local.policy",
    "lib/modules",
    "lib/tzdb.dat",
];

/// JCE 管辖策略文件（参考 JDK 21 / 25 `conf/security/policy`，两版本逐字节相同）：由翻译的
/// `JceSecurity.setupJurisdictionPolicies` 按 `crypto.policy`（java.security 中为 unlimited）选目录读取
static POLICY_LIMITED_US_EXPORT: &[u8] = include_bytes!("policy/limited/default_US_export.policy");
static POLICY_LIMITED_LOCAL: &[u8] = include_bytes!("policy/limited/default_local.policy");
static POLICY_LIMITED_EXEMPT: &[u8] = include_bytes!("policy/limited/exempt_local.policy");
static POLICY_UNLIMITED_US_EXPORT: &[u8] = include_bytes!("policy/unlimited/default_US_export.policy");
static POLICY_UNLIMITED_LOCAL: &[u8] = include_bytes!("policy/unlimited/default_local.policy");

/// 绝对路径 → 相对 java.home 的路径（根为 ""）；树外为 None
pub fn relative(path: &str) -> Option<&str> {
    let rest = path.strip_prefix(JAVA_RUNTIME_HOME)?;
    if rest.is_empty() {
        return Some("");
    }
    rest.strip_prefix('/').map(|r| r.trim_end_matches('/'))
}

/// 文件内容；目录与不存在的路径为 None
pub fn content(rel: &str) -> Option<&'static [u8]> {
    Some(match rel {
        "conf/security/java.security" => super::security_properties_text().as_bytes(),
        "conf/security/policy/limited/default_US_export.policy" => POLICY_LIMITED_US_EXPORT,
        "conf/security/policy/limited/default_local.policy" => POLICY_LIMITED_LOCAL,
        "conf/security/policy/limited/exempt_local.policy" => POLICY_LIMITED_EXEMPT,
        "conf/security/policy/unlimited/default_US_export.policy" => POLICY_UNLIMITED_US_EXPORT,
        "conf/security/policy/unlimited/default_local.policy" => POLICY_UNLIMITED_LOCAL,
        // 本程序 jimage（boot-image §5.7）：ImageReaderFactory / SystemModuleFinders 以 isRegularFile 判定运行时映像，
        // 内容读取经 NativeImageBuffer.getNativeMap 的直接缓冲区
        "lib/modules" => crate::meta::module_image(),
        "lib/tzdb.dat" => super::TZDB_DAT,
        _ => return None,
    })
}

/// 目录：根或某文件路径的真前缀
pub fn is_dir(rel: &str) -> bool {
    rel.is_empty() || FILES.iter().any(|f| f.strip_prefix(rel).is_some_and(|r| r.starts_with('/')))
}

/// 目录的直接子项名（排序去重）
fn children(rel: &str) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = FILES
        .iter()
        .filter_map(|f| if rel.is_empty() { Some(*f) } else { f.strip_prefix(rel)?.strip_prefix('/') })
        .map(|r| r.split('/').next().unwrap_or(r))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

fn stat_of(mode: libc::mode_t, size: usize) -> libc::stat {
    // SAFETY: libc::stat 为纯数据结构，全零是合法值
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    st.st_mode = mode;
    st.st_nlink = 1;
    st.st_size = size as libc::off_t;
    st
}

pub fn dir_stat() -> libc::stat {
    stat_of(libc::S_IFDIR | 0o555, 0)
}

pub fn file_stat(len: usize) -> libc::stat {
    stat_of(libc::S_IFREG | 0o444, len)
}

/// stat(2) 的虚拟树形态：Some(Ok) = 节点属性；Some(Err(errno)) = 树内不存在（ENOENT）；None = 树外路径
pub fn stat(path: &str) -> Option<std::result::Result<libc::stat, i32>> {
    let rel = relative(path)?;
    Some(match content(rel) {
        Some(data) => Ok(file_stat(data.len())),
        None if is_dir(rel) => Ok(dir_stat()),
        None => Err(libc::ENOENT),
    })
}

/// access(2)：树内节点只读可执行（目录）——写请求得 EROFS，不存在得 ENOENT；树外为 None
pub fn access(path: &str, amode: i32) -> Option<i32> {
    let st = match stat(path)? {
        Ok(st) => st,
        Err(e) => return Some(e),
    };
    if amode & libc::W_OK != 0 {
        return Some(libc::EROFS);
    }
    if amode & libc::X_OK != 0 && st.st_mode & libc::S_IFMT != libc::S_IFDIR {
        return Some(libc::EACCES);
    }
    Some(0)
}

/// 目录流：子项名序列与读位置；`fd` 为 fdopendir 接管的目录句柄（closedir 时一并关闭）
struct DirStream {
    names: Vec<&'static str>,
    next: usize,
    fd: Option<i32>,
}

static NEXT_DIR: AtomicI64 = AtomicI64::new(-2);

fn streams() -> &'static Mutex<HashMap<i64, DirStream>> {
    static STREAMS: std::sync::OnceLock<Mutex<HashMap<i64, DirStream>>> = std::sync::OnceLock::new();
    STREAMS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn open_stream(rel: &str, fd: Option<i32>) -> i64 {
    // readdir(3) 同样给出 "." 与 ".."（UnixDirectoryStream 跳过二者）
    let mut names = vec![".", ".."];
    names.extend(children(rel));
    let h = NEXT_DIR.fetch_sub(1, Ordering::Relaxed);
    streams().lock().unwrap().insert(h, DirStream { names, next: 0, fd });
    h
}

/// opendir(3)：Some(Ok(句柄))；树内非目录 / 不存在 Some(Err(errno))；树外 None
pub fn opendir(path: &str) -> Option<std::result::Result<i64, i32>> {
    let rel = relative(path)?;
    Some(if is_dir(rel) {
        Ok(open_stream(rel, None))
    } else if content(rel).is_some() {
        Err(libc::ENOTDIR)
    } else {
        Err(libc::ENOENT)
    })
}

/// fdopendir(3)：Some(Ok(句柄))，目录句柄由流接管；虚拟文件句柄 Some(Err(ENOTDIR))；非虚拟 fd 为 None
pub fn fdopendir(fd: i32) -> Option<std::result::Result<i64, i32>> {
    if !super::is_virtual(fd) {
        return None;
    }
    Some(match super::virtual_dir(fd) {
        Some(rel) => Ok(open_stream(&rel, Some(fd))),
        None => Err(libc::ENOTDIR),
    })
}

/// 是否为虚拟目录流句柄
pub fn is_stream(h: i64) -> bool {
    h < 0 && streams().lock().unwrap().contains_key(&h)
}

/// readdir(3)：下一子项名；读完为 None
pub fn readdir(h: i64) -> Option<&'static str> {
    let mut map = streams().lock().unwrap();
    let s = map.get_mut(&h)?;
    let name = s.names.get(s.next).copied();
    s.next += 1;
    name
}

/// closedir(3)：注销流并关闭其接管的目录句柄
pub fn closedir(h: i64) {
    let fd = streams().lock().unwrap().remove(&h).and_then(|s| s.fd);
    if let Some(fd) = fd {
        super::virtual_close(fd);
    }
}
