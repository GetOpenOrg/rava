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

impl From<ClassScope> for AbstractScope<Object> {
    fn from(v: ClassScope) -> AbstractScope<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/reflect/generics/scope/ClassScope"]
    #[super_class       = "sun/reflect/generics/scope/AbstractScope"]
    #[interfaces        = "sun/reflect/generics/scope/Scope"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Lsun/reflect/generics/scope/AbstractScope<Ljava/lang/Class<*>;>;Lsun/reflect/generics/scope/Scope;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ClassScope.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractScope<Object>"]
    #[superclass_fields(recvr: Object, enclosingScope: Object)]
    #[all_supertypes    = "java/lang/Object;sun/reflect/generics/scope/AbstractScope;sun/reflect/generics/scope/ClassScope;sun/reflect/generics/scope/Scope"]

    pub struct ClassScope;

    impl ClassScope {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn new(c: Object) -> Result<Self> {
            panic!("stub: sun/reflect/generics/scope/ClassScope.<init>:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "computeEnclosingScope", descriptor = "()Lsun/reflect/generics/scope/Scope;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeEnclosingScope(&self) -> Result<Object> {
            panic!("stub: sun/reflect/generics/scope/ClassScope.computeEnclosingScope:()Lsun/reflect/generics/scope/Scope;")
        }

        #[java_method(name = "make", descriptor = "(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Lsun/reflect/generics/scope/ClassScope;")]
        pub fn make(c: Object) -> Result<ClassScope> {
            panic!("stub: sun/reflect/generics/scope/ClassScope.make:(Ljava/lang/Class;)Lsun/reflect/generics/scope/ClassScope;")
        }
    }
}
