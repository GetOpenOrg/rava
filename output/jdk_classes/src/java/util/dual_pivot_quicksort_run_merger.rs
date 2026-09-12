#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$RunMerger",
    super_class = "java/util/concurrent/RecursiveTask",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_RunMerger {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Object>,
    #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub b: Field<Object>,
    #[cfg_attr(any(), java_field(name = "run", descriptor = "[I", access = "private final"))]
    pub run: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "aim", descriptor = "I", access = "private final"))]
    pub aim: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo", descriptor = "I", access = "private final"))]
    pub lo: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi", descriptor = "I", access = "private final"))]
    pub hi: Field<i32>,
}

impl DualPivotQuicksort_RunMerger {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;II[III)V
    pub fn new(&self, a: Object, b: Object, offset: i32, aim: i32, run: Vec<i32>, lo: i32, hi: i32) -> Result<()> {
        todo!("abstract java/util/DualPivotQuicksort$RunMerger.<init>")
    }

    // java: compute()Ljava/lang/Object;
    pub fn compute(&self) -> Result<Object> {
        todo!("abstract java/util/DualPivotQuicksort$RunMerger.compute")
    }

    // java: forkMe()Ljava/util/DualPivotQuicksort$RunMerger;
    pub fn forkMe(&self) -> Result<Object> {
        todo!("abstract java/util/DualPivotQuicksort$RunMerger.forkMe")
    }

    // java: getDestination()Ljava/lang/Object;
    pub fn getDestination(&self) -> Result<Object> {
        todo!("abstract java/util/DualPivotQuicksort$RunMerger.getDestination")
    }
}
