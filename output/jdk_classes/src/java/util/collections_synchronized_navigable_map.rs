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
    // java: <init>(Ljava/util/NavigableMap;)V
    pub fn new__naviga(m: Object) -> Result<Self> {
        let this = Self { nm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSortedMap.<init>:(Ljava/util/SortedMap;)V */
        this.nm.set(m);
        Ok(this)
    }

    // java: <init>(Ljava/util/NavigableMap;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/NavigableMap;Ljava/lang/Object;)V
    pub fn new__naviga_obj(m: Object, mutex: Object) -> Result<Self> {
        let this = Self { nm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedSortedMap.<init>:(Ljava/util/SortedMap;Ljava/lang/Object;)V */
        this.nm.set(m);
        Ok(this)
    }

    // java: lowerEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn lowerEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().lowerEntry(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: lowerKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn lowerKey(&self, key: K) -> Result<K> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().lowerKey(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: floorEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn floorEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().floorEntry(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: floorKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn floorKey(&self, key: K) -> Result<K> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().floorKey(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: ceilingEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn ceilingEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().ceilingEntry(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: ceilingKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn ceilingKey(&self, key: K) -> Result<K> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().ceilingKey(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: higherEntry(Ljava/lang/Object;)Ljava/util/Map$Entry;
    pub fn higherEntry(&self, key: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().higherEntry(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: higherKey(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn higherKey(&self, key: K) -> Result<K> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().higherKey(key)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: firstEntry()Ljava/util/Map$Entry;
    pub fn firstEntry(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().firstEntry()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: lastEntry()Ljava/util/Map$Entry;
    pub fn lastEntry(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().lastEntry()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollFirstEntry()Ljava/util/Map$Entry;
    pub fn pollFirstEntry(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().pollFirstEntry()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: pollLastEntry()Ljava/util/Map$Entry;
    pub fn pollLastEntry(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().pollLastEntry()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingMap()Ljava/util/NavigableMap;
    pub fn descendingMap(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().descendingMap()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
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
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().navigableKeySet()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: descendingKeySet()Ljava/util/NavigableSet;
    pub fn descendingKeySet(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().descendingKeySet()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Collections_SynchronizedNavigableSet = Collections_SynchronizedNavigableSet::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap__obj_obj(&self, fromKey: K, toKey: K) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().subMap(fromKey, 1i32, toKey, 0i32)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap__obj(&self, toKey: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().headMap(toKey, 0i32)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap__obj(&self, fromKey: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().tailMap(fromKey, 1i32)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: subMap(Ljava/lang/Object;ZLjava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn subMap__obj_z_obj_z(&self, fromKey: K, fromInclusive: bool, toKey: K, toInclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_5: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().subMap(fromKey, fromInclusive, toKey, toInclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_5);
        let mut local_6: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: headMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn headMap__obj_z(&self, toKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().headMap(toKey, inclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    // java: tailMap(Ljava/lang/Object;Z)Ljava/util/NavigableMap;
    pub fn tailMap__obj_z(&self, fromKey: K, inclusive: bool) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.nm.get().tailMap(fromKey, inclusive)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedNavigableMap = Collections_SynchronizedNavigableMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
