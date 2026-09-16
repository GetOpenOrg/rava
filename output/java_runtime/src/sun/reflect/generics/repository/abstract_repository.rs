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
    #[binary_name       = "sun/reflect/generics/repository/AbstractRepository"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T::Lsun/reflect/generics/tree/Tree;>Ljava/lang/Object;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractRepository.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/reflect/generics/repository/AbstractRepository"]

    pub struct AbstractRepository<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "factory", descriptor = "Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "final", is_static = false))]
        pub factory: Object,
        #[cfg_attr(any(), java_field(name = "tree", descriptor = "Lsun/reflect/generics/tree/Tree;", access = "private", modifiers = "final", is_static = false, generic_signature = "TT;"))]
        pub tree: T,
    }

    impl<T> AbstractRepository<T> {
        #[java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFactory(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/AbstractRepository.getFactory:()Lsun/reflect/generics/factory/GenericsFactory;")
        }

        #[java_method(name = "getTree", descriptor = "()Lsun/reflect/generics/tree/Tree;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;")]
        pub fn getTree(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/AbstractRepository.getTree:()Lsun/reflect/generics/tree/Tree;")
        }

        #[java_method(name = "getReifier", descriptor = "()Lsun/reflect/generics/visitor/Reifier;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getReifier(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/AbstractRepository.getReifier:()Lsun/reflect/generics/visitor/Reifier;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(rawSig: String, f: Object) -> Result<Self> {
            panic!("stub: sun/reflect/generics/repository/AbstractRepository.<init>:(Ljava/lang/String;Lsun/reflect/generics/factory/GenericsFactory;)V")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/lang/String;)TT;")]
        pub fn parse(&self, arg0: String) -> Result<Object> {
            panic!("stub: sun/reflect/generics/repository/AbstractRepository.parse:(Ljava/lang/String;)Lsun/reflect/generics/tree/Tree;")
        }
    }
}
