#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/RuntimeException",
    super_class = "java/lang/Exception",
    interfaces  = "",
    access      = "public",
    source      = "RuntimeException.java",
))]
pub struct RuntimeException;

impl RuntimeException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Exception.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(message: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Exception.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(message: String, cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Exception.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Exception.<init>:(Ljava/lang/Throwable;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V
    pub fn new__str_throwa_z_z(message: String, cause: Object, enableSuppression: bool, writableStackTrace: bool) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Exception.<init>:(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V */
        Ok(this)
    }
}
