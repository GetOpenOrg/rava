use crate::prelude::*;
use super::*;

use crate::sync_model::__RefSlot as RefCell;
use parking_lot::ReentrantMutex;

// 全局可重入锁：单线程 JVM 模拟环境下所有 InternalLock 实例共享同一把锁
static GLOBAL: ReentrantMutex<()> = ReentrantMutex::new(());

thread_local! {
    // guard 栈支持同线程重入：lock/unlock 可跨方法调用配对
    static GUARDS: RefCell<Vec<parking_lot::ReentrantMutexGuard<'static, ()>>>
        = RefCell::new(Vec::new());
}

impl InternalLock {
    /// 对应 -Djdk.io.useMonitors=true 的取值：返回 null，
    /// 调用方（PrintStream/Writer 等）改走 synchronized(this) 监视器路径。
    #[jvm_boundary]
    pub fn newLockOrNull() -> Result<InternalLock> {
        Ok(InternalLock::default())
    }

    /// 同一取值（jdk.io.useMonitors=true）：沿用调用方对象自身作监视器。
    #[jvm_boundary]
    pub fn newLockOr(obj: Object) -> Result<Object> {
        Ok(obj)
    }

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
