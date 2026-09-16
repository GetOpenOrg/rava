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

impl From<NumberFormatProvider> for LocaleServiceProvider {
    fn from(v: NumberFormatProvider) -> LocaleServiceProvider { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/text/spi/NumberFormatProvider"]
    #[super_class       = "java/util/spi/LocaleServiceProvider"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "NumberFormatProvider.java"]
    #[inner_classes     = "java/text/NumberFormat$Style:java/text/NumberFormat:Style:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "LocaleServiceProvider"]
    #[all_supertypes    = "java/lang/Object;java/text/spi/NumberFormatProvider;java/util/spi/LocaleServiceProvider"]

    pub struct NumberFormatProvider;

    impl NumberFormatProvider {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/text/spi/NumberFormatProvider.<init>:()V")
        }

        #[java_method(name = "getCurrencyInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCurrencyInstance(&self, arg0: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/spi/NumberFormatProvider.getCurrencyInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getIntegerInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getIntegerInstance(&self, arg0: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/spi/NumberFormatProvider.getIntegerInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getNumberInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getNumberInstance(&self, arg0: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/spi/NumberFormatProvider.getNumberInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getPercentInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getPercentInstance(&self, arg0: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/spi/NumberFormatProvider.getPercentInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getCompactNumberInstance", descriptor = "(Ljava/util/Locale;Ljava/text/NumberFormat$Style;)Ljava/text/NumberFormat;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCompactNumberInstance(&self, mut locale: Locale, mut formatStyle: Object) -> Result<NumberFormat> {
            let this = self;
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("The ")))?;
            let _t1 = this.getClass()?;
            let _vdispatch2: String = if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _t3 = _t0.append_str(Clone::clone(&_vdispatch2))?;
            let _t4 = _t3.append_str(Clone::clone(&String::from(" should override this method to return compact number format instance of ")))?;
            let _t5 = _t4.append_obj(Object::from_any(locale.clone()))?;
            let _t6 = _t5.append_str(Clone::clone(&String::from(" locale and ")))?;
            let _t7 = _t6.append_obj(Clone::clone(&formatStyle))?;
            let _t8 = _t7.append_str(Clone::clone(&String::from(" style.")))?;
            let _t9 = _t8.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
    }
}
