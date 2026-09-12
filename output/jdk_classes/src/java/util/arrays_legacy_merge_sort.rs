#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$LegacyMergeSort",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "Arrays.java",
))]
pub struct Arrays_LegacyMergeSort;

impl Arrays_LegacyMergeSort {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }
}
