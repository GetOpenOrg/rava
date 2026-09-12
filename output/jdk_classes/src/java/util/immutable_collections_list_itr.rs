#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$ListItr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/ListIterator",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_ListItr<E> {
    #[cfg_attr(any(), java_field(name = "list", descriptor = "Ljava/util/List;", access = "private final"))]
    pub list: Field<Object>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "isListIterator", descriptor = "Z", access = "private final"))]
    pub isListIterator: Field<bool>,
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private"))]
    pub cursor: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_ListItr<E> {
    // java: <init>(Ljava/util/List;I)V
    pub fn new__list_i(&self, list: Object, size: i32) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$ListItr.<init>")
    }

    // java: <init>(Ljava/util/List;II)V
    pub fn new__list_i_i(&self, list: Object, size: i32, index: i32) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$ListItr.<init>")
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$ListItr.hasNext")
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$ListItr.next")
    }

    // java: remove()V
    pub fn remove(&self) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$ListItr.remove")
    }

    // java: hasPrevious()Z
    pub fn hasPrevious(&self) -> Result<bool> {
        todo!("abstract java/util/ImmutableCollections$ListItr.hasPrevious")
    }

    // java: previous()Ljava/lang/Object;
    pub fn previous(&self) -> Result<Object> {
        todo!("abstract java/util/ImmutableCollections$ListItr.previous")
    }

    // java: nextIndex()I
    pub fn nextIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$ListItr.nextIndex")
    }

    // java: previousIndex()I
    pub fn previousIndex(&self) -> Result<i32> {
        todo!("abstract java/util/ImmutableCollections$ListItr.previousIndex")
    }

    // java: set(Ljava/lang/Object;)V
    pub fn set(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$ListItr.set")
    }

    // java: add(Ljava/lang/Object;)V
    pub fn add(&self, e: Object) -> Result<()> {
        todo!("abstract java/util/ImmutableCollections$ListItr.add")
    }
}
