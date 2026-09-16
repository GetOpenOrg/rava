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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/math/FloatToDecimal"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FloatToDecimal.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/math/FloatToDecimal"]

    pub struct FloatToDecimal {
        #[cfg_attr(any(), java_field(name = "bytes", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub bytes: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub index: i32,
    }

    impl FloatToDecimal {
        #[cfg_attr(any(), java_field(name = "P", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: P:I
        pub fn P() -> i32 {
            24
        }

        #[cfg_attr(any(), java_field(name = "W", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: W:I
        pub fn W() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "Q_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-149"))]
        // static field: Q_MIN:I
        pub fn Q_MIN() -> i32 {
            -149
        }

        #[cfg_attr(any(), java_field(name = "Q_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "104"))]
        // static field: Q_MAX:I
        pub fn Q_MAX() -> i32 {
            104
        }

        #[cfg_attr(any(), java_field(name = "E_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-44"))]
        // static field: E_MIN:I
        pub fn E_MIN() -> i32 {
            -44
        }

        #[cfg_attr(any(), java_field(name = "E_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "39"))]
        // static field: E_MAX:I
        pub fn E_MAX() -> i32 {
            39
        }

        #[cfg_attr(any(), java_field(name = "C_TINY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: C_TINY:I
        pub fn C_TINY() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "K_MIN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "-45"))]
        // static field: K_MIN:I
        pub fn K_MIN() -> i32 {
            -45
        }

        #[cfg_attr(any(), java_field(name = "K_MAX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "31"))]
        // static field: K_MAX:I
        pub fn K_MAX() -> i32 {
            31
        }

        #[cfg_attr(any(), java_field(name = "H", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: H:I
        pub fn H() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "C_MIN", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8388608"))]
        // static field: C_MIN:I
        pub fn C_MIN() -> i32 {
            8388608
        }

        #[cfg_attr(any(), java_field(name = "BQ_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "255"))]
        // static field: BQ_MASK:I
        pub fn BQ_MASK() -> i32 {
            255
        }

        #[cfg_attr(any(), java_field(name = "T_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8388607"))]
        // static field: T_MASK:I
        pub fn T_MASK() -> i32 {
            8388607
        }

        #[cfg_attr(any(), java_field(name = "MASK_32", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4294967295"))]
        // static field: MASK_32:J
        pub fn MASK_32() -> i64 {
            4294967295i64
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

        #[cfg_attr(any(), java_field(name = "MAX_CHARS", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: MAX_CHARS:I
        pub fn MAX_CHARS() -> i32 {
            15
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/math/FloatToDecimal.<init>:()V")
        }

        #[java_method(name = "toString", descriptor = "(F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(v: f32) -> Result<String> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toString:(F)Ljava/lang/String;")
        }

        #[java_method(name = "appendTo", descriptor = "(FLjava/lang/Appendable;)Ljava/lang/Appendable;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn appendTo(v: f32, app: Object) -> Result<Object> {
            panic!("stub: jdk/internal/math/FloatToDecimal.appendTo:(FLjava/lang/Appendable;)Ljava/lang/Appendable;")
        }

        #[java_method(name = "toDecimalString", descriptor = "(F)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimalString(&self, v: f32) -> Result<String> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toDecimalString:(F)Ljava/lang/String;")
        }

        #[java_method(name = "appendDecimalTo", descriptor = "(FLjava/lang/Appendable;)Ljava/lang/Appendable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn appendDecimalTo(&self, v: f32, app: Object) -> Result<Object> {
            panic!("stub: jdk/internal/math/FloatToDecimal.appendDecimalTo:(FLjava/lang/Appendable;)Ljava/lang/Appendable;")
        }

        #[java_method(name = "toDecimal", descriptor = "(F)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimal_f(&self, v: f32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toDecimal:(F)I")
        }

        #[java_method(name = "toDecimal", descriptor = "(III)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDecimal_i_i_i(&self, q: i32, c: i32, dk: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toDecimal:(III)I")
        }

        #[java_method(name = "rop", descriptor = "(JJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rop(g: i64, arg1: i64) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.rop:(JJ)I")
        }

        #[java_method(name = "toChars", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars(&self, f: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toChars:(II)I")
        }

        #[java_method(name = "toChars1", descriptor = "(III)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars1(&self, h: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toChars1:(III)I")
        }

        #[java_method(name = "toChars2", descriptor = "(III)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars2(&self, h: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toChars2:(III)I")
        }

        #[java_method(name = "toChars3", descriptor = "(III)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars3(&self, h: i32, l: i32, e: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.toChars3:(III)I")
        }

        #[java_method(name = "append8Digits", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append8Digits(&self, m: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FloatToDecimal.append8Digits:(I)V")
        }

        #[java_method(name = "removeTrailingZeroes", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeTrailingZeroes(&self) -> Result<()> {
            panic!("stub: jdk/internal/math/FloatToDecimal.removeTrailingZeroes:()V")
        }

        #[java_method(name = "y", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn y(&self, a: i32) -> Result<i32> {
            panic!("stub: jdk/internal/math/FloatToDecimal.y:(I)I")
        }

        #[java_method(name = "exponent", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exponent(&self, e: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FloatToDecimal.exponent:(I)V")
        }

        #[java_method(name = "append", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append(&self, c: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FloatToDecimal.append:(I)V")
        }

        #[java_method(name = "appendDigit", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendDigit(&self, d: i32) -> Result<()> {
            panic!("stub: jdk/internal/math/FloatToDecimal.appendDigit:(I)V")
        }

        #[java_method(name = "charsToString", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charsToString(&self) -> Result<String> {
            panic!("stub: jdk/internal/math/FloatToDecimal.charsToString:()Ljava/lang/String;")
        }
    }
}
