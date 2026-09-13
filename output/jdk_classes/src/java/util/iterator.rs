#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Iterator",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Iterator.java",
))]
pub struct Iterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Iterator<E> {
    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/Iterator.hasNext:()Z")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/Iterator.next:()Ljava/lang/Object;")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/Iterator.remove:()V")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Iterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
