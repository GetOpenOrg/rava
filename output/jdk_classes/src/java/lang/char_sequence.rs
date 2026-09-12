#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharSequence",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "CharSequence.java",
))]
pub struct CharSequence;

impl CharSequence {
    // java: length()I
    pub fn length(&self) -> Result<i32> {
        todo!("abstract java/lang/CharSequence.length")
    }

    // java: charAt(I)C
    pub fn charAt(&self, arg0: i32) -> Result<u16> {
        todo!("abstract java/lang/CharSequence.charAt")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/lang/CharSequence.isEmpty")
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/lang/CharSequence.subSequence")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/CharSequence.toString")
    }

    // java: chars()Ljava/util/stream/IntStream;
    pub fn chars(&self) -> Result<Object> {
        todo!("abstract java/lang/CharSequence.chars")
    }

    // java: codePoints()Ljava/util/stream/IntStream;
    pub fn codePoints(&self) -> Result<Object> {
        todo!("abstract java/lang/CharSequence.codePoints")
    }

    // java: compare(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)I
    pub fn compare(cs1: Object, cs2: Object) -> Result<i32> {
        todo!("abstract java/lang/CharSequence.compare")
    }
}
