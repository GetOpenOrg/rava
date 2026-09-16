#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ConcurrentHashMap$Node"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Map$Entry"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Map$Entry<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConcurrentHashMap.java"]
    #[inner_classes     = "java/util/concurrent/ConcurrentHashMap$Node:java/util/concurrent/ConcurrentHashMap:Node:8;java/util/Map$Entry:java/util/Map:Entry:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Map$Entry;java/util/concurrent/ConcurrentHashMap$Node"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ConcurrentHashMap_Node<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub hash: i32,
        #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final", is_static = false, generic_signature = "TK;"))]
        pub key: K,
        #[cfg_attr(any(), java_field(name = "val", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "TV;"))]
        pub val: V,
        #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/util/concurrent/ConcurrentHashMap$Node;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;"))]
        pub next: ConcurrentHashMap_Node<K, V>,
    }

    impl<K, V> ConcurrentHashMap_Node<K, V> {
        #[java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;)V")]
        // java: <init>(ILjava/lang/Object;Ljava/lang/Object;)V
        pub fn new_i_obj_obj(mut hash: i32, mut key: K, mut val: V) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_hash(hash);
            this.__set_key(Clone::clone(&key));
            this.__set_val(Clone::clone(&val));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;)V")]
        // java: <init>(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/concurrent/ConcurrentHashMap$Node;)V
        pub fn new_i_obj_obj_concur(mut hash: i32, mut key: K, mut val: V, mut next: ConcurrentHashMap_Node<K, V>) -> Result<Self> {
            let mut this = Self::default();
            this = ConcurrentHashMap_Node::new_i_obj_obj(hash, Clone::clone(&key), Clone::clone(&val))?;
            this.__set_next(Clone::clone(&next));
            Ok(this)
        }

        #[java_method(name = "getKey", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;")]
        pub fn getKey(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$Node.getKey:()Ljava/lang/Object;")
        }

        #[java_method(name = "getValue", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn getValue(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$Node.getValue:()Ljava/lang/Object;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "setValue", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)TV;")]
        pub fn setValue(&self, value: V) -> Result<Object> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$Node.setValue:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/concurrent/ConcurrentHashMap$Node.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "find", descriptor = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/lang/Object;)Ljava/util/concurrent/ConcurrentHashMap$Node<TK;TV;>;")]
        pub fn find(&self, mut h: i32, mut k: Object) -> Result<ConcurrentHashMap_Node<K, V>> {
            let this = self;
            let mut e = this.clone();
            loop {
                let mut ek = e.__get_key();
                let _vdispatch0: bool = if let Some(__f) = k.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&ek))? } else { Default::default() };
                if _vdispatch0 {
                    return Ok(Default::default());
                }
                let mut e = e.__get_next();
                if _is_jnull(&e.__get_next()) { break; }
            }
            Ok(Default::default())
        }
    }
}
