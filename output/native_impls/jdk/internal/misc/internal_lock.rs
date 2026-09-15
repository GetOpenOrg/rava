// jdk/internal/misc/InternalLock — 内部边界类（完整手写）
//
// JDK 21 的 InternalLock 是对 java.util.concurrent.locks.ReentrantLock 的薄封装，
// 用于 PrintStream / BufferedWriter 等 I/O 类的同步。
//
// 此处直接用 parking_lot::ReentrantMutex 实现，省去整条 JDK 锁类翻译链。
use java_runtime::prelude::*;
use super::*;

use std::cell::RefCell;

#[java_rta_macros::java_class(
    binary_name       = "jdk/internal/misc/InternalLock",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "InternalLock.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct InternalLock {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/util/concurrent/locks/ReentrantLock;", access = "private", modifiers = "final", is_static = false))]
    pub lock: JField<Object>,
    pub _mutex: java_runtime::MutexHolder,
}

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
            >(self._mutex.0.lock())
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
