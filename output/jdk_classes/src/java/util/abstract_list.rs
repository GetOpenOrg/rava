#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList",
    super_class = "java/util/AbstractCollection",
    interfaces  = "java/util/List",
    access      = "public abstract",
    source      = "AbstractList.java",
))]
pub struct AbstractList<E> {
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "protected"))]
    pub modCount: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/AbstractList.<init>")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: Object) -> Result<bool> {
        todo!("abstract java/util/AbstractList.add")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList.get")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        todo!("abstract java/util/AbstractList.set")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: Object) -> Result<()> {
        todo!("abstract java/util/AbstractList.add")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList.remove")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        todo!("abstract java/util/AbstractList.indexOf")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        todo!("abstract java/util/AbstractList.lastIndexOf")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/AbstractList.clear")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        todo!("abstract java/util/AbstractList.addAll")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractList.iterator")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        todo!("abstract java/util/AbstractList.listIterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList.listIterator")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        todo!("abstract java/util/AbstractList.subList")
    }

    // java: subListRangeCheck(III)V
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList.subListRangeCheck")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/AbstractList.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/AbstractList.hashCode")
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList.removeRange")
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        todo!("abstract java/util/AbstractList.rangeCheckForAdd")
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        todo!("abstract java/util/AbstractList.outOfBoundsMsg")
    }
}
