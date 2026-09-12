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
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        Ok(this)
    }

    // java: getClass()Ljava/lang/Class;
    pub fn getClass(&self) -> Result<Object> {
        todo!("native java/lang/Object.getClass")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("native java/lang/Object.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    // java: clone()Ljava/lang/Object;
    pub fn clone(&self) -> Result<Object> {
        todo!("native java/lang/Object.clone")
    }

    // java: toString()Ljava/lang/String;
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

    // java: notify()V
    pub fn notify(&self) -> Result<()> {
        todo!("native java/lang/Object.notify")
    }

    // java: notifyAll()V
    pub fn notifyAll(&self) -> Result<()> {
        todo!("native java/lang/Object.notifyAll")
    }

    // java: wait()V
    // java: wait()V
    pub fn wait(&self) -> Result<()> {
        let this = self;
        this.wait(0i64)?;
        Ok(())
    }

    // java: wait(J)V
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
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_7: bool = _t2;
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: wait0(J)V
    pub fn wait0(&self, arg0: i64) -> Result<()> {
        todo!("native java/lang/Object.wait0")
    }

    // java: wait(JI)V
    // java: wait(JI)V
    pub fn wait__l_i(&self, timeoutMillis: i64, arg_1: i32) -> Result<()> {
        let this = self;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: lcmp  */
        timeoutMillis = (timeoutMillis).wrapping_add(1i64);
        this.wait(timeoutMillis)?;
        Ok(())
    }

    // java: finalize()V
    pub fn finalize(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
