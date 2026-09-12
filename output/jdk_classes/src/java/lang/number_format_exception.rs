#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/NumberFormatException",
    super_class = "java/lang/IllegalArgumentException",
    interfaces  = "",
    access      = "public",
    source      = "NumberFormatException.java",
))]
pub struct NumberFormatException;

impl NumberFormatException {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/lang/NumberFormatException.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/NumberFormatException.<init>")
    }

    // java: forInputString(Ljava/lang/String;I)Ljava/lang/NumberFormatException;
    pub fn forInputString(s: String, radix: i32) -> Result<Object> {
        todo!("abstract java/lang/NumberFormatException.forInputString")
    }

    // java: forCharSequence(Ljava/lang/CharSequence;III)Ljava/lang/NumberFormatException;
    pub fn forCharSequence(s: Object, beginIndex: i32, endIndex: i32, errorIndex: i32) -> Result<Object> {
        todo!("abstract java/lang/NumberFormatException.forCharSequence")
    }
}
