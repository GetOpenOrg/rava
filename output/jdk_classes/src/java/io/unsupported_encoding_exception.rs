#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/UnsupportedEncodingException",
    super_class = "java/io/IOException",
    interfaces  = "",
    access      = "public",
    source      = "UnsupportedEncodingException.java",
))]
pub struct UnsupportedEncodingException;

impl UnsupportedEncodingException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/IOException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/IOException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }
}
