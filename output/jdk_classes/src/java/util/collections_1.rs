#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$1",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_1 {
    #[cfg_attr(any(), java_field(name = "hasNext", descriptor = "Z", access = "private"))]
    pub hasNext: Field<bool>,
    #[cfg_attr(any(), java_field(name = "val$e", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub val_e: Field<Object>,
}

impl Collections_1 {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/Collections$1.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$1.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$1.next")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/Collections$1.remove")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$1.forEachRemaining")
    }
}
