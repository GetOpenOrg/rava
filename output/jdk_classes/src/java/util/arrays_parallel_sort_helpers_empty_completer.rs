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
    pub fn new(&self, p: Object) -> Result<()> {
        todo!("abstract java/util/ArraysParallelSortHelpers$EmptyCompleter.<init>")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        todo!("abstract java/util/ArraysParallelSortHelpers$EmptyCompleter.compute")
    }
}
