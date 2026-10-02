//! `jdk/internal/misc/InnocuousThread` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 语义：系统线程工厂——守护线程、无访问控制上下文、运行后清空线程局部变量；
//! `newSystemThread` 线程归属顶层系统线程组、上下文类加载器为 null，`newThread` 为平台类加载器。
//! 典型调用方：进程回收线程（ProcessHandleImpl 的 "process reaper"）、Cleaner 公共线程。
//!
//! 原生侧没有 AccessControlContext / 线程组层级 / 类加载器可观测差异，线程局部变量随线程结束
//! 回收，故以普通 `Thread(ThreadGroup, Runnable, String, long)` 承载：守护 + 指定优先级 +
//! JDK 同形的线程名（`InnocuousThread-<n>`）。

use crate::prelude::*;
use super::innocuous_thread::InnocuousThread;
use crate::java::lang::{Runnable, Thread, ThreadGroup};
use std::sync::atomic::{AtomicI32, Ordering};

/// JDK `threadNumber`：未命名线程的序号（从 1 开始）。
static THREAD_NUMBER: AtomicI32 = AtomicI32::new(1);

const NORM_PRIORITY: i32 = 5;

fn system_thread(name: String, target: Runnable, stack_size: i64, priority: i32) -> Result<Thread> {
    let t = Thread::new_threadgroup_runnable_str_l(ThreadGroup::default(), target, name, stack_size)?;
    t.setDaemon(true)?;
    t.setPriority(priority)?;
    Ok(t)
}

impl InnocuousThread {
    /// `newName()`：`InnocuousThread-<n>`。
    #[jvm_boundary]
    pub fn newName() -> Result<String> {
        Ok(String::from(format!("InnocuousThread-{}", THREAD_NUMBER.fetch_add(1, Ordering::Relaxed))))
    }

    #[jvm_boundary]
    pub fn newThread_runnable(target: Runnable) -> Result<Thread> {
        system_thread(Self::newName()?, target, 0, NORM_PRIORITY)
    }

    #[jvm_boundary]
    pub fn newThread_str_runnable(name: String, target: Runnable) -> Result<Thread> {
        system_thread(name, target, 0, NORM_PRIORITY)
    }

    #[jvm_boundary]
    pub fn newThread_str_runnable_i(name: String, target: Runnable, priority: i32) -> Result<Thread> {
        system_thread(name, target, 0, priority)
    }

    #[jvm_boundary]
    pub fn newSystemThread_runnable(target: Runnable) -> Result<Thread> {
        system_thread(Self::newName()?, target, 0, NORM_PRIORITY)
    }

    #[jvm_boundary]
    pub fn newSystemThread_str_runnable(name: String, target: Runnable) -> Result<Thread> {
        system_thread(name, target, 0, NORM_PRIORITY)
    }

    #[jvm_boundary]
    pub fn newSystemThread_str_runnable_i(name: String, target: Runnable, priority: i32) -> Result<Thread> {
        system_thread(name, target, 0, priority)
    }

    #[jvm_boundary]
    pub fn newSystemThread_str_runnable_l_i(name: String, target: Runnable, stack_size: i64, priority: i32) -> Result<Thread> {
        system_thread(name, target, stack_size, priority)
    }
}
