//! 套接字系统调用的窄封装（libnet `net_util.c` / `net_util_md.c` 的非 JNI 部分）：协议栈可用性探测、
//! 地址字节 ↔ sockaddr 编解码、errno 文案。只处理字节与整数，不构造 Java 对象——
//! Java 侧对象（InetAddress / 异常）由各类的共置手写 native 文件构造。
//! 消费方：`InetAddress.isIPv4Available / isIPv6Supported`、`sun.nio.ch.Net` 的套接字 native。

use std::sync::OnceLock;

pub fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// strerror 文案（去掉 std 追加的 ` (os error N)`）；err = 0 时为 None。
pub fn strerror(err: i32) -> Option<std::string::String> {
    if err == 0 {
        return None;
    }
    let s = std::io::Error::from_raw_os_error(err).to_string();
    s.split(" (os error").next().filter(|m| !m.is_empty()).map(str::to_owned)
}

/// `JNU_ThrowByNameWithLastError` 的消息：有 errno 文案用文案，否则用缺省消息。
pub fn last_error_message(default: &str) -> std::string::String {
    strerror(errno()).unwrap_or_else(|| default.to_owned())
}

fn probe_socket(domain: i32) -> bool {
    // SAFETY: 新建后立即关闭的探测套接字
    unsafe {
        let fd = libc::socket(domain, libc::SOCK_STREAM, 0);
        if fd < 0 {
            return false;
        }
        libc::close(fd);
        true
    }
}

/// `IPv4_supported()`：能创建 AF_INET 流套接字。
pub fn ipv4_supported() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| probe_socket(libc::AF_INET))
}

/// `IPv6_supported()`：能创建 AF_INET6 流套接字，且标准输入不是 AF_INET 套接字
/// （inetd 以 IPv4 套接字作 fd 0 启动时 JDK 关闭 IPv6，保持一致）。
pub fn ipv6_supported() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| probe_socket(libc::AF_INET6) && !stdin_is_ipv4_socket())
}

fn stdin_is_ipv4_socket() -> bool {
    // SAFETY: getsockname 写入栈上 sockaddr_storage
    unsafe {
        let mut ss: libc::sockaddr_storage = std::mem::zeroed();
        let mut len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        libc::getsockname(0, &mut ss as *mut _ as *mut libc::sockaddr, &mut len) == 0 && ss.ss_family as i32 == libc::AF_INET
    }
}

/// 编码后的 sockaddr（`SOCKETADDRESS`）。
pub struct SockAddr {
    pub storage: libc::sockaddr_storage,
    pub len: libc::socklen_t,
}

impl SockAddr {
    pub fn as_ptr(&self) -> *const libc::sockaddr {
        &self.storage as *const _ as *const libc::sockaddr
    }

    pub fn empty() -> SockAddr {
        // SAFETY: sockaddr_storage 为纯 C 结构，全零合法
        SockAddr { storage: unsafe { std::mem::zeroed() }, len: std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t }
    }

    pub fn as_mut_ptr(&mut self) -> *mut libc::sockaddr {
        &mut self.storage as *mut _ as *mut libc::sockaddr
    }

    pub fn family(&self) -> i32 {
        self.storage.ss_family as i32
    }
}

/// `NET_InetAddressToSockaddr`：地址字节（4 / 16）+ scope + 端口 → sockaddr。
/// prefer_v6：IPv4 地址编码为 v4 映射地址（通配地址编码为 `::`）；否则 IPv6 地址返回 Err
/// （JDK 抛 SocketException「Protocol family unavailable」）。
pub fn encode(addr: &[u8], scope_id: u32, port: u16, prefer_v6: bool) -> Result<SockAddr, ()> {
    let mut sa = SockAddr::empty();
    if prefer_v6 {
        let mut b = [0u8; 16];
        if addr.len() == 4 {
            if addr != [0, 0, 0, 0] {
                b[10] = 0xff;
                b[11] = 0xff;
                b[12..].copy_from_slice(addr);
            }
        } else {
            b.copy_from_slice(&addr[..16]);
        }
        // SAFETY: sockaddr_storage 足以容纳 sockaddr_in6
        let sin6 = unsafe { &mut *(sa.as_mut_ptr() as *mut libc::sockaddr_in6) };
        sin6.sin6_family = libc::AF_INET6 as libc::sa_family_t;
        sin6.sin6_port = port.to_be();
        sin6.sin6_addr.s6_addr = b;
        sin6.sin6_scope_id = if addr.len() == 16 { scope_id } else { 0 };
        set_len(sin6, std::mem::size_of::<libc::sockaddr_in6>());
        sa.len = std::mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t;
    } else {
        if addr.len() != 4 {
            return Err(());
        }
        // SAFETY: sockaddr_storage 足以容纳 sockaddr_in
        let sin = unsafe { &mut *(sa.as_mut_ptr() as *mut libc::sockaddr_in) };
        sin.sin_family = libc::AF_INET as libc::sa_family_t;
        sin.sin_port = port.to_be();
        sin.sin_addr.s_addr = u32::from_ne_bytes([addr[0], addr[1], addr[2], addr[3]]);
        set_len(sin, std::mem::size_of::<libc::sockaddr_in>());
        sa.len = std::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
    }
    Ok(sa)
}

/// BSD 系 sockaddr 带长度字节（sin_len / sin6_len）。
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
fn set_len<T>(sa: &mut T, len: usize) {
    // SAFETY: sockaddr_in / sockaddr_in6 首字节均为长度字段
    unsafe { *(sa as *mut T as *mut u8) = len as u8 };
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
fn set_len<T>(_sa: &mut T, _len: usize) {}

/// 解码后的对端 / 本端地址（`NET_SockaddrToInetAddress` 的输入形态）。
pub struct Decoded {
    /// IPv4 4 字节（含 v4 映射地址还原）/ IPv6 16 字节
    pub addr: Vec<u8>,
    pub scope_id: u32,
    pub port: i32,
}

/// sockaddr → 地址字节 + scope + 端口；v4 映射的 IPv6 地址还原为 IPv4（JDK 同）。
pub fn decode(sa: &SockAddr) -> Decoded {
    if sa.family() == libc::AF_INET6 {
        // SAFETY: 族为 AF_INET6 时存储内容为 sockaddr_in6
        let sin6 = unsafe { &*(sa.as_ptr() as *const libc::sockaddr_in6) };
        let b = sin6.sin6_addr.s6_addr;
        let port = u16::from_be(sin6.sin6_port) as i32;
        if b[..10].iter().all(|x| *x == 0) && b[10] == 0xff && b[11] == 0xff {
            return Decoded { addr: b[12..].to_vec(), scope_id: 0, port };
        }
        Decoded { addr: b.to_vec(), scope_id: sin6.sin6_scope_id, port }
    } else {
        // SAFETY: 其余族按 sockaddr_in 解读（套接字只以 AF_INET / AF_INET6 创建）
        let sin = unsafe { &*(sa.as_ptr() as *const libc::sockaddr_in) };
        Decoded { addr: sin.sin_addr.s_addr.to_ne_bytes().to_vec(), scope_id: 0, port: u16::from_be(sin.sin_port) as i32 }
    }
}

/// getsockname(2) / getpeername(2)：失败返回 errno。
pub fn sock_name(fd: i32, peer: bool) -> Result<SockAddr, i32> {
    let mut sa = SockAddr::empty();
    // SAFETY: 写入 sa.storage，长度为其容量
    let rc = unsafe {
        if peer {
            libc::getpeername(fd, sa.as_mut_ptr(), &mut sa.len)
        } else {
            libc::getsockname(fd, sa.as_mut_ptr(), &mut sa.len)
        }
    };
    if rc < 0 {
        Err(errno())
    } else {
        Ok(sa)
    }
}

/// setsockopt(2)，int 形参。
pub fn set_int(fd: i32, level: i32, opt: i32, value: i32) -> i32 {
    // SAFETY: value 为栈上 int
    unsafe {
        libc::setsockopt(fd, level, opt, &value as *const i32 as *const libc::c_void, std::mem::size_of::<i32>() as libc::socklen_t)
    }
}

/// getsockopt(2)，int 形参：成功返回值，失败返回 errno。
pub fn get_int(fd: i32, level: i32, opt: i32) -> Result<i32, i32> {
    let mut v: i32 = 0;
    let mut len = std::mem::size_of::<i32>() as libc::socklen_t;
    // SAFETY: v 为栈上 int
    let rc = unsafe { libc::getsockopt(fd, level, opt, &mut v as *mut i32 as *mut libc::c_void, &mut len) };
    if rc < 0 {
        Err(errno())
    } else {
        Ok(v)
    }
}

/// 在新建的探测套接字上设置（`set` 为 Some）或读取选项：返回调用失败时的 errno，成功 Ok。
/// 套接字创建失败返回 Err(0)。消费方：jdk.net 平台选项的可用性探测。
pub fn probe_option(domain: i32, ty: i32, level: i32, opt: i32, set: Option<i32>) -> Result<(), i32> {
    // SAFETY: 探测套接字，返回前关闭
    unsafe {
        let fd = libc::socket(domain, ty, 0);
        if fd < 0 {
            return Err(0);
        }
        let r = match set {
            Some(v) => if set_int(fd, level, opt, v) < 0 { Err(errno()) } else { Ok(()) },
            None => get_int(fd, level, opt).map(|_| ()),
        };
        libc::close(fd);
        r
    }
}

// ── 名字服务（libnet `Inet4AddressImpl.c` / `Inet6AddressImpl.c` 的系统调用部分）──────────

/// 解析结果中的一个地址：地址字节（4 / 16）与 IPv6 scope_id。
pub struct HostAddr {
    pub addr: Vec<u8>,
    pub scope_id: u32,
}

/// `gethostname`；失败为 "localhost"（JDK 同）。
pub fn host_name() -> std::string::String {
    let mut buf = [0u8; 1026];
    // SAFETY: 写入栈上缓冲区，末字节保留为 NUL
    if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len() - 1) } != 0 {
        return "localhost".to_owned();
    }
    let end = buf.iter().position(|b| *b == 0).unwrap_or(buf.len() - 1);
    std::string::String::from_utf8_lossy(&buf[..end]).into_owned()
}

/// getaddrinfo(host, AI_CANONNAME, family)：按系统顺序返回去重后的地址；失败返回
/// `"<host>: <gai_strerror>"`（`NET_ThrowUnknownHostExceptionWithGaiError` 的消息）。
pub fn resolve(host: &str, family: i32) -> Result<Vec<HostAddr>, std::string::String> {
    let c = std::ffi::CString::new(host).map_err(|_| format!("{host}: invalid host name"))?;
    // SAFETY: hints 为纯 C 结构，全零合法；res 链表在返回前以 freeaddrinfo 释放
    unsafe {
        let mut hints: libc::addrinfo = std::mem::zeroed();
        hints.ai_flags = libc::AI_CANONNAME;
        hints.ai_family = family;
        let mut res: *mut libc::addrinfo = std::ptr::null_mut();
        let err = libc::getaddrinfo(c.as_ptr(), std::ptr::null(), &hints, &mut res);
        if err != 0 {
            let msg = std::ffi::CStr::from_ptr(libc::gai_strerror(err)).to_string_lossy().into_owned();
            return Err(format!("{host}: {msg}"));
        }
        let mut out: Vec<HostAddr> = Vec::new();
        let mut cur = res;
        while !cur.is_null() {
            let ai = &*cur;
            cur = ai.ai_next;
            let sa = SockAddr { storage: copy_storage(ai.ai_addr, ai.ai_addrlen), len: ai.ai_addrlen };
            let fam = sa.family();
            if fam != libc::AF_INET && fam != libc::AF_INET6 {
                continue;
            }
            let addr = raw_addr(&sa);
            if !out.iter().any(|h| h.addr == addr.addr && h.scope_id == addr.scope_id) {
                out.push(addr);
            }
        }
        libc::freeaddrinfo(res);
        Ok(out)
    }
}

/// sockaddr 指针 → sockaddr_storage 副本。
///
/// SAFETY: sa 指向长度为 len 的有效 sockaddr。
unsafe fn copy_storage(sa: *const libc::sockaddr, len: libc::socklen_t) -> libc::sockaddr_storage {
    let mut ss: libc::sockaddr_storage = std::mem::zeroed();
    let n = (len as usize).min(std::mem::size_of::<libc::sockaddr_storage>());
    std::ptr::copy_nonoverlapping(sa as *const u8, &mut ss as *mut _ as *mut u8, n);
    ss
}

/// 地址字节（不做 v4 映射还原：名字服务结果按族原样返回）。
fn raw_addr(sa: &SockAddr) -> HostAddr {
    if sa.family() == libc::AF_INET6 {
        // SAFETY: 族为 AF_INET6
        let sin6 = unsafe { &*(sa.as_ptr() as *const libc::sockaddr_in6) };
        HostAddr { addr: sin6.sin6_addr.s6_addr.to_vec(), scope_id: sin6.sin6_scope_id }
    } else {
        // SAFETY: 族为 AF_INET
        let sin = unsafe { &*(sa.as_ptr() as *const libc::sockaddr_in) };
        HostAddr { addr: sin.sin_addr.s_addr.to_ne_bytes().to_vec(), scope_id: 0 }
    }
}

/// getnameinfo(NI_NAMEREQD)：地址字节（4 / 16）→ 主机名；无名字为 None。
pub fn reverse(addr: &[u8]) -> Option<std::string::String> {
    let sa = encode(addr, 0, 0, addr.len() == 16).ok()?;
    let mut host = [0 as libc::c_char; 1025];
    // SAFETY: sa 为已编码 sockaddr，host 为栈上缓冲区
    let rc = unsafe {
        libc::getnameinfo(sa.as_ptr(), sa.len, host.as_mut_ptr(), host.len() as libc::socklen_t, std::ptr::null_mut(), 0, libc::NI_NAMEREQD)
    };
    if rc != 0 {
        return None;
    }
    // SAFETY: getnameinfo 成功时写入 NUL 结尾字符串
    Some(unsafe { std::ffi::CStr::from_ptr(host.as_ptr()) }.to_string_lossy().into_owned())
}

/// macOS `lookupIfLocalhost`：host 等于本机名时以全部接口地址作解析结果（本机名不在 DNS / hosts
/// 中时 getaddrinfo 失败的兜底）。只有回环地址时才包含回环；v6_first 决定族的先后。
/// host 不是本机名或 getifaddrs 失败返回 None。
#[cfg(target_os = "macos")]
pub fn local_host_fallback(host: &str, include_v6: bool, v6_first: bool) -> Option<Vec<HostAddr>> {
    if host_name() != host {
        return None;
    }
    let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
    // SAFETY: getifaddrs 分配链表写入 ifap，使用完毕 freeifaddrs 释放
    if unsafe { libc::getifaddrs(&mut ifap) } != 0 {
        return None;
    }
    let mut all: Vec<(HostAddr, bool)> = Vec::new();
    let mut cur = ifap;
    while !cur.is_null() {
        // SAFETY: cur 为链表内有效节点
        let ifa = unsafe { &*cur };
        cur = ifa.ifa_next;
        // SAFETY: ifa_name 为 NUL 结尾接口名
        if ifa.ifa_addr.is_null() || ifa.ifa_name.is_null() || unsafe { *ifa.ifa_name } == 0 {
            continue;
        }
        // SAFETY: ifa_addr 非空
        let fam = unsafe { (*ifa.ifa_addr).sa_family } as i32;
        if fam != libc::AF_INET && !(fam == libc::AF_INET6 && include_v6) {
            continue;
        }
        // SAFETY: BSD sockaddr 首字节 sa_len 为结构实际长度
        let len = unsafe { (*ifa.ifa_addr).sa_len } as libc::socklen_t;
        let sa = SockAddr { storage: unsafe { copy_storage(ifa.ifa_addr, len) }, len };
        let d = decode(&sa);
        all.push((HostAddr { addr: d.addr, scope_id: d.scope_id }, ifa.ifa_flags & libc::IFF_LOOPBACK as u32 != 0));
    }
    // SAFETY: 释放 getifaddrs 分配的链表
    unsafe { libc::freeifaddrs(ifap) };
    let include_loopback = all.iter().all(|(_, lo)| *lo);
    let kept = all.into_iter().filter(|(_, lo)| include_loopback || !lo).map(|(h, _)| h);
    let (v4, v6): (Vec<HostAddr>, Vec<HostAddr>) = kept.partition(|h| h.addr.len() == 4);
    Some(if v6_first { v6.into_iter().chain(v4).collect() } else { v4.into_iter().chain(v6).collect() })
}

#[cfg(not(target_os = "macos"))]
pub fn local_host_fallback(_host: &str, _include_v6: bool, _v6_first: bool) -> Option<Vec<HostAddr>> {
    None
}
