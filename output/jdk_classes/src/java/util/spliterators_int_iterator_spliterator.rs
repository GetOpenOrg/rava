#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$IntIteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfInt",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_IntIteratorSpliterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/PrimitiveIterator$OfInt;", access = "private final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
}

impl Spliterators_IntIteratorSpliterator {
    // java: <init>(Ljava/util/PrimitiveIterator$OfInt;JI)V
    pub fn new__primit_l_i(&self, iterator: Object, size: i64, arg2: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.<init>")
    }

    // java: <init>(Ljava/util/PrimitiveIterator$OfInt;I)V
    pub fn new__primit_i(&self, iterator: Object, characteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator$OfInt;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/IntConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$IntIteratorSpliterator.getComparator")
    }
}
