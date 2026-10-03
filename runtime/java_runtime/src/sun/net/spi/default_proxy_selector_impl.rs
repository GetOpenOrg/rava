//! `sun/net/spi/DefaultProxySelector` 的 ACC_NATIVE（类 1，libnet `DefaultProxySelector.c`）。
//!
//! 仅 `java.net.useSystemProxies=true` 时由类初始化调用 init()；平台查询在
//! `default_proxy_selector_ext.rs`（macOS CFNetwork / Linux GIO、GConf），这里只把查询结果
//! 构造成 `Proxy[]`（JNI `proxy_util.c` 的 createProxy：未解析地址 + HTTP / SOCKS 类型）。

use crate::prelude::*;
use super::default_proxy_selector::DefaultProxySelector;
use super::default_proxy_selector_ext::{self as system, SystemProxy};
use crate::java::net::{InetSocketAddress, Proxy, Proxy_Type, SocketAddress};

fn create_proxy(ty: Proxy_Type, host: &str, port: i32) -> Result<Proxy> {
    let sa = InetSocketAddress::createUnresolved(String::from(host), port)?;
    Proxy::new_proxy_type_socketaddress(ty, <SocketAddress as From<InetSocketAddress>>::from(sa))
}

impl DefaultProxySelector {
    /// native `init()`：探测平台系统代理设施，可用返回 true。
    #[jvm_native]
    pub fn init() -> Result<bool> {
        Ok(system::init())
    }

    /// native `getSystemProxies(String proto, String host)`：按系统配置给出 `proto://host` 的代理列表；
    /// 无设施 / 查询失败 / 条目不完整时返回 null（由 Java 侧回落为 DIRECT）。
    #[jvm_native]
    pub fn getSystemProxies(&self, proto: String, host: String) -> Result<JArray<Proxy>> {
        if proto.is_jvm_null() || host.is_jvm_null() {
            return Ok(JArray::default());
        }
        let Some(entries) = system::lookup(&proto.to_string(), &host.to_string()) else {
            return Ok(JArray::default());
        };
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            out.push(match e {
                SystemProxy::Direct => Proxy::NO_PROXY()?,
                SystemProxy::Http(h, p) => create_proxy(Proxy_Type::HTTP()?, &h, p)?,
                SystemProxy::Socks(h, p) => create_proxy(Proxy_Type::SOCKS()?, &h, p)?,
            });
        }
        Ok(JArray::from(out))
    }
}
