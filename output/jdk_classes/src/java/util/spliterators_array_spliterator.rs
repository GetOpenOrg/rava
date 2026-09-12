#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$ArraySpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "final",
    source      = "Spliterators.java",
))]
pub struct Spliterators_ArraySpliterator<T> {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub array: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "characteristics", descriptor = "I", access = "private final"))]
    pub characteristics: Field<i32>,
    #[cfg_attr(any(), java_field(name = "estimatedSize", descriptor = "J", access = "private"))]
    pub estimatedSize: Field<i64>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Spliterators_ArraySpliterator<T> {
    // java: <init>([Ljava/lang/Object;I)V
    pub fn new__arr_obj_i(&self, array: Vec<Object>, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.<init>")
    }

    // java: <init>([Ljava/lang/Object;III)V
    pub fn new__arr_obj_i_i_i(&self, array: Vec<Object>, origin: i32, fence: i32, additionalCharacteristics: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.<init>")
    }

    // java: <init>([Ljava/lang/Object;IIIJ)V
    pub fn new__arr_obj_i_i_i_l(&self, array: Vec<Object>, origin: i32, fence: i32, characteristics: i32, estimatedSize: i64) -> Result<()> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.trySplit")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.forEachRemaining")
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.tryAdvance")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.characteristics")
    }

    // java: getComparator()Ljava/util/Comparator;
    pub fn getComparator(&self) -> Result<Object> {
        todo!("abstract java/util/Spliterators$ArraySpliterator.getComparator")
    }
}
