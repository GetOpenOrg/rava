#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/CharacterCodingException",
    super_class = "java/io/IOException",
    interfaces  = "",
    access      = "public",
    source      = "CharacterCodingException.java",
))]
pub struct CharacterCodingException;

impl CharacterCodingException {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/IOException.<init>:()V */
        Ok(this)
    }
}
