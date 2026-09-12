#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Throwable$SentinelHolder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Throwable.java",
))]
pub struct Throwable_SentinelHolder;

impl Throwable_SentinelHolder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }
}
