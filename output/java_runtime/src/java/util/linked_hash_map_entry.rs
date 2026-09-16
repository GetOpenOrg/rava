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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<LinkedHashMap_Entry<K, V>> for HashMap_Node<K, V> {
    fn from(v: LinkedHashMap_Entry<K, V>) -> HashMap_Node<K, V> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/LinkedHashMap$Entry"]
    #[super_class       = "java/util/HashMap$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/HashMap$Node<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LinkedHashMap.java"]
    #[inner_classes     = "java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/LinkedHashMap$Entry:java/util/LinkedHashMap:Entry:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "HashMap_Node<K, V>"]
    #[superclass_fields(hash: i32, key: K, value: V, next: HashMap_Node<K, V>)]
    #[all_supertypes    = "java/lang/Object;java/util/HashMap$Node;java/util/LinkedHashMap$Entry;java/util/Map$Entry"]

    pub struct LinkedHashMap_Entry<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "before", descriptor = "Ljava/util/LinkedHashMap$Entry;", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
        pub before: LinkedHashMap_Entry<K, V>,
        #[cfg_attr(any(), java_field(name = "after", descriptor = "Ljava/util/LinkedHashMap$Entry;", is_static = false, generic_signature = "Ljava/util/LinkedHashMap$Entry<TK;TV;>;"))]
        pub after: LinkedHashMap_Entry<K, V>,
    }

    impl<K, V> LinkedHashMap_Entry<K, V> {
        #[java_method(name = "<init>", descriptor = "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITK;TV;Ljava/util/HashMap$Node<TK;TV;>;)V")]
        pub fn new(mut hash: i32, mut key: K, mut value: V, mut next: HashMap_Node<K, V>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(HashMap_Node::new(hash, Clone::clone(&key), Clone::clone(&value), Clone::clone(&next))?);
            Ok(this)
        }
    }
}
