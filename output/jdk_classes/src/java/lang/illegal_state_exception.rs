#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/IllegalStateException",
    super_class = "java/lang/RuntimeException",
    interfaces  = "",
    access      = "public",
    source      = "IllegalStateException.java",
))]
pub struct IllegalStateException;

impl IllegalStateException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/RuntimeException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(message: String, cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/Throwable;)V */
        Ok(this)
    }
}
