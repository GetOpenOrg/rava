#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SingletonMap",
    super_class = "java/util/AbstractMap",
    interfaces  = "java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SingletonMap<K, V> {
    #[cfg_attr(any(), java_field(name = "k", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub k: Field<Object>,
    #[cfg_attr(any(), java_field(name = "v", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub v: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub keySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "private"))]
    pub values: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_SingletonMap<K, V> {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new(&self, key: Object, value: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SingletonMap.<init>")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SingletonMap.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$SingletonMap.isEmpty")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SingletonMap.containsKey")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SingletonMap.containsValue")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.get")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.keySet")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.entrySet")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.values")
    }

    // java: getOrDefault(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn getOrDefault(&self, key: Object, defaultValue: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.getOrDefault")
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SingletonMap.forEach")
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        todo!("abstract java/util/Collections$SingletonMap.replaceAll")
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.putIfAbsent")
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove(&self, key: Object, value: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SingletonMap.remove")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$SingletonMap.replace")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.replace")
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.computeIfAbsent")
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.computeIfPresent")
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.compute")
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$SingletonMap.merge")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$SingletonMap.hashCode")
    }
}
