#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/IndexOutOfBoundsException",
    super_class = "java/lang/RuntimeException",
    interfaces  = "",
    access      = "public",
    source      = "IndexOutOfBoundsException.java",
))]
pub struct IndexOutOfBoundsException;

impl IndexOutOfBoundsException {
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

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(index: i32) -> Result<Self> {
        let this = Self {};
        String::new().append(&String::from("Index out of range:"))?;
        String::new().append(&index)?;
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(J)V
    // java: <init>(J)V
    pub fn new__l(index: i64) -> Result<Self> {
        let this = Self {};
        String::new().append(&String::from("Index out of range:"))?;
        String::new().append(&index)?;
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }
}
