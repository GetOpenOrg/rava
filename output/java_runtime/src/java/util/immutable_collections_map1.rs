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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ImmutableCollections_Map1<K, V>> for ImmutableCollections_AbstractImmutableMap<K, V> {
    fn from(v: ImmutableCollections_Map1<K, V>) -> ImmutableCollections_AbstractImmutableMap<K, V> { v.__into_super() }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ImmutableCollections_Map1<K, V>> for AbstractMap<K, V> {
    fn from(v: ImmutableCollections_Map1<K, V>) -> AbstractMap<K, V> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/ImmutableCollections$Map1"]
    #[super_class       = "java/util/ImmutableCollections$AbstractImmutableMap"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/ImmutableCollections$AbstractImmutableMap<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ImmutableCollections.java"]
    #[inner_classes     = "java/util/ImmutableCollections$AbstractImmutableMap:java/util/ImmutableCollections:AbstractImmutableMap:1032;java/util/ImmutableCollections$Map1:java/util/ImmutableCollections:Map1:24;java/util/Map$Entry:java/util/Map:Entry:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ImmutableCollections_AbstractImmutableMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/AbstractMap;java/util/ImmutableCollections$AbstractImmutableMap;java/util/ImmutableCollections$Map1;java/util/Map"]
    #[has_hash_code_method = true]

    pub struct ImmutableCollections_Map1<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "k0", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false, generic_signature = "TK;"))]
        pub k0: K,
        #[cfg_attr(any(), java_field(name = "v0", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false, generic_signature = "TV;"))]
        pub v0: V,
    }

    impl<K, V> ImmutableCollections_Map1<K, V> {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;TV;)V")]
        pub fn new(mut k0: K, mut v0: V) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ImmutableCollections_AbstractImmutableMap::new()?);
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&k0))?;
            this.__set_k0(Clone::clone(&_t0));
            let _t1: Object = Objects::requireNonNull_obj(Clone::clone(&v0))?;
            this.__set_v0(Clone::clone(&_t1));
            Ok(this)
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$Map1.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, o: Object) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$Map1.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$Map1.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$Map1.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/ImmutableCollections$Map1.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$Map1.isEmpty:()Z")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, in_: Object) -> Result<()> {
            panic!("stub: java/util/ImmutableCollections$Map1.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$Map1.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }
    }
}
