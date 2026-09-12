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
        todo!("abstract java/util/Arrays.<init>")
    }

    // java: sort([I)V
    pub fn sort__arr_i(a: Vec<i32>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([III)V
    pub fn sort__arr_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([J)V
    pub fn sort__arr_l(a: Vec<i64>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([JII)V
    pub fn sort__arr_l_i_i(a: Vec<i64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([S)V
    pub fn sort__arr_s(a: Vec<i16>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([SII)V
    pub fn sort__arr_s_i_i(a: Vec<i16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([C)V
    pub fn sort__arr_c(a: Vec<u16>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([CII)V
    pub fn sort__arr_c_i_i(a: Vec<u16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([B)V
    pub fn sort__arr_b(a: Vec<i8>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([BII)V
    pub fn sort__arr_b_i_i(a: Vec<i8>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([F)V
    pub fn sort__arr_f(a: Vec<f32>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([FII)V
    pub fn sort__arr_f_i_i(a: Vec<f32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([D)V
    pub fn sort__arr_d(a: Vec<f64>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: sort([DII)V
    pub fn sort__arr_d_i_i(a: Vec<f64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: parallelSort([B)V
    pub fn parallelSort__arr_b(a: Vec<i8>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([BII)V
    pub fn parallelSort__arr_b_i_i(a: Vec<i8>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([C)V
    pub fn parallelSort__arr_c(a: Vec<u16>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([CII)V
    pub fn parallelSort__arr_c_i_i(a: Vec<u16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([S)V
    pub fn parallelSort__arr_s(a: Vec<i16>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([SII)V
    pub fn parallelSort__arr_s_i_i(a: Vec<i16>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([I)V
    pub fn parallelSort__arr_i(a: Vec<i32>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([III)V
    pub fn parallelSort__arr_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([J)V
    pub fn parallelSort__arr_l(a: Vec<i64>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([JII)V
    pub fn parallelSort__arr_l_i_i(a: Vec<i64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([F)V
    pub fn parallelSort__arr_f(a: Vec<f32>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([FII)V
    pub fn parallelSort__arr_f_i_i(a: Vec<f32>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([D)V
    pub fn parallelSort__arr_d(a: Vec<f64>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([DII)V
    pub fn parallelSort__arr_d_i_i(a: Vec<f64>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: rangeCheck(III)V
    pub fn rangeCheck(arrayLength: i32, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.rangeCheck")
    }

    // java: parallelSort([Ljava/lang/Comparable;)V
    pub fn parallelSort__arr_cmp(a: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([Ljava/lang/Comparable;II)V
    pub fn parallelSort__arr_cmp_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn parallelSort__arr_obj_compar(a: Vec<Object>, cmp: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: parallelSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn parallelSort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, cmp: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSort")
    }

    // java: sort([Ljava/lang/Object;)V
    pub fn sort__arr_obj(a: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: legacyMergeSort([Ljava/lang/Object;)V
    pub fn legacyMergeSort__arr_obj(a: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/Arrays.legacyMergeSort")
    }

    // java: sort([Ljava/lang/Object;II)V
    pub fn sort__arr_obj_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: legacyMergeSort([Ljava/lang/Object;II)V
    pub fn legacyMergeSort__arr_obj_i_i(a: Vec<Object>, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.legacyMergeSort")
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i(src: Vec<Object>, dest: Vec<Object>, low: i32, high: i32, off: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.mergeSort")
    }

    // java: swap([Ljava/lang/Object;II)V
    pub fn swap(x: Vec<Object>, a: i32, b: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.swap")
    }

    // java: sort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn sort__arr_obj_compar(a: Vec<Object>, c: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_compar(a: Vec<Object>, c: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.legacyMergeSort")
    }

    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn sort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.sort")
    }

    // java: legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_i_i_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.legacyMergeSort")
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i_compar(src: Vec<Object>, dest: Vec<Object>, low: i32, high: i32, off: i32, c: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.mergeSort")
    }

    // java: parallelPrefix([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_binary(array: Vec<Object>, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_i_i_binary(array: Vec<Object>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([JLjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_longbi(array: Vec<i64>, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([JIILjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_i_i_longbi(array: Vec<i64>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([DLjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_double(array: Vec<f64>, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([DIILjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_i_i_double(array: Vec<f64>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([ILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_intbin(array: Vec<i32>, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: parallelPrefix([IIILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_i_i_intbin(array: Vec<i32>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelPrefix")
    }

    // java: binarySearch([JJ)I
    pub fn binarySearch__arr_l_l(a: Vec<i64>, key: i64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([JIIJ)I
    pub fn binarySearch__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([JIIJ)I
    pub fn binarySearch0__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([II)I
    pub fn binarySearch__arr_i_i(a: Vec<i32>, key: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([IIII)I
    pub fn binarySearch__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([IIII)I
    pub fn binarySearch0__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([SS)I
    pub fn binarySearch__arr_s_s(a: Vec<i16>, key: i16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([SIIS)I
    pub fn binarySearch__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([SIIS)I
    pub fn binarySearch0__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([CC)I
    pub fn binarySearch__arr_c_c(a: Vec<u16>, key: u16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([CIIC)I
    pub fn binarySearch__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([CIIC)I
    pub fn binarySearch0__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([BB)I
    pub fn binarySearch__arr_b_b(a: Vec<i8>, key: i8) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([BIIB)I
    pub fn binarySearch__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([BIIB)I
    pub fn binarySearch0__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([DD)I
    pub fn binarySearch__arr_d_d(a: Vec<f64>, key: f64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([DIID)I
    pub fn binarySearch__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([DIID)I
    pub fn binarySearch0__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([FF)I
    pub fn binarySearch__arr_f_f(a: Vec<f32>, key: f32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([FIIF)I
    pub fn binarySearch__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([FIIF)I
    pub fn binarySearch0__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn binarySearch__arr_obj_obj(a: Vec<Object>, key: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch0__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_obj_compar(a: Vec<Object>, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_i_i_obj_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch")
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch0__arr_obj_i_i_obj_compar(a: Vec<Object>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.binarySearch0")
    }

    // java: equals([J[J)Z
    pub fn equals__arr_l_arr_l(a: Vec<i64>, a2: Vec<i64>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([JII[JII)Z
    pub fn equals__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([I[I)Z
    pub fn equals__arr_i_arr_i(a: Vec<i32>, a2: Vec<i32>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([III[III)Z
    pub fn equals__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([S[S)Z
    pub fn equals__arr_s_arr_s(a: Vec<i16>, a2: Vec<i16>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([SII[SII)Z
    pub fn equals__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([C[C)Z
    pub fn equals__arr_c_arr_c(a: Vec<u16>, a2: Vec<u16>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([CII[CII)Z
    pub fn equals__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([B[B)Z
    pub fn equals__arr_b_arr_b(a: Vec<i8>, a2: Vec<i8>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([BII[BII)Z
    pub fn equals__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([Z[Z)Z
    pub fn equals__arr_z_arr_z(a: Vec<bool>, a2: Vec<bool>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([ZII[ZII)Z
    pub fn equals__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([D[D)Z
    pub fn equals__arr_d_arr_d(a: Vec<f64>, a2: Vec<f64>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([DII[DII)Z
    pub fn equals__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([F[F)Z
    pub fn equals__arr_f_arr_f(a: Vec<f32>, a2: Vec<f32>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([FII[FII)Z
    pub fn equals__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn equals__arr_obj_arr_obj(a: Vec<Object>, a2: Vec<Object>) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;II)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z
    pub fn equals__arr_obj_arr_obj_compar(a: Vec<Object>, a2: Vec<Object>, cmp: Object) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<bool> {
        todo!("abstract java/util/Arrays.equals")
    }

    // java: fill([JJ)V
    pub fn fill__arr_l_l(a: Vec<i64>, val: i64) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([JIIJ)V
    pub fn fill__arr_l_i_i_l(a: Vec<i64>, fromIndex: i32, toIndex: i32, val: i64) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([II)V
    pub fn fill__arr_i_i(a: Vec<i32>, val: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([IIII)V
    pub fn fill__arr_i_i_i_i(a: Vec<i32>, fromIndex: i32, toIndex: i32, val: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([SS)V
    pub fn fill__arr_s_s(a: Vec<i16>, val: i16) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([SIIS)V
    pub fn fill__arr_s_i_i_s(a: Vec<i16>, fromIndex: i32, toIndex: i32, val: i16) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([CC)V
    pub fn fill__arr_c_c(a: Vec<u16>, val: u16) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([CIIC)V
    pub fn fill__arr_c_i_i_c(a: Vec<u16>, fromIndex: i32, toIndex: i32, val: u16) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([BB)V
    pub fn fill__arr_b_b(a: Vec<i8>, val: i8) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([BIIB)V
    pub fn fill__arr_b_i_i_b(a: Vec<i8>, fromIndex: i32, toIndex: i32, val: i8) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([ZZ)V
    pub fn fill__arr_z_z(a: Vec<bool>, val: bool) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([ZIIZ)V
    pub fn fill__arr_z_i_i_z(a: Vec<bool>, fromIndex: i32, toIndex: i32, val: bool) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([DD)V
    pub fn fill__arr_d_d(a: Vec<f64>, val: f64) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([DIID)V
    pub fn fill__arr_d_i_i_d(a: Vec<f64>, fromIndex: i32, toIndex: i32, val: f64) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([FF)V
    pub fn fill__arr_f_f(a: Vec<f32>, val: f32) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([FIIF)V
    pub fn fill__arr_f_i_i_f(a: Vec<f32>, fromIndex: i32, toIndex: i32, val: f32) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn fill__arr_obj_obj(a: Vec<Object>, val: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: fill([Ljava/lang/Object;IILjava/lang/Object;)V
    pub fn fill__arr_obj_i_i_obj(a: Vec<Object>, fromIndex: i32, toIndex: i32, val: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.fill")
    }

    // java: copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i(original: Vec<Object>, newLength: i32) -> Result<Vec<Object>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i_class(original: Vec<Object>, newLength: i32, newType: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([BI)[B
    pub fn copyOf__arr_b_i(original: Vec<i8>, newLength: i32) -> Result<Vec<i8>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([SI)[S
    pub fn copyOf__arr_s_i(original: Vec<i16>, newLength: i32) -> Result<Vec<i16>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([II)[I
    pub fn copyOf__arr_i_i(original: Vec<i32>, newLength: i32) -> Result<Vec<i32>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([JI)[J
    pub fn copyOf__arr_l_i(original: Vec<i64>, newLength: i32) -> Result<Vec<i64>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([CI)[C
    pub fn copyOf__arr_c_i(original: Vec<u16>, newLength: i32) -> Result<Vec<u16>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([FI)[F
    pub fn copyOf__arr_f_i(original: Vec<f32>, newLength: i32) -> Result<Vec<f32>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([DI)[D
    pub fn copyOf__arr_d_i(original: Vec<f64>, newLength: i32) -> Result<Vec<f64>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOf([ZI)[Z
    pub fn copyOf__arr_z_i(original: Vec<bool>, newLength: i32) -> Result<Vec<bool>> {
        todo!("abstract java/util/Arrays.copyOf")
    }

    // java: copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i(original: Vec<Object>, from: i32, to: i32) -> Result<Vec<Object>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i_class(original: Vec<Object>, from: i32, to: i32, newType: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: checkLength(II)V
    pub fn checkLength(from: i32, to: i32) -> Result<()> {
        todo!("abstract java/util/Arrays.checkLength")
    }

    // java: copyOfRange([BII)[B
    pub fn copyOfRange__arr_b_i_i(original: Vec<i8>, from: i32, to: i32) -> Result<Vec<i8>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeByte([BII)[B
    pub fn copyOfRangeByte(original: Vec<i8>, from: i32, to: i32) -> Result<Vec<i8>> {
        todo!("abstract java/util/Arrays.copyOfRangeByte")
    }

    // java: copyOfRange([SII)[S
    pub fn copyOfRange__arr_s_i_i(original: Vec<i16>, from: i32, to: i32) -> Result<Vec<i16>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeShort([SII)[S
    pub fn copyOfRangeShort(original: Vec<i16>, from: i32, to: i32) -> Result<Vec<i16>> {
        todo!("abstract java/util/Arrays.copyOfRangeShort")
    }

    // java: copyOfRange([III)[I
    pub fn copyOfRange__arr_i_i_i(original: Vec<i32>, from: i32, to: i32) -> Result<Vec<i32>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeInt([III)[I
    pub fn copyOfRangeInt(original: Vec<i32>, from: i32, to: i32) -> Result<Vec<i32>> {
        todo!("abstract java/util/Arrays.copyOfRangeInt")
    }

    // java: copyOfRange([JII)[J
    pub fn copyOfRange__arr_l_i_i(original: Vec<i64>, from: i32, to: i32) -> Result<Vec<i64>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeLong([JII)[J
    pub fn copyOfRangeLong(original: Vec<i64>, from: i32, to: i32) -> Result<Vec<i64>> {
        todo!("abstract java/util/Arrays.copyOfRangeLong")
    }

    // java: copyOfRange([CII)[C
    pub fn copyOfRange__arr_c_i_i(original: Vec<u16>, from: i32, to: i32) -> Result<Vec<u16>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeChar([CII)[C
    pub fn copyOfRangeChar(original: Vec<u16>, from: i32, to: i32) -> Result<Vec<u16>> {
        todo!("abstract java/util/Arrays.copyOfRangeChar")
    }

    // java: copyOfRange([FII)[F
    pub fn copyOfRange__arr_f_i_i(original: Vec<f32>, from: i32, to: i32) -> Result<Vec<f32>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeFloat([FII)[F
    pub fn copyOfRangeFloat(original: Vec<f32>, from: i32, to: i32) -> Result<Vec<f32>> {
        todo!("abstract java/util/Arrays.copyOfRangeFloat")
    }

    // java: copyOfRange([DII)[D
    pub fn copyOfRange__arr_d_i_i(original: Vec<f64>, from: i32, to: i32) -> Result<Vec<f64>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeDouble([DII)[D
    pub fn copyOfRangeDouble(original: Vec<f64>, from: i32, to: i32) -> Result<Vec<f64>> {
        todo!("abstract java/util/Arrays.copyOfRangeDouble")
    }

    // java: copyOfRange([ZII)[Z
    pub fn copyOfRange__arr_z_i_i(original: Vec<bool>, from: i32, to: i32) -> Result<Vec<bool>> {
        todo!("abstract java/util/Arrays.copyOfRange")
    }

    // java: copyOfRangeBoolean([ZII)[Z
    pub fn copyOfRangeBoolean(original: Vec<bool>, from: i32, to: i32) -> Result<Vec<bool>> {
        todo!("abstract java/util/Arrays.copyOfRangeBoolean")
    }

    // java: asList([Ljava/lang/Object;)Ljava/util/List;
    pub fn asList(a: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/Arrays.asList")
    }

    // java: hashCode([J)I
    pub fn hashCode__arr_l(a: Vec<i64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([I)I
    pub fn hashCode__arr_i(a: Vec<i32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([S)I
    pub fn hashCode__arr_s(a: Vec<i16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([C)I
    pub fn hashCode__arr_c(a: Vec<u16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([B)I
    pub fn hashCode__arr_b(a: Vec<i8>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([Z)I
    pub fn hashCode__arr_z(a: Vec<bool>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([F)I
    pub fn hashCode__arr_f(a: Vec<f32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([D)I
    pub fn hashCode__arr_d(a: Vec<f64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: hashCode([Ljava/lang/Object;)I
    pub fn hashCode__arr_obj(a: Vec<Object>) -> Result<i32> {
        todo!("abstract java/util/Arrays.hashCode")
    }

    // java: deepHashCode([Ljava/lang/Object;)I
    pub fn deepHashCode(a: Vec<Object>) -> Result<i32> {
        todo!("abstract java/util/Arrays.deepHashCode")
    }

    // java: primitiveArrayHashCode(Ljava/lang/Object;Ljava/lang/Class;)I
    pub fn primitiveArrayHashCode(a: Object, cl: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.primitiveArrayHashCode")
    }

    // java: deepEquals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn deepEquals(a1: Vec<Object>, a2: Vec<Object>) -> Result<bool> {
        todo!("abstract java/util/Arrays.deepEquals")
    }

    // java: deepEquals0(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn deepEquals0(e1: Object, e2: Object) -> Result<bool> {
        todo!("abstract java/util/Arrays.deepEquals0")
    }

    // java: toString([J)Ljava/lang/String;
    pub fn toString__arr_l(a: Vec<i64>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([I)Ljava/lang/String;
    pub fn toString__arr_i(a: Vec<i32>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([S)Ljava/lang/String;
    pub fn toString__arr_s(a: Vec<i16>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([C)Ljava/lang/String;
    pub fn toString__arr_c(a: Vec<u16>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([B)Ljava/lang/String;
    pub fn toString__arr_b(a: Vec<i8>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([Z)Ljava/lang/String;
    pub fn toString__arr_z(a: Vec<bool>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([F)Ljava/lang/String;
    pub fn toString__arr_f(a: Vec<f32>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([D)Ljava/lang/String;
    pub fn toString__arr_d(a: Vec<f64>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: toString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn toString__arr_obj(a: Vec<Object>) -> Result<String> {
        todo!("abstract java/util/Arrays.toString")
    }

    // java: deepToString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn deepToString__arr_obj(a: Vec<Object>) -> Result<String> {
        todo!("abstract java/util/Arrays.deepToString")
    }

    // java: deepToString([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V
    pub fn deepToString__arr_obj_sb_set(a: Vec<Object>, buf: Object, dejaVu: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.deepToString")
    }

    // java: setAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn setAll__arr_obj_intfun(array: Vec<Object>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.setAll")
    }

    // java: parallelSetAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn parallelSetAll__arr_obj_intfun(array: Vec<Object>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSetAll")
    }

    // java: setAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn setAll__arr_i_intuna(array: Vec<i32>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.setAll")
    }

    // java: parallelSetAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn parallelSetAll__arr_i_intuna(array: Vec<i32>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSetAll")
    }

    // java: setAll([JLjava/util/function/IntToLongFunction;)V
    pub fn setAll__arr_l_inttol(array: Vec<i64>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.setAll")
    }

    // java: parallelSetAll([JLjava/util/function/IntToLongFunction;)V
    pub fn parallelSetAll__arr_l_inttol(array: Vec<i64>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSetAll")
    }

    // java: setAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn setAll__arr_d_inttod(array: Vec<f64>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.setAll")
    }

    // java: parallelSetAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn parallelSetAll__arr_d_inttod(array: Vec<f64>, generator: Object) -> Result<()> {
        todo!("abstract java/util/Arrays.parallelSetAll")
    }

    // java: spliterator([Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj(array: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i_i(array: Vec<Object>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([I)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i(array: Vec<i32>) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([III)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i_i(array: Vec<i32>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([J)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l(array: Vec<i64>) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([JII)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i_i(array: Vec<i64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([D)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d(array: Vec<f64>) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: spliterator([DII)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i_i(array: Vec<f64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.spliterator")
    }

    // java: stream([Ljava/lang/Object;)Ljava/util/stream/Stream;
    pub fn stream__arr_obj(array: Vec<Object>) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;
    pub fn stream__arr_obj_i_i(array: Vec<Object>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([I)Ljava/util/stream/IntStream;
    pub fn stream__arr_i(array: Vec<i32>) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([III)Ljava/util/stream/IntStream;
    pub fn stream__arr_i_i_i(array: Vec<i32>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([J)Ljava/util/stream/LongStream;
    pub fn stream__arr_l(array: Vec<i64>) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([JII)Ljava/util/stream/LongStream;
    pub fn stream__arr_l_i_i(array: Vec<i64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([D)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d(array: Vec<f64>) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: stream([DII)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d_i_i(array: Vec<f64>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
        todo!("abstract java/util/Arrays.stream")
    }

    // java: compare([Z[Z)I
    pub fn compare__arr_z_arr_z(a: Vec<bool>, b: Vec<bool>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([ZII[ZII)I
    pub fn compare__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([B[B)I
    pub fn compare__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([BII[BII)I
    pub fn compare__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compareUnsigned([B[B)I
    pub fn compareUnsigned__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compareUnsigned([BII[BII)I
    pub fn compareUnsigned__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compare([S[S)I
    pub fn compare__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([SII[SII)I
    pub fn compare__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compareUnsigned([S[S)I
    pub fn compareUnsigned__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compareUnsigned([SII[SII)I
    pub fn compareUnsigned__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compare([C[C)I
    pub fn compare__arr_c_arr_c(a: Vec<u16>, b: Vec<u16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([CII[CII)I
    pub fn compare__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([I[I)I
    pub fn compare__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([III[III)I
    pub fn compare__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compareUnsigned([I[I)I
    pub fn compareUnsigned__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compareUnsigned([III[III)I
    pub fn compareUnsigned__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compare([J[J)I
    pub fn compare__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([JII[JII)I
    pub fn compare__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compareUnsigned([J[J)I
    pub fn compareUnsigned__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compareUnsigned([JII[JII)I
    pub fn compareUnsigned__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compareUnsigned")
    }

    // java: compare([F[F)I
    pub fn compare__arr_f_arr_f(a: Vec<f32>, b: Vec<f32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([FII[FII)I
    pub fn compare__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([D[D)I
    pub fn compare__arr_d_arr_d(a: Vec<f64>, b: Vec<f64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([DII[DII)I
    pub fn compare__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I
    pub fn compare__arr_cmp_arr_cmp(a: Vec<Object>, b: Vec<Object>) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I
    pub fn compare__arr_cmp_i_i_arr_cmp_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn compare__arr_obj_arr_obj_compar(a: Vec<Object>, b: Vec<Object>, cmp: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: compare([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn compare__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.compare")
    }

    // java: mismatch([Z[Z)I
    pub fn mismatch__arr_z_arr_z(a: Vec<bool>, b: Vec<bool>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([ZII[ZII)I
    pub fn mismatch__arr_z_i_i_arr_z_i_i(a: Vec<bool>, aFromIndex: i32, aToIndex: i32, b: Vec<bool>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([B[B)I
    pub fn mismatch__arr_b_arr_b(a: Vec<i8>, b: Vec<i8>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([BII[BII)I
    pub fn mismatch__arr_b_i_i_arr_b_i_i(a: Vec<i8>, aFromIndex: i32, aToIndex: i32, b: Vec<i8>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([C[C)I
    pub fn mismatch__arr_c_arr_c(a: Vec<u16>, b: Vec<u16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([CII[CII)I
    pub fn mismatch__arr_c_i_i_arr_c_i_i(a: Vec<u16>, aFromIndex: i32, aToIndex: i32, b: Vec<u16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([S[S)I
    pub fn mismatch__arr_s_arr_s(a: Vec<i16>, b: Vec<i16>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([SII[SII)I
    pub fn mismatch__arr_s_i_i_arr_s_i_i(a: Vec<i16>, aFromIndex: i32, aToIndex: i32, b: Vec<i16>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([I[I)I
    pub fn mismatch__arr_i_arr_i(a: Vec<i32>, b: Vec<i32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([III[III)I
    pub fn mismatch__arr_i_i_i_arr_i_i_i(a: Vec<i32>, aFromIndex: i32, aToIndex: i32, b: Vec<i32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([J[J)I
    pub fn mismatch__arr_l_arr_l(a: Vec<i64>, b: Vec<i64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([JII[JII)I
    pub fn mismatch__arr_l_i_i_arr_l_i_i(a: Vec<i64>, aFromIndex: i32, aToIndex: i32, b: Vec<i64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([F[F)I
    pub fn mismatch__arr_f_arr_f(a: Vec<f32>, b: Vec<f32>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([FII[FII)I
    pub fn mismatch__arr_f_i_i_arr_f_i_i(a: Vec<f32>, aFromIndex: i32, aToIndex: i32, b: Vec<f32>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([D[D)I
    pub fn mismatch__arr_d_arr_d(a: Vec<f64>, b: Vec<f64>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([DII[DII)I
    pub fn mismatch__arr_d_i_i_arr_d_i_i(a: Vec<f64>, aFromIndex: i32, aToIndex: i32, b: Vec<f64>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;)I
    pub fn mismatch__arr_obj_arr_obj(a: Vec<Object>, b: Vec<Object>) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;II)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn mismatch__arr_obj_arr_obj_compar(a: Vec<Object>, b: Vec<Object>, cmp: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i_compar(a: Vec<Object>, aFromIndex: i32, aToIndex: i32, b: Vec<Object>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        todo!("abstract java/util/Arrays.mismatch")
    }
}
