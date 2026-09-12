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
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.<init>")
    }

    // java: add(ILjava/lang/Object;)V
    pub fn add(&self, index: i32, element: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.add")
    }

    // java: addAll(ILjava/util/Collection;)Z
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.addAll")
    }

    // java: remove(I)Ljava/lang/Object;
    pub fn remove(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.remove")
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.replaceAll")
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.set")
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.sort")
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.subList")
    }

    // java: subListRangeCheck(III)V
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.subListRangeCheck")
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.iterator")
    }

    // java: listIterator()Ljava/util/ListIterator;
    pub fn listIterator(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.listIterator")
    }

    // java: listIterator(I)Ljava/util/ListIterator;
    pub fn listIterator__i(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.listIterator")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.hashCode")
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.contains")
    }

    // java: reversed()Ljava/util/List;
    pub fn reversed(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.reversed")
    }

    // java: outOfBounds(I)Ljava/lang/IndexOutOfBoundsException;
    pub fn outOfBounds(&self, index: i32) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$AbstractImmutableList.outOfBounds")
    }
}
