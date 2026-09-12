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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "lock", descriptor = "()Ljava/lang/Object;", access = "abstract"))]
    pub fn lock(&self) -> Result<Object> {
        todo!("abstract java/lang/Throwable$PrintStreamOrWriter.lock")
    }

    #[cfg_attr(any(), java_method(name = "isLockedByCurrentThread", descriptor = "()Z"))]
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

    #[cfg_attr(any(), java_native(name = "println", descriptor = "(Ljava/lang/Object;)V", access = "abstract"))]
    pub fn println(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/lang/Throwable$PrintStreamOrWriter.println")
    }
}
