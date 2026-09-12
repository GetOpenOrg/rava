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
    pub fn new(m: Object, keyType: Object, valueType: Object) -> Result<Self> {
        let this = Self { nm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$CheckedSortedMap.<init>:(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)V */
        this.nm.set(m);
        Ok(this)
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().comparator()?;
        Ok(_t0)
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().firstKey()?;
        Ok(_t0)
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().lastKey()?;
        Ok(_t0)
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().lowerEntry(key)?;
        let mut lower: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(lower, this.valueType.get())?)
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().lowerKey(key)?;
        Ok(_t0)
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().floorEntry(key)?;
        let mut floor: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(floor, this.valueType.get())?)
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().floorKey(key)?;
        Ok(_t0)
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().ceilingEntry(key)?;
        let mut ceiling: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(ceiling, this.valueType.get())?)
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().ceilingKey(key)?;
        Ok(_t0)
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().higherEntry(key)?;
        let mut higher: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(higher, this.valueType.get())?)
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().higherKey(key)?;
        Ok(_t0)
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().firstEntry()?;
        let mut first: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(first, this.valueType.get())?)
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().lastEntry()?;
        let mut last: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(last, this.valueType.get())?)
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().pollFirstEntry()?;
        let mut entry: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(entry, this.valueType.get())?)
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().pollLastEntry()?;
        let mut entry: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_CheckedMap_CheckedEntrySet_CheckedEntry::new(entry, this.valueType.get())?)
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().descendingMap()?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: keySet()Ljava/util/NavigableSet;
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.navigableKeySet()?;
        Ok(_t0)
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().navigableKeySet()?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.keyType.get())?;
        Ok(_t1)
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().descendingKeySet()?;
        let _t1: Object = Collections::checkedNavigableSet(_t0, this.keyType.get())?;
        Ok(_t1)
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableMap;
    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn subMap__obj_obj(&self, fromKey: K, toKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().subMap(fromKey, 1i32, toKey, 0i32)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    // java: headMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn headMap__obj(&self, toKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().headMap(toKey, 0i32)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    // java: tailMap(Ljava/lang/Object;)Ljava/util/NavigableMap;
    pub fn tailMap__obj(&self, fromKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().tailMap(fromKey, 1i32)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap__obj_z_obj_z(&self, fromKey: K, fromInclusive: bool, toKey: K, toInclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().subMap(fromKey, fromInclusive, toKey, toInclusive)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap__obj_z(&self, toKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().headMap(toKey, inclusive)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap__obj_z(&self, fromKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().tailMap(fromKey, inclusive)?;
        let _t1: Object = Collections::checkedNavigableMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }
}
