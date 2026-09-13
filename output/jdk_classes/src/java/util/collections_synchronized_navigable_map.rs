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
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.<init>:(Ljava/util/NavigableMap;)V")
    }

    // java: <init>(Ljava/util/NavigableMap;Ljava/lang/Object;)V
    pub fn new__naviga_obj(&self, m: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.<init>:(Ljava/util/NavigableMap;Ljava/lang/Object;)V")
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.lowerEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.lowerKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.floorEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.floorKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.ceilingEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.ceilingKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.higherEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.higherKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.firstEntry:()Ljava/util/Map$Entry;")
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.lastEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.pollFirstEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.pollLastEntry:()Ljava/util/Map$Entry;")
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.descendingMap:()Ljava/util/NavigableMap;")
    }

    // java: keySet()Ljava/util/NavigableSet;
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.keySet:()Ljava/util/NavigableSet;")
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.navigableKeySet:()Ljava/util/NavigableSet;")
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.descendingKeySet:()Ljava/util/NavigableSet;")
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap__obj_obj(&self, fromKey: Object, toKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.subMap:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;")
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap__obj(&self, toKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.headMap:(Ljava/lang/Object;)Ljava/util/SortedMap;")
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap__obj(&self, fromKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.tailMap:(Ljava/lang/Object;)Ljava/util/SortedMap;")
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap__obj_z_obj_z(&self, fromKey: Object, fromInclusive: bool, toKey: Object, toInclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.subMap:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap__obj_z(&self, toKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.headMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap__obj_z(&self, fromKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedNavigableMap.tailMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }
}
