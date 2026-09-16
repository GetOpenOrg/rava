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
    #[binary_name       = "sun/util/locale/LocaleObjectCache"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleObjectCache.java"]
    #[inner_classes     = "sun/util/locale/LocaleObjectCache$CacheEntry:sun/util/locale/LocaleObjectCache:CacheEntry:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/LocaleObjectCache"]

    pub struct LocaleObjectCache<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentMap<TK;Lsun/util/locale/LocaleObjectCache$CacheEntry<TK;TV;>;>;"))]
        pub map: Object,
        #[cfg_attr(any(), java_field(name = "queue", descriptor = "Ljava/lang/ref/ReferenceQueue;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/ref/ReferenceQueue<TV;>;"))]
        pub queue: ReferenceQueue<V>,
    }

    impl<K, V> LocaleObjectCache<K, V> {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/LocaleObjectCache.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(IFI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f_i(initialCapacity: i32, loadFactor: f32, concurrencyLevel: i32) -> Result<Self> {
            panic!("stub: sun/util/locale/LocaleObjectCache.<init>:(IFI)V")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TV;")]
        pub fn get(&self, key: K) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleObjectCache.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)TV;")]
        pub fn put(&self, key: K, value: V) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleObjectCache.put:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "cleanStaleEntries", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cleanStaleEntries(&self) -> Result<()> {
            panic!("stub: sun/util/locale/LocaleObjectCache.cleanStaleEntries:()V")
        }

        #[java_method(name = "createObject", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TK;)TV;")]
        pub fn createObject(&self, arg0: K) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleObjectCache.createObject:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "normalizeKey", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)TK;")]
        pub fn normalizeKey(&self, key: K) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleObjectCache.normalizeKey:(Ljava/lang/Object;)Ljava/lang/Object;")
        }
    }
}
