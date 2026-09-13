#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$Merger",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_Merger {
    #[cfg_attr(any(), java_field(name = "dst", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub dst: Field<Object>,
    #[cfg_attr(any(), java_field(name = "a1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a1: Field<Object>,
    #[cfg_attr(any(), java_field(name = "a2", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a2: Field<Object>,
    #[cfg_attr(any(), java_field(name = "k", descriptor = "I", access = "private final"))]
    pub k: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo1", descriptor = "I", access = "private final"))]
    pub lo1: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi1", descriptor = "I", access = "private final"))]
    pub hi1: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo2", descriptor = "I", access = "private final"))]
    pub lo2: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi2", descriptor = "I", access = "private final"))]
    pub hi2: Field<i32>,
}

impl DualPivotQuicksort_Merger {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V
    pub fn new(&self, parent: Object, dst: Object, k: i32, a1: Object, lo1: i32, hi1: i32, a2: Object, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort$Merger.<init>:(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort$Merger.compute:()V")
    }

    // java: forkMerger(Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V
    pub fn forkMerger(&self, dst: Object, k: i32, a1: Object, lo1: i32, hi1: i32, a2: Object, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort$Merger.forkMerger:(Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V")
    }
}
