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

    /// `start(Thread)`：按字节码——已关闭抛 IllegalStateException，否则经
    /// `JLA.start(thread, this)` 即 `Thread.start(ThreadContainer)` 启动（平台线程
    /// 经 start0 入模拟线程就绪队列，线程模型方案 A）。消费方：JDK 25
    /// ForkJoinPool.createWorker（工作线程经池容器启动；JDK 21 为 wt.start() 直调）。
    #[jvm_boundary(upcalls = "java/lang/Thread.start:(Ljdk/internal/vm/ThreadContainer;)V java/lang/IllegalStateException.<init>:()V")]
    pub fn __impl_start(&self, thread: crate::java::lang::Thread) -> Result<()> {
        if self.__get_closed() {
            return Err(JvmError::from(crate::java::lang::IllegalStateException::new()?));
        }
        thread.start_threadcontainer(Clone::clone(self).into())
    }
}
