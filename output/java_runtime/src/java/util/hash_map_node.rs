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
    binary_name       = "java/util/HashMap$Node",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Map$Entry",
    access            = "package",
    modifiers         = "",
    generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Map$Entry<TK;TV;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "HashMap.java",
    inner_classes     = "java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/Map$Entry:java/util/Map:Entry:1545",
    all_supertypes    = "java/lang/Object;java/util/HashMap$Node;java/util/Map$Entry",
)]
#[derive(Clone, Default, PartialEq)]
pub struct HashMap_Node<K: Clone + Default + 'static, V: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
    pub hash: JField<i32>,
    #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final", is_static = false, generic_signature = "TK;"))]
    pub key: JField<K>,
    #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TV;"))]
    pub value: JField<V>,
    #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/util/HashMap$Node;", is_static = false, generic_signature = "Ljava/util/HashMap$Node<TK;TV;>;"))]
    pub next: JField<HashMap_Node<K, V>>,
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> HashMap_Node<K, V> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)V"))]
    pub fn new(mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<Object, Object>) -> Result<Self> {
        let mut this = Self { hash: JField::new(0), key: JField::new_uninit(), value: JField::new_uninit(), next: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.hash.set(hash);
        this.key.set(Clone::clone(&key));
        this.value.set(Clone::clone(&value));
        this.next.set(Clone::clone(&next));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getKey", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;"))]
    pub fn getKey(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap$Node.getKey:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "getValue", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;"))]
    pub fn getValue(&self) -> Result<Object> {
        panic!("stub: java/util/HashMap$Node.getValue:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/HashMap$Node.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/HashMap$Node.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "setValue", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)TV;"))]
    pub fn setValue(&self, newValue: Object) -> Result<Object> {
        panic!("stub: java/util/HashMap$Node.setValue:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/HashMap$Node.equals:(Ljava/lang/Object;)Z")
    }
}
