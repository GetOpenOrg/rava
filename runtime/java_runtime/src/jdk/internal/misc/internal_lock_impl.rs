use crate::prelude::*;
use super::*;

// 每实例可重入锁（FS-T5）：以实例身份键入监视器表（`crate::monitor::enter` / `exit`，
// 与 synchronized 同一实现：可重入、竞争时阻塞、按线程计数），lock / unlock
// 可跨方法调用配对（JDK InternalLock 包装的 ReentrantLock 语义）。

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
        crate::monitor::enter(self.__lock_identity(), false)
    }

    #[jvm_boundary]
    pub fn unlock(&self) -> Result<()> {
        crate::monitor::exit(self.__lock_identity())
    }

    fn __lock_identity(&self) -> usize {
        Object::from(Clone::clone(self)).0.__identity() as usize
    }
}
