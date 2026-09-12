#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Terminator",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Terminator.java",
))]
pub struct Terminator;

impl Terminator {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "setup", descriptor = "()V", access = "static"))]
    pub fn setup() -> Result<()> {
        return Ok(());
        let mut sh: Terminator_1 = Terminator_1::new()?;
        Terminator::handler(sh);
        let _t0: Object = Signal::handle(Signal::new(String::from("HUP"))?, sh)?;
        let mut local_1: Object = Terminator::handler();
        let _t1: Object = Signal::handle(Signal::new(String::from("INT"))?, sh)?;
        local_1 = todo!("stack underflow");
        let _t2: Object = Signal::handle(Signal::new(String::from("TERM"))?, sh)?;
        local_1 = todo!("stack underflow");
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "teardown", descriptor = "()V", access = "static"))]
    pub fn teardown() -> Result<()> {
        Ok(())
    }
}
