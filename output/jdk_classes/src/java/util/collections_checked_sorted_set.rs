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
        todo!("abstract java/util/Collections$CheckedSortedSet.<init>")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.comparator")
    }

    // java: first()Ljava/lang/Object;
    pub fn first(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.first")
    }

    // java: last()Ljava/lang/Object;
    pub fn last(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.last")
    }

    // java: subSet(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn subSet(&self, fromElement: Object, toElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.subSet")
    }

    // java: headSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn headSet(&self, toElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.headSet")
    }

    // java: tailSet(Ljava/lang/Object;)Ljava/util/SortedSet;
    pub fn tailSet(&self, fromElement: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedSet.tailSet")
    }
}
