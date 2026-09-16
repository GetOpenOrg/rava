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
    #[binary_name       = "java/util/Currency$CurrencyNameGetter"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter<Ljava/util/spi/CurrencyNameProvider;Ljava/lang/String;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Currency.java"]
    #[inner_classes     = "java/util/Currency$CurrencyNameGetter:java/util/Currency:CurrencyNameGetter:10;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Currency$CurrencyNameGetter;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter"]

    pub struct Currency_CurrencyNameGetter;

    impl Currency_CurrencyNameGetter {
        #[cfg_attr(any(), java_field(name = "INSTANCE", descriptor = "Ljava/util/Currency$CurrencyNameGetter;", access = "private", modifiers = "static final", is_static = true))]
        // static field: INSTANCE:Ljava/util/Currency$CurrencyNameGetter;
        pub fn INSTANCE() -> Currency_CurrencyNameGetter {
            panic!("stub: java/util/Currency$CurrencyNameGetter.INSTANCE:Ljava/util/Currency$CurrencyNameGetter;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Currency$CurrencyNameGetter.<init>:()V")
        }

        #[java_method(name = "getObject", descriptor = "(Ljava/util/spi/CurrencyNameProvider;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getObject(&self, currencyNameProvider: Object, locale: Locale, key: String, params: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            panic!("stub: java/util/Currency$CurrencyNameGetter.getObject:(Ljava/util/spi/CurrencyNameProvider;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;")
        }
    }
}
