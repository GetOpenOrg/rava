#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$IteratorSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_IteratorSpliterator<T> {
    #[cfg_attr(any(), java_field(name = "collection", descriptor = "Ljava/util/Collection;", access = "private final"))]
    pub collection: Field<Object>,
    #[cfg_attr(any(), java_field(name = "it", descriptor = "Ljava/util/Iterator;", access = "private"))]
    pub it: Field<Object>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J", access = "private"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "batch", descriptor = "I", access = "private"))]
    pub batch: Field<i32>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Spliterators_IteratorSpliterator<T> {
    // java: <init>(Ljava/util/Collection;I)V
    pub fn new__coll_i(&self, collection: Object, characteristics: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.<init>:(Ljava/util/Collection;I)V")
    }

    // java: <init>(Ljava/util/Iterator;JI)V
    pub fn new__iterat_l_i(&self, iterator: Object, size: i64, arg2: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.<init>:(Ljava/util/Iterator;JI)V")
    }

    // java: <init>(Ljava/util/Iterator;I)V
    pub fn new__iterat_i(&self, iterator: Object, characteristics: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.<init>:(Ljava/util/Iterator;I)V")
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.trySplit:()Ljava/util/Spliterator;")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.tryAdvance:(Ljava/util/function/Consumer;)Z")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.estimateSize:()J")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.characteristics:()I")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        panic!("stub: java/util/Spliterators$IteratorSpliterator.getComparator:()Ljava/util/Comparator;")
    }
}
