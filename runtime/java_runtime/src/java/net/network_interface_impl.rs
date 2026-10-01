//! `java/net/NetworkInterface` 的 native 方法（与生成的 network_interface.rs 共置）。
//!
//! 对标 JDK NetworkInterface.c（Linux）：接口枚举经 getifaddrs（名字、flags、IPv4 / IPv6 地址、
//! 掩码、广播），index 经 if_nametoindex，MAC / MTU 读 /sys/class/net/<name>/{address,mtu}。
//! 对象构造与 JDK 同形：无参构造后逐字段写入 name / displayName（= name）/ index / addrs /
//! bindings / childs（别名子接口不建模，恒空）。典型消费方：SecureRandom 播种的
//! SeedGenerator.addNetworkAdapterInfo（接口名与 MAC 入熵）。

use crate::prelude::*;
use super::network_interface::NetworkInterface;
use super::{Inet4Address, Inet6Address, InetAddress, InterfaceAddress};

/// getifaddrs 的一条地址记录。
struct IfAddr {
    /// IPv4 4 字节 / IPv6 16 字节
    addr: Vec<u8>,
    /// 前缀长度（掩码中置位数）
    prefix: i16,
    /// IPv4 广播地址（IFF_BROADCAST 时）
    broadcast: Option<Vec<u8>>,
}

/// 一个接口的聚合信息（同名记录合并）。
struct IfInfo {
    name: std::string::String,
    flags: u32,
    addrs: Vec<IfAddr>,
}

/// sockaddr → 地址字节（AF_INET / AF_INET6，其余族 None）。
///
/// SAFETY: sa 为 getifaddrs 返回的有效 sockaddr 指针或 null。
unsafe fn sockaddr_bytes(sa: *const libc::sockaddr) -> Option<Vec<u8>> {
    if sa.is_null() {
        return None;
    }
    match (*sa).sa_family as i32 {
        libc::AF_INET => {
            let sin = &*(sa as *const libc::sockaddr_in);
            Some(sin.sin_addr.s_addr.to_ne_bytes().to_vec())
        }
        libc::AF_INET6 => {
            let sin6 = &*(sa as *const libc::sockaddr_in6);
            Some(sin6.sin6_addr.s6_addr.to_vec())
        }
        _ => None,
    }
}

/// 全部接口（按名字聚合，保持首次出现顺序）。
fn enumerate() -> Vec<IfInfo> {
    let mut list: Vec<IfInfo> = Vec::new();
    let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
    // SAFETY: getifaddrs 分配链表写入 ifap，使用完毕 freeifaddrs 释放
    if unsafe { libc::getifaddrs(&mut ifap) } != 0 {
        return list;
    }
    let mut cur = ifap;
    while !cur.is_null() {
        // SAFETY: cur 为链表内有效节点
        let ifa = unsafe { &*cur };
        cur = ifa.ifa_next;
        if ifa.ifa_name.is_null() {
            continue;
        }
        // SAFETY: ifa_name 为以 NUL 结尾的接口名
        let name = unsafe { std::ffi::CStr::from_ptr(ifa.ifa_name) }.to_string_lossy().into_owned();
        let pos = match list.iter().position(|i| i.name == name) {
            Some(p) => p,
            None => {
                list.push(IfInfo { name, flags: ifa.ifa_flags, addrs: Vec::new() });
                list.len() - 1
            }
        };
        // SAFETY: ifa_addr / ifa_netmask / 广播地址为节点内的 sockaddr 指针（可为 null）
        if let Some(addr) = unsafe { sockaddr_bytes(ifa.ifa_addr) } {
            let prefix = unsafe { sockaddr_bytes(ifa.ifa_netmask) }
                .map(|m| m.iter().map(|b| b.count_ones() as i16).sum())
                .unwrap_or(0);
            let broadcast = if addr.len() == 4 && ifa.ifa_flags & libc::IFF_BROADCAST as u32 != 0 {
                unsafe { sockaddr_bytes(broadcast_addr(ifa)) }
            } else {
                None
            };
            list[pos].addrs.push(IfAddr { addr, prefix, broadcast });
        }
    }
    // SAFETY: 释放 getifaddrs 分配的链表
    unsafe { libc::freeifaddrs(ifap) };
    list
}

/// 广播地址字段：Linux 为 `ifa_ifu` 联合体，BSD 系（含 macOS）为 `ifa_dstaddr`
#[cfg(any(target_os = "linux", target_os = "android"))]
fn broadcast_addr(ifa: &libc::ifaddrs) -> *mut libc::sockaddr {
    ifa.ifa_ifu
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn broadcast_addr(ifa: &libc::ifaddrs) -> *mut libc::sockaddr {
    ifa.ifa_dstaddr
}

fn if_index(name: &str) -> i32 {
    match std::ffi::CString::new(name) {
        // SAFETY: 以 NUL 结尾的接口名
        Ok(c) => unsafe { libc::if_nametoindex(c.as_ptr()) as i32 },
        Err(_) => 0,
    }
}

fn to_jbytes(b: &[u8]) -> JArray<i8> {
    JArray::from(b.iter().map(|x| *x as i8).collect::<Vec<i8>>())
}

/// 地址字节 → Inet4Address / Inet6Address（IPv6 链路本地地址带接口 scope_id）。
fn inet_address(bytes: &[u8], index: i32) -> Result<InetAddress> {
    if bytes.len() == 4 {
        let a = Inet4Address::new_str_arr_b(String::default(), to_jbytes(bytes))?;
        Ok(<InetAddress as From<Inet4Address>>::from(a))
    } else {
        let link_local = bytes[0] == 0xfe && (bytes[1] & 0xc0) == 0x80;
        let a = Inet6Address::new_str_arr_b_i(String::default(), to_jbytes(bytes), if link_local { index } else { 0 })?;
        Ok(<InetAddress as From<Inet6Address>>::from(a))
    }
}

/// IfInfo → NetworkInterface 对象（JDK createNetworkInterface 同形）。
fn build(info: &IfInfo) -> Result<NetworkInterface> {
    let index = if_index(&info.name);
    let ni = NetworkInterface::new()?;
    ni.__set_name(String::from(info.name.as_str()));
    ni.__set_displayName(String::from(info.name.as_str()));
    ni.__set_index(index);
    let mut addrs = Vec::new();
    let mut bindings = Vec::new();
    for a in &info.addrs {
        let ia = inet_address(&a.addr, index)?;
        let binding = InterfaceAddress::new()?;
        binding.__set_address(Clone::clone(&ia));
        binding.__set_maskLength(a.prefix);
        if let Some(b) = &a.broadcast {
            binding.__set_broadcast(Inet4Address::new_str_arr_b(String::default(), to_jbytes(b))?);
        }
        addrs.push(ia);
        bindings.push(binding);
    }
    ni.__set_addrs(JArray::from(addrs));
    ni.__set_bindings(JArray::from(bindings));
    ni.__set_childs(JArray::from(Vec::<NetworkInterface>::new()));
    Ok(ni)
}

fn find(name: &str) -> Option<IfInfo> {
    enumerate().into_iter().find(|i| i.name == name)
}

fn flags_of(name: &String) -> u32 {
    find(&name.to_string()).map(|i| i.flags).unwrap_or(0)
}

/// /sys/class/net/<name>/<attr> 的文本值。
fn sysfs(name: &str, attr: &str) -> Option<std::string::String> {
    std::fs::read_to_string(format!("/sys/class/net/{}/{}", name, attr)).ok().map(|s| s.trim().to_owned())
}

/// `SIOCGIFMTU`：Linux 取自 libc；macOS 为 `_IOWR('i', 51, struct ifreq)`（libc 未导出）。
#[cfg(target_os = "macos")]
const SIOCGIFMTU: libc::c_ulong = 0xc020_6933;
#[cfg(not(target_os = "macos"))]
const SIOCGIFMTU: libc::c_ulong = libc::SIOCGIFMTU as libc::c_ulong;

/// 接口 MTU（`ioctl(SIOCGIFMTU)`，名称超长或调用失败为 None）。
fn ioctl_mtu(name: &str) -> Option<i32> {
    let bytes = name.as_bytes();
    if bytes.len() >= libc::IFNAMSIZ {
        return None;
    }
    // SAFETY：ifreq 为纯 C 结构，全零合法；套接字在返回前关闭
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
        if fd < 0 {
            return None;
        }
        let mut req: libc::ifreq = std::mem::zeroed();
        for (d, b) in req.ifr_name.iter_mut().zip(bytes) {
            *d = *b as libc::c_char;
        }
        let rc = libc::ioctl(fd, SIOCGIFMTU as _, &mut req);
        libc::close(fd);
        (rc >= 0).then_some(req.ifr_ifru.ifru_mtu)
    }
}

/// InetAddress → 地址字节（经其 getAddress）。
fn address_bytes(addr: &InetAddress) -> Result<Vec<u8>> {
    Ok(addr.getAddress()?.to_vec().into_iter().map(|b| b as u8).collect())
}

impl NetworkInterface {
    /// native `init()`：缓存 JNI 类 / 字段 ID——无需。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }

    /// native `getAll()`：全部接口。
    #[jvm_native]
    pub fn getAll() -> Result<JArray<NetworkInterface>> {
        let mut out = Vec::new();
        for info in enumerate() {
            out.push(build(&info)?);
        }
        Ok(JArray::from(out))
    }

    /// native `getByName0(String)`：按名查找；不存在返回 null。
    #[jvm_native]
    pub fn getByName0(name: String) -> Result<NetworkInterface> {
        match find(&name.to_string()) {
            Some(info) => build(&info),
            None => Ok(NetworkInterface::default()),
        }
    }

    /// native `getByIndex0(int)`：按 index 查找；不存在返回 null。
    #[jvm_native]
    pub fn getByIndex0(index: i32) -> Result<NetworkInterface> {
        match enumerate().into_iter().find(|i| if_index(&i.name) == index) {
            Some(info) => build(&info),
            None => Ok(NetworkInterface::default()),
        }
    }

    /// native `boundInetAddress0(InetAddress)`：地址是否绑定在某个接口上。
    #[jvm_native]
    pub fn boundInetAddress0(addr: InetAddress) -> Result<bool> {
        let bytes = address_bytes(&addr)?;
        Ok(enumerate().iter().any(|i| i.addrs.iter().any(|a| a.addr == bytes)))
    }

    /// native `getByInetAddress0(InetAddress)`：绑定该地址的接口；无则 null。
    #[jvm_native]
    pub fn getByInetAddress0(addr: InetAddress) -> Result<NetworkInterface> {
        let bytes = address_bytes(&addr)?;
        match enumerate().into_iter().find(|i| i.addrs.iter().any(|a| a.addr == bytes)) {
            Some(info) => build(&info),
            None => Ok(NetworkInterface::default()),
        }
    }

    #[jvm_native]
    pub fn isUp0(name: String, _index: i32) -> Result<bool> {
        let f = flags_of(&name);
        Ok(f & libc::IFF_UP as u32 != 0 && f & libc::IFF_RUNNING as u32 != 0)
    }

    #[jvm_native]
    pub fn isLoopback0(name: String, _index: i32) -> Result<bool> {
        Ok(flags_of(&name) & libc::IFF_LOOPBACK as u32 != 0)
    }

    #[jvm_native]
    pub fn supportsMulticast0(name: String, _index: i32) -> Result<bool> {
        Ok(flags_of(&name) & libc::IFF_MULTICAST as u32 != 0)
    }

    #[jvm_native]
    pub fn isP2P0(name: String, _index: i32) -> Result<bool> {
        Ok(flags_of(&name) & libc::IFF_POINTOPOINT as u32 != 0)
    }

    /// native `getMacAddr0(byte[] inAddr, String name, int index)`：硬件地址；无硬件地址或全零
    /// （loopback 等）返回 null（JDK getMacAddress 同）。
    #[jvm_native]
    pub fn getMacAddr0(_in_addr: JArray<i8>, name: String, _index: i32) -> Result<JArray<i8>> {
        let mac: Option<Vec<u8>> = sysfs(&name.to_string(), "address").map(|s| {
            s.split(':').filter_map(|h| u8::from_str_radix(h, 16).ok()).collect()
        });
        match mac {
            Some(m) if !m.is_empty() && m.iter().any(|b| *b != 0) => Ok(to_jbytes(&m)),
            _ => Ok(JArray::default()),
        }
    }

    /// native `getMTU0(String name, int index)`：与 JDK `NetworkInterface.c` 同取法——数据报套接字上
    /// `ioctl(SIOCGIFMTU)`；不可得返回 -1。
    #[jvm_native]
    pub fn getMTU0(name: String, _index: i32) -> Result<i32> {
        Ok(ioctl_mtu(&name.to_string()).unwrap_or(-1))
    }
}
