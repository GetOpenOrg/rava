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
use crate::jdk::internal::loader::AbstractClassLoaderValue_Sub;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/loader/AbstractClassLoaderValue"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<CLV:Ljdk/internal/loader/AbstractClassLoaderValue<TCLV;TV;>;V:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractClassLoaderValue.java"]
    #[inner_classes     = "jdk/internal/loader/AbstractClassLoaderValue$Sub:jdk/internal/loader/AbstractClassLoaderValue:Sub:17;jdk/internal/loader/AbstractClassLoaderValue$Memoizer:jdk/internal/loader/AbstractClassLoaderValue:Memoizer:26;jdk/internal/loader/AbstractClassLoaderValue$Memoizer$RecursiveInvocationException:jdk/internal/loader/AbstractClassLoaderValue$Memoizer:RecursiveInvocationException:8;java/util/concurrent/ConcurrentHashMap$KeySetView:java/util/concurrent/ConcurrentHashMap:KeySetView:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/loader/AbstractClassLoaderValue"]

    pub struct AbstractClassLoaderValue<CLV: Clone + Default + 'static, V: Clone + Default + 'static>;

    impl<CLV, V> AbstractClassLoaderValue<CLV, V> {
        #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JLA:Ljdk/internal/access/JavaLangAccess;
        pub fn JLA() -> Object {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.JLA:Ljdk/internal/access/JavaLangAccess;")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.<init>:()V")
        }

        #[java_method(name = "key", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn key(&self) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.key:()Ljava/lang/Object;")
        }

        #[java_method(name = "sub", descriptor = "(Ljava/lang/Object;)Ljdk/internal/loader/AbstractClassLoaderValue$Sub;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;>(TK;)Ljdk/internal/loader/AbstractClassLoaderValue<TCLV;TV;>.Sub<TK;>;")]
        pub fn sub(&self, key: Object) -> Result<AbstractClassLoaderValue_Sub<Object>> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.sub:(Ljava/lang/Object;)Ljdk/internal/loader/AbstractClassLoaderValue$Sub;")
        }

        #[java_method(name = "isEqualOrDescendantOf", descriptor = "(Ljdk/internal/loader/AbstractClassLoaderValue;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljdk/internal/loader/AbstractClassLoaderValue<*TV;>;)Z")]
        pub fn isEqualOrDescendantOf(&self, arg0: AbstractClassLoaderValue<Object, V>) -> Result<bool> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.isEqualOrDescendantOf:(Ljdk/internal/loader/AbstractClassLoaderValue;)Z")
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/ClassLoader;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;)TV;")]
        pub fn get(&self, cl: ClassLoader) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.get:(Ljava/lang/ClassLoader;)Ljava/lang/Object;")
        }

        #[java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;TV;)TV;")]
        pub fn putIfAbsent(&self, cl: ClassLoader, v: V) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.putIfAbsent:(Ljava/lang/ClassLoader;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self, cl: ClassLoader, v: Object) -> Result<bool> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.remove:(Ljava/lang/ClassLoader;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/ClassLoader;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalStateException", generic_signature = "(Ljava/lang/ClassLoader;Ljava/util/function/BiFunction<-Ljava/lang/ClassLoader;-TCLV;+TV;>;)TV;")]
        pub fn computeIfAbsent(&self, cl: ClassLoader, mappingFunction: Object) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.computeIfAbsent:(Ljava/lang/ClassLoader;Ljava/util/function/BiFunction;)Ljava/lang/Object;")
        }

        #[java_method(name = "removeAll", descriptor = "(Ljava/lang/ClassLoader;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeAll(&self, cl: ClassLoader) -> Result<()> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.removeAll:(Ljava/lang/ClassLoader;)V")
        }

        #[java_method(name = "map", descriptor = "(Ljava/lang/ClassLoader;)Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<CLV:Ljdk/internal/loader/AbstractClassLoaderValue<TCLV;*>;>(Ljava/lang/ClassLoader;)Ljava/util/concurrent/ConcurrentHashMap<TCLV;Ljava/lang/Object;>;")]
        pub fn map(cl: ClassLoader) -> Result<ConcurrentHashMap<Object, Object>> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.map:(Ljava/lang/ClassLoader;)Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[java_method(name = "extractValue", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn extractValue(&self, memoizerOrValue: Object) -> Result<Object> {
            panic!("stub: jdk/internal/loader/AbstractClassLoaderValue.extractValue:(Ljava/lang/Object;)Ljava/lang/Object;")
        }
    }
}
