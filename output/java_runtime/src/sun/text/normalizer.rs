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
use crate::jdk::internal::icu::text::NormalizerBase;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/text/Normalizer"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Normalizer.java"]
    #[inner_classes     = "java/text/Normalizer$Form:java/text/Normalizer:Form:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/text/Normalizer"]

    pub struct Normalizer;

    impl Normalizer {
        #[cfg_attr(any(), java_field(name = "UNICODE_3_2", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: UNICODE_3_2:I
        pub fn UNICODE_3_2() -> i32 {
            32
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/text/Normalizer.<init>:()V")
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(src: Object, form: Normalizer_Form, option: i32) -> Result<String> {
            panic!("stub: sun/text/Normalizer.normalize:(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;I)Ljava/lang/String;")
        }

        #[java_method(name = "isNormalized", descriptor = "(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized(src: Object, form: Normalizer_Form, option: i32) -> Result<bool> {
            panic!("stub: sun/text/Normalizer.isNormalized:(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;I)Z")
        }

        #[java_method(name = "getCombiningClass", descriptor = "(I)I", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCombiningClass(ch: i32) -> Result<i32> {
            panic!("stub: sun/text/Normalizer.getCombiningClass:(I)I")
        }
    }
}
