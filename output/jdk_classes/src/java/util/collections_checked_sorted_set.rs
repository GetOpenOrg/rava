#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedSortedSet",
    super_class = "java/util/Collections$CheckedSet",
    interfaces  = "java/util/SortedSet,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedSortedSet<E> {
    #[cfg_attr(any(), java_field(name = "ss", descriptor = "Ljava/util/SortedSet;", access = "private final"))]
    pub ss: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedSortedSet<E> {
    // java: <init>(Ljava/util/SortedSet;Ljava/lang/Class;)V
    pub fn new(&self, s: Object, type_: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedSortedSet.<init>:(Ljava/util/SortedSet;Ljava/lang/Class;)V")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.comparator:()Ljava/util/Comparator;")
    }

    // java: first()Ljava/lang/Object;
    pub fn first(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.first:()Ljava/lang/Object;")
    }

    // java: last()Ljava/lang/Object;
    pub fn last(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.last:()Ljava/lang/Object;")
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn subSet(&self, fromElement: Object, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.subSet:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;")
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn headSet(&self, toElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.headSet:(Ljava/lang/Object;)Ljava/util/SortedSet;")
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn tailSet(&self, fromElement: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedSortedSet.tailSet:(Ljava/lang/Object;)Ljava/util/SortedSet;")
    }
}
