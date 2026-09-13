#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$AbstractImmutableList",
    super_class = "java/util/ImmutableCollections$AbstractImmutableCollection",
    interfaces  = "java/util/List,java/util/RandomAccess",
    access      = "abstract",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_AbstractImmutableList<E>(std::marker::PhantomData<E>);

impl<E: Clone + 'static> ImmutableCollections_AbstractImmutableList<E> {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.<init>:()V")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.add:(ILjava/lang/Object;)V")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.addAll:(ILjava/util/Collection;)Z")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.remove:(I)Ljava/lang/Object;")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.sort:(Ljava/util/Comparator;)V")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.subList:(II)Ljava/util/List;")
    }

    // java: subListRangeCheck(III)V
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.subListRangeCheck:(III)V")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.iterator:()Ljava/util/Iterator;")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.listIterator:()Ljava/util/ListIterator;")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.listIterator:(I)Ljava/util/ListIterator;")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.hashCode:()I")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.contains:(Ljava/lang/Object;)Z")
    }

    // java: reversed()Ljava/util/List;
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.reversed:()Ljava/util/List;")
    }

    // java: outOfBounds(I)Ljava/lang/IndexOutOfBoundsException;
    pub fn outOfBounds(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/ImmutableCollections$AbstractImmutableList.outOfBounds:(I)Ljava/lang/IndexOutOfBoundsException;")
    }
}
