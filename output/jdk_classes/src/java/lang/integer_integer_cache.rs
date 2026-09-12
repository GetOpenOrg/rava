#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Integer$IntegerCache",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "Integer.java",
))]
pub struct Integer_IntegerCache;

impl Integer_IntegerCache {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/lang/Integer$IntegerCache.<init>")
    }
}
