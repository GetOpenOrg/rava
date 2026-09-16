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

impl From<BigInteger> for Number {
    fn from(v: BigInteger) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/math/BigInteger"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/math/BigInteger;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BigInteger.java"]
    #[inner_classes     = "java/math/BigInteger$RecursiveOp:java/math/BigInteger:RecursiveOp:1034;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/math/BigInteger$UnsafeHolder:java/math/BigInteger:UnsafeHolder:10;java/io/ObjectOutputStream$PutField:java/io/ObjectOutputStream:PutField:1033;java/math/BigInteger$RecursiveOp$RecursiveSquare:java/math/BigInteger$RecursiveOp:RecursiveSquare:26;java/math/BigInteger$RecursiveOp$RecursiveMultiply:java/math/BigInteger$RecursiveOp:RecursiveMultiply:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Number;java/lang/Object;java/math/BigInteger"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct BigInteger {
        #[cfg_attr(any(), java_field(name = "signum", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub signum: i32,
        #[cfg_attr(any(), java_field(name = "mag", descriptor = "[I", access = "package", modifiers = "final", is_static = false))]
        pub mag: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "bitCountPlusOne", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub bitCountPlusOne: i32,
        #[cfg_attr(any(), java_field(name = "bitLengthPlusOne", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub bitLengthPlusOne: i32,
        #[cfg_attr(any(), java_field(name = "lowestSetBitPlusTwo", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub lowestSetBitPlusTwo: i32,
        #[cfg_attr(any(), java_field(name = "firstNonzeroIntNumPlusTwo", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub firstNonzeroIntNumPlusTwo: i32,
    }

    impl BigInteger {
        #[cfg_attr(any(), java_field(name = "LONG_MASK", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "4294967295"))]
        // static field: LONG_MASK:J
        pub fn LONG_MASK() -> i64 {
            4294967295i64
        }

        #[cfg_attr(any(), java_field(name = "MAX_MAG_LENGTH", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "67108864"))]
        // static field: MAX_MAG_LENGTH:I
        pub fn MAX_MAG_LENGTH() -> i32 {
            67108864
        }

        #[cfg_attr(any(), java_field(name = "PRIME_SEARCH_BIT_LENGTH_LIMIT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "500000000"))]
        // static field: PRIME_SEARCH_BIT_LENGTH_LIMIT:I
        pub fn PRIME_SEARCH_BIT_LENGTH_LIMIT() -> i32 {
            500000000
        }

        #[cfg_attr(any(), java_field(name = "KARATSUBA_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "80"))]
        // static field: KARATSUBA_THRESHOLD:I
        pub fn KARATSUBA_THRESHOLD() -> i32 {
            80
        }

        #[cfg_attr(any(), java_field(name = "TOOM_COOK_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "240"))]
        // static field: TOOM_COOK_THRESHOLD:I
        pub fn TOOM_COOK_THRESHOLD() -> i32 {
            240
        }

        #[cfg_attr(any(), java_field(name = "KARATSUBA_SQUARE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: KARATSUBA_SQUARE_THRESHOLD:I
        pub fn KARATSUBA_SQUARE_THRESHOLD() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "TOOM_COOK_SQUARE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "216"))]
        // static field: TOOM_COOK_SQUARE_THRESHOLD:I
        pub fn TOOM_COOK_SQUARE_THRESHOLD() -> i32 {
            216
        }

        #[cfg_attr(any(), java_field(name = "BURNIKEL_ZIEGLER_THRESHOLD", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "80"))]
        // static field: BURNIKEL_ZIEGLER_THRESHOLD:I
        pub fn BURNIKEL_ZIEGLER_THRESHOLD() -> i32 {
            80
        }

        #[cfg_attr(any(), java_field(name = "BURNIKEL_ZIEGLER_OFFSET", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "40"))]
        // static field: BURNIKEL_ZIEGLER_OFFSET:I
        pub fn BURNIKEL_ZIEGLER_OFFSET() -> i32 {
            40
        }

        #[cfg_attr(any(), java_field(name = "SCHOENHAGE_BASE_CONVERSION_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "20"))]
        // static field: SCHOENHAGE_BASE_CONVERSION_THRESHOLD:I
        pub fn SCHOENHAGE_BASE_CONVERSION_THRESHOLD() -> i32 {
            20
        }

        #[cfg_attr(any(), java_field(name = "MULTIPLY_SQUARE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "20"))]
        // static field: MULTIPLY_SQUARE_THRESHOLD:I
        pub fn MULTIPLY_SQUARE_THRESHOLD() -> i32 {
            20
        }

        #[cfg_attr(any(), java_field(name = "MONTGOMERY_INTRINSIC_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: MONTGOMERY_INTRINSIC_THRESHOLD:I
        pub fn MONTGOMERY_INTRINSIC_THRESHOLD() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "bitsPerDigit", descriptor = "[J", access = "private", modifiers = "static", is_static = true))]
        // static field: bitsPerDigit:[J
        pub fn bitsPerDigit() -> Rc<RefCell<Vec<i64>>> {
            panic!("stub: java/math/BigInteger.bitsPerDigit:[J")
        }

        #[cfg_attr(any(), java_field(name = "SMALL_PRIME_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "95"))]
        // static field: SMALL_PRIME_THRESHOLD:I
        pub fn SMALL_PRIME_THRESHOLD() -> i32 {
            95
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_PRIME_CERTAINTY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "100"))]
        // static field: DEFAULT_PRIME_CERTAINTY:I
        pub fn DEFAULT_PRIME_CERTAINTY() -> i32 {
            100
        }

        #[cfg_attr(any(), java_field(name = "SMALL_PRIME_PRODUCT", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: SMALL_PRIME_PRODUCT:Ljava/math/BigInteger;
        pub fn SMALL_PRIME_PRODUCT() -> BigInteger {
            panic!("stub: java/math/BigInteger.SMALL_PRIME_PRODUCT:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "MAX_CONSTANT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: MAX_CONSTANT:I
        pub fn MAX_CONSTANT() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "posConst", descriptor = "[Ljava/math/BigInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: posConst:[Ljava/math/BigInteger;
        pub fn posConst() -> Rc<RefCell<Vec<BigInteger>>> {
            panic!("stub: java/math/BigInteger.posConst:[Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "negConst", descriptor = "[Ljava/math/BigInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: negConst:[Ljava/math/BigInteger;
        pub fn negConst() -> Rc<RefCell<Vec<BigInteger>>> {
            panic!("stub: java/math/BigInteger.negConst:[Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "powerCache", descriptor = "[[Ljava/math/BigInteger;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: powerCache:[[Ljava/math/BigInteger;
        pub fn powerCache() -> Rc<RefCell<Vec<Rc<RefCell<Vec<BigInteger>>>>>> {
            panic!("stub: java/math/BigInteger.powerCache:[[Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "logCache", descriptor = "[D", access = "private", modifiers = "static final", is_static = true))]
        // static field: logCache:[D
        pub fn logCache() -> Rc<RefCell<Vec<f64>>> {
            panic!("stub: java/math/BigInteger.logCache:[D")
        }

        #[cfg_attr(any(), java_field(name = "LOG_TWO", descriptor = "D", access = "private", modifiers = "static final", is_static = true))]
        // static field: LOG_TWO:D
        pub fn LOG_TWO() -> f64 {
            panic!("stub: java/math/BigInteger.LOG_TWO:D")
        }

        #[cfg_attr(any(), java_field(name = "ZERO", descriptor = "Ljava/math/BigInteger;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ZERO:Ljava/math/BigInteger;
        pub fn ZERO() -> BigInteger {
            panic!("stub: java/math/BigInteger.ZERO:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "ONE", descriptor = "Ljava/math/BigInteger;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ONE:Ljava/math/BigInteger;
        pub fn ONE() -> BigInteger {
            panic!("stub: java/math/BigInteger.ONE:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "TWO", descriptor = "Ljava/math/BigInteger;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TWO:Ljava/math/BigInteger;
        pub fn TWO() -> BigInteger {
            panic!("stub: java/math/BigInteger.TWO:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "NEGATIVE_ONE", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: NEGATIVE_ONE:Ljava/math/BigInteger;
        pub fn NEGATIVE_ONE() -> BigInteger {
            panic!("stub: java/math/BigInteger.NEGATIVE_ONE:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "TEN", descriptor = "Ljava/math/BigInteger;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TEN:Ljava/math/BigInteger;
        pub fn TEN() -> BigInteger {
            panic!("stub: java/math/BigInteger.TEN:Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "bnExpModThreshTable", descriptor = "[I", access = "package", modifiers = "static", is_static = true))]
        // static field: bnExpModThreshTable:[I
        pub fn bnExpModThreshTable() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/math/BigInteger.bnExpModThreshTable:[I")
        }

        #[cfg_attr(any(), java_field(name = "NUM_ZEROS", descriptor = "I", access = "private", modifiers = "static", is_static = true))]
        // static field: NUM_ZEROS:I
        pub fn NUM_ZEROS() -> i32 {
            63
        }

        #[cfg_attr(any(), java_field(name = "ZEROS", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ZEROS:Ljava/lang/String;
        pub fn ZEROS() -> String {
            panic!("stub: java/math/BigInteger.ZEROS:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "digitsPerLong", descriptor = "[I", access = "private", modifiers = "static", is_static = true))]
        // static field: digitsPerLong:[I
        pub fn digitsPerLong() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/math/BigInteger.digitsPerLong:[I")
        }

        #[cfg_attr(any(), java_field(name = "longRadix", descriptor = "[Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static = true))]
        // static field: longRadix:[Ljava/math/BigInteger;
        pub fn longRadix() -> Rc<RefCell<Vec<BigInteger>>> {
            panic!("stub: java/math/BigInteger.longRadix:[Ljava/math/BigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "digitsPerInt", descriptor = "[I", access = "private", modifiers = "static", is_static = true))]
        // static field: digitsPerInt:[I
        pub fn digitsPerInt() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/math/BigInteger.digitsPerInt:[I")
        }

        #[cfg_attr(any(), java_field(name = "intRadix", descriptor = "[I", access = "private", modifiers = "static", is_static = true))]
        // static field: intRadix:[I
        pub fn intRadix() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/math/BigInteger.intRadix:[I")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-8287574255936472291"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -8287574255936472291i64
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/math/BigInteger.serialPersistentFields:[Ljava/io/ObjectStreamField;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([BII)V
        pub fn new_arr_b_i_i(mut val: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            if ((val.borrow().len() as i32)==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i32 = Objects::checkFromIndexSize_i_i_i(off, len, (val.borrow().len() as i32))?;
            if (len==0) {
                this.__set_mag(Clone::clone(&BigInteger::ZERO().__get_mag()));
                this.__set_signum(BigInteger::ZERO().__get_signum());
                return Ok(this);
            }
            let mut b = (val.borrow()[off as usize] as i32);
            if (b<0) {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::makePositive_i_arr_b_i_i(b, Clone::clone(&val), off, len)?;
                this.__set_mag(Clone::clone(&_t1));
                this.__set_signum(-1i32);
            } else {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::stripLeadingZeroBytes_i_arr_b_i_i(b, Clone::clone(&val), off, len)?;
                this.__set_mag(Clone::clone(&_t1));
                this.__set_signum((((this.__get_mag().borrow().len() as i32)!=0)) as i32);
            }
            if (this.__get_mag().borrow().len() as i32) >= 67108864i32 {
                this.checkRange()?;
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([B)V
        pub fn new_arr_b(mut val: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
            let mut this = Self::default();
            this = BigInteger::new_arr_b_i_i(Clone::clone(&val), 0i32, (val.borrow().len() as i32))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_i(val: Rc<RefCell<Vec<i32>>>) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:([I)V")
        }

        #[java_method(name = "<init>", descriptor = "(I[BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_arr_b_i_i(signum: i32, magnitude: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(I[BII)V")
        }

        #[java_method(name = "<init>", descriptor = "(I[B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_arr_b(signum: i32, magnitude: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(I[B)V")
        }

        #[java_method(name = "<init>", descriptor = "(I[I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_arr_i(signum: i32, magnitude: Rc<RefCell<Vec<i32>>>) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(I[I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_i(val: String, radix: i32) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "<init>", descriptor = "([CII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_i_i(val: Rc<RefCell<Vec<u16>>>, sign: i32, len: i32) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:([CII)V")
        }

        #[java_method(name = "parseInt", descriptor = "([CII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseInt(&self, source: Rc<RefCell<Vec<u16>>>, start: i32, end: i32) -> Result<i32> {
            panic!("stub: java/math/BigInteger.parseInt:([CII)I")
        }

        #[java_method(name = "destructiveMulAdd", descriptor = "([III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn destructiveMulAdd(x: Rc<RefCell<Vec<i32>>>, y: i32, z: i32) -> Result<()> {
            panic!("stub: java/math/BigInteger.destructiveMulAdd:([III)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(val: String) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(ILjava/util/Random;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_random(numBits: i32, rnd: Random) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(ILjava/util/Random;)V")
        }

        #[java_method(name = "randomBits", descriptor = "(ILjava/util/Random;)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn randomBits(numBits: i32, rnd: Random) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/math/BigInteger.randomBits:(ILjava/util/Random;)[B")
        }

        #[java_method(name = "<init>", descriptor = "(IILjava/util/Random;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_random(bitLength: i32, certainty: i32, rnd: Random) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:(IILjava/util/Random;)V")
        }

        #[java_method(name = "probablePrime", descriptor = "(ILjava/util/Random;)Ljava/math/BigInteger;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn probablePrime(bitLength: i32, rnd: Random) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.probablePrime:(ILjava/util/Random;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "smallPrime", descriptor = "(IILjava/util/Random;)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn smallPrime(bitLength: i32, certainty: i32, rnd: Random) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.smallPrime:(IILjava/util/Random;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "largePrime", descriptor = "(IILjava/util/Random;)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn largePrime(bitLength: i32, certainty: i32, rnd: Random) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.largePrime:(IILjava/util/Random;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "nextProbablePrime", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextProbablePrime(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.nextProbablePrime:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "getPrimeSearchLen", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPrimeSearchLen(bitLength: i32) -> Result<i32> {
            panic!("stub: java/math/BigInteger.getPrimeSearchLen:(I)I")
        }

        #[java_method(name = "primeToCertainty", descriptor = "(ILjava/util/Random;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn primeToCertainty(&self, certainty: i32, random: Random) -> Result<bool> {
            panic!("stub: java/math/BigInteger.primeToCertainty:(ILjava/util/Random;)Z")
        }

        #[java_method(name = "passesLucasLehmer", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn passesLucasLehmer(&self) -> Result<bool> {
            panic!("stub: java/math/BigInteger.passesLucasLehmer:()Z")
        }

        #[java_method(name = "jacobiSymbol", descriptor = "(ILjava/math/BigInteger;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn jacobiSymbol(p: i32, n: BigInteger) -> Result<i32> {
            panic!("stub: java/math/BigInteger.jacobiSymbol:(ILjava/math/BigInteger;)I")
        }

        #[java_method(name = "lucasLehmerSequence", descriptor = "(ILjava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lucasLehmerSequence(z: i32, k: BigInteger, n: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.lucasLehmerSequence:(ILjava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "passesMillerRabin", descriptor = "(ILjava/util/Random;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn passesMillerRabin(&self, iterations: i32, rnd: Random) -> Result<bool> {
            panic!("stub: java/math/BigInteger.passesMillerRabin:(ILjava/util/Random;)Z")
        }

        #[java_method(name = "<init>", descriptor = "([II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([II)V
        pub fn new_arr_i_i(mut magnitude: Rc<RefCell<Vec<i32>>>, mut signum: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            this.__set_signum((if ((magnitude.borrow().len() as i32)==0) { 0i32 } else { signum }));
            this.__set_mag(Clone::clone(&magnitude));
            if (this.__get_mag().borrow().len() as i32) >= 67108864i32 {
                this.checkRange()?;
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([BI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_i(magnitude: Rc<RefCell<Vec<i8>>>, signum: i32) -> Result<Self> {
            panic!("stub: java/math/BigInteger.<init>:([BI)V")
        }

        #[java_method(name = "checkRange", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRange(&self) -> Result<()> {
            let this = self;
            if (this.__get_mag().borrow()[0i32 as usize]<0) {
                BigInteger::reportOverflow()?;
            }
            Ok(())
        }

        #[java_method(name = "reportOverflow", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reportOverflow() -> Result<()> {
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "valueOf", descriptor = "(J)Ljava/math/BigInteger;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(J)Ljava/math/BigInteger;
        pub fn valueOf_l(mut val: i64) -> Result<BigInteger> {
            if (((val>(0i64)) as i32-((val)<(0i64)) as i32)==0) {
                return Ok(BigInteger::ZERO());
            }
            if (((val>(16i64)) as i32-((val)<(16i64)) as i32)<=0) {
                return Ok(Clone::clone(&BigInteger::posConst().borrow()[(val as i32) as usize]));
            }
            if (((val>(-16i64)) as i32-((val)<(-16i64)) as i32)>=0) {
                return Ok(Clone::clone(&BigInteger::negConst().borrow()[((val).wrapping_neg() as i32) as usize]));
            }
            Ok(BigInteger::new_l(val)?)
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(J)V
        pub fn new_l(mut val: i64) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            if (((val>(0i64)) as i32-((val)<(0i64)) as i32)<0) {
                val = (val).wrapping_neg();
                this.__set_signum(-1i32);
            } else {
                this.__set_signum(1i32);
            }
            let mut highWord: i32 = (((val as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
            if (highWord==0) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 1i32 as usize]));
                this.__set_mag(Clone::clone(&_arr0));
                this.__get_mag().borrow_mut()[0i32 as usize] = (val as i32);
            } else {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 2i32 as usize]));
                this.__set_mag(Clone::clone(&_arr0));
                this.__get_mag().borrow_mut()[0i32 as usize] = highWord;
                this.__get_mag().borrow_mut()[1i32 as usize] = (val as i32);
            }
            Ok(this)
        }

        #[java_method(name = "valueOf", descriptor = "([I)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_arr_i(val: Rc<RefCell<Vec<i32>>>) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.valueOf:([I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "add", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: add(Ljava/math/BigInteger;)Ljava/math/BigInteger;
        pub fn add_bigint(&self, mut val: BigInteger) -> Result<BigInteger> {
            let this = self;
            if (val.__get_signum()==0) {
                return Ok(Clone::clone(this));
            }
            if (this.__get_signum()==0) {
                return Ok(val);
            }
            if val.__get_signum() == this.__get_signum() {
                let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::add_arr_i_arr_i(Clone::clone(&this.__get_mag()), Clone::clone(&val.__get_mag()))?;
                return Ok(BigInteger::new_arr_i_i(Clone::clone(&_t0), this.__get_signum())?);
            }
            let _t0 = this.compareMagnitude_bigint(Clone::clone(&val))?;
            let mut cmp: i32 = _t0;
            if (cmp==0) {
                return Ok(BigInteger::ZERO());
            }
            let mut _merged2: Rc<RefCell<Vec<i32>>>;
            if (cmp>0) {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::subtract_arr_i_arr_i(Clone::clone(&this.__get_mag()), Clone::clone(&val.__get_mag()))?;
                _merged2 = _t1;
            } else {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::subtract_arr_i_arr_i(Clone::clone(&val.__get_mag()), Clone::clone(&this.__get_mag()))?;
                _merged2 = _t1;
            }
            let mut resultMag: Rc<RefCell<Vec<i32>>> = _merged2;
            let _t3: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&resultMag))?;
            resultMag = _t3;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&resultMag), (cmp != this.__get_signum() as i32))?)
        }

        #[java_method(name = "add", descriptor = "(J)Ljava/math/BigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_l(&self, val: i64) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.add:(J)Ljava/math/BigInteger;")
        }

        #[java_method(name = "add", descriptor = "([IJ)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add_arr_i_l(x: Rc<RefCell<Vec<i32>>>, val: i64) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.add:([IJ)[I")
        }

        #[java_method(name = "add", descriptor = "([I[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: add([I[I)[I
        pub fn add_arr_i_arr_i(mut x: Rc<RefCell<Vec<i32>>>, mut y: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            if (x.borrow().len() as i32) < (y.borrow().len() as i32) {
                let mut tmp: Rc<RefCell<Vec<i32>>> = x;
                x = y;
                y = tmp;
            }
            let mut tmp = (x.borrow().len() as i32);
            let mut yIndex = (y.borrow().len() as i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; tmp as usize]));
            let mut result: Rc<RefCell<Vec<i32>>> = _arr0;
            let mut sum: i64 = 0i64;
            if yIndex == 1i32 {
                tmp = tmp.wrapping_sub(1i32);
                sum = (((x.borrow()[tmp as usize] as i64)&(4294967295i64))).wrapping_add(((y.borrow()[0i32 as usize] as i64)&(4294967295i64)));
                result.borrow_mut()[tmp as usize] = (sum as i32);
            } else {
                loop {
                    if (yIndex<=0) { break; }
                    tmp = tmp.wrapping_sub(1i32);
                    yIndex = yIndex.wrapping_sub(1i32);
                    sum = ((((x.borrow()[tmp as usize] as i64)&(4294967295i64))).wrapping_add(((y.borrow()[yIndex as usize] as i64)&(4294967295i64)))).wrapping_add(((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64));
                    result.borrow_mut()[tmp as usize] = (sum as i32);
                }
            }
            let mut carry = ((((((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64)>(0i64)) as i32-((((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64))<(0i64)) as i32)!=0)) as i32;
            loop {
                if (tmp<=0) { break; }
                tmp = tmp.wrapping_sub(1i32);
                result.borrow_mut()[tmp as usize] = (x.borrow()[tmp as usize]).wrapping_add(1i32);
                carry = (((x.borrow()[tmp as usize]).wrapping_add(1i32)==0)) as i32;
            }
            loop {
                if (tmp<=0) { break; }
                tmp = tmp.wrapping_sub(1i32);
                result.borrow_mut()[tmp as usize] = x.borrow()[tmp as usize];
            }
            if (carry!=0) {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((result.borrow().len() as i32)).wrapping_add(1i32) as usize]));
                let mut bigger: Rc<RefCell<Vec<i32>>> = _arr1;
                System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(bigger.clone()), 1i32, (result.borrow().len() as i32))?;
                bigger.borrow_mut()[0i32 as usize] = 1i32;
                return Ok(bigger);
            }
            Ok(result)
        }

        #[java_method(name = "subtract", descriptor = "(J[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtract_l_arr_i(val: i64, arg1: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.subtract:(J[I)[I")
        }

        #[java_method(name = "subtract", descriptor = "([IJ)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtract_arr_i_l(big: Rc<RefCell<Vec<i32>>>, val: i64) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.subtract:([IJ)[I")
        }

        #[java_method(name = "subtract", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: subtract(Ljava/math/BigInteger;)Ljava/math/BigInteger;
        pub fn subtract_bigint(&self, mut val: BigInteger) -> Result<BigInteger> {
            let this = self;
            if (val.__get_signum()==0) {
                return Ok(Clone::clone(this));
            }
            if (this.__get_signum()==0) {
                let _t0 = val.negate()?;
                return Ok(_t0);
            }
            if val.__get_signum() != this.__get_signum() {
                let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::add_arr_i_arr_i(Clone::clone(&this.__get_mag()), Clone::clone(&val.__get_mag()))?;
                return Ok(BigInteger::new_arr_i_i(Clone::clone(&_t0), this.__get_signum())?);
            }
            let _t0 = this.compareMagnitude_bigint(Clone::clone(&val))?;
            let mut cmp: i32 = _t0;
            if (cmp==0) {
                return Ok(BigInteger::ZERO());
            }
            let mut _merged2: Rc<RefCell<Vec<i32>>>;
            if (cmp>0) {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::subtract_arr_i_arr_i(Clone::clone(&this.__get_mag()), Clone::clone(&val.__get_mag()))?;
                _merged2 = _t1;
            } else {
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::subtract_arr_i_arr_i(Clone::clone(&val.__get_mag()), Clone::clone(&this.__get_mag()))?;
                _merged2 = _t1;
            }
            let mut resultMag: Rc<RefCell<Vec<i32>>> = _merged2;
            let _t3: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&resultMag))?;
            resultMag = _t3;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&resultMag), (cmp != this.__get_signum() as i32))?)
        }

        #[java_method(name = "subtract", descriptor = "([I[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: subtract([I[I)[I
        pub fn subtract_arr_i_arr_i(mut big: Rc<RefCell<Vec<i32>>>, mut little: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut bigIndex = (big.borrow().len() as i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; bigIndex as usize]));
            let mut result: Rc<RefCell<Vec<i32>>> = _arr0;
            let mut littleIndex = (little.borrow().len() as i32);
            let mut difference: i64 = 0i64;
            loop {
                if (littleIndex<=0) { break; }
                bigIndex = bigIndex.wrapping_sub(1i32);
                littleIndex = littleIndex.wrapping_sub(1i32);
                difference = ((((big.borrow()[bigIndex as usize] as i64)&(4294967295i64))).wrapping_sub(((little.borrow()[littleIndex as usize] as i64)&(4294967295i64)))).wrapping_add((difference).wrapping_shr((32i32&0x3f) as u32));
                result.borrow_mut()[bigIndex as usize] = (difference as i32);
            }
            let mut borrow = (((((difference).wrapping_shr((32i32&0x3f) as u32)>(0i64)) as i32-(((difference).wrapping_shr((32i32&0x3f) as u32))<(0i64)) as i32)!=0)) as i32;
            loop {
                if (bigIndex<=0) { break; }
                bigIndex = bigIndex.wrapping_sub(1i32);
                result.borrow_mut()[bigIndex as usize] = (big.borrow()[bigIndex as usize]).wrapping_sub(1i32);
                borrow = ((big.borrow()[bigIndex as usize]).wrapping_sub(1i32) == -1i32) as i32;
            }
            loop {
                if (bigIndex<=0) { break; }
                bigIndex = bigIndex.wrapping_sub(1i32);
                result.borrow_mut()[bigIndex as usize] = big.borrow()[bigIndex as usize];
            }
            Ok(result)
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: multiply(Ljava/math/BigInteger;)Ljava/math/BigInteger;
        pub fn multiply_bigint(&self, mut val: BigInteger) -> Result<BigInteger> {
            let this = self;
            let _t0 = this.multiply_bigint_z_z_i(Clone::clone(&val), (0i32 != 0i32), (0i32 != 0i32), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "parallelMultiply", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelMultiply(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.parallelMultiply:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigInteger;ZZI)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: multiply(Ljava/math/BigInteger;ZZI)Ljava/math/BigInteger;
        pub fn multiply_bigint_z_z_i(&self, mut val: BigInteger, mut isRecursion: bool, mut parallel: bool, mut depth: i32) -> Result<BigInteger> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(BigInteger::ZERO());
            }
            let mut xlen = (this.__get_mag().borrow().len() as i32);
            if xlen > 20i32 {
                let _t0 = this.square_z_z_i((1i32 != 0i32), parallel, depth)?;
                return Ok(_t0);
            }
            let mut ylen = (val.__get_mag().borrow().len() as i32);
            let mut resultSign = (this.__get_signum() != val.__get_signum()) as i32;
            if (val.__get_mag().borrow().len() as i32) == 1i32 {
                let _t0: BigInteger = BigInteger::multiplyByInt(Clone::clone(&this.__get_mag()), val.__get_mag().borrow()[0i32 as usize], resultSign)?;
                return Ok(_t0);
            }
            if (this.__get_mag().borrow().len() as i32) == 1i32 {
                let _t0: BigInteger = BigInteger::multiplyByInt(Clone::clone(&val.__get_mag()), this.__get_mag().borrow()[0i32 as usize], resultSign)?;
                return Ok(_t0);
            }
            let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::multiplyToLen(Clone::clone(&this.__get_mag()), xlen, Clone::clone(&val.__get_mag()), ylen, Default::default())?;
            let mut result: Rc<RefCell<Vec<i32>>> = _t0;
            let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&result))?;
            result = _t1;
            return Ok(BigInteger::new_arr_i_i(Clone::clone(&result), resultSign)?);
            if ylen < 240i32 {
                let _t2: BigInteger = BigInteger::multiplyKaratsuba(Clone::clone(this), Clone::clone(&val))?;
                return Ok(_t2);
            }
            let _t2: i32 = BigInteger::bitLength_arr_i_i(Clone::clone(&this.__get_mag()), (this.__get_mag().borrow().len() as i32))?;
            let _t3: i32 = BigInteger::bitLength_arr_i_i(Clone::clone(&val.__get_mag()), (val.__get_mag().borrow().len() as i32))?;
            if (((((_t2 as i64)).wrapping_add((_t3 as i64))>(2147483648i64)) as i32-((((_t2 as i64)).wrapping_add((_t3 as i64)))<(2147483648i64)) as i32)>0) {
                BigInteger::reportOverflow()?;
            }
            let _t4: BigInteger = BigInteger::multiplyToomCook3(Clone::clone(this), Clone::clone(&val), parallel, depth)?;
            Ok(_t4)
        }

        #[java_method(name = "multiplyByInt", descriptor = "([III)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyByInt(mut x: Rc<RefCell<Vec<i32>>>, mut y: i32, mut sign: i32) -> Result<BigInteger> {
            let _t0: i32 = Integer::bitCount(y)?;
            if _t0 == 1i32 {
                let _t1: i32 = Integer::numberOfTrailingZeros(y)?;
                let _t2: Rc<RefCell<Vec<i32>>> = BigInteger::shiftLeft_arr_i_i(Clone::clone(&x), _t1)?;
                return Ok(BigInteger::new_arr_i_i(Clone::clone(&_t2), sign)?);
            }
            let mut xlen = (x.borrow().len() as i32);
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (xlen).wrapping_add(1i32) as usize]));
            let mut rmag: Rc<RefCell<Vec<i32>>> = _arr1;
            let mut carry: i64 = 0i64;
            let mut yl = ((y as i64)&(4294967295i64));
            let mut rstart = ((rmag.borrow().len() as i32)).wrapping_sub(1i32);
            let mut i = (xlen).wrapping_sub(1i32);
            loop {
                if (i<0) { break; }
                let mut product = ((((x.borrow()[i as usize] as i64)&(4294967295i64))).wrapping_mul(yl)).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                rmag.borrow_mut()[rstart as usize] = (product as i32);
                carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                i = i.wrapping_sub(1i32);
            }
            if (((carry>(0i64)) as i32-((carry)<(0i64)) as i32)==0) {
                let _t2: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&rmag), 1i32, (rmag.borrow().len() as i32))?;
                rmag = _t2;
            } else {
                rmag.borrow_mut()[rstart as usize] = (carry as i32);
            }
            Ok(BigInteger::new_arr_i_i(Clone::clone(&rmag), sign)?)
        }

        #[java_method(name = "multiply", descriptor = "(J)Ljava/math/BigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: multiply(J)Ljava/math/BigInteger;
        pub fn multiply_l(&self, mut v: i64) -> Result<BigInteger> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(BigInteger::ZERO());
            }
            if (((v>(-9223372036854775808i64)) as i32-((v)<(-9223372036854775808i64)) as i32)==0) {
                let _t0 = this.multiply_bigint(v)?;
                return Ok(_t0);
            }
            let mut rsign = (if (((v>(0i64)) as i32-((v)<(0i64)) as i32)>0) { this.__get_signum() } else { (this.__get_signum()).wrapping_neg() });
            if (((v>(0i64)) as i32-((v)<(0i64)) as i32)<0) {
                v = (v).wrapping_neg();
            }
            let mut dh = ((v as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            let mut dl = (v&(4294967295i64));
            let mut xlen = (this.__get_mag().borrow().len() as i32);
            let mut value = this.__get_mag();
            let mut _merged1: Rc<RefCell<Vec<i32>>>;
            if (((dh>(0i64)) as i32-((dh)<(0i64)) as i32)==0) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (xlen).wrapping_add(1i32) as usize]));
                _merged1 = _arr0;
            } else {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (xlen).wrapping_add(2i32) as usize]));
                _merged1 = _arr0;
            }
            let mut rmag: Rc<RefCell<Vec<i32>>> = _merged1;
            let mut carry: i64 = 0i64;
            let mut rstart = ((rmag.borrow().len() as i32)).wrapping_sub(1i32);
            let mut i = (xlen).wrapping_sub(1i32);
            let mut product = Default::default();
            loop {
                if (i<0) { break; }
                product = ((((value.borrow()[i as usize] as i64)&(4294967295i64))).wrapping_mul(dl)).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                rmag.borrow_mut()[rstart as usize] = (product as i32);
                carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                i = i.wrapping_sub(1i32);
            }
            rmag.borrow_mut()[rstart as usize] = (carry as i32);
            carry = 0i64;
            rstart = ((rmag.borrow().len() as i32)).wrapping_sub(2i32);
            i = (xlen).wrapping_sub(1i32);
            loop {
                if (i<0) { break; }
                product = (((((value.borrow()[i as usize] as i64)&(4294967295i64))).wrapping_mul(dh)).wrapping_add(((rmag.borrow()[rstart as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                rmag.borrow_mut()[rstart as usize] = (product as i32);
                carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                i = i.wrapping_sub(1i32);
            }
            rmag.borrow_mut()[0i32 as usize] = (carry as i32);
            if (((carry>(0i64)) as i32-((carry)<(0i64)) as i32)==0) {
                let _t2: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&rmag), 1i32, (rmag.borrow().len() as i32))?;
                rmag = _t2;
            }
            Ok(BigInteger::new_arr_i_i(Clone::clone(&rmag), rsign)?)
        }

        #[java_method(name = "multiplyToLen", descriptor = "([II[II[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyToLen(mut x: Rc<RefCell<Vec<i32>>>, mut xlen: i32, mut y: Rc<RefCell<Vec<i32>>>, mut ylen: i32, mut z: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            BigInteger::multiplyToLenCheck(Clone::clone(&x), xlen)?;
            BigInteger::multiplyToLenCheck(Clone::clone(&y), ylen)?;
            let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::implMultiplyToLen(Clone::clone(&x), xlen, Clone::clone(&y), ylen, Clone::clone(&z))?;
            Ok(_t0)
        }

        #[java_method(name = "implMultiplyToLen", descriptor = "([II[II[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implMultiplyToLen(mut x: Rc<RefCell<Vec<i32>>>, mut xlen: i32, mut y: Rc<RefCell<Vec<i32>>>, mut ylen: i32, mut z: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut xstart = (xlen).wrapping_sub(1i32);
            let mut ystart = (ylen).wrapping_sub(1i32);
            if (z.borrow().len() as i32) < (xlen).wrapping_add(ylen) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (xlen).wrapping_add(ylen) as usize]));
                z = _arr0;
            }
            let mut carry: i64 = 0i64;
            let mut j: i32 = ystart;
            let mut k = ((ystart).wrapping_add(1i32)).wrapping_add(xstart);
            loop {
                if (j<0) { break; }
                let mut product = ((((y.borrow()[j as usize] as i64)&(4294967295i64))).wrapping_mul(((x.borrow()[xstart as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                z.borrow_mut()[k as usize] = (product as i32);
                carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                j = j.wrapping_sub(1i32);
                k = k.wrapping_sub(1i32);
            }
            z.borrow_mut()[xstart as usize] = (carry as i32);
            j = (xstart).wrapping_sub(1i32);
            loop {
                if (j<0) { break; }
                carry = 0i64;
                k = ystart;
                let mut product = ((ystart).wrapping_add(1i32)).wrapping_add(j);
                loop {
                    if (k<0) { break; }
                    let mut product = (((((y.borrow()[k as usize] as i64)&(4294967295i64))).wrapping_mul(((x.borrow()[j as usize] as i64)&(4294967295i64)))).wrapping_add(((z.borrow()[product as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                    z.borrow_mut()[product as usize] = (product as i32);
                    carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                    k = k.wrapping_sub(1i32);
                    product = product.wrapping_sub(1i32);
                }
                z.borrow_mut()[j as usize] = (carry as i32);
                j = j.wrapping_sub(1i32);
            }
            Ok(z)
        }

        #[java_method(name = "multiplyToLenCheck", descriptor = "([II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyToLenCheck(mut array: Rc<RefCell<Vec<i32>>>, mut length: i32) -> Result<()> {
            if (length<=0) {
                return Ok(());
            }
            let _t0: Object = Objects::requireNonNull_obj(Object::from_any(array.clone()))?;
            if length > (array.borrow().len() as i32) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "multiplyKaratsuba", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyKaratsuba(mut x: BigInteger, mut y: BigInteger) -> Result<BigInteger> {
            let mut xlen = (x.__get_mag().borrow().len() as i32);
            let mut ylen = (y.__get_mag().borrow().len() as i32);
            let _t0: i32 = Math::max_i_i(xlen, ylen)?;
            let mut half = ((_t0).wrapping_add(1i32)/2i32);
            let _t1 = x.getLower(half)?;
            let mut xl: BigInteger = _t1;
            let _t2 = x.getUpper(half)?;
            let mut xh: BigInteger = _t2;
            let _t3 = y.getLower(half)?;
            let mut yl: BigInteger = _t3;
            let _t4 = y.getUpper(half)?;
            let mut yh: BigInteger = _t4;
            let _t5 = xh.multiply_bigint(Clone::clone(&yh))?;
            let mut p1: BigInteger = _t5;
            let _t6 = xl.multiply_bigint(Clone::clone(&yl))?;
            let mut p2: BigInteger = _t6;
            let _t7 = xh.add_bigint(Clone::clone(&xl))?;
            let _t8 = yh.add_bigint(Clone::clone(&yl))?;
            let _t9 = _t7.multiply_bigint(Clone::clone(&_t8))?;
            let mut p3: BigInteger = _t9;
            let _t10 = p1.shiftLeft_i((32i32).wrapping_mul(half))?;
            let _t11 = p3.subtract_bigint(Clone::clone(&p1))?;
            let _t12 = _t11.subtract_bigint(Clone::clone(&p2))?;
            let _t13 = _t10.add_bigint(Clone::clone(&_t12))?;
            let _t14 = _t13.shiftLeft_i((32i32).wrapping_mul(half))?;
            let _t15 = _t14.add_bigint(Clone::clone(&p2))?;
            let mut result: BigInteger = _t15;
            if x.__get_signum() != y.__get_signum() {
                let _t16 = result.negate()?;
                return Ok(_t16);
            }
            Ok(result)
        }

        #[java_method(name = "multiplyToomCook3", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyToomCook3(mut a: BigInteger, mut b: BigInteger, mut parallel: bool, mut depth: i32) -> Result<BigInteger> {
            let mut alen = (a.__get_mag().borrow().len() as i32);
            let mut blen = (b.__get_mag().borrow().len() as i32);
            let _t0: i32 = Math::max_i_i(alen, blen)?;
            let mut largest: i32 = _t0;
            let mut k = ((largest).wrapping_add(2i32)/3i32);
            let mut r = (largest).wrapping_sub((2i32).wrapping_mul(k));
            let _t1 = a.getToomSlice(k, r, 0i32, largest)?;
            let mut a2: BigInteger = _t1;
            let _t2 = a.getToomSlice(k, r, 1i32, largest)?;
            let mut a1: BigInteger = _t2;
            let _t3 = a.getToomSlice(k, r, 2i32, largest)?;
            let mut a0: BigInteger = _t3;
            let _t4 = b.getToomSlice(k, r, 0i32, largest)?;
            let mut b2: BigInteger = _t4;
            let _t5 = b.getToomSlice(k, r, 1i32, largest)?;
            let mut b1: BigInteger = _t5;
            let _t6 = b.getToomSlice(k, r, 2i32, largest)?;
            let mut b0: BigInteger = _t6;
            depth = depth.wrapping_add(1i32);
            let _t7: RecursiveTask<Object> = BigInteger_RecursiveOp::multiply(Clone::clone(&a0), Clone::clone(&b0), parallel, depth)?;
            let mut v0_task: RecursiveTask<Object> = _t7;
            let _t8 = a2.add_bigint(Clone::clone(&a0))?;
            let mut da1: BigInteger = _t8;
            let _t9 = b2.add_bigint(Clone::clone(&b0))?;
            let mut db1: BigInteger = _t9;
            let _t10 = da1.subtract_bigint(Clone::clone(&a1))?;
            let _t11 = db1.subtract_bigint(Clone::clone(&b1))?;
            let _t12: RecursiveTask<Object> = BigInteger_RecursiveOp::multiply(Clone::clone(&_t10), Clone::clone(&_t11), parallel, depth)?;
            let mut vm1_task: RecursiveTask<Object> = _t12;
            let _t13 = da1.add_bigint(Clone::clone(&a1))?;
            da1 = _t13;
            let _t14 = db1.add_bigint(Clone::clone(&b1))?;
            db1 = _t14;
            let _t15: RecursiveTask<Object> = BigInteger_RecursiveOp::multiply(Clone::clone(&da1), Clone::clone(&db1), parallel, depth)?;
            let mut v1_task: RecursiveTask<Object> = _t15;
            let _t16 = da1.add_bigint(Clone::clone(&a2))?;
            let _t17 = _t16.shiftLeft_i(1i32)?;
            let _t18 = _t17.subtract_bigint(Clone::clone(&a0))?;
            let _t19 = db1.add_bigint(Clone::clone(&b2))?;
            let _t20 = _t19.shiftLeft_i(1i32)?;
            let _t21 = _t20.subtract_bigint(Clone::clone(&b0))?;
            let _t22 = _t18.multiply_bigint_z_z_i(Clone::clone(&_t21), (1i32 != 0i32), parallel, depth)?;
            let mut v2: BigInteger = _t22;
            let _t23 = a2.multiply_bigint_z_z_i(Clone::clone(&b2), (1i32 != 0i32), parallel, depth)?;
            let mut vinf: BigInteger = _t23;
            let _t24 = v0_task.__super().join()?;
            let mut v0 = (_t24).downcast::<BigInteger>();
            let _t25 = vm1_task.__super().join()?;
            let mut vm1 = (_t25).downcast::<BigInteger>();
            let _t26 = v1_task.__super().join()?;
            let mut v1 = (_t26).downcast::<BigInteger>();
            let _t27 = v2.subtract_bigint(Clone::clone(&vm1))?;
            let _t28 = _t27.exactDivideBy3()?;
            let mut t2: BigInteger = _t28;
            let _t29 = v1.subtract_bigint(Clone::clone(&vm1))?;
            let _t30 = _t29.shiftRight(1i32)?;
            let mut tm1: BigInteger = _t30;
            let _t31 = v1.subtract_bigint(Clone::clone(&v0))?;
            let mut t1: BigInteger = _t31;
            let _t32 = t2.subtract_bigint(Clone::clone(&t1))?;
            let _t33 = _t32.shiftRight(1i32)?;
            t2 = _t33;
            let _t34 = t1.subtract_bigint(Clone::clone(&tm1))?;
            let _t35 = _t34.subtract_bigint(Clone::clone(&vinf))?;
            t1 = _t35;
            let _t36 = vinf.shiftLeft_i(1i32)?;
            let _t37 = t2.subtract_bigint(Clone::clone(&_t36))?;
            t2 = _t37;
            let _t38 = tm1.subtract_bigint(Clone::clone(&t2))?;
            tm1 = _t38;
            let mut ss = (k).wrapping_mul(32i32);
            let _t39 = vinf.shiftLeft_i(ss)?;
            let _t40 = _t39.add_bigint(Clone::clone(&t2))?;
            let _t41 = _t40.shiftLeft_i(ss)?;
            let _t42 = _t41.add_bigint(Clone::clone(&t1))?;
            let _t43 = _t42.shiftLeft_i(ss)?;
            let _t44 = _t43.add_bigint(Clone::clone(&tm1))?;
            let _t45 = _t44.shiftLeft_i(ss)?;
            let _t46 = _t45.add_bigint(Clone::clone(&v0))?;
            let mut result: BigInteger = _t46;
            if a.__get_signum() != b.__get_signum() {
                let _t47 = result.negate()?;
                return Ok(_t47);
            }
            Ok(result)
        }

        #[java_method(name = "getToomSlice", descriptor = "(IIII)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getToomSlice(&self, mut lowerSize: i32, mut upperSize: i32, mut slice: i32, mut fullsize: i32) -> Result<BigInteger> {
            let this = self;
            let mut len = (this.__get_mag().borrow().len() as i32);
            let mut offset = (fullsize).wrapping_sub(len);
        let mut end = Default::default();
        let mut start = Default::default();
            if (slice==0) {
                start = (0i32).wrapping_sub(offset);
                end = ((upperSize).wrapping_sub(1i32)).wrapping_sub(offset);
            } else {
                start = ((upperSize).wrapping_add(((slice).wrapping_sub(1i32)).wrapping_mul(lowerSize))).wrapping_sub(offset);
                end = ((start).wrapping_add(lowerSize)).wrapping_sub(1i32);
            }
            if (start<0) {
                start = 0i32;
            }
            if (end<0) {
                return Ok(BigInteger::ZERO());
            }
            let mut sliceSize = ((end).wrapping_sub(start)).wrapping_add(1i32);
            if (sliceSize<=0) {
                return Ok(BigInteger::ZERO());
            }
            if sliceSize >= len {
                let _t0 = this.abs()?;
                return Ok(_t0);
            }
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; sliceSize as usize]));
            let mut intSlice: Rc<RefCell<Vec<i32>>> = _arr0;
            System::arraycopy(Object::from_any(this.__get_mag().clone()), start, Object::from_any(intSlice.clone()), 0i32, sliceSize)?;
            let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&intSlice))?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t1), 1i32)?)
        }

        #[java_method(name = "exactDivideBy3", descriptor = "()Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exactDivideBy3(&self) -> Result<BigInteger> {
            let this = self;
            let mut len = (this.__get_mag().borrow().len() as i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; len as usize]));
            let mut result: Rc<RefCell<Vec<i32>>> = _arr0;
            let mut borrow: i64 = 0i64;
            let mut i = (len).wrapping_sub(1i32);
            loop {
                if (i<0) { break; }
                let mut x = ((this.__get_mag().borrow()[i as usize] as i64)&(4294967295i64));
                let mut w = (x).wrapping_sub(borrow);
                if (((borrow>(x)) as i32-((borrow)<(x)) as i32)>0) {
                    borrow = 1i64;
                } else {
                    borrow = 0i64;
                }
                let mut q = ((w).wrapping_mul(2863311531i64)&(4294967295i64));
                result.borrow_mut()[i as usize] = (q as i32);
                borrow = (borrow).wrapping_add(1i64);
                if (((q>(2863311531i64)) as i32-((q)<(2863311531i64)) as i32)>=0) {
                    borrow = (borrow).wrapping_add(1i64);
                }
                i = i.wrapping_sub(1i32);
            }
            let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&result))?;
            result = _t1;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&result), this.__get_signum())?)
        }

        #[java_method(name = "getLower", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLower(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            let mut len = (this.__get_mag().borrow().len() as i32);
            if len <= n {
                let _t0 = this.abs()?;
                return Ok(_t0);
            }
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; n as usize]));
            let mut lowerInts: Rc<RefCell<Vec<i32>>> = _arr0;
            System::arraycopy(Object::from_any(this.__get_mag().clone()), (len).wrapping_sub(n), Object::from_any(lowerInts.clone()), 0i32, n)?;
            let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&lowerInts))?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t1), 1i32)?)
        }

        #[java_method(name = "getUpper", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUpper(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            let mut len = (this.__get_mag().borrow().len() as i32);
            if len <= n {
                return Ok(BigInteger::ZERO());
            }
            let mut upperLen = (len).wrapping_sub(n);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; upperLen as usize]));
            let mut upperInts: Rc<RefCell<Vec<i32>>> = _arr0;
            System::arraycopy(Object::from_any(this.__get_mag().clone()), 0i32, Object::from_any(upperInts.clone()), 0i32, upperLen)?;
            let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&upperInts))?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t1), 1i32)?)
        }

        #[java_method(name = "square", descriptor = "()Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: square()Ljava/math/BigInteger;
        pub fn square(&self) -> Result<BigInteger> {
            let this = self;
            let _t0 = this.square_z_z_i((0i32 != 0i32), (0i32 != 0i32), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "square", descriptor = "(ZZI)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: square(ZZI)Ljava/math/BigInteger;
        pub fn square_z_z_i(&self, mut isRecursion: bool, mut parallel: bool, mut depth: i32) -> Result<BigInteger> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(BigInteger::ZERO());
            }
            let mut len = (this.__get_mag().borrow().len() as i32);
            if len < 128i32 {
                let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::squareToLen(Clone::clone(&this.__get_mag()), len, Default::default())?;
                let mut z: Rc<RefCell<Vec<i32>>> = _t0;
                let _t1: Rc<RefCell<Vec<i32>>> = BigInteger::trustedStripLeadingZeroInts(Clone::clone(&z))?;
                return Ok(BigInteger::new_arr_i_i(Clone::clone(&_t1), 1i32)?);
            }
            if len < 216i32 {
                let _t0 = this.squareKaratsuba()?;
                return Ok(_t0);
            }
            let _t0: i32 = BigInteger::bitLength_arr_i_i(Clone::clone(&this.__get_mag()), (this.__get_mag().borrow().len() as i32))?;
            if ((((_t0 as i64)>(1073741824i64)) as i32-(((_t0 as i64))<(1073741824i64)) as i32)>0) {
                BigInteger::reportOverflow()?;
            }
            let _t1 = this.squareToomCook3(parallel, depth)?;
            Ok(_t1)
        }

        #[java_method(name = "squareToLen", descriptor = "([II[I)[I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn squareToLen(mut x: Rc<RefCell<Vec<i32>>>, mut len: i32, mut z: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut zlen = (len<<(1i32&0x1f));
            if (z.borrow().len() as i32) < zlen {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; zlen as usize]));
                z = _arr0;
            }
            BigInteger::implSquareToLenChecks(Clone::clone(&x), len, Clone::clone(&z), zlen)?;
            let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::implSquareToLen(Clone::clone(&x), len, Clone::clone(&z), zlen)?;
            Ok(_t0)
        }

        #[java_method(name = "implSquareToLenChecks", descriptor = "([II[II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/RuntimeException")]
        pub fn implSquareToLenChecks(mut x: Rc<RefCell<Vec<i32>>>, mut len: i32, mut z: Rc<RefCell<Vec<i32>>>, mut zlen: i32) -> Result<()> {
            if len < 1i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("invalid input length: ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if len > (x.borrow().len() as i32) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input length out of bound: ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i((x.borrow().len() as i32))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (len).wrapping_mul(2i32) > (z.borrow().len() as i32) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input length out of bound: ")))?;
                let _t1 = _t0.append_i((len).wrapping_mul(2i32))?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i((z.borrow().len() as i32))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if zlen < 1i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("invalid input length: ")))?;
                let _t1 = _t0.append_i(zlen)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if zlen > (z.borrow().len() as i32) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input length out of bound: ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i((z.borrow().len() as i32))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "implSquareToLen", descriptor = "([II[II)[I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implSquareToLen(mut x: Rc<RefCell<Vec<i32>>>, mut len: i32, mut z: Rc<RefCell<Vec<i32>>>, mut zlen: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut lastProductLowWord: i32 = 0i32;
            let mut j: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if j >= len { break; }
                let mut piece = ((x.borrow()[j as usize] as i64)&(4294967295i64));
                let mut product = (piece).wrapping_mul(piece);
                i = i.wrapping_add(1i32);
                z.borrow_mut()[i as usize] = ((lastProductLowWord<<(31i32&0x1f))|(((product as u64).wrapping_shr((33i32&0x3f) as u32) as i64) as i32));
                i = i.wrapping_add(1i32);
                z.borrow_mut()[i as usize] = (((product as u64).wrapping_shr((1i32&0x3f) as u32) as i64) as i32);
                lastProductLowWord = (product as i32);
                j = j.wrapping_add(1i32);
            }
            j = len;
            i = 1i32;
            loop {
                if (j<=0) { break; }
                let mut piece = x.borrow()[(j).wrapping_sub(1i32) as usize];
                let _t0: i32 = BigInteger::mulAdd(Clone::clone(&z), Clone::clone(&x), i, (j).wrapping_sub(1i32), piece)?;
                piece = _t0;
                let _t1: i32 = BigInteger::addOne(Clone::clone(&z), (i).wrapping_sub(1i32), j, piece)?;
                j = j.wrapping_sub(1i32);
                i = i.wrapping_add(2i32);
            }
            BigInteger::primitiveLeftShift(Clone::clone(&z), zlen, 1i32)?;
            let _tmp_val0 = (z.borrow()[(zlen).wrapping_sub(1i32) as usize]|(x.borrow()[(len).wrapping_sub(1i32) as usize]&1i32));
            z.borrow_mut()[(zlen).wrapping_sub(1i32) as usize] = _tmp_val0;
            Ok(z)
        }

        #[java_method(name = "squareKaratsuba", descriptor = "()Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn squareKaratsuba(&self) -> Result<BigInteger> {
            let this = self;
            let mut half = (((this.__get_mag().borrow().len() as i32)).wrapping_add(1i32)/2i32);
            let _t0 = this.getLower(half)?;
            let mut xl: BigInteger = _t0;
            let _t1 = this.getUpper(half)?;
            let mut xh: BigInteger = _t1;
            let _t2 = xh.square()?;
            let mut xhs: BigInteger = _t2;
            let _t3 = xl.square()?;
            let mut xls: BigInteger = _t3;
            let _t4 = xhs.shiftLeft_i((half).wrapping_mul(32i32))?;
            let _t5 = xl.add_bigint(Clone::clone(&xh))?;
            let _t6 = _t5.square()?;
            let _t7 = xhs.add_bigint(Clone::clone(&xls))?;
            let _t8 = _t6.subtract_bigint(Clone::clone(&_t7))?;
            let _t9 = _t4.add_bigint(Clone::clone(&_t8))?;
            let _t10 = _t9.shiftLeft_i((half).wrapping_mul(32i32))?;
            let _t11 = _t10.add_bigint(Clone::clone(&xls))?;
            Ok(_t11)
        }

        #[java_method(name = "squareToomCook3", descriptor = "(ZI)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn squareToomCook3(&self, mut parallel: bool, mut depth: i32) -> Result<BigInteger> {
            let this = self;
            let mut len = (this.__get_mag().borrow().len() as i32);
            let mut k = ((len).wrapping_add(2i32)/3i32);
            let mut r = (len).wrapping_sub((2i32).wrapping_mul(k));
            let _t0 = this.getToomSlice(k, r, 0i32, len)?;
            let mut a2: BigInteger = _t0;
            let _t1 = this.getToomSlice(k, r, 1i32, len)?;
            let mut a1: BigInteger = _t1;
            let _t2 = this.getToomSlice(k, r, 2i32, len)?;
            let mut a0: BigInteger = _t2;
            depth = depth.wrapping_add(1i32);
            let _t3: RecursiveTask<Object> = BigInteger_RecursiveOp::square(Clone::clone(&a0), parallel, depth)?;
            let mut v0_fork: RecursiveTask<Object> = _t3;
            let _t4 = a2.add_bigint(Clone::clone(&a0))?;
            let mut da1: BigInteger = _t4;
            let _t5 = da1.subtract_bigint(Clone::clone(&a1))?;
            let _t6: RecursiveTask<Object> = BigInteger_RecursiveOp::square(Clone::clone(&_t5), parallel, depth)?;
            let mut vm1_fork: RecursiveTask<Object> = _t6;
            let _t7 = da1.add_bigint(Clone::clone(&a1))?;
            da1 = _t7;
            let _t8: RecursiveTask<Object> = BigInteger_RecursiveOp::square(Clone::clone(&da1), parallel, depth)?;
            let mut v1_fork: RecursiveTask<Object> = _t8;
            let _t9 = a2.square_z_z_i((1i32 != 0i32), parallel, depth)?;
            let mut vinf: BigInteger = _t9;
            let _t10 = da1.add_bigint(Clone::clone(&a2))?;
            let _t11 = _t10.shiftLeft_i(1i32)?;
            let _t12 = _t11.subtract_bigint(Clone::clone(&a0))?;
            let _t13 = _t12.square_z_z_i((1i32 != 0i32), parallel, depth)?;
            let mut v2: BigInteger = _t13;
            let _t14 = v0_fork.__super().join()?;
            let mut v0 = (_t14).downcast::<BigInteger>();
            let _t15 = vm1_fork.__super().join()?;
            let mut vm1 = (_t15).downcast::<BigInteger>();
            let _t16 = v1_fork.__super().join()?;
            let mut v1 = (_t16).downcast::<BigInteger>();
            let _t17 = v2.subtract_bigint(Clone::clone(&vm1))?;
            let _t18 = _t17.exactDivideBy3()?;
            let mut t2: BigInteger = _t18;
            let _t19 = v1.subtract_bigint(Clone::clone(&vm1))?;
            let _t20 = _t19.shiftRight(1i32)?;
            let mut tm1: BigInteger = _t20;
            let _t21 = v1.subtract_bigint(Clone::clone(&v0))?;
            let mut t1: BigInteger = _t21;
            let _t22 = t2.subtract_bigint(Clone::clone(&t1))?;
            let _t23 = _t22.shiftRight(1i32)?;
            t2 = _t23;
            let _t24 = t1.subtract_bigint(Clone::clone(&tm1))?;
            let _t25 = _t24.subtract_bigint(Clone::clone(&vinf))?;
            t1 = _t25;
            let _t26 = vinf.shiftLeft_i(1i32)?;
            let _t27 = t2.subtract_bigint(Clone::clone(&_t26))?;
            t2 = _t27;
            let _t28 = tm1.subtract_bigint(Clone::clone(&t2))?;
            tm1 = _t28;
            let mut ss = (k).wrapping_mul(32i32);
            let _t29 = vinf.shiftLeft_i(ss)?;
            let _t30 = _t29.add_bigint(Clone::clone(&t2))?;
            let _t31 = _t30.shiftLeft_i(ss)?;
            let _t32 = _t31.add_bigint(Clone::clone(&t1))?;
            let _t33 = _t32.shiftLeft_i(ss)?;
            let _t34 = _t33.add_bigint(Clone::clone(&tm1))?;
            let _t35 = _t34.shiftLeft_i(ss)?;
            let _t36 = _t35.add_bigint(Clone::clone(&v0))?;
            Ok(_t36)
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.divide:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "divideKnuth", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideKnuth(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.divideKnuth:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "divideAndRemainder", descriptor = "(Ljava/math/BigInteger;)[Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainder(&self, mut val: BigInteger) -> Result<Rc<RefCell<Vec<BigInteger>>>> {
            let this = self;
            if ((this.__get_mag().borrow().len() as i32)).wrapping_sub((val.__get_mag().borrow().len() as i32)) < 40i32 {
                let _t0 = this.divideAndRemainderKnuth(Clone::clone(&val))?;
                return Ok(_t0);
            }
            let _t0 = this.divideAndRemainderBurnikelZiegler(Clone::clone(&val))?;
            Ok(_t0)
        }

        #[java_method(name = "divideAndRemainderKnuth", descriptor = "(Ljava/math/BigInteger;)[Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainderKnuth(&self, mut val: BigInteger) -> Result<Rc<RefCell<Vec<BigInteger>>>> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<BigInteger>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            let mut result: Rc<RefCell<Vec<BigInteger>>> = _arr0;
            let mut q = MutableBigInteger::new()?;
            let mut a = MutableBigInteger::new_arr_i(Clone::clone(&this.__get_mag()))?;
            let mut b = MutableBigInteger::new_arr_i(Clone::clone(&val.__get_mag()))?;
            let _t1 = a.divideKnuth_mutabl_mutabl(Clone::clone(&b), Clone::clone(&q))?;
            let mut r: MutableBigInteger = _t1;
            let _t2 = q.toBigInteger_i((this.__get_signum() != val.__get_signum() as i32))?;
            result.borrow_mut()[0i32 as usize] = Clone::clone(&_t2);
            let _t3 = r.toBigInteger_i(this.__get_signum())?;
            result.borrow_mut()[1i32 as usize] = Clone::clone(&_t3);
            Ok(result)
        }

        #[java_method(name = "remainder", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainder(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.remainder:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "remainderKnuth", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainderKnuth(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.remainderKnuth:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "divideBurnikelZiegler", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideBurnikelZiegler(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.divideBurnikelZiegler:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "remainderBurnikelZiegler", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainderBurnikelZiegler(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.remainderBurnikelZiegler:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "divideAndRemainderBurnikelZiegler", descriptor = "(Ljava/math/BigInteger;)[Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainderBurnikelZiegler(&self, mut val: BigInteger) -> Result<Rc<RefCell<Vec<BigInteger>>>> {
            let this = self;
            let mut q = MutableBigInteger::new()?;
            let _t0 = MutableBigInteger::new_bigint(Clone::clone(this))?.divideAndRemainderBurnikelZiegler(Clone::clone(&MutableBigInteger::new_bigint(Clone::clone(&val))?), Clone::clone(&q))?;
            let mut r: MutableBigInteger = _t0;
            let _t1 = q.isZero()?;
            let mut _merged3: BigInteger;
            if _t1 {
                _merged3 = BigInteger::ZERO();
            } else {
                let _t2 = q.toBigInteger_i((this.__get_signum()).wrapping_mul(val.__get_signum()))?;
                _merged3 = _t2;
            }
            let mut qBigInt: BigInteger = _merged3;
            let _t4 = r.isZero()?;
            let mut _merged6: BigInteger;
            if _t4 {
                _merged6 = BigInteger::ZERO();
            } else {
                let _t5 = r.toBigInteger_i(this.__get_signum())?;
                _merged6 = _t5;
            }
            let mut rBigInt: BigInteger = _merged6;
            let mut _arr7: Rc<RefCell<Vec<BigInteger>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            _arr7.borrow_mut()[0i32 as usize] = Clone::clone(&qBigInt);
            _arr7.borrow_mut()[1i32 as usize] = Clone::clone(&rBigInt);
            Ok(_arr7)
        }

        #[java_method(name = "pow", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pow(&self, mut exponent: i32) -> Result<BigInteger> {
            let this = self;
            if (exponent<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            return Ok((if (exponent==0) { BigInteger::ONE() } else { Clone::clone(this) }));
            let _t0 = this.abs()?;
            let mut partToSquare: BigInteger = _t0;
            let _t1 = partToSquare.getLowestSetBit()?;
            let mut powersOfTwo: i32 = _t1;
            let mut bitsToShiftLong = ((powersOfTwo as i64)).wrapping_mul((exponent as i64));
            if (((bitsToShiftLong>(2147483647i64)) as i32-((bitsToShiftLong)<(2147483647i64)) as i32)>0) {
                BigInteger::reportOverflow()?;
            }
            let mut bitsToShift: i32 = (bitsToShiftLong as i32);
            let _t2 = partToSquare.shiftRight(powersOfTwo)?;
            partToSquare = _t2;
            let _t3 = partToSquare.bitLength()?;
            let mut remainingBits: i32 = _t3;
            if (exponent&1i32) == 1i32 {
                let _t4 = BigInteger::NEGATIVE_ONE().shiftLeft_i(bitsToShift)?;
                return Ok(_t4);
            }
            let _t4 = BigInteger::ONE().shiftLeft_i(bitsToShift)?;
            return Ok(_t4);
            let _t5 = partToSquare.bitLength()?;
            remainingBits = _t5;
            if (exponent&1i32) == 1i32 {
                return Ok(BigInteger::NEGATIVE_ONE());
            }
            return Ok(BigInteger::ONE());
            let mut scaleFactor = ((remainingBits as i64)).wrapping_mul((exponent as i64));
            let mut newSign = ((if (this.__get_signum()<0) { (exponent&1i32) != 1i32 } else { (1i32 != 0) })) as i32;
            let mut result: i64 = 1i64;
            let mut baseToPow2 = ((partToSquare.__get_mag().borrow()[0i32 as usize] as i64)&(4294967295i64));
            let mut workingExponent: i32 = exponent;
            loop {
                if (workingExponent==0) { break; }
                if (workingExponent&1i32) == 1i32 {
                    result = (result).wrapping_mul(baseToPow2);
                }
                workingExponent = ((workingExponent as u32>>(1i32&0x1f)) as i32);
                baseToPow2 = (baseToPow2).wrapping_mul(baseToPow2);
            }
            if (((((bitsToShift as i64)).wrapping_add(scaleFactor)>(62i64)) as i32-((((bitsToShift as i64)).wrapping_add(scaleFactor))<(62i64)) as i32)<=0) {
                return Ok(((result).wrapping_shl((bitsToShift&0x3f) as u32)).wrapping_mul((newSign as i64)));
            }
            let _t6 = (result).wrapping_mul((newSign as i64)).shiftLeft(bitsToShift)?;
            return Ok(_t6);
            return Ok((result).wrapping_mul((newSign as i64)));
            let _t7 = this.bitLength()?;
            if ((((((_t7 as i64)).wrapping_mul((exponent as i64))/32i64)>(67108864i64)) as i32-(((((_t7 as i64)).wrapping_mul((exponent as i64))/32i64))<(67108864i64)) as i32)>0) {
                BigInteger::reportOverflow()?;
            }
            let mut newSign: BigInteger = BigInteger::ONE();
            let mut result: i32 = exponent;
            loop {
                if (result==0) { break; }
                if (result&1i32) == 1i32 {
                    let _t8 = newSign.multiply_bigint(Clone::clone(&partToSquare))?;
                    newSign = _t8;
                }
                result = ((result as u32>>(1i32&0x1f)) as i32);
                let _t8 = partToSquare.square()?;
                partToSquare = _t8;
            }
            if (powersOfTwo>0) {
                let _t8 = newSign.shiftLeft_i(bitsToShift)?;
                newSign = _t8;
            }
            if (exponent&1i32) == 1i32 {
                let _t8 = newSign.negate()?;
                return Ok(_t8);
            }
            Ok(newSign)
        }

        #[java_method(name = "sqrt", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sqrt(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.sqrt:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "sqrtAndRemainder", descriptor = "()[Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sqrtAndRemainder(&self) -> Result<Rc<RefCell<Vec<BigInteger>>>> {
            panic!("stub: java/math/BigInteger.sqrtAndRemainder:()[Ljava/math/BigInteger;")
        }

        #[java_method(name = "gcd", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn gcd(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.gcd:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "bitLengthForInt", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bitLengthForInt(mut n: i32) -> Result<i32> {
            let _t0: i32 = Integer::numberOfLeadingZeros(n)?;
            Ok((32i32).wrapping_sub(_t0))
        }

        #[java_method(name = "leftShift", descriptor = "([III)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn leftShift(a: Rc<RefCell<Vec<i32>>>, len: i32, n: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.leftShift:([III)[I")
        }

        #[java_method(name = "primitiveRightShift", descriptor = "([III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn primitiveRightShift(a: Rc<RefCell<Vec<i32>>>, len: i32, n: i32) -> Result<()> {
            panic!("stub: java/math/BigInteger.primitiveRightShift:([III)V")
        }

        #[java_method(name = "primitiveLeftShift", descriptor = "([III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn primitiveLeftShift(mut a: Rc<RefCell<Vec<i32>>>, mut len: i32, mut n: i32) -> Result<()> {
            if (n==0) {
                return Ok(());
            }
            let _t0: i32 = Objects::checkFromToIndex_i_i_i(0i32, len, (a.borrow().len() as i32))?;
            BigInteger::shiftLeftImplWorker(Clone::clone(&a), Clone::clone(&a), 0i32, n, (len).wrapping_sub(1i32))?;
            let _tmp_val1 = (a.borrow()[(len).wrapping_sub(1i32) as usize]<<(n&0x1f));
            a.borrow_mut()[(len).wrapping_sub(1i32) as usize] = _tmp_val1;
            Ok(())
        }

        #[java_method(name = "bitLength", descriptor = "([II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: bitLength([II)I
        pub fn bitLength_arr_i_i(mut val: Rc<RefCell<Vec<i32>>>, mut len: i32) -> Result<i32> {
            if (len==0) {
                return Ok(0i32);
            }
            let _t0: i32 = BigInteger::bitLengthForInt(val.borrow()[0i32 as usize])?;
            Ok((((len).wrapping_sub(1i32)<<(5i32&0x1f))).wrapping_add(_t0))
        }

        #[java_method(name = "abs", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs(&self) -> Result<BigInteger> {
            let this = self;
            let mut _merged1: BigInteger;
            if (this.__get_signum()>=0) {
                _merged1 = Clone::clone(this);
            } else {
                let _t0 = this.negate()?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "negate", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negate(&self) -> Result<BigInteger> {
            let this = self;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&this.__get_mag()), (this.__get_signum()).wrapping_neg())?)
        }

        #[java_method(name = "signum", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_signum())
        }

        #[java_method(name = "mod", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mod_(&self, m: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.mod:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "modPow", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modPow(&self, exponent: BigInteger, m: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.modPow:(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "montgomeryMultiply", descriptor = "([I[I[IIJ[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn montgomeryMultiply(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>, n: Rc<RefCell<Vec<i32>>>, len: i32, inv: i64, arg5: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.montgomeryMultiply:([I[I[IIJ[I)[I")
        }

        #[java_method(name = "montgomerySquare", descriptor = "([I[IIJ[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn montgomerySquare(a: Rc<RefCell<Vec<i32>>>, n: Rc<RefCell<Vec<i32>>>, len: i32, inv: i64, arg4: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.montgomerySquare:([I[IIJ[I)[I")
        }

        #[java_method(name = "implMontgomeryMultiplyChecks", descriptor = "([I[I[II[I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/RuntimeException")]
        pub fn implMontgomeryMultiplyChecks(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>, n: Rc<RefCell<Vec<i32>>>, len: i32, product: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            panic!("stub: java/math/BigInteger.implMontgomeryMultiplyChecks:([I[I[II[I)V")
        }

        #[java_method(name = "materialize", descriptor = "([II)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn materialize(z: Rc<RefCell<Vec<i32>>>, len: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.materialize:([II)[I")
        }

        #[java_method(name = "implMontgomeryMultiply", descriptor = "([I[I[IIJ[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implMontgomeryMultiply(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>, n: Rc<RefCell<Vec<i32>>>, len: i32, inv: i64, arg5: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.implMontgomeryMultiply:([I[I[IIJ[I)[I")
        }

        #[java_method(name = "implMontgomerySquare", descriptor = "([I[IIJ[I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implMontgomerySquare(a: Rc<RefCell<Vec<i32>>>, n: Rc<RefCell<Vec<i32>>>, len: i32, inv: i64, arg4: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.implMontgomerySquare:([I[IIJ[I)[I")
        }

        #[java_method(name = "oddModPow", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn oddModPow(&self, y: BigInteger, z: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.oddModPow:(Ljava/math/BigInteger;Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "montReduce", descriptor = "([I[III)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn montReduce(n: Rc<RefCell<Vec<i32>>>, mod_: Rc<RefCell<Vec<i32>>>, mlen: i32, inv: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.montReduce:([I[III)[I")
        }

        #[java_method(name = "intArrayCmpToLen", descriptor = "([I[II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intArrayCmpToLen(arg1: Rc<RefCell<Vec<i32>>>, arg2: Rc<RefCell<Vec<i32>>>, len: i32) -> Result<i32> {
            panic!("stub: java/math/BigInteger.intArrayCmpToLen:([I[II)I")
        }

        #[java_method(name = "subN", descriptor = "([I[II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subN(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>, len: i32) -> Result<i32> {
            panic!("stub: java/math/BigInteger.subN:([I[II)I")
        }

        #[java_method(name = "mulAdd", descriptor = "([I[IIII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mulAdd(mut out: Rc<RefCell<Vec<i32>>>, mut in_: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut len: i32, mut k: i32) -> Result<i32> {
            BigInteger::implMulAddCheck(Clone::clone(&out), Clone::clone(&in_), offset, len, k)?;
            let _t0: i32 = BigInteger::implMulAdd(Clone::clone(&out), Clone::clone(&in_), offset, len, k)?;
            Ok(_t0)
        }

        #[java_method(name = "implMulAddCheck", descriptor = "([I[IIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implMulAddCheck(mut out: Rc<RefCell<Vec<i32>>>, mut in_: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut len: i32, mut k: i32) -> Result<()> {
            if len > (in_.borrow().len() as i32) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input length is out of bound: ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i((in_.borrow().len() as i32))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (offset<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input offset is invalid: ")))?;
                let _t1 = _t0.append_i(offset)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if offset > ((out.borrow().len() as i32)).wrapping_sub(1i32) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input offset is out of bound: ")))?;
                let _t1 = _t0.append_i(offset)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i(((out.borrow().len() as i32)).wrapping_sub(1i32))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if len > ((out.borrow().len() as i32)).wrapping_sub(offset) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("input len is out of bound: ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i(((out.borrow().len() as i32)).wrapping_sub(offset))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "implMulAdd", descriptor = "([I[IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implMulAdd(mut out: Rc<RefCell<Vec<i32>>>, mut in_: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut len: i32, mut k: i32) -> Result<i32> {
            let mut kLong = ((k as i64)&(4294967295i64));
            let mut carry: i64 = 0i64;
            offset = (((out.borrow().len() as i32)).wrapping_sub(offset)).wrapping_sub(1i32);
            let mut j = (len).wrapping_sub(1i32);
            loop {
                if (j<0) { break; }
                let mut product = (((((in_.borrow()[j as usize] as i64)&(4294967295i64))).wrapping_mul(kLong)).wrapping_add(((out.borrow()[offset as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                offset = offset.wrapping_sub(1i32);
                out.borrow_mut()[offset as usize] = (product as i32);
                carry = ((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                j = j.wrapping_sub(1i32);
            }
            Ok((carry as i32))
        }

        #[java_method(name = "addOne", descriptor = "([IIII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addOne(mut a: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut mlen: i32, mut carry: i32) -> Result<i32> {
            offset = ((((a.borrow().len() as i32)).wrapping_sub(1i32)).wrapping_sub(mlen)).wrapping_sub(offset);
            let mut t = (((a.borrow()[offset as usize] as i64)&(4294967295i64))).wrapping_add(((carry as i64)&(4294967295i64)));
            a.borrow_mut()[offset as usize] = (t as i32);
            if (((((t as u64).wrapping_shr((32i32&0x3f) as u32) as i64)>(0i64)) as i32-((((t as u64).wrapping_shr((32i32&0x3f) as u32) as i64))<(0i64)) as i32)==0) {
                return Ok(0i32);
            }
            loop {
                mlen = mlen.wrapping_sub(1i32);
                offset = offset.wrapping_sub(1i32);
                if (offset<0) {
                    return Ok(1i32);
                }
                let _tmp_val0 = (a.borrow()[offset as usize]).wrapping_add(1i32);
                a.borrow_mut()[offset as usize] = _tmp_val0;
                if (a.borrow()[offset as usize]!=0) { break; }
            }
            return Ok(0i32);
            Ok(1i32)
        }

        #[java_method(name = "modPow2", descriptor = "(Ljava/math/BigInteger;I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modPow2(&self, exponent: BigInteger, p: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.modPow2:(Ljava/math/BigInteger;I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "mod2", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mod2(&self, p: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.mod2:(I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "modInverse", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modInverse(&self, m: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.modInverse:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "shiftLeft", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: shiftLeft(I)Ljava/math/BigInteger;
        pub fn shiftLeft_i(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(BigInteger::ZERO());
            }
            if (n>0) {
                let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::shiftLeft_arr_i_i(Clone::clone(&this.__get_mag()), n)?;
                return Ok(BigInteger::new_arr_i_i(Clone::clone(&_t0), this.__get_signum())?);
            }
            if (n==0) {
                return Ok(Clone::clone(this));
            }
            let _t0 = this.shiftRightImpl((n).wrapping_neg())?;
            Ok(_t0)
        }

        #[java_method(name = "shiftLeft", descriptor = "([II)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: shiftLeft([II)[I
        pub fn shiftLeft_arr_i_i(mut mag: Rc<RefCell<Vec<i32>>>, mut n: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut nInts = ((n as u32>>(5i32&0x1f)) as i32);
            let mut nBits = (n&31i32);
            let mut magLen = (mag.borrow().len() as i32);
            let mut newMag: Object = Object::default();
            if (nBits==0) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (magLen).wrapping_add(nInts) as usize]));
                let mut newMag: Rc<RefCell<Vec<i32>>> = _arr0;
                System::arraycopy(Object::from_any(mag.clone()), 0i32, Object::from_any(newMag.clone()), 0i32, magLen)?;
            } else {
                let mut i: i32 = 0i32;
                let mut nBits2 = (32i32).wrapping_sub(nBits);
                let mut highBits = ((mag.borrow()[0i32 as usize] as u32>>(nBits2&0x1f)) as i32);
                if (highBits!=0) {
                    let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((magLen).wrapping_add(nInts)).wrapping_add(1i32) as usize]));
                    let mut newMag: Rc<RefCell<Vec<i32>>> = _arr0;
                    i = i.wrapping_add(1i32);
                    newMag.borrow_mut()[i as usize] = highBits;
                } else {
                    let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (magLen).wrapping_add(nInts) as usize]));
                    let mut newMag: Rc<RefCell<Vec<i32>>> = _arr0;
                }
                let mut numIter = (magLen).wrapping_sub(1i32);
                let _t0: i32 = Objects::checkFromToIndex_i_i_i(0i32, (numIter).wrapping_add(1i32), (mag.borrow().len() as i32))?;
                let _t1: i32 = Objects::checkFromToIndex_i_i_i(i, ((numIter).wrapping_add(i)).wrapping_add(1i32), (newMag.borrow().len() as i32))?;
                BigInteger::shiftLeftImplWorker(Clone::clone(&newMag), Clone::clone(&mag), i, nBits, numIter)?;
                newMag.borrow_mut()[(numIter).wrapping_add(i) as usize] = (mag.borrow()[numIter as usize]<<(nBits&0x1f));
            }
            Ok(newMag)
        }

        #[java_method(name = "shiftLeftImplWorker", descriptor = "([I[IIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shiftLeftImplWorker(mut newArr: Rc<RefCell<Vec<i32>>>, mut oldArr: Rc<RefCell<Vec<i32>>>, mut newIdx: i32, mut shiftCount: i32, mut numIter: i32) -> Result<()> {
            let mut shiftCountRight = (32i32).wrapping_sub(shiftCount);
            let mut oldIdx: i32 = 0i32;
            loop {
                if oldIdx >= numIter { break; }
                newIdx = newIdx.wrapping_add(1i32);
                oldIdx = oldIdx.wrapping_add(1i32);
                newArr.borrow_mut()[newIdx as usize] = ((oldArr.borrow()[oldIdx as usize]<<(shiftCount&0x1f))|((oldArr.borrow()[oldIdx as usize] as u32>>(shiftCountRight&0x1f)) as i32));
            }
            Ok(())
        }

        #[java_method(name = "shiftRight", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shiftRight(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(BigInteger::ZERO());
            }
            if (n>0) {
                let _t0 = this.shiftRightImpl(n)?;
                return Ok(_t0);
            }
            if (n==0) {
                return Ok(Clone::clone(this));
            }
            let _t0: Rc<RefCell<Vec<i32>>> = BigInteger::shiftLeft_arr_i_i(Clone::clone(&this.__get_mag()), (n).wrapping_neg())?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t0), this.__get_signum())?)
        }

        #[java_method(name = "shiftRightImpl", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shiftRightImpl(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            let mut nInts = ((n as u32>>(5i32&0x1f)) as i32);
            let mut nBits = (n&31i32);
            let mut magLen = (this.__get_mag().borrow().len() as i32);
            let mut newMag: Object = Object::default();
            return Ok((if (this.__get_signum()>=0) { BigInteger::ZERO() } else { Clone::clone(&BigInteger::negConst().borrow()[1i32 as usize]) }));
            if (nBits==0) {
                let mut newMagLen = (magLen).wrapping_sub(nInts);
                let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&this.__get_mag()), newMagLen)?;
                let mut newMag: Rc<RefCell<Vec<i32>>> = _t0;
            } else {
                let mut newMagLen: i32 = 0i32;
                let mut highBits = ((this.__get_mag().borrow()[0i32 as usize] as u32>>(nBits&0x1f)) as i32);
                if (highBits!=0) {
                    let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (magLen).wrapping_sub(nInts) as usize]));
                    let mut newMag: Rc<RefCell<Vec<i32>>> = _arr0;
                    newMagLen = newMagLen.wrapping_add(1i32);
                    newMag.borrow_mut()[newMagLen as usize] = highBits;
                } else {
                    let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((magLen).wrapping_sub(nInts)).wrapping_sub(1i32) as usize]));
                    let mut newMag: Rc<RefCell<Vec<i32>>> = _arr0;
                }
                let mut numIter = ((magLen).wrapping_sub(nInts)).wrapping_sub(1i32);
                let _t0: i32 = Objects::checkFromToIndex_i_i_i(0i32, (numIter).wrapping_add(1i32), (this.__get_mag().borrow().len() as i32))?;
                let _t1: i32 = Objects::checkFromToIndex_i_i_i(newMagLen, (numIter).wrapping_add(newMagLen), (newMag.borrow().len() as i32))?;
                BigInteger::shiftRightImplWorker(Clone::clone(&newMag), Clone::clone(&this.__get_mag()), newMagLen, nBits, numIter)?;
            }
            let mut newMagLen: i32 = 0i32;
            let mut highBits = (magLen).wrapping_sub(1i32);
            let mut numIter = (magLen).wrapping_sub(nInts);
            loop {
                if highBits < numIter { break; }
                newMagLen = ((this.__get_mag().borrow()[highBits as usize]!=0)) as i32;
                highBits = highBits.wrapping_sub(1i32);
            }
            newMagLen = (((this.__get_mag().borrow()[((magLen).wrapping_sub(nInts)).wrapping_sub(1i32) as usize]<<((32i32).wrapping_sub(nBits)&0x1f))!=0)) as i32;
            if (newMagLen!=0) {
                let _t0 = this.javaIncrement(Clone::clone(&newMag))?;
                let mut newMag = _t0;
            }
            Ok(BigInteger::new_arr_i_i(Clone::clone(&newMag), this.__get_signum())?)
        }

        #[java_method(name = "shiftRightImplWorker", descriptor = "([I[IIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shiftRightImplWorker(mut newArr: Rc<RefCell<Vec<i32>>>, mut oldArr: Rc<RefCell<Vec<i32>>>, mut newIdx: i32, mut shiftCount: i32, mut numIter: i32) -> Result<()> {
            let mut shiftCountLeft = (32i32).wrapping_sub(shiftCount);
            let mut idx: i32 = numIter;
            let mut nidx = (if (newIdx==0) { (numIter).wrapping_sub(1i32) } else { numIter });
            loop {
                if nidx < newIdx { break; }
                nidx = nidx.wrapping_sub(1i32);
                idx = idx.wrapping_sub(1i32);
                newArr.borrow_mut()[nidx as usize] = (((oldArr.borrow()[idx as usize] as u32>>(shiftCount&0x1f)) as i32)|(oldArr.borrow()[idx as usize]<<(shiftCountLeft&0x1f)));
            }
            Ok(())
        }

        #[java_method(name = "javaIncrement", descriptor = "([I)[I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaIncrement(&self, mut val: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            let this = self;
            let mut lastSum: i32 = 0i32;
            let mut i = ((val.borrow().len() as i32)).wrapping_sub(1i32);
            loop {
                if (i<0) { break; }
                if (lastSum==0) {
                    let _tmp_val0 = (val.borrow()[i as usize]).wrapping_add(1i32);
                    val.borrow_mut()[i as usize] = _tmp_val0;
                    lastSum = (val.borrow()[i as usize]).wrapping_add(1i32);
                    i = i.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            if (lastSum==0) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((val.borrow().len() as i32)).wrapping_add(1i32) as usize]));
                val = _arr0;
                val.borrow_mut()[0i32 as usize] = 1i32;
            }
            Ok(val)
        }

        #[java_method(name = "and", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn and(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.and:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "or", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn or(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.or:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "xor", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn xor(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.xor:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "not", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn not(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.not:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "andNot", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn andNot(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.andNot:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "testBit", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn testBit(&self, mut n: i32) -> Result<bool> {
            let this = self;
            if (n<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.getInt(((n as u32>>(5i32&0x1f)) as i32))?;
            Ok(((_t0&(1i32<<((n&31i32)&0x1f)))!=0))
        }

        #[java_method(name = "setBit", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setBit(&self, n: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.setBit:(I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "clearBit", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearBit(&self, n: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.clearBit:(I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "flipBit", descriptor = "(I)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flipBit(&self, n: i32) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.flipBit:(I)Ljava/math/BigInteger;")
        }

        #[java_method(name = "getLowestSetBit", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLowestSetBit(&self) -> Result<i32> {
            let this = self;
            let mut lsb = (this.__get_lowestSetBitPlusTwo()).wrapping_sub(2i32);
            lsb = 0i32;
            if (this.__get_signum()==0) {
                lsb = lsb.wrapping_sub(1i32);
            } else {
                let mut i: i32 = 0i32;
                let mut b: i32 = Default::default();
                loop {
                    let _t0 = this.getInt(i)?;
                    b = _t0;
                    if (b!=0) { break; }
                    i = i.wrapping_add(1i32);
                }
                let _t0: i32 = Integer::numberOfTrailingZeros(b)?;
                lsb = (lsb).wrapping_add(((i<<(5i32&0x1f))).wrapping_add(_t0));
            }
            this.__set_lowestSetBitPlusTwo((lsb).wrapping_add(2i32));
            Ok(lsb)
        }

        #[java_method(name = "bitLength", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: bitLength()I
        pub fn bitLength(&self) -> Result<i32> {
            let this = self;
            let mut n = (this.__get_bitLengthPlusOne()).wrapping_sub(1i32);
            let mut m = this.__get_mag();
            let mut len = (m.borrow().len() as i32);
            if (len==0) {
                n = 0i32;
            } else {
                let _t0: i32 = BigInteger::bitLengthForInt(this.__get_mag().borrow()[0i32 as usize])?;
                let mut magBitLength = (((len).wrapping_sub(1i32)<<(5i32&0x1f))).wrapping_add(_t0);
                if (this.__get_signum()<0) {
                    let _t1: i32 = Integer::bitCount(this.__get_mag().borrow()[0i32 as usize])?;
                    let mut pow2 = (_t1 == 1i32) as i32;
                    let mut i: i32 = 1i32;
                    loop {
                        if i >= len { break; }
                        pow2 = ((this.__get_mag().borrow()[i as usize]==0)) as i32;
                        i = i.wrapping_add(1i32);
                    }
                    n = (if (pow2!=0) { (magBitLength).wrapping_sub(1i32) } else { magBitLength });
                } else {
                    n = magBitLength;
                }
            }
            this.__set_bitLengthPlusOne((n).wrapping_add(1i32));
            Ok(n)
        }

        #[java_method(name = "bitCount", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bitCount(&self) -> Result<i32> {
            panic!("stub: java/math/BigInteger.bitCount:()I")
        }

        #[java_method(name = "isProbablePrime", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isProbablePrime(&self, certainty: i32) -> Result<bool> {
            panic!("stub: java/math/BigInteger.isProbablePrime:(I)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/math/BigInteger;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, val: BigInteger) -> Result<i32> {
            panic!("stub: java/math/BigInteger.compareTo:(Ljava/math/BigInteger;)I")
        }

        #[java_method(name = "compareMagnitude", descriptor = "(Ljava/math/BigInteger;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareMagnitude(Ljava/math/BigInteger;)I
        pub fn compareMagnitude_bigint(&self, mut val: BigInteger) -> Result<i32> {
            let this = self;
            let mut m1 = this.__get_mag();
            let mut len1 = (m1.borrow().len() as i32);
            let mut m2 = val.__get_mag();
            let mut len2 = (m2.borrow().len() as i32);
            if len1 < len2 {
                return Ok(-1i32);
            }
            if len1 > len2 {
                return Ok(1i32);
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= len1 { break; }
                let mut a = m1.borrow()[i as usize];
                let mut b = m2.borrow()[i as usize];
                return Ok(((((((a as i64)&(4294967295i64))>(((b as i64)&(4294967295i64)))) as i32-((((a as i64)&(4294967295i64)))<(((b as i64)&(4294967295i64)))) as i32)>=0)) as i32);
                i = i.wrapping_add(1i32);
            }
            Ok(0i32)
        }

        #[java_method(name = "compareMagnitude", descriptor = "(J)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareMagnitude_l(&self, val: i64) -> Result<i32> {
            panic!("stub: java/math/BigInteger.compareMagnitude:(J)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut x: Object) -> Result<bool> {
            let this = self;
            if x == Object::from_any(this.clone()) {
                return Ok((1i32 != 0i32));
            }
        let mut xInt = Default::default();
            if (x.is_instance_of("java/math/BigInteger")) {
                xInt = (x).downcast::<BigInteger>();
            } else {
                return Ok((0i32 != 0i32));
            }
            if xInt.__get_signum() != this.__get_signum() {
                return Ok((0i32 != 0i32));
            }
            let mut m = this.__get_mag();
            let mut len = (m.borrow().len() as i32);
            let mut xm = xInt.__get_mag();
            if len != (xm.borrow().len() as i32) {
                return Ok((0i32 != 0i32));
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                if xm.borrow()[i as usize] != m.borrow()[i as usize] {
                    return Ok((0i32 != 0i32));
                }
                i = i.wrapping_add(1i32);
            }
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "min", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.min:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "max", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(&self, val: BigInteger) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger.max:(Ljava/math/BigInteger;)Ljava/math/BigInteger;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString(I)Ljava/lang/String;
        pub fn toString_i(&self, mut radix: i32) -> Result<String> {
            let this = self;
            if (this.__get_signum()==0) {
                return Ok(String::from("0"));
            }
            if radix > 36i32 {
                radix = 10i32;
            }
            let _t0 = this.abs()?;
            let mut abs: BigInteger = _t0;
            let _t1 = abs.bitLength()?;
            let mut b: i32 = _t1;
            let _t2: f64 = Math::floor((((b as f64)*BigInteger::LOG_TWO())/BigInteger::logCache().borrow()[radix as usize]))?;
            let mut numChars = (((_t2+1f64) as i32)).wrapping_add(((this.__get_signum()<0) as i32));
            let mut sb = StringBuilder::new_i(numChars)?;
            if (this.__get_signum()<0) {
                let _t3 = sb.append_c(((45i32) as u16))?;
            }
            BigInteger::toString_bigint_sb_i_i(Clone::clone(&abs), Clone::clone(&sb), radix, 0i32)?;
            let _t3 = sb.toString()?;
            Ok(_t3)
        }

        #[java_method(name = "padWithZeros", descriptor = "(Ljava/lang/StringBuilder;I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn padWithZeros(mut buf: StringBuilder, mut numZeros: i32) -> Result<()> {
            loop {
                if numZeros < BigInteger::NUM_ZEROS() { break; }
                let _t0 = buf.append_str(Clone::clone(&BigInteger::ZEROS()))?;
                numZeros = (numZeros).wrapping_sub(BigInteger::NUM_ZEROS());
            }
            if (numZeros>0) {
                let _t0 = buf.append_seq_i_i(Object::from_any(BigInteger::ZEROS().clone()), 0i32, numZeros)?;
            }
            Ok(())
        }

        #[java_method(name = "smallToString", descriptor = "(ILjava/lang/StringBuilder;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn smallToString(&self, mut radix: i32, mut buf: StringBuilder, mut digits: i32) -> Result<()> {
            let this = self;
            if (this.__get_signum()<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (this.__get_signum()==0) {
                BigInteger::padWithZeros(Clone::clone(&buf), digits)?;
                return Ok(());
            }
            let mut maxNumDigitGroups = (((4i32).wrapping_mul((this.__get_mag().borrow().len() as i32))).wrapping_add(6i32)/7i32);
            let mut _arr0: Rc<RefCell<Vec<i64>>> = Rc::new(RefCell::new(vec![0i64; maxNumDigitGroups as usize]));
            let mut digitGroups: Rc<RefCell<Vec<i64>>> = _arr0;
            let mut tmp = this.clone();
            let mut numGroups: i32 = 0i32;
            loop {
                if (tmp.__get_signum()==0) { break; }
                let mut d = Clone::clone(&BigInteger::longRadix().borrow()[radix as usize]);
                let mut q = MutableBigInteger::new()?;
                let mut a = MutableBigInteger::new_arr_i(Clone::clone(&tmp.__get_mag()))?;
                let mut b = MutableBigInteger::new_arr_i(Clone::clone(&d.__get_mag()))?;
                let _t1 = a.divide_mutabl_mutabl(Clone::clone(&b), Clone::clone(&q))?;
                let mut r: MutableBigInteger = _t1;
                let _t2 = q.toBigInteger_i((tmp.__get_signum()).wrapping_mul(d.__get_signum()))?;
                let mut q2: BigInteger = _t2;
                let _t3 = r.toBigInteger_i((tmp.__get_signum()).wrapping_mul(d.__get_signum()))?;
                let mut r2: BigInteger = _t3;
                numGroups = numGroups.wrapping_add(1i32);
                let _t4: i64 = r2.longValue()?;
                digitGroups.borrow_mut()[numGroups as usize] = _t4;
                tmp = q2;
            }
            let _t1: String = Long::toString_l_i(digitGroups.borrow()[(numGroups).wrapping_sub(1i32) as usize], radix)?;
            let mut d: String = _t1;
            let _t2 = d.length()?;
            BigInteger::padWithZeros(Clone::clone(&buf), (digits).wrapping_sub((_t2).wrapping_add(((numGroups).wrapping_sub(1i32)).wrapping_mul(BigInteger::digitsPerLong().borrow()[radix as usize]))))?;
            let _t3 = buf.append_str(Clone::clone(&d))?;
            let mut q = (numGroups).wrapping_sub(2i32);
            loop {
                if (q<0) { break; }
                let _t4: String = Long::toString_l_i(digitGroups.borrow()[q as usize], radix)?;
                d = _t4;
                let _t5 = d.length()?;
                let mut a = (BigInteger::digitsPerLong().borrow()[radix as usize]).wrapping_sub(_t5);
                if (a!=0) {
                    let _t6 = buf.append_seq_i_i(Object::from_any(BigInteger::ZEROS().clone()), 0i32, a)?;
                }
                let _t6 = buf.append_str(Clone::clone(&d))?;
                q = q.wrapping_sub(1i32);
            }
            Ok(())
        }

        #[java_method(name = "toString", descriptor = "(Ljava/math/BigInteger;Ljava/lang/StringBuilder;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString(Ljava/math/BigInteger;Ljava/lang/StringBuilder;II)V
        pub fn toString_bigint_sb_i_i(mut u: BigInteger, mut sb: StringBuilder, mut radix: i32, mut digits: i32) -> Result<()> {
            let _t0 = u.signum()?;
            if (_t0<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (u.__get_mag().borrow().len() as i32) <= 20i32 {
                u.smallToString(radix, Clone::clone(&sb), digits)?;
                return Ok(());
            }
            let _t1 = u.bitLength()?;
            let mut b: i32 = _t1;
            let _t2: f64 = Math::log((((b as f64)*BigInteger::LOG_TWO())/BigInteger::logCache().borrow()[radix as usize]))?;
            let _t3: i64 = Math::round_d(((_t2/BigInteger::LOG_TWO())-1f64))?;
            let mut n: i32 = (_t3 as i32);
            let _t4: BigInteger = BigInteger::getRadixConversionCache(radix, n)?;
            let mut v: BigInteger = _t4;
            let _t5 = u.divideAndRemainder(Clone::clone(&v))?;
            let mut results: Rc<RefCell<Vec<BigInteger>>> = _t5;
            let mut expectedDigits = (1i32<<(n&0x1f));
            BigInteger::toString_bigint_sb_i_i(Clone::clone(&Clone::clone(&results.borrow()[0i32 as usize])), Clone::clone(&sb), radix, (digits).wrapping_sub(expectedDigits))?;
            BigInteger::toString_bigint_sb_i_i(Clone::clone(&Clone::clone(&results.borrow()[1i32 as usize])), Clone::clone(&sb), radix, expectedDigits)?;
            Ok(())
        }

        #[java_method(name = "getRadixConversionCache", descriptor = "(II)Ljava/math/BigInteger;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRadixConversionCache(mut radix: i32, mut exponent: i32) -> Result<BigInteger> {
            let mut cacheLine = Clone::clone(&BigInteger::powerCache().borrow()[radix as usize]);
            if exponent < (cacheLine.borrow().len() as i32) {
                return Ok(Clone::clone(&cacheLine.borrow()[exponent as usize]));
            }
            let mut oldLength = (cacheLine.borrow().len() as i32);
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (exponent).wrapping_add(1i32))?;
            cacheLine = Default::default();
            let mut i: i32 = oldLength;
            loop {
                if i > exponent { break; }
                let _t1 = Clone::clone(&cacheLine.borrow()[(i).wrapping_sub(1i32) as usize]).pow(2i32)?;
                cacheLine.borrow_mut()[i as usize] = Clone::clone(&_t1);
                i = i.wrapping_add(1i32);
            }
            let mut i: Rc<RefCell<Vec<Rc<RefCell<Vec<BigInteger>>>>>> = BigInteger::powerCache();
            if exponent >= (Clone::clone(&i.borrow()[radix as usize]).borrow().len() as i32) {
                let _t1: Object = Object::from_any(i.clone());
                i = (_t1).downcast::<Rc<RefCell<Vec<Rc<RefCell<Vec<BigInteger>>>>>>>();
                i.borrow_mut()[radix as usize] = Clone::clone(&cacheLine);
                BigInteger::set_powerCache(i);
            }
            Ok(Clone::clone(&cacheLine.borrow()[exponent as usize]))
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString()Ljava/lang/String;
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let _t0 = this.toString_i(10i32)?;
            Ok(_t0)
        }

        #[java_method(name = "toByteArray", descriptor = "()[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toByteArray(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            let this = self;
            let _t0 = this.bitLength()?;
            let mut byteLen = ((_t0/8i32)).wrapping_add(1i32);
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; byteLen as usize]));
            let mut byteArray: Rc<RefCell<Vec<i8>>> = _arr1;
            let mut i = (byteLen).wrapping_sub(1i32);
            let mut bytesCopied: i32 = 4i32;
            let mut nextInt: i32 = 0i32;
            let mut intIndex: i32 = 0i32;
            loop {
                if (i<0) { break; }
                if bytesCopied == 4i32 {
                    intIndex = intIndex.wrapping_add(1i32);
                    let _t2 = this.getInt(intIndex)?;
                    nextInt = _t2;
                    bytesCopied = 1i32;
                } else {
                    nextInt = ((nextInt as u32>>(8i32&0x1f)) as i32);
                    bytesCopied = bytesCopied.wrapping_add(1i32);
                }
                byteArray.borrow_mut()[i as usize] = (((nextInt) as i8 as i32)) as i8;
                i = i.wrapping_sub(1i32);
            }
            Ok(byteArray)
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/math/BigInteger.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            let this = self;
            let mut result: i64 = 0i64;
            let mut i: i32 = 1i32;
            loop {
                if (i<0) { break; }
                let _t0 = this.getInt(i)?;
                result = ((result).wrapping_shl((32i32&0x3f) as u32)).wrapping_add(((_t0 as i64)&(4294967295i64)));
                i = i.wrapping_sub(1i32);
            }
            Ok(result)
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/math/BigInteger.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/math/BigInteger.doubleValue:()D")
        }

        #[java_method(name = "stripLeadingZeroInts", descriptor = "([I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripLeadingZeroInts(val: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.stripLeadingZeroInts:([I)[I")
        }

        #[java_method(name = "trustedStripLeadingZeroInts", descriptor = "([I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trustedStripLeadingZeroInts(mut val: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut vlen = (val.borrow().len() as i32);
            let mut keep: i32 = 0i32;
            loop {
                if keep >= vlen { break; }
                if (val.borrow()[keep as usize]==0) {
                    keep = keep.wrapping_add(1i32);
                    continue;
                }
                break;
            }
            let mut _merged1: Rc<RefCell<Vec<i32>>>;
            if (keep==0) {
                _merged1 = val;
            } else {
                let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&val), keep, vlen)?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "stripLeadingZeroBytes", descriptor = "([BII)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripLeadingZeroBytes_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, from: i32, len: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.stripLeadingZeroBytes:([BII)[I")
        }

        #[java_method(name = "stripLeadingZeroBytes", descriptor = "(I[BII)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: stripLeadingZeroBytes(I[BII)[I
        pub fn stripLeadingZeroBytes_i_arr_b_i_i(mut b: i32, mut a: Rc<RefCell<Vec<i8>>>, mut from: i32, mut len: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            if (len==0) {
                return Ok(BigInteger::ZERO().__get_mag());
            }
            let mut to = (from).wrapping_add(len);
            if b < -128i32 {
                b = (a.borrow()[from as usize] as i32);
            }
            from = from.wrapping_add(1i32);
            loop {
                if (b!=0) { break; }
                if from < to {
                    from = from.wrapping_add(1i32);
                    b = (a.borrow()[from as usize] as i32);
                    continue;
                }
                break;
            }
            if (b==0) {
                return Ok(BigInteger::ZERO().__get_mag());
            }
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (((to).wrapping_sub(from)>>((2i32&0x1f)))).wrapping_add(1i32) as usize]));
            let mut res: Rc<RefCell<Vec<i32>>> = _arr0;
            let mut d0 = (b&255i32);
            loop {
                if (((to).wrapping_sub(from)&3i32)==0) { break; }
                from = from.wrapping_add(1i32);
                d0 = ((d0<<(8i32&0x1f))|((a.borrow()[from as usize] as i32)&255i32));
            }
            res.borrow_mut()[0i32 as usize] = d0;
            let mut i: i32 = 1i32;
            loop {
                if from >= to { break; }
                i = i.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                res.borrow_mut()[i as usize] = (((((a.borrow()[from as usize] as i32)<<(24i32&0x1f))|(((a.borrow()[from as usize] as i32)&255i32)<<(16i32&0x1f)))|(((a.borrow()[from as usize] as i32)&255i32)<<(8i32&0x1f)))|((a.borrow()[from as usize] as i32)&255i32));
            }
            Ok(res)
        }

        #[java_method(name = "makePositive", descriptor = "(I[BII)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: makePositive(I[BII)[I
        pub fn makePositive_i_arr_b_i_i(mut b: i32, mut a: Rc<RefCell<Vec<i8>>>, mut from: i32, mut len: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            let mut to = (from).wrapping_add(len);
            from = from.wrapping_add(1i32);
            loop {
                if b != -1i32 { break; }
                if from < to {
                    from = from.wrapping_add(1i32);
                    b = (a.borrow()[from as usize] as i32);
                    continue;
                }
                break;
            }
            let mut d0 = (-256i32|(b&255i32));
            loop {
                if (((to).wrapping_sub(from)&3i32)==0) { break; }
                from = from.wrapping_add(1i32);
                b = (a.borrow()[from as usize] as i32);
                d0 = ((d0<<(8i32&0x1f))|((a.borrow()[from as usize] as i32)&255i32));
            }
            let mut f: i32 = from;
            loop {
                if (b!=0) { break; }
                if from < to {
                    from = from.wrapping_add(1i32);
                    b = (a.borrow()[from as usize] as i32);
                    continue;
                }
                break;
            }
            let mut d = (b&255i32);
            loop {
                if (((to).wrapping_sub(from)&3i32)==0) { break; }
                from = from.wrapping_add(1i32);
                d = ((d<<(8i32&0x1f))|((a.borrow()[from as usize] as i32)&255i32));
            }
            let mut c = (((((to).wrapping_sub(from)|d0)|d)==0)) as i32;
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; ((c).wrapping_add(1i32)).wrapping_add(((to).wrapping_sub(f)>>((2i32&0x1f)))) as usize]));
            let mut res: Rc<RefCell<Vec<i32>>> = _arr0;
            res.borrow_mut()[0i32 as usize] = (if (c==0) { d0 } else { -1i32 });
            let mut i = ((res.borrow().len() as i32)).wrapping_sub(((to).wrapping_sub(from)>>((2i32&0x1f))));
            if i > 1i32 {
                res.borrow_mut()[(i).wrapping_sub(1i32) as usize] = d;
            }
            loop {
                if from >= to { break; }
                i = i.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                from = from.wrapping_add(1i32);
                res.borrow_mut()[i as usize] = (((((a.borrow()[from as usize] as i32)<<(24i32&0x1f))|(((a.borrow()[from as usize] as i32)&255i32)<<(16i32&0x1f)))|(((a.borrow()[from as usize] as i32)&255i32)<<(8i32&0x1f)))|((a.borrow()[from as usize] as i32)&255i32));
            }
            loop {
                i = i.wrapping_sub(1i32);
                if (i<0) { break; }
                if (res.borrow()[i as usize]==0) {
                    continue;
                }
                break;
            }
            let _tmp_val1 = (res.borrow()[i as usize]).wrapping_neg();
            res.borrow_mut()[i as usize] = _tmp_val1;
            loop {
                i = i.wrapping_sub(1i32);
                if (i<0) { break; }
                let _tmp_val2 = (res.borrow()[i as usize]^-1i32);
                res.borrow_mut()[i as usize] = _tmp_val2;
            }
            Ok(res)
        }

        #[java_method(name = "makePositive", descriptor = "([I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makePositive_arr_i(a: Rc<RefCell<Vec<i32>>>) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/BigInteger.makePositive:([I)[I")
        }

        #[java_method(name = "intLength", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intLength(&self) -> Result<i32> {
            panic!("stub: java/math/BigInteger.intLength:()I")
        }

        #[java_method(name = "signBit", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signBit(&self) -> Result<i32> {
            panic!("stub: java/math/BigInteger.signBit:()I")
        }

        #[java_method(name = "signInt", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signInt(&self) -> Result<i32> {
            let this = self;
            Ok(((this.__get_signum()<0)) as i32)
        }

        #[java_method(name = "getInt", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt(&self, mut n: i32) -> Result<i32> {
            let this = self;
            if (n<0) {
                return Ok(0i32);
            }
            if n >= (this.__get_mag().borrow().len() as i32) {
                let _t0 = this.signInt()?;
                return Ok(_t0);
            }
            let mut magInt = this.__get_mag().borrow()[(((this.__get_mag().borrow().len() as i32)).wrapping_sub(n)).wrapping_sub(1i32) as usize];
            let mut _merged1: i32;
            if (this.__get_signum()>=0) {
                _merged1 = magInt;
            } else {
                let _t0 = this.firstNonzeroIntNum()?;
                _merged1 = (if n <= _t0 { (magInt).wrapping_neg() } else { (magInt^-1i32) });
            }
            Ok(_merged1)
        }

        #[java_method(name = "firstNonzeroIntNum", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstNonzeroIntNum(&self) -> Result<i32> {
            let this = self;
            let mut fn_ = (this.__get_firstNonzeroIntNumPlusTwo()).wrapping_sub(2i32);
            let mut mlen = (this.__get_mag().borrow().len() as i32);
            let mut i = (mlen).wrapping_sub(1i32);
            loop {
                if (i<0) { break; }
                if (this.__get_mag().borrow()[i as usize]==0) {
                    i = i.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            fn_ = ((mlen).wrapping_sub(i)).wrapping_sub(1i32);
            this.__set_firstNonzeroIntNumPlusTwo((fn_).wrapping_add(2i32));
            Ok(fn_)
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/math/BigInteger.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readObjectNoData", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException")]
        pub fn readObjectNoData(&self) -> Result<()> {
            panic!("stub: java/math/BigInteger.readObjectNoData:()V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/math/BigInteger.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "magSerializedForm", descriptor = "()[B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn magSerializedForm(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/math/BigInteger.magSerializedForm:()[B")
        }

        #[java_method(name = "longValueExact", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValueExact(&self) -> Result<i64> {
            panic!("stub: java/math/BigInteger.longValueExact:()J")
        }

        #[java_method(name = "intValueExact", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValueExact(&self) -> Result<i32> {
            panic!("stub: java/math/BigInteger.intValueExact:()I")
        }

        #[java_method(name = "shortValueExact", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValueExact(&self) -> Result<i16> {
            panic!("stub: java/math/BigInteger.shortValueExact:()S")
        }

        #[java_method(name = "byteValueExact", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValueExact(&self) -> Result<i8> {
            panic!("stub: java/math/BigInteger.byteValueExact:()B")
        }
    }
}
