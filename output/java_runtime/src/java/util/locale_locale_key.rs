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
    #[binary_name       = "java/util/Locale$LocaleKey"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Locale.java"]
    #[inner_classes     = "java/util/Locale$LocaleKey:java/util/Locale:LocaleKey:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Locale$LocaleKey"]
    #[has_hash_code_method = true]

    pub struct Locale_LocaleKey {
        #[cfg_attr(any(), java_field(name = "base", descriptor = "Lsun/util/locale/BaseLocale;", access = "private", modifiers = "final", is_static = false))]
        pub base: BaseLocale,
        #[cfg_attr(any(), java_field(name = "exts", descriptor = "Lsun/util/locale/LocaleExtensions;", access = "private", modifiers = "final", is_static = false))]
        pub exts: LocaleExtensions,
        #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub hash: i32,
    }

    impl Locale_LocaleKey {
        #[java_method(name = "<init>", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut baseLocale: BaseLocale, mut extensions: LocaleExtensions) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_base(Clone::clone(&baseLocale));
            this.__set_exts(Clone::clone(&extensions));
            let _t0 = this.__get_base().hashCode()?;
            let mut h: i32 = _t0;
            if !_is_jnull(&this.__get_exts()) {
                let _t1 = this.__get_exts().hashCode()?;
                h = (h^_t1);
            }
            this.__set_hash(h);
            Ok(this)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/Locale$LocaleKey.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }
    }
}
