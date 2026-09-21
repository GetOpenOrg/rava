use crate::prelude::*;
use super::*;

impl Thread {
    /// `Thread.registerNatives:()V`：JVM 内部的 JNI 方法注册钩子（HotSpot 在
    /// `<clinit>` 中调用）。转译运行时的 native 在编译期静态解析，注册动作无
    /// 对应物 → no-op（与 CDS.initializeFromArchive 同一处置）。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native currentThread()：每个 OS 线程对应一个平台线程对象（线程内唯一）。
    #[jvm_native]
    pub fn currentThread() -> Result<Thread> {
        thread_local! {
            static CURRENT: Thread = {
                let mut t = Thread::default();
                t._init_not_null();
                t
            };
        }
        Ok(CURRENT.with(Clone::clone))
    }

    #[jvm_native]
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    #[jvm_native]
    pub fn interrupt(&self) -> Result<()> {
        self.__set_interrupted(true);
        Ok(())
    }

    #[jvm_native]
    pub fn getThreadGroup(&self) -> Result<ThreadGroup> {
        Ok(ThreadGroup::default())
    }
}
