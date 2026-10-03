//! `java/net/Inet6AddressImpl` 的 ACC_NATIVE（类 1，libnet `Inet6AddressImpl.c`）：双栈名字服务。
//! 生成类落在 `inet6_address_impl_t.rs`（与 Inet6Address 的共置文件同名主干）。

use crate::prelude::*;
use super::inet4_address_impl_impl::{host_arg, to_jbytes, unknown_host};
use super::{Inet4Address, Inet6Address, Inet6AddressImpl, InetAddress};
use crate::net_posix;

/// `InetAddressResolver.LookupPolicy` 特征位
const IPV4: i32 = 1 << 0;
const IPV6: i32 = 1 << 1;
const IPV4_FIRST: i32 = 1 << 2;
const IPV6_FIRST: i32 = 1 << 3;

fn family_of(characteristics: i32) -> i32 {
    match (characteristics & IPV4 != 0, characteristics & IPV6 != 0) {
        (true, false) => libc::AF_INET,
        (false, true) => libc::AF_INET6,
        _ => libc::AF_UNSPEC,
    }
}

impl Inet6AddressImpl {
    /// native `getLocalHostName()`：gethostname，失败为 "localhost"。
    #[jvm_native]
    pub fn getLocalHostName(&self) -> Result<String> {
        Ok(String::from(net_posix::host_name()))
    }

    /// native `lookupAllHostAddr(String, int characteristics)`：getaddrinfo（族由特征位决定），
    /// IPV4_FIRST / IPV6_FIRST 按族重排，否则保持系统顺序；IPv6 非零 scope 记入 scope_id。
    /// 失败抛 UnknownHostException（「host: gai_strerror」）。macOS 上本机名解析失败时取接口地址。
    #[jvm_native]
    pub fn lookupAllHostAddr_str_i(&self, host: String, characteristics: i32) -> Result<JArray<InetAddress>> {
        let name = host_arg(&host)?;
        let v6_first = characteristics & IPV6_FIRST != 0;
        let addrs = match net_posix::resolve(&name, family_of(characteristics)) {
            Ok(a) => a,
            Err(msg) => net_posix::local_host_fallback(&name, true, v6_first).ok_or_else(|| unknown_host(Some(msg)))?,
        };
        let ordered: Vec<net_posix::HostAddr> = if characteristics & (IPV4_FIRST | IPV6_FIRST) == 0 {
            addrs
        } else {
            let (v4, v6): (Vec<_>, Vec<_>) = addrs.into_iter().partition(|a| a.addr.len() == 4);
            if v6_first { v6.into_iter().chain(v4).collect() } else { v4.into_iter().chain(v6).collect() }
        };
        let mut out = Vec::with_capacity(ordered.len());
        for a in ordered {
            let ia = if a.addr.len() == 4 {
                <InetAddress as From<Inet4Address>>::from(Inet4Address::new_str_arr_b(Clone::clone(&host), to_jbytes(&a.addr))?)
            } else if a.scope_id != 0 {
                let v6 = Inet6Address::new_str_arr_b_i(Clone::clone(&host), to_jbytes(&a.addr), a.scope_id as i32)?;
                <InetAddress as From<Inet6Address>>::from(v6)
            } else {
                <InetAddress as From<Inet6Address>>::from(Inet6Address::new_str_arr_b(Clone::clone(&host), to_jbytes(&a.addr))?)
            };
            out.push(ia);
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
