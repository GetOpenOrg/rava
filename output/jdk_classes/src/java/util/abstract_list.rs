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
        panic!("stub: java/util/AbstractList.<init>:()V")
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add__obj(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.add:(Ljava/lang/Object;)Z")
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, arg0: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.get:(I)Ljava/lang/Object;")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/AbstractList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add__i_obj(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList.add:(ILjava/lang/Object;)V")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.remove:(I)Ljava/lang/Object;")
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/AbstractList.indexOf:(Ljava/lang/Object;)I")
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/AbstractList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/AbstractList.clear:()V")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.addAll:(ILjava/util/Collection;)Z")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList.iterator:()Ljava/util/Iterator;")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList.listIterator:()Ljava/util/ListIterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.subList:(II)Ljava/util/List;")
    }

    // java: subListRangeCheck(III)V
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.subListRangeCheck:(III)V")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList.hashCode:()I")
    }

    // java: removeRange(II)V
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.removeRange:(II)V")
    }

    // java: rangeCheckForAdd(I)V
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.rangeCheckForAdd:(I)V")
    }

    // java: outOfBoundsMsg(I)Ljava/lang/String;
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        panic!("stub: java/util/AbstractList.outOfBoundsMsg:(I)Ljava/lang/String;")
    }
}
