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
use crate::jdk::internal::loader::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::loader::AbstractClassLoaderValue;

impl<K: Clone + Default + 'static> From<AbstractClassLoaderValue_Sub<K>> for AbstractClassLoaderValue<K, Object> {
    fn from(v: AbstractClassLoaderValue_Sub<K>) -> AbstractClassLoaderValue<K, Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/loader/AbstractClassLoaderValue$Sub"]
    #[super_class       = "jdk/internal/loader/AbstractClassLoaderValue"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;>Ljdk/internal/loader/AbstractClassLoaderValue<Ljdk/internal/loader/AbstractClassLoaderValue<TCLV;TV;>.Sub<TK;>;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractClassLoaderValue.java"]
    #[inner_classes     = "jdk/internal/loader/AbstractClassLoaderValue$Sub:jdk/internal/loader/AbstractClassLoaderValue:Sub:17"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractClassLoaderValue<K, Object>"]
    #[all_supertypes    = "java/lang/Object;jdk/internal/loader/AbstractClassLoaderValue;jdk/internal/loader/AbstractClassLoaderValue$Sub"]
    #[has_hash_code_method = true]

    pub struct AbstractClassLoaderValue_Sub<K: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false, generic_signature = "TK;"))]
        pub key: K,
        #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljdk/internal/loader/AbstractClassLoaderValue;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub this_0: AbstractClassLoaderValue<Object, Object>,
    }

    impl<K> AbstractClassLoaderValue_Sub<K> {
        #[java_method(name = "<init>", descriptor = "(Ljdk/internal/loader/AbstractClassLoaderValue;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TK;)V", method_parameters = ":32784;:0")]
        pub fn new(this_0: AbstractClassLoaderValue<Object, Object>, key: Object) -> Result<Self> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue$Sub.<init>:(Ljdk/internal/loader/AbstractClassLoaderValue;Ljava/lang/Object;)V")
        }

        #[java_method(name = "parent", descriptor = "()Ljdk/internal/loader/AbstractClassLoaderValue;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljdk/internal/loader/AbstractClassLoaderValue<TCLV;TV;>;")]
        pub fn parent(&self) -> Result<AbstractClassLoaderValue<Object, Object>> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue$Sub.parent:()Ljdk/internal/loader/AbstractClassLoaderValue;")
        }

        #[java_method(name = "key", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TK;")]
        pub fn key(&self) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue$Sub.key:()Ljava/lang/Object;")
        }

        #[java_method(name = "isEqualOrDescendantOf", descriptor = "(Ljdk/internal/loader/AbstractClassLoaderValue;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljdk/internal/loader/AbstractClassLoaderValue<*TV;>;)Z")]
        pub fn isEqualOrDescendantOf(&self, clv: AbstractClassLoaderValue<Object, Object>) -> Result<bool> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue$Sub.isEqualOrDescendantOf:(Ljdk/internal/loader/AbstractClassLoaderValue;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue$Sub.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }
    }
}
