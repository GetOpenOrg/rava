#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset$ThreadTrackHolder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Charset.java",
))]
pub struct Charset_ThreadTrackHolder;

impl Charset_ThreadTrackHolder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }
}
