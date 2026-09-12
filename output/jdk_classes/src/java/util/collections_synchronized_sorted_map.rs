#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedSortedMap",
    super_class = "java/util/Collections$SynchronizedMap",
    interfaces  = "java/util/SortedMap",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedSortedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "sm", descriptor = "Ljava/util/SortedMap;", access = "private final"))]
    pub sm: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SynchronizedSortedMap<K, V> {
    // java: <init>(Ljava/util/SortedMap;)V
    pub fn new__sorted(&self, m: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.<init>")
    }

    // java: <init>(Ljava/util/SortedMap;Ljava/lang/Object;)V
    pub fn new__sorted_obj(&self, m: Object, mutex: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.<init>")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.comparator")
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap(&self, fromKey: Object, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.subMap")
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap(&self, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.headMap")
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap(&self, fromKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.tailMap")
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.firstKey")
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedSortedMap.lastKey")
    }
}
