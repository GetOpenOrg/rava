use crate::prelude::*;
use super::*;

impl Thread {
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
    pub fn getThreadGroup(&self) -> Result<Object> {
        Ok(Object::default())
    }
}
