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
use crate::jdk::internal::reflect::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::reflect::ReflectionFactory;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/security/PrivilegedAction"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/security/PrivilegedAction<Ljdk/internal/reflect/ReflectionFactory;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReflectionFactory.java"]
    #[inner_classes     = "jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/security/PrivilegedAction;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction"]

    pub struct ReflectionFactory_GetReflectionFactoryAction;

    impl ReflectionFactory_GetReflectionFactoryAction {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction.<init>:()V")
        }

        #[java_method(name = "run", descriptor = "()Ljdk/internal/reflect/ReflectionFactory;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<ReflectionFactory> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction.run:()Ljdk/internal/reflect/ReflectionFactory;")
        }
    }
}
