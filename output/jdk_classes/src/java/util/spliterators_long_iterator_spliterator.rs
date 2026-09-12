#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$LongIteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator$OfLong",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_LongIteratorSpliterator {
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/PrimitiveIterator$OfLong;", access = "private final"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
}

impl Spliterators_LongIteratorSpliterator {
    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;JI)V
    pub fn new__primit_l_i(&self, iterator: Object, size: i64, arg2: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.<init>")
    }

    // java: <init>(Ljava/util/PrimitiveIterator$OfLong;I)V
    pub fn new__primit_i(&self, iterator: Object, characteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator$OfLong;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/LongConsumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$LongIteratorSpliterator.getComparator")
    }
}
