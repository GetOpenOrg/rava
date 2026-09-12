#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$DoubleArraySpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfDouble",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_DoubleArraySpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[D", access = "private final"))]
    pub array: Field<Vec<f64>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private"))]
    pub estimatedSize: Field<i64>,
}

impl Spliterators_DoubleArraySpliterator {
    // java: <init>([DI)V
    pub fn new__arr_d_i(&self, array: Vec<f64>, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.<init>")
    }

    // java: <init>([DIII)V
    pub fn new__arr_d_i_i_i(&self, array: Vec<f64>, origin: i32, fence: i32, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.<init>")
    }

    // java: <init>([DIIIJ)V
    pub fn new__arr_d_i_i_i_l(&self, array: Vec<f64>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator$OfDouble;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/DoubleConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/DoubleConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$DoubleArraySpliterator.getComparator")
    }
}
