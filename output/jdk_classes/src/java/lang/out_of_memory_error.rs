#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/OutOfMemoryError",
    super_class = "java/lang/VirtualMachineError",
    interfaces  = "",
    access      = "public",
    source      = "OutOfMemoryError.java",
))]
pub struct OutOfMemoryError;

impl OutOfMemoryError {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/VirtualMachineError.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/VirtualMachineError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }
}
