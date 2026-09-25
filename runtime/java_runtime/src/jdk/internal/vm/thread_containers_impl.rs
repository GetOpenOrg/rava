//! `jdk/internal/vm/ThreadContainers` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 容器注册表只服务 serviceability 工具枚举线程容器（JDK 默认仅弱引用登记），
//! 无可观察行为——与 shared_thread_container_impl 同一取舍，不引入注册表。

use crate::prelude::*;
use super::thread_containers::ThreadContainers;
use super::thread_container::ThreadContainer;

impl ThreadContainers {
    /// `registerContainer(ThreadContainer)`：返回注册键（不跟踪 → null，JDK 在
    /// 未开启 TRACK_ALL_CONTAINERS 时同样可返回 null 键）。
    #[jvm_boundary]
    pub fn registerContainer(_container: ThreadContainer) -> Result<Object> {
        Ok(Object::default())
    }

    /// `deregisterContainer(Object key)`：null 键 no-op。
    #[jvm_boundary]
    pub fn deregisterContainer(_key: Object) -> Result<()> {
        Ok(())
    }
}
