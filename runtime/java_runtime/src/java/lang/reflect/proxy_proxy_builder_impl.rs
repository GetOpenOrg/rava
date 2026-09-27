//! `java/lang/reflect/Proxy$ProxyBuilder.isProxyClass` 的 VM 承载（FS-R R4a）。
//!
//! JDK 以 reverseProxyCache（defineProxyClass 登记的「代理类 → 定义加载器」表）判定；原生二进制
//! 的代理类即 VM 支持类 `Proxy$Dyn`（newProxyInstance 内建，不经 defineProxyClass 登记），
//! 判定按类身份应答（与 intrinsics.txt 中 newProxyInstance 同一准入）。

use crate::prelude::*;
use super::proxy_proxy_builder::Proxy_ProxyBuilder;
use crate::java::lang::Class;

impl Proxy_ProxyBuilder {
    #[jvm_native]
    pub fn isProxyClass(c: Class) -> Result<bool> {
        Ok(format!("{}", c.__get_name()) == "java.lang.reflect.Proxy$Dyn")
    }
}
