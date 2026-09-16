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
    #[binary_name       = "java/lang/Class$3"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/security/PrivilegedAction"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/security/PrivilegedAction<Ljava/lang/Object;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Class.java"]
    #[inner_classes     = "java/lang/Class$3:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Class$3;java/lang/Object;java/security/PrivilegedAction"]

    pub struct Class_3 {
        #[cfg_attr(any(), java_field(name = "val$values", descriptor = "Ljava/lang/reflect/Method;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_values: Method,
    }

    impl Class_3 {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Method;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32784;:4112")]
        pub fn new(mut this_0: Object, mut arg_1: Method) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_values(Clone::clone(&arg_1));
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "run", descriptor = "()Ljava/lang/Void;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<Object> {
            panic!("stub: java/lang/Class$3.run:()Ljava/lang/Void;")
        }
    }
}
