//! `sun/nio/ch/Net` 的 ACC_NATIVE（类 1，libnio `Net.c` + libnet `net_util_md.c`）。其余方法按字节码翻译。
//!
//! 返回值与异常按 JNI 同款：失败经 `handleSocketError` 按 errno 选异常类（EINPROGRESS 视为 0），
//! 消息为 strerror 文案；IOStatus：UNAVAILABLE = -2、INTERRUPTED = -3。
//! 套接字选项、poll 与可用性探测见同类的 `net_ext.rs`。组播 / 带外数据 native 不在档案调用链上。

use crate::prelude::*;
use super::net::Net;
use crate::java::io::FileDescriptor;
use crate::java::net::{Inet4Address, Inet6Address, InetAddress, InetSocketAddress};
use crate::net_posix::{self, errno, SockAddr};

pub(super) const IOS_UNAVAILABLE: i32 = -2;
pub(super) const IOS_INTERRUPTED: i32 = -3;

/// `handleSocketError`：EINPROGRESS → Ok(0)，其余按 errno 选 java.net 异常类。
pub(super) fn socket_error(err: i32) -> Result<i32> {
    let msg = String::from(net_posix::strerror(err).unwrap_or_else(|| "NioSocketError".to_owned()));
    let e = match err {
        libc::EINPROGRESS => return Ok(0),
        libc::EPROTO => crate::java::net::ProtocolException::new_str(msg).map(JvmError::from),
        libc::ECONNREFUSED | libc::ETIMEDOUT | libc::ENOTCONN => crate::java::net::ConnectException::new_str(msg).map(JvmError::from),
        libc::EHOSTUNREACH => crate::java::net::NoRouteToHostException::new_str(msg).map(JvmError::from),
        libc::EADDRINUSE | libc::EADDRNOTAVAIL | libc::EACCES => crate::java::net::BindException::new_str(msg).map(JvmError::from),
        _ => crate::java::net::SocketException::new_str(msg).map(JvmError::from),
    };
    Err(e.unwrap_or_else(|e| e))
}

/// `JNU_ThrowByNameWithLastError(env, "java/net/SocketException", default)`
pub(super) fn socket_exception(default: &str) -> JvmError {
    let msg = String::from(net_posix::last_error_message(default));
    crate::java::net::SocketException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

/// `JNU_ThrowIOExceptionWithLastError`
pub(super) fn io_exception(default: &str) -> JvmError {
    let msg = String::from(net_posix::last_error_message(default));
    crate::java::io::IOException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

/// `java.net.preferIPv4Stack`（libnet JNI_OnLoad 读取，Boolean.getBoolean 语义）。
fn prefer_ipv4_stack() -> Result<bool> {
    let v = crate::java::lang::System::getProperty_str(String::of("java.net.preferIPv4Stack"))?;
    Ok(!v.is_jvm_null() && v.to_string().eq_ignore_ascii_case("true"))
}

/// `ipv6_available()`：协议栈支持 IPv6 且未设置 preferIPv4Stack。
pub(super) fn ipv6_available() -> Result<bool> {
    Ok(net_posix::ipv6_supported() && !prefer_ipv4_stack()?)
}

/// InetAddress → (地址字节, scope_id)。
fn address_of(ia: &InetAddress) -> Result<(Vec<u8>, u32)> {
    let bytes: Vec<u8> = ia.getAddress()?.to_vec().into_iter().map(|b| b as u8).collect();
    let obj = Object::from(Clone::clone(ia));
    let scope = if obj.is_instance_of("java/net/Inet6Address") {
        obj.try_cast::<Inet6Address>("java/net/Inet6Address")?.getScopeId()? as u32
    } else {
        0
    };
    Ok((bytes, scope))
}

/// `NET_InetAddressToSockaddr`：IPv6 地址在 IPv4 套接字上 → SocketException「Protocol family unavailable」。
fn to_sockaddr(ia: &InetAddress, port: i32, prefer_v6: bool) -> Result<SockAddr> {
    let (bytes, scope) = address_of(ia)?;
    net_posix::encode(&bytes, scope, port as u16, prefer_v6).map_err(|_| {
        crate::java::net::SocketException::new_str(String::from("Protocol family unavailable"))
            .map(JvmError::from)
            .unwrap_or_else(|e| e)
    })
}

fn to_jbytes(b: &[u8]) -> JArray<i8> {
    JArray::from(b.iter().map(|x| *x as i8).collect::<Vec<i8>>())
}

/// `NET_SockaddrToInetAddress`：v4 映射地址还原为 Inet4Address，IPv6 带非零 scope 时记入 scope_id。
fn inet_address(sa: &SockAddr) -> Result<(InetAddress, i32)> {
    let d = net_posix::decode(sa);
    let ia = if d.addr.len() == 4 {
        <InetAddress as From<Inet4Address>>::from(Inet4Address::new_str_arr_b(String::default(), to_jbytes(&d.addr))?)
    } else if d.scope_id != 0 {
        <InetAddress as From<Inet6Address>>::from(Inet6Address::new_str_arr_b_i(String::default(), to_jbytes(&d.addr), d.scope_id as i32)?)
    } else {
        <InetAddress as From<Inet6Address>>::from(Inet6Address::new_str_arr_b(String::default(), to_jbytes(&d.addr))?)
    };
    Ok((ia, d.port))
}

fn sock_name(fdo: &FileDescriptor, peer: bool) -> Result<Option<SockAddr>> {
    match net_posix::sock_name(fdo.__get_fd(), peer) {
        Ok(sa) => Ok(Some(sa)),
        Err(err) => socket_error(err).map(|_| None),
    }
}

/// Linux 允许绑定 127.x.x.255 形式的回环广播地址但后续操作失败，JDK 在 bind 前即报 EADDRNOTAVAIL。
#[cfg(target_os = "linux")]
fn bind_rejected(sa: &SockAddr) -> bool {
    if sa.family() != libc::AF_INET {
        return false;
    }
    // SAFETY: 族为 AF_INET
    let sin = unsafe { &*(sa.as_ptr() as *const libc::sockaddr_in) };
    u32::from_be(sin.sin_addr.s_addr) & 0x7f00_00ff == 0x7f00_00ff
}

#[cfg(not(target_os = "linux"))]
fn bind_rejected(_sa: &SockAddr) -> bool {
    false
}

impl Net {
    /// native `initIDs()`：缓存 JNI 类与方法 ID——无需。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native `socket0(boolean preferIPv6, boolean stream, boolean reuse, boolean fastLoopback)`：
    /// IPv6 套接字在 IPv4 可用时关闭 IPV6_V6ONLY（双栈）；reuse 设 SO_REUSEADDR；数据报套接字按平台补选项。
    /// fastLoopback 仅 Windows 使用。
    #[jvm_native]
    pub fn socket0(prefer_ipv6: bool, stream: bool, reuse: bool, _fast_loopback: bool) -> Result<i32> {
        let domain = if ipv6_available()? && prefer_ipv6 { libc::AF_INET6 } else { libc::AF_INET };
        let ty = if stream { libc::SOCK_STREAM } else { libc::SOCK_DGRAM };
        // SAFETY: 新建套接字；失败路径在返回前关闭
        let fd = unsafe { libc::socket(domain, ty, 0) };
        if fd < 0 {
            return socket_error(errno());
        }
        let fail = |msg: &str| -> Result<i32> {
            let e = socket_exception(msg);
            // SAFETY: 关闭本函数刚创建的 fd
            unsafe { libc::close(fd) };
            Err(e)
        };
        if domain == libc::AF_INET6 && net_posix::ipv4_supported() && net_posix::set_int(fd, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, 0) < 0 {
            return fail("Unable to set IPV6_V6ONLY");
        }
        if reuse && net_posix::set_int(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1) < 0 {
            return fail("Unable to set SO_REUSEADDR");
        }
        if !stream {
            if let Err(msg) = datagram_defaults(fd, domain) {
                return fail(msg);
            }
        }
        Ok(fd)
    }

    /// native `bind0(FileDescriptor, boolean preferIPv6, boolean useExclBind, InetAddress, int port)`。
    #[jvm_native]
    pub fn bind0(fdo: FileDescriptor, prefer_ipv6: bool, _use_excl_bind: bool, ia: InetAddress, port: i32) -> Result<()> {
        let sa = to_sockaddr(&ia, port, prefer_ipv6)?;
        if bind_rejected(&sa) {
            return socket_error(libc::EADDRNOTAVAIL).map(|_| ());
        }
        // SAFETY: sa 为已编码的 sockaddr
        if unsafe { libc::bind(fdo.__get_fd(), sa.as_ptr(), sa.len) } != 0 {
            socket_error(errno())?;
        }
        Ok(())
    }

    /// native `listen(FileDescriptor, int backlog)`。
    #[jvm_native]
    pub fn listen(fdo: FileDescriptor, backlog: i32) -> Result<()> {
        // SAFETY: listen 只作用于 fd
        if unsafe { libc::listen(fdo.__get_fd(), backlog) } < 0 {
            socket_error(errno())?;
        }
        Ok(())
    }

    /// native `connect0(boolean preferIPv6, FileDescriptor, InetAddress, int port)`：
    /// 成功 1；非阻塞进行中 UNAVAILABLE；被信号打断 INTERRUPTED。
    #[jvm_native]
    pub fn connect0(prefer_ipv6: bool, fdo: FileDescriptor, ia: InetAddress, port: i32) -> Result<i32> {
        let sa = to_sockaddr(&ia, port, prefer_ipv6)?;
        let fd = fdo.__get_fd();
        // SAFETY: sa 为已编码的 sockaddr
        let rv = crate::gil::blocking(|| unsafe { libc::connect(fd, sa.as_ptr(), sa.len) });
        if rv != 0 {
            return match errno() {
                libc::EINPROGRESS => Ok(IOS_UNAVAILABLE),
                libc::EINTR => Ok(IOS_INTERRUPTED),
                err => socket_error(err),
            };
        }
        Ok(1)
    }

    /// native `accept(FileDescriptor, FileDescriptor newfd, InetSocketAddress[] isaa)`：
    /// 新连接 fd 写入 newfd，对端地址写入 isaa[0]；ECONNABORTED 重试。
    #[jvm_native]
    pub fn accept(fdo: FileDescriptor, newfdo: FileDescriptor, isaa: JArray<InetSocketAddress>) -> Result<i32> {
        let fd = fdo.__get_fd();
        let mut sa = SockAddr::empty();
        let newfd = loop {
            sa.len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
            // SAFETY: 对端地址写入 sa.storage
            let n = crate::gil::blocking(|| unsafe { libc::accept(fd, sa.as_mut_ptr(), &mut sa.len) });
            if n >= 0 || errno() != libc::ECONNABORTED {
                break n;
            }
        };
        if newfd < 0 {
            return match errno() {
                libc::EAGAIN => Ok(IOS_UNAVAILABLE),
                libc::EINTR => Ok(IOS_INTERRUPTED),
                _ => Err(io_exception("Accept failed")),
            };
        }
        newfdo.__set_fd(newfd);
        let (ia, port) = inet_address(&sa)?;
        isaa.set(0, InetSocketAddress::new_inetaddress_i(ia, port)?)?;
        Ok(1)
    }

    /// native `shutdown(FileDescriptor, int how)`：SHUT_RD = 0 / SHUT_WR = 1 / 其余 SHUT_RDWR；未连接不报错。
    #[jvm_native]
    pub fn shutdown(fdo: FileDescriptor, how: i32) -> Result<()> {
        let how = match how {
            0 => libc::SHUT_RD,
            1 => libc::SHUT_WR,
            _ => libc::SHUT_RDWR,
        };
        // SAFETY: shutdown 只作用于 fd
        if unsafe { libc::shutdown(fdo.__get_fd(), how) } < 0 && errno() != libc::ENOTCONN {
            socket_error(errno())?;
        }
        Ok(())
    }

    /// native `localPort(FileDescriptor)`：getsockname 的端口。
    #[jvm_native]
    pub fn localPort(fdo: FileDescriptor) -> Result<i32> {
        Ok(sock_name(&fdo, false)?.map(|sa| net_posix::decode(&sa).port).unwrap_or(-1))
    }

    /// native `localInetAddress(FileDescriptor)`：getsockname 的地址。
    #[jvm_native]
    pub fn localInetAddress(fdo: FileDescriptor) -> Result<InetAddress> {
        match sock_name(&fdo, false)? {
            Some(sa) => Ok(inet_address(&sa)?.0),
            None => Ok(InetAddress::default()),
        }
    }

    /// native `remotePort(FileDescriptor)`：getpeername 的端口。
    #[jvm_native]
    pub fn remotePort(fdo: FileDescriptor) -> Result<i32> {
        Ok(sock_name(&fdo, true)?.map(|sa| net_posix::decode(&sa).port).unwrap_or(-1))
    }

    /// native `remoteInetAddress(FileDescriptor)`：getpeername 的地址。
    #[jvm_native]
    pub fn remoteInetAddress(fdo: FileDescriptor) -> Result<InetAddress> {
        match sock_name(&fdo, true)? {
            Some(sa) => Ok(inet_address(&sa)?.0),
            None => Ok(InetAddress::default()),
        }
    }
}

/// 数据报套接字的平台缺省（socket0）：Linux 关闭 IP_MULTICAST_ALL（内核不支持时忽略）、
/// IPv6 组播跳数设 1。
#[cfg(target_os = "linux")]
fn datagram_defaults(fd: i32, domain: i32) -> std::result::Result<(), &'static str> {
    if net_posix::set_int(fd, libc::IPPROTO_IP, libc::IP_MULTICAST_ALL, 0) < 0 && errno() != libc::ENOPROTOOPT {
        return Err("Unable to set IP_MULTICAST_ALL");
    }
    if domain == libc::AF_INET6 && net_posix::set_int(fd, libc::IPPROTO_IPV6, libc::IPV6_MULTICAST_HOPS, 1) < 0 {
        return Err("Unable to set IPV6_MULTICAST_HOPS");
    }
    Ok(())
}

/// macOS：SO_SNDBUF 至少能容纳最大 UDP 数据报（net.inet.udp.maxdgram 缺省 9216）。
#[cfg(not(target_os = "linux"))]
fn datagram_defaults(fd: i32, domain: i32) -> std::result::Result<(), &'static str> {
    if let Ok(size) = net_posix::get_int(fd, libc::SOL_SOCKET, libc::SO_SNDBUF) {
        let min = if domain == libc::AF_INET6 { 65527 } else { 65507 };
        if size < min {
            net_posix::set_int(fd, libc::SOL_SOCKET, libc::SO_SNDBUF, min);
        }
    }
    Ok(())
}
