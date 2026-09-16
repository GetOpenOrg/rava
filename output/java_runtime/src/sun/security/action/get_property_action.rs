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
    #[binary_name       = "sun/security/action/GetPropertyAction"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/security/PrivilegedAction"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/security/PrivilegedAction<Ljava/lang/String;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "GetPropertyAction.java"]
    #[inner_classes     = "sun/security/action/GetPropertyAction$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/security/PrivilegedAction;sun/security/action/GetPropertyAction"]

    pub struct GetPropertyAction {
        #[cfg_attr(any(), java_field(name = "theProp", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub theProp: String,
        #[cfg_attr(any(), java_field(name = "defaultVal", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub defaultVal: String,
    }

    impl GetPropertyAction {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(theProp: String) -> Result<Self> {
            panic!("stub: sun/security/action/GetPropertyAction.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_str(theProp: String, defaultVal: String) -> Result<Self> {
            panic!("stub: sun/security/action/GetPropertyAction.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "run", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<String> {
            panic!("stub: sun/security/action/GetPropertyAction.run:()Ljava/lang/String;")
        }

        #[java_method(name = "privilegedGetProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privilegedGetProperty_str(theProp: String) -> Result<String> {
            panic!("stub: sun/security/action/GetPropertyAction.privilegedGetProperty:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "privilegedGetProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privilegedGetProperty_str_str(theProp: String, defaultVal: String) -> Result<String> {
            panic!("stub: sun/security/action/GetPropertyAction.privilegedGetProperty:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "privilegedGetProperties", descriptor = "()Ljava/util/Properties;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privilegedGetProperties() -> Result<Properties> {
            panic!("stub: sun/security/action/GetPropertyAction.privilegedGetProperties:()Ljava/util/Properties;")
        }

        #[java_method(name = "privilegedGetTimeoutProp", descriptor = "(Ljava/lang/String;ILsun/security/util/Debug;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privilegedGetTimeoutProp(prop: String, def: i32, dbg: Object) -> Result<i32> {
            panic!("stub: sun/security/action/GetPropertyAction.privilegedGetTimeoutProp:(Ljava/lang/String;ILsun/security/util/Debug;)I")
        }

        #[java_method(name = "privilegedGetBooleanProp", descriptor = "(Ljava/lang/String;ZLsun/security/util/Debug;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privilegedGetBooleanProp(prop: String, def: bool, dbg: Object) -> Result<bool> {
            panic!("stub: sun/security/action/GetPropertyAction.privilegedGetBooleanProp:(Ljava/lang/String;ZLsun/security/util/Debug;)Z")
        }
    }
}
