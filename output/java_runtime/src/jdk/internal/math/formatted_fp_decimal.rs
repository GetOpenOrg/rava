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
use crate::jdk::internal::math::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::math::DoubleToDecimal;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/math/FormattedFPDecimal"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FormattedFPDecimal.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/math/FormattedFPDecimal"]

    pub struct FormattedFPDecimal {
        #[cfg_attr(any(), java_field(name = "f", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub f: i64,
        #[cfg_attr(any(), java_field(name = "e", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub e: i32,
        #[cfg_attr(any(), java_field(name = "n", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub n: i32,
        #[cfg_attr(any(), java_field(name = "digits", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub digits: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "exp", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub exp: Rc<RefCell<Vec<u16>>>,
    }

    impl FormattedFPDecimal {
        #[cfg_attr(any(), java_field(name = "SCIENTIFIC", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "101"))]
        // static field: SCIENTIFIC:C
        pub fn SCIENTIFIC() -> u16 {
            101
        }

        #[cfg_attr(any(), java_field(name = "PLAIN", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "102"))]
        // static field: PLAIN:C
        pub fn PLAIN() -> u16 {
            102
        }

        #[cfg_attr(any(), java_field(name = "GENERAL", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "103"))]
        // static field: GENERAL:C
        pub fn GENERAL() -> u16 {
            103
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.<init>:()V")
        }

        #[java_method(name = "valueOf", descriptor = "(DIC)Ljdk/internal/math/FormattedFPDecimal;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf(v: f64, arg1: i32, prec: u16) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.valueOf:(DIC)Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "set", descriptor = "(JII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self, f: i64, arg1: i32, e: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.set:(JII)V")
        }

        #[java_method(name = "getExponent", descriptor = "()[C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponent(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.getExponent:()[C")
        }

        #[java_method(name = "getMantissa", descriptor = "()[C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMantissa(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.getMantissa:()[C")
        }

        #[java_method(name = "getExponentRounded", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponentRounded(&self) -> Result<i32> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.getExponentRounded:()I")
        }

        #[java_method(name = "plain", descriptor = "(I)Ljdk/internal/math/FormattedFPDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plain(&self, prec: i32) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.plain:(I)Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "plainChars", descriptor = "()Ljdk/internal/math/FormattedFPDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plainChars(&self) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.plainChars:()Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "plainCharsPureInteger", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plainCharsPureInteger(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.plainCharsPureInteger:()V")
        }

        #[java_method(name = "plainCharsMixed", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plainCharsMixed(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.plainCharsMixed:()V")
        }

        #[java_method(name = "plainCharsPureFraction", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plainCharsPureFraction(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.plainCharsPureFraction:()V")
        }

        #[java_method(name = "scientific", descriptor = "(I)Ljdk/internal/math/FormattedFPDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scientific(&self, prec: i32) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.scientific:(I)Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "scientificChars", descriptor = "(I)Ljdk/internal/math/FormattedFPDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scientificChars(&self, prec: i32) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.scientificChars:(I)Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "scientificCharsWithFraction", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scientificCharsWithFraction(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.scientificCharsWithFraction:()V")
        }

        #[java_method(name = "scientificCharsNoFraction", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scientificCharsNoFraction(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.scientificCharsNoFraction:()V")
        }

        #[java_method(name = "general", descriptor = "(I)Ljdk/internal/math/FormattedFPDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn general(&self, prec: i32) -> Result<FormattedFPDecimal> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.general:(I)Ljdk/internal/math/FormattedFPDecimal;")
        }

        #[java_method(name = "expChars", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expChars(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.expChars:()V")
        }

        #[java_method(name = "round", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round(&self, pp: i64) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.round:(J)V")
        }

        #[java_method(name = "fillWithDigits", descriptor = "(JII)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fillWithDigits(&self, x: i64, arg1: i32, from: i32) -> Result<i64> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.fillWithDigits:(JII)J")
        }

        #[java_method(name = "fillWithZeros", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fillWithZeros(&self, from: i32, to: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.fillWithZeros:(II)V")
        }

        #[java_method(name = "toDigit", descriptor = "(J)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDigit_l(d: i64) -> Result<u16> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.toDigit:(J)C")
        }

        #[java_method(name = "toDigit", descriptor = "(I)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDigit_i(d: i32) -> Result<u16> {
            panic!("stub: jdk/internal/math/FormattedFPDecimal.toDigit:(I)C")
        }
    }
}
