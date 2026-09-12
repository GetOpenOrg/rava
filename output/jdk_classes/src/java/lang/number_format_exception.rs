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
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/IllegalArgumentException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: forInputString(Ljava/lang/String;I)Ljava/lang/NumberFormatException;
    pub fn forInputString(s: String, radix: i32) -> Result<Object> {
        String::new().append(&String::from("For input string: \""))?;
        String::new().append(&s)?;
        String::new().append(&String::from("\""))?;
        String::new().append(&String::from("under radix"))?;
        String::new().append(&radix)?;
        String::from("").append(&String::new())?;
        /* invokespecial Method java/lang/NumberFormatException.<init>:(Ljava/lang/String;)V */
        Ok(radix)
    }

    // java: forCharSequence(Ljava/lang/CharSequence;III)Ljava/lang/NumberFormatException;
    pub fn forCharSequence(s: Object, beginIndex: i32, endIndex: i32, errorIndex: i32) -> Result<Object> {
        String::new().append(&String::from("Error at index"))?;
        String::new().append(&(errorIndex).wrapping_sub(beginIndex))?;
        String::new().append(&String::from("in: \""))?;
        let _t0 = s.subSequence(beginIndex, endIndex)?;
        String::new().append(&_t0)?;
        String::new().append(&String::from("\""))?;
        Ok(NumberFormatException::new(String::new())?)
    }
}
