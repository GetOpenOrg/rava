#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedMap",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedMap<K, V> {
    #[cfg_attr(any(), java_field(name = "m", descriptor = "Ljava/util/Map;", access = "private final"))]
    pub m: Field<Object>,
    #[cfg_attr(any(), java_field(name = "keyType", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub keyType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "valueType", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub valueType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "entrySet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub entrySet: Field<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> Collections_CheckedMap<K, V> {
    // java: typeCheck(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn typeCheck__obj_obj(&self, key: Object, value: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.typeCheck")
    }

    // java: typeCheck(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;
    pub fn typeCheck__bifunc(&self, func: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.typeCheck")
    }

    // java: badKeyMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badKeyMsg(&self, key: Object) -> Result<String> {
        todo!("abstract java/util/Collections$CheckedMap.badKeyMsg")
    }

    // java: badValueMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badValueMsg(&self, value: Object) -> Result<String> {
        todo!("abstract java/util/Collections$CheckedMap.badValueMsg")
    }

    // java: <init>(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)V
    pub fn new(&self, m: Object, keyType: Object, valueType: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.<init>")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$CheckedMap.size")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.isEmpty")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.containsKey")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, v: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.containsValue")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.get")
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.remove")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.clear")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.keySet")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.values")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/util/Collections$CheckedMap.hashCode")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/util/Collections$CheckedMap.toString")
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.put")
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, t: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.putAll")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.entrySet")
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.forEach")
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        todo!("abstract java/util/Collections$CheckedMap.replaceAll")
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.putIfAbsent")
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.remove")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        todo!("abstract java/util/Collections$CheckedMap.replace")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.replace")
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.computeIfAbsent")
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.computeIfPresent")
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.compute")
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        todo!("abstract java/util/Collections$CheckedMap.merge")
    }
}
