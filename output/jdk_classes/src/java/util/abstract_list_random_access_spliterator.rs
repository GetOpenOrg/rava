#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/AbstractList$RandomAccessSpliterator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Spliterator",
    access      = "final",
    source      = "AbstractList.java",
))]
pub struct AbstractList_RandomAccessSpliterator<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "private final"))]
    pub list: Field<Object>,
    #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private"))]
    pub index: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "private"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "alist", descriptor = "Ljava/util/AbstractList;", access = "private final"))]
    pub alist: Field<Object>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", access = "private"))]
    pub expectedModCount: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> AbstractList_RandomAccessSpliterator<E> {
    // java: <init>(Ljava/util/List;)V
    pub fn new__list(&self, list: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.<init>:(Ljava/util/List;)V")
    }

    // java: <init>(Ljava/util/AbstractList$RandomAccessSpliterator;II)V
    pub fn new__abstra_i_i(&self, parent: Object, origin: i32, fence: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.<init>:(Ljava/util/AbstractList$RandomAccessSpliterator;II)V")
    }

    // java: getFence()I
    pub fn getFence(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.getFence:()I")
    }

    // java: trySplit()Ljava/util/Spliterator;
    pub fn trySplit(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.trySplit:()Ljava/util/Spliterator;")
    }

    // java: tryAdvance(Ljava/util/function/Consumer;)Z
    pub fn tryAdvance(&self, action: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.tryAdvance:(Ljava/util/function/Consumer;)Z")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }

    // java: estimateSize()J
    pub fn estimateSize(&self) -> Result<i64> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.estimateSize:()J")
    }

    // java: characteristics()I
    pub fn characteristics(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.characteristics:()I")
    }

    // java: get(Ljava/util/List;I)Ljava/lang/Object;
    pub fn get(list: Object, i: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.get:(Ljava/util/List;I)Ljava/lang/Object;")
    }

    // java: checkAbstractListModCount(Ljava/util/AbstractList;I)V
    pub fn checkAbstractListModCount(alist: Object, expectedModCount: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList$RandomAccessSpliterator.checkAbstractListModCount:(Ljava/util/AbstractList;I)V")
    }
}
