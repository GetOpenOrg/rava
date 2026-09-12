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
    // java: <init>(Ljava/util/SortedMap;)V
    pub fn new__sorted(m: Object) -> Result<Self> {
        let this = Self { sm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedMap.<init>:(Ljava/util/Map;)V */
        this.sm.set(m);
        Ok(this)
    }

    // java: <init>(Ljava/util/SortedMap;Ljava/lang/Object;)V
    // java: <init>(Ljava/util/SortedMap;Ljava/lang/Object;)V
    pub fn new__sorted_obj(m: Object, mutex: Object) -> Result<Self> {
        let this = Self { sm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$SynchronizedMap.<init>:(Ljava/util/Map;Ljava/lang/Object;)V */
        this.sm.set(m);
        Ok(this)
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().comparator()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap(&self, fromKey: K, toKey: K) -> Result<Object> {
        let this = self;
        let mut local_3: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().subMap(fromKey, toKey)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_4: Collections_SynchronizedSortedMap = Collections_SynchronizedSortedMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap(&self, toKey: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().headMap(toKey)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedSortedMap = Collections_SynchronizedSortedMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap(&self, fromKey: K) -> Result<Object> {
        let this = self;
        let mut local_2: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().tailMap(fromKey)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: Collections_SynchronizedSortedMap = Collections_SynchronizedSortedMap::new(_t0, this.mutex.get())?;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<K> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().firstKey()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<K> {
        let this = self;
        let mut local_1: Object = this.mutex.get();
        /* TODO: monitorenter  */
        let _t0 = this.sm.get().lastKey()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_2: Object = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }
}
