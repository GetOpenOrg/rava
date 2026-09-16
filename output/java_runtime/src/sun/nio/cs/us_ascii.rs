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

impl From<US_ASCII> for Charset {
    fn from(v: US_ASCII) -> Charset { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/nio/cs/US_ASCII"]
    #[super_class       = "java/nio/charset/Charset"]
    #[interfaces        = "sun/nio/cs/HistoricallyNamedCharset"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "US_ASCII.java"]
    #[inner_classes     = "sun/nio/cs/US_ASCII$Decoder:sun/nio/cs/US_ASCII:Decoder:10;sun/nio/cs/US_ASCII$Encoder:sun/nio/cs/US_ASCII:Encoder:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Charset"]
    #[superclass_fields(name: String, aliases: Rc<RefCell<Vec<String>>>, aliasSet: Object)]
    #[all_supertypes    = "java/lang/Comparable;java/lang/Object;java/nio/charset/Charset;sun/nio/cs/HistoricallyNamedCharset;sun/nio/cs/US_ASCII"]

    pub struct US_ASCII;

    impl US_ASCII {
        #[cfg_attr(any(), java_field(name = "INSTANCE", descriptor = "Lsun/nio/cs/US_ASCII;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSTANCE:Lsun/nio/cs/US_ASCII;
        pub fn INSTANCE() -> US_ASCII {
            panic!("stub: sun/nio/cs/US_ASCII.INSTANCE:Lsun/nio/cs/US_ASCII;")
        }

        #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JLA:Ljdk/internal/access/JavaLangAccess;
        pub fn JLA() -> Object {
            panic!("stub: sun/nio/cs/US_ASCII.JLA:Ljdk/internal/access/JavaLangAccess;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/nio/cs/US_ASCII.<init>:()V")
        }

        #[java_method(name = "historicalName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn historicalName(&self) -> Result<String> {
            panic!("stub: sun/nio/cs/US_ASCII.historicalName:()Ljava/lang/String;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/nio/charset/Charset;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, cs: Charset) -> Result<bool> {
            panic!("stub: sun/nio/cs/US_ASCII.contains:(Ljava/nio/charset/Charset;)Z")
        }

        #[java_method(name = "newDecoder", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newDecoder(&self) -> Result<Object> {
            panic!("stub: sun/nio/cs/US_ASCII.newDecoder:()Ljava/nio/charset/CharsetDecoder;")
        }

        #[java_method(name = "newEncoder", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newEncoder(&self) -> Result<CharsetEncoder> {
            panic!("stub: sun/nio/cs/US_ASCII.newEncoder:()Ljava/nio/charset/CharsetEncoder;")
        }
    }
}
