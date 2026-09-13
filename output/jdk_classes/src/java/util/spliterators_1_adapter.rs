#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators$1Adapter",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator,java/util/function/Consumer",
    access      = "",
    source      = "Spliterators.java",
))]
pub struct Spliterators_1Adapter {
    #[cfg_attr(any(), java_field(name = "valueReady", descriptor = "Z"))]
    pub valueReady: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nextElement", descriptor = "Ljava/lang/Object;"))]
    pub nextElement: Field<Object>,
    #[cfg_attr(any(), java_field(name = "val$spliterator", descriptor = "Ljava/util/Spliterator;", access = "final"))]
    pub val_spliterator: Field<Object>,
}

impl Spliterators_1Adapter {
    // java: <init>(Ljava/util/Spliterator;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$1Adapter.<init>:(Ljava/util/Spliterator;)V")
    }

    // java: accept(Ljava/lang/Object;)V
    pub fn accept(&self, t: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$1Adapter.accept:(Ljava/lang/Object;)V")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/Spliterators$1Adapter.hasNext:()Z")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/Spliterators$1Adapter.next:()Ljava/lang/Object;")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Spliterators$1Adapter.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
