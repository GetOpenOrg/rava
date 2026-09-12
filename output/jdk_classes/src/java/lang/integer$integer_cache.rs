#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Integer$IntegerCache",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "Integer.java",
))]
pub struct Integer_IntegerCache;

impl Integer_IntegerCache {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }
}
