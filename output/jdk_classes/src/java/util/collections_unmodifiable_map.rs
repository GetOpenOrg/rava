#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$UnmodifiableMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_UnmodifiableMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_UnmodifiableMap<K, V> {
    // java: <init>(Ljava/util/Map;)V
    pub fn new(&self, m: Object) -> Result<()> {
        todo!("abstract java/util/Collections$UnmodifiableMap.<init>")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$UnmodifiableMap.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.isEmpty")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.containsKey")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, val: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.containsValue")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.get")
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.put")
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.remove")
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, m: Object) -> Result<()> {
        todo!("abstract java/util/Collections$UnmodifiableMap.putAll")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$UnmodifiableMap.clear")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.keySet")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.entrySet")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.values")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$UnmodifiableMap.hashCode")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$UnmodifiableMap.toString")
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, k: Object, defaultValue: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.getOrDefault")
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$UnmodifiableMap.forEach")
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        todo!("abstract java/util/Collections$UnmodifiableMap.replaceAll")
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.putIfAbsent")
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.remove")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$UnmodifiableMap.replace")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.replace")
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.computeIfAbsent")
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.computeIfPresent")
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.compute")
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$UnmodifiableMap.merge")
    }
}
