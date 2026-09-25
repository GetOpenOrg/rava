//! `jdk/internal/vm/ThreadContainer` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 消费方：`ThreadPerTaskExecutor.<init>` 的 `super(true)`（Executors.newVirtualThread-
//! PerTaskExecutor 链）。JDK 构造体仅 `StackableScope(shared)`：共享容器不记录 owner
//! 线程（StackableScope 栈只服务 structured concurrency 的嵌套校验）。本档位线程层
//! 为单线程协作调度（thread_impl），容器不参与调度，构造即簿记对象。

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
}
