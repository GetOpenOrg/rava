#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringIndexOutOfBoundsException",
    super_class = "java/lang/IndexOutOfBoundsException",
    interfaces  = "",
    access      = "public",
    source      = "StringIndexOutOfBoundsException.java",
))]
pub struct StringIndexOutOfBoundsException;

impl StringIndexOutOfBoundsException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/IndexOutOfBoundsException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/IndexOutOfBoundsException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(index: i32) -> Result<Self> {
        let this = Self {};
        String::new().append(&String::from("String index out of range:"))?;
        String::new().append(&index)?;
        /* invokespecial Method java/lang/IndexOutOfBoundsException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }
}
