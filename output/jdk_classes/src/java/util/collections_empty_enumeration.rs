#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyEnumeration",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Enumeration",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyEnumeration<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyEnumeration<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyEnumeration.<init>")
    }

    // java: hasMoreElements()Z
    pub fn hasMoreElements(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$EmptyEnumeration.hasMoreElements")
    }

    // java: nextElement()Ljava/lang/Object;
    pub fn nextElement(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$EmptyEnumeration.nextElement")
    }

    // java: asIterator()Ljava/util/Iterator;
    pub fn asIterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$EmptyEnumeration.asIterator")
    }
}
