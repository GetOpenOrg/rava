//! 编译期嵌入的 JDK 数据文件——伪 `$JAVA_HOME` 的只读镜像。
//!
//! 运行时产物是零依赖单二进制：JDK 随附数据文件（tzdb.dat 等）以
//! `include_bytes!` 进入二进制，时区等数据与宿主安装的 JDK、`TZ` 环境
//! 完全解耦——golden 与数据同源（JDK21 的 tzdb.dat），任何宿主上运行
//! 结果恒定。
//!
//! 路径协议：`StaticProperty.javaHome` 返回稳定伪值 [`JAVA_RUNTIME_HOME`]
//! （`"/java-runtime"`，宿主文件系统不存在该目录），Java 侧拼出的
//! `<javaHome>/lib/<name>` 路径在 [`lookup`] 按相对特征命中即供嵌入字节。
//! 文件读取的单一决策点在 `FileInputStream.open0`（native 层）：
//! 命中嵌入资源 → 句柄化为虚拟 fd；未命中 → 打开宿主真实文件。
//!
//! 虚拟 fd 空间：`FileDescriptor.fd`（i32）按 Unix 语义存 OS fd 号
//! （非负），-1 为 invalid——嵌入资源句柄从 -2 起递减分配，与 OS fd
//! 空间天然无碰撞；光标状态（`&'static [u8]` + 位置）存全局注册表。

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

/// `StaticProperty.javaHome` 的稳定伪值。值本身无语义，仅作为嵌入资源
/// 路径前缀的锚点；真实 `java.home` 在原生二进制中不存在。
pub const JAVA_RUNTIME_HOME: &str = "/java-runtime";

/// JDK21 `$JAVA_HOME/lib/tzdb.dat`（102,820 字节，sha256
/// 36cf71e6…c7a1e3）——`TzdbZoneRulesProvider` 的时区规则数据源，
/// 与 `tests/expected/TestZonedDateTime.txt` 的金本位同源。
static TZDB_DAT: &[u8] = include_bytes!("tzdb.dat");

/// 嵌入资源句柄的光标状态。
struct VirtualHandle {
    data: &'static [u8],
    pos:  usize,
}

static NEXT_VIRTUAL_FD: AtomicI32 = AtomicI32::new(-2);

fn handles() -> &'static Mutex<HashMap<i32, VirtualHandle>> {
    static HANDLES: std::sync::OnceLock<Mutex<HashMap<i32, VirtualHandle>>> =
        std::sync::OnceLock::new();
    HANDLES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 按路径查询嵌入资源。`path` 为 Java 侧拼出的绝对路径
/// （`<JAVA_RUNTIME_HOME>/lib/tzdb.dat`）或等价相对形式（`lib/tzdb.dat`）。
pub fn lookup(path: &str) -> Option<&'static [u8]> {
    let rel = path.strip_prefix(JAVA_RUNTIME_HOME).unwrap_or(path);
    let rel = rel.trim_start_matches('/');
    match rel {
        "lib/tzdb.dat" => Some(TZDB_DAT),
        _ => None,
    }
}

/// 将嵌入资源句柄化：分配虚拟 fd（负数）并登记初始光标。
/// 调用方（`FileInputStream.open0`）把返回值写入 `FileDescriptor.fd`。
pub fn open_embedded(path: &str) -> Option<i32> {
    let data = lookup(path)?;
    let fd = NEXT_VIRTUAL_FD.fetch_sub(1, Ordering::Relaxed);
    handles().lock().unwrap().insert(fd, VirtualHandle { data, pos: 0 });
    Some(fd)
}

/// 虚拟 fd 读：读入 `buf`，返回读取字节数（EOF 返回 0）。
pub fn virtual_read(fd: i32, buf: &mut [u8]) -> usize {
    let mut map = handles().lock().unwrap();
    let Some(h) = map.get_mut(&fd) else { return 0 };
    let n = h.data.len().saturating_sub(h.pos).min(buf.len());
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
    let remaining = (h.data.len() - h.pos) as i64;
    let skip = n.min(remaining);
    h.pos += skip as usize;
    skip
}

/// 虚拟 fd 未读字节数（`available0` 的 FIONREAD 语义）。
pub fn virtual_available(fd: i32) -> i64 {
    let map = handles().lock().unwrap();
    map.get(&fd).map(|h| (h.data.len() - h.pos) as i64).unwrap_or(0)
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

/// 虚拟 fd 关闭：注销光标状态（字节在二进制 .rodata，无需释放）。
pub fn virtual_close(fd: i32) {
    handles().lock().unwrap().remove(&fd);
}

/// JDK `$JAVA_HOME/conf/security/java.security` 的生效属性（按语料 JDK 版本各一份，机械提取见
/// 文件头注）——`Security.getProperty` 的只读数据源。版本随 `crate::jdk_feature()`（21 / 25 间
/// 有键增删，如 25 移除 policy.provider）。
static SECURITY_PROPERTIES_21: &str = include_str!("java.security.21.properties");
static SECURITY_PROPERTIES_25: &str = include_str!("java.security.25.properties");

/// 按键查 java.security 基线属性（未定义 → None）。
pub fn security_property(key: &str) -> Option<&'static str> {
    let table = if crate::jdk_feature() >= 22 { SECURITY_PROPERTIES_25 } else { SECURITY_PROPERTIES_21 };
    table.lines()
        .filter(|l| !l.starts_with('#'))
        .find_map(|l| l.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v))
}
