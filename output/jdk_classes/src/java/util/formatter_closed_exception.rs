#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/FormatterClosedException",
    super_class = "java/lang/IllegalStateException",
    interfaces  = "",
    access      = "public",
    source      = "FormatterClosedException.java",
))]
pub struct FormatterClosedException;

impl FormatterClosedException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/IllegalStateException.<init>:()V */
        Ok(this)
    }
}
