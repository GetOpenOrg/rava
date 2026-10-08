//! 编译期嵌入的 JDK 数据文件——`${java.home}` 的只读虚拟树（用户决策 U14：java.home 构建期钉值）。
//!
//! 原生二进制不依赖 JDK 安装：`java.home` 钉为 [`JAVA_RUNTIME_HOME`]（`"/java-runtime"`，宿主文件系统
//! 不存在该目录），其下的文件在构建期嵌入（[`tree`]：`lib/tzdb.dat`、`lib/modules` = 本程序 jimage、
//! `conf/security/java.security`、`conf/security/policy/{limited,unlimited}/*.policy`），内容来自参考 JDK
//! 发行版，与运行机器无关。读这些文件的 JDK 代码全部按字节码执行（`TzdbZoneRulesProvider`、
//! `Security.<clinit>`、`JceSecurity.setupJurisdictionPolicies`、`ImageReaderFactory`），只有文件系统
//! native 在路径落入虚拟树时改读嵌入数据：
//! - java.io：`FileInputStream.open0`、`UnixFileSystem.getBooleanAttributes0`；
//! - NIO：`UnixNativeDispatcher` 的 stat / lstat / access / open / dup / close / opendir / fdopendir /
//!   readdir / closedir / fstat / realpath，`UnixFileDispatcherImpl` 的 read / pread / seek / size。
//!
//! 虚拟 fd 空间：`FileDescriptor.fd`（i32）按 Unix 语义存 OS fd 号
//! （非负），-1 为 invalid——虚拟树句柄从 -2 起递减分配，与 OS fd
//! 空间天然无碰撞；光标状态（`&'static [u8]` + 位置，目录句柄另记目录路径）存全局注册表。

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

pub mod tree;

/// 钉值的 `java.home`（U14）：虚拟树的根，宿主文件系统不存在该目录。
pub const JAVA_RUNTIME_HOME: &str = "/java-runtime";

/// JDK21 `$JAVA_HOME/lib/tzdb.dat`（102,820 字节，sha256
/// 36cf71e6…c7a1e3）——`TzdbZoneRulesProvider` 的时区规则数据源，
/// 与 `tests/expected/TestZonedDateTime.txt` 的金本位同源。
static TZDB_DAT: &[u8] = include_bytes!("tzdb.dat");

/// 嵌入资源句柄的光标状态。
struct VirtualHandle {
    data: &'static [u8],
    pos:  usize,
    /// 目录句柄：目录的相对路径（文件句柄为 None）
    dir:  Option<std::string::String>,
}

static NEXT_VIRTUAL_FD: AtomicI32 = AtomicI32::new(-2);

fn handles() -> &'static Mutex<HashMap<i32, VirtualHandle>> {
    static HANDLES: std::sync::OnceLock<Mutex<HashMap<i32, VirtualHandle>>> =
        std::sync::OnceLock::new();
    HANDLES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 按路径查询虚拟树中的文件内容。`path` 为 Java 侧拼出的绝对路径（`<JAVA_RUNTIME_HOME>/lib/tzdb.dat`）；
/// 目录与树外路径为 None。
pub fn lookup(path: &str) -> Option<&'static [u8]> {
    tree::content(tree::relative(path)?)
}

fn register(data: &'static [u8], dir: Option<std::string::String>) -> i32 {
    let fd = NEXT_VIRTUAL_FD.fetch_sub(1, Ordering::Relaxed);
    handles().lock().unwrap().insert(fd, VirtualHandle { data, pos: 0, dir });
    fd
}

/// 将虚拟树中的文件句柄化：分配虚拟 fd（负数）并登记初始光标。
/// 调用方（`FileInputStream.open0`）把返回值写入 `FileDescriptor.fd`。
pub fn open_embedded(path: &str) -> Option<i32> {
    Some(register(lookup(path)?, None))
}

/// open(2) 的虚拟树形态（NIO `open0`）：文件得光标句柄，目录得目录句柄（供 `fdopendir`）；树外为 None
pub fn open_node(path: &str) -> Option<i32> {
    let rel = tree::relative(path)?;
    match tree::content(rel) {
        Some(data) => Some(register(data, None)),
        None if tree::is_dir(rel) => Some(register(&[], Some(rel.to_string()))),
        None => None,
    }
}

/// NIO `open0` / `openat0` 的虚拟树形态：树外 None；树内只读打开得句柄，写 / 创建 / 截断得 EROFS，不存在得 ENOENT
pub fn nio_open(path: &str, flags: i32) -> Option<std::result::Result<i32, i32>> {
    tree::relative(path)?;
    if flags & libc::O_ACCMODE != libc::O_RDONLY || flags & (libc::O_CREAT | libc::O_TRUNC) != 0 {
        return Some(Err(libc::EROFS));
    }
    Some(open_node(path).ok_or(libc::ENOENT))
}

/// `*at` 系统调用的路径：dfd 为虚拟目录句柄时拼成虚拟树绝对路径（name 为绝对路径时取 name）；否则 None
pub fn at_path(dfd: i32, name: &str) -> Option<std::string::String> {
    if name.starts_with('/') {
        return tree::relative(name).map(|_| name.to_string());
    }
    let dir = virtual_dir(dfd)?;
    Some(if dir.is_empty() { format!("{JAVA_RUNTIME_HOME}/{name}") } else { format!("{JAVA_RUNTIME_HOME}/{dir}/{name}") })
}

/// fd 是否为已登记的虚拟句柄
pub fn is_virtual(fd: i32) -> bool {
    fd <= -2 && handles().lock().unwrap().contains_key(&fd)
}

/// dup(2)：新句柄指向同一节点（光标复制）
pub fn virtual_dup(fd: i32) -> Option<i32> {
    let (data, pos, dir) = {
        let map = handles().lock().unwrap();
        let h = map.get(&fd)?;
        (h.data, h.pos, h.dir.clone())
    };
    let nfd = register(data, dir);
    handles().lock().unwrap().get_mut(&nfd)?.pos = pos;
    Some(nfd)
}

/// 目录句柄的目录相对路径（文件句柄 / 非虚拟 fd 为 None）
pub fn virtual_dir(fd: i32) -> Option<std::string::String> {
    handles().lock().unwrap().get(&fd)?.dir.clone()
}

/// fstat(2) 的虚拟形态
pub fn virtual_fstat(fd: i32) -> Option<libc::stat> {
    let map = handles().lock().unwrap();
    let h = map.get(&fd)?;
    Some(match &h.dir {
        Some(_) => tree::dir_stat(),
        None => tree::file_stat(h.data.len()),
    })
}

/// pread(2)：自 `pos` 读入 `buf`，不动光标；返回读取字节数（EOF 为 0）
pub fn virtual_pread(fd: i32, buf: &mut [u8], pos: i64) -> usize {
    let map = handles().lock().unwrap();
    let Some(h) = map.get(&fd) else { return 0 };
    let start = (pos.max(0) as usize).min(h.data.len());
    let n = (h.data.len() - start).min(buf.len());
    buf[..n].copy_from_slice(&h.data[start..start + n]);
    n
}

/// lseek(2) SEEK_SET（可越过 EOF，读时得 EOF），返回新位置
pub fn virtual_seek(fd: i32, offset: i64) -> i64 {
    let mut map = handles().lock().unwrap();
    let Some(h) = map.get_mut(&fd) else { return 0 };
    h.pos = offset.max(0) as usize;
    h.pos as i64
}

/// 虚拟 fd 读：读入 `buf`，返回读取字节数（EOF 返回 0）。
pub fn virtual_read(fd: i32, buf: &mut [u8]) -> usize {
    let mut map = handles().lock().unwrap();
    let Some(h) = map.get_mut(&fd) else { return 0 };
    let n = h.data.len().saturating_sub(h.pos).min(buf.len());
    if n == 0 {
        return 0;
    }
    buf[..n].copy_from_slice(&h.data[h.pos..h.pos + n]);
    h.pos += n;
    n
}

/// 虚拟 fd 单字节读：EOF 或句柄失效返回 `None`。
pub fn virtual_read_byte(fd: i32) -> Option<u8> {
    let mut one = [0u8; 1];
    (virtual_read(fd, &mut one) == 1).then_some(one[0])
}

/// 虚拟 fd 跳读：至多跳到 EOF（HotSpot `skip0` 的 lseek 夹取语义），
/// 返回实际跳过字节数。
pub fn virtual_skip(fd: i32, n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut map = handles().lock().unwrap();
    let Some(h) = map.get_mut(&fd) else { return 0 };
    let remaining = h.data.len().saturating_sub(h.pos) as i64;
    let skip = n.min(remaining);
    h.pos += skip as usize;
    skip
}

/// 虚拟 fd 未读字节数（`available0` 的 FIONREAD 语义）。
pub fn virtual_available(fd: i32) -> i64 {
    let map = handles().lock().unwrap();
    map.get(&fd).map(|h| h.data.len().saturating_sub(h.pos) as i64).unwrap_or(0)
}

/// 虚拟 fd 当前偏移 / 数据总长（`position0` / `length0`）。
pub fn virtual_position(fd: i32) -> i64 {
    let map = handles().lock().unwrap();
    map.get(&fd).map(|h| h.pos as i64).unwrap_or(0)
}

pub fn virtual_length(fd: i32) -> i64 {
    let map = handles().lock().unwrap();
    map.get(&fd).map(|h| h.data.len() as i64).unwrap_or(0)
}

/// 虚拟 fd 关闭：注销光标状态（字节在二进制 .rodata，无需释放）。由 `FileDescriptor.close0` /
/// NIO `close0` / 虚拟 `closedir`（fdopendir 接管的目录句柄）调用。
pub fn virtual_close(fd: i32) {
    handles().lock().unwrap().remove(&fd);
}

/// JDK `$JAVA_HOME/conf/security/java.security` 的生效属性（按语料 JDK 版本各一份，机械提取见
/// 文件头注）——嵌入资源 `<java.home>/conf/security/java.security`，由翻译字节码的
/// `Security.<clinit>`（loadProps → Properties.load）读取。版本随 `crate::jdk_feature()`（21 / 25 间
/// 有键增删，如 25 移除 policy.provider）。
static SECURITY_PROPERTIES_21: &str = include_str!("java.security.21.properties");
static SECURITY_PROPERTIES_25: &str = include_str!("java.security.25.properties");

/// 当前 JDK 版本的 java.security 生效属性文本（Properties 文件格式，同时作为嵌入资源
/// `<java.home>/conf/security/java.security` 的内容）。
pub(crate) fn security_properties_text() -> &'static str {
    if crate::jdk_feature() >= 22 { SECURITY_PROPERTIES_25 } else { SECURITY_PROPERTIES_21 }
}
