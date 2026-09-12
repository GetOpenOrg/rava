#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedNavigableSet",
    super_class = "java/util/Collections$SynchronizedSortedSet",
    interfaces  = "java/util/NavigableSet",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SynchronizedNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;)V
    pub fn new__naviga(&self, s: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.<init>")
    }

    // java: <init>(Ljava/util/NavigableSet;Ljava/lang/Object;)V
    pub fn new__naviga_obj(&self, s: Object, mutex: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.<init>")
    }

    // java: lower(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lower(&self, e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.lower")
    }

    // java: floor(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floor(&self, e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.floor")
    }

    // java: ceiling(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceiling(&self, e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.ceiling")
    }

    // java: higher(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higher(&self, e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.higher")
    }

    // java: pollFirst()Ljava/lang/Object;
    pub fn pollFirst(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.pollFirst")
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.pollLast")
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.descendingSet")
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.descendingIterator")
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn subSet__obj_obj(&self, fromElement: Object, toElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.subSet")
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn headSet__obj(&self, toElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.headSet")
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn tailSet__obj(&self, fromElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.tailSet")
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet__obj_z_obj_z(&self, fromElement: Object, fromInclusive: bool, toElement: Object, toInclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.subSet")
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet__obj_z(&self, toElement: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.headSet")
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet__obj_z(&self, fromElement: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableSet.tailSet")
    }
}
