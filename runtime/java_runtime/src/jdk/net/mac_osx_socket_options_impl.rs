//! `jdk/net/MacOSXSocketOptions` 的 ACC_NATIVE（类 1，libextnet `MacOSXSocketOptions.c`，macOS）。
//! 失败：ENOPROTOOPT 抛 UnsupportedOperationException「unsupported socket option」，其余抛
//! SocketException（strerror 文案）。
#![cfg(target_os = "macos")]

use crate::prelude::*;
use super::mac_osx_socket_options::MacOSXSocketOptions;
use crate::net_posix::{self, errno};

/// netinet6/in6.h（libc 未导出）
const IPV6_DONTFRAG: i32 = 62;

/// JNI `handleError`（rv < 0 时）。
fn option_error(default: &str) -> JvmError {
    if errno() == libc::ENOPROTOOPT {
        let e = crate::java::lang::UnsupportedOperationException::new_str(String::from("unsupported socket option"));
        return e.map(JvmError::from).unwrap_or_else(|e| e);
    }
    let msg = String::from(net_posix::last_error_message(default));
    crate::java::net::SocketException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

fn set_tcp(fd: i32, opt: i32, value: i32, msg: &str) -> Result<()> {
    if net_posix::set_int(fd, libc::IPPROTO_TCP, opt, value) < 0 {
        return Err(option_error(msg));
    }
    Ok(())
}

fn get_tcp(fd: i32, opt: i32, msg: &str) -> Result<i32> {
    net_posix::get_int(fd, libc::IPPROTO_TCP, opt).map_err(|_| option_error(msg))
}

/// TCP 选项可设（ENOPROTOOPT 视为不支持，其余错误视为支持——JNI 同款）。
fn tcp_option_supported(opt: i32) -> bool {
    match net_posix::probe_option(libc::PF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP, opt, Some(1)) {
        Ok(()) => true,
        Err(0) => false,
        Err(e) => e != libc::ENOPROTOOPT,
    }
}

fn dont_frag_level(is_ipv6: bool) -> (i32, i32) {
    if is_ipv6 {
        (libc::IPPROTO_IPV6, IPV6_DONTFRAG)
    } else {
        (libc::IPPROTO_IP, libc::IP_DONTFRAG)
    }
}

impl MacOSXSocketOptions {
    #[jvm_native]
    pub fn keepAliveOptionsSupported0() -> Result<bool> {
        Ok(tcp_option_supported(libc::TCP_KEEPALIVE) && tcp_option_supported(libc::TCP_KEEPINTVL) && tcp_option_supported(libc::TCP_KEEPCNT))
    }

    /// native `ipDontFragmentSupported0()`：IPv4 与 IPv6 数据报套接字上均可设 DONTFRAG。
    #[jvm_native]
    pub fn ipDontFragmentSupported0() -> Result<bool> {
        let v4 = net_posix::probe_option(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP, libc::IP_DONTFRAG, Some(1));
        let v6 = || net_posix::probe_option(libc::AF_INET6, libc::SOCK_DGRAM, libc::IPPROTO_IPV6, IPV6_DONTFRAG, Some(1));
        Ok(v4.is_ok() && v6().is_ok())
    }

    #[jvm_native]
    pub fn setTcpKeepAliveProbes0(fd: i32, value: i32) -> Result<()> {
        set_tcp(fd, libc::TCP_KEEPCNT, value, "set option TCP_KEEPCNT failed")
    }

    #[jvm_native]
    pub fn setTcpKeepAliveTime0(fd: i32, value: i32) -> Result<()> {
        set_tcp(fd, libc::TCP_KEEPALIVE, value, "set option TCP_KEEPALIVE failed")
    }

    #[jvm_native]
    pub fn setTcpKeepAliveIntvl0(fd: i32, value: i32) -> Result<()> {
        set_tcp(fd, libc::TCP_KEEPINTVL, value, "set option TCP_KEEPINTVL failed")
    }

    #[jvm_native]
    pub fn getTcpKeepAliveProbes0(fd: i32) -> Result<i32> {
        get_tcp(fd, libc::TCP_KEEPCNT, "get option TCP_KEEPCNT failed")
    }

    #[jvm_native]
    pub fn getTcpKeepAliveTime0(fd: i32) -> Result<i32> {
        get_tcp(fd, libc::TCP_KEEPALIVE, "get option TCP_KEEPALIVE failed")
    }

    #[jvm_native]
    pub fn getTcpKeepAliveIntvl0(fd: i32) -> Result<i32> {
        get_tcp(fd, libc::TCP_KEEPINTVL, "get option TCP_KEEPINTVL failed")
    }

    #[jvm_native]
    pub fn setIpDontFragment0(fd: i32, value: bool, is_ipv6: bool) -> Result<()> {
        let (level, opt) = dont_frag_level(is_ipv6);
        if net_posix::set_int(fd, level, opt, value as i32) < 0 {
            return Err(option_error("set option IP_DONTFRAGMENT failed"));
        }
        Ok(())
    }

    #[jvm_native]
    pub fn getIpDontFragment0(fd: i32, is_ipv6: bool) -> Result<bool> {
        let (level, opt) = dont_frag_level(is_ipv6);
        let v = net_posix::get_int(fd, level, opt).map_err(|_| option_error("get option IP_DONTFRAGMENT failed"))?;
        Ok(v != 0)
    }

    /// native `getSoPeerCred0(int fd)`：getpeereid，`(uid << 32) | gid`；失败抛异常。
    #[jvm_native]
    pub fn getSoPeerCred0(fd: i32) -> Result<i64> {
        let mut uid: libc::uid_t = 0;
        let mut gid: libc::gid_t = 0;
        // SAFETY: 写入栈上 uid / gid
        if unsafe { libc::getpeereid(fd, &mut uid, &mut gid) } < 0 {
            return Err(option_error("get peer eid failed"));
        }
        Ok(((uid as i64) << 32) | (gid as i64 & 0xffff_ffff))
    }
}
