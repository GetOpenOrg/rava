#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$LongArraySpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfLong",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_LongArraySpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[J", access = "private final"))]
    pub array: Field<Vec<i64>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private"))]
    pub estimatedSize: Field<i64>,
}

impl Spliterators_LongArraySpliterator {
    // java: <init>([JI)V
    pub fn new__arr_l_i(&self, array: Vec<i64>, additionalCharacteristics: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.<init>:([JI)V")
    }

    // java: <init>([JIII)V
    pub fn new__arr_l_i_i_i(&self, array: Vec<i64>, origin: i32, fence: i32, additionalCharacteristics: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.<init>:([JIII)V")
    }

    // java: <init>([JIIIJ)V
    pub fn new__arr_l_i_i_i_l(&self, array: Vec<i64>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<()> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.<init>:([JIIIJ)V")
    }

    // java: trySplit()Ljava/util/Spliterator$OfLong;
    pub fn trySplit(&self) -> Result<Object> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.trySplit:()Ljava/util/Spliterator$OfLong;")
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.forEachRemaining:(Ljava/util/function/LongConsumer;)V")
    }

    // java: tryAdvance(Ljava/util/function/LongConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.tryAdvance:(Ljava/util/function/LongConsumer;)Z")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.estimateSize:()J")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.characteristics:()I")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        panic!("stub: java/util/Spliterators$LongArraySpliterator.getComparator:()Ljava/util/Comparator;")
    }
}
