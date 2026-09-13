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
        panic!("stub: java/lang/NullPointerException.<init>:()V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/NullPointerException.<init>:(Ljava/lang/String;)V")
    }

    // java: fillInStackTrace()Ljava/lang/Throwable;
    pub fn fillInStackTrace(&self) -> Result<Object> {
        panic!("stub: java/lang/NullPointerException.fillInStackTrace:()Ljava/lang/Throwable;")
    }

    // java: getMessage()Ljava/lang/String;
    pub fn getMessage(&self) -> Result<String> {
        panic!("stub: java/lang/NullPointerException.getMessage:()Ljava/lang/String;")
    }

    // java: getExtendedNPEMessage()Ljava/lang/String;
    pub fn getExtendedNPEMessage(&self) -> Result<String> {
        panic!("native: java/lang/NullPointerException.getExtendedNPEMessage:()Ljava/lang/String;")
    }
}
