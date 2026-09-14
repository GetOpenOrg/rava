// @jvm_class: java/lang/Thread
use java_runtime::prelude::*;
use super::*;

impl super::Thread {
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    // Thread.interrupt(): 简化实现，跳过 this != currentThread 引用比较
    pub fn interrupt(&self) -> Result<()> {
        self.interrupted.set(true);
        Ok(())
    }

    // Thread.getThreadGroup(): 简化实现，避免生成的 ternary stack underflow
    pub fn getThreadGroup(&self) -> Result<Object> {
        Ok(Object::default())
    }
}
