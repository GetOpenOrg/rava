#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/ObjectStreamException",
    super_class = "java/io/IOException",
    interfaces  = "",
    access      = "public abstract",
    source      = "ObjectStreamException.java",
))]
pub struct ObjectStreamException;

impl ObjectStreamException {
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, message: String) -> Result<()> {
        todo!("abstract java/io/ObjectStreamException.<init>")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(&self, message: String, cause: Object) -> Result<()> {
        todo!("abstract java/io/ObjectStreamException.<init>")
    }

    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/io/ObjectStreamException.<init>")
    }

    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(&self, cause: Object) -> Result<()> {
        todo!("abstract java/io/ObjectStreamException.<init>")
    }
}
