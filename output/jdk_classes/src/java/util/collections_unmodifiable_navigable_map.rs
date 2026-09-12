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
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.<init>")
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.lowerKey")
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.floorKey")
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.ceilingKey")
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.higherKey")
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.lowerEntry")
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.floorEntry")
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.ceilingEntry")
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.higherEntry")
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.firstEntry")
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.lastEntry")
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.pollFirstEntry")
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.pollLastEntry")
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.descendingMap")
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.navigableKeySet")
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.descendingKeySet")
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap(&self, fromKey: Object, fromInclusive: bool, toKey: Object, toInclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.subMap")
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap(&self, toKey: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.headMap")
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap(&self, fromKey: Object, inclusive: bool) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableNavigableMap.tailMap")
    }
}
