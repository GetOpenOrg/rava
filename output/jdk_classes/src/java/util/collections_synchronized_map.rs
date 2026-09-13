#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SynchronizedMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SynchronizedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "private final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "mutex", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub mutex: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SynchronizedMap<K, V> {
    // java: <init>(Ljava/util/Map;)V
    pub fn new__map(&self, m: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.<init>:(Ljava/util/Map;)V")
    }

    // java: <init>(Ljava/util/Map;Ljava/lang/Object;)V
    pub fn new__map_obj(&self, m: Object, mutex: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.<init>:(Ljava/util/Map;Ljava/lang/Object;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedMap.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.isEmpty:()Z")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.containsKey:(Ljava/lang/Object;)Z")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.containsValue:(Ljava/lang/Object;)Z")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.get:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, map: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.putAll:(Ljava/util/Map;)V")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.clear:()V")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.keySet:()Ljava/util/Set;")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.entrySet:()Ljava/util/Set;")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.values:()Ljava/util/Collection;")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$SynchronizedMap.hashCode:()I")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$SynchronizedMap.toString:()Ljava/lang/String;")
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, k: Object, defaultValue: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.forEach:(Ljava/util/function/BiConsumer;)V")
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.replaceAll:(Ljava/util/function/BiFunction;)V")
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$SynchronizedMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$SynchronizedMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/Collections$SynchronizedMap.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }
}
