#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ListIterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "public abstract",
    source      = "ListIterator.java",
))]
pub struct ListIterator<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ListIterator<E> {
    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/ListIterator.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/ListIterator.next")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        todo!("abstract java/util/ListIterator.hasPrevious")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        todo!("abstract java/util/ListIterator.previous")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ListIterator.nextIndex")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ListIterator.previousIndex")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/ListIterator.remove")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/ListIterator.set")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/ListIterator.add")
    }
}
