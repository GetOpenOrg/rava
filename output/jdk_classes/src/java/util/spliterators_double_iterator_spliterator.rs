#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$DoubleIteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfDouble",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_DoubleIteratorSpliterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/PrimitiveIterator$OfDouble;", access = "private final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
}

impl Spliterators_DoubleIteratorSpliterator {
    // java: <init>(Ljava/util/PrimitiveIterator$OfDouble;JI)V
    pub fn new__primit_l_i(&self, iterator: Object, size: i64, arg2: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.<init>")
    }

    // java: <init>(Ljava/util/PrimitiveIterator$OfDouble;I)V
    pub fn new__primit_i(&self, iterator: Object, characteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator$OfDouble;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/DoubleConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/DoubleConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$DoubleIteratorSpliterator.getComparator")
    }
}
