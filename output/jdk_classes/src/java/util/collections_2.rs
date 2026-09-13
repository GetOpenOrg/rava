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
        panic!("stub: java/util/Collections$2.<init>:(Ljava/lang/Object;)V")
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$2.trySplit:()Ljava/util/Spliterator;")
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, consumer: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$2.tryAdvance:(Ljava/util/function/Consumer;)Z")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, consumer: Object) -> Result<()> {
        panic!("stub: java/util/Collections$2.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        panic!("stub: java/util/Collections$2.estimateSize:()J")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$2.characteristics:()I")
    }
}
