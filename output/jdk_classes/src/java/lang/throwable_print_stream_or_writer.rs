#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Throwable$PrintStreamOrWriter",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "abstract",
    source      = "Throwable.java",
))]
pub struct Throwable_PrintStreamOrWriter;

impl Throwable_PrintStreamOrWriter {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: lock()Ljava/lang/Object;
    pub fn lock(&self) -> Result<Object> {
        todo!("abstract java/lang/Throwable$PrintStreamOrWriter.lock")
    }

    // java: isLockedByCurrentThread()Z
    pub fn isLockedByCurrentThread(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.lock()?;
        let mut lock: Object = _t0;
        let mut locker: Object = lock;
        let _t1 = locker.isHeldByCurrentThread()?;
        return Ok(_t1);
        let _t2: bool = Thread::holdsLock(lock)?;
        Ok(_t2)
    }

    // java: println(Ljava/lang/Object;)V
    pub fn println(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/lang/Throwable$PrintStreamOrWriter.println")
    }
}
