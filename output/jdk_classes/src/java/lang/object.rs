#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Object",
    super_class = "",
    interfaces  = "",
    access      = "public",
    source      = "Object.java",
))]
pub struct Object;

impl Object {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "getClass", descriptor = "()Ljava/lang/Class;", access = "public final native"))]
    pub fn getClass(&self) -> Result<Object> {
        todo!("native java/lang/Object.getClass")
    }

    #[cfg_attr(any(), java_native(name = "hashCode", descriptor = "()I", access = "public native"))]
    pub fn hashCode(&self) -> Result<i32> {
        todo!("native java/lang/Object.hashCode")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    #[cfg_attr(any(), java_native(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected native"))]
    pub fn clone(&self) -> Result<Object> {
        todo!("native java/lang/Object.clone")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = _t0.getName()?;
        String::new().append(&_t1)?;
        String::new().append(&String::from("@"))?;
        let _t2 = this.hashCode()?;
        let _t3: String = Integer::toHexString(_t2)?;
        String::new().append(&_t3)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_native(name = "notify", descriptor = "()V", access = "public final native"))]
    pub fn notify(&self) -> Result<()> {
        todo!("native java/lang/Object.notify")
    }

    #[cfg_attr(any(), java_native(name = "notifyAll", descriptor = "()V", access = "public final native"))]
    pub fn notifyAll(&self) -> Result<()> {
        todo!("native java/lang/Object.notifyAll")
    }

    #[cfg_attr(any(), java_method(name = "wait", descriptor = "()V", access = "public final"))]
    // java: wait()V
    pub fn wait(&self) -> Result<()> {
        let this = self;
        this.wait(0i64)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "wait", descriptor = "(J)V", access = "public final"))]
    // java: wait(J)V
    pub fn wait__l(&self, timeoutMillis: i64) -> Result<()> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        this.wait0(timeoutMillis)?;
        Blocker::end(comp)?;
        let mut e: i32 = todo!("stack underflow");
        let _t1: Object = Thread::currentThread()?;
        let mut thread: Object = _t1;
        let _t2 = thread.isVirtual()?;
        let _t3 = thread.getAndClearInterrupt()?;
        panic!("{}", /* e */);
        let mut local_7: bool = _t2;
        Blocker::end(comp)?;
        panic!("{}", /* local_7 */);
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "wait0", descriptor = "(J)V", access = "private final native"))]
    pub fn wait0(&self, arg0: i64) -> Result<()> {
        todo!("native java/lang/Object.wait0")
    }

    #[cfg_attr(any(), java_method(name = "wait", descriptor = "(JI)V", access = "public final"))]
    // java: wait(JI)V
    pub fn wait__l_i(&self, timeoutMillis: i64, arg_1: i32) -> Result<()> {
        let this = self;
        /* TODO: lcmp  */
        panic!("{}", /* IllegalArgumentException::new(String::from("timeoutMillis value is negative"))? */);
        panic!("{}", /* IllegalArgumentException::new(String::from("nanosecond timeout value out of range"))? */);
        /* TODO: lcmp  */
        timeoutMillis = (timeoutMillis).wrapping_add(1i64);
        this.wait(timeoutMillis)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "finalize", descriptor = "()V", access = "protected"))]
    pub fn finalize(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
