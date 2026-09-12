#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$EmptyListIterator",
    super_class = "java/util/Collections$EmptyIterator",
    interfaces  = "java/util/ListIterator",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_EmptyListIterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_EmptyListIterator<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyListIterator.<init>")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$EmptyListIterator.hasPrevious")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$EmptyListIterator.previous")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$EmptyListIterator.nextIndex")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$EmptyListIterator.previousIndex")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyListIterator.set")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/Collections$EmptyListIterator.add")
    }
}
