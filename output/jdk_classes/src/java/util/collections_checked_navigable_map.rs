#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedNavigableMap",
    super_class = "java/util/Collections$CheckedSortedMap",
    interfaces  = "java/util/NavigableMap,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedNavigableMap<K, V> {
    #[cfg_attr(any(), java_field(name = "nm", descriptor = "Ljava/util/NavigableMap;", access = "private final"))]
    pub nm: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_CheckedNavigableMap<K, V> {
    // java: <init>(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)V
    pub fn new(&self, m: Object, keyType: Object, valueType: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.<init>:(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)V")
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.comparator:()Ljava/util/Comparator;")
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.firstKey:()Ljava/lang/Object;")
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.lastKey:()Ljava/lang/Object;")
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.lowerEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.lowerKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.floorEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.floorKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.ceilingEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.ceilingKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.higherEntry:(Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.higherKey:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.firstEntry:()Ljava/util/Map$Entry;")
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.lastEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.pollFirstEntry:()Ljava/util/Map$Entry;")
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.pollLastEntry:()Ljava/util/Map$Entry;")
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.descendingMap:()Ljava/util/NavigableMap;")
    }

    // java: keySet()Ljava/util/NavigableSet;
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.keySet:()Ljava/util/NavigableSet;")
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.navigableKeySet:()Ljava/util/NavigableSet;")
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.descendingKeySet:()Ljava/util/NavigableSet;")
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn subMap__obj_obj(&self, fromKey: Object, toKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.subMap:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableMap;")
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn headMap__obj(&self, toKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.headMap:(Ljava/lang/Object;)Ljava/util/NavigableMap;")
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn tailMap__obj(&self, fromKey: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.tailMap:(Ljava/lang/Object;)Ljava/util/NavigableMap;")
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap__obj_z_obj_z(&self, fromKey: Object, fromInclusive: bool, toKey: Object, toInclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.subMap:(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap__obj_z(&self, toKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.headMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap__obj_z(&self, fromKey: Object, inclusive: bool) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedNavigableMap.tailMap:(Ljava/lang/Object;Z)Ljava/util/NavigableMap;")
    }
}
