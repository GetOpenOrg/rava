use java_runtime::prelude::*;
use super::*;

use std::cell::RefCell;
use parking_lot::ReentrantMutex;

// 全局可重入锁：单线程 JVM 模拟环境下所有 InternalLock 实例共享同一把锁
static GLOBAL: ReentrantMutex<()> = ReentrantMutex::new(());

thread_local! {
    // guard 栈支持同线程重入：lock/unlock 可跨方法调用配对
    static GUARDS: RefCell<Vec<parking_lot::ReentrantMutexGuard<'static, ()>>>
        = RefCell::new(Vec::new());
}

impl InternalLock {
    #[jvm_boundary]
    pub fn lock(&self) -> Result<()> {
        let guard = unsafe {
            std::mem::transmute::<
                parking_lot::ReentrantMutexGuard<'_, ()>,
                parking_lot::ReentrantMutexGuard<'static, ()>,
            >(GLOBAL.lock())
        };
        GUARDS.with(|g| g.borrow_mut().push(guard));
        Ok(())
    }

    #[jvm_boundary]
    pub fn unlock(&self) -> Result<()> {
        GUARDS.with(|g| { g.borrow_mut().pop(); });
        Ok(())
    }
}
