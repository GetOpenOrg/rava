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
    pub fn new(m: Object, keyType: Object, valueType: Object) -> Result<Self> {
        let this = Self { sm: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/Collections$CheckedMap.<init>:(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)V */
        this.sm.set(m);
        Ok(this)
    }

    // java: comparator()Ljava/util/Comparator;
    pub fn comparator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.sm.get().comparator()?;
        Ok(_t0)
    }

    // java: firstKey()Ljava/lang/Object;
    pub fn firstKey(&self) -> Result<K> {
        let this = self;
        let _t0 = this.sm.get().firstKey()?;
        Ok(_t0)
    }

    // java: lastKey()Ljava/lang/Object;
    pub fn lastKey(&self) -> Result<K> {
        let this = self;
        let _t0 = this.sm.get().lastKey()?;
        Ok(_t0)
    }

    // java: subMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn subMap(&self, fromKey: K, toKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.sm.get().subMap(fromKey, toKey)?;
        let _t1: Object = Collections::checkedSortedMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: headMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn headMap(&self, toKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.sm.get().headMap(toKey)?;
        let _t1: Object = Collections::checkedSortedMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }

    // java: tailMap(Ljava/lang/Object;)Ljava/util/SortedMap;
    pub fn tailMap(&self, fromKey: K) -> Result<Object> {
        let this = self;
        let _t0 = this.sm.get().tailMap(fromKey)?;
        let _t1: Object = Collections::checkedSortedMap(_t0, this.keyType.get(), this.valueType.get())?;
        Ok(_t1)
    }
}
