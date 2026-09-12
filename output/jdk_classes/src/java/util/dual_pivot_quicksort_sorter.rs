#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$Sorter",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_Sorter {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Object>,
    #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub b: Field<Object>,
    #[cfg_attr(any(), java_field(name = "low", descriptor = "I", access = "private final"))]
    pub low: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private final"))]
    pub depth: Field<i32>,
}

impl DualPivotQuicksort_Sorter {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;Ljava/lang/Object;IIII)V
    pub fn new(&self, parent: Object, a: Object, b: Object, low: i32, size: i32, offset: i32, depth: i32) -> Result<()> {
        todo!("abstract java/util/DualPivotQuicksort$Sorter.<init>")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        todo!("abstract java/util/DualPivotQuicksort$Sorter.compute")
    }

    // java: onCompletion(Ljava/util/concurrent/CountedCompleter;)V
    pub fn onCompletion(&self, caller: Object) -> Result<()> {
        todo!("abstract java/util/DualPivotQuicksort$Sorter.onCompletion")
    }

    // java: forkSorter(III)V
    pub fn forkSorter(&self, depth: i32, low: i32, high: i32) -> Result<()> {
        todo!("abstract java/util/DualPivotQuicksort$Sorter.forkSorter")
    }
}
