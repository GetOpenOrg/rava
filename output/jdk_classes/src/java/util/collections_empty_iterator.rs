#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyIterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyIterator<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyIterator.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$EmptyIterator.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$EmptyIterator.next")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyIterator.remove")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyIterator.forEachRemaining")
    }
}
