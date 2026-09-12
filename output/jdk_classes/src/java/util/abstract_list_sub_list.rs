#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$SubList",
    super_class = "java/util/AbstractList",
    interfaces  = "",
    access      = "",
    source      = "AbstractList.java",
))]
pub struct AbstractList_SubList<E> {
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/AbstractList;", access = "private final"))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/AbstractList$SubList;", access = "private final"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "protected"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList_SubList<E> {
    // java: <init>(Ljava/util/AbstractList;II)V
    pub fn new__abstra_i_i(&self, root: Object, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.<init>")
    }

    // java: <init>(Ljava/util/AbstractList$SubList;II)V
    pub fn new__abstra_i_i_1(&self, parent: Object, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.<init>")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.set")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.get")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/AbstractList$SubList.size")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: Object) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.add")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.remove")
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.removeRange")
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll__coll(&self, c: Object) -> Result<bool> {
        todo!("abstract java/util/AbstractList$SubList.addAll")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll__i_coll(&self, index: i32, c: Object) -> Result<bool> {
        todo!("abstract java/util/AbstractList$SubList.addAll")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.iterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.listIterator")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList$SubList.subList")
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.rangeCheckForAdd")
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        todo!("abstract java/util/AbstractList$SubList.outOfBoundsMsg")
    }

    // java: checkForComodification()V
    pub fn checkForComodification(&self) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.checkForComodification")
    }

    // java: updateSizeAndModCount(I)V
    pub fn updateSizeAndModCount(&self, sizeChange: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList$SubList.updateSizeAndModCount")
    }
}
