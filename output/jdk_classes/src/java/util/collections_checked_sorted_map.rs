#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedSortedMap",
    super_class = "java/util/Collections$CheckedMap",
    interfaces  = "java/util/SortedMap,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedSortedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "sm", descriptor = "Ljava/util/SortedMap;", access = "private final"))]
    pub sm: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_CheckedSortedMap<K, V> {
    // java: <init>(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)V
    pub fn new(&self, m: Object, keyType: Object, valueType: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedSortedMap.<init>")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.comparator")
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.firstKey")
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.lastKey")
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap(&self, fromKey: Object, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.subMap")
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap(&self, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.headMap")
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap(&self, fromKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedSortedMap.tailMap")
    }
}
