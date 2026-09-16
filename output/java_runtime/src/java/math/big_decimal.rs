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

impl From<BigDecimal> for Number {
    fn from(v: BigDecimal) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/math/BigDecimal"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/math/BigDecimal;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BigDecimal.java"]
    #[inner_classes     = "java/math/BigDecimal$1:::4104;java/math/BigDecimal$LongOverflow:java/math/BigDecimal:LongOverflow:10;java/math/BigDecimal$StringBuilderHelper:java/math/BigDecimal:StringBuilderHelper:8;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/math/BigDecimal$UnsafeHolder:java/math/BigDecimal:UnsafeHolder:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Number;java/lang/Object;java/math/BigDecimal"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct BigDecimal {
        #[cfg_attr(any(), java_field(name = "intVal", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "final", is_static = false))]
        pub intVal: BigInteger,
        #[cfg_attr(any(), java_field(name = "scale", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub scale: i32,
        #[cfg_attr(any(), java_field(name = "precision", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub precision: i32,
        #[cfg_attr(any(), java_field(name = "stringCache", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub stringCache: String,
        #[cfg_attr(any(), java_field(name = "intCompact", descriptor = "J", access = "private", modifiers = "final transient", is_static = false))]
        pub intCompact: i64,
    }

    impl BigDecimal {
        #[cfg_attr(any(), java_field(name = "L", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "3.321928094887362"))]
        // static field: L:D
        pub fn L() -> f64 {
            3.321928094887362f64
        }

        #[cfg_attr(any(), java_field(name = "P_F", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: P_F:I
        pub fn P_F() -> i32 {
            24
        }

        #[cfg_attr(any(), java_field(name = "Q_MIN_F", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "-149"))]
        // static field: Q_MIN_F:I
        pub fn Q_MIN_F() -> i32 {
            -149
        }

        #[cfg_attr(any(), java_field(name = "Q_MAX_F", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "104"))]
        // static field: Q_MAX_F:I
        pub fn Q_MAX_F() -> i32 {
            104
        }

        #[cfg_attr(any(), java_field(name = "P_D", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "53"))]
        // static field: P_D:I
        pub fn P_D() -> i32 {
            53
        }

        #[cfg_attr(any(), java_field(name = "Q_MIN_D", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "-1074"))]
        // static field: Q_MIN_D:I
        pub fn Q_MIN_D() -> i32 {
            -1074
        }

        #[cfg_attr(any(), java_field(name = "Q_MAX_D", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "971"))]
        // static field: Q_MAX_D:I
        pub fn Q_MAX_D() -> i32 {
            971
        }

        #[cfg_attr(any(), java_field(name = "INFLATED", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-9223372036854775808"))]
        // static field: INFLATED:J
        pub fn INFLATED() -> i64 {
            -9223372036854775808i64
        }

        #[cfg_attr(any(), java_field(name = "INFLATED_BIGINT", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: INFLATED_BIGINT:Ljava/math/BigInteger;
        pub fn INFLATED_BIGINT() -> BigInteger {
            panic!("stub: java/math/BigDecimal.INFLATED_BIGINT:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "MAX_COMPACT_DIGITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: MAX_COMPACT_DIGITS:I
        pub fn MAX_COMPACT_DIGITS() -> i32 {
            18
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6108874887143696463"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6108874887143696463i64
        }

        #[cfg_attr(any(), java_field(name = "ZERO_THROUGH_TEN", descriptor = "[Ljava/math/BigDecimal;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ZERO_THROUGH_TEN:[Ljava/math/BigDecimal;
        pub fn ZERO_THROUGH_TEN() -> Rc<RefCell<Vec<BigDecimal>>> {
            panic!("stub: java/math/BigDecimal.ZERO_THROUGH_TEN:[Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "ZERO_SCALED_BY", descriptor = "[Ljava/math/BigDecimal;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ZERO_SCALED_BY:[Ljava/math/BigDecimal;
        pub fn ZERO_SCALED_BY() -> Rc<RefCell<Vec<BigDecimal>>> {
            panic!("stub: java/math/BigDecimal.ZERO_SCALED_BY:[Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "HALF_LONG_MAX_VALUE", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4611686018427387903"))]
        // static field: HALF_LONG_MAX_VALUE:J
        pub fn HALF_LONG_MAX_VALUE() -> i64 {
            4611686018427387903i64
        }

        #[cfg_attr(any(), java_field(name = "HALF_LONG_MIN_VALUE", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-4611686018427387904"))]
        // static field: HALF_LONG_MIN_VALUE:J
        pub fn HALF_LONG_MIN_VALUE() -> i64 {
            -4611686018427387904i64
        }

        #[cfg_attr(any(), java_field(name = "ZERO", descriptor = "Ljava/math/BigDecimal;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ZERO:Ljava/math/BigDecimal;
        pub fn ZERO() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.ZERO:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "ONE", descriptor = "Ljava/math/BigDecimal;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ONE:Ljava/math/BigDecimal;
        pub fn ONE() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.ONE:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "TWO", descriptor = "Ljava/math/BigDecimal;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TWO:Ljava/math/BigDecimal;
        pub fn TWO() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.TWO:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "TEN", descriptor = "Ljava/math/BigDecimal;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TEN:Ljava/math/BigDecimal;
        pub fn TEN() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.TEN:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "ONE_TENTH", descriptor = "Ljava/math/BigDecimal;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ONE_TENTH:Ljava/math/BigDecimal;
        pub fn ONE_TENTH() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.ONE_TENTH:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "ONE_HALF", descriptor = "Ljava/math/BigDecimal;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ONE_HALF:Ljava/math/BigDecimal;
        pub fn ONE_HALF() -> BigDecimal {
            panic!("stub: java/math/BigDecimal.ONE_HALF:Ljava/math/BigDecimal;")
        }

        #[cfg_attr(any(), java_field(name = "ROUND_UP", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0", is_deprecated = true))]
        // static field: ROUND_UP:I
        pub fn ROUND_UP() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "ROUND_DOWN", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1", is_deprecated = true))]
        // static field: ROUND_DOWN:I
        pub fn ROUND_DOWN() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "ROUND_CEILING", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2", is_deprecated = true))]
        // static field: ROUND_CEILING:I
        pub fn ROUND_CEILING() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ROUND_FLOOR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3", is_deprecated = true))]
        // static field: ROUND_FLOOR:I
        pub fn ROUND_FLOOR() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "ROUND_HALF_UP", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4", is_deprecated = true))]
        // static field: ROUND_HALF_UP:I
        pub fn ROUND_HALF_UP() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "ROUND_HALF_DOWN", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5", is_deprecated = true))]
        // static field: ROUND_HALF_DOWN:I
        pub fn ROUND_HALF_DOWN() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "ROUND_HALF_EVEN", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6", is_deprecated = true))]
        // static field: ROUND_HALF_EVEN:I
        pub fn ROUND_HALF_EVEN() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "ROUND_UNNECESSARY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7", is_deprecated = true))]
        // static field: ROUND_UNNECESSARY:I
        pub fn ROUND_UNNECESSARY() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "DOUBLE_10_POW", descriptor = "[D", access = "private", modifiers = "static final", is_static = true))]
        // static field: DOUBLE_10_POW:[D
        pub fn DOUBLE_10_POW() -> Rc<RefCell<Vec<f64>>> {
            panic!("stub: java/math/BigDecimal.DOUBLE_10_POW:[D")
        }

        #[cfg_attr(any(), java_field(name = "FLOAT_10_POW", descriptor = "[F", access = "private", modifiers = "static final", is_static = true))]
        // static field: FLOAT_10_POW:[F
        pub fn FLOAT_10_POW() -> Rc<RefCell<Vec<f32>>> {
            panic!("stub: java/math/BigDecimal.FLOAT_10_POW:[F")
        }

        #[cfg_attr(any(), java_field(name = "LONG_TEN_POWERS_TABLE", descriptor = "[J", access = "private", modifiers = "static final", is_static = true))]
        // static field: LONG_TEN_POWERS_TABLE:[J
        pub fn LONG_TEN_POWERS_TABLE() -> Rc<RefCell<Vec<i64>>> {
            panic!("stub: java/math/BigDecimal.LONG_TEN_POWERS_TABLE:[J")
        }

        #[cfg_attr(any(), java_field(name = "BIG_TEN_POWERS_TABLE", descriptor = "[Ljava/math/BigInteger;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: BIG_TEN_POWERS_TABLE:[Ljava/math/BigInteger;
        pub fn BIG_TEN_POWERS_TABLE() -> Rc<RefCell<Vec<BigInteger>>> {
            panic!("stub: java/math/BigDecimal.BIG_TEN_POWERS_TABLE:[Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "BIG_TEN_POWERS_TABLE_INITLEN", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: BIG_TEN_POWERS_TABLE_INITLEN:I
        pub fn BIG_TEN_POWERS_TABLE_INITLEN() -> i32 {
            panic!("stub: java/math/BigDecimal.BIG_TEN_POWERS_TABLE_INITLEN:I")
        }

        #[cfg_attr(any(), java_field(name = "BIG_TEN_POWERS_TABLE_MAX", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: BIG_TEN_POWERS_TABLE_MAX:I
        pub fn BIG_TEN_POWERS_TABLE_MAX() -> i32 {
            panic!("stub: java/math/BigDecimal.BIG_TEN_POWERS_TABLE_MAX:I")
        }

        #[cfg_attr(any(), java_field(name = "THRESHOLDS_TABLE", descriptor = "[J", access = "private", modifiers = "static final", is_static = true))]
        // static field: THRESHOLDS_TABLE:[J
        pub fn THRESHOLDS_TABLE() -> Rc<RefCell<Vec<i64>>> {
            panic!("stub: java/math/BigDecimal.THRESHOLDS_TABLE:[J")
        }

        #[cfg_attr(any(), java_field(name = "DIV_NUM_BASE", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4294967296"))]
        // static field: DIV_NUM_BASE:J
        pub fn DIV_NUM_BASE() -> i64 {
            4294967296i64
        }

        #[cfg_attr(any(), java_field(name = "LONGLONG_TEN_POWERS_TABLE", descriptor = "[[J", access = "private", modifiers = "static final", is_static = true))]
        // static field: LONGLONG_TEN_POWERS_TABLE:[[J
        pub fn LONGLONG_TEN_POWERS_TABLE() -> Rc<RefCell<Vec<Rc<RefCell<Vec<i64>>>>>> {
            panic!("stub: java/math/BigDecimal.LONGLONG_TEN_POWERS_TABLE:[[J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;JII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/math/BigInteger;JII)V
        pub fn new_bigint_l_i_i(mut intVal: BigInteger, mut val: i64, mut scale: i32, mut prec: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            this.__set_scale(scale);
            this.__set_precision(prec);
            this.__set_intCompact(val);
            this.__set_intVal(Clone::clone(&intVal));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_i_i(in_: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:([CII)V")
        }

        #[java_method(name = "<init>", descriptor = "([CIILjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_i_i_mathco(in_: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32, mc: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:([CIILjava/math/MathContext;)V")
        }

        #[java_method(name = "parseExp", descriptor = "([CII)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseExp(in_: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.parseExp:([CII)J")
        }

        #[java_method(name = "<init>", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c(in_: Rc<RefCell<Vec<u16>>>) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:([C)V")
        }

        #[java_method(name = "<init>", descriptor = "([CLjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_mathco(in_: Rc<RefCell<Vec<u16>>>, mc: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:([CLjava/math/MathContext;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(val: String) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_mathco(val: String, mc: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(Ljava/lang/String;Ljava/math/MathContext;)V")
        }

        #[java_method(name = "<init>", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_d(val: f64) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(D)V")
        }

        #[java_method(name = "<init>", descriptor = "(DLjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_d_mathco(val: f64, arg1: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(DLjava/math/MathContext;)V")
        }

        #[java_method(name = "toStrictBigInteger", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toStrictBigInteger(mut val: BigInteger) -> Result<BigInteger> {
            let _t0 = val.getClass()?;
            let mut _merged3: BigInteger;
            if _t0 == Object::default() {
                _merged3 = val;
            } else {
                let _t1 = val.toByteArray()?;
                let _t2: Object = Object::from_any(_t1.clone());
                _merged3 = BigInteger::new_arr_b(Clone::clone(&(_t2).downcast::<Rc<RefCell<Vec<i8>>>>()))?;
            }
            Ok(_merged3)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_bigint(val: BigInteger) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(Ljava/math/BigInteger;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;Ljava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_bigint_mathco(val: BigInteger, mc: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(Ljava/math/BigInteger;Ljava/math/MathContext;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_bigint_i(unscaledVal: BigInteger, scale: i32) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(Ljava/math/BigInteger;I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;ILjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/math/BigInteger;ILjava/math/MathContext;)V
        pub fn new_bigint_i_mathco(mut unscaledVal: BigInteger, mut scale: i32, mut mc: MathContext) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            let _t0: BigInteger = BigDecimal::toStrictBigInteger(Clone::clone(&unscaledVal))?;
            unscaledVal = _t0;
            let _t1: i64 = BigDecimal::compactValFor(Clone::clone(&unscaledVal))?;
            let mut compactVal: i64 = _t1;
            let mut mcp = mc.__get_precision();
            let mut prec: i32 = 0i32;
            let mut mode = mc.__get_roundingMode().__get_oldMode();
            let _t2: i32 = BigDecimal::bigDigitLength(Clone::clone(&unscaledVal))?;
            prec = _t2;
            let mut drop = (prec).wrapping_sub(mcp);
            loop {
                if (drop<=0) { break; }
                let _t3: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
                scale = _t3;
                let _t4: BigInteger = BigDecimal::divideAndRoundByTenPow(Clone::clone(&unscaledVal), drop, mode)?;
                unscaledVal = _t4;
                let _t5: i64 = BigDecimal::compactValFor(Clone::clone(&unscaledVal))?;
                compactVal = _t5;
                if (((compactVal>(-9223372036854775808i64)) as i32-((compactVal)<(-9223372036854775808i64)) as i32)!=0) {
                    break;
                }
                let _t6: i32 = BigDecimal::bigDigitLength(Clone::clone(&unscaledVal))?;
                prec = _t6;
                drop = (prec).wrapping_sub(mcp);
            }
            let _t3: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t3;
            drop = (prec).wrapping_sub(mcp);
            loop {
                if (drop<=0) { break; }
                let _t4: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
                scale = _t4;
                let _t5: i64 = BigDecimal::divideAndRound_l_l_i(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[drop as usize], mode)?;
                compactVal = _t5;
                let _t6: i32 = BigDecimal::longDigitLength(compactVal)?;
                prec = _t6;
                drop = (prec).wrapping_sub(mcp);
            }
            let mut unscaledVal: Object = Object::default();
            this.__set_intVal(Clone::clone(&unscaledVal));
            this.__set_intCompact(compactVal);
            this.__set_scale(scale);
            this.__set_precision(prec);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(val: i32) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "(ILjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_mathco(val: i32, mc: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(ILjava/math/MathContext;)V")
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l(val: i64) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(J)V")
        }

        #[java_method(name = "<init>", descriptor = "(JLjava/math/MathContext;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l_mathco(val: i64, arg1: MathContext) -> Result<Self> {
            panic!("stub: java/math/BigDecimal.<init>:(JLjava/math/MathContext;)V")
        }

        #[java_method(name = "valueOf", descriptor = "(JI)Ljava/math/BigDecimal;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(JI)Ljava/math/BigDecimal;
        pub fn valueOf_l_i(mut unscaledVal: i64, mut scale: i32) -> Result<BigDecimal> {
            if (scale==0) {
                let _t0: BigDecimal = BigDecimal::valueOf_l(unscaledVal)?;
                return Ok(_t0);
            }
            if (((unscaledVal>(0i64)) as i32-((unscaledVal)<(0i64)) as i32)==0) {
                let _t0: BigDecimal = BigDecimal::zeroValueOf(scale)?;
                return Ok(_t0);
            }
            Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&(if (((unscaledVal>(-9223372036854775808i64)) as i32-((unscaledVal)<(-9223372036854775808i64)) as i32)==0) { BigDecimal::INFLATED_BIGINT() } else { Default::default() })), unscaledVal, scale, 0i32)?)
        }

        #[java_method(name = "valueOf", descriptor = "(J)Ljava/math/BigDecimal;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(J)Ljava/math/BigDecimal;
        pub fn valueOf_l(mut val: i64) -> Result<BigDecimal> {
            if (((val>(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32-((val)<(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32)<0) {
                return Ok(Clone::clone(&BigDecimal::ZERO_THROUGH_TEN().borrow()[(val as i32) as usize]));
            }
            if (((val>(-9223372036854775808i64)) as i32-((val)<(-9223372036854775808i64)) as i32)!=0) {
                return Ok(BigDecimal::new_bigint_l_i_i(Default::default(), val, 0i32, 0i32)?);
            }
            Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&BigDecimal::INFLATED_BIGINT()), val, 0i32, 0i32)?)
        }

        #[java_method(name = "valueOf", descriptor = "(JII)Ljava/math/BigDecimal;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(JII)Ljava/math/BigDecimal;
        pub fn valueOf_l_i_i(mut unscaledVal: i64, mut scale: i32, mut prec: i32) -> Result<BigDecimal> {
            if (((unscaledVal>(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32-((unscaledVal)<(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32)<0) {
                return Ok(Clone::clone(&BigDecimal::ZERO_THROUGH_TEN().borrow()[(unscaledVal as i32) as usize]));
            }
            if (((unscaledVal>(0i64)) as i32-((unscaledVal)<(0i64)) as i32)==0) {
                let _t0: BigDecimal = BigDecimal::zeroValueOf(scale)?;
                return Ok(_t0);
            }
            Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&(if (((unscaledVal>(-9223372036854775808i64)) as i32-((unscaledVal)<(-9223372036854775808i64)) as i32)==0) { BigDecimal::INFLATED_BIGINT() } else { Default::default() })), unscaledVal, scale, prec)?)
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/math/BigInteger;II)Ljava/math/BigDecimal;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(Ljava/math/BigInteger;II)Ljava/math/BigDecimal;
        pub fn valueOf_bigint_i_i(mut intVal: BigInteger, mut scale: i32, mut prec: i32) -> Result<BigDecimal> {
            let _t0: i64 = BigDecimal::compactValFor(Clone::clone(&intVal))?;
            let mut val: i64 = _t0;
            if (((val>(0i64)) as i32-((val)<(0i64)) as i32)==0) {
                let _t1: BigDecimal = BigDecimal::zeroValueOf(scale)?;
                return Ok(_t1);
            }
            if (((val>(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32-((val)<(((BigDecimal::ZERO_THROUGH_TEN().borrow().len() as i32) as i64))) as i32)<0) {
                return Ok(Clone::clone(&BigDecimal::ZERO_THROUGH_TEN().borrow()[(val as i32) as usize]));
            }
            Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&intVal), val, scale, prec)?)
        }

        #[java_method(name = "zeroValueOf", descriptor = "(I)Ljava/math/BigDecimal;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn zeroValueOf(mut scale: i32) -> Result<BigDecimal> {
            if scale < (BigDecimal::ZERO_SCALED_BY().borrow().len() as i32) {
                return Ok(Clone::clone(&BigDecimal::ZERO_SCALED_BY().borrow()[scale as usize]));
            }
            Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&BigInteger::ZERO()), 0i64, scale, 1i32)?)
        }

        #[java_method(name = "valueOf", descriptor = "(D)Ljava/math/BigDecimal;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_d(val: f64) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.valueOf:(D)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_bigdec(&self, augend: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_bigdec_mathco(&self, augend: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "preAlign", descriptor = "(Ljava/math/BigDecimal;Ljava/math/BigDecimal;JLjava/math/MathContext;)[Ljava/math/BigDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn preAlign(&self, lhs: BigDecimal, augend: BigDecimal, padding: i64, arg3: MathContext) -> Result<Rc<RefCell<Vec<BigDecimal>>>> {
            panic!("stub: java/math/BigDecimal.preAlign:(Ljava/math/BigDecimal;Ljava/math/BigDecimal;JLjava/math/MathContext;)[Ljava/math/BigDecimal;")
        }

        #[java_method(name = "subtract", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtract_bigdec(&self, subtrahend: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.subtract:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "subtract", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtract_bigdec_mathco(&self, subtrahend: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.subtract:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_bigdec(&self, multiplicand: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiply:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_bigdec_mathco(&self, multiplicand: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiply:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;II)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn divide_bigdec_i_i(&self, divisor: BigDecimal, scale: i32, roundingMode: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;II)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;ILjava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigdec_i_roundi(&self, divisor: BigDecimal, scale: i32, roundingMode: RoundingMode) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;ILjava/math/RoundingMode;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn divide_bigdec_i(&self, divisor: BigDecimal, roundingMode: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;Ljava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigdec_roundi(&self, divisor: BigDecimal, roundingMode: RoundingMode) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;Ljava/math/RoundingMode;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigdec(&self, divisor: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigdec_mathco(&self, divisor: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideToIntegralValue", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideToIntegralValue_bigdec(&self, divisor: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divideToIntegralValue:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideToIntegralValue", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideToIntegralValue_bigdec_mathco(&self, divisor: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divideToIntegralValue:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "remainder", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainder_bigdec(&self, divisor: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.remainder:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "remainder", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainder_bigdec_mathco(&self, divisor: BigDecimal, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.remainder:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideAndRemainder", descriptor = "(Ljava/math/BigDecimal;)[Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainder_bigdec(&self, divisor: BigDecimal) -> Result<Rc<RefCell<Vec<BigDecimal>>>> {
            panic!("stub: java/math/BigDecimal.divideAndRemainder:(Ljava/math/BigDecimal;)[Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideAndRemainder", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)[Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainder_bigdec_mathco(&self, divisor: BigDecimal, mc: MathContext) -> Result<Rc<RefCell<Vec<BigDecimal>>>> {
            panic!("stub: java/math/BigDecimal.divideAndRemainder:(Ljava/math/BigDecimal;Ljava/math/MathContext;)[Ljava/math/BigDecimal;")
        }

        #[java_method(name = "sqrt", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sqrt(&self, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.sqrt:(Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "square", descriptor = "()Ljava/math/BigDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn square(&self) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.square:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "isPowerOfTen", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPowerOfTen(&self) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.isPowerOfTen:()Z")
        }

        #[java_method(name = "squareRootResultAssertions", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn squareRootResultAssertions(&self, result: BigDecimal, mc: MathContext) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.squareRootResultAssertions:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z")
        }

        #[java_method(name = "squareRootZeroResultAssertions", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn squareRootZeroResultAssertions(&self, result: BigDecimal, mc: MathContext) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.squareRootZeroResultAssertions:(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z")
        }

        #[java_method(name = "pow", descriptor = "(I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pow_i(&self, n: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.pow:(I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "pow", descriptor = "(ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pow_i_mathco(&self, n: i32, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.pow:(ILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "abs", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: abs()Ljava/math/BigDecimal;
        pub fn abs(&self) -> Result<BigDecimal> {
            let this = self;
            let _t0 = this.signum()?;
            let mut _merged2: BigDecimal;
            if (_t0<0) {
                let _t1 = this.negate()?;
                _merged2 = _t1;
            } else {
                _merged2 = Clone::clone(this);
            }
            Ok(_merged2)
        }

        #[java_method(name = "abs", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_mathco(&self, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.abs:(Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "negate", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: negate()Ljava/math/BigDecimal;
        pub fn negate(&self) -> Result<BigDecimal> {
            let this = self;
            if (((this.__get_intCompact()>(-9223372036854775808i64)) as i32-((this.__get_intCompact())<(-9223372036854775808i64)) as i32)==0) {
                let _t0 = this.__get_intVal().negate()?;
                return Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&_t0), -9223372036854775808i64, this.__get_scale(), this.__get_precision())?);
            }
            let _t0: BigDecimal = BigDecimal::valueOf_l_i_i((this.__get_intCompact()).wrapping_neg(), this.__get_scale(), this.__get_precision())?;
            Ok(_t0)
        }

        #[java_method(name = "negate", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negate_mathco(&self, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.negate:(Ljava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "plus", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus(&self) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.plus:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "plus", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: plus(Ljava/math/MathContext;)Ljava/math/BigDecimal;
        pub fn plus_mathco(&self, mut mc: MathContext) -> Result<BigDecimal> {
            let this = self;
            if (mc.__get_precision()==0) {
                return Ok(Clone::clone(this));
            }
            let _t0: BigDecimal = BigDecimal::doRound_bigdec_mathco(Clone::clone(this), Clone::clone(&mc))?;
            Ok(_t0)
        }

        #[java_method(name = "signum", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum(&self) -> Result<i32> {
            let this = self;
            let mut _merged1: i32;
            if (((this.__get_intCompact()>(-9223372036854775808i64)) as i32-((this.__get_intCompact())<(-9223372036854775808i64)) as i32)!=0) {
                let _t0: i32 = Long::signum(this.__get_intCompact())?;
                _merged1 = _t0;
            } else {
                let _t0 = this.__get_intVal().signum()?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "scale", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scale(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_scale())
        }

        #[java_method(name = "precision", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: precision()I
        pub fn precision(&self) -> Result<i32> {
            let this = self;
            let mut result = this.__get_precision();
            let mut s = this.__get_intCompact();
            if (((s>(-9223372036854775808i64)) as i32-((s)<(-9223372036854775808i64)) as i32)!=0) {
                let _t0: i32 = BigDecimal::longDigitLength(s)?;
                result = _t0;
            } else {
                let _t0: i32 = BigDecimal::bigDigitLength(Clone::clone(&this.__get_intVal()))?;
                result = _t0;
            }
            this.__set_precision(result);
            Ok(result)
        }

        #[java_method(name = "unscaledValue", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unscaledValue(&self) -> Result<BigInteger> {
            let this = self;
            let _t0 = this.inflated()?;
            Ok(_t0)
        }

        #[java_method(name = "round", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round(&self, mut mc: MathContext) -> Result<BigDecimal> {
            let this = self;
            let _t0 = this.plus_mathco(Clone::clone(&mc))?;
            Ok(_t0)
        }

        #[java_method(name = "setScale", descriptor = "(ILjava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: setScale(ILjava/math/RoundingMode;)Ljava/math/BigDecimal;
        pub fn setScale_i_roundi(&self, mut newScale: i32, mut roundingMode: RoundingMode) -> Result<BigDecimal> {
            let this = self;
            let _t0 = this.setScale_i_i(newScale, roundingMode.__get_oldMode())?;
            Ok(_t0)
        }

        #[java_method(name = "setScale", descriptor = "(II)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        // java: setScale(II)Ljava/math/BigDecimal;
        pub fn setScale_i_i(&self, mut newScale: i32, mut roundingMode: i32) -> Result<BigDecimal> {
            let this = self;
            if roundingMode > 7i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut oldScale = this.__get_scale();
            if newScale == oldScale {
                return Ok(Clone::clone(this));
            }
            let _t0 = this.signum()?;
            if (_t0==0) {
                let _t1: BigDecimal = BigDecimal::zeroValueOf(newScale)?;
                return Ok(_t1);
            }
            let mut rs = this.__get_intCompact();
            let _t1 = this.checkScale_l(((newScale as i64)).wrapping_sub((oldScale as i64)))?;
            let mut raise: i32 = _t1;
            let _t2: i64 = BigDecimal::longMultiplyPowerTen(rs, raise)?;
            rs = _t2;
            if (((rs>(-9223372036854775808i64)) as i32-((rs)<(-9223372036854775808i64)) as i32)!=0) {
                let _t3: BigDecimal = BigDecimal::valueOf_l_i(rs, newScale)?;
                return Ok(_t3);
            }
            let _t3 = this.bigMultiplyPowerTen_i(raise)?;
            let mut rb: BigInteger = _t3;
            return Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&rb), -9223372036854775808i64, newScale, (if (this.__get_precision()>0) { (this.__get_precision()).wrapping_add(raise) } else { 0i32 }))?);
            let _t4 = this.checkScale_l(((oldScale as i64)).wrapping_sub((newScale as i64)))?;
            raise = _t4;
            if raise < (BigDecimal::LONG_TEN_POWERS_TABLE().borrow().len() as i32) {
                let _t5: BigDecimal = BigDecimal::divideAndRound_l_l_i_i_i(rs, BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[raise as usize], newScale, roundingMode, newScale)?;
                return Ok(_t5);
            }
            let _t5 = this.inflated()?;
            let _t6: BigInteger = BigDecimal::bigTenToThe(raise)?;
            let _t7: BigDecimal = BigDecimal::divideAndRound_bigint_bigint_i_i_i(Clone::clone(&_t5), Clone::clone(&_t6), newScale, roundingMode, newScale)?;
            return Ok(_t7);
            let _t8 = this.checkScale_l(((newScale as i64)).wrapping_sub((oldScale as i64)))?;
            let mut rs: i32 = _t8;
            let _t9: BigInteger = BigDecimal::bigMultiplyPowerTen_bigint_i(Clone::clone(&this.__get_intVal()), rs)?;
            let mut rb: BigInteger = _t9;
            return Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&rb), -9223372036854775808i64, newScale, (if (this.__get_precision()>0) { (this.__get_precision()).wrapping_add(rs) } else { 0i32 }))?);
            let _t10 = this.checkScale_l(((oldScale as i64)).wrapping_sub((newScale as i64)))?;
            rs = _t10;
            if rs < (BigDecimal::LONG_TEN_POWERS_TABLE().borrow().len() as i32) {
                let _t11: BigDecimal = BigDecimal::divideAndRound_bigint_l_i_i_i(Clone::clone(&this.__get_intVal()), BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[rs as usize], newScale, roundingMode, newScale)?;
                return Ok(_t11);
            }
            let _t11: BigInteger = BigDecimal::bigTenToThe(rs)?;
            let _t12: BigDecimal = BigDecimal::divideAndRound_bigint_bigint_i_i_i(Clone::clone(&this.__get_intVal()), Clone::clone(&_t11), newScale, roundingMode, newScale)?;
            Ok(_t12)
        }

        #[java_method(name = "setScale", descriptor = "(I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setScale_i(&self, newScale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.setScale:(I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "movePointLeft", descriptor = "(I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn movePointLeft(&self, n: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.movePointLeft:(I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "movePointRight", descriptor = "(I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn movePointRight(&self, n: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.movePointRight:(I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "scaleByPowerOfTen", descriptor = "(I)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scaleByPowerOfTen(&self, n: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.scaleByPowerOfTen:(I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "stripTrailingZeros", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripTrailingZeros(&self) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.stripTrailingZeros:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/math/BigDecimal;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut val: BigDecimal) -> Result<i32> {
            let this = self;
            let mut xs = this.__get_intCompact();
            let mut ys = val.__get_intCompact();
            return Ok(((if (((xs>(ys)) as i32-((xs)<(ys)) as i32)!=0) { (((xs>(ys)) as i32-((xs)<(ys)) as i32)<=0) } else { (0i32 != 0) })) as i32);
            let _t0 = this.signum()?;
            let mut xs: i32 = _t0;
            let _t1 = val.signum()?;
            let mut ysign: i32 = _t1;
            return Ok((xs <= ysign) as i32);
            if (xs==0) {
                return Ok(0i32);
            }
            let _t2 = this.compareMagnitude(Clone::clone(&val))?;
            let mut ys: i32 = _t2;
            Ok((if (xs>0) { ys } else { (ys).wrapping_neg() }))
        }

        #[java_method(name = "compareMagnitude", descriptor = "(Ljava/math/BigDecimal;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareMagnitude(&self, mut val: BigDecimal) -> Result<i32> {
            let this = self;
            let mut ys = val.__get_intCompact();
            let mut xs = this.__get_intCompact();
            return Ok(((((ys>(0i64)) as i32-((ys)<(0i64)) as i32)!=0)) as i32);
            if (((ys>(0i64)) as i32-((ys)<(0i64)) as i32)==0) {
                return Ok(1i32);
            }
            let mut sdiff = ((this.__get_scale() as i64)).wrapping_sub((val.__get_scale() as i64));
            let _t0 = this.precision()?;
            let mut xae = ((_t0 as i64)).wrapping_sub((this.__get_scale() as i64));
            let _t1 = val.precision()?;
            let mut yae = ((_t1 as i64)).wrapping_sub((val.__get_scale() as i64));
            if (((xae>(yae)) as i32-((xae)<(yae)) as i32)<0) {
                return Ok(-1i32);
            }
            if (((xae>(yae)) as i32-((xae)<(yae)) as i32)>0) {
                return Ok(1i32);
            }
            let _t2: i64 = BigDecimal::longMultiplyPowerTen(xs, ((sdiff).wrapping_neg() as i32))?;
            xs = _t2;
            let _t3 = this.bigMultiplyPowerTen_i(((sdiff).wrapping_neg() as i32))?;
            let mut rb: BigInteger = _t3;
            let _t4 = rb.compareMagnitude_bigint(Clone::clone(&val.__get_intVal()))?;
            return Ok(_t4);
            let _t5: i64 = BigDecimal::longMultiplyPowerTen(ys, (sdiff as i32))?;
            ys = _t5;
            if (((xs>(-9223372036854775808i64)) as i32-((xs)<(-9223372036854775808i64)) as i32)==0) {
                let _t6 = val.bigMultiplyPowerTen_i((sdiff as i32))?;
                rb = _t6;
                let _t7 = this.__get_intVal().compareMagnitude_bigint(Clone::clone(&rb))?;
                return Ok(_t7);
            }
            let mut _merged7: i32;
            if (((ys>(-9223372036854775808i64)) as i32-((ys)<(-9223372036854775808i64)) as i32)!=0) {
                let _t6: i32 = BigDecimal::longCompareMagnitude(xs, ys)?;
                _merged7 = _t6;
            } else {
                _merged7 = -1i32;
            }
            return Ok(_merged7);
            if (((ys>(-9223372036854775808i64)) as i32-((ys)<(-9223372036854775808i64)) as i32)!=0) {
                return Ok(1i32);
            }
            let _t8 = this.__get_intVal().compareMagnitude_bigint(Clone::clone(&val.__get_intVal()))?;
            Ok(_t8)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut x: Object) -> Result<bool> {
            let this = self;
        let mut xDec = Default::default();
            if (x.is_instance_of("java/math/BigDecimal")) {
                xDec = (x).downcast::<BigDecimal>();
            } else {
                return Ok((0i32 != 0i32));
            }
            if x == Object::from_any(this.clone()) {
                return Ok((1i32 != 0i32));
            }
            if this.__get_scale() != xDec.__get_scale() {
                return Ok((0i32 != 0i32));
            }
            let mut s = this.__get_intCompact();
            let mut xs = xDec.__get_intCompact();
            if (((xs>(-9223372036854775808i64)) as i32-((xs)<(-9223372036854775808i64)) as i32)==0) {
                let _t0: i64 = BigDecimal::compactValFor(Clone::clone(&xDec.__get_intVal()))?;
                xs = _t0;
            }
            return Ok((((xs>(s)) as i32-((xs)<(s)) as i32)==0));
            let _t0: i64 = BigDecimal::compactValFor(Clone::clone(&this.__get_intVal()))?;
            return Ok((((xs>(_t0)) as i32-((xs)<(_t0)) as i32)==0));
            let _t1 = this.inflated()?;
            let _t2 = xDec.inflated()?;
            let _t3 = _t1.equals(Object::from_any(_t2.clone()))?;
            Ok(_t3)
        }

        #[java_method(name = "min", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(&self, val: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.min:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "max", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(&self, val: BigDecimal) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.max:(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "toEngineeringString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEngineeringString(&self) -> Result<String> {
            panic!("stub: java/math/BigDecimal.toEngineeringString:()Ljava/lang/String;")
        }

        #[java_method(name = "toPlainString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toPlainString(&self) -> Result<String> {
            panic!("stub: java/math/BigDecimal.toPlainString:()Ljava/lang/String;")
        }

        #[java_method(name = "getValueString", descriptor = "(ILjava/lang/String;I)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getValueString(signum: i32, intString: String, scale: i32) -> Result<String> {
            panic!("stub: java/math/BigDecimal.getValueString:(ILjava/lang/String;I)Ljava/lang/String;")
        }

        #[java_method(name = "toBigInteger", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBigInteger(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigDecimal.toBigInteger:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "toBigIntegerExact", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBigIntegerExact(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigDecimal.toBigIntegerExact:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.longValue:()J")
        }

        #[java_method(name = "fractionOnly", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fractionOnly(&self) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.fractionOnly:()Z")
        }

        #[java_method(name = "longValueExact", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValueExact(&self) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.longValueExact:()J")
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.intValue:()I")
        }

        #[java_method(name = "intValueExact", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValueExact(&self) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.intValueExact:()I")
        }

        #[java_method(name = "shortValueExact", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValueExact(&self) -> Result<i16> {
            panic!("stub: java/math/BigDecimal.shortValueExact:()S")
        }

        #[java_method(name = "byteValueExact", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValueExact(&self) -> Result<i8> {
            panic!("stub: java/math/BigDecimal.byteValueExact:()B")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/math/BigDecimal.floatValue:()F")
        }

        #[java_method(name = "fullFloatValue", descriptor = "()F", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fullFloatValue(&self) -> Result<f32> {
            panic!("stub: java/math/BigDecimal.fullFloatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/math/BigDecimal.doubleValue:()D")
        }

        #[java_method(name = "fullDoubleValue", descriptor = "()D", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fullDoubleValue(&self) -> Result<f64> {
            panic!("stub: java/math/BigDecimal.fullDoubleValue:()D")
        }

        #[java_method(name = "ulp", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ulp(&self) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.ulp:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "layoutChars", descriptor = "(Z)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn layoutChars(&self, sci: bool) -> Result<String> {
            panic!("stub: java/math/BigDecimal.layoutChars:(Z)Ljava/lang/String;")
        }

        #[java_method(name = "bigTenToThe", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bigTenToThe(mut n: i32) -> Result<BigInteger> {
            if (n<0) {
                return Ok(BigInteger::ZERO());
            }
            let mut pows: Rc<RefCell<Vec<BigInteger>>> = BigDecimal::BIG_TEN_POWERS_TABLE();
            if n < (pows.borrow().len() as i32) {
                return Ok(Clone::clone(&pows.borrow()[n as usize]));
            }
            let _t0: BigInteger = BigDecimal::expandBigIntegerTenPowers(n)?;
            return Ok(_t0);
            let _t1 = BigInteger::TEN().pow(n)?;
            Ok(_t1)
        }

        #[java_method(name = "expandBigIntegerTenPowers", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expandBigIntegerTenPowers(n: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigDecimal.expandBigIntegerTenPowers:(I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "longMultiplyPowerTen", descriptor = "(JI)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longMultiplyPowerTen(mut val: i64, mut n: i32) -> Result<i64> {
            if (n<=0) {
                return Ok(val);
            }
            let mut tab: Rc<RefCell<Vec<i64>>> = BigDecimal::LONG_TEN_POWERS_TABLE();
            let mut bounds: Rc<RefCell<Vec<i64>>> = BigDecimal::THRESHOLDS_TABLE();
            let mut tenpower = tab.borrow()[n as usize];
            if (((val>(1i64)) as i32-((val)<(1i64)) as i32)==0) {
                return Ok(tenpower);
            }
            let _t0: i64 = Math::abs_l(val)?;
            if (((_t0>(bounds.borrow()[n as usize])) as i32-((_t0)<(bounds.borrow()[n as usize])) as i32)<=0) {
                return Ok((val).wrapping_mul(tenpower));
            }
            Ok(-9223372036854775808i64)
        }

        #[java_method(name = "bigMultiplyPowerTen", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: bigMultiplyPowerTen(I)Ljava/math/BigInteger;
        pub fn bigMultiplyPowerTen_i(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            if (n<=0) {
                let _t0 = this.inflated()?;
                return Ok(_t0);
            }
            if (((this.__get_intCompact()>(-9223372036854775808i64)) as i32-((this.__get_intCompact())<(-9223372036854775808i64)) as i32)!=0) {
                let _t0: BigInteger = BigDecimal::bigTenToThe(n)?;
                let _t1 = _t0.multiply_l(this.__get_intCompact())?;
                return Ok(_t1);
            }
            let _t0: BigInteger = BigDecimal::bigTenToThe(n)?;
            let _t1 = this.__get_intVal().multiply_bigint(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "inflated", descriptor = "()Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inflated(&self) -> Result<BigInteger> {
            let this = self;
            if _is_jnull(&this.__get_intVal()) {
                return Ok(this.__get_intCompact());
            }
            Ok(this.__get_intVal())
        }

        #[java_method(name = "matchScale", descriptor = "([Ljava/math/BigDecimal;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matchScale(val: Rc<RefCell<Vec<BigDecimal>>>) -> Result<()> {
            panic!("stub: java/math/BigDecimal.matchScale:([Ljava/math/BigDecimal;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/math/BigDecimal.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readObjectNoData", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException")]
        pub fn readObjectNoData(&self) -> Result<()> {
            panic!("stub: java/math/BigDecimal.readObjectNoData:()V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/math/BigDecimal.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "longDigitLength", descriptor = "(J)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longDigitLength(mut x: i64) -> Result<i32> {
            if (((x>(-9223372036854775808i64)) as i32-((x)<(-9223372036854775808i64)) as i32)==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (((x>(0i64)) as i32-((x)<(0i64)) as i32)<0) {
                x = (x).wrapping_neg();
            }
            if (((x>(10i64)) as i32-((x)<(10i64)) as i32)<0) {
                return Ok(1i32);
            }
            let _t0: i32 = Long::numberOfLeadingZeros(x)?;
            let mut r = (((((64i32).wrapping_sub(_t0)).wrapping_add(1i32)).wrapping_mul(1233i32) as u32>>(12i32&0x1f)) as i32);
            let mut tab: Rc<RefCell<Vec<i64>>> = BigDecimal::LONG_TEN_POWERS_TABLE();
            Ok((if (((x>(tab.borrow()[r as usize])) as i32-((x)<(tab.borrow()[r as usize])) as i32)<0) { r } else { (r).wrapping_add(1i32) }))
        }

        #[java_method(name = "bigDigitLength", descriptor = "(Ljava/math/BigInteger;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bigDigitLength(mut b: BigInteger) -> Result<i32> {
            if (b.__get_signum()==0) {
                return Ok(1i32);
            }
            let _t0 = b.bitLength()?;
            let mut r: i32 = ((((((_t0 as i64)).wrapping_add(1i64)).wrapping_mul(646456993i64) as u64).wrapping_shr((31i32&0x3f) as u32) as i64) as i32);
            let _t1: BigInteger = BigDecimal::bigTenToThe(r)?;
            let _t2 = b.compareMagnitude_bigint(Clone::clone(&_t1))?;
            Ok((if (_t2<0) { r } else { (r).wrapping_add(1i32) }))
        }

        #[java_method(name = "checkScale", descriptor = "(J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkScale(J)I
        pub fn checkScale_l(&self, mut val: i64) -> Result<i32> {
            let this = self;
            let mut asInt: i32 = (val as i32);
            asInt = (if (((val>(2147483647i64)) as i32-((val)<(2147483647i64)) as i32)>0) { 874i32 } else { 875i32 });
            let mut b = this.__get_intVal();
            let _t0 = b.signum()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(asInt)
        }

        #[java_method(name = "compactValFor", descriptor = "(Ljava/math/BigInteger;)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compactValFor(mut b: BigInteger) -> Result<i64> {
            let mut m = b.__get_mag();
            let mut len = (m.borrow().len() as i32);
            if (len==0) {
                return Ok(0i64);
            }
            let mut d = m.borrow()[0i32 as usize];
            if (d<0) {
                return Ok(-9223372036854775808i64);
            }
            let mut u = (if len == 2i32 { (((m.borrow()[1i32 as usize] as i64)&(4294967295i64))).wrapping_add(((d as i64)).wrapping_shl((32i32&0x3f) as u32)) } else { ((d as i64)&(4294967295i64)) });
            Ok((if (b.__get_signum()<0) { (u).wrapping_neg() } else { u }))
        }

        #[java_method(name = "longCompareMagnitude", descriptor = "(JJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longCompareMagnitude(mut x: i64, mut y: i64) -> Result<i32> {
            if (((x>(0i64)) as i32-((x)<(0i64)) as i32)<0) {
                x = (x).wrapping_neg();
            }
            if (((y>(0i64)) as i32-((y)<(0i64)) as i32)<0) {
                y = (y).wrapping_neg();
            }
            let _t0: i32 = Long::compare(x, y)?;
            Ok(_t0)
        }

        #[java_method(name = "saturateLong", descriptor = "(J)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn saturateLong(s: i64) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.saturateLong:(J)I")
        }

        #[java_method(name = "print", descriptor = "(Ljava/lang/String;Ljava/math/BigDecimal;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn print(name: String, bd: BigDecimal) -> Result<()> {
            panic!("stub: java/math/BigDecimal.print:(Ljava/lang/String;Ljava/math/BigDecimal;)V")
        }

        #[java_method(name = "audit", descriptor = "()Ljava/math/BigDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn audit(&self) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.audit:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "checkScaleNonZero", descriptor = "(J)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkScaleNonZero(mut val: i64) -> Result<i32> {
            let mut asInt: i32 = (val as i32);
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(asInt)
        }

        #[java_method(name = "checkScale", descriptor = "(JJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkScale(JJ)I
        pub fn checkScale_l_l(mut intCompact: i64, mut val: i64) -> Result<i32> {
            let mut asInt: i32 = (val as i32);
            asInt = (if (((val>(2147483647i64)) as i32-((val)<(2147483647i64)) as i32)>0) { 874i32 } else { 875i32 });
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(asInt)
        }

        #[java_method(name = "checkScale", descriptor = "(Ljava/math/BigInteger;J)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkScale(Ljava/math/BigInteger;J)I
        pub fn checkScale_bigint_l(mut intVal: BigInteger, mut val: i64) -> Result<i32> {
            let mut asInt: i32 = (val as i32);
            asInt = (if (((val>(2147483647i64)) as i32-((val)<(2147483647i64)) as i32)>0) { 874i32 } else { 875i32 });
            let _t0 = intVal.signum()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(asInt)
        }

        #[java_method(name = "doRound", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: doRound(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
        pub fn doRound_bigdec_mathco(mut val: BigDecimal, mut mc: MathContext) -> Result<BigDecimal> {
            let mut mcp = mc.__get_precision();
            let mut wasDivided: i32 = 0i32;
            let mut intVal = val.__get_intVal();
            let mut compactVal = val.__get_intCompact();
            let mut scale = val.__get_scale();
            let _t0 = val.precision()?;
            let mut prec: i32 = _t0;
            let mut mode = mc.__get_roundingMode().__get_oldMode();
            let mut drop = (prec).wrapping_sub(mcp);
            loop {
                if (drop<=0) { break; }
                let _t1: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
                scale = _t1;
                let _t2: BigInteger = BigDecimal::divideAndRoundByTenPow(Clone::clone(&intVal), drop, mode)?;
                intVal = _t2;
                wasDivided = 1i32;
                let _t3: i64 = BigDecimal::compactValFor(Clone::clone(&intVal))?;
                compactVal = _t3;
                if (((compactVal>(-9223372036854775808i64)) as i32-((compactVal)<(-9223372036854775808i64)) as i32)!=0) {
                    let _t4: i32 = BigDecimal::longDigitLength(compactVal)?;
                    prec = _t4;
                    break;
                }
                let _t4: i32 = BigDecimal::bigDigitLength(Clone::clone(&intVal))?;
                prec = _t4;
                drop = (prec).wrapping_sub(mcp);
            }
            drop = (prec).wrapping_sub(mcp);
            let mut intVal: Object = Default::default();
            loop {
                if (drop<=0) { break; }
                let _t1: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
                scale = _t1;
                let _t2: i64 = BigDecimal::divideAndRound_l_l_i(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[drop as usize], mc.__get_roundingMode().__get_oldMode())?;
                compactVal = _t2;
                wasDivided = 1i32;
                let _t3: i32 = BigDecimal::longDigitLength(compactVal)?;
                prec = _t3;
                drop = (prec).wrapping_sub(mcp);
                intVal = Object::default();
            }
            return Ok((if (wasDivided!=0) { BigDecimal::new_bigint_l_i_i(Clone::clone(&intVal), compactVal, scale, prec)? } else { val }));
            Ok(val)
        }

        #[java_method(name = "doRound", descriptor = "(JILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doRound_l_i_mathco(compactVal: i64, arg1: i32, scale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.doRound:(JILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "doRound", descriptor = "(Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doRound_bigint_i_mathco(intVal: BigInteger, scale: i32, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.doRound:(Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideAndRoundByTenPow", descriptor = "(Ljava/math/BigInteger;II)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRoundByTenPow(mut intVal: BigInteger, mut tenPow: i32, mut roundingMode: i32) -> Result<BigInteger> {
            if tenPow < (BigDecimal::LONG_TEN_POWERS_TABLE().borrow().len() as i32) {
                let _t0: BigInteger = BigDecimal::divideAndRound_bigint_l_i(Clone::clone(&intVal), BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[tenPow as usize], roundingMode)?;
                intVal = _t0;
            } else {
                let _t0: BigInteger = BigDecimal::bigTenToThe(tenPow)?;
                let _t1: BigInteger = BigDecimal::divideAndRound_bigint_bigint_i(Clone::clone(&intVal), Clone::clone(&_t0), roundingMode)?;
                intVal = _t1;
            }
            Ok(intVal)
        }

        #[java_method(name = "divideAndRound", descriptor = "(JJIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(JJIII)Ljava/math/BigDecimal;
        pub fn divideAndRound_l_l_i_i_i(mut ldividend: i64, mut ldivisor: i64, mut scale: i32, mut roundingMode: i32, mut preferredScale: i32) -> Result<BigDecimal> {
            let mut q = (ldividend/ldivisor);
            if scale == preferredScale {
                let _t0: BigDecimal = BigDecimal::valueOf_l_i(q, scale)?;
                return Ok(_t0);
            }
            let mut r = (ldividend%(ldivisor));
            let mut qsign = ((((ldividend>(0i64)) as i32-((ldividend)<(0i64)) as i32)<0) != (((ldivisor>(0i64)) as i32-((ldivisor)<(0i64)) as i32)<0)) as i32;
            let _t0: bool = BigDecimal::needIncrement_l_i_i_l_l(ldivisor, roundingMode, qsign, q, r)?;
            let mut increment = (_t0) as i32;
            let _t1: BigDecimal = BigDecimal::valueOf_l_i((if (increment!=0) { (q).wrapping_add((qsign as i64)) } else { q }), scale)?;
            return Ok(_t1);
            if preferredScale != scale {
                let _t2: BigDecimal = BigDecimal::createAndStripZerosToMatchScale_l_i_l(q, scale, (preferredScale as i64))?;
                return Ok(_t2);
            }
            let _t2: BigDecimal = BigDecimal::valueOf_l_i(q, scale)?;
            Ok(_t2)
        }

        #[java_method(name = "divideAndRound", descriptor = "(JJI)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(JJI)J
        pub fn divideAndRound_l_l_i(mut ldividend: i64, mut ldivisor: i64, mut roundingMode: i32) -> Result<i64> {
            let mut q = (ldividend/ldivisor);
            if roundingMode == 1i32 {
                return Ok(q);
            }
            let mut r = (ldividend%(ldivisor));
            let mut qsign = ((((ldividend>(0i64)) as i32-((ldividend)<(0i64)) as i32)<0) != (((ldivisor>(0i64)) as i32-((ldivisor)<(0i64)) as i32)<0)) as i32;
            let _t0: bool = BigDecimal::needIncrement_l_i_i_l_l(ldivisor, roundingMode, qsign, q, r)?;
            let mut increment = (_t0) as i32;
            return Ok((if (increment!=0) { (q).wrapping_add((qsign as i64)) } else { q }));
            Ok(q)
        }

        #[java_method(name = "commonNeedIncrement", descriptor = "(IIIZ)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn commonNeedIncrement(roundingMode: i32, qsign: i32, cmpFracHalf: i32, oddQuot: bool) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.commonNeedIncrement:(IIIZ)Z")
        }

        #[java_method(name = "needIncrement", descriptor = "(JIIJJ)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: needIncrement(JIIJJ)Z
        pub fn needIncrement_l_i_i_l_l(mut ldivisor: i64, mut roundingMode: i32, mut qsign: i32, mut q: i64, mut r: i64) -> Result<bool> {
            if (((r>(0i64)) as i32-((r)<(0i64)) as i32)==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
        let mut cmpFracHalf: i32 = Default::default();
            if (((r>(4611686018427387903i64)) as i32-((r)<(4611686018427387903i64)) as i32)>0) {
                cmpFracHalf = 1i32;
            } else {
                let _t0: i32 = BigDecimal::longCompareMagnitude((2i64).wrapping_mul(r), ldivisor)?;
                cmpFracHalf = _t0;
            }
            let _t0: bool = BigDecimal::commonNeedIncrement(roundingMode, qsign, cmpFracHalf, ((((q&(1i64))>(0i64)) as i32-(((q&(1i64)))<(0i64)) as i32)!=0))?;
            Ok(_t0)
        }

        #[java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;JI)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(Ljava/math/BigInteger;JI)Ljava/math/BigInteger;
        pub fn divideAndRound_bigint_l_i(mut bdividend: BigInteger, mut ldivisor: i64, mut roundingMode: i32) -> Result<BigInteger> {
            let mut mdividend = MutableBigInteger::new_arr_i(Clone::clone(&bdividend.__get_mag()))?;
            let mut mq = MutableBigInteger::new()?;
            let _t0 = mdividend.divide_l_mutabl(ldivisor, Clone::clone(&mq))?;
            let mut r: i64 = _t0;
            let mut isRemainderZero = ((((r>(0i64)) as i32-((r)<(0i64)) as i32)==0)) as i32;
            let mut qsign = (if (((ldivisor>(0i64)) as i32-((ldivisor)<(0i64)) as i32)<0) { (bdividend.__get_signum()).wrapping_neg() } else { bdividend.__get_signum() });
            let _t1: bool = BigDecimal::needIncrement_l_i_i_mutabl_l(ldivisor, roundingMode, qsign, Clone::clone(&mq), r)?;
            if _t1 {
                mq.add(Clone::clone(&MutableBigInteger::ONE()))?;
            }
            let _t2 = mq.toBigInteger_i(qsign)?;
            Ok(_t2)
        }

        #[java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;JIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(Ljava/math/BigInteger;JIII)Ljava/math/BigDecimal;
        pub fn divideAndRound_bigint_l_i_i_i(mut bdividend: BigInteger, mut ldivisor: i64, mut scale: i32, mut roundingMode: i32, mut preferredScale: i32) -> Result<BigDecimal> {
            let mut mdividend = MutableBigInteger::new_arr_i(Clone::clone(&bdividend.__get_mag()))?;
            let mut mq = MutableBigInteger::new()?;
            let _t0 = mdividend.divide_l_mutabl(ldivisor, Clone::clone(&mq))?;
            let mut r: i64 = _t0;
            let mut isRemainderZero = ((((r>(0i64)) as i32-((r)<(0i64)) as i32)==0)) as i32;
            let mut qsign = (if (((ldivisor>(0i64)) as i32-((ldivisor)<(0i64)) as i32)<0) { (bdividend.__get_signum()).wrapping_neg() } else { bdividend.__get_signum() });
            let _t1: bool = BigDecimal::needIncrement_l_i_i_mutabl_l(ldivisor, roundingMode, qsign, Clone::clone(&mq), r)?;
            if _t1 {
                mq.add(Clone::clone(&MutableBigInteger::ONE()))?;
            }
            let _t2 = mq.toBigDecimal(qsign, scale)?;
            return Ok(_t2);
            let _t3 = mq.toCompactValue(qsign)?;
            let mut compactVal: i64 = _t3;
            if (((compactVal>(-9223372036854775808i64)) as i32-((compactVal)<(-9223372036854775808i64)) as i32)!=0) {
                let _t4: BigDecimal = BigDecimal::createAndStripZerosToMatchScale_l_i_l(compactVal, scale, (preferredScale as i64))?;
                return Ok(_t4);
            }
            let _t4 = mq.toBigInteger_i(qsign)?;
            let mut intVal: BigInteger = _t4;
            let _t5: BigDecimal = BigDecimal::createAndStripZerosToMatchScale_bigint_i_l(Clone::clone(&intVal), scale, (preferredScale as i64))?;
            return Ok(_t5);
            let _t6 = mq.toBigDecimal(qsign, scale)?;
            Ok(_t6)
        }

        #[java_method(name = "needIncrement", descriptor = "(JIILjava/math/MutableBigInteger;J)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: needIncrement(JIILjava/math/MutableBigInteger;J)Z
        pub fn needIncrement_l_i_i_mutabl_l(mut ldivisor: i64, mut roundingMode: i32, mut qsign: i32, mut mq: MutableBigInteger, mut r: i64) -> Result<bool> {
            if (((r>(0i64)) as i32-((r)<(0i64)) as i32)==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
        let mut cmpFracHalf: i32 = Default::default();
            if (((r>(4611686018427387903i64)) as i32-((r)<(4611686018427387903i64)) as i32)>0) {
                cmpFracHalf = 1i32;
            } else {
                let _t0: i32 = BigDecimal::longCompareMagnitude((2i64).wrapping_mul(r), ldivisor)?;
                cmpFracHalf = _t0;
            }
            let _t0 = mq.isOdd()?;
            let _t1: bool = BigDecimal::commonNeedIncrement(roundingMode, qsign, cmpFracHalf, _t0)?;
            Ok(_t1)
        }

        #[java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigInteger;
        pub fn divideAndRound_bigint_bigint_i(mut bdividend: BigInteger, mut bdivisor: BigInteger, mut roundingMode: i32) -> Result<BigInteger> {
            let mut mdividend = MutableBigInteger::new_arr_i(Clone::clone(&bdividend.__get_mag()))?;
            let mut mq = MutableBigInteger::new()?;
            let mut mdivisor = MutableBigInteger::new_arr_i(Clone::clone(&bdivisor.__get_mag()))?;
            let _t0 = mdividend.divide_mutabl_mutabl(Clone::clone(&mdivisor), Clone::clone(&mq))?;
            let mut mr: MutableBigInteger = _t0;
            let _t1 = mr.isZero()?;
            let mut isRemainderZero = (_t1) as i32;
            let mut qsign = (bdividend.__get_signum() == bdivisor.__get_signum()) as i32;
            let _t2: bool = BigDecimal::needIncrement_mutabl_i_i_mutabl_mutabl(Clone::clone(&mdivisor), roundingMode, qsign, Clone::clone(&mq), Clone::clone(&mr))?;
            if _t2 {
                mq.add(Clone::clone(&MutableBigInteger::ONE()))?;
            }
            let _t3 = mq.toBigInteger_i(qsign)?;
            Ok(_t3)
        }

        #[java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;III)Ljava/math/BigDecimal;
        pub fn divideAndRound_bigint_bigint_i_i_i(mut bdividend: BigInteger, mut bdivisor: BigInteger, mut scale: i32, mut roundingMode: i32, mut preferredScale: i32) -> Result<BigDecimal> {
            let mut mdividend = MutableBigInteger::new_arr_i(Clone::clone(&bdividend.__get_mag()))?;
            let mut mq = MutableBigInteger::new()?;
            let mut mdivisor = MutableBigInteger::new_arr_i(Clone::clone(&bdivisor.__get_mag()))?;
            let _t0 = mdividend.divide_mutabl_mutabl(Clone::clone(&mdivisor), Clone::clone(&mq))?;
            let mut mr: MutableBigInteger = _t0;
            let _t1 = mr.isZero()?;
            let mut isRemainderZero = (_t1) as i32;
            let mut qsign = (bdividend.__get_signum() == bdivisor.__get_signum()) as i32;
            let _t2: bool = BigDecimal::needIncrement_mutabl_i_i_mutabl_mutabl(Clone::clone(&mdivisor), roundingMode, qsign, Clone::clone(&mq), Clone::clone(&mr))?;
            if _t2 {
                mq.add(Clone::clone(&MutableBigInteger::ONE()))?;
            }
            let _t3 = mq.toBigDecimal(qsign, scale)?;
            return Ok(_t3);
            let _t4 = mq.toCompactValue(qsign)?;
            let mut compactVal: i64 = _t4;
            if (((compactVal>(-9223372036854775808i64)) as i32-((compactVal)<(-9223372036854775808i64)) as i32)!=0) {
                let _t5: BigDecimal = BigDecimal::createAndStripZerosToMatchScale_l_i_l(compactVal, scale, (preferredScale as i64))?;
                return Ok(_t5);
            }
            let _t5 = mq.toBigInteger_i(qsign)?;
            let mut intVal: BigInteger = _t5;
            let _t6: BigDecimal = BigDecimal::createAndStripZerosToMatchScale_bigint_i_l(Clone::clone(&intVal), scale, (preferredScale as i64))?;
            return Ok(_t6);
            let _t7 = mq.toBigDecimal(qsign, scale)?;
            Ok(_t7)
        }

        #[java_method(name = "needIncrement", descriptor = "(Ljava/math/MutableBigInteger;IILjava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: needIncrement(Ljava/math/MutableBigInteger;IILjava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Z
        pub fn needIncrement_mutabl_i_i_mutabl_mutabl(mut mdivisor: MutableBigInteger, mut roundingMode: i32, mut qsign: i32, mut mq: MutableBigInteger, mut mr: MutableBigInteger) -> Result<bool> {
            let _t0 = mr.isZero()?;
            if _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1 = mr.compareHalf(Clone::clone(&mdivisor))?;
            let mut cmpFracHalf: i32 = _t1;
            let _t2 = mq.isOdd()?;
            let _t3: bool = BigDecimal::commonNeedIncrement(roundingMode, qsign, cmpFracHalf, _t2)?;
            Ok(_t3)
        }

        #[java_method(name = "createAndStripZerosToMatchScale", descriptor = "(Ljava/math/BigInteger;IJ)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: createAndStripZerosToMatchScale(Ljava/math/BigInteger;IJ)Ljava/math/BigDecimal;
        pub fn createAndStripZerosToMatchScale_bigint_i_l(mut intVal: BigInteger, mut scale: i32, mut preferredScale: i64) -> Result<BigDecimal> {
            loop {
                let _t0 = intVal.compareMagnitude_bigint(Clone::clone(&BigInteger::TEN()))?;
                if (_t0<0) { break; }
                let _t0 = intVal.testBit(0i32)?;
                if _t0 {
                    break;
                }
                let _t1 = intVal.divideAndRemainder(Clone::clone(&BigInteger::TEN()))?;
                let mut qr: Rc<RefCell<Vec<BigInteger>>> = _t1;
                let _t2 = Clone::clone(&qr.borrow()[1i32 as usize]).signum()?;
                if (_t2!=0) {
                    break;
                }
                intVal = Clone::clone(&qr.borrow()[0i32 as usize]);
                let _t3: i32 = BigDecimal::checkScale_bigint_l(Clone::clone(&intVal), ((scale as i64)).wrapping_sub(1i64))?;
                scale = _t3;
            }
            let _t0: BigDecimal = BigDecimal::valueOf_bigint_i_i(Clone::clone(&intVal), scale, 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "createAndStripZerosToMatchScale", descriptor = "(JIJ)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: createAndStripZerosToMatchScale(JIJ)Ljava/math/BigDecimal;
        pub fn createAndStripZerosToMatchScale_l_i_l(mut compactVal: i64, mut scale: i32, mut preferredScale: i64) -> Result<BigDecimal> {
            loop {
                let _t0: i64 = Math::abs_l(compactVal)?;
                if (((_t0>(10i64)) as i32-((_t0)<(10i64)) as i32)<0) { break; }
                if ((((compactVal&(1i64))>(0i64)) as i32-(((compactVal&(1i64)))<(0i64)) as i32)!=0) {
                    break;
                }
                let mut r = (compactVal%(10i64));
                if (((r>(0i64)) as i32-((r)<(0i64)) as i32)!=0) {
                    break;
                }
                compactVal = (compactVal/10i64);
                let _t0: i32 = BigDecimal::checkScale_l_l(compactVal, ((scale as i64)).wrapping_sub(1i64))?;
                scale = _t0;
            }
            let _t0: BigDecimal = BigDecimal::valueOf_l_i(compactVal, scale)?;
            Ok(_t0)
        }

        #[java_method(name = "stripZerosToMatchScale", descriptor = "(Ljava/math/BigInteger;JII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripZerosToMatchScale(intVal: BigInteger, intCompact: i64, arg2: i32, scale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.stripZerosToMatchScale:(Ljava/math/BigInteger;JII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(JJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_l_l(xs: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.add:(JJ)J")
        }

        #[java_method(name = "add", descriptor = "(JJI)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_l_l_i(xs: i64, arg1: i64, ys: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(JJI)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(JIJI)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_l_i_l_i(xs: i64, arg1: i32, scale1: i64, ys: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(JIJI)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(JILjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_l_i_bigint_i(xs: i64, arg1: i32, scale1: BigInteger, snd: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(JILjava/math/BigInteger;I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "add", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_bigint_i_bigint_i(fst: BigInteger, scale1: i32, snd: BigInteger, scale2: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.add:(Ljava/math/BigInteger;ILjava/math/BigInteger;I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "bigMultiplyPowerTen", descriptor = "(JI)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bigMultiplyPowerTen_l_i(value: i64, arg1: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigDecimal.bigMultiplyPowerTen:(JI)Ljava/math/BigInteger;")
        }

        #[java_method(name = "bigMultiplyPowerTen", descriptor = "(Ljava/math/BigInteger;I)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: bigMultiplyPowerTen(Ljava/math/BigInteger;I)Ljava/math/BigInteger;
        pub fn bigMultiplyPowerTen_bigint_i(mut value: BigInteger, mut n: i32) -> Result<BigInteger> {
            if (n<=0) {
                return Ok(value);
            }
            if n < (BigDecimal::LONG_TEN_POWERS_TABLE().borrow().len() as i32) {
                let _t0 = value.multiply_l(BigDecimal::LONG_TEN_POWERS_TABLE().borrow()[n as usize])?;
                return Ok(_t0);
            }
            let _t0: BigInteger = BigDecimal::bigTenToThe(n)?;
            let _t1 = value.multiply_bigint(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "divideSmallFastPath", descriptor = "(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideSmallFastPath(xs: i64, arg1: i32, xscale: i64, ys: i32, arg4: i64, yscale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divideSmallFastPath:(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_l_i_l_i_l_mathco(xs: i64, arg1: i32, xscale: i64, ys: i32, arg4: i64, yscale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;IJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigint_i_l_i_l_mathco(xs: BigInteger, xscale: i32, ys: i64, arg3: i32, yscale: i64, preferredScale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigInteger;IJIJLjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(JILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_l_i_bigint_i_l_mathco(xs: i64, arg1: i32, xscale: BigInteger, ys: i32, yscale: i64, preferredScale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(JILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigint_i_bigint_i_l_mathco(xs: BigInteger, xscale: i32, ys: BigInteger, yscale: i32, preferredScale: i64, arg5: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigInteger;ILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiplyDivideAndRound", descriptor = "(JJJIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyDivideAndRound(dividend0: i64, arg1: i64, dividend1: i64, arg3: i32, divisor: i32, arg5: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiplyDivideAndRound:(JJJIII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divideAndRound128", descriptor = "(JJJIIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRound128(dividendHi: i64, arg1: i64, dividendLo: i64, arg3: i32, divisor: i32, arg5: i32, sign: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divideAndRound128:(JJJIIII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "roundedTenPower", descriptor = "(IIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roundedTenPower(qsign: i32, raise: i32, scale: i32, preferredScale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.roundedTenPower:(IIII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "scaledTenPow", descriptor = "(III)Ljava/math/BigDecimal;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scaledTenPow(n: i32, sign: i32, scale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.scaledTenPow:(III)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divRemNegativeLong", descriptor = "(JJ)[J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divRemNegativeLong(n: i64, arg1: i64) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/math/BigDecimal.divRemNegativeLong:(JJ)[J")
        }

        #[java_method(name = "make64", descriptor = "(JJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn make64(hi: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.make64:(JJ)J")
        }

        #[java_method(name = "mulsub", descriptor = "(JJJJJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mulsub(u1: i64, arg1: i64, u0: i64, arg3: i64, v1: i64) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.mulsub:(JJJJJ)J")
        }

        #[java_method(name = "unsignedLongCompare", descriptor = "(JJ)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unsignedLongCompare(one: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.unsignedLongCompare:(JJ)Z")
        }

        #[java_method(name = "unsignedLongCompareEq", descriptor = "(JJ)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unsignedLongCompareEq(one: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.unsignedLongCompareEq:(JJ)Z")
        }

        #[java_method(name = "compareMagnitudeNormalized", descriptor = "(JIJI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareMagnitudeNormalized_l_i_l_i(xs: i64, arg1: i32, xscale: i64, ys: i32) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.compareMagnitudeNormalized:(JIJI)I")
        }

        #[java_method(name = "compareMagnitudeNormalized", descriptor = "(JILjava/math/BigInteger;I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareMagnitudeNormalized_l_i_bigint_i(xs: i64, arg1: i32, xscale: BigInteger, ys: i32) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.compareMagnitudeNormalized:(JILjava/math/BigInteger;I)I")
        }

        #[java_method(name = "compareMagnitudeNormalized", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareMagnitudeNormalized_bigint_i_bigint_i(xs: BigInteger, xscale: i32, ys: BigInteger, yscale: i32) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.compareMagnitudeNormalized:(Ljava/math/BigInteger;ILjava/math/BigInteger;I)I")
        }

        #[java_method(name = "multiply", descriptor = "(JJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/math/BigDecimal.multiply:(JJ)J")
        }

        #[java_method(name = "multiply", descriptor = "(JJI)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_l_l_i(x: i64, arg1: i64, y: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiply:(JJI)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiply", descriptor = "(JLjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_l_bigint_i(x: i64, arg1: BigInteger, y: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiply:(JLjava/math/BigInteger;I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply_bigint_bigint_i(x: BigInteger, y: BigInteger, scale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiply:(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiplyAndRound", descriptor = "(JJILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyAndRound_l_l_i_mathco(x: i64, arg1: i64, y: i32, arg3: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiplyAndRound:(JJILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiplyAndRound", descriptor = "(JLjava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyAndRound_l_bigint_i_mathco(x: i64, arg1: BigInteger, y: i32, scale: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiplyAndRound:(JLjava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "multiplyAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyAndRound_bigint_bigint_i_mathco(x: BigInteger, y: BigInteger, scale: i32, mc: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.multiplyAndRound:(Ljava/math/BigInteger;Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "doRound128", descriptor = "(JJIILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doRound128(hi: i64, arg1: i64, lo: i32, arg3: i32, sign: MathContext) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.doRound128:(JJIILjava/math/MathContext;)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "precision", descriptor = "(JJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn precision_l_l(hi: i64, arg1: i64) -> Result<i32> {
            panic!("stub: java/math/BigDecimal.precision:(JJ)I")
        }

        #[java_method(name = "longLongCompareMagnitude", descriptor = "(JJJJ)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longLongCompareMagnitude(hi0: i64, arg1: i64, lo0: i64, arg3: i64) -> Result<bool> {
            panic!("stub: java/math/BigDecimal.longLongCompareMagnitude:(JJJJ)Z")
        }

        #[java_method(name = "divide", descriptor = "(JIJIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_l_i_l_i_i_i(dividend: i64, arg1: i32, dividendScale: i64, divisor: i32, arg4: i32, divisorScale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(JIJIII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;IJIII)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigint_i_l_i_i_i(dividend: BigInteger, dividendScale: i32, divisor: i64, arg3: i32, divisorScale: i32, scale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigInteger;IJIII)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(JILjava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_l_i_bigint_i_i_i(dividend: i64, arg1: i32, dividendScale: BigInteger, divisor: i32, divisorScale: i32, scale: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(JILjava/math/BigInteger;III)Ljava/math/BigDecimal;")
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide_bigint_i_bigint_i_i_i(dividend: BigInteger, dividendScale: i32, divisor: BigInteger, divisorScale: i32, scale: i32, roundingMode: i32) -> Result<BigDecimal> {
            panic!("stub: java/math/BigDecimal.divide:(Ljava/math/BigInteger;ILjava/math/BigInteger;III)Ljava/math/BigDecimal;")
        }
    }
}
