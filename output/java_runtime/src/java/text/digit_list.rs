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
    #[binary_name       = "java/text/DigitList"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DigitList.java"]
    #[inner_classes     = "jdk/internal/math/FloatingDecimal$BinaryToASCIIConverter:jdk/internal/math/FloatingDecimal:BinaryToASCIIConverter:1545;java/text/DigitList$1:::4104"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Cloneable;java/lang/Object;java/text/DigitList"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct DigitList {
        #[cfg_attr(any(), java_field(name = "decimalAt", descriptor = "I", access = "public", modifiers = "", is_static = false))]
        pub decimalAt: i32,
        #[cfg_attr(any(), java_field(name = "count", descriptor = "I", access = "public", modifiers = "", is_static = false))]
        pub count: i32,
        #[cfg_attr(any(), java_field(name = "digits", descriptor = "[C", access = "public", modifiers = "", is_static = false))]
        pub digits: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "data", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub data: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "roundingMode", descriptor = "Ljava/math/RoundingMode;", access = "private", modifiers = "", is_static = false))]
        pub roundingMode: RoundingMode,
        #[cfg_attr(any(), java_field(name = "isNegative", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub isNegative: bool,
        #[cfg_attr(any(), java_field(name = "tempBuilder", descriptor = "Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static = false))]
        pub tempBuilder: StringBuilder,
    }

    impl DigitList {
        #[cfg_attr(any(), java_field(name = "MAX_COUNT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: MAX_COUNT:I
        pub fn MAX_COUNT() -> i32 {
            19
        }

        #[cfg_attr(any(), java_field(name = "LONG_MIN_REP", descriptor = "[C", access = "private", modifiers = "static final", is_static = true))]
        // static field: LONG_MIN_REP:[C
        pub fn LONG_MIN_REP() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/text/DigitList.LONG_MIN_REP:[C")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_decimalAt(0i32);
            this.__set_count(0i32);
            let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 19i32 as usize]));
            this.__set_digits(Clone::clone(&_arr0));
            this.__set_roundingMode(Clone::clone(&RoundingMode::HALF_EVEN()));
            this.__set_isNegative((0i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "isZero", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isZero(&self) -> Result<bool> {
            panic!("stub: java/text/DigitList.isZero:()Z")
        }

        #[java_method(name = "setRoundingMode", descriptor = "(Ljava/math/RoundingMode;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRoundingMode(&self, r: RoundingMode) -> Result<()> {
            panic!("stub: java/text/DigitList.setRoundingMode:(Ljava/math/RoundingMode;)V")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/text/DigitList.clear:()V")
        }

        #[java_method(name = "append", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append(&self, digit: u16) -> Result<()> {
            panic!("stub: java/text/DigitList.append:(C)V")
        }

        #[java_method(name = "getDouble", descriptor = "()D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble(&self) -> Result<f64> {
            panic!("stub: java/text/DigitList.getDouble:()D")
        }

        #[java_method(name = "getLong", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self) -> Result<i64> {
            panic!("stub: java/text/DigitList.getLong:()J")
        }

        #[java_method(name = "getBigDecimal", descriptor = "()Ljava/math/BigDecimal;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBigDecimal(&self) -> Result<BigDecimal> {
            panic!("stub: java/text/DigitList.getBigDecimal:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "fitsIntoLong", descriptor = "(ZZ)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fitsIntoLong(&self, isPositive: bool, ignoreNegativeZero: bool) -> Result<bool> {
            panic!("stub: java/text/DigitList.fitsIntoLong:(ZZ)Z")
        }

        #[java_method(name = "set", descriptor = "(ZDI)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_d_i(&self, isNegative: bool, source: f64, arg2: i32) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZDI)V")
        }

        #[java_method(name = "set", descriptor = "(ZDIZ)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_d_i_z(&self, isNegative: bool, source: f64, arg2: i32, maximumDigits: bool) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZDIZ)V")
        }

        #[java_method(name = "set", descriptor = "(ZLjava/lang/String;ZZIZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_str_z_z_i_z(&self, isNegative: bool, s: String, roundedUp: bool, valueExactAsDecimal: bool, maximumDigits: i32, fixedPoint: bool) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZLjava/lang/String;ZZIZ)V")
        }

        #[java_method(name = "round", descriptor = "(IZZ)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round(&self, maximumDigits: i32, alreadyRounded: bool, valueExactAsDecimal: bool) -> Result<()> {
            panic!("stub: java/text/DigitList.round:(IZZ)V")
        }

        #[java_method(name = "shouldRoundUp", descriptor = "(IZZ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shouldRoundUp(&self, maximumDigits: i32, alreadyRounded: bool, valueExactAsDecimal: bool) -> Result<bool> {
            panic!("stub: java/text/DigitList.shouldRoundUp:(IZZ)Z")
        }

        #[java_method(name = "set", descriptor = "(ZJ)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_l(&self, isNegative: bool, source: i64) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZJ)V")
        }

        #[java_method(name = "set", descriptor = "(ZJI)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_l_i(&self, isNegative: bool, source: i64, arg2: i32) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZJI)V")
        }

        #[java_method(name = "set", descriptor = "(ZLjava/math/BigDecimal;IZ)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_bigdec_i_z(&self, isNegative: bool, source: BigDecimal, maximumDigits: i32, fixedPoint: bool) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZLjava/math/BigDecimal;IZ)V")
        }

        #[java_method(name = "set", descriptor = "(ZLjava/math/BigInteger;I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_z_bigint_i(&self, isNegative: bool, source: BigInteger, maximumDigits: i32) -> Result<()> {
            panic!("stub: java/text/DigitList.set:(ZLjava/math/BigInteger;I)V")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/DigitList.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/DigitList.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "isLongMIN_VALUE", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLongMIN_VALUE(&self) -> Result<bool> {
            panic!("stub: java/text/DigitList.isLongMIN_VALUE:()Z")
        }

        #[java_method(name = "parseInt", descriptor = "([CII)I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseInt(str: Rc<RefCell<Vec<u16>>>, offset: i32, strLen: i32) -> Result<i32> {
            panic!("stub: java/text/DigitList.parseInt:([CII)I")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getStringBuilder", descriptor = "()Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStringBuilder(&self) -> Result<StringBuilder> {
            panic!("stub: java/text/DigitList.getStringBuilder:()Ljava/lang/StringBuilder;")
        }

        #[java_method(name = "extendDigits", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn extendDigits(&self, len: i32) -> Result<()> {
            panic!("stub: java/text/DigitList.extendDigits:(I)V")
        }

        #[java_method(name = "getDataChars", descriptor = "(I)[C", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDataChars(&self, length: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/text/DigitList.getDataChars:(I)[C")
        }
    }
}
