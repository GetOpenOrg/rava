//! `jdk/net/LinuxSocketOptions` 的 ACC_NATIVE（类 1，libextnet `LinuxSocketOptions.c`，Linux）。
//! 失败：ENOPROTOOPT 抛 UnsupportedOperationException「unsupported socket option」，其余抛
//! SocketException（strerror 文案）。
#![cfg(target_os = "linux")]

use crate::prelude::*;
use super::linux_socket_options::LinuxSocketOptions;
use crate::net_posix::{self, errno};

/// asm-generic/socket.h（libc 未导出）
const SO_INCOMING_NAPI_ID: i32 = 56;

/// JNI `handleError`（rv < 0 时）。
fn option_error(default: &str) -> JvmError {
    if errno() == libc::ENOPROTOOPT {
        let e = crate::java::lang::UnsupportedOperationException::new_str(String::from("unsupported socket option"));
        return e.map(JvmError::from).unwrap_or_else(|e| e);
    }
    let msg = String::from(net_posix::last_error_message(default));
    crate::java::net::SocketException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

fn set_opt(fd: i32, level: i32, opt: i32, value: i32, msg: &str) -> Result<()> {
    if net_posix::set_int(fd, level, opt, value) < 0 {
        return Err(option_error(msg));
    }
    Ok(())
}

fn get_opt(fd: i32, level: i32, opt: i32, msg: &str) -> Result<i32> {
    net_posix::get_int(fd, level, opt).map_err(|_| option_error(msg))
}

/// `socketOptionSupported(level, optname)`：探测套接字上 getsockopt，ENOPROTOOPT 视为不支持。
fn option_supported(level: i32, opt: i32) -> bool {
    match net_posix::probe_option(libc::PF_INET, libc::SOCK_STREAM, level, opt, None) {
        Ok(()) => true,
        Err(0) => false,
        Err(e) => e != libc::ENOPROTOOPT,
    }
}

fn mtu_discover_level(is_ipv6: bool) -> (i32, i32) {
    if is_ipv6 {
        (libc::IPPROTO_IPV6, libc::IPV6_MTU_DISCOVER)
    } else {
        (libc::IPPROTO_IP, libc::IP_MTU_DISCOVER)
    }
}

impl LinuxSocketOptions {
    #[jvm_native]
    pub fn keepAliveOptionsSupported0() -> Result<bool> {
        Ok(option_supported(libc::SOL_TCP, libc::TCP_KEEPIDLE)
            && option_supported(libc::SOL_TCP, libc::TCP_KEEPCNT)
            && option_supported(libc::SOL_TCP, libc::TCP_KEEPINTVL))
    }

    #[jvm_native]
    pub fn quickAckSupported0() -> Result<bool> {
        Ok(option_supported(libc::SOL_SOCKET, libc::TCP_QUICKACK))
    }

    #[jvm_native]
    pub fn incomingNapiIdSupported0() -> Result<bool> {
        Ok(option_supported(libc::SOL_SOCKET, SO_INCOMING_NAPI_ID))
    }

    #[jvm_native]
    pub fn peerCredentialsSupported0() -> Result<bool> {
        Ok(true)
    }

    /// native `ipDontFragmentSupported0()`：IPv4 与 IPv6 数据报套接字上均可设 MTU_DISCOVER。
    #[jvm_native]
    pub fn ipDontFragmentSupported0() -> Result<bool> {
        let v4 = net_posix::probe_option(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP, libc::IP_MTU_DISCOVER, Some(libc::IP_PMTUDISC_DO));
        let v6 = || net_posix::probe_option(libc::AF_INET6, libc::SOCK_DGRAM, libc::IPPROTO_IPV6, libc::IPV6_MTU_DISCOVER, Some(libc::IP_PMTUDISC_DO));
        Ok(v4.is_ok() && v6().is_ok())
    }

    #[jvm_native]
    pub fn setQuickAck0(fd: i32, on: bool) -> Result<()> {
        set_opt(fd, libc::SOL_SOCKET, libc::TCP_QUICKACK, on as i32, "set option TCP_QUICKACK failed")
    }

    #[jvm_native]
    pub fn getQuickAck0(fd: i32) -> Result<bool> {
        Ok(get_opt(fd, libc::SOL_SOCKET, libc::TCP_QUICKACK, "get option TCP_QUICKACK failed")? != 0)
    }

    #[jvm_native]
    pub fn getIncomingNapiId0(fd: i32) -> Result<i32> {
        get_opt(fd, libc::SOL_SOCKET, SO_INCOMING_NAPI_ID, "get option SO_INCOMING_NAPI_ID failed")
    }

    #[jvm_native]
    pub fn setTcpkeepAliveProbes0(fd: i32, value: i32) -> Result<()> {
        set_opt(fd, libc::SOL_TCP, libc::TCP_KEEPCNT, value, "set option TCP_KEEPCNT failed")
    }

    #[jvm_native]
    pub fn setTcpKeepAliveTime0(fd: i32, value: i32) -> Result<()> {
        set_opt(fd, libc::SOL_TCP, libc::TCP_KEEPIDLE, value, "set option TCP_KEEPIDLE failed")
    }

    #[jvm_native]
    pub fn setTcpKeepAliveIntvl0(fd: i32, value: i32) -> Result<()> {
        set_opt(fd, libc::SOL_TCP, libc::TCP_KEEPINTVL, value, "set option TCP_KEEPINTVL failed")
    }

    #[jvm_native]
    pub fn getTcpkeepAliveProbes0(fd: i32) -> Result<i32> {
        get_opt(fd, libc::SOL_TCP, libc::TCP_KEEPCNT, "get option TCP_KEEPCNT failed")
    }

    #[jvm_native]
    pub fn getTcpKeepAliveTime0(fd: i32) -> Result<i32> {
        get_opt(fd, libc::SOL_TCP, libc::TCP_KEEPIDLE, "get option TCP_KEEPIDLE failed")
    }

    #[jvm_native]
    pub fn getTcpKeepAliveIntvl0(fd: i32) -> Result<i32> {
        get_opt(fd, libc::SOL_TCP, libc::TCP_KEEPINTVL, "get option TCP_KEEPINTVL failed")
    }

    #[jvm_native]
    pub fn setIpDontFragment0(fd: i32, value: bool, is_ipv6: bool) -> Result<()> {
        let (level, opt) = mtu_discover_level(is_ipv6);
        let v = if value { libc::IP_PMTUDISC_DO } else { libc::IP_PMTUDISC_DONT };
        set_opt(fd, level, opt, v, "set option IP_DONTFRAGMENT failed")
    }

    #[jvm_native]
    pub fn getIpDontFragment0(fd: i32, is_ipv6: bool) -> Result<bool> {
        let (level, opt) = mtu_discover_level(is_ipv6);
        Ok(get_opt(fd, level, opt, "get option IP_DONTFRAGMENT failed")? == libc::IP_PMTUDISC_DO)
    }

    /// native `getSoPeerCred0(int fd)`：SO_PEERCRED，`(uid << 32) | gid`；失败抛异常。
    #[jvm_native]
    pub fn getSoPeerCred0(fd: i32) -> Result<i64> {
        // SAFETY: ucred 为纯 C 结构，全零合法；getsockopt 写入栈上结构
        let mut cred: libc::ucred = unsafe { std::mem::zeroed() };
        let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
        let rv = unsafe { libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_PEERCRED, &mut cred as *mut _ as *mut libc::c_void, &mut len) };
        if rv < 0 {
            return Err(option_error("get SO_PEERCRED failed"));
        }
        Ok(((cred.uid as i64) << 32) | (cred.gid as i64 & 0xffff_ffff))
    }
}
