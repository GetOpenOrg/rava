#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedNavigableMap",
    super_class = "java/util/Collections$SynchronizedSortedMap",
    interfaces  = "java/util/NavigableMap",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedNavigableMap<K, V> {
    #[cfg_attr(any(), java_field(name = "nm", descriptor = "Ljava/util/NavigableMap;", access = "private final"))]
    pub nm: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SynchronizedNavigableMap<K, V> {
    // java: <init>(Ljava/util/NavigableMap;)V
    pub fn new__naviga(&self, m: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.<init>")
    }

    // java: <init>(Ljava/util/NavigableMap;Ljava/lang/Object;)V
    pub fn new__naviga_obj(&self, m: Object, mutex: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.<init>")
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.lowerEntry")
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.lowerKey")
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.floorEntry")
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.floorKey")
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.ceilingEntry")
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.ceilingKey")
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.higherEntry")
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.higherKey")
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.firstEntry")
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.lastEntry")
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.pollFirstEntry")
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.pollLastEntry")
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.descendingMap")
    }

    // java: keySet()Ljava/util/NavigableSet;
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.keySet")
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.navigableKeySet")
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.descendingKeySet")
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap__obj_obj(&self, fromKey: Object, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.subMap")
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap__obj(&self, toKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.headMap")
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap__obj(&self, fromKey: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.tailMap")
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap__obj_z_obj_z(&self, fromKey: Object, fromInclusive: bool, toKey: Object, toInclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.subMap")
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap__obj_z(&self, toKey: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.headMap")
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap__obj_z(&self, fromKey: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$SynchronizedNavigableMap.tailMap")
    }
}
