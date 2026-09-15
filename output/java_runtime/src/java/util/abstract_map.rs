#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/AbstractMap",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Map",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Map<TK;TV;>;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractMap.java",
    inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;java/util/AbstractMap$1:::0;java/util/AbstractMap$2:::0;java/util/AbstractMap$ViewCollection:java/util/AbstractMap:ViewCollection:1032;java/util/AbstractMap$SimpleImmutableEntry:java/util/AbstractMap:SimpleImmutableEntry:9;java/util/AbstractMap$SimpleEntry:java/util/AbstractMap:SimpleEntry:9;java/util/AbstractMap$2$1:::0;java/util/AbstractMap$1$1:::0",
    all_supertypes    = "java/lang/Object;java/util/AbstractMap;java/util/Map",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractMap<K: Clone + Default + 'static, V: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "keySet", descriptor = "Ljava/util/Set;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Set<TK;>;"))]
    pub keySet: JField<Object>,
    #[cfg_attr(any(), java_field(name = "values", descriptor = "Ljava/util/Collection;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/Collection<TV;>;"))]
    pub values: JField<Object>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> AbstractMap<K, V> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self { keySet: JField::new(Default::default()), values: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractMap.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/AbstractMap.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractMap.containsValue:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractMap.containsKey:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn get(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.get:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;"))]
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;"))]
    pub fn remove(&self, key: Object) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.remove:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<+TK;+TV;>;)V"))]
    pub fn putAll(&self, m: Object) -> Result<()> {
        panic!("stub: java/util/AbstractMap.putAll:(Ljava/util/Map;)V")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/AbstractMap.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<TK;>;"))]
    pub fn keySet(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.keySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Collection<TV;>;"))]
    pub fn values(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.values:()Ljava/util/Collection;")
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;"))]
    pub fn entrySet(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.entrySet:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractMap.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractMap.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/AbstractMap.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException"))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractMap.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "eq", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn eq(o1: Object, o2: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractMap.eq:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }
}
