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
        panic!("stub: java/util/ListIterator.hasNext:()Z")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/ListIterator.next:()Ljava/lang/Object;")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        panic!("stub: java/util/ListIterator.hasPrevious:()Z")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        panic!("stub: java/util/ListIterator.previous:()Ljava/lang/Object;")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ListIterator.nextIndex:()I")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ListIterator.previousIndex:()I")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/ListIterator.remove:()V")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/ListIterator.set:(Ljava/lang/Object;)V")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/ListIterator.add:(Ljava/lang/Object;)V")
    }
}
