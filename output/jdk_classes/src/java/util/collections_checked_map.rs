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
        panic!("stub: java/util/Collections$CheckedMap.typeCheck:(Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    // java: typeCheck(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;
    pub fn typeCheck__bifunc(&self, func: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.typeCheck:(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;")
    }

    // java: badKeyMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badKeyMsg(&self, key: Object) -> Result<String> {
        panic!("stub: java/util/Collections$CheckedMap.badKeyMsg:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: badValueMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badValueMsg(&self, value: Object) -> Result<String> {
        panic!("stub: java/util/Collections$CheckedMap.badValueMsg:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: <init>(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)V
    pub fn new(&self, m: Object, keyType: Object, valueType: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedMap.<init>:(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)V")
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$CheckedMap.size:()I")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.isEmpty:()Z")
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.containsKey:(Ljava/lang/Object;)Z")
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, v: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.containsValue:(Ljava/lang/Object;)Z")
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.get:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedMap.clear:()V")
    }

    // java: keySet()Ljava/util/Set;
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.keySet:()Ljava/util/Set;")
    }

    // java: values()Ljava/util/Collection;
    pub fn values(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.values:()Ljava/util/Collection;")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collections$CheckedMap.hashCode:()I")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/Collections$CheckedMap.toString:()Ljava/lang/String;")
    }

    // java: put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: putAll(Ljava/util/Map;)V
    pub fn putAll(&self, t: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedMap.putAll:(Ljava/util/Map;)V")
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.entrySet:()Ljava/util/Set;")
    }

    // java: forEach(Ljava/util/function/BiConsumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedMap.forEach:(Ljava/util/function/BiConsumer;)V")
    }

    // java: replaceAll(Ljava/util/function/BiFunction;)V
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        panic!("stub: java/util/Collections$CheckedMap.replaceAll:(Ljava/util/function/BiFunction;)V")
    }

    // java: putIfAbsent(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        panic!("stub: java/util/Collections$CheckedMap.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: computeIfAbsent(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
    }

    // java: computeIfPresent(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    // java: compute(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    // java: merge(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Collections$CheckedMap.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }
}
