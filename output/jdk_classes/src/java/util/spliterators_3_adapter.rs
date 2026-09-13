#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$3Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfLong,java/util/function/LongConsumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_3Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "J"))]
    pub nextElement: Field<i64>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator$OfLong;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_3Adapter {
    // java: <init>(Ljava/util/Spliterator$OfLong;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$3Adapter.<init>:(Ljava/util/Spliterator$OfLong;)V")
    }

    // java: accept(J)V
    pub fn accept(&self, t: i64) -> Result<()> {
        panic!("stub: java/util/Spliterators$3Adapter.accept:(J)V")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/Spliterators$3Adapter.hasNext:()Z")
    }

    // java: nextLong()J
    pub fn nextLong(&self) -> Result<i64> {
        panic!("stub: java/util/Spliterators$3Adapter.nextLong:()J")
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$3Adapter.forEachRemaining:(Ljava/util/function/LongConsumer;)V")
    }
}
