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
        panic!("stub: java/lang/CharSequence.length:()I")
    }

    // java: charAt(I)C
    pub fn charAt(&self, arg0: i32) -> Result<u16> {
        panic!("stub: java/lang/CharSequence.charAt:(I)C")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/lang/CharSequence.isEmpty:()Z")
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, arg0: i32, arg1: i32) -> Result<Object> {
        panic!("stub: java/lang/CharSequence.subSequence:(II)Ljava/lang/CharSequence;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/CharSequence.toString:()Ljava/lang/String;")
    }

    // java: chars()Ljava/util/stream/IntStream;
    pub fn chars(&self) -> Result<Object> {
        panic!("stub: java/lang/CharSequence.chars:()Ljava/util/stream/IntStream;")
    }

    // java: codePoints()Ljava/util/stream/IntStream;
    pub fn codePoints(&self) -> Result<Object> {
        panic!("stub: java/lang/CharSequence.codePoints:()Ljava/util/stream/IntStream;")
    }

    // java: compare(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)I
    pub fn compare(cs1: Object, cs2: Object) -> Result<i32> {
        panic!("stub: java/lang/CharSequence.compare:(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)I")
    }
}
