#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$2",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_2 {
    #[cfg_attr(any(), java_field(name = "est", descriptor = "J"))]
    pub est: Field<i64>,
    #[cfg_attr(any(), java_field(name = "val$element", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub val_element: Field<Object>,
}

impl Collections_2 {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/Collections$2.<init>")
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$2.trySplit")
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, consumer: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$2.tryAdvance")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, consumer: Object) -> Result<()> {
        todo!("abstract java/util/Collections$2.forEachRemaining")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        todo!("abstract java/util/Collections$2.estimateSize")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$2.characteristics")
    }
}
