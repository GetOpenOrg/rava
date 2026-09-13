#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedNavigableSet",
    super_class = "java/util/Collections$CheckedSortedSet",
    interfaces  = "java/util/NavigableSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedNavigableSet<E> {
    #[cfg_attr(any(), java_field(name = "ns", descriptor = "Ljava/util/NavigableSet;", access = "private final"))]
    pub ns: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedNavigableSet<E> {
    // java: <init>(Ljava/util/NavigableSet;Ljava/lang/Class;)V
    pub fn new(&self, s: Object, type_: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.<init>:(Ljava/util/NavigableSet;Ljava/lang/Class;)V")
    }

    // java: lower(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lower(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.lower:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: floor(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floor(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.floor:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: ceiling(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceiling(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.ceiling:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: higher(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higher(&self, e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.higher:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: pollFirst()Ljava/lang/Object;
    pub fn pollFirst(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.pollFirst:()Ljava/lang/Object;")
    }

    // java: pollLast()Ljava/lang/Object;
    pub fn pollLast(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.pollLast:()Ljava/lang/Object;")
    }

    // java: descendingSet()Ljava/util/NavigableSet;
    pub fn descendingSet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.descendingSet:()Ljava/util/NavigableSet;")
    }

    // java: descendingIterator()Ljava/util/Iterator;
    pub fn descendingIterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.descendingIterator:()Ljava/util/Iterator;")
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn subSet__obj_obj(&self, fromElement: Object, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.subSet:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableSet;")
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn headSet__obj(&self, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.headSet:(Ljava/lang/Object;)Ljava/util/NavigableSet;")
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/NavigableSet;
    pub fn tailSet__obj(&self, fromElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.tailSet:(Ljava/lang/Object;)Ljava/util/NavigableSet;")
    }

    // java: subSet(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn subSet__obj_z_obj_z(&self, fromElement: Object, fromInclusive: bool, toElement: Object, toInclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.subSet:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableSet;")
    }

    // java: headSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn headSet__obj_z(&self, toElement: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.headSet:(Ljava/lang/Object;Z)Ljava/util/NavigableSet;")
    }

    // java: tailSet(Ljava/lang/Object;Z)Ljava/util/NavigableSet;
    pub fn tailSet__obj_z(&self, fromElement: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableSet.tailSet:(Ljava/lang/Object;Z)Ljava/util/NavigableSet;")
    }
}
