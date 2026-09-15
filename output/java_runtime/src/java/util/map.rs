#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/Map",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Map.java",
    inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;java/util/ImmutableCollections$MapN:java/util/ImmutableCollections:MapN:24;java/util/ImmutableCollections$Map1:java/util/ImmutableCollections:Map1:24;java/util/ImmutableCollections$AbstractImmutableMap:java/util/ImmutableCollections:AbstractImmutableMap:1032",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Map<K: Clone + Default + 'static, V: Clone + Default + 'static>(std::marker::PhantomData<(K, V,)>);

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> Map<K, V> {
    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Map.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Map.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn containsKey(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Map.containsKey:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn containsValue(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Map.containsValue:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn get(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/Map.get:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn put(&self, arg0: Object, arg1: Object) -> Result<Object> {
        panic!("stub: java/util/Map.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn remove_obj(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/Map.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V"))]
    pub fn putAll(&self, arg0: Map<Object, Object>) -> Result<()> {
        panic!("stub: java/util/Map.putAll:(Ljava/util/Map;)V")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Map.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;"))]
    pub fn keySet(&self) -> Result<Set<Object>> {
        panic!("stub: java/util/Map.keySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;"))]
    pub fn values(&self) -> Result<Object> {
        panic!("stub: java/util/Map.values:()Ljava/util/Collection;")
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
    pub fn entrySet(&self) -> Result<Set<Object>> {
        panic!("stub: java/util/Map.entrySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Map.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Map.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;TV;)TV;"))]
    pub fn getOrDefault(&self, key: Object, defaultValue: Object) -> Result<Object> {
        panic!("stub: java/util/Map.getOrDefault:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TK;-TV;>;)V"))]
    pub fn forEach(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/Map.forEach:(Ljava/util/function/BiConsumer;)V")
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)V"))]
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        panic!("stub: java/util/Map.replaceAll:(Ljava/util/function/BiFunction;)V")
    }

    #[cfg_attr(any(), java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Map.putIfAbsent:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove_obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        panic!("stub: java/util/Map.remove:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;TV;)Z"))]
    pub fn replace_obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        panic!("stub: java/util/Map.replace:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn replace_obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Map.replace:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/Function<-TK;+TV;>;)TV;"))]
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Map.computeIfAbsent:(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;"))]
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Map.computeIfPresent:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;Ljava/util/function/BiFunction<-TK;-TV;+TV;>;)TV;"))]
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Map.compute:(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/function/BiFunction<-TV;-TV;+TV;>;)TV;"))]
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        panic!("stub: java/util/Map.merge:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>()Ljava/util/Map<TK;TV;>;"))]
    pub fn of() -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:()Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj(k1: Object, v1: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object, k6: Object, v6: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object, k6: Object, v6: Object, k7: Object, v7: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object, k6: Object, v6: Object, k7: Object, v7: Object, k8: Object, v8: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object, k6: Object, v6: Object, k7: Object, v7: Object, k8: Object, v8: Object, k9: Object, v9: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;TK;TV;)Ljava/util/Map<TK;TV;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(k1: Object, v1: Object, k2: Object, v2: Object, k3: Object, v3: Object, k4: Object, v4: Object, k5: Object, v5: Object, k6: Object, v6: Object, k7: Object, v7: Object, k8: Object, v8: Object, k9: Object, v9: Object, k10: Object, v10: Object) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "ofEntries", descriptor = "([Ljava/util/Map$Entry;)Ljava/util/Map;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>([Ljava/util/Map$Entry<+TK;+TV;>;)Ljava/util/Map<TK;TV;>;"))]
    pub fn ofEntries(entries: Rc<RefCell<Vec<Object>>>) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.ofEntries:([Ljava/util/Map$Entry;)Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "entry", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;)Ljava/util/Map$Entry<TK;TV;>;"))]
    pub fn entry(k: Object, v: Object) -> Result<Object> {
        panic!("stub: java/util/Map.entry:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map$Entry;")
    }

    #[cfg_attr(any(), java_method(name = "copyOf", descriptor = "(Ljava/util/Map;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/Map<+TK;+TV;>;)Ljava/util/Map<TK;TV;>;"))]
    pub fn copyOf(map: Map<Object, Object>) -> Result<Map<Object, Object>> {
        panic!("stub: java/util/Map.copyOf:(Ljava/util/Map;)Ljava/util/Map;")
    }
}
