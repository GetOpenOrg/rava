#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableNavigableMap",
    super_class = "java/util/Collections$UnmodifiableSortedMap",
    interfaces  = "java/util/NavigableMap,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableNavigableMap<K, V> {
    #[cfg_attr(any(), java_field(name = "nm", descriptor = "Ljava/util/NavigableMap;", access = "private final"))]
    pub nm: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_UnmodifiableNavigableMap<K, V> {
    // java: <init>(Ljava/util/NavigableMap;)V
    pub fn new(&self, m: Object) -> Result<()> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.<init>:(Ljava/util/NavigableMap;)V")
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.lowerKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.floorKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.ceilingKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.higherKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.lowerEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.floorEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.ceilingEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.higherEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.firstEntry:()Ljava/util/Map$Entry;")
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.lastEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.pollFirstEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.pollLastEntry:()Ljava/util/Map$Entry;")
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.descendingMap:()Ljava/util/NavigableMap;")
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.navigableKeySet:()Ljava/util/NavigableSet;")
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.descendingKeySet:()Ljava/util/NavigableSet;")
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap(&self, fromKey: Object, fromInclusive: bool, toKey: Object, toInclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.subMap:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap(&self, toKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.headMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap(&self, fromKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$UnmodifiableNavigableMap.tailMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }
}
