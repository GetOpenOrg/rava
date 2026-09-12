#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/NullPointerException",
    super_class = "java/lang/RuntimeException",
    interfaces  = "",
    access      = "public",
    source      = "NullPointerException.java",
))]
pub struct NullPointerException {
    #[cfg_attr(any(), java_field(name = "extendedMessageState", descriptor = "I", access = "private"))]
    pub extendedMessageState: Field<i32>,
    #[cfg_attr(any(), java_field(name = "extendedMessage", descriptor = "Ljava/lang/String;", access = "private"))]
    pub extendedMessage: Field<String>,
}

impl NullPointerException {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/lang/NullPointerException.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/NullPointerException.<init>")
    }

    // java: fillInStackTrace()Ljava/lang/Throwable;
    pub fn fillInStackTrace(&self) -> Result<Object> {
        todo!("abstract java/lang/NullPointerException.fillInStackTrace")
    }

    // java: getMessage()Ljava/lang/String;
    pub fn getMessage(&self) -> Result<String> {
        todo!("abstract java/lang/NullPointerException.getMessage")
    }

    // java: getExtendedNPEMessage()Ljava/lang/String;
    pub fn getExtendedNPEMessage(&self) -> Result<String> {
        todo!("native java/lang/NullPointerException.getExtendedNPEMessage")
    }
}
