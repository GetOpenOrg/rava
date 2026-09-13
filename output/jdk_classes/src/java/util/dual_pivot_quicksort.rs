#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort;

impl DualPivotQuicksort {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.<init>:()V")
    }

    // java: getDepth(II)I
    pub fn getDepth(parallelism: i32, size: i32) -> Result<i32> {
        panic!("stub: java/util/DualPivotQuicksort.getDepth:(II)I")
    }

    // java: sort([IIII)V
    pub fn sort__arr_i_i_i_i(a: Vec<i32>, parallelism: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([IIII)V")
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V
    pub fn sort__dualpi_arr_i_i_i_i(sorter: Object, a: Vec<i32>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V")
    }

    // java: mixedInsertionSort([IIII)V
    pub fn mixedInsertionSort__arr_i_i_i_i(a: Vec<i32>, low: i32, end: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([IIII)V")
    }

    // java: insertionSort([III)V
    pub fn insertionSort__arr_i_i_i(a: Vec<i32>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([III)V")
    }

    // java: heapSort([III)V
    pub fn heapSort__arr_i_i_i(a: Vec<i32>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.heapSort:([III)V")
    }

    // java: pushDown([IIIII)V
    pub fn pushDown__arr_i_i_i_i_i(a: Vec<i32>, p: i32, value: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.pushDown:([IIIII)V")
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[III)Z
    pub fn tryMergeRuns__dualpi_arr_i_i_i(sorter: Object, a: Vec<i32>, low: i32, size: i32) -> Result<bool> {
        panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[III)Z")
    }

    // java: mergeRuns([I[IIIZ[III)[I
    pub fn mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(a: Vec<i32>, b: Vec<i32>, offset: i32, aim: i32, parallel: bool, run: Vec<i32>, lo: i32, hi: i32) -> Result<Vec<i32>> {
        panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([I[IIIZ[III)[I")
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V
    pub fn mergeParts__dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(merger: Object, dst: Vec<i32>, k: i32, a1: Vec<i32>, lo1: i32, hi1: i32, a2: Vec<i32>, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V")
    }

    // java: sort([JIII)V
    pub fn sort__arr_l_i_i_i(a: Vec<i64>, parallelism: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([JIII)V")
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V
    pub fn sort__dualpi_arr_l_i_i_i(sorter: Object, a: Vec<i64>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V")
    }

    // java: mixedInsertionSort([JIII)V
    pub fn mixedInsertionSort__arr_l_i_i_i(a: Vec<i64>, low: i32, end: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([JIII)V")
    }

    // java: insertionSort([JII)V
    pub fn insertionSort__arr_l_i_i(a: Vec<i64>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([JII)V")
    }

    // java: heapSort([JII)V
    pub fn heapSort__arr_l_i_i(a: Vec<i64>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.heapSort:([JII)V")
    }

    // java: pushDown([JIJII)V
    pub fn pushDown__arr_l_i_l_i_i(a: Vec<i64>, p: i32, value: i64, arg3: i32, low: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.pushDown:([JIJII)V")
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z
    pub fn tryMergeRuns__dualpi_arr_l_i_i(sorter: Object, a: Vec<i64>, low: i32, size: i32) -> Result<bool> {
        panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z")
    }

    // java: mergeRuns([J[JIIZ[III)[J
    pub fn mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(a: Vec<i64>, b: Vec<i64>, offset: i32, aim: i32, parallel: bool, run: Vec<i32>, lo: i32, hi: i32) -> Result<Vec<i64>> {
        panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([J[JIIZ[III)[J")
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V
    pub fn mergeParts__dualpi_arr_l_i_arr_l_i_i_arr_l_i_i(merger: Object, dst: Vec<i64>, k: i32, a1: Vec<i64>, lo1: i32, hi1: i32, a2: Vec<i64>, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V")
    }

    // java: sort([BII)V
    pub fn sort__arr_b_i_i(a: Vec<i8>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([BII)V")
    }

    // java: insertionSort([BII)V
    pub fn insertionSort__arr_b_i_i(a: Vec<i8>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([BII)V")
    }

    // java: countingSort([BII)V
    pub fn countingSort__arr_b_i_i(a: Vec<i8>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.countingSort:([BII)V")
    }

    // java: sort([CII)V
    pub fn sort__arr_c_i_i(a: Vec<u16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([CII)V")
    }

    // java: sort([CIII)V
    pub fn sort__arr_c_i_i_i(a: Vec<u16>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([CIII)V")
    }

    // java: insertionSort([CII)V
    pub fn insertionSort__arr_c_i_i(a: Vec<u16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([CII)V")
    }

    // java: countingSort([CII)V
    pub fn countingSort__arr_c_i_i(a: Vec<u16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.countingSort:([CII)V")
    }

    // java: sort([SII)V
    pub fn sort__arr_s_i_i(a: Vec<i16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([SII)V")
    }

    // java: sort([SIII)V
    pub fn sort__arr_s_i_i_i(a: Vec<i16>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([SIII)V")
    }

    // java: insertionSort([SII)V
    pub fn insertionSort__arr_s_i_i(a: Vec<i16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([SII)V")
    }

    // java: countingSort([SII)V
    pub fn countingSort__arr_s_i_i(a: Vec<i16>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.countingSort:([SII)V")
    }

    // java: sort([FIII)V
    pub fn sort__arr_f_i_i_i(a: Vec<f32>, parallelism: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([FIII)V")
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V
    pub fn sort__dualpi_arr_f_i_i_i(sorter: Object, a: Vec<f32>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V")
    }

    // java: mixedInsertionSort([FIII)V
    pub fn mixedInsertionSort__arr_f_i_i_i(a: Vec<f32>, low: i32, end: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([FIII)V")
    }

    // java: insertionSort([FII)V
    pub fn insertionSort__arr_f_i_i(a: Vec<f32>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([FII)V")
    }

    // java: heapSort([FII)V
    pub fn heapSort__arr_f_i_i(a: Vec<f32>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.heapSort:([FII)V")
    }

    // java: pushDown([FIFII)V
    pub fn pushDown__arr_f_i_f_i_i(a: Vec<f32>, p: i32, value: f32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.pushDown:([FIFII)V")
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z
    pub fn tryMergeRuns__dualpi_arr_f_i_i(sorter: Object, a: Vec<f32>, low: i32, size: i32) -> Result<bool> {
        panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z")
    }

    // java: mergeRuns([F[FIIZ[III)[F
    pub fn mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(a: Vec<f32>, b: Vec<f32>, offset: i32, aim: i32, parallel: bool, run: Vec<i32>, lo: i32, hi: i32) -> Result<Vec<f32>> {
        panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([F[FIIZ[III)[F")
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V
    pub fn mergeParts__dualpi_arr_f_i_arr_f_i_i_arr_f_i_i(merger: Object, dst: Vec<f32>, k: i32, a1: Vec<f32>, lo1: i32, hi1: i32, a2: Vec<f32>, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V")
    }

    // java: sort([DIII)V
    pub fn sort__arr_d_i_i_i(a: Vec<f64>, parallelism: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:([DIII)V")
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V
    pub fn sort__dualpi_arr_d_i_i_i(sorter: Object, a: Vec<f64>, bits: i32, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V")
    }

    // java: mixedInsertionSort([DIII)V
    pub fn mixedInsertionSort__arr_d_i_i_i(a: Vec<f64>, low: i32, end: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([DIII)V")
    }

    // java: insertionSort([DII)V
    pub fn insertionSort__arr_d_i_i(a: Vec<f64>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.insertionSort:([DII)V")
    }

    // java: heapSort([DII)V
    pub fn heapSort__arr_d_i_i(a: Vec<f64>, low: i32, high: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.heapSort:([DII)V")
    }

    // java: pushDown([DIDII)V
    pub fn pushDown__arr_d_i_d_i_i(a: Vec<f64>, p: i32, value: f64, arg3: i32, low: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.pushDown:([DIDII)V")
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z
    pub fn tryMergeRuns__dualpi_arr_d_i_i(sorter: Object, a: Vec<f64>, low: i32, size: i32) -> Result<bool> {
        panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z")
    }

    // java: mergeRuns([D[DIIZ[III)[D
    pub fn mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(a: Vec<f64>, b: Vec<f64>, offset: i32, aim: i32, parallel: bool, run: Vec<i32>, lo: i32, hi: i32) -> Result<Vec<f64>> {
        panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([D[DIIZ[III)[D")
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V
    pub fn mergeParts__dualpi_arr_d_i_arr_d_i_i_arr_d_i_i(merger: Object, dst: Vec<f64>, k: i32, a1: Vec<f64>, lo1: i32, hi1: i32, a2: Vec<f64>, lo2: i32, hi2: i32) -> Result<()> {
        panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V")
    }
}
