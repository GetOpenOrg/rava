#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedRandomAccessList",
    super_class = "java/util/Collections$SynchronizedList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedRandomAccessList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> Collections_SynchronizedRandomAccessList<E> {
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(&self, list: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedRandomAccessList.<init>:(Ljava/util/List;)V")
    }

    // java: <init>(Ljava/util/List;Ljava/lang/Object;)V
    pub fn new__list_obj(&self, list: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedRandomAccessList.<init>:(Ljava/util/List;Ljava/lang/Object;)V")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedRandomAccessList.subList:(II)Ljava/util/List;")
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedRandomAccessList.writeReplace:()Ljava/lang/Object;")
    }
}
