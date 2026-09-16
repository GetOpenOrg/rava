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
    #[binary_name       = "java/text/BreakIterator$BreakIteratorCache"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BreakIterator.java"]
    #[inner_classes     = "java/text/BreakIterator$BreakIteratorCache:java/text/BreakIterator:BreakIteratorCache:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/text/BreakIterator$BreakIteratorCache"]

    pub struct BreakIterator_BreakIteratorCache {
        #[cfg_attr(any(), java_field(name = "iter", descriptor = "Ljava/text/BreakIterator;", access = "private", modifiers = "", is_static = false))]
        pub iter: BreakIterator,
        #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "", is_static = false))]
        pub locale: Locale,
    }

    impl BreakIterator_BreakIteratorCache {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;Ljava/text/BreakIterator;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut locale: Locale, mut iter: BreakIterator) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_locale(Clone::clone(&locale));
            let _t0: Object = Object::from_any(iter.clone());
            this.__set_iter(Clone::clone(&(_t0).downcast::<BreakIterator>()));
            Ok(this)
        }

        #[java_method(name = "getLocale", descriptor = "()Ljava/util/Locale;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocale(&self) -> Result<Locale> {
            let this = self;
            Ok(this.__get_locale())
        }

        #[java_method(name = "createBreakInstance", descriptor = "()Ljava/text/BreakIterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createBreakInstance(&self) -> Result<BreakIterator> {
            let this = self;
            let _t0: Object = Object::from_any(this.__get_iter().clone());
            Ok((_t0).downcast::<BreakIterator>())
        }
    }
}
