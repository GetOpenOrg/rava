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
    #[binary_name       = "java/text/Normalizer"]
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
    #[all_supertypes    = "java/lang/Object;java/text/Normalizer"]

    pub struct Normalizer;

    impl Normalizer {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/text/Normalizer.<init>:()V")
        }

        #[java_method(name = "normalize", descriptor = "(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(mut src: Object, mut form: Normalizer_Form) -> Result<String> {
            let _vdispatch0: String = if let Some(_d) = src.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = src.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = src.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = src.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = src.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = src.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = src.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _t1: String = NormalizerBase::normalize_str_normal(Clone::clone(&_vdispatch0), Clone::clone(&form))?;
            Ok(_t1)
        }

        #[java_method(name = "isNormalized", descriptor = "(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized(src: Object, form: Normalizer_Form) -> Result<bool> {
            panic!("stub: java/text/Normalizer.isNormalized:(Ljava/lang/CharSequence;Ljava/text/Normalizer$Form;)Z")
        }
    }
}
