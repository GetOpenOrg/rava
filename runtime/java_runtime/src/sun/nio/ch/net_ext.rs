//! `sun/nio/ch/Net` 的 ACC_NATIVE 续（类 1，libnio `Net.c`）：套接字选项、poll、可用字节数与平台能力探测。
//! 选项转换对标 libnet `NET_GetSockOpt` / `NET_SetSockOpt`（mayNeedConversion 时）。

use crate::prelude::*;
use super::net::Net;
use super::net_impl::{ipv6_available, socket_error, socket_exception, io_exception};
use crate::java::io::FileDescriptor;
use crate::net_posix::{self, errno};

/// netinet/ip.h 的 `IPTOS_TOS_MASK | IPTOS_PREC_MASK`
const IPTOS_MASK: i32 = 0x1e | 0xe0;

/// 选项值的 C 形态：组播 TTL / LOOP 为 u_char，SO_LINGER 为 struct linger，其余 int。
enum OptVal {
    Byte(u8),
    Linger(libc::linger),
    Int(i32),
}

impl OptVal {
    fn for_option(level: i32, opt: i32) -> OptVal {
        if level == libc::IPPROTO_IP && (opt == libc::IP_MULTICAST_TTL || opt == libc::IP_MULTICAST_LOOP) {
            OptVal::Byte(0)
        } else if level == libc::SOL_SOCKET && opt == libc::SO_LINGER {
            OptVal::Linger(libc::linger { l_onoff: 0, l_linger: 0 })
        } else {
            OptVal::Int(0)
        }
    }

    fn ptr_len(&mut self) -> (*mut libc::c_void, libc::socklen_t) {
        match self {
            OptVal::Byte(b) => (b as *mut u8 as *mut libc::c_void, 1),
            OptVal::Linger(l) => (l as *mut libc::linger as *mut libc::c_void, std::mem::size_of::<libc::linger>() as libc::socklen_t),
            OptVal::Int(i) => (i as *mut i32 as *mut libc::c_void, std::mem::size_of::<i32>() as libc::socklen_t),
        }
    }
}

/// `NET_GetSockOpt` 的结果修正：Linux 的 SO_SNDBUF / SO_RCVBUF 报告内核加倍后的值，取半；
/// macOS 的 l_linger 按 unsigned short 解读。
fn get_conversion(level: i32, opt: i32, v: &mut OptVal) {
    if level != libc::SOL_SOCKET {
        return;
    }
    match v {
        OptVal::Int(n) if cfg!(target_os = "linux") && (opt == libc::SO_SNDBUF || opt == libc::SO_RCVBUF) => *n /= 2,
        OptVal::Linger(l) if cfg!(target_os = "macos") => l.l_linger = l.l_linger as u16 as _,
        _ => {}
    }
}

/// `NET_SetSockOpt` 的入参修正（int 选项）。
fn set_conversion(fd: i32, level: i32, opt: i32, n: &mut i32) -> std::result::Result<(), ()> {
    if level == libc::IPPROTO_IP && opt == libc::IP_TOS {
        tos_ipv6_companion(fd, *n)?;
        *n &= IPTOS_MASK;
    }
    if level == libc::SOL_SOCKET {
        if opt == libc::SO_SNDBUF || opt == libc::SO_RCVBUF {
            clamp_sockbuf(n);
        }
        if opt == libc::SO_RCVBUF && *n < 1024 {
            *n = 1024;
        }
        reuseaddr_companion(fd, opt, *n);
    }
    Ok(())
}

/// Linux：IPv6 可用时 IP_TOS 同时设 IPV6_FLOWINFO_SEND 与 IPV6_TCLASS。
#[cfg(target_os = "linux")]
fn tos_ipv6_companion(fd: i32, value: i32) -> std::result::Result<(), ()> {
    /// linux/in6.h（libc 未导出）
    const IPV6_FLOWINFO_SEND: i32 = 33;
    if net_posix::ipv6_supported() {
        if net_posix::set_int(fd, libc::IPPROTO_IPV6, IPV6_FLOWINFO_SEND, 1) < 0 {
            return Err(());
        }
        if net_posix::set_int(fd, libc::IPPROTO_IPV6, libc::IPV6_TCLASS, value) < 0 {
            return Err(());
        }
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn tos_ipv6_companion(_fd: i32, _value: i32) -> std::result::Result<(), ()> {
    Ok(())
}

/// BSD 系：缓冲区上限为 kern.ipc.maxsockbuf 的 4/5。
#[cfg(target_os = "macos")]
fn clamp_sockbuf(n: &mut i32) {
    let mut max: libc::c_int = 0;
    let mut len = std::mem::size_of::<libc::c_int>();
    // SAFETY: sysctlbyname 写入栈上 int
    let rc = unsafe {
        libc::sysctlbyname(c"kern.ipc.maxsockbuf".as_ptr(), &mut max as *mut _ as *mut libc::c_void, &mut len, std::ptr::null_mut(), 0)
    };
    if rc == 0 {
        let max = (max / 5) * 4;
        if *n > max {
            *n = max;
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn clamp_sockbuf(_n: &mut i32) {}

/// BSD 系：数据报套接字设 SO_REUSEADDR 时同设 SO_REUSEPORT。
#[cfg(target_os = "macos")]
fn reuseaddr_companion(fd: i32, opt: i32, value: i32) {
    if opt == libc::SO_REUSEADDR && net_posix::get_int(fd, libc::SOL_SOCKET, libc::SO_TYPE) == Ok(libc::SOCK_DGRAM) {
        net_posix::set_int(fd, libc::SOL_SOCKET, libc::SO_REUSEPORT, value);
    }
}

#[cfg(not(target_os = "macos"))]
fn reuseaddr_companion(_fd: i32, _opt: i32, _value: i32) {}

/// Linux：IPv6 套接字设 IPV6_TCLASS 时同设 IPv4 的 IP_TOS。
#[cfg(target_os = "linux")]
fn tclass_companion(fd: i32, level: i32, opt: i32, is_ipv6: bool, v: &mut OptVal) {
    if level == libc::IPPROTO_IPV6 && opt == libc::IPV6_TCLASS && is_ipv6 {
        let (p, len) = v.ptr_len();
        // SAFETY: p / len 指向 v 的 C 形态
        unsafe { libc::setsockopt(fd, libc::IPPROTO_IP, libc::IP_TOS, p, len) };
    }
}

#[cfg(not(target_os = "linux"))]
fn tclass_companion(_fd: i32, _level: i32, _opt: i32, _is_ipv6: bool, _v: &mut OptVal) {}

/// poll 超时裁剪到 [-1, INT_MAX]。
fn clamp_timeout(timeout: i64) -> i32 {
    timeout.clamp(-1, i32::MAX as i64) as i32
}

impl Net {
    /// native `isIPv6Available0()`
    #[jvm_native]
    pub fn isIPv6Available0() -> Result<bool> {
        ipv6_available()
    }

    /// native `isReusePortAvailable0()`：探测套接字上 SO_REUSEPORT 可设。
    #[jvm_native]
    pub fn isReusePortAvailable0() -> Result<bool> {
        // SAFETY: 探测套接字，返回前关闭
        unsafe {
            let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if fd < 0 {
                return Ok(false);
            }
            let ok = net_posix::set_int(fd, libc::SOL_SOCKET, libc::SO_REUSEPORT, 1) == 0;
            libc::close(fd);
            Ok(ok)
        }
    }

    /// native `isExclusiveBindAvailable()`：仅 Windows 有独占绑定，-1。
    #[jvm_native]
    pub fn isExclusiveBindAvailable() -> Result<i32> {
        Ok(-1)
    }

    /// native `shouldShutdownWriteBeforeClose0()`：仅 Windows 需要。
    #[jvm_native]
    pub fn shouldShutdownWriteBeforeClose0() -> Result<bool> {
        Ok(false)
    }

    /// native `getIntOption0(FileDescriptor, boolean mayNeedConversion, int level, int opt)`。
    /// SO_LINGER 开启时返回秒数，否则 -1。
    #[jvm_native]
    pub fn getIntOption0(fdo: FileDescriptor, may_need_conversion: bool, level: i32, opt: i32) -> Result<i32> {
        let mut v = OptVal::for_option(level, opt);
        let (p, mut len) = v.ptr_len();
        // SAFETY: p / len 指向 v 的 C 形态
        if unsafe { libc::getsockopt(fdo.__get_fd(), level, opt, p, &mut len) } < 0 {
            return Err(socket_exception("sun.nio.ch.Net.getIntOption"));
        }
        if may_need_conversion {
            get_conversion(level, opt, &mut v);
        }
        Ok(match v {
            OptVal::Byte(b) => b as i32,
            OptVal::Linger(l) => if l.l_onoff != 0 { l.l_linger as i32 } else { -1 },
            OptVal::Int(n) => n,
        })
    }

    /// native `setIntOption0(FileDescriptor, boolean mayNeedConversion, int level, int opt, int arg, boolean isIPv6)`。
    /// SO_LINGER：arg ≥ 0 开启并设秒数，否则关闭。
    #[jvm_native]
    pub fn setIntOption0(fdo: FileDescriptor, may_need_conversion: bool, level: i32, opt: i32, arg: i32, is_ipv6: bool) -> Result<()> {
        let fd = fdo.__get_fd();
        let mut v = match OptVal::for_option(level, opt) {
            OptVal::Byte(_) => OptVal::Byte(arg as u8),
            OptVal::Linger(_) => OptVal::Linger(if arg >= 0 {
                libc::linger { l_onoff: 1, l_linger: arg as _ }
            } else {
                libc::linger { l_onoff: 0, l_linger: 0 }
            }),
            OptVal::Int(_) => OptVal::Int(arg),
        };
        if may_need_conversion {
            if let OptVal::Int(n) = &mut v {
                if set_conversion(fd, level, opt, n).is_err() {
                    return Err(socket_exception("sun.nio.ch.Net.setIntOption"));
                }
            }
        }
        let (p, len) = v.ptr_len();
        // SAFETY: p / len 指向 v 的 C 形态
        if unsafe { libc::setsockopt(fd, level, opt, p, len) } < 0 {
            return Err(socket_exception("sun.nio.ch.Net.setIntOption"));
        }
        tclass_companion(fd, level, opt, is_ipv6, &mut v);
        Ok(())
    }

    /// native `poll(FileDescriptor, int events, long timeout)`：返回就绪事件；被信号打断返回 0。
    #[jvm_native]
    pub fn poll(fdo: FileDescriptor, events: i32, timeout: i64) -> Result<i32> {
        let mut pfd = libc::pollfd { fd: fdo.__get_fd(), events: events as i16, revents: 0 };
        // SAFETY: 单个栈上 pollfd
        let rv = crate::gil::blocking(|| unsafe { libc::poll(&mut pfd, 1, clamp_timeout(timeout)) });
        if rv >= 0 {
            return Ok(pfd.revents as i32);
        }
        match errno() {
            libc::EINTR => Ok(0),
            err => socket_error(err),
        }
    }

    /// native `pollConnect(FileDescriptor, long timeout)`：等待非阻塞连接完成；
    /// 已连接 true，超时 / 被打断 false，连接失败按 SO_ERROR 抛异常。
    #[jvm_native]
    pub fn pollConnect(fdo: FileDescriptor, timeout: i64) -> Result<bool> {
        let fd = fdo.__get_fd();
        let mut pfd = libc::pollfd { fd, events: libc::POLLOUT, revents: 0 };
        // SAFETY: 单个栈上 pollfd
        let rv = crate::gil::blocking(|| unsafe { libc::poll(&mut pfd, 1, clamp_timeout(timeout)) });
        if rv > 0 {
            let err = match net_posix::get_int(fd, libc::SOL_SOCKET, libc::SO_ERROR) {
                Ok(e) => e,
                Err(e) => return socket_error(e).map(|_| false),
            };
            if err != 0 {
                return socket_error(err).map(|_| false);
            }
            if pfd.revents & libc::POLLHUP != 0 {
                return socket_error(libc::ENOTCONN).map(|_| false);
            }
            return Ok(true);
        }
        if rv == 0 || errno() == libc::EINTR {
            return Ok(false);
        }
        Err(io_exception("poll failed"))
    }

    /// native `available(FileDescriptor)`：ioctl(FIONREAD)。
    #[jvm_native]
    pub fn available(fdo: FileDescriptor) -> Result<i32> {
        let mut count: libc::c_int = 0;
        // SAFETY: FIONREAD 写入栈上 int
        if unsafe { libc::ioctl(fdo.__get_fd(), libc::FIONREAD, &mut count) } != 0 {
            return socket_error(errno());
        }
        Ok(count)
    }

    #[jvm_native]
    pub fn pollinValue() -> Result<i16> {
        Ok(libc::POLLIN)
    }

    #[jvm_native]
    pub fn polloutValue() -> Result<i16> {
        Ok(libc::POLLOUT)
    }

    #[jvm_native]
    pub fn pollerrValue() -> Result<i16> {
        Ok(libc::POLLERR)
    }

    #[jvm_native]
    pub fn pollhupValue() -> Result<i16> {
        Ok(libc::POLLHUP)
    }

    #[jvm_native]
    pub fn pollnvalValue() -> Result<i16> {
        Ok(libc::POLLNVAL)
    }

    /// native `pollconnValue()`：连接完成以可写表示。
    #[jvm_native]
    pub fn pollconnValue() -> Result<i16> {
        Ok(libc::POLLOUT)
    }
}
