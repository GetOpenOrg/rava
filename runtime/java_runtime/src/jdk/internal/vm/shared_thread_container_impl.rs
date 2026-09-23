use crate::prelude::*;
use super::shared_thread_container::SharedThreadContainer;
use crate::java::lang::String;

// jdk.internal.vm.SharedThreadContainer：线程容器的簿记对象（名称 + 关闭位 +
// 注册键），服务真实 OS 线程的分层容器体系。本运行时线程层是单线程协作调度
// （S-11，java/lang/thread_impl.rs），容器不承担调度——工厂方法构造簿记对象
// 即可，父容器注册（ThreadContainers.registerContainer）的唯一用途是
// serviceability 工具枚举容器，无可观察行为，不引入注册表。

impl SharedThreadContainer {
    /// `SharedThreadContainer.create(String name)`（JDK 21 单参工厂）：
    /// `create(ThreadContainers.root(), name)`——root 容器无 owner，父容器
    /// 校验恒通过。消费方：ForkJoinPool 公共池构造（makeCommonPool 的
    /// `SharedThreadContainer.create("ForkJoinPool.commonPool")`），容器仅
    /// 挂在 pool.container 字段上，不进入执行路径。
    #[jvm_boundary]
    pub fn create_str(name: String) -> Result<SharedThreadContainer> {
        let mut stc = SharedThreadContainer::default();
        stc._init_not_null();
        stc.__set_name(name);
        Ok(stc)
    }
}
