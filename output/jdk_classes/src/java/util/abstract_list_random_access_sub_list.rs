#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$RandomAccessSubList",
    super_class = "java/util/AbstractList$SubList",
    interfaces  = "java/util/RandomAccess",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_RandomAccessSubList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> AbstractList_RandomAccessSubList<E> {
    // java: <init>(Ljava/util/AbstractList;II)V
    pub fn new__abstra_i_i(&self, root: Object, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$RandomAccessSubList.<init>")
    }

    // java: <init>(Ljava/util/AbstractList$RandomAccessSubList;II)V
    pub fn new__abstra_i_i_1(&self, parent: Object, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$RandomAccessSubList.<init>")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList$RandomAccessSubList.subList")
    }
}
