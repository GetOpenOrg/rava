//! `java/lang/reflect/Proxy` 的运行期类定义点（FS-R R4a，vm_intrinsics.toml「运行期类定义点」）。
//!
//! JDK 的 `newProxyInstance` 经 ProxyBuilder（动态模块映射）→ `defineProxyClass`（ProxyGenerator
//! 生成字节码 + JavaLangAccess.defineClass）定义 `$ProxyN` 后反射构造实例。原生二进制无运行期
//! 类定义：全部代理实例由 VM 支持类 `Proxy$Dyn`（字节码翻译）承载，实例携带接口列表。
//! 其余 Proxy 公开方法（isProxyClass / getInvocationHandler）按字节码翻译。

use crate::prelude::*;
use super::proxy::Proxy;
use super::Proxy_Dyn;
use crate::java::lang::{Class, ClassLoader};

impl Proxy {
    /// `newProxyInstance(ClassLoader, Class[], InvocationHandler)`：定义代理类并实例化 →
    /// `Proxy$Dyn.create`（接口校验、h 非空检查在其字节码体内）。
    #[jvm_native(upcalls = "java/lang/reflect/Proxy$Dyn.create:([Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object; java/lang/reflect/Proxy$Dyn.dispatch:(Ljava/lang/reflect/Method;[Ljava/lang/Object;)Ljava/lang/Object; java/lang/reflect/Proxy$Dyn.hashCode:()I java/lang/reflect/Proxy$Dyn.equals:(Ljava/lang/Object;)Z java/lang/reflect/Proxy$Dyn.toString:()Ljava/lang/String;")]
    pub fn newProxyInstance_classloader_arr_class_invocationhandler(
        _loader: ClassLoader, interfaces: JArray<Class>, h: Object) -> Result<Object> {
        Proxy_Dyn::create(interfaces, h.into())
    }
}
