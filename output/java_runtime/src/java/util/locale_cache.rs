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

impl From<Locale_Cache> for LocaleObjectCache<Object, Object> {
    fn from(v: Locale_Cache) -> LocaleObjectCache<Object, Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Locale$Cache"]
    #[super_class       = "sun/util/locale/LocaleObjectCache"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Lsun/util/locale/LocaleObjectCache<Ljava/lang/Object;Ljava/util/Locale;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Locale.java"]
    #[inner_classes     = "java/util/Locale$LocaleKey:java/util/Locale:LocaleKey:26;java/util/Locale$Cache:java/util/Locale:Cache:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "LocaleObjectCache<Object, Object>"]
    #[superclass_fields(map: Object, queue: ReferenceQueue<Object>)]
    #[all_supertypes    = "java/lang/Object;java/util/Locale$Cache;sun/util/locale/LocaleObjectCache"]

    pub struct Locale_Cache;

    impl Locale_Cache {
        #[cfg_attr(any(), java_field(name = "LOCALECACHE", descriptor = "Ljava/util/Locale$Cache;", access = "private", modifiers = "static final", is_static = true))]
        // static field: LOCALECACHE:Ljava/util/Locale$Cache;
        pub fn LOCALECACHE() -> Locale_Cache {
            panic!("stub: java/util/Locale$Cache.LOCALECACHE:Ljava/util/Locale$Cache;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Locale$Cache.<init>:()V")
        }

        #[java_method(name = "createObject", descriptor = "(Ljava/lang/Object;)Ljava/util/Locale;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createObject(&self, key: Object) -> Result<Locale> {
            panic!("stub: java/util/Locale$Cache.createObject:(Ljava/lang/Object;)Ljava/util/Locale;")
        }
    }
}
