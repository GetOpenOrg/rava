#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/StreamCorruptedException",
    super_class = "java/io/ObjectStreamException",
    interfaces  = "",
    access      = "public",
    source      = "StreamCorruptedException.java",
))]
pub struct StreamCorruptedException;

impl StreamCorruptedException {
    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(reason: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/ObjectStreamException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/ObjectStreamException.<init>:()V */
        Ok(this)
    }
}
