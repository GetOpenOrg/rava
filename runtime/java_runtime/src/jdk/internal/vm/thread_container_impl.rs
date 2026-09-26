//! `jdk/internal/vm/ThreadContainer` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 消费方：`ThreadPerTaskExecutor.<init>` 的 `super(true)`（Executors.newVirtualThread-
//! PerTaskExecutor 链）。JDK 构造体仅 `StackableScope(shared)`：共享容器不记录 owner
//! 线程（StackableScope 栈只服务 structured concurrency 的嵌套校验）。本档位线程层
//! 为OS 线程 + GIL（thread_impl，#42），容器不参与调度，构造即簿记对象。

use crate::prelude::*;
use super::thread_container::ThreadContainer;

impl ThreadContainer {
    /// `<init>(boolean shared)`（抽象类，仅经子类 super 调用）。
    #[jvm_boundary]
    pub fn new(shared: bool) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on(this, shared)
    }

    /// 构造器双入口的 `this` 形态（子类 invokespecial super(shared) 的落点）。
    #[doc(hidden)]
    pub fn __init_on(this: Self, _shared: bool) -> Result<Self> {
        Ok(this)
    }

    /// final `add(Thread)`：按字节码——pin/unpin（虚拟线程即 OS 线程、无 continuation，空操作）包裹
    /// 虚调用 `onStart(thread)`（子类簿记钩子）。消费方：JDK 25 `Thread.start(ThreadContainer)`
    ///（ForkJoinPool 工作线程经池容器启动）。
    #[jvm_boundary(upcalls = "jdk/internal/vm/ThreadContainer.onStart:(Ljava/lang/Thread;)V")]
    pub fn add(&self, thread: crate::java::lang::Thread) -> Result<()> {
        self.onStart(thread)
    }

    /// final `remove(Thread)`：同上，虚调用 `onExit(thread)`（线程终结 / 启动失败回滚）。
    #[jvm_boundary(upcalls = "jdk/internal/vm/ThreadContainer.onExit:(Ljava/lang/Thread;)V")]
    pub fn remove(&self, thread: crate::java::lang::Thread) -> Result<()> {
        self.onExit(thread)
    }

    /// `onStart(Thread)` / `onExit(Thread)`：JDK 基类默认空体（子类覆盖做成员簿记）。
    #[jvm_boundary]
    pub fn __impl_onStart(&self, _thread: crate::java::lang::Thread) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_onExit(&self, _thread: crate::java::lang::Thread) -> Result<()> {
        Ok(())
    }
}
