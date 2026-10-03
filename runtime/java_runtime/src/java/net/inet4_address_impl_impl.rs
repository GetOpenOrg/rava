//! `java/net/Inet4AddressImpl` 的 ACC_NATIVE（类 1，libnet `Inet4AddressImpl.c`）：仅 IPv4 的名字服务。
//! 生成类落在 `inet4_address_impl_t.rs`（与 Inet4Address 的共置文件同名主干）。

use crate::prelude::*;
use super::{Inet4Address, Inet4AddressImpl, InetAddress};
use crate::net_posix;

pub(super) fn unknown_host(msg: Option<std::string::String>) -> JvmError {
    let msg = msg.map(String::from).unwrap_or_default();
    super::UnknownHostException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

pub(super) fn to_jbytes(b: &[u8]) -> JArray<i8> {
    JArray::from(b.iter().map(|x| *x as i8).collect::<Vec<i8>>())
}

pub(super) fn host_arg(host: &String) -> Result<std::string::String> {
    if host.is_jvm_null() {
        let e = crate::java::lang::NullPointerException::new_str(String::from("host argument is null"));
        return Err(e.map(JvmError::from).unwrap_or_else(|e| e));
    }
    Ok(host.to_string())
}

impl Inet4AddressImpl {
    /// native `getLocalHostName()`：gethostname，失败为 "localhost"。
    #[jvm_native]
    pub fn getLocalHostName(&self) -> Result<String> {
        Ok(String::from(net_posix::host_name()))
    }

    /// native `lookupAllHostAddr(String)`：getaddrinfo(AF_INET)，结果的 hostName 为查询名；
    /// 失败抛 UnknownHostException（「host: gai_strerror」）。macOS 上本机名解析失败时取接口地址。
    #[jvm_native]
    pub fn lookupAllHostAddr_str(&self, host: String) -> Result<JArray<InetAddress>> {
        let name = host_arg(&host)?;
        let addrs = match net_posix::resolve(&name, libc::AF_INET) {
            Ok(a) => a,
            Err(msg) => net_posix::local_host_fallback(&name, false, false).ok_or_else(|| unknown_host(Some(msg)))?,
        };
        let mut out = Vec::with_capacity(addrs.len());
        for a in addrs {
            let ia = Inet4Address::new_str_arr_b(Clone::clone(&host), to_jbytes(&a.addr))?;
            out.push(<InetAddress as From<Inet4Address>>::from(ia));
        }
        Ok(JArray::from(out))
    }

    /// native `getHostByAddr(byte[])`：getnameinfo(NI_NAMEREQD)；无名字抛 UnknownHostException（消息为 null）。
    #[jvm_native]
    pub fn getHostByAddr(&self, addr: JArray<i8>) -> Result<String> {
        let bytes: Vec<u8> = addr.to_vec().into_iter().map(|b| b as u8).collect();
        net_posix::reverse(&bytes).map(String::from).ok_or_else(|| unknown_host(None))
    }
}
