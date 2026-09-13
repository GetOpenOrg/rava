#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableNavigableSet",
    super_class = "java/util/Collections$UnmodifiableSortedSet",
    interfaces  = "java/util/NavigableSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_UnmodifiableNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;)V
    pub fn new(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.<init>:(Ljava/util/NavigableSet;)V")
    }

    // java: lower(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lower(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.lower:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: floor(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floor(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.floor:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: ceiling(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceiling(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.ceiling:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: higher(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higher(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.higher:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: pollFirst()Ljava/lang/Object;
    pub fn pollFirst(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.pollFirst:()Ljava/lang/Object;")
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.pollLast:()Ljava/lang/Object;")
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.descendingSet:()Ljava/util/NavigableSet;")
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.descendingIterator:()Ljava/util/Iterator;")
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet(&self, fromElement: Object, fromInclusive: bool, toElement: Object, toInclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.subSet:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;")
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet(&self, toElement: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.headSet:(Ljava/lang/Object;Z)Ljava/util/NavigableSet;")
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet(&self, fromElement: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableSet.tailSet:(Ljava/lang/Object;Z)Ljava/util/NavigableSet;")
    }
}
