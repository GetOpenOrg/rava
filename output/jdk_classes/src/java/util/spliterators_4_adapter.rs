#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$4Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator$OfDouble,java/util/function/DoubleConsumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_4Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "D"))]
    pub nextElement: Field<f64>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator$OfDouble;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_4Adapter {
    // java: <init>(Ljava/util/Spliterator$OfDouble;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$4Adapter.<init>")
    }

    // java: accept(D)V
    pub fn accept(&self, t: f64) -> Result<()> {
        todo!("abstract java/util/Spliterators$4Adapter.accept")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/Spliterators$4Adapter.hasNext")
    }

    // java: nextDouble()D
    pub fn nextDouble(&self) -> Result<f64> {
        todo!("abstract java/util/Spliterators$4Adapter.nextDouble")
    }

    // java: forEachRemaining(Ljava/util/function/DoubleConsumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Spliterators$4Adapter.forEachRemaining")
    }
}
