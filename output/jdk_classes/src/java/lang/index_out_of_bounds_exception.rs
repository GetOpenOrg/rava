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
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/lang/IndexOutOfBoundsException.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/IndexOutOfBoundsException.<init>")
    }

    // java: <init>(I)V
    pub fn new__i(&self, index: i32) -> Result<()> {
        todo!("abstract java/lang/IndexOutOfBoundsException.<init>")
    }

    // java: <init>(J)V
    pub fn new__l(&self, index: i64) -> Result<()> {
        todo!("abstract java/lang/IndexOutOfBoundsException.<init>")
    }
}
