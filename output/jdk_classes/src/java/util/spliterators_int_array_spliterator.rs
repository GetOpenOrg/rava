#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$IntArraySpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfInt",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_IntArraySpliterator {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[I", access = "private final"))]
    pub array: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private"))]
    pub estimatedSize: Field<i64>,
}

impl Spliterators_IntArraySpliterator {
    // java: <init>([II)V
    pub fn new__arr_i_i(&self, array: Vec<i32>, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.<init>")
    }

    // java: <init>([IIII)V
    pub fn new__arr_i_i_i_i(&self, array: Vec<i32>, origin: i32, fence: i32, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.<init>")
    }

    // java: <init>([IIIIJ)V
    pub fn new__arr_i_i_i_i_l(&self, array: Vec<i32>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator$OfInt;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/IntConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$IntArraySpliterator.getComparator")
    }
}
