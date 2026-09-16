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
    #[binary_name       = "java/util/TreeMap$Entry"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Map$Entry"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Map$Entry<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TreeMap.java"]
    #[inner_classes     = "java/util/TreeMap$Entry:java/util/TreeMap:Entry:24;java/util/Map$Entry:java/util/Map:Entry:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Map$Entry;java/util/TreeMap$Entry"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TreeMap_Entry<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TK;"))]
        pub key: K,
        #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TV;"))]
        pub value: V,
        #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/TreeMap$Entry;", is_static = false, generic_signature = "Ljava/util/TreeMap$Entry<TK;TV;>;"))]
        pub left: TreeMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/TreeMap$Entry;", is_static = false, generic_signature = "Ljava/util/TreeMap$Entry<TK;TV;>;"))]
        pub right: TreeMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/TreeMap$Entry;", is_static = false, generic_signature = "Ljava/util/TreeMap$Entry<TK;TV;>;"))]
        pub parent: TreeMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "color", descriptor = "Z", is_static = false))]
        pub color: bool,
    }

    impl<K, V> TreeMap_Entry<K, V> {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;Ljava/util/TreeMap$Entry<TK;TV;>;)V")]
        pub fn new(mut key: K, mut value: V, mut parent: TreeMap_Entry<K, V>) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_color((1i32 != 0i32));
            this.__set_key(Clone::clone(&key));
            this.__set_value(Clone::clone(&value));
            this.__set_parent(Clone::clone(&parent));
            Ok(this)
        }

        #[java_method(name = "getKey", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;")]
        pub fn getKey(&self) -> Result<K> {
            let this = self;
            Ok(this.__get_key())
        }

        #[java_method(name = "getValue", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TV;")]
        pub fn getValue(&self) -> Result<V> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "setValue", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TV;)TV;")]
        pub fn setValue(&self, value: V) -> Result<Object> {
            panic!("stub: java/util/TreeMap$Entry.setValue:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/TreeMap$Entry.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
