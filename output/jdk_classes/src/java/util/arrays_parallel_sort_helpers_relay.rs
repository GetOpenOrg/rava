#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArraysParallelSortHelpers$Relay",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArraysParallelSortHelpers.java",
))]
pub struct ArraysParallelSortHelpers_Relay {
    #[cfg_attr(any(), java_field(name = "task", descriptor = "Ljava/util/concurrent/CountedCompleter;", access = "final"))]
    pub task: Field<Object>,
}

impl ArraysParallelSortHelpers_Relay {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;)V
    pub fn new(&self, task: Object) -> Result<()> {
        panic!("stub: java/util/ArraysParallelSortHelpers$Relay.<init>:(Ljava/util/concurrent/CountedCompleter;)V")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        panic!("stub: java/util/ArraysParallelSortHelpers$Relay.compute:()V")
    }

    // java: onCompletion(Ljava/util/concurrent/CountedCompleter;)V
    pub fn onCompletion(&self, t: Object) -> Result<()> {
        panic!("stub: java/util/ArraysParallelSortHelpers$Relay.onCompletion:(Ljava/util/concurrent/CountedCompleter;)V")
    }
}
