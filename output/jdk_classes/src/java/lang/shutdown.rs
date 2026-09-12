#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Shutdown",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Shutdown.java",
))]
pub struct Shutdown;

impl Shutdown {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: add(IZLjava/lang/Runnable;)V
    pub fn add(slot: i32, registerShutdownInProgress: bool, hook: Object) -> Result<()> {
        String::new().append(&String::from("Invalid slot:"))?;
        String::new().append(&slot)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_3: Object = Shutdown::lock();
        /* TODO: monitorenter  */
        String::new().append(&String::from("Shutdown hook at slot"))?;
        String::new().append(&slot)?;
        String::new().append(&String::from("already registered"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: bool = VM::isShutdown()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Shutdown::hooks()[slot as usize] = hook;
        /* TODO: monitorexit  */
        let mut local_4: Object = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: runHooks()V
    pub fn runHooks() -> Result<()> {
        let mut i: Object = Shutdown::lock();
        /* TODO: monitorenter  */
        let _t0: bool = VM::isShutdown()?;
        /* TODO: monitorexit  */
        return Ok(());
        /* TODO: monitorexit  */
        let mut hook: Object = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        i = 0i32;
        loop {
            if i >= 10i32 { break; }
            let mut local_2: Object = Shutdown::lock();
            /* TODO: monitorenter  */
            Shutdown::currentRunningHook(i);
            hook = Shutdown::hooks()[i as usize].clone();
            /* TODO: monitorexit  */
            let mut local_3: Object = local_2;
            /* TODO: monitorexit  */
            return Err(JvmError::Custom("athrow".to_owned()));
            hook.run()?;
            hook = hook;
            i = i.wrapping_add(1i32);
        }
        VM::shutdown()?;
        Ok(())
    }

    // java: beforeHalt()V
    pub fn beforeHalt() -> Result<()> {
        todo!("native java/lang/Shutdown.beforeHalt")
    }

    // java: halt(I)V
    pub fn halt(status: i32) -> Result<()> {
        let mut local_1: Object = Shutdown::haltLock();
        /* TODO: monitorenter  */
        Shutdown::halt0(status)?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: halt0(I)V
    pub fn halt0(arg0: i32) -> Result<()> {
        todo!("native java/lang/Shutdown.halt0")
    }

    // java: exit(I)V
    pub fn exit(status: i32) -> Result<()> {
        Shutdown::logRuntimeExit(status)?;
        let mut local_1: i32 = 7i32;
        /* TODO: monitorenter  */
        Shutdown::beforeHalt()?;
        Shutdown::runHooks()?;
        Shutdown::halt(status)?;
        /* TODO: monitorexit  */
        let mut local_2: i32 = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: logRuntimeExit(I)V
    pub fn logRuntimeExit(status: i32) -> Result<()> {
        let _t0: Object = System::getLogger(String::from("java.lang.Runtime"))?;
        let mut log: Object = _t0;
        let _t1 = log.isLoggable(System_Logger_Level::DEBUG())?;
        String::new().append(&String::from("Runtime.exit("))?;
        String::new().append(&status)?;
        String::new().append(&String::from(")"))?;
        let mut throwable: Throwable = Throwable::new(String::new())?;
        String::new().append(&String::from("Runtime.exit() called with status:"))?;
        String::new().append(&status)?;
        log.log(System_Logger_Level::DEBUG(), String::new(), throwable)?;
        log = _t1;
        String::new().append(&String::from("Runtime.exit("))?;
        String::new().append(&status)?;
        String::new().append(&String::from(") logging failed:"))?;
        let _t2 = log.getMessage()?;
        String::new().append(&_t2)?;
        System::err().println(String::new())?;
        throwable = todo!("stack underflow");
        Ok(())
    }

    // java: shutdown()V
    pub fn shutdown() -> Result<()> {
        let mut local_0: i32 = 7i32;
        /* TODO: monitorenter  */
        Shutdown::runHooks()?;
        /* TODO: monitorexit  */
        let mut local_1: i32 = local_0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
