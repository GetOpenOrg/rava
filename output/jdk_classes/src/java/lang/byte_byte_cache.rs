#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Byte$ByteCache",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "Byte.java",
))]
pub struct Byte_ByteCache;

impl Byte_ByteCache {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/lang/Byte$ByteCache.<init>")
    }
}
