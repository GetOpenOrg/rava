#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/TimSort",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "TimSort.java",
))]
pub struct TimSort<T> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Comparator;", access = "private final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "minGallop", descriptor = "I", access = "private"))]
    pub minGallop: Field<i32>,
    #[cfg_attr(any(), java_field(name = "tmp", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub tmp: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "tmpBase", descriptor = "I", access = "private"))]
    pub tmpBase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "tmpLen", descriptor = "I", access = "private"))]
    pub tmpLen: Field<i32>,
    #[cfg_attr(any(), java_field(name = "stackSize", descriptor = "I", access = "private"))]
    pub stackSize: Field<i32>,
    #[cfg_attr(any(), java_field(name = "runBase", descriptor = "[I", access = "private final"))]
    pub runBase: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "runLen", descriptor = "[I", access = "private final"))]
    pub runLen: Field<Vec<i32>>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> TimSort<T> {
    // java: <init>([Ljava/lang/Object;Ljava/util/Comparator;[Ljava/lang/Object;II)V
    pub fn new(&self, a: Vec<Object>, c: Object, work: Vec<Object>, workBase: i32, workLen: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.<init>:([Ljava/lang/Object;Ljava/util/Comparator;[Ljava/lang/Object;II)V")
    }

    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;[Ljava/lang/Object;II)V
    pub fn sort(a: Vec<Object>, lo: i32, hi: i32, c: Object, work: Vec<Object>, workBase: i32, workLen: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.sort:([Ljava/lang/Object;IILjava/util/Comparator;[Ljava/lang/Object;II)V")
    }

    // java: binarySort([Ljava/lang/Object;IIILjava/util/Comparator;)V
    pub fn binarySort(a: Vec<Object>, lo: i32, hi: i32, start: i32, c: Object) -> Result<()> {
        panic!("stub: java/util/TimSort.binarySort:([Ljava/lang/Object;IIILjava/util/Comparator;)V")
    }

    // java: countRunAndMakeAscending([Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn countRunAndMakeAscending(a: Vec<Object>, lo: i32, hi: i32, c: Object) -> Result<i32> {
        panic!("stub: java/util/TimSort.countRunAndMakeAscending:([Ljava/lang/Object;IILjava/util/Comparator;)I")
    }

    // java: reverseRange([Ljava/lang/Object;II)V
    pub fn reverseRange(a: Vec<Object>, lo: i32, hi: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.reverseRange:([Ljava/lang/Object;II)V")
    }

    // java: minRunLength(I)I
    pub fn minRunLength(n: i32) -> Result<i32> {
        panic!("stub: java/util/TimSort.minRunLength:(I)I")
    }

    // java: pushRun(II)V
    pub fn pushRun(&self, runBase: i32, runLen: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.pushRun:(II)V")
    }

    // java: mergeCollapse()V
    pub fn mergeCollapse(&self) -> Result<()> {
        panic!("stub: java/util/TimSort.mergeCollapse:()V")
    }

    // java: mergeForceCollapse()V
    pub fn mergeForceCollapse(&self) -> Result<()> {
        panic!("stub: java/util/TimSort.mergeForceCollapse:()V")
    }

    // java: mergeAt(I)V
    pub fn mergeAt(&self, i: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.mergeAt:(I)V")
    }

    // java: gallopLeft(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I
    pub fn gallopLeft(key: Object, a: Vec<Object>, base: i32, len: i32, hint: i32, c: Object) -> Result<i32> {
        panic!("stub: java/util/TimSort.gallopLeft:(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I")
    }

    // java: gallopRight(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I
    pub fn gallopRight(key: Object, a: Vec<Object>, base: i32, len: i32, hint: i32, c: Object) -> Result<i32> {
        panic!("stub: java/util/TimSort.gallopRight:(Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)I")
    }

    // java: mergeLo(IIII)V
    pub fn mergeLo(&self, base1: i32, len1: i32, base2: i32, len2: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.mergeLo:(IIII)V")
    }

    // java: mergeHi(IIII)V
    pub fn mergeHi(&self, base1: i32, len1: i32, base2: i32, len2: i32) -> Result<()> {
        panic!("stub: java/util/TimSort.mergeHi:(IIII)V")
    }

    // java: ensureCapacity(I)[Ljava/lang/Object;
    pub fn ensureCapacity(&self, minCapacity: i32) -> Result<Vec<Object>> {
        panic!("stub: java/util/TimSort.ensureCapacity:(I)[Ljava/lang/Object;")
    }
}
