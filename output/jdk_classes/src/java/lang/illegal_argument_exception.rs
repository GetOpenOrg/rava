#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/IllegalArgumentException",
    super_class = "java/lang/RuntimeException",
    interfaces  = "",
    access      = "public",
    source      = "IllegalArgumentException.java",
))]
pub struct IllegalArgumentException;

impl IllegalArgumentException {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/IllegalArgumentException.<init>:()V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(&self, message: String, cause: Object) -> Result<()> {
        panic!("stub: java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(&self, cause: Object) -> Result<()> {
        panic!("stub: java/lang/IllegalArgumentException.<init>:(Ljava/lang/Throwable;)V")
    }
}
