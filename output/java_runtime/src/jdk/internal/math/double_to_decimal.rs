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
use crate::jdk::internal::math::FormattedFPDecimal;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/math/DoubleToDecimal"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DoubleToDecimal.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/math/DoubleToDecimal"]

    pub struct DoubleToDecimal {
        #[cfg_attr(any(), java_field(name = "bytes", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub bytes: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub index: i32,
    }

    impl DoubleToDecimal {
        #[cfg_attr(any(), java_field(name = "P", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "53"))]
        // static field: P:I
        pub fn P() -> i32 {
            53
        }

        #[cfg_attr(any(), java_field(name = "W", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: W:I
        pub fn W() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "Q_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-1074"))]
        // static field: Q_MIN:I
        pub fn Q_MIN() -> i32 {
            -1074
        }

        #[cfg_attr(any(), java_field(name = "Q_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "971"))]
        // static field: Q_MAX:I
        pub fn Q_MAX() -> i32 {
            971
        }

        #[cfg_attr(any(), java_field(name = "E_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-323"))]
        // static field: E_MIN:I
        pub fn E_MIN() -> i32 {
            -323
        }

        #[cfg_attr(any(), java_field(name = "E_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "309"))]
        // static field: E_MAX:I
        pub fn E_MAX() -> i32 {
            309
        }

        #[cfg_attr(any(), java_field(name = "C_TINY", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: C_TINY:J
        pub fn C_TINY() -> i64 {
            3i64
        }

        #[cfg_attr(any(), java_field(name = "K_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-324"))]
        // static field: K_MIN:I
        pub fn K_MIN() -> i32 {
            -324
        }

        #[cfg_attr(any(), java_field(name = "K_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "292"))]
        // static field: K_MAX:I
        pub fn K_MAX() -> i32 {
            292
        }

        #[cfg_attr(any(), java_field(name = "H", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "17"))]
        // static field: H:I
        pub fn H() -> i32 {
            17
        }

        #[cfg_attr(any(), java_field(name = "C_MIN", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4503599627370496"))]
        // static field: C_MIN:J
        pub fn C_MIN() -> i64 {
            4503599627370496i64
        }

        #[cfg_attr(any(), java_field(name = "BQ_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2047"))]
        // static field: BQ_MASK:I
        pub fn BQ_MASK() -> i32 {
            2047
        }

        #[cfg_attr(any(), java_field(name = "T_MASK", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4503599627370495"))]
        // static field: T_MASK:J
        pub fn T_MASK() -> i64 {
            4503599627370495i64
        }

        #[cfg_attr(any(), java_field(name = "MASK_63", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "9223372036854775807"))]
        // static field: MASK_63:J
        pub fn MASK_63() -> i64 {
            9223372036854775807i64
        }

        #[cfg_attr(any(), java_field(name = "MASK_28", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "268435455"))]
        // static field: MASK_28:I
        pub fn MASK_28() -> i32 {
            268435455
        }

        #[cfg_attr(any(), java_field(name = "NON_SPECIAL", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: NON_SPECIAL:I
        pub fn NON_SPECIAL() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "PLUS_ZERO", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: PLUS_ZERO:I
        pub fn PLUS_ZERO() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MINUS_ZERO", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MINUS_ZERO:I
        pub fn MINUS_ZERO() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "PLUS_INF", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: PLUS_INF:I
        pub fn PLUS_INF() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "MINUS_INF", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: MINUS_INF:I
        pub fn MINUS_INF() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "NAN", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: NAN:I
        pub fn NAN() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "MAX_CHARS", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: MAX_CHARS:I
        pub fn MAX_CHARS() -> i32 {
            24
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(noChars: bool) -> Result<Self> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.<init>:(Z)V")
        }

        #[java_method(name = "toString", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(v: f64) -> Result<String> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toString:(D)Ljava/lang/String;")
        }

        #[java_method(name = "split", descriptor = "(DLjdk/internal/math/FormattedFPDecimal;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn split(v: f64, arg1: FormattedFPDecimal) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.split:(DLjdk/internal/math/FormattedFPDecimal;)V")
        }

        #[java_method(name = "appendTo", descriptor = "(DLjava/lang/Appendable;)Ljava/lang/Appendable;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn appendTo(v: f64, arg1: Object) -> Result<Object> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.appendTo:(DLjava/lang/Appendable;)Ljava/lang/Appendable;")
        }

        #[java_method(name = "toDecimalString", descriptor = "(D)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimalString(&self, v: f64) -> Result<String> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toDecimalString:(D)Ljava/lang/String;")
        }

        #[java_method(name = "appendDecimalTo", descriptor = "(DLjava/lang/Appendable;)Ljava/lang/Appendable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn appendDecimalTo(&self, v: f64, arg1: Object) -> Result<Object> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.appendDecimalTo:(DLjava/lang/Appendable;)Ljava/lang/Appendable;")
        }

        #[java_method(name = "toDecimal", descriptor = "(DLjdk/internal/math/FormattedFPDecimal;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimal_d_format(&self, v: f64, arg1: FormattedFPDecimal) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toDecimal:(DLjdk/internal/math/FormattedFPDecimal;)I")
        }

        #[java_method(name = "toDecimal", descriptor = "(IJILjdk/internal/math/FormattedFPDecimal;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimal_i_l_i_format(&self, q: i32, c: i64, arg2: i32, dk: FormattedFPDecimal) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toDecimal:(IJILjdk/internal/math/FormattedFPDecimal;)I")
        }

        #[java_method(name = "rop", descriptor = "(JJJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rop(g1: i64, arg1: i64, g0: i64) -> Result<i64> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.rop:(JJJ)J")
        }

        #[java_method(name = "toChars", descriptor = "(JILjdk/internal/math/FormattedFPDecimal;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars(&self, f: i64, arg1: i32, e: FormattedFPDecimal) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toChars:(JILjdk/internal/math/FormattedFPDecimal;)I")
        }

        #[java_method(name = "toChars1", descriptor = "(IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars1(&self, h: i32, m: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toChars1:(IIII)I")
        }

        #[java_method(name = "toChars2", descriptor = "(IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars2(&self, h: i32, m: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toChars2:(IIII)I")
        }

        #[java_method(name = "toChars3", descriptor = "(IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars3(&self, h: i32, m: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.toChars3:(IIII)I")
        }

        #[java_method(name = "lowDigits", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lowDigits(&self, l: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.lowDigits:(I)V")
        }

        #[java_method(name = "append8Digits", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append8Digits(&self, m: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.append8Digits:(I)V")
        }

        #[java_method(name = "removeTrailingZeroes", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeTrailingZeroes(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.removeTrailingZeroes:()V")
        }

        #[java_method(name = "y", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn y(&self, a: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.y:(I)I")
        }

        #[java_method(name = "exponent", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exponent(&self, e: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.exponent:(I)V")
        }

        #[java_method(name = "append", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append(&self, c: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.append:(I)V")
        }

        #[java_method(name = "appendDigit", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendDigit(&self, d: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.appendDigit:(I)V")
        }

        #[java_method(name = "charsToString", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charsToString(&self) -> Result<String> {
            panic!("stub: jdk/internal/math/DoubleToDecimal.charsToString:()Ljava/lang/String;")
        }
    }
}
