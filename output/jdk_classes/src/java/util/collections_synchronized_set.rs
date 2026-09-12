#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedSet",
    super_class = "java/util/Collections$SynchronizedCollection",
    interfaces  = "java/util/Set",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedSet<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_SynchronizedSet<E> {
    // java: <init>(Ljava/util/Set;)V
    pub fn new__set(&self, s: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedSet.<init>")
    }

    // java: <init>(Ljava/util/Set;Ljava/lang/Object;)V
    pub fn new__set_obj(&self, s: Object, mutex: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedSet.<init>")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SynchronizedSet.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SynchronizedSet.hashCode")
    }
}
