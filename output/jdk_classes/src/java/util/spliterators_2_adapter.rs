#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$2Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfInt,java/util/function/IntConsumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_2Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "I"))]
    pub nextElement: Field<i32>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator$OfInt;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_2Adapter {
    // java: <init>(Ljava/util/Spliterator$OfInt;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$2Adapter.<init>:(Ljava/util/Spliterator$OfInt;)V")
    }

    // java: accept(I)V
    pub fn accept(&self, t: i32) -> Result<()> {
        panic!("stub: java/util/Spliterators$2Adapter.accept:(I)V")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/Spliterators$2Adapter.hasNext:()Z")
    }

    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        panic!("stub: java/util/Spliterators$2Adapter.nextInt:()I")
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$2Adapter.forEachRemaining:(Ljava/util/function/IntConsumer;)V")
    }
}
