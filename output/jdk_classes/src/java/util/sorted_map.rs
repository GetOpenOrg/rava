#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/SortedMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/SequencedMap",
    access      = "public abstract",
    source      = "SortedMap.java",
))]
pub struct SortedMap<K, V>;

impl<K: Clone + 'static, V: Clone + 'static> SortedMap<K, V> {
    #[cfg_attr(any(), java_native(name = "comparator", descriptor = "()Ljava/util/Comparator;", access = "public abstract"))]
    pub fn comparator(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.comparator")
    }

    #[cfg_attr(any(), java_native(name = "subMap", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public abstract"))]
    pub fn subMap(&self, arg0: Object, arg1: Object) -> Result<Object> {
        todo!("abstract java/util/SortedMap.subMap")
    }

    #[cfg_attr(any(), java_native(name = "headMap", descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public abstract"))]
    pub fn headMap(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/SortedMap.headMap")
    }

    #[cfg_attr(any(), java_native(name = "tailMap", descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", access = "public abstract"))]
    pub fn tailMap(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/SortedMap.tailMap")
    }

    #[cfg_attr(any(), java_native(name = "firstKey", descriptor = "()Ljava/lang/Object;", access = "public abstract"))]
    pub fn firstKey(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.firstKey")
    }

    #[cfg_attr(any(), java_native(name = "lastKey", descriptor = "()Ljava/lang/Object;", access = "public abstract"))]
    pub fn lastKey(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.lastKey")
    }

    #[cfg_attr(any(), java_native(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public abstract"))]
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.keySet")
    }

    #[cfg_attr(any(), java_native(name = "values", descriptor = "()Ljava/util/Collection;", access = "public abstract"))]
    pub fn values(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.values")
    }

    #[cfg_attr(any(), java_native(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public abstract"))]
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/SortedMap.entrySet")
    }

    #[cfg_attr(any(), java_method(name = "putFirst", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn putFirst(&self, k: K, v: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "putLast", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn putLast(&self, k: K, v: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "reversed", descriptor = "()Ljava/util/SortedMap;", access = "public"))]
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = ReverseOrderSortedMapView::of(this)?;
        Ok(_t0)
    }
}
