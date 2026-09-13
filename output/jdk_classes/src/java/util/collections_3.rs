#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$3",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Enumeration",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_3 {
    #[cfg_attr(any(), java_field(name = "i", descriptor = "Ljava/util/Iterator;", access = "private final"))]
    pub i: Field<Object>,
    #[cfg_attr(any(), java_field(name = "val$c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub val_c: Field<Object>,
}

impl Collections_3 {
    // java: <init>(Ljava/util/Collection;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/Collections$3.<init>:(Ljava/util/Collection;)V")
    }

    // java: hasMoreElements()Z
    pub fn hasMoreElements(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$3.hasMoreElements:()Z")
    }

    // java: nextElement()Ljava/lang/Object;
    pub fn nextElement(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$3.nextElement:()Ljava/lang/Object;")
    }
}
