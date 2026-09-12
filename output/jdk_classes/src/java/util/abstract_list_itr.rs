#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$Itr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_Itr {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I"))]
    pub lastRet: Field<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I"))]
    pub expectedModCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "final"))]
    pub this_0: Field<Object>,
}

impl AbstractList_Itr {
    // java: <init>(Ljava/util/AbstractList;)V
    pub fn new(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/AbstractList$Itr.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/AbstractList$Itr.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractList$Itr.next")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/AbstractList$Itr.remove")
    }

    // java: checkForComodification()V
    pub fn checkForComodification(&self) -> Result<()> {
        todo!("abstract java/util/AbstractList$Itr.checkForComodification")
    }
}
