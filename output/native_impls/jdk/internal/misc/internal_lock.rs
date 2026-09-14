// jdk/internal/misc/InternalLock — Rust 实现
//
// JDK 21 的 InternalLock 是对 java.util.concurrent.locks.ReentrantLock 的薄封装，
// 用于 PrintStream / BufferedWriter 等 I/O 类的同步。
//
// 此处直接用 parking_lot::ReentrantMutex 实现，省去整条 JDK 锁类翻译链。
use java_runtime::prelude::*;
use super::*;

/// @field _mutex: std::sync::Arc<parking_lot::ReentrantMutex<()>>

use std::cell::RefCell;

thread_local! {
    // 每线程 guard 栈：支持同线程重入、lock/unlock 跨越方法调用边界。
    static GUARDS: RefCell<Vec<parking_lot::ReentrantMutexGuard<'static, ()>>>
        = RefCell::new(Vec::new());
}

impl super::InternalLock {
    pub fn lock(&self) -> Result<()> {
        // SAFETY:
        //   1. _mutex 存储在 Arc 中，引用计数保证在 InternalLock 实例存活期间 mutex 不释放。
        //   2. guard 存入同线程的 GUARDS 栈，不会跨线程使用，满足 Send 约束。
        //   3. unlock() 在同线程弹出 guard，生命周期配对由 Java try/finally 语义保证。
        let guard = unsafe {
            std::mem::transmute::<
                parking_lot::ReentrantMutexGuard<'_, ()>,
                parking_lot::ReentrantMutexGuard<'static, ()>,
            >(self._mutex.lock())
        };
        GUARDS.with(|g| g.borrow_mut().push(guard));
        Ok(())
    }

    pub fn unlock(&self) -> Result<()> {
        // 弹出栈顶 guard → drop → 释放锁
        GUARDS.with(|g| { g.borrow_mut().pop(); });
        Ok(())
    }
}
