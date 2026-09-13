#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Arrays.java",
))]
pub struct Arrays;

impl Arrays {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/Arrays.<init>:()V")
    }

    // java: sort([I)V
    pub fn sort__arr_i(a: Vec<i32>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([I)V")
    }

    // java: sort([III)V
    pub fn sort__arr_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([III)V")
    }

    // java: sort([J)V
    pub fn sort__arr_l(a: Vec<i64>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([J)V")
    }

    // java: sort([JII)V
    pub fn sort__arr_l_i_i(a: Vec<i64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([JII)V")
    }

    // java: sort([S)V
    pub fn sort__arr_s(a: Vec<i16>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([S)V")
    }

    // java: sort([SII)V
    pub fn sort__arr_s_i_i(a: Vec<i16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([SII)V")
    }

    // java: sort([C)V
    pub fn sort__arr_c(a: Vec<u16>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([C)V")
    }

    // java: sort([CII)V
    pub fn sort__arr_c_i_i(a: Vec<u16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([CII)V")
    }

    // java: sort([B)V
    pub fn sort__arr_b(a: Vec<i8>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([B)V")
    }

    // java: sort([BII)V
    pub fn sort__arr_b_i_i(a: Vec<i8>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([BII)V")
    }

    // java: sort([F)V
    pub fn sort__arr_f(a: Vec<f32>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([F)V")
    }

    // java: sort([FII)V
    pub fn sort__arr_f_i_i(a: Vec<f32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([FII)V")
    }

    // java: sort([D)V
    pub fn sort__arr_d(a: Vec<f64>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([D)V")
    }

    // java: sort([DII)V
    pub fn sort__arr_d_i_i(a: Vec<f64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([DII)V")
    }

    // java: parallelSort([B)V
    pub fn parallelSort__arr_b(a: Vec<i8>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([B)V")
    }

    // java: parallelSort([BII)V
    pub fn parallelSort__arr_b_i_i(a: Vec<i8>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([BII)V")
    }

    // java: parallelSort([C)V
    pub fn parallelSort__arr_c(a: Vec<u16>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([C)V")
    }

    // java: parallelSort([CII)V
    pub fn parallelSort__arr_c_i_i(a: Vec<u16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([CII)V")
    }

    // java: parallelSort([S)V
    pub fn parallelSort__arr_s(a: Vec<i16>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([S)V")
    }

    // java: parallelSort([SII)V
    pub fn parallelSort__arr_s_i_i(a: Vec<i16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([SII)V")
    }

    // java: parallelSort([I)V
    pub fn parallelSort__arr_i(a: Vec<i32>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([I)V")
    }

    // java: parallelSort([III)V
    pub fn parallelSort__arr_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([III)V")
    }

    // java: parallelSort([J)V
    pub fn parallelSort__arr_l(a: Vec<i64>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([J)V")
    }

    // java: parallelSort([JII)V
    pub fn parallelSort__arr_l_i_i(a: Vec<i64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([JII)V")
    }

    // java: parallelSort([F)V
    pub fn parallelSort__arr_f(a: Vec<f32>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([F)V")
    }

    // java: parallelSort([FII)V
    pub fn parallelSort__arr_f_i_i(a: Vec<f32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([FII)V")
    }

    // java: parallelSort([D)V
    pub fn parallelSort__arr_d(a: Vec<f64>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([D)V")
    }

    // java: parallelSort([DII)V
    pub fn parallelSort__arr_d_i_i(a: Vec<f64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([DII)V")
    }

    // java: rangeCheck(III)V
    pub fn rangeCheck(arrayLength: i32, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.rangeCheck:(III)V")
    }

    // java: parallelSort([Ljava/lang/Comparable;)V
    pub fn parallelSort__arr_cmp(a: Vec<Object>) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Comparable;)V")
    }

    // java: parallelSort([Ljava/lang/Comparable;II)V
    pub fn parallelSort__arr_cmp_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Comparable;II)V")
    }

    // java: parallelSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn parallelSort__arr_obj_compar(a: Vec<Object>, cmp: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Object;Ljava/util/Comparator;)V")
    }

    // java: parallelSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn parallelSort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, cmp: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Object;IILjava/util/Comparator;)V")
    }

    // java: sort([Ljava/lang/Object;)V
    pub fn sort__arr_obj(a: Vec<Object>) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([Ljava/lang/Object;)V")
    }

    // java: legacyMergeSort([Ljava/lang/Object;)V
    pub fn legacyMergeSort__arr_obj(a: Vec<Object>) -> Result<()> {
        panic!("stub: java/util/Arrays.legacyMergeSort:([Ljava/lang/Object;)V")
    }

    // java: sort([Ljava/lang/Object;II)V
    pub fn sort__arr_obj_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([Ljava/lang/Object;II)V")
    }

    // java: legacyMergeSort([Ljava/lang/Object;II)V
    pub fn legacyMergeSort__arr_obj_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.legacyMergeSort:([Ljava/lang/Object;II)V")
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i(src: Vec<Object>, dest: Vec<Object>, low: i32, high: i32, off: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.mergeSort:([Ljava/lang/Object;[Ljava/lang/Object;III)V")
    }

    // java: swap([Ljava/lang/Object;II)V
    pub fn swap(x: Vec<Object>, a: i32, b: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.swap:([Ljava/lang/Object;II)V")
    }

    // java: sort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn sort__arr_obj_compar(a: Vec<Object>, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([Ljava/lang/Object;Ljava/util/Comparator;)V")
    }

    // java: legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_compar(a: Vec<Object>, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.legacyMergeSort:([Ljava/lang/Object;Ljava/util/Comparator;)V")
    }

    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn sort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.sort:([Ljava/lang/Object;IILjava/util/Comparator;)V")
    }

    // java: legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.legacyMergeSort:([Ljava/lang/Object;IILjava/util/Comparator;)V")
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i_compar(src: Vec<Object>, dest: Vec<Object>, low: i32, high: i32, off: i32, c: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.mergeSort:([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V")
    }

    // java: parallelPrefix([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_binary(array: Vec<Object>, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V")
    }

    // java: parallelPrefix([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_i_i_binary(array: Vec<Object>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V")
    }

    // java: parallelPrefix([JLjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_longbi(array: Vec<i64>, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([JLjava/util/function/LongBinaryOperator;)V")
    }

    // java: parallelPrefix([JIILjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_i_i_longbi(array: Vec<i64>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([JIILjava/util/function/LongBinaryOperator;)V")
    }

    // java: parallelPrefix([DLjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_double(array: Vec<f64>, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([DLjava/util/function/DoubleBinaryOperator;)V")
    }

    // java: parallelPrefix([DIILjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_i_i_double(array: Vec<f64>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([DIILjava/util/function/DoubleBinaryOperator;)V")
    }

    // java: parallelPrefix([ILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_intbin(array: Vec<i32>, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([ILjava/util/function/IntBinaryOperator;)V")
    }

    // java: parallelPrefix([IIILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_i_i_intbin(array: Vec<i32>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelPrefix:([IIILjava/util/function/IntBinaryOperator;)V")
    }

    // java: binarySearch([JJ)I
    pub fn binarySearch__arr_l_l(a: Vec<i64>, key: i64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([JJ)I")
    }

    // java: binarySearch([JIIJ)I
    pub fn binarySearch__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([JIIJ)I")
    }

    // java: binarySearch0([JIIJ)I
    pub fn binarySearch0__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([JIIJ)I")
    }

    // java: binarySearch([II)I
    pub fn binarySearch__arr_i_i(a: Vec<i32>, key: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([II)I")
    }

    // java: binarySearch([IIII)I
    pub fn binarySearch__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([IIII)I")
    }

    // java: binarySearch0([IIII)I
    pub fn binarySearch0__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([IIII)I")
    }

    // java: binarySearch([SS)I
    pub fn binarySearch__arr_s_s(a: Vec<i16>, key: i16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([SS)I")
    }

    // java: binarySearch([SIIS)I
    pub fn binarySearch__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([SIIS)I")
    }

    // java: binarySearch0([SIIS)I
    pub fn binarySearch0__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([SIIS)I")
    }

    // java: binarySearch([CC)I
    pub fn binarySearch__arr_c_c(a: Vec<u16>, key: u16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([CC)I")
    }

    // java: binarySearch([CIIC)I
    pub fn binarySearch__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([CIIC)I")
    }

    // java: binarySearch0([CIIC)I
    pub fn binarySearch0__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([CIIC)I")
    }

    // java: binarySearch([BB)I
    pub fn binarySearch__arr_b_b(a: Vec<i8>, key: i8) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([BB)I")
    }

    // java: binarySearch([BIIB)I
    pub fn binarySearch__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([BIIB)I")
    }

    // java: binarySearch0([BIIB)I
    pub fn binarySearch0__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([BIIB)I")
    }

    // java: binarySearch([DD)I
    pub fn binarySearch__arr_d_d(a: Vec<f64>, key: f64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([DD)I")
    }

    // java: binarySearch([DIID)I
    pub fn binarySearch__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([DIID)I")
    }

    // java: binarySearch0([DIID)I
    pub fn binarySearch0__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([DIID)I")
    }

    // java: binarySearch([FF)I
    pub fn binarySearch__arr_f_f(a: Vec<f32>, key: f32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([FF)I")
    }

    // java: binarySearch([FIIF)I
    pub fn binarySearch__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([FIIF)I")
    }

    // java: binarySearch0([FIIF)I
    pub fn binarySearch0__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([FIIF)I")
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn binarySearch__arr_obj_obj(a: Vec<Object>, key: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;Ljava/lang/Object;)I")
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;IILjava/lang/Object;)I")
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch0__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([Ljava/lang/Object;IILjava/lang/Object;)I")
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_obj_compar(a: Vec<Object>, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_i_i_obj_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch0__arr_obj_i_i_obj_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.binarySearch0:([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: equals([J[J)Z
    pub fn equals__arr_l_arr_l(a: Vec<i64>, a2: Vec<i64>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([J[J)Z")
    }

    // java: equals([JII[JII)Z
    pub fn equals__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([JII[JII)Z")
    }

    // java: equals([I[I)Z
    pub fn equals__arr_i_arr_i(a: Vec<i32>, a2: Vec<i32>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([I[I)Z")
    }

    // java: equals([III[III)Z
    pub fn equals__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([III[III)Z")
    }

    // java: equals([S[S)Z
    pub fn equals__arr_s_arr_s(a: Vec<i16>, a2: Vec<i16>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([S[S)Z")
    }

    // java: equals([SII[SII)Z
    pub fn equals__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([SII[SII)Z")
    }

    // java: equals([C[C)Z
    pub fn equals__arr_c_arr_c(a: Vec<u16>, a2: Vec<u16>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([C[C)Z")
    }

    // java: equals([CII[CII)Z
    pub fn equals__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([CII[CII)Z")
    }

    // java: equals([B[B)Z
    pub fn equals__arr_b_arr_b(a: Vec<i8>, a2: Vec<i8>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([B[B)Z")
    }

    // java: equals([BII[BII)Z
    pub fn equals__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([BII[BII)Z")
    }

    // java: equals([Z[Z)Z
    pub fn equals__arr_z_arr_z(a: Vec<bool>, a2: Vec<bool>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([Z[Z)Z")
    }

    // java: equals([ZII[ZII)Z
    pub fn equals__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([ZII[ZII)Z")
    }

    // java: equals([D[D)Z
    pub fn equals__arr_d_arr_d(a: Vec<f64>, a2: Vec<f64>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([D[D)Z")
    }

    // java: equals([DII[DII)Z
    pub fn equals__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([DII[DII)Z")
    }

    // java: equals([F[F)Z
    pub fn equals__arr_f_arr_f(a: Vec<f32>, a2: Vec<f32>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([F[F)Z")
    }

    // java: equals([FII[FII)Z
    pub fn equals__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([FII[FII)Z")
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn equals__arr_obj_arr_obj(a: Vec<Object>, a2: Vec<Object>) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;[Ljava/lang/Object;)Z")
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;II)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;II[Ljava/lang/Object;II)Z")
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z
    pub fn equals__arr_obj_arr_obj_compar(a: Vec<Object>, a2: Vec<Object>, cmp: Object) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z")
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<bool> {
        panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z")
    }

    // java: fill([JJ)V
    pub fn fill__arr_l_l(a: Vec<i64>, val: i64) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([JJ)V")
    }

    // java: fill([JIIJ)V
    pub fn fill__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, val: i64) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([JIIJ)V")
    }

    // java: fill([II)V
    pub fn fill__arr_i_i(a: Vec<i32>, val: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([II)V")
    }

    // java: fill([IIII)V
    pub fn fill__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, val: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([IIII)V")
    }

    // java: fill([SS)V
    pub fn fill__arr_s_s(a: Vec<i16>, val: i16) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([SS)V")
    }

    // java: fill([SIIS)V
    pub fn fill__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, val: i16) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([SIIS)V")
    }

    // java: fill([CC)V
    pub fn fill__arr_c_c(a: Vec<u16>, val: u16) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([CC)V")
    }

    // java: fill([CIIC)V
    pub fn fill__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, val: u16) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([CIIC)V")
    }

    // java: fill([BB)V
    pub fn fill__arr_b_b(a: Vec<i8>, val: i8) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([BB)V")
    }

    // java: fill([BIIB)V
    pub fn fill__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, val: i8) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([BIIB)V")
    }

    // java: fill([ZZ)V
    pub fn fill__arr_z_z(a: Vec<bool>, val: bool) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([ZZ)V")
    }

    // java: fill([ZIIZ)V
    pub fn fill__arr_z_i_i_z(a: Vec<bool>, fromIndex: i32, toIndex: i32, val: bool) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([ZIIZ)V")
    }

    // java: fill([DD)V
    pub fn fill__arr_d_d(a: Vec<f64>, val: f64) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([DD)V")
    }

    // java: fill([DIID)V
    pub fn fill__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, val: f64) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([DIID)V")
    }

    // java: fill([FF)V
    pub fn fill__arr_f_f(a: Vec<f32>, val: f32) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([FF)V")
    }

    // java: fill([FIIF)V
    pub fn fill__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, val: f32) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([FIIF)V")
    }

    // java: fill([Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn fill__arr_obj_obj(a: Vec<Object>, val: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    // java: fill([Ljava/lang/Object;IILjava/lang/Object;)V
    pub fn fill__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, val: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.fill:([Ljava/lang/Object;IILjava/lang/Object;)V")
    }

    // java: copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i(original: Vec<Object>, newLength: i32) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays.copyOf:([Ljava/lang/Object;I)[Ljava/lang/Object;")
    }

    // java: copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i_class(original: Vec<Object>, newLength: i32, newType: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays.copyOf:([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;")
    }

    // java: copyOf([BI)[B
    pub fn copyOf__arr_b_i(original: Vec<i8>, newLength: i32) -> Result<Vec<i8>> {
        panic!("stub: java/util/Arrays.copyOf:([BI)[B")
    }

    // java: copyOf([SI)[S
    pub fn copyOf__arr_s_i(original: Vec<i16>, newLength: i32) -> Result<Vec<i16>> {
        panic!("stub: java/util/Arrays.copyOf:([SI)[S")
    }

    // java: copyOf([II)[I
    pub fn copyOf__arr_i_i(original: Vec<i32>, newLength: i32) -> Result<Vec<i32>> {
        panic!("stub: java/util/Arrays.copyOf:([II)[I")
    }

    // java: copyOf([JI)[J
    pub fn copyOf__arr_l_i(original: Vec<i64>, newLength: i32) -> Result<Vec<i64>> {
        panic!("stub: java/util/Arrays.copyOf:([JI)[J")
    }

    // java: copyOf([CI)[C
    pub fn copyOf__arr_c_i(original: Vec<u16>, newLength: i32) -> Result<Vec<u16>> {
        panic!("stub: java/util/Arrays.copyOf:([CI)[C")
    }

    // java: copyOf([FI)[F
    pub fn copyOf__arr_f_i(original: Vec<f32>, newLength: i32) -> Result<Vec<f32>> {
        panic!("stub: java/util/Arrays.copyOf:([FI)[F")
    }

    // java: copyOf([DI)[D
    pub fn copyOf__arr_d_i(original: Vec<f64>, newLength: i32) -> Result<Vec<f64>> {
        panic!("stub: java/util/Arrays.copyOf:([DI)[D")
    }

    // java: copyOf([ZI)[Z
    pub fn copyOf__arr_z_i(original: Vec<bool>, newLength: i32) -> Result<Vec<bool>> {
        panic!("stub: java/util/Arrays.copyOf:([ZI)[Z")
    }

    // java: copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i(original: Vec<Object>, from: i32, to: i32) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays.copyOfRange:([Ljava/lang/Object;II)[Ljava/lang/Object;")
    }

    // java: copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i_class(original: Vec<Object>, from: i32, to: i32, newType: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Arrays.copyOfRange:([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;")
    }

    // java: checkLength(II)V
    pub fn checkLength(from: i32, to: i32) -> Result<()> {
        panic!("stub: java/util/Arrays.checkLength:(II)V")
    }

    // java: copyOfRange([BII)[B
    pub fn copyOfRange__arr_b_i_i(original: Vec<i8>, from: i32, to: i32) -> Result<Vec<i8>> {
        panic!("stub: java/util/Arrays.copyOfRange:([BII)[B")
    }

    // java: copyOfRangeByte([BII)[B
    pub fn copyOfRangeByte(original: Vec<i8>, from: i32, to: i32) -> Result<Vec<i8>> {
        panic!("stub: java/util/Arrays.copyOfRangeByte:([BII)[B")
    }

    // java: copyOfRange([SII)[S
    pub fn copyOfRange__arr_s_i_i(original: Vec<i16>, from: i32, to: i32) -> Result<Vec<i16>> {
        panic!("stub: java/util/Arrays.copyOfRange:([SII)[S")
    }

    // java: copyOfRangeShort([SII)[S
    pub fn copyOfRangeShort(original: Vec<i16>, from: i32, to: i32) -> Result<Vec<i16>> {
        panic!("stub: java/util/Arrays.copyOfRangeShort:([SII)[S")
    }

    // java: copyOfRange([III)[I
    pub fn copyOfRange__arr_i_i_i(original: Vec<i32>, from: i32, to: i32) -> Result<Vec<i32>> {
        panic!("stub: java/util/Arrays.copyOfRange:([III)[I")
    }

    // java: copyOfRangeInt([III)[I
    pub fn copyOfRangeInt(original: Vec<i32>, from: i32, to: i32) -> Result<Vec<i32>> {
        panic!("stub: java/util/Arrays.copyOfRangeInt:([III)[I")
    }

    // java: copyOfRange([JII)[J
    pub fn copyOfRange__arr_l_i_i(original: Vec<i64>, from: i32, to: i32) -> Result<Vec<i64>> {
        panic!("stub: java/util/Arrays.copyOfRange:([JII)[J")
    }

    // java: copyOfRangeLong([JII)[J
    pub fn copyOfRangeLong(original: Vec<i64>, from: i32, to: i32) -> Result<Vec<i64>> {
        panic!("stub: java/util/Arrays.copyOfRangeLong:([JII)[J")
    }

    // java: copyOfRange([CII)[C
    pub fn copyOfRange__arr_c_i_i(original: Vec<u16>, from: i32, to: i32) -> Result<Vec<u16>> {
        panic!("stub: java/util/Arrays.copyOfRange:([CII)[C")
    }

    // java: copyOfRangeChar([CII)[C
    pub fn copyOfRangeChar(original: Vec<u16>, from: i32, to: i32) -> Result<Vec<u16>> {
        panic!("stub: java/util/Arrays.copyOfRangeChar:([CII)[C")
    }

    // java: copyOfRange([FII)[F
    pub fn copyOfRange__arr_f_i_i(original: Vec<f32>, from: i32, to: i32) -> Result<Vec<f32>> {
        panic!("stub: java/util/Arrays.copyOfRange:([FII)[F")
    }

    // java: copyOfRangeFloat([FII)[F
    pub fn copyOfRangeFloat(original: Vec<f32>, from: i32, to: i32) -> Result<Vec<f32>> {
        panic!("stub: java/util/Arrays.copyOfRangeFloat:([FII)[F")
    }

    // java: copyOfRange([DII)[D
    pub fn copyOfRange__arr_d_i_i(original: Vec<f64>, from: i32, to: i32) -> Result<Vec<f64>> {
        panic!("stub: java/util/Arrays.copyOfRange:([DII)[D")
    }

    // java: copyOfRangeDouble([DII)[D
    pub fn copyOfRangeDouble(original: Vec<f64>, from: i32, to: i32) -> Result<Vec<f64>> {
        panic!("stub: java/util/Arrays.copyOfRangeDouble:([DII)[D")
    }

    // java: copyOfRange([ZII)[Z
    pub fn copyOfRange__arr_z_i_i(original: Vec<bool>, from: i32, to: i32) -> Result<Vec<bool>> {
        panic!("stub: java/util/Arrays.copyOfRange:([ZII)[Z")
    }

    // java: copyOfRangeBoolean([ZII)[Z
    pub fn copyOfRangeBoolean(original: Vec<bool>, from: i32, to: i32) -> Result<Vec<bool>> {
        panic!("stub: java/util/Arrays.copyOfRangeBoolean:([ZII)[Z")
    }

    // java: asList([Ljava/lang/Object;)Ljava/util/List;
    pub fn asList(a: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/Arrays.asList:([Ljava/lang/Object;)Ljava/util/List;")
    }

    // java: hashCode([J)I
    pub fn hashCode__arr_l(a: Vec<i64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([J)I")
    }

    // java: hashCode([I)I
    pub fn hashCode__arr_i(a: Vec<i32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([I)I")
    }

    // java: hashCode([S)I
    pub fn hashCode__arr_s(a: Vec<i16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([S)I")
    }

    // java: hashCode([C)I
    pub fn hashCode__arr_c(a: Vec<u16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([C)I")
    }

    // java: hashCode([B)I
    pub fn hashCode__arr_b(a: Vec<i8>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([B)I")
    }

    // java: hashCode([Z)I
    pub fn hashCode__arr_z(a: Vec<bool>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([Z)I")
    }

    // java: hashCode([F)I
    pub fn hashCode__arr_f(a: Vec<f32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([F)I")
    }

    // java: hashCode([D)I
    pub fn hashCode__arr_d(a: Vec<f64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([D)I")
    }

    // java: hashCode([Ljava/lang/Object;)I
    pub fn hashCode__arr_obj(a: Vec<Object>) -> Result<i32> {
        panic!("stub: java/util/Arrays.hashCode:([Ljava/lang/Object;)I")
    }

    // java: deepHashCode([Ljava/lang/Object;)I
    pub fn deepHashCode(a: Vec<Object>) -> Result<i32> {
        panic!("stub: java/util/Arrays.deepHashCode:([Ljava/lang/Object;)I")
    }

    // java: primitiveArrayHashCode(Ljava/lang/Object;Ljava/lang/Class;)I
    pub fn primitiveArrayHashCode(a: Object, cl: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.primitiveArrayHashCode:(Ljava/lang/Object;Ljava/lang/Class;)I")
    }

    // java: deepEquals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn deepEquals(a1: Vec<Object>, a2: Vec<Object>) -> Result<bool> {
        panic!("stub: java/util/Arrays.deepEquals:([Ljava/lang/Object;[Ljava/lang/Object;)Z")
    }

    // java: deepEquals0(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn deepEquals0(e1: Object, e2: Object) -> Result<bool> {
        panic!("stub: java/util/Arrays.deepEquals0:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: toString([J)Ljava/lang/String;
    pub fn toString__arr_l(a: Vec<i64>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([J)Ljava/lang/String;")
    }

    // java: toString([I)Ljava/lang/String;
    pub fn toString__arr_i(a: Vec<i32>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([I)Ljava/lang/String;")
    }

    // java: toString([S)Ljava/lang/String;
    pub fn toString__arr_s(a: Vec<i16>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([S)Ljava/lang/String;")
    }

    // java: toString([C)Ljava/lang/String;
    pub fn toString__arr_c(a: Vec<u16>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([C)Ljava/lang/String;")
    }

    // java: toString([B)Ljava/lang/String;
    pub fn toString__arr_b(a: Vec<i8>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([B)Ljava/lang/String;")
    }

    // java: toString([Z)Ljava/lang/String;
    pub fn toString__arr_z(a: Vec<bool>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([Z)Ljava/lang/String;")
    }

    // java: toString([F)Ljava/lang/String;
    pub fn toString__arr_f(a: Vec<f32>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([F)Ljava/lang/String;")
    }

    // java: toString([D)Ljava/lang/String;
    pub fn toString__arr_d(a: Vec<f64>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([D)Ljava/lang/String;")
    }

    // java: toString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn toString__arr_obj(a: Vec<Object>) -> Result<String> {
        panic!("stub: java/util/Arrays.toString:([Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: deepToString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn deepToString__arr_obj(a: Vec<Object>) -> Result<String> {
        panic!("stub: java/util/Arrays.deepToString:([Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: deepToString([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V
    pub fn deepToString__arr_obj_sb_set(a: Vec<Object>, buf: Object, dejaVu: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.deepToString:([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V")
    }

    // java: setAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn setAll__arr_obj_intfun(array: Vec<Object>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.setAll:([Ljava/lang/Object;Ljava/util/function/IntFunction;)V")
    }

    // java: parallelSetAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn parallelSetAll__arr_obj_intfun(array: Vec<Object>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSetAll:([Ljava/lang/Object;Ljava/util/function/IntFunction;)V")
    }

    // java: setAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn setAll__arr_i_intuna(array: Vec<i32>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.setAll:([ILjava/util/function/IntUnaryOperator;)V")
    }

    // java: parallelSetAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn parallelSetAll__arr_i_intuna(array: Vec<i32>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSetAll:([ILjava/util/function/IntUnaryOperator;)V")
    }

    // java: setAll([JLjava/util/function/IntToLongFunction;)V
    pub fn setAll__arr_l_inttol(array: Vec<i64>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.setAll:([JLjava/util/function/IntToLongFunction;)V")
    }

    // java: parallelSetAll([JLjava/util/function/IntToLongFunction;)V
    pub fn parallelSetAll__arr_l_inttol(array: Vec<i64>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSetAll:([JLjava/util/function/IntToLongFunction;)V")
    }

    // java: setAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn setAll__arr_d_inttod(array: Vec<f64>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.setAll:([DLjava/util/function/IntToDoubleFunction;)V")
    }

    // java: parallelSetAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn parallelSetAll__arr_d_inttod(array: Vec<f64>, generator: Object) -> Result<()> {
        panic!("stub: java/util/Arrays.parallelSetAll:([DLjava/util/function/IntToDoubleFunction;)V")
    }

    // java: spliterator([Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj(array: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([Ljava/lang/Object;)Ljava/util/Spliterator;")
    }

    // java: spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i_i(array: Vec<Object>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([Ljava/lang/Object;II)Ljava/util/Spliterator;")
    }

    // java: spliterator([I)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i(array: Vec<i32>) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([I)Ljava/util/Spliterator$OfInt;")
    }

    // java: spliterator([III)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i_i(array: Vec<i32>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([III)Ljava/util/Spliterator$OfInt;")
    }

    // java: spliterator([J)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l(array: Vec<i64>) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([J)Ljava/util/Spliterator$OfLong;")
    }

    // java: spliterator([JII)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i_i(array: Vec<i64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([JII)Ljava/util/Spliterator$OfLong;")
    }

    // java: spliterator([D)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d(array: Vec<f64>) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([D)Ljava/util/Spliterator$OfDouble;")
    }

    // java: spliterator([DII)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i_i(array: Vec<f64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.spliterator:([DII)Ljava/util/Spliterator$OfDouble;")
    }

    // java: stream([Ljava/lang/Object;)Ljava/util/stream/Stream;
    pub fn stream__arr_obj(array: Vec<Object>) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([Ljava/lang/Object;)Ljava/util/stream/Stream;")
    }

    // java: stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;
    pub fn stream__arr_obj_i_i(array: Vec<Object>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([Ljava/lang/Object;II)Ljava/util/stream/Stream;")
    }

    // java: stream([I)Ljava/util/stream/IntStream;
    pub fn stream__arr_i(array: Vec<i32>) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([I)Ljava/util/stream/IntStream;")
    }

    // java: stream([III)Ljava/util/stream/IntStream;
    pub fn stream__arr_i_i_i(array: Vec<i32>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([III)Ljava/util/stream/IntStream;")
    }

    // java: stream([J)Ljava/util/stream/LongStream;
    pub fn stream__arr_l(array: Vec<i64>) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([J)Ljava/util/stream/LongStream;")
    }

    // java: stream([JII)Ljava/util/stream/LongStream;
    pub fn stream__arr_l_i_i(array: Vec<i64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([JII)Ljava/util/stream/LongStream;")
    }

    // java: stream([D)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d(array: Vec<f64>) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([D)Ljava/util/stream/DoubleStream;")
    }

    // java: stream([DII)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d_i_i(array: Vec<f64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        panic!("stub: java/util/Arrays.stream:([DII)Ljava/util/stream/DoubleStream;")
    }

    // java: compare([Z[Z)I
    pub fn compare__arr_z_arr_z(a: Vec<bool>, b: Vec<bool>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([Z[Z)I")
    }

    // java: compare([ZII[ZII)I
    pub fn compare__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([ZII[ZII)I")
    }

    // java: compare([B[B)I
    pub fn compare__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([B[B)I")
    }

    // java: compare([BII[BII)I
    pub fn compare__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([BII[BII)I")
    }

    // java: compareUnsigned([B[B)I
    pub fn compareUnsigned__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([B[B)I")
    }

    // java: compareUnsigned([BII[BII)I
    pub fn compareUnsigned__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([BII[BII)I")
    }

    // java: compare([S[S)I
    pub fn compare__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([S[S)I")
    }

    // java: compare([SII[SII)I
    pub fn compare__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([SII[SII)I")
    }

    // java: compareUnsigned([S[S)I
    pub fn compareUnsigned__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([S[S)I")
    }

    // java: compareUnsigned([SII[SII)I
    pub fn compareUnsigned__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([SII[SII)I")
    }

    // java: compare([C[C)I
    pub fn compare__arr_c_arr_c(a: Vec<u16>, b: Vec<u16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([C[C)I")
    }

    // java: compare([CII[CII)I
    pub fn compare__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([CII[CII)I")
    }

    // java: compare([I[I)I
    pub fn compare__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([I[I)I")
    }

    // java: compare([III[III)I
    pub fn compare__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([III[III)I")
    }

    // java: compareUnsigned([I[I)I
    pub fn compareUnsigned__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([I[I)I")
    }

    // java: compareUnsigned([III[III)I
    pub fn compareUnsigned__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([III[III)I")
    }

    // java: compare([J[J)I
    pub fn compare__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([J[J)I")
    }

    // java: compare([JII[JII)I
    pub fn compare__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([JII[JII)I")
    }

    // java: compareUnsigned([J[J)I
    pub fn compareUnsigned__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([J[J)I")
    }

    // java: compareUnsigned([JII[JII)I
    pub fn compareUnsigned__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compareUnsigned:([JII[JII)I")
    }

    // java: compare([F[F)I
    pub fn compare__arr_f_arr_f(a: Vec<f32>, b: Vec<f32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([F[F)I")
    }

    // java: compare([FII[FII)I
    pub fn compare__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([FII[FII)I")
    }

    // java: compare([D[D)I
    pub fn compare__arr_d_arr_d(a: Vec<f64>, b: Vec<f64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([D[D)I")
    }

    // java: compare([DII[DII)I
    pub fn compare__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([DII[DII)I")
    }

    // java: compare([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I
    pub fn compare__arr_cmp_arr_cmp(a: Vec<Object>, b: Vec<Object>) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I")
    }

    // java: compare([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I
    pub fn compare__arr_cmp_i_i_arr_cmp_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I")
    }

    // java: compare([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn compare__arr_obj_arr_obj_compar(a: Vec<Object>, b: Vec<Object>, cmp: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: compare([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn compare__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.compare:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I")
    }

    // java: mismatch([Z[Z)I
    pub fn mismatch__arr_z_arr_z(a: Vec<bool>, b: Vec<bool>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([Z[Z)I")
    }

    // java: mismatch([ZII[ZII)I
    pub fn mismatch__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([ZII[ZII)I")
    }

    // java: mismatch([B[B)I
    pub fn mismatch__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([B[B)I")
    }

    // java: mismatch([BII[BII)I
    pub fn mismatch__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([BII[BII)I")
    }

    // java: mismatch([C[C)I
    pub fn mismatch__arr_c_arr_c(a: Vec<u16>, b: Vec<u16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([C[C)I")
    }

    // java: mismatch([CII[CII)I
    pub fn mismatch__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([CII[CII)I")
    }

    // java: mismatch([S[S)I
    pub fn mismatch__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([S[S)I")
    }

    // java: mismatch([SII[SII)I
    pub fn mismatch__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([SII[SII)I")
    }

    // java: mismatch([I[I)I
    pub fn mismatch__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([I[I)I")
    }

    // java: mismatch([III[III)I
    pub fn mismatch__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([III[III)I")
    }

    // java: mismatch([J[J)I
    pub fn mismatch__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([J[J)I")
    }

    // java: mismatch([JII[JII)I
    pub fn mismatch__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([JII[JII)I")
    }

    // java: mismatch([F[F)I
    pub fn mismatch__arr_f_arr_f(a: Vec<f32>, b: Vec<f32>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([F[F)I")
    }

    // java: mismatch([FII[FII)I
    pub fn mismatch__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([FII[FII)I")
    }

    // java: mismatch([D[D)I
    pub fn mismatch__arr_d_arr_d(a: Vec<f64>, b: Vec<f64>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([D[D)I")
    }

    // java: mismatch([DII[DII)I
    pub fn mismatch__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([DII[DII)I")
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;)I
    pub fn mismatch__arr_obj_arr_obj(a: Vec<Object>, b: Vec<Object>) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;[Ljava/lang/Object;)I")
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;II)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;II[Ljava/lang/Object;II)I")
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn mismatch__arr_obj_arr_obj_compar(a: Vec<Object>, b: Vec<Object>, cmp: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I")
    }
}
