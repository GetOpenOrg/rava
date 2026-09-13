#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedSortedSet",
    super_class = "java/util/Collections$SynchronizedSet",
    interfaces  = "java/util/SortedSet",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedSortedSet<E> {
    #[cfg_attr(any(), java_field(name = "ss", descriptor = "Ljava/util/SortedSet;", access = "private final"))]
    pub ss: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedSortedSet<E> {
    // java: <init>(Ljava/util/SortedSet;)V
    pub fn new__sorted(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.<init>:(Ljava/util/SortedSet;)V")
    }

    // java: <init>(Ljava/util/SortedSet;Ljava/lang/Object;)V
    pub fn new__sorted_obj(&self, s: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.<init>:(Ljava/util/SortedSet;Ljava/lang/Object;)V")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.comparator:()Ljava/util/Comparator;")
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn subSet(&self, fromElement: Object, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.subSet:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;")
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn headSet(&self, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.headSet:(Ljava/lang/Object;)Ljava/util/SortedSet;")
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn tailSet(&self, fromElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.tailSet:(Ljava/lang/Object;)Ljava/util/SortedSet;")
    }

    // java: first()Ljava/lang/Object;
    pub fn first(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.first:()Ljava/lang/Object;")
    }

    // java: last()Ljava/lang/Object;
    pub fn last(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedSortedSet.last:()Ljava/lang/Object;")
    }
}
