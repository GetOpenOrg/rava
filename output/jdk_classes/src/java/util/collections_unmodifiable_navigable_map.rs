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
    pub fn new(m: Object) -> Result<Self> {
        let this = Self { nm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$UnmodifiableSortedMap.<init>:(Ljava/util/SortedMap;)V */
        this.nm.set(m);
        Ok(this)
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().lowerKey(key)?;
        Ok(_t0)
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().floorKey(key)?;
        Ok(_t0)
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().ceilingKey(key)?;
        Ok(_t0)
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: K) -> Result<K> {
        let this = self;
        let _t0 = this.nm.get().higherKey(key)?;
        Ok(_t0)
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().lowerEntry(key)?;
        let mut lower: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(lower)?)
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().floorEntry(key)?;
        let mut floor: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(floor)?)
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().ceilingEntry(key)?;
        let mut ceiling: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(ceiling)?)
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().higherEntry(key)?;
        let mut higher: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(higher)?)
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().firstEntry()?;
        let mut first: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(first)?)
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().lastEntry()?;
        let mut last: Object = _t0;
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        Ok(Collections_UnmodifiableMap_UnmodifiableEntrySet_UnmodifiableEntry::new(last)?)
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().descendingMap()?;
        let _t1: Object = Collections::unmodifiableNavigableMap(_t0)?;
        Ok(_t1)
    }

    // java: navigableKeySet()Ljava/util/NavigableSet;
    pub fn navigableKeySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().navigableKeySet()?;
        let _t1: Object = Collections::unmodifiableNavigableSet(_t0)?;
        Ok(_t1)
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().descendingKeySet()?;
        let _t1: Object = Collections::unmodifiableNavigableSet(_t0)?;
        Ok(_t1)
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap(&self, fromKey: K, fromInclusive: bool, toKey: K, toInclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().subMap(fromKey, fromInclusive, toKey, toInclusive)?;
        let _t1: Object = Collections::unmodifiableNavigableMap(_t0)?;
        Ok(_t1)
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap(&self, toKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().headMap(toKey, inclusive)?;
        let _t1: Object = Collections::unmodifiableNavigableMap(_t0)?;
        Ok(_t1)
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap(&self, fromKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.nm.get().tailMap(fromKey, inclusive)?;
        let _t1: Object = Collections::unmodifiableNavigableMap(_t0)?;
        Ok(_t1)
    }
}
