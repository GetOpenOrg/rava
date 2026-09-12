#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArraysParallelSortHelpers$EmptyCompleter",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArraysParallelSortHelpers.java",
))]
pub struct ArraysParallelSortHelpers_EmptyCompleter;

impl ArraysParallelSortHelpers_EmptyCompleter {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;)V
    pub fn new(p: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
