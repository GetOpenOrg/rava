#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/InvalidObjectException",
    super_class = "java/io/ObjectStreamException",
    interfaces  = "",
    access      = "public",
    source      = "InvalidObjectException.java",
))]
pub struct InvalidObjectException;

impl InvalidObjectException {
    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(reason: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/ObjectStreamException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(reason: String, cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/io/ObjectStreamException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V */
        Ok(this)
    }
}
