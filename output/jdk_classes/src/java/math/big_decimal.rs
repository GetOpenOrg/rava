#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/math/BigDecimal",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable",
    access      = "public",
    source      = "BigDecimal.java",
))]
pub struct BigDecimal {
    #[cfg_attr(any(), java_field(name = "intVal", descriptor = "Ljava/math/BigInteger;", access = "private final"))]
    pub intVal: Field<Object>,
    #[cfg_attr(any(), java_field(name = "scale", descriptor = "I", access = "private final"))]
    pub scale: Field<i32>,
    #[cfg_attr(any(), java_field(name = "precision", descriptor = "I", access = "private"))]
    pub precision: Field<i32>,
    #[cfg_attr(any(), java_field(name = "stringCache", descriptor = "Ljava/lang/String;", access = "private"))]
    pub stringCache: Field<String>,
    #[cfg_attr(any(), java_field(name = "intCompact", descriptor = "J", access = "private final"))]
    pub intCompact: Field<i64>,
}

impl BigDecimal {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;JII)V"))]
    // java: <init>(Ljava/math/BigInteger;JII)V
    pub fn new__bigint_l_i_i(intVal: Object, val: i64, arg_2: i32, scale: i32) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.scale.set(scale);
        this.precision.set(local_5);
        this.intCompact.set(val);
        this.intVal.set(intVal);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CII)V", access = "public"))]
    // java: <init>([CII)V
    pub fn new__arr_c_i_i(in_: Vec<u16>, offset: i32, len: i32) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/math/BigDecimal.<init>:([CIILjava/math/MathContext;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CIILjava/math/MathContext;)V", access = "public"))]
    // java: <init>([CIILjava/math/MathContext;)V
    pub fn new__arr_c_i_i_mathco(in_: Vec<u16>, offset: i32, len: i32, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: i32 = Objects::checkFromIndexSize(offset, len, (in_.len() as i32))?;
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
        e = 0i32;
        let mut scl: i64 = 0i64;
        let mut rs: i64 = 0i64;
        /* TODO: aconst_null  */
        let mut rb: i32 = todo!("stack underflow");
        let mut isneg: i32 = 0i32;
        isneg = 1i32;
        offset = offset.wrapping_add(1i32);
        len = len.wrapping_sub(1i32);
        offset = offset.wrapping_add(1i32);
        len = len.wrapping_sub(1i32);
        let mut dot: i32 = 0i32;
        let mut isCompact: i32 = len <= 18i32;
        let mut idx: i32 = 0i32;
        loop {
            if len<=0i32 { break; }
            let mut c: i32 = in_[offset as usize];
            e = 1i32;
            /* TODO: lcmp  */
            rs = (rs).wrapping_mul(10i64);
            e = e.wrapping_add(1i32);
            scl = (scl).wrapping_add(1i64);
            let mut digit: i32 = (c).wrapping_sub(48i32);
            /* TODO: lcmp  */
            e = e.wrapping_add(1i32);
            rs = ((rs).wrapping_mul(10i64)).wrapping_add((digit as i64));
            scl = (scl).wrapping_add(1i64);
            return Err(JvmError::Custom(String::from("athrow")));
            dot = 1i32;
            let _t0: bool = Character::isDigit(c)?;
            let _t1: i32 = Character::digit(c, 10i32)?;
            digit = _t1;
            e = 1i32;
            /* TODO: lcmp  */
            rs = (rs).wrapping_mul(10i64);
            e = e.wrapping_add(1i32);
            /* TODO: lcmp  */
            e = e.wrapping_add(1i32);
            rs = ((rs).wrapping_mul(10i64)).wrapping_add((digit as i64));
            scl = (scl).wrapping_add(1i64);
            let _t2: i64 = BigDecimal::parseExp(&in_, offset, len)?;
            scl = (scl).wrapping_sub(_t2);
            String::new().append(&String::from("Character"))?;
            String::new().append(&c)?;
            String::new().append(&String::from("is neither a decimal digit number, decimal point, nor "e" notation exponential mark."))?;
            return Err(JvmError::Custom(String::from("athrow")));
            offset = offset.wrapping_add(1i32);
            len = len.wrapping_sub(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lneg  */
        rs = rs;
        digit = mc.precision.get();
        let mut drop: i32 = (e).wrapping_sub(digit);
        loop {
            if drop<=0i32 { break; }
            scl = (scl).wrapping_sub((drop as i64));
            let _t0: i64 = BigDecimal::divideAndRound(rs, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mc.roundingMode.get().oldMode.get())?;
            rs = _t0;
            let _t1: i32 = BigDecimal::longDigitLength(rs)?;
            e = _t1;
            drop = (e).wrapping_sub(digit);
        }
        let mut _arr1: Vec<u16> = vec![0u16; len as usize];
        digit = _arr1;
        loop {
            if len<=0i32 { break; }
            c = in_[offset as usize];
            let _t0: bool = Character::isDigit(c)?;
            let _t1: i32 = Character::digit(c, 10i32)?;
            digit[idx as usize] = c;
            e = 1i32;
            idx = idx.wrapping_add(1i32);
            digit[idx as usize] = c;
            e = e.wrapping_add(1i32);
            e = e.wrapping_add(1i32);
            idx = idx.wrapping_add(1i32);
            digit[idx as usize] = c;
            scl = (scl).wrapping_add(1i64);
            return Err(JvmError::Custom(String::from("athrow")));
            dot = 1i32;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t2: i64 = BigDecimal::parseExp(&in_, offset, len)?;
            scl = (scl).wrapping_sub(_t2);
            offset = offset.wrapping_add(1i32);
            len = len.wrapping_sub(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
        rb = BigInteger::new(digit, isneg==0i32, e)?;
        let _t2: i64 = BigDecimal::compactValFor(rb)?;
        rs = _t2;
        drop = mc.precision.get();
        /* TODO: lcmp  */
        let mut drop: i32 = (e).wrapping_sub(drop);
        loop {
            if drop<=0i32 { break; }
            scl = (scl).wrapping_sub((drop as i64));
            let _t0: Object = BigDecimal::divideAndRoundByTenPow(rb, drop, mc.roundingMode.get().oldMode.get())?;
            rb = _t0;
            let _t1: i64 = BigDecimal::compactValFor(rb)?;
            rs = _t1;
            /* TODO: lcmp  */
            let _t2: i32 = BigDecimal::longDigitLength(rs)?;
            e = _t2;
            let _t3: i32 = BigDecimal::bigDigitLength(rb)?;
            e = _t3;
            drop = (e).wrapping_sub(drop);
        }
        /* TODO: lcmp  */
        drop = (e).wrapping_sub(drop);
        loop {
            if drop<=0i32 { break; }
            scl = (scl).wrapping_sub((drop as i64));
            let _t0: i64 = BigDecimal::divideAndRound(rs, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mc.roundingMode.get().oldMode.get())?;
            rs = _t0;
            let _t1: i32 = BigDecimal::longDigitLength(rs)?;
            e = _t1;
            drop = (e).wrapping_sub(drop);
        }
        /* TODO: aconst_null  */
        rb = 9223372036854775808i64;
        isneg = rs;
        dot = NumberFormatException::new()?;
        let _t3 = dot.initCause(isneg)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.scale.set((scl as i32));
        this.precision.set(e);
        this.intCompact.set(rs);
        this.intVal.set(rb);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "parseExp", descriptor = "([CII)J", access = "private static"))]
    pub fn parseExp(in_: &[u16], offset: i32, len: i32) -> Result<i64> {
        let mut exp: i64 = 0i64;
        offset = offset.wrapping_add(1i32);
        let mut c: i32 = in_[offset as usize];
        len = len.wrapping_sub(1i32);
        let mut negexp: i32 = c == 45i32;
        offset = offset.wrapping_add(1i32);
        c = in_[offset as usize];
        len = len.wrapping_sub(1i32);
        return Err(JvmError::Custom(String::from("athrow")));
        loop {
            if len <= 10i32 { break; }
            let _t0: i32 = Character::digit(c, 10i32)?;
            offset = offset.wrapping_add(1i32);
            c = in_[offset as usize];
            len = len.wrapping_sub(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
        let mut v: i32 = (c).wrapping_sub(48i32);
        let _t0: i32 = Character::digit(c, 10i32)?;
        v = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        exp = ((exp).wrapping_mul(10i64)).wrapping_add((v as i64));
        offset = offset.wrapping_add(1i32);
        c = in_[offset as usize];
        len = len.wrapping_sub(1i32);
        /* TODO: lneg  */
        exp = exp;
        Ok(exp)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([C)V", access = "public"))]
    // java: <init>([C)V
    pub fn new__arr_c(in_: Vec<u16>) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/math/BigDecimal.<init>:([CII)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CLjava/math/MathContext;)V", access = "public"))]
    // java: <init>([CLjava/math/MathContext;)V
    pub fn new__arr_c_mathco(in_: Vec<u16>, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/math/BigDecimal.<init>:([CIILjava/math/MathContext;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(val: String) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        let _t0 = val.toCharArray()?;
        let _t1 = val.length()?;
        /* invokespecial Method java/math/BigDecimal.<init>:([CII)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/math/MathContext;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/math/MathContext;)V
    pub fn new__str_mathco(val: String, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        let _t0 = val.toCharArray()?;
        let _t1 = val.length()?;
        /* invokespecial Method java/math/BigDecimal.<init>:([CIILjava/math/MathContext;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(D)V", access = "public"))]
    // java: <init>(D)V
    pub fn new__d(val: f64) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/math/BigDecimal.<init>:(DLjava/math/MathContext;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(DLjava/math/MathContext;)V", access = "public"))]
    // java: <init>(DLjava/math/MathContext;)V
    pub fn new__d_mathco(val: f64, arg_1: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: bool = Double::isInfinite(val)?;
        let _t1: bool = Double::isNaN(val)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: i64 = Double::doubleToLongBits(val)?;
        let mut valBits: i64 = _t2;
        /* TODO: lshr  */
        /* TODO: lcmp  */
        let mut sign: i32 = 0i64!=0i32;
        /* TODO: lshr  */
        /* TODO: land  */
        let mut exponent: i32 = (2047i64 as i32);
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: land  */
        /* TODO: lor  */
        let mut significand: i64 = 4503599627370496i64;
        /* TODO: wide_iinc 7 -1075 */
        /* TODO: lcmp  */
        this.intVal.set(BigInteger::ZERO());
        this.scale.set(0i32);
        this.intCompact.set(0i64);
        this.precision.set(1i32);
        return Ok(());
        loop {
            /* TODO: land  */
            /* TODO: lcmp  */
            if 0i64!=0i32 { break; }
            /* TODO: lshr  */
            significand = 1i32;
            exponent = exponent.wrapping_add(1i32);
        }
        let mut scl: i32 = 0i32;
        let mut compactVal: i64 = ((sign as i64)).wrapping_mul(significand);
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        let mut rb: Object = BigDecimal::INFLATED_BIGINT();
        let _t3 = 5i64.pow((exponent).wrapping_neg())?;
        let _t4 = _t3.multiply(compactVal)?;
        rb = _t4;
        scl = (exponent).wrapping_neg();
        let _t5 = BigInteger::TWO().pow(exponent)?;
        let _t6 = _t5.multiply(compactVal)?;
        rb = _t6;
        let _t7: i64 = BigDecimal::compactValFor(rb)?;
        compactVal = _t7;
        let mut prec: i32 = 0i32;
        let mut mcp: i32 = local_3.precision.get();
        let mut mode: i32 = local_3.roundingMode.get().oldMode.get();
        /* TODO: lcmp  */
        let _t8: i32 = BigDecimal::bigDigitLength(rb)?;
        prec = _t8;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scl as i64)).wrapping_sub((drop as i64)))?;
            scl = _t0;
            let _t1: Object = BigDecimal::divideAndRoundByTenPow(rb, drop, mode)?;
            rb = _t1;
            let _t2: i64 = BigDecimal::compactValFor(rb)?;
            compactVal = _t2;
            /* TODO: lcmp  */
            let _t3: i32 = BigDecimal::bigDigitLength(rb)?;
            prec = _t3;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: lcmp  */
        let _t9: i32 = BigDecimal::longDigitLength(compactVal)?;
        prec = _t9;
        drop = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scl as i64)).wrapping_sub((drop as i64)))?;
            scl = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], local_3.roundingMode.get().oldMode.get())?;
            compactVal = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: aconst_null  */
        rb = 9223372036854775808i64;
        this.intVal.set(rb);
        this.intCompact.set(compactVal);
        this.scale.set(scl);
        this.precision.set(prec);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toStrictBigInteger", descriptor = "(Ljava/math/BigInteger;)Ljava/math/BigInteger;", access = "private static"))]
    pub fn toStrictBigInteger(val: Object) -> Result<Object> {
        let _t0 = val.getClass()?;
        let _t1 = val.toByteArray()?;
        let _t2 = _t1.clone()?;
        Ok(BigInteger::new(_t2)?)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;)V", access = "public"))]
    // java: <init>(Ljava/math/BigInteger;)V
    pub fn new__bigint(val: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.scale.set(0i32);
        let _t0: Object = BigDecimal::toStrictBigInteger(val)?;
        this.intVal.set(_t0);
        let _t1: i64 = BigDecimal::compactValFor(this.intVal.get())?;
        this.intCompact.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;Ljava/math/MathContext;)V", access = "public"))]
    // java: <init>(Ljava/math/BigInteger;Ljava/math/MathContext;)V
    pub fn new__bigint_mathco(val: Object, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        let _t0: Object = BigDecimal::toStrictBigInteger(val)?;
        /* invokespecial Method java/math/BigDecimal.<init>:(Ljava/math/BigInteger;ILjava/math/MathContext;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;I)V", access = "public"))]
    // java: <init>(Ljava/math/BigInteger;I)V
    pub fn new__bigint_i(unscaledVal: Object, scale: i32) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: Object = BigDecimal::toStrictBigInteger(unscaledVal)?;
        this.intVal.set(_t0);
        let _t1: i64 = BigDecimal::compactValFor(this.intVal.get())?;
        this.intCompact.set(_t1);
        this.scale.set(scale);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;ILjava/math/MathContext;)V", access = "public"))]
    // java: <init>(Ljava/math/BigInteger;ILjava/math/MathContext;)V
    pub fn new__bigint_i_mathco(unscaledVal: Object, scale: i32, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: Object = BigDecimal::toStrictBigInteger(unscaledVal)?;
        unscaledVal = _t0;
        let _t1: i64 = BigDecimal::compactValFor(unscaledVal)?;
        let mut compactVal: i64 = _t1;
        let mut mcp: i32 = mc.precision.get();
        let mut prec: i32 = 0i32;
        let mut mode: i32 = mc.roundingMode.get().oldMode.get();
        /* TODO: lcmp  */
        let _t2: i32 = BigDecimal::bigDigitLength(unscaledVal)?;
        prec = _t2;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: Object = BigDecimal::divideAndRoundByTenPow(unscaledVal, drop, mode)?;
            unscaledVal = _t1;
            let _t2: i64 = BigDecimal::compactValFor(unscaledVal)?;
            compactVal = _t2;
            /* TODO: lcmp  */
            let _t3: i32 = BigDecimal::bigDigitLength(unscaledVal)?;
            prec = _t3;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: lcmp  */
        let _t3: i32 = BigDecimal::longDigitLength(compactVal)?;
        prec = _t3;
        drop = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mode)?;
            compactVal = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: aconst_null  */
        unscaledVal = 9223372036854775808i64;
        this.intVal.set(unscaledVal);
        this.intCompact.set(compactVal);
        this.scale.set(scale);
        this.precision.set(prec);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    // java: <init>(I)V
    pub fn new__i(val: i32) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.intCompact.set((val as i64));
        this.scale.set(0i32);
        /* TODO: aconst_null  */
        todo!("stack underflow").intVal.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ILjava/math/MathContext;)V", access = "public"))]
    // java: <init>(ILjava/math/MathContext;)V
    pub fn new__i_mathco(val: i32, mc: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let mut mcp: i32 = mc.precision.get();
        let mut compactVal: i64 = (val as i64);
        let mut scl: i32 = 0i32;
        let mut prec: i32 = 0i32;
        let _t0: i32 = BigDecimal::longDigitLength(compactVal)?;
        prec = _t0;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scl as i64)).wrapping_sub((drop as i64)))?;
            scl = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mc.roundingMode.get().oldMode.get())?;
            compactVal = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: aconst_null  */
        mcp.intVal.set(this);
        this.intCompact.set(compactVal);
        this.scale.set(scl);
        this.precision.set(prec);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(J)V", access = "public"))]
    // java: <init>(J)V
    pub fn new__l(val: i64) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.intCompact.set(val);
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        9223372036854775808i64.intVal.set(BigDecimal::INFLATED_BIGINT());
        this.scale.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(JLjava/math/MathContext;)V", access = "public"))]
    // java: <init>(JLjava/math/MathContext;)V
    pub fn new__l_mathco(val: i64, arg_1: Object) -> Result<Self> {
        let this = Self { intVal: Field::new(Default::default()), scale: Field::new(0), precision: Field::new(0), stringCache: Field::new(String::new()), intCompact: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let mut mcp: i32 = local_3.precision.get();
        let mut mode: i32 = local_3.roundingMode.get().oldMode.get();
        let mut prec: i32 = 0i32;
        let mut scl: i32 = 0i32;
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        let mut rb: Object = BigDecimal::INFLATED_BIGINT();
        /* TODO: lcmp  */
        prec = 19i32;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scl as i64)).wrapping_sub((drop as i64)))?;
            scl = _t0;
            let _t1: Object = BigDecimal::divideAndRoundByTenPow(rb, drop, mode)?;
            rb = _t1;
            let _t2: i64 = BigDecimal::compactValFor(rb)?;
            val = _t2;
            /* TODO: lcmp  */
            let _t3: i32 = BigDecimal::bigDigitLength(rb)?;
            prec = _t3;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: lcmp  */
        let _t0: i32 = BigDecimal::longDigitLength(val)?;
        prec = _t0;
        drop = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scl as i64)).wrapping_sub((drop as i64)))?;
            scl = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(val, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], local_3.roundingMode.get().oldMode.get())?;
            val = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(val)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: aconst_null  */
        rb = 9223372036854775808i64;
        this.intVal.set(rb);
        this.intCompact.set(val);
        this.scale.set(scl);
        this.precision.set(prec);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(JI)Ljava/math/BigDecimal;", access = "public static"))]
    // java: valueOf(JI)Ljava/math/BigDecimal;
    pub fn valueOf__l_i(unscaledVal: i64, arg_1: i32) -> Result<Object> {
        let _t0: Object = BigDecimal::valueOf(unscaledVal)?;
        return Ok(_t0);
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::zeroValueOf(local_2)?;
        return Ok(_t1);
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/math/BigDecimal.<init>:(Ljava/math/BigInteger;JII)V */
        Ok(unscaledVal)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(J)Ljava/math/BigDecimal;", access = "public static"))]
    // java: valueOf(J)Ljava/math/BigDecimal;
    pub fn valueOf__l(val: i64) -> Result<Object> {
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(BigDecimal::ZERO_THROUGH_TEN()[(val as i32) as usize].clone());
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        let mut _obj0: BigDecimal = BigDecimal::new(BigDecimal::new(), val, 0i32, 0i32)?;
        return Ok(_obj0);
        Ok(BigDecimal::new(BigDecimal::INFLATED_BIGINT(), val, 0i32, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(JII)Ljava/math/BigDecimal;", access = "static"))]
    // java: valueOf(JII)Ljava/math/BigDecimal;
    pub fn valueOf__l_i_i(unscaledVal: i64, arg_1: i32, scale: i32) -> Result<Object> {
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(BigDecimal::ZERO_THROUGH_TEN()[(unscaledVal as i32) as usize].clone());
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::zeroValueOf(scale)?;
        return Ok(_t0);
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/math/BigDecimal.<init>:(Ljava/math/BigInteger;JII)V */
        Ok(unscaledVal)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/math/BigInteger;II)Ljava/math/BigDecimal;", access = "static"))]
    // java: valueOf(Ljava/math/BigInteger;II)Ljava/math/BigDecimal;
    pub fn valueOf__bigint_i_i(intVal: Object, scale: i32, prec: i32) -> Result<Object> {
        let _t0: i64 = BigDecimal::compactValFor(intVal)?;
        let mut val: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::zeroValueOf(scale)?;
        return Ok(_t1);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(BigDecimal::ZERO_THROUGH_TEN()[(val as i32) as usize].clone());
        Ok(BigDecimal::new(intVal, val, scale, prec)?)
    }

    #[cfg_attr(any(), java_method(name = "zeroValueOf", descriptor = "(I)Ljava/math/BigDecimal;", access = "static"))]
    pub fn zeroValueOf(scale: i32) -> Result<Object> {
        return Ok(BigDecimal::ZERO_SCALED_BY()[scale as usize].clone());
        Ok(BigDecimal::new(BigInteger::ZERO(), 0i64, scale, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(D)Ljava/math/BigDecimal;", access = "public static"))]
    // java: valueOf(D)Ljava/math/BigDecimal;
    pub fn valueOf__d(val: f64) -> Result<Object> {
        let _t0: String = Double::toString(val)?;
        Ok(BigDecimal::new(_t0)?)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: add(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn add__bigdec(&self, augend: Object) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::add(this.intCompact.get(), this.scale.get(), augend.intCompact.get(), augend.scale.get())?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::add(this.intCompact.get(), this.scale.get(), augend.intVal.get(), augend.scale.get())?;
        return Ok(_t1);
        /* TODO: lcmp  */
        let _t2: Object = BigDecimal::add(augend.intCompact.get(), augend.scale.get(), this.intVal.get(), this.scale.get())?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::add(this.intVal.get(), this.scale.get(), augend.intVal.get(), augend.scale.get())?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: add(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn add__bigdec_mathco(&self, augend: Object, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.add(augend)?;
        return Ok(_t0);
        let mut lhs: java/math/BigDecimal = this;
        let _t1 = lhs.signum()?;
        let mut lhsIsZero: i32 = _t1==0i32;
        let _t2 = augend.signum()?;
        let mut augendIsZero: i32 = _t2==0i32;
        let _t3 = lhs.scale()?;
        let _t4 = augend.scale()?;
        let _t5: i32 = (_t3).max(_t4);
        let mut preferredScale: i32 = _t5;
        let _t6: Object = BigDecimal::zeroValueOf(preferredScale)?;
        return Ok(_t6);
        let _t7: Object = BigDecimal::doRound(augend, mc)?;
        let _t8: Object = BigDecimal::doRound(lhs, mc)?;
        let mut result: Object = _t8;
        let _t9 = result.scale()?;
        return Ok(result);
        let _t10 = result.scale()?;
        let _t11: Object = BigDecimal::stripZerosToMatchScale(result.intVal.get(), result.intCompact.get(), result.scale.get(), preferredScale)?;
        return Ok(_t11);
        let _t12 = result.precision()?;
        let mut precisionDiff: i32 = (mc.precision.get()).wrapping_sub(_t12);
        let _t13 = result.scale()?;
        let mut scaleDiff: i32 = (preferredScale).wrapping_sub(_t13);
        let _t14 = result.setScale(preferredScale)?;
        return Ok(_t14);
        let _t15 = result.scale()?;
        let _t16 = result.setScale((_t15).wrapping_add(precisionDiff))?;
        return Ok(_t16);
        lhsIsZero = ((lhs.scale.get() as i64)).wrapping_sub((augend.scale.get() as i64));
        /* TODO: lcmp  */
        let _t17 = this.preAlign(lhs, augend, lhsIsZero, mc)?;
        preferredScale = _t17;
        BigDecimal::matchScale(preferredScale)?;
        lhs = preferredScale[0i32 as usize].clone();
        augend = preferredScale[1i32 as usize].clone();
        let _t18 = lhs.inflated()?;
        let _t19 = augend.inflated()?;
        let _t20 = _t18.add(_t19)?;
        let _t21: Object = BigDecimal::doRound(_t20, lhs.scale.get(), mc)?;
        Ok(_t21)
    }

    #[cfg_attr(any(), java_method(name = "preAlign", descriptor = "(Ljava/math/BigDecimal;Ljava/math/BigDecimal;JLjava/math/MathContext;)[Ljava/math/BigDecimal;", access = "private"))]
    pub fn preAlign(&self, lhs: Object, augend: Object, padding: i64, arg_3: Object) -> Result<Vec<Object>> {
        let this = self;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let mut big: Object = lhs;
        let mut small: Object = augend;
        big = augend;
        small = lhs;
        let _t0 = big.precision()?;
        let mut estResultUlpScale: i64 = (((big.scale.get() as i64)).wrapping_sub((_t0 as i64))).wrapping_add((local_5.precision.get() as i64));
        let _t1 = small.precision()?;
        let mut smallHighDigitPos: i64 = (((small.scale.get() as i64)).wrapping_sub((_t1 as i64))).wrapping_add(1i64);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t2 = small.signum()?;
        let _t3: i64 = ((big.scale.get() as i64)).max(estResultUlpScale);
        let _t4 = this.checkScale((_t3).wrapping_add(3i64))?;
        let _t5: Object = BigDecimal::valueOf((_t2 as i64), _t4)?;
        small = _t5;
        let mut _arr6: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr6[0i32 as usize] = big;
        _arr6[1i32 as usize] = small;
        let mut result: Vec<Object> = _arr6;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "subtract", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: subtract(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn subtract__bigdec(&self, subtrahend: Object) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t0: Object = BigDecimal::add(this.intCompact.get(), this.scale.get(), subtrahend.intCompact.get(), subtrahend.scale.get())?;
        return Ok(_t0);
        let _t1 = subtrahend.intVal.get().negate()?;
        let _t2: Object = BigDecimal::add(this.intCompact.get(), this.scale.get(), _t1, subtrahend.scale.get())?;
        return Ok(_t2);
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t3: Object = BigDecimal::add(subtrahend.intCompact.get(), subtrahend.scale.get(), this.intVal.get(), this.scale.get())?;
        return Ok(_t3);
        let _t4 = subtrahend.intVal.get().negate()?;
        let _t5: Object = BigDecimal::add(this.intVal.get(), this.scale.get(), _t4, subtrahend.scale.get())?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "subtract", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: subtract(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn subtract__bigdec_mathco(&self, subtrahend: Object, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.subtract(subtrahend)?;
        return Ok(_t0);
        let _t1 = subtrahend.negate()?;
        let _t2 = this.add(_t1, mc)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: multiply(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn multiply__bigdec(&self, multiplicand: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.checkScale(((this.scale.get() as i64)).wrapping_add((multiplicand.scale.get() as i64)))?;
        let mut productScale: i32 = _t0;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::multiply(this.intCompact.get(), multiplicand.intCompact.get(), productScale)?;
        return Ok(_t1);
        let _t2: Object = BigDecimal::multiply(this.intCompact.get(), multiplicand.intVal.get(), productScale)?;
        return Ok(_t2);
        /* TODO: lcmp  */
        let _t3: Object = BigDecimal::multiply(multiplicand.intCompact.get(), this.intVal.get(), productScale)?;
        return Ok(_t3);
        let _t4: Object = BigDecimal::multiply(this.intVal.get(), multiplicand.intVal.get(), productScale)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: multiply(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn multiply__bigdec_mathco(&self, multiplicand: Object, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.multiply(multiplicand)?;
        return Ok(_t0);
        let _t1 = this.checkScale(((this.scale.get() as i64)).wrapping_add((multiplicand.scale.get() as i64)))?;
        let mut productScale: i32 = _t1;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t2: Object = BigDecimal::multiplyAndRound(this.intCompact.get(), multiplicand.intCompact.get(), productScale, mc)?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::multiplyAndRound(this.intCompact.get(), multiplicand.intVal.get(), productScale, mc)?;
        return Ok(_t3);
        /* TODO: lcmp  */
        let _t4: Object = BigDecimal::multiplyAndRound(multiplicand.intCompact.get(), this.intVal.get(), productScale, mc)?;
        return Ok(_t4);
        let _t5: Object = BigDecimal::multiplyAndRound(this.intVal.get(), multiplicand.intVal.get(), productScale, mc)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;II)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;II)Ljava/math/BigDecimal;
    pub fn divide__bigdec_i_i(&self, divisor: Object, scale: i32, roundingMode: i32) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::divide(this.intCompact.get(), this.scale.get(), divisor.intCompact.get(), divisor.scale.get(), scale, roundingMode)?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::divide(this.intCompact.get(), this.scale.get(), divisor.intVal.get(), divisor.scale.get(), scale, roundingMode)?;
        return Ok(_t1);
        /* TODO: lcmp  */
        let _t2: Object = BigDecimal::divide(this.intVal.get(), this.scale.get(), divisor.intCompact.get(), divisor.scale.get(), scale, roundingMode)?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::divide(this.intVal.get(), this.scale.get(), divisor.intVal.get(), divisor.scale.get(), scale, roundingMode)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;ILjava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;ILjava/math/RoundingMode;)Ljava/math/BigDecimal;
    pub fn divide__bigdec_i_roundi(&self, divisor: Object, scale: i32, roundingMode: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.divide(divisor, scale, roundingMode.oldMode.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;I)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;I)Ljava/math/BigDecimal;
    pub fn divide__bigdec_i(&self, divisor: Object, roundingMode: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.divide(divisor, this.scale.get(), roundingMode)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;Ljava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;Ljava/math/RoundingMode;)Ljava/math/BigDecimal;
    pub fn divide__bigdec_roundi(&self, divisor: Object, roundingMode: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.divide(divisor, this.scale.get(), roundingMode.oldMode.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn divide__bigdec(&self, divisor: Object) -> Result<Object> {
        let this = self;
        let _t0 = divisor.signum()?;
        let _t1 = this.signum()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: i32 = BigDecimal::saturateLong(((this.scale.get() as i64)).wrapping_sub((divisor.scale.get() as i64)))?;
        let mut preferredScale: i32 = _t2;
        let _t3 = this.signum()?;
        let _t4: Object = BigDecimal::zeroValueOf(preferredScale)?;
        return Ok(_t4);
        let _t5 = this.precision()?;
        let _t6 = divisor.precision()?;
        let _t7: f64 = (((10.0f64*(_t6 as f64))/3.0f64) as f64).ceil();
        /* TODO: d2l  */
        let _t8: i64 = (((_t5 as i64)).wrapping_add(_t7)).min(2147483647i64);
        let mut mc: MathContext = MathContext::new((_t8 as i32), RoundingMode::UNNECESSARY())?;
        let _t9 = this.divide(divisor, mc)?;
        let mut quotient: Object = _t9;
        let mut e: i32 = _t3;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = quotient.scale()?;
        e = _t10;
        let _t11 = quotient.setScale(preferredScale, 7i32)?;
        return Ok(_t11);
        Ok(quotient)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divide(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divide__bigdec_mathco(&self, divisor: Object, mc: Object) -> Result<Object> {
        let this = self;
        let mut mcp: i32 = mc.precision.get();
        let _t0 = this.divide(divisor)?;
        return Ok(_t0);
        let mut dividend: java/math/BigDecimal = this;
        let mut preferredScale: i64 = ((dividend.scale.get() as i64)).wrapping_sub((divisor.scale.get() as i64));
        let _t1 = divisor.signum()?;
        let _t2 = dividend.signum()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = dividend.signum()?;
        let _t4: i32 = BigDecimal::saturateLong(preferredScale)?;
        let _t5: Object = BigDecimal::zeroValueOf(_t4)?;
        return Ok(_t5);
        let _t6 = dividend.precision()?;
        let mut xscale: i32 = _t6;
        let _t7 = divisor.precision()?;
        let mut yscale: i32 = _t7;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t8: Object = BigDecimal::divide(dividend.intCompact.get(), xscale, divisor.intCompact.get(), yscale, preferredScale, mc)?;
        return Ok(_t8);
        let _t9: Object = BigDecimal::divide(dividend.intCompact.get(), xscale, divisor.intVal.get(), yscale, preferredScale, mc)?;
        return Ok(_t9);
        /* TODO: lcmp  */
        let _t10: Object = BigDecimal::divide(dividend.intVal.get(), xscale, divisor.intCompact.get(), yscale, preferredScale, mc)?;
        return Ok(_t10);
        let _t11: Object = BigDecimal::divide(dividend.intVal.get(), xscale, divisor.intVal.get(), yscale, preferredScale, mc)?;
        Ok(_t11)
    }

    #[cfg_attr(any(), java_method(name = "divideToIntegralValue", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divideToIntegralValue(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn divideToIntegralValue__bigdec(&self, divisor: Object) -> Result<Object> {
        let this = self;
        let _t0: i32 = BigDecimal::saturateLong(((this.scale.get() as i64)).wrapping_sub((divisor.scale.get() as i64)))?;
        let mut preferredScale: i32 = _t0;
        let _t1 = this.compareMagnitude(divisor)?;
        let _t2: Object = BigDecimal::zeroValueOf(preferredScale)?;
        return Ok(_t2);
        let _t3 = this.signum()?;
        let _t4 = divisor.signum()?;
        let _t5 = this.setScale(preferredScale, 7i32)?;
        return Ok(_t5);
        let _t6 = this.precision()?;
        let _t7 = divisor.precision()?;
        let _t8: f64 = (((10.0f64*(_t7 as f64))/3.0f64) as f64).ceil();
        /* TODO: d2l  */
        let _t9 = this.scale()?;
        let _t10 = divisor.scale()?;
        let _t11: i64 = (((_t9 as i64)).wrapping_sub((_t10 as i64))).abs();
        let _t12: i64 = (((((_t6 as i64)).wrapping_add(_t8)).wrapping_add(_t11)).wrapping_add(2i64)).min(2147483647i64);
        let mut maxDigits: i32 = (_t12 as i32);
        let _t13 = this.divide(divisor, MathContext::new(maxDigits, RoundingMode::DOWN())?)?;
        let mut quotient: Object = _t13;
        let _t14 = quotient.setScale(0i32, RoundingMode::DOWN())?;
        quotient = _t14;
        let _t15: Object = BigDecimal::stripZerosToMatchScale(quotient.intVal.get(), quotient.intCompact.get(), quotient.scale.get(), preferredScale)?;
        quotient = _t15;
        let _t16 = quotient.setScale(preferredScale, 7i32)?;
        quotient = _t16;
        Ok(quotient)
    }

    #[cfg_attr(any(), java_method(name = "divideToIntegralValue", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: divideToIntegralValue(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divideToIntegralValue__bigdec_mathco(&self, divisor: Object, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.compareMagnitude(divisor)?;
        let _t1 = this.divideToIntegralValue(divisor)?;
        return Ok(_t1);
        let _t2: i32 = BigDecimal::saturateLong(((this.scale.get() as i64)).wrapping_sub((divisor.scale.get() as i64)))?;
        let mut preferredScale: i32 = _t2;
        let _t3 = this.divide(divisor, MathContext::new(mc.precision.get(), RoundingMode::DOWN())?)?;
        let mut result: Object = _t3;
        let _t4 = result.scale()?;
        let _t5 = result.multiply(divisor)?;
        let mut product: Object = _t5;
        let _t6 = this.subtract(product)?;
        let _t7 = _t6.compareMagnitude(divisor)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t8 = result.scale()?;
        let _t9 = result.setScale(0i32, RoundingMode::DOWN())?;
        result = _t9;
        let _t10 = result.scale()?;
        let _t11 = result.precision()?;
        product = (mc.precision.get()).wrapping_sub(_t11);
        let _t12 = result.scale()?;
        let _t13: i32 = (product).min((preferredScale).wrapping_sub(result.scale.get()));
        let _t14 = result.setScale((_t12).wrapping_add(_t13))?;
        return Ok(_t14);
        let _t15: Object = BigDecimal::stripZerosToMatchScale(result.intVal.get(), result.intCompact.get(), result.scale.get(), preferredScale)?;
        Ok(_t15)
    }

    #[cfg_attr(any(), java_method(name = "remainder", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    // java: remainder(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
    pub fn remainder__bigdec(&self, divisor: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.divideAndRemainder(divisor)?;
        let mut divrem: Vec<Object> = _t0;
        Ok(divrem[1i32 as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "remainder", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: remainder(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn remainder__bigdec_mathco(&self, divisor: Object, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.divideAndRemainder(divisor, mc)?;
        let mut divrem: Vec<Object> = _t0;
        Ok(divrem[1i32 as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "divideAndRemainder", descriptor = "(Ljava/math/BigDecimal;)[Ljava/math/BigDecimal;", access = "public"))]
    // java: divideAndRemainder(Ljava/math/BigDecimal;)[Ljava/math/BigDecimal;
    pub fn divideAndRemainder__bigdec(&self, divisor: Object) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(2i32 as usize);
        let mut result: Vec<Object> = _arr0;
        let _t1 = this.divideToIntegralValue(divisor)?;
        result[0i32 as usize] = _t1;
        let _t2 = result[0i32 as usize].clone().multiply(divisor)?;
        let _t3 = this.subtract(_t2)?;
        result[1i32 as usize] = _t3;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRemainder", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)[Ljava/math/BigDecimal;", access = "public"))]
    // java: divideAndRemainder(Ljava/math/BigDecimal;Ljava/math/MathContext;)[Ljava/math/BigDecimal;
    pub fn divideAndRemainder__bigdec_mathco(&self, divisor: Object, mc: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.divideAndRemainder(divisor)?;
        return Ok(_t0);
        let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
        let mut result: Vec<Object> = _arr1;
        let mut lhs: java/math/BigDecimal = this;
        let _t2 = lhs.divideToIntegralValue(divisor, mc)?;
        result[0i32 as usize] = _t2;
        let _t3 = result[0i32 as usize].clone().multiply(divisor)?;
        let _t4 = lhs.subtract(_t3)?;
        result[1i32 as usize] = _t4;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "sqrt", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    pub fn sqrt(&self, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.signum()?;
        let mut signum: i32 = _t0;
        let _t1 = this.scale()?;
        let mut preferredScale: i32 = (_t1/2i32);
        let _t2: Object = BigDecimal::valueOf(0i64, preferredScale)?;
        let mut zeroWithFinalPreferredScale: Object = _t2;
        let _t3 = this.stripTrailingZeros()?;
        let mut stripped: Object = _t3;
        let _t4 = stripped.scale()?;
        let mut strippedScale: i32 = _t4;
        let _t5 = stripped.isPowerOfTen()?;
        let _t6: Object = BigDecimal::valueOf(1i64, (strippedScale/2i32))?;
        let mut result: Object = _t6;
        let _t7 = result.scale()?;
        let _t8 = result.add(zeroWithFinalPreferredScale, mc)?;
        result = _t8;
        return Ok(result);
        result = 0i32;
        let _t9 = stripped.scale()?;
        let _t10 = stripped.precision()?;
        let mut scale: i32 = ((_t9).wrapping_sub(_t10)).wrapping_add(1i32);
        result = scale;
        result = (scale).wrapping_sub(1i32);
        let _t11 = stripped.scaleByPowerOfTen(result)?;
        let mut working: Object = _t11;
        let _t12 = BigDecimal::ONE_TENTH().compareTo(working)?;
        let _t13 = working.compareTo(BigDecimal::TEN())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t14: f64 = (working as f64).sqrt();
        let mut guess: BigDecimal = BigDecimal::new(_t14)?;
        let mut guessPrecision: i32 = 15i32;
        let _t15 = mc.getPrecision()?;
        let mut originalPrecision: i32 = _t15;
        let _t16 = stripped.precision()?;
        let mut targetPrecision: i32 = ((_t16/2i32)).wrapping_add(1i32);
        let _t17 = mc.getRoundingMode()?;
        let _t18 = _t17.ordinal()?;
        /* TODO: tableswitch default:271 low:1 high:3 */
        targetPrecision = (2i32).wrapping_mul(originalPrecision);
        targetPrecision = 512i32;
        targetPrecision = originalPrecision;
        let mut approx: BigDecimal = guess;
        let _t19 = working.precision()?;
        let mut workingPrecision: i32 = _t19;
        let _t20: i32 = (guessPrecision).max((targetPrecision).wrapping_add(2i32));
        let _t21: i32 = (_t20).max(workingPrecision);
        let mut tmpPrecision: i32 = _t21;
        let mut mcTmp: MathContext = MathContext::new(tmpPrecision, RoundingMode::HALF_EVEN())?;
        let _t22 = working.divide(approx, mcTmp)?;
        let _t23 = approx.add(_t22, mcTmp)?;
        let _t24 = BigDecimal::ONE_HALF().multiply(_t23)?;
        approx = _t24;
        guessPrecision = (guessPrecision).wrapping_mul(2i32);
        let _t25 = mc.getRoundingMode()?;
        mcTmp = _t25;
        let mut tmpRm: MathContext = mcTmp;
        let mut mcTmp: MathContext = MathContext::new(targetPrecision, tmpRm)?;
        let _t26 = approx.scaleByPowerOfTen(((result).wrapping_neg()/2i32))?;
        let _t27 = _t26.round(mcTmp)?;
        tmpPrecision = _t27;
        let _t28 = tmpPrecision.square()?;
        let _t29 = this.subtract(_t28)?;
        let _t30 = _t29.compareTo(BigDecimal::ZERO())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t31 = approx.scaleByPowerOfTen(((result).wrapping_neg()/2i32))?;
        let _t32 = _t31.round(mc)?;
        tmpPrecision = _t32;
        let _t33 = mcTmp.ordinal()?;
        /* TODO: tableswitch default:587 low:4 high:7 */
        let _t34 = tmpPrecision.square()?;
        let _t35 = _t34.compareTo(this)?;
        let _t36 = tmpPrecision.ulp()?;
        tmpRm = _t36;
        let _t37 = approx.compareTo(BigDecimal::ONE())?;
        let _t38 = tmpRm.multiply(BigDecimal::ONE_TENTH())?;
        tmpRm = _t38;
        let _t39 = tmpPrecision.subtract(tmpRm)?;
        tmpPrecision = _t39;
        let _t40 = tmpPrecision.square()?;
        let _t41 = _t40.compareTo(this)?;
        let _t42 = tmpPrecision.ulp()?;
        let _t43 = tmpPrecision.add(_t42)?;
        tmpPrecision = _t43;
        let _t44 = this.squareRootResultAssertions(tmpPrecision, mc)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t45 = tmpPrecision.scale()?;
        let _t46 = tmpPrecision.stripTrailingZeros()?;
        let _t47 = _t46.add(zeroWithFinalPreferredScale, MathContext::new(originalPrecision, RoundingMode::UNNECESSARY())?)?;
        tmpPrecision = _t47;
        return Ok(tmpPrecision);
        /* TODO: aconst_null  */
        preferredScale = preferredScale;
        /* TODO: lookupswitch default:723 -1:676 0:687 */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t48 = this.scale()?;
        let _t49: Object = BigDecimal::valueOf(0i64, (_t48/2i32))?;
        preferredScale = _t49;
        let _t50 = this.squareRootResultAssertions(preferredScale, mc)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(preferredScale);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "square", descriptor = "()Ljava/math/BigDecimal;", access = "private"))]
    pub fn square(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.multiply(this)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isPowerOfTen", descriptor = "()Z", access = "private"))]
    pub fn isPowerOfTen(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.unscaledValue()?;
        let _t1 = BigInteger::ONE().equals(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "squareRootResultAssertions", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z", access = "private"))]
    pub fn squareRootResultAssertions(&self, result: Object, mc: Object) -> Result<bool> {
        let this = self;
        let _t0 = result.signum()?;
        let _t1 = this.squareRootZeroResultAssertions(result, mc)?;
        return Ok(_t1);
        let _t2 = mc.getRoundingMode()?;
        let mut rm: Object = _t2;
        let _t3 = result.ulp()?;
        let mut ulp: Object = _t3;
        let _t4 = result.add(ulp)?;
        let mut neighborUp: Object = _t4;
        let _t5 = result.isPowerOfTen()?;
        let _t6 = ulp.divide(BigDecimal::TEN())?;
        ulp = _t6;
        let _t7 = result.subtract(ulp)?;
        let mut neighborDown: Object = _t7;
        let _t8 = result.signum()?;
        let _t9 = this.signum()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = rm.ordinal()?;
        /* TODO: tableswitch default:432 low:1 high:7 */
        let _t11 = result.square()?;
        let _t12 = _t11.compareTo(this)?;
        let _t13 = neighborUp.square()?;
        let _t14 = _t13.compareTo(this)?;
        String::new().append(&String::from("Square of result out for bounds rounding"))?;
        String::new().append(&rm)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(1i32);
        let _t15 = result.square()?;
        let _t16 = _t15.compareTo(this)?;
        let _t17 = neighborDown.square()?;
        let _t18 = _t17.compareTo(this)?;
        String::new().append(&String::from("Square of result out for bounds rounding"))?;
        String::new().append(&rm)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(1i32);
        let _t19 = result.square()?;
        let _t20 = _t19.subtract(this)?;
        let _t21 = _t20.abs()?;
        let mut err: Object = _t21;
        let _t22 = neighborUp.square()?;
        let _t23 = _t22.subtract(this)?;
        let mut errUp: Object = _t23;
        let _t24 = neighborDown.square()?;
        let _t25 = this.subtract(_t24)?;
        let mut errDown: Object = _t25;
        let _t26 = err.compareTo(errUp)?;
        let mut err_comp_errUp: i32 = _t26;
        let _t27 = err.compareTo(errDown)?;
        let mut err_comp_errDown: i32 = _t27;
        let _t28 = errUp.signum()?;
        let _t29 = errDown.signum()?;
        return Err(JvmError::Custom(String::from("athrow")));
        String::new().append(&String::from("Computed square root has larger error than neighbors for"))?;
        String::new().append(&rm)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(1i32);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "squareRootZeroResultAssertions", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Z", access = "private"))]
    pub fn squareRootZeroResultAssertions(&self, result: Object, mc: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.compareTo(BigDecimal::ZERO())?;
        Ok(_t0==0i32)
    }

    #[cfg_attr(any(), java_method(name = "pow", descriptor = "(I)Ljava/math/BigDecimal;", access = "public"))]
    // java: pow(I)Ljava/math/BigDecimal;
    pub fn pow__i(&self, n: i32) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.checkScale(((this.scale.get() as i64)).wrapping_mul((n as i64)))?;
        let mut newScale: i32 = _t0;
        let _t1 = this.inflated()?;
        let _t2 = _t1.pow(n)?;
        Ok(BigDecimal::new(_t2, newScale)?)
    }

    #[cfg_attr(any(), java_method(name = "pow", descriptor = "(ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: pow(ILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn pow__i_mathco(&self, n: i32, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.pow(n)?;
        return Ok(_t0);
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(BigDecimal::ONE());
        let mut lhs: java/math/BigDecimal = this;
        let mut workmc: Object = mc;
        let _t1: i32 = (n).abs();
        let mut mag: i32 = _t1;
        let _t2: i32 = BigDecimal::longDigitLength((mag as i64))?;
        let mut elength: i32 = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        workmc = MathContext::new(((mc.precision.get()).wrapping_add(elength)).wrapping_add(1i32), mc.roundingMode.get())?;
        elength = BigDecimal::ONE();
        let mut seenbit: i32 = 0i32;
        let mut i: i32 = 1i32;
        mag = (mag).wrapping_add(mag);
        seenbit = 1i32;
        let _t3 = elength.multiply(lhs, workmc)?;
        elength = _t3;
        let _t4 = elength.multiply(elength, workmc)?;
        elength = _t4;
        i = i.wrapping_add(1i32);
        let _t5 = BigDecimal::ONE().divide(elength, workmc)?;
        elength = _t5;
        let _t6: Object = BigDecimal::doRound(elength, mc)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "()Ljava/math/BigDecimal;", access = "public"))]
    // java: abs()Ljava/math/BigDecimal;
    pub fn abs(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.signum()?;
        let _t1 = this.negate()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: abs(Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn abs__mathco(&self, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.signum()?;
        let _t1 = this.negate(mc)?;
        let _t2 = this.plus(mc)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "negate", descriptor = "()Ljava/math/BigDecimal;", access = "public"))]
    // java: negate()Ljava/math/BigDecimal;
    pub fn negate(&self) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        let _t0 = this.intVal.get().negate()?;
        return Ok(BigDecimal::new(_t0, 9223372036854775808i64, this.scale.get(), this.precision.get())?);
        /* TODO: lneg  */
        let _t1: Object = BigDecimal::valueOf(this.intCompact.get(), this.scale.get(), this.precision.get())?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "negate", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: negate(Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn negate__mathco(&self, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.negate()?;
        let _t1 = _t0.plus(mc)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "plus", descriptor = "()Ljava/math/BigDecimal;", access = "public"))]
    // java: plus()Ljava/math/BigDecimal;
    pub fn plus(&self) -> Result<Object> {
        let this = self;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "plus", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    // java: plus(Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn plus__mathco(&self, mc: Object) -> Result<Object> {
        let this = self;
        return Ok(this);
        let _t0: Object = BigDecimal::doRound(this, mc)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "()I", access = "public"))]
    pub fn signum(&self) -> Result<i32> {
        let this = self;
        /* TODO: lcmp  */
        let _t0: i32 = Long::signum(this.intCompact.get())?;
        let _t1 = this.intVal.get().signum()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "scale", descriptor = "()I", access = "public"))]
    pub fn scale(&self) -> Result<i32> {
        let this = self;
        Ok(this.scale.get())
    }

    #[cfg_attr(any(), java_method(name = "precision", descriptor = "()I", access = "public"))]
    // java: precision()I
    pub fn precision(&self) -> Result<i32> {
        let this = self;
        let mut result: i32 = this.precision.get();
        let mut s: i64 = this.intCompact.get();
        /* TODO: lcmp  */
        let _t0: i32 = BigDecimal::longDigitLength(s)?;
        result = _t0;
        let _t1: i32 = BigDecimal::bigDigitLength(this.intVal.get())?;
        result = _t1;
        this.precision.set(result);
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "unscaledValue", descriptor = "()Ljava/math/BigInteger;", access = "public"))]
    pub fn unscaledValue(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.inflated()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "round", descriptor = "(Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "public"))]
    pub fn round(&self, mc: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.plus(mc)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setScale", descriptor = "(ILjava/math/RoundingMode;)Ljava/math/BigDecimal;", access = "public"))]
    // java: setScale(ILjava/math/RoundingMode;)Ljava/math/BigDecimal;
    pub fn setScale__i_roundi(&self, newScale: i32, roundingMode: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.setScale(newScale, roundingMode.oldMode.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setScale", descriptor = "(II)Ljava/math/BigDecimal;", access = "public"))]
    // java: setScale(II)Ljava/math/BigDecimal;
    pub fn setScale__i_i(&self, newScale: i32, roundingMode: i32) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut oldScale: i32 = this.scale.get();
        return Ok(this);
        let _t0 = this.signum()?;
        let _t1: Object = BigDecimal::zeroValueOf(newScale)?;
        return Ok(_t1);
        /* TODO: lcmp  */
        let mut rs: i64 = this.intCompact.get();
        let _t2 = this.checkScale(((newScale as i64)).wrapping_sub((oldScale as i64)))?;
        let mut raise: i32 = _t2;
        let _t3: i64 = BigDecimal::longMultiplyPowerTen(rs, raise)?;
        /* TODO: dup2  */
        rs = _t3;
        /* TODO: lcmp  */
        let _t4: Object = BigDecimal::valueOf(rs, newScale)?;
        return Ok(_t4);
        let _t5 = this.bigMultiplyPowerTen(raise)?;
        let mut rb: Object = _t5;
        /* invokespecial Method java/math/BigDecimal.<init>:(Ljava/math/BigInteger;JII)V */
        return Ok(rb);
        let _t6 = this.checkScale(((oldScale as i64)).wrapping_sub((newScale as i64)))?;
        raise = _t6;
        let _t7: Object = BigDecimal::divideAndRound(rs, BigDecimal::LONG_TEN_POWERS_TABLE()[raise as usize], newScale, roundingMode, newScale)?;
        return Ok(_t7);
        let _t8 = this.inflated()?;
        let _t9: Object = BigDecimal::bigTenToThe(raise)?;
        let _t10: Object = BigDecimal::divideAndRound(_t8, _t9, newScale, roundingMode, newScale)?;
        return Ok(_t10);
        let _t11 = this.checkScale(((newScale as i64)).wrapping_sub((oldScale as i64)))?;
        rs = _t11;
        let _t12: Object = BigDecimal::bigMultiplyPowerTen(this.intVal.get(), rs)?;
        let mut rb: Object = _t12;
        /* invokespecial Method java/math/BigDecimal.<init>:(Ljava/math/BigInteger;JII)V */
        return Ok(rb);
        let _t13 = this.checkScale(((oldScale as i64)).wrapping_sub((newScale as i64)))?;
        rs = _t13;
        let _t14: Object = BigDecimal::divideAndRound(this.intVal.get(), BigDecimal::LONG_TEN_POWERS_TABLE()[rs as usize], newScale, roundingMode, newScale)?;
        return Ok(_t14);
        let _t15: Object = BigDecimal::bigTenToThe(rs)?;
        let _t16: Object = BigDecimal::divideAndRound(this.intVal.get(), _t15, newScale, roundingMode, newScale)?;
        Ok(_t16)
    }

    #[cfg_attr(any(), java_method(name = "setScale", descriptor = "(I)Ljava/math/BigDecimal;", access = "public"))]
    // java: setScale(I)Ljava/math/BigDecimal;
    pub fn setScale__i(&self, newScale: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.setScale(newScale, 7i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "movePointLeft", descriptor = "(I)Ljava/math/BigDecimal;", access = "public"))]
    pub fn movePointLeft(&self, n: i32) -> Result<Object> {
        let this = self;
        return Ok(this);
        let _t0 = this.checkScale(((this.scale.get() as i64)).wrapping_add((n as i64)))?;
        let mut newScale: i32 = _t0;
        let mut num: BigDecimal = BigDecimal::new(this.intVal.get(), this.intCompact.get(), newScale, 0i32)?;
        let _t1 = num.setScale(0i32, 7i32)?;
        Ok(num)
    }

    #[cfg_attr(any(), java_method(name = "movePointRight", descriptor = "(I)Ljava/math/BigDecimal;", access = "public"))]
    pub fn movePointRight(&self, n: i32) -> Result<Object> {
        let this = self;
        return Ok(this);
        let _t0 = this.checkScale(((this.scale.get() as i64)).wrapping_sub((n as i64)))?;
        let mut newScale: i32 = _t0;
        let mut num: BigDecimal = BigDecimal::new(this.intVal.get(), this.intCompact.get(), newScale, 0i32)?;
        let _t1 = num.setScale(0i32, 7i32)?;
        Ok(num)
    }

    #[cfg_attr(any(), java_method(name = "scaleByPowerOfTen", descriptor = "(I)Ljava/math/BigDecimal;", access = "public"))]
    pub fn scaleByPowerOfTen(&self, n: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.checkScale(((this.scale.get() as i64)).wrapping_sub((n as i64)))?;
        Ok(BigDecimal::new(this.intVal.get(), this.intCompact.get(), _t0, this.precision.get())?)
    }

    #[cfg_attr(any(), java_method(name = "stripTrailingZeros", descriptor = "()Ljava/math/BigDecimal;", access = "public"))]
    pub fn stripTrailingZeros(&self) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        let _t0 = this.intVal.get().signum()?;
        return Ok(BigDecimal::ZERO());
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::createAndStripZerosToMatchScale(this.intCompact.get(), this.scale.get(), 9223372036854775808i64)?;
        return Ok(_t1);
        let _t2: Object = BigDecimal::createAndStripZerosToMatchScale(this.intVal.get(), this.scale.get(), 9223372036854775808i64)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/math/BigDecimal;)I", access = "public"))]
    pub fn compareTo(&self, val: Object) -> Result<i32> {
        let this = self;
        let mut xs: i64 = this.intCompact.get();
        let mut ys: i64 = val.intCompact.get();
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(ys<=0i32);
        let _t0 = this.signum()?;
        xs = _t0;
        let _t1 = val.signum()?;
        let mut ysign: i32 = _t1;
        return Ok(xs <= ysign);
        return Ok(0i32);
        let _t2 = this.compareMagnitude(val)?;
        ys = _t2;
        Ok((ys).wrapping_neg())
    }

    #[cfg_attr(any(), java_method(name = "compareMagnitude", descriptor = "(Ljava/math/BigDecimal;)I", access = "private"))]
    pub fn compareMagnitude(&self, val: Object) -> Result<i32> {
        let this = self;
        let mut ys: i64 = val.intCompact.get();
        let mut xs: i64 = this.intCompact.get();
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(0i64!=0i32);
        /* TODO: lcmp  */
        return Ok(1i32);
        let mut sdiff: i64 = ((this.scale.get() as i64)).wrapping_sub((val.scale.get() as i64));
        /* TODO: lcmp  */
        let _t0 = this.precision()?;
        let mut xae: i64 = ((_t0 as i64)).wrapping_sub((this.scale.get() as i64));
        let _t1 = val.precision()?;
        let mut yae: i64 = ((_t1 as i64)).wrapping_sub((val.scale.get() as i64));
        /* TODO: lcmp  */
        return Ok(-1i32);
        /* TODO: lcmp  */
        return Ok(1i32);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t2: i64 = BigDecimal::longMultiplyPowerTen(xs, (sdiff as i32))?;
        /* TODO: dup2  */
        xs = _t2;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t3 = this.bigMultiplyPowerTen((sdiff as i32))?;
        let mut rb: Object = _t3;
        let _t4 = rb.compareMagnitude(val.intVal.get())?;
        return Ok(_t4);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t5: i64 = BigDecimal::longMultiplyPowerTen(ys, (sdiff as i32))?;
        /* TODO: dup2  */
        ys = _t5;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t6 = val.bigMultiplyPowerTen((sdiff as i32))?;
        rb = _t6;
        let _t7 = this.intVal.get().compareMagnitude(rb)?;
        return Ok(_t7);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t8: i32 = BigDecimal::longCompareMagnitude(xs, ys)?;
        return Ok(-1i32);
        /* TODO: lcmp  */
        return Ok(1i32);
        let _t9 = this.intVal.get().compareMagnitude(val.intVal.get())?;
        Ok(_t9)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, x: Object) -> Result<bool> {
        let this = self;
        let mut xDec: Object = x;
        return Ok(0i32);
        return Ok(1i32);
        return Ok(0i32);
        let mut s: i64 = this.intCompact.get();
        let mut xs: i64 = xDec.intCompact.get();
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t0: i64 = BigDecimal::compactValFor(xDec.intVal.get())?;
        xs = _t0;
        /* TODO: lcmp  */
        return Ok(s==0i32);
        /* TODO: lcmp  */
        let _t1: i64 = BigDecimal::compactValFor(this.intVal.get())?;
        /* TODO: lcmp  */
        return Ok(_t1==0i32);
        let _t2 = this.inflated()?;
        let _t3 = xDec.inflated()?;
        let _t4 = _t2.equals(_t3)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    pub fn min(&self, val: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.compareTo(val)?;
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;", access = "public"))]
    pub fn max(&self, val: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.compareTo(val)?;
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let mut val2: i64 = this.intCompact.get();
        /* TODO: lushr  */
        /* TODO: land  */
        let mut temp: i32 = ((val2).wrapping_add(4294967295i64) as i32);
        /* TODO: lcmp  */
        return Ok((((temp).wrapping_neg()).wrapping_mul(temp)).wrapping_add(this.scale.get()));
        let _t0 = this.intVal.get().hashCode()?;
        Ok(((31i32).wrapping_mul(_t0)).wrapping_add(this.scale.get()))
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut sc: String = this.stringCache.get();
        let _t0 = this.layoutChars(1i32)?;
        sc = _t0;
        this.stringCache.set(_t0);
        Ok(sc)
    }

    #[cfg_attr(any(), java_method(name = "toEngineeringString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toEngineeringString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.layoutChars(0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toPlainString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toPlainString(&self) -> Result<String> {
        let this = self;
        /* TODO: lcmp  */
        let _t0: String = Long::toString(this.intCompact.get())?;
        return Ok(_t0);
        let _t1 = this.intVal.get().toString()?;
        return Ok(_t1);
        let _t2 = this.signum()?;
        return Ok(String::from("0"));
        /* TODO: lneg  */
        let _t3: i32 = BigDecimal::checkScaleNonZero((this.scale.get() as i64))?;
        let mut trailingZeros: i32 = _t3;
        /* TODO: lcmp  */
        let _t4: String = Long::toString(this.intCompact.get())?;
        let _t5 = this.intVal.get().toString()?;
        let mut str: String = _t5;
        let _t6 = str.length()?;
        let mut len: i32 = (_t6).wrapping_add(trailingZeros);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut buf: String = String::new();
        buf.append(&str)?;
        let _t7 = buf.repeat(48i32, trailingZeros)?;
        return Ok(buf);
        /* TODO: lcmp  */
        let _t8: i64 = (this.intCompact.get()).abs();
        let _t9: String = Long::toString(_t8)?;
        trailingZeros = _t9;
        let _t10 = this.intVal.get().abs()?;
        let _t11 = _t10.toString()?;
        trailingZeros = _t11;
        let _t12 = this.signum()?;
        let _t13: String = BigDecimal::getValueString(_t12, trailingZeros, this.scale.get())?;
        Ok(_t13)
    }

    #[cfg_attr(any(), java_method(name = "getValueString", descriptor = "(ILjava/lang/String;I)Ljava/lang/String;", access = "private static"))]
    pub fn getValueString(signum: i32, intString: String, scale: i32) -> Result<String> {
        let _t0 = intString.length()?;
        let mut insertionPoint: i32 = (_t0).wrapping_sub(scale);
        String::from("-0.").append(&String::from("0."))?;
        String::from("-0.").append(&intString)?;
        return Ok(String::from("-0."));
        let mut buf: String = String::new();
        let _t1 = buf.insert(insertionPoint, 46i32)?;
        let _t2 = buf.insert(0i32, 45i32)?;
        let mut len: i32 = (signum>=0i32).wrapping_add(scale);
        return Err(JvmError::Custom(String::from("athrow")));
        buf = String::new();
        String::from("-0.").append(&String::from("0."))?;
        let _ = String::from("-0.");
        let _t3 = buf.repeat(48i32, (insertionPoint).wrapping_neg())?;
        buf.append(&intString)?;
        Ok(buf)
    }

    #[cfg_attr(any(), java_method(name = "toBigInteger", descriptor = "()Ljava/math/BigInteger;", access = "public"))]
    pub fn toBigInteger(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.setScale(0i32, 1i32)?;
        let _t1 = _t0.inflated()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toBigIntegerExact", descriptor = "()Ljava/math/BigInteger;", access = "public"))]
    pub fn toBigIntegerExact(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.setScale(0i32, 7i32)?;
        let _t1 = _t0.inflated()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "longValue", descriptor = "()J", access = "public"))]
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(this.intCompact.get());
        let _t0 = this.signum()?;
        let _t1 = this.fractionOnly()?;
        return Ok(0i64);
        let _t2 = this.toBigInteger()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "fractionOnly", descriptor = "()Z", access = "private"))]
    pub fn fractionOnly(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.signum()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.precision()?;
        Ok(_t1 <= this.scale.get())
    }

    #[cfg_attr(any(), java_method(name = "longValueExact", descriptor = "()J", access = "public"))]
    pub fn longValueExact(&self) -> Result<i64> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(this.intCompact.get());
        let _t0 = this.signum()?;
        return Ok(0i64);
        let _t1 = this.fractionOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.precision()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = this.setScale(0i32, 7i32)?;
        let mut num: Object = _t3;
        let _t4 = num.precision()?;
        BigDecimal$LongOverflow::check(num)?;
        let _t5 = num.inflated()?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "intValue", descriptor = "()I", access = "public"))]
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        /* TODO: lcmp  */
        Ok((this as i32))
    }

    #[cfg_attr(any(), java_method(name = "intValueExact", descriptor = "()I", access = "public"))]
    pub fn intValueExact(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.longValueExact()?;
        let mut num: i64 = _t0;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok((num as i32))
    }

    #[cfg_attr(any(), java_method(name = "shortValueExact", descriptor = "()S", access = "public"))]
    pub fn shortValueExact(&self) -> Result<i16> {
        let this = self;
        let _t0 = this.longValueExact()?;
        let mut num: i64 = _t0;
        /* TODO: i2s  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: i2s  */
        Ok((num as i32))
    }

    #[cfg_attr(any(), java_method(name = "byteValueExact", descriptor = "()B", access = "public"))]
    pub fn byteValueExact(&self) -> Result<i8> {
        let this = self;
        let _t0 = this.longValueExact()?;
        let mut num: i64 = _t0;
        /* TODO: i2b  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: i2b  */
        Ok((num as i32))
    }

    #[cfg_attr(any(), java_method(name = "floatValue", descriptor = "()F", access = "public"))]
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        /* TODO: lcmp  */
        /* TODO: l2f  */
        let mut v: f32 = this.intCompact.get();
        return Ok(v);
        /* TODO: f2l  */
        /* TODO: lcmp  */
        return Ok((v/BigDecimal::FLOAT_10_POW()[this.scale.get() as usize]));
        return Ok((v*BigDecimal::FLOAT_10_POW()[(this.scale.get()).wrapping_neg() as usize]));
        let _t0 = this.fullFloatValue()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "fullFloatValue", descriptor = "()F", access = "private"))]
    pub fn fullFloatValue(&self) -> Result<f32> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(0f32);
        let _t0 = this.unscaledValue()?;
        let _t1 = _t0.abs()?;
        let mut w: Object = _t1;
        let _t2 = w.bitLength()?;
        let _t3: f64 = (((this.scale.get() as f64)*3.321928094887362f64) as f64).ceil();
        /* TODO: d2l  */
        let mut qb: i64 = ((_t2 as i64)).wrapping_sub(_t3);
        /* TODO: lcmp  */
        let _t4 = this.signum()?;
        return Ok(((_t4 as f32)*0f32));
        /* TODO: lcmp  */
        let _t5 = this.signum()?;
        return Ok(((_t5 as f32)*723i32));
        let _t6 = this.signum()?;
        let _t7: Object = BigDecimal::bigTenToThe((this.scale.get()).wrapping_neg())?;
        let _t8 = w.multiply(_t7)?;
        return Ok(((_t6 as f32)*_t8));
        let _t9 = this.signum()?;
        return Ok(((_t9 as f32)*w));
        let mut ql: i32 = ((qb as i32)).wrapping_sub(27i32);
        let _t10: Object = BigDecimal::bigTenToThe(this.scale.get())?;
        let mut pow10: Object = _t10;
        let _t11 = w.shiftLeft((ql).wrapping_neg())?;
        let mut m: Object = _t11;
        let mut n: Object = pow10;
        m = w;
        let _t12 = pow10.shiftLeft(ql)?;
        n = _t12;
        let _t13 = m.divideAndRemainder(n)?;
        let mut qr: Vec<Object> = _t13;
        let mut i: i32 = qr[0i32 as usize].clone();
        let _t14 = qr[1i32 as usize].clone().signum()?;
        let mut sb: i32 = _t14;
        let _t15: i32 = Integer::numberOfLeadingZeros(i)?;
        let mut dq: i32 = (6i32).wrapping_sub(_t15);
        let mut eq: i32 = (-151i32).wrapping_sub(ql);
        let _t16 = this.signum()?;
        let _t17: f32 = (((i|sb) as f32)).abs();
        return Ok(((_t16 as f32)*_t17));
        let mut mask: i32 = ((1i32<<(eq&0x1f))).wrapping_sub(1i32);
        let _t18: i32 = Integer::signum((i&mask))?;
        let mut j: i32 = (((i>>((eq&0x1f)))|_t18)|sb);
        let _t19 = this.signum()?;
        let _t20: f32 = ((j as f32)).abs();
        Ok(((_t19 as f32)*_t20))
    }

    #[cfg_attr(any(), java_method(name = "doubleValue", descriptor = "()D", access = "public"))]
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        /* TODO: lcmp  */
        /* TODO: l2d  */
        let mut v: f64 = this.intCompact.get();
        return Ok(v);
        /* TODO: d2l  */
        /* TODO: lcmp  */
        return Ok((v/BigDecimal::DOUBLE_10_POW()[this.scale.get() as usize]));
        return Ok((v*BigDecimal::DOUBLE_10_POW()[(this.scale.get()).wrapping_neg() as usize]));
        let _t0 = this.fullDoubleValue()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "fullDoubleValue", descriptor = "()D", access = "private"))]
    pub fn fullDoubleValue(&self) -> Result<f64> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(0f64);
        let _t0 = this.unscaledValue()?;
        let _t1 = _t0.abs()?;
        let mut w: Object = _t1;
        let _t2 = w.bitLength()?;
        let _t3: f64 = (((this.scale.get() as f64)*3.321928094887362f64) as f64).ceil();
        /* TODO: d2l  */
        let mut qb: i64 = ((_t2 as i64)).wrapping_sub(_t3);
        /* TODO: lcmp  */
        let _t4 = this.signum()?;
        return Ok(((_t4 as f64)*0f64));
        /* TODO: lcmp  */
        let _t5 = this.signum()?;
        return Ok(((_t5 as f64)*inff64));
        let _t6 = this.signum()?;
        let _t7: Object = BigDecimal::bigTenToThe((this.scale.get()).wrapping_neg())?;
        let _t8 = w.multiply(_t7)?;
        return Ok(((_t6 as f64)*_t8));
        let _t9 = this.signum()?;
        return Ok(((_t9 as f64)*w));
        let mut ql: i32 = ((qb as i32)).wrapping_sub(56i32);
        let _t10: Object = BigDecimal::bigTenToThe(this.scale.get())?;
        let mut pow10: Object = _t10;
        let _t11 = w.shiftLeft((ql).wrapping_neg())?;
        let mut m: Object = _t11;
        let mut n: Object = pow10;
        m = w;
        let _t12 = pow10.shiftLeft(ql)?;
        n = _t12;
        let _t13 = m.divideAndRemainder(n)?;
        let mut qr: Vec<Object> = _t13;
        let mut i: i64 = qr[0i32 as usize].clone();
        let _t14 = qr[1i32 as usize].clone().signum()?;
        let mut sb: i32 = _t14;
        let _t15: i32 = Long::numberOfLeadingZeros(i)?;
        let mut dq: i32 = (9i32).wrapping_sub(_t15);
        let mut eq: i32 = (-1076i32).wrapping_sub(ql);
        let _t16 = this.signum()?;
        /* TODO: lor  */
        /* TODO: l2d  */
        let _t17: f64 = ((sb as i64)).abs();
        return Ok((i*_t17));
        /* TODO: lshl  */
        let mut mask: i64 = (eq).wrapping_sub(1i64);
        /* TODO: lshr  */
        /* TODO: land  */
        let _t18: i32 = Long::signum(mask)?;
        /* TODO: lor  */
        /* TODO: lor  */
        let mut j: i64 = (sb as i64);
        let _t19 = this.signum()?;
        /* TODO: l2d  */
        let _t20: f64 = (j).abs();
        Ok(((_t19 as f64)*_t20))
    }

    #[cfg_attr(any(), java_method(name = "ulp", descriptor = "()Ljava/math/BigDecimal;", access = "public"))]
    pub fn ulp(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.scale()?;
        let _t1: Object = BigDecimal::valueOf(1i64, _t0, 1i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "layoutChars", descriptor = "(Z)Ljava/lang/String;", access = "private"))]
    pub fn layoutChars(&self, sci: bool) -> Result<String> {
        let this = self;
        /* TODO: lcmp  */
        let _t0: String = Long::toString(this.intCompact.get())?;
        let _t1 = this.intVal.get().toString()?;
        return Ok(_t1);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut lowInt: i32 = ((this.intCompact.get() as i32)%100i32);
        let mut highInt: i32 = ((this.intCompact.get() as i32)/100i32);
        let _t2: String = Integer::toString(highInt)?;
        String::new().append(&_t2)?;
        String::new().append(&46i32)?;
        String::new().append(&BigDecimal$StringBuilderHelper::DIGIT_TENS()[lowInt as usize])?;
        String::new().append(&BigDecimal$StringBuilderHelper::DIGIT_ONES()[lowInt as usize])?;
        return Ok(String::new());
        lowInt = BigDecimal_StringBuilderHelper::new()?;
        /* TODO: lcmp  */
        let _t3: i64 = (this.intCompact.get()).abs();
        let _t4 = lowInt.putIntCompact(_t3)?;
        let mut offset: i32 = _t4;
        let _t5 = lowInt.getCompactCharArray()?;
        highInt = _t5;
        offset = 0i32;
        let _t6 = this.intVal.get().abs()?;
        let _t7 = _t6.toString()?;
        let _t8 = _t7.toCharArray()?;
        highInt = _t8;
        let _t9 = lowInt.getStringBuilder()?;
        let mut buf: Object = _t9;
        let _t10 = this.signum()?;
        buf.append(&45i32)?;
        let mut coeffLen: i32 = ((highInt.len() as i32)).wrapping_sub(offset);
        /* TODO: lneg  */
        let mut adjusted: i64 = ((this.scale.get() as i64)).wrapping_add(((coeffLen).wrapping_sub(1i32) as i64));
        /* TODO: lcmp  */
        let mut pad: i32 = (this.scale.get()).wrapping_sub(coeffLen);
        buf.append(&48i32)?;
        buf.append(&46i32)?;
        loop {
            if pad<=0i32 { break; }
            buf.append(&48i32)?;
            pad = pad.wrapping_sub(1i32);
        }
        buf.append(&highInt)?;
        buf.append(&highInt)?;
        buf.append(&46i32)?;
        buf.append(&highInt)?;
        buf.append(&highInt[offset as usize])?;
        buf.append(&46i32)?;
        buf.append(&highInt)?;
        /* TODO: lrem  */
        pad = (3i64 as i32);
        pad = pad.wrapping_add(3i32);
        adjusted = (adjusted).wrapping_sub((pad as i64));
        pad = pad.wrapping_add(1i32);
        let _t11 = this.signum()?;
        /* TODO: tableswitch default:499 low:1 high:3 */
        buf.append(&48i32)?;
        buf.append(&String::from("0.00"))?;
        adjusted = (adjusted).wrapping_add(3i64);
        buf.append(&String::from("0.0"))?;
        adjusted = (adjusted).wrapping_add(3i64);
        String::new().append(&String::from("Unexpected sig value"))?;
        String::new().append(&pad)?;
        return Err(JvmError::Custom(String::from("athrow")));
        buf.append(&highInt)?;
        let mut i: i32 = (pad).wrapping_sub(coeffLen);
        loop {
            if i<=0i32 { break; }
            buf.append(&48i32)?;
            i = i.wrapping_sub(1i32);
        }
        buf.append(&highInt)?;
        buf.append(&46i32)?;
        buf.append(&highInt)?;
        /* TODO: lcmp  */
        buf.append(&69i32)?;
        /* TODO: lcmp  */
        buf.append(&43i32)?;
        buf.append(&adjusted)?;
        Ok(buf)
    }

    #[cfg_attr(any(), java_method(name = "bigTenToThe", descriptor = "(I)Ljava/math/BigInteger;", access = "private static"))]
    pub fn bigTenToThe(n: i32) -> Result<Object> {
        return Ok(BigInteger::ZERO());
        let mut pows: Vec<Object> = BigDecimal::BIG_TEN_POWERS_TABLE();
        return Ok(pows[n as usize].clone());
        let _t0: Object = BigDecimal::expandBigIntegerTenPowers(n)?;
        return Ok(_t0);
        let _t1 = BigInteger::TEN().pow(n)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "expandBigIntegerTenPowers", descriptor = "(I)Ljava/math/BigInteger;", access = "private static"))]
    pub fn expandBigIntegerTenPowers(n: i32) -> Result<Object> {
        let mut local_1: i32 = 3i32;
        /* TODO: monitorenter  */
        let mut pows: Vec<Object> = BigDecimal::BIG_TEN_POWERS_TABLE();
        let mut curLen: i32 = (pows.len() as i32);
        let mut newLen: i32 = (curLen<<(1i32&0x1f));
        loop {
            if newLen > n { break; }
            newLen = (newLen<<(1i32&0x1f));
        }
        let _t0: Vec<Object> = Arrays::copyOf(&pows, newLen)?;
        pows = _t0;
        let mut i: i32 = curLen;
        loop {
            if i >= newLen { break; }
            let _t0 = pows[(i).wrapping_sub(1i32) as usize].clone().multiply(BigInteger::TEN())?;
            pows[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        BigDecimal::BIG_TEN_POWERS_TABLE(pows);
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_6: Object = pows[n as usize].clone();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "longMultiplyPowerTen", descriptor = "(JI)J", access = "private static"))]
    pub fn longMultiplyPowerTen(val: i64, arg_1: i32) -> Result<i64> {
        /* TODO: lcmp  */
        return Ok(val);
        let mut tab: Vec<i64> = BigDecimal::LONG_TEN_POWERS_TABLE();
        let mut bounds: Vec<i64> = BigDecimal::THRESHOLDS_TABLE();
        let mut tenpower: i64 = tab[local_2 as usize];
        /* TODO: lcmp  */
        return Ok(tenpower);
        let _t0: i64 = (val).abs();
        /* TODO: lcmp  */
        return Ok((val).wrapping_mul(tenpower));
        Ok(9223372036854775808i64)
    }

    #[cfg_attr(any(), java_method(name = "bigMultiplyPowerTen", descriptor = "(I)Ljava/math/BigInteger;", access = "private"))]
    // java: bigMultiplyPowerTen(I)Ljava/math/BigInteger;
    pub fn bigMultiplyPowerTen__i(&self, n: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.inflated()?;
        return Ok(_t0);
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::bigTenToThe(n)?;
        let _t2 = _t1.multiply(this.intCompact.get())?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::bigTenToThe(n)?;
        let _t4 = this.intVal.get().multiply(_t3)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "inflated", descriptor = "()Ljava/math/BigInteger;", access = "private"))]
    pub fn inflated(&self) -> Result<Object> {
        let this = self;
        return Ok(this.intCompact.get());
        Ok(this.intVal.get())
    }

    #[cfg_attr(any(), java_method(name = "matchScale", descriptor = "([Ljava/math/BigDecimal;)V", access = "private static"))]
    pub fn matchScale(val: &[Object]) -> Result<()> {
        let _t0 = val[0i32 as usize].clone().setScale(val[1i32 as usize].clone().scale.get(), 7i32)?;
        val[0i32 as usize] = _t0;
        let _t1 = val[1i32 as usize].clone().setScale(val[0i32 as usize].clone().scale.get(), 7i32)?;
        val[1i32 as usize] = _t1;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let mut fields: Object = _t0;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").get(fields, String::from("intVal"))?;
        let mut serialIntVal: Object = _t1;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = BigDecimal::toStrictBigInteger(serialIntVal)?;
        serialIntVal = _t2;
        let _t3 = fields.get(String::from("scale"), 0i32)?;
        let mut serialScale: i32 = _t3;
        BigDecimal$UnsafeHolder::setIntValAndScale(this, serialIntVal, serialScale)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObjectNoData", descriptor = "()V", access = "private"))]
    pub fn readObjectNoData(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        BigDecimal$UnsafeHolder::setIntValVolatile(this, this.intCompact.get())?;
        s.defaultWriteObject()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "longDigitLength", descriptor = "(J)I", access = "static"))]
    pub fn longDigitLength(x: i64) -> Result<i32> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lneg  */
        x = x;
        /* TODO: lcmp  */
        return Ok(1i32);
        let _t0: i32 = Long::numberOfLeadingZeros(x)?;
        let mut r: i32 = (((((64i32).wrapping_sub(_t0)).wrapping_add(1i32)).wrapping_mul(1233i32) as u32>>(12i32&0x1f)) as i32);
        let mut tab: Vec<i64> = BigDecimal::LONG_TEN_POWERS_TABLE();
        /* TODO: lcmp  */
        Ok((r).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "bigDigitLength", descriptor = "(Ljava/math/BigInteger;)I", access = "private static"))]
    pub fn bigDigitLength(b: Object) -> Result<i32> {
        return Ok(1i32);
        let _t0 = b.bitLength()?;
        /* TODO: lushr  */
        let mut r: i32 = (31i32 as i32);
        let _t1: Object = BigDecimal::bigTenToThe(r)?;
        let _t2 = b.compareMagnitude(_t1)?;
        Ok((r).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "checkScale", descriptor = "(J)I", access = "private"))]
    // java: checkScale(J)I
    pub fn checkScale__l(&self, val: i64) -> Result<i32> {
        let this = self;
        let mut asInt: i32 = (val as i32);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        asInt = 875i32;
        /* TODO: lcmp  */
        let mut b: Object = this.intVal.get();
        let _t0 = b.signum()?;
        /* invokespecial Method java/lang/ArithmeticException.<init>:(Ljava/lang/String;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(asInt)
    }

    #[cfg_attr(any(), java_method(name = "compactValFor", descriptor = "(Ljava/math/BigInteger;)J", access = "private static"))]
    pub fn compactValFor(b: Object) -> Result<i64> {
        let mut m: Vec<i32> = b.mag.get();
        let mut len: i32 = (m.len() as i32);
        return Ok(0i64);
        let mut d: i32 = m[0i32 as usize];
        return Ok(9223372036854775808i64);
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: land  */
        let mut u: i64 = 4294967295i64;
        /* TODO: lneg  */
        Ok(u)
    }

    #[cfg_attr(any(), java_method(name = "longCompareMagnitude", descriptor = "(JJ)I", access = "private static"))]
    pub fn longCompareMagnitude(x: i64, arg_1: i64) -> Result<i32> {
        /* TODO: lcmp  */
        /* TODO: lneg  */
        x = x;
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let mut y: i64 = local_2;
        let _t0: i32 = Long::compare(x, y)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "saturateLong", descriptor = "(J)I", access = "private static"))]
    pub fn saturateLong(s: i64) -> Result<i32> {
        let mut i: i32 = (s as i32);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        Ok(874i32)
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/String;Ljava/math/BigDecimal;)V", access = "private static"))]
    pub fn print(name: String, bd: Object) -> Result<()> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(5i32 as usize);
        _arr0[0i32 as usize] = name;
        _arr0[1i32 as usize] = bd.intCompact.get();
        _arr0[2i32 as usize] = bd.intVal.get();
        _arr0[3i32 as usize] = bd.scale.get();
        _arr0[4i32 as usize] = bd.precision.get();
        let _t1 = System::err().format(String::from("%s:	intCompact %d	intVal %d	scale %d	precision %d%n"), _arr0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "audit", descriptor = "()Ljava/math/BigDecimal;", access = "private"))]
    pub fn audit(&self) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        BigDecimal::print(String::from("audit"), this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = BigDecimal::bigDigitLength(this.intVal.get())?;
        BigDecimal::print(String::from("audit"), this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut val: i64 = this.intVal.get();
        /* TODO: lcmp  */
        BigDecimal::print(String::from("audit"), this)?;
        String::new().append(&String::from("Inconsistent state, intCompact="))?;
        String::new().append(&this.intCompact.get())?;
        String::new().append(&String::from("intVal="))?;
        String::new().append(&val)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: i32 = BigDecimal::longDigitLength(this.intCompact.get())?;
        BigDecimal::print(String::from("audit"), this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "checkScaleNonZero", descriptor = "(J)I", access = "private static"))]
    pub fn checkScaleNonZero(val: i64) -> Result<i32> {
        let mut asInt: i32 = (val as i32);
        /* TODO: lcmp  */
        /* invokespecial Method java/lang/ArithmeticException.<init>:(Ljava/lang/String;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(asInt)
    }

    #[cfg_attr(any(), java_method(name = "checkScale", descriptor = "(JJ)I", access = "private static"))]
    // java: checkScale(JJ)I
    pub fn checkScale__l_l(intCompact: i64, arg_1: i64) -> Result<i32> {
        let mut asInt: i32 = (local_2 as i32);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        asInt = 875i32;
        /* TODO: lcmp  */
        /* invokespecial Method java/lang/ArithmeticException.<init>:(Ljava/lang/String;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(asInt)
    }

    #[cfg_attr(any(), java_method(name = "checkScale", descriptor = "(Ljava/math/BigInteger;J)I", access = "private static"))]
    // java: checkScale(Ljava/math/BigInteger;J)I
    pub fn checkScale__bigint_l(intVal: Object, val: i64) -> Result<i32> {
        let mut asInt: i32 = (val as i32);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        asInt = 875i32;
        let _t0 = intVal.signum()?;
        /* invokespecial Method java/lang/ArithmeticException.<init>:(Ljava/lang/String;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(asInt)
    }

    #[cfg_attr(any(), java_method(name = "doRound", descriptor = "(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: doRound(Ljava/math/BigDecimal;Ljava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn doRound__bigdec_mathco(val: Object, mc: Object) -> Result<Object> {
        let mut mcp: i32 = mc.precision.get();
        let mut wasDivided: i32 = 0i32;
        let mut intVal: Object = val.intVal.get();
        let mut compactVal: i64 = val.intCompact.get();
        let mut scale: i32 = val.scale.get();
        let _t0 = val.precision()?;
        let mut prec: i32 = _t0;
        let mut mode: i32 = mc.roundingMode.get().oldMode.get();
        /* TODO: lcmp  */
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: Object = BigDecimal::divideAndRoundByTenPow(intVal, drop, mode)?;
            intVal = _t1;
            wasDivided = 1i32;
            let _t2: i64 = BigDecimal::compactValFor(intVal)?;
            compactVal = _t2;
            /* TODO: lcmp  */
            let _t3: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t3;
            let _t4: i32 = BigDecimal::bigDigitLength(intVal)?;
            prec = _t4;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: lcmp  */
        drop = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mc.roundingMode.get().oldMode.get())?;
            compactVal = _t1;
            wasDivided = 1i32;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
            /* TODO: aconst_null  */
            intVal = todo!("stack underflow");
        }
        return Ok(val);
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "doRound", descriptor = "(JILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: doRound(JILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn doRound__l_i_mathco(compactVal: i64, arg_1: i32, scale: Object) -> Result<Object> {
        let mut mcp: i32 = local_3.precision.get();
        let _t0: i32 = BigDecimal::longDigitLength(compactVal)?;
        let mut prec: i32 = _t0;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], local_3.roundingMode.get().oldMode.get())?;
            compactVal = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        let _t1: Object = BigDecimal::valueOf(compactVal, scale, prec)?;
        return Ok(_t1);
        let _t2: Object = BigDecimal::valueOf(compactVal, scale)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "doRound", descriptor = "(Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: doRound(Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn doRound__bigint_i_mathco(intVal: Object, scale: i32, mc: Object) -> Result<Object> {
        let mut mcp: i32 = mc.precision.get();
        let mut prec: i32 = 0i32;
        let _t0: i64 = BigDecimal::compactValFor(intVal)?;
        let mut compactVal: i64 = _t0;
        let mut mode: i32 = mc.roundingMode.get().oldMode.get();
        /* TODO: lcmp  */
        let _t1: i32 = BigDecimal::bigDigitLength(intVal)?;
        prec = _t1;
        let mut drop: i32 = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: Object = BigDecimal::divideAndRoundByTenPow(intVal, drop, mode)?;
            intVal = _t1;
            let _t2: i64 = BigDecimal::compactValFor(intVal)?;
            compactVal = _t2;
            /* TODO: lcmp  */
            let _t3: i32 = BigDecimal::bigDigitLength(intVal)?;
            prec = _t3;
            drop = (prec).wrapping_sub(mcp);
        }
        /* TODO: lcmp  */
        let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
        prec = _t2;
        drop = (prec).wrapping_sub(mcp);
        loop {
            if drop<=0i32 { break; }
            let _t0: i32 = BigDecimal::checkScaleNonZero(((scale as i64)).wrapping_sub((drop as i64)))?;
            scale = _t0;
            let _t1: i64 = BigDecimal::divideAndRound(compactVal, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], mc.roundingMode.get().oldMode.get())?;
            compactVal = _t1;
            let _t2: i32 = BigDecimal::longDigitLength(compactVal)?;
            prec = _t2;
            drop = (prec).wrapping_sub(mcp);
        }
        let _t3: Object = BigDecimal::valueOf(compactVal, scale, prec)?;
        return Ok(_t3);
        Ok(BigDecimal::new(intVal, 9223372036854775808i64, scale, prec)?)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRoundByTenPow", descriptor = "(Ljava/math/BigInteger;II)Ljava/math/BigInteger;", access = "private static"))]
    pub fn divideAndRoundByTenPow(intVal: Object, tenPow: i32, roundingMode: i32) -> Result<Object> {
        let _t0: Object = BigDecimal::divideAndRound(intVal, BigDecimal::LONG_TEN_POWERS_TABLE()[tenPow as usize], roundingMode)?;
        intVal = _t0;
        let _t1: Object = BigDecimal::bigTenToThe(tenPow)?;
        let _t2: Object = BigDecimal::divideAndRound(intVal, _t1, roundingMode)?;
        intVal = _t2;
        Ok(intVal)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(JJIII)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divideAndRound(JJIII)Ljava/math/BigDecimal;
    pub fn divideAndRound__l_l_i_i_i(ldividend: i64, arg_1: i64, ldivisor: i32, arg_3: i32, scale: i32) -> Result<Object> {
        let mut q: i64 = (ldividend/ldivisor);
        let _t0: Object = BigDecimal::valueOf(q, scale)?;
        return Ok(_t0);
        /* TODO: lrem  */
        let mut r: i64 = ldivisor;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut qsign: i32 = ldivisor != 0i64<0i32;
        /* TODO: lcmp  */
        let _t1: bool = BigDecimal::needIncrement(ldivisor, local_5, qsign, q, r)?;
        let mut increment: i32 = _t1;
        let _t2: Object = BigDecimal::valueOf(q, scale)?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::createAndStripZerosToMatchScale(q, scale, (local_6 as i64))?;
        return Ok(_t3);
        let _t4: Object = BigDecimal::valueOf(q, scale)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(JJI)J", access = "private static"))]
    // java: divideAndRound(JJI)J
    pub fn divideAndRound__l_l_i(ldividend: i64, arg_1: i64, ldivisor: i32) -> Result<i64> {
        let mut q: i64 = (ldividend/ldivisor);
        return Ok(q);
        /* TODO: lrem  */
        let mut r: i64 = ldivisor;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut qsign: i32 = ldivisor != 0i64<0i32;
        /* TODO: lcmp  */
        let _t0: bool = BigDecimal::needIncrement(ldivisor, local_4, qsign, q, r)?;
        let mut increment: i32 = _t0;
        return Ok(q);
        Ok(q)
    }

    #[cfg_attr(any(), java_method(name = "commonNeedIncrement", descriptor = "(IIIZ)Z", access = "private static"))]
    pub fn commonNeedIncrement(roundingMode: i32, qsign: i32, cmpFracHalf: i32, oddQuot: bool) -> Result<bool> {
        /* TODO: tableswitch default:83 low:0 high:7 */
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(1i32);
        return Ok(0i32);
        return Ok(qsign>0i32);
        return Ok(qsign<0i32);
        String::new().append(&String::from("Unexpected rounding mode"))?;
        let _t0: Object = RoundingMode::valueOf(roundingMode)?;
        String::new().append(&_t0)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(0i32);
        return Ok(1i32);
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: tableswitch default:200 low:4 high:6 */
        String::new().append(&String::from("Unexpected rounding mode"))?;
        String::new().append(&roundingMode)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(oddQuot)
    }

    #[cfg_attr(any(), java_method(name = "needIncrement", descriptor = "(JIIJJ)Z", access = "private static"))]
    // java: needIncrement(JIIJJ)Z
    pub fn needIncrement__l_i_i_l_l(ldivisor: i64, arg_1: i32, roundingMode: i32, qsign: i64, q: i64) -> Result<bool> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut cmpFracHalf: i32 = 1i32;
        let _t0: i32 = BigDecimal::longCompareMagnitude((2i64).wrapping_mul(local_6), ldivisor)?;
        cmpFracHalf = _t0;
        /* TODO: land  */
        /* TODO: lcmp  */
        let _t1: bool = BigDecimal::commonNeedIncrement(cmpFracHalf, q, 1i64, 0i64!=0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;JI)Ljava/math/BigInteger;", access = "private static"))]
    // java: divideAndRound(Ljava/math/BigInteger;JI)Ljava/math/BigInteger;
    pub fn divideAndRound__bigint_l_i(bdividend: Object, ldivisor: i64, arg_2: i32) -> Result<Object> {
        let mut mdividend: MutableBigInteger = MutableBigInteger::new(bdividend.mag.get())?;
        let mut mq: MutableBigInteger = MutableBigInteger::new()?;
        let _t0 = mdividend.divide(ldivisor, mq)?;
        let mut r: i64 = _t0;
        /* TODO: lcmp  */
        let mut isRemainderZero: i32 = 0i64==0i32;
        /* TODO: lcmp  */
        let mut qsign: i32 = bdividend.signum.get();
        let _t1: bool = BigDecimal::needIncrement(ldivisor, local_3, qsign, mq, r)?;
        mq.add(MutableBigInteger::ONE())?;
        let _t2 = mq.toBigInteger(qsign)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;JIII)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divideAndRound(Ljava/math/BigInteger;JIII)Ljava/math/BigDecimal;
    pub fn divideAndRound__bigint_l_i_i_i(bdividend: Object, ldivisor: i64, arg_2: i32, scale: i32, roundingMode: i32) -> Result<Object> {
        let mut mdividend: MutableBigInteger = MutableBigInteger::new(bdividend.mag.get())?;
        let mut mq: MutableBigInteger = MutableBigInteger::new()?;
        let _t0 = mdividend.divide(ldivisor, mq)?;
        let mut r: i64 = _t0;
        /* TODO: lcmp  */
        let mut isRemainderZero: i32 = 0i64==0i32;
        /* TODO: lcmp  */
        let mut qsign: i32 = bdividend.signum.get();
        let _t1: bool = BigDecimal::needIncrement(ldivisor, roundingMode, qsign, mq, r)?;
        mq.add(MutableBigInteger::ONE())?;
        let _t2 = mq.toBigDecimal(qsign, scale)?;
        return Ok(_t2);
        let _t3 = mq.toCompactValue(qsign)?;
        let mut compactVal: i64 = _t3;
        /* TODO: lcmp  */
        let _t4: Object = BigDecimal::createAndStripZerosToMatchScale(compactVal, scale, (local_5 as i64))?;
        return Ok(_t4);
        let _t5 = mq.toBigInteger(qsign)?;
        let mut intVal: Object = _t5;
        let _t6: Object = BigDecimal::createAndStripZerosToMatchScale(intVal, scale, (local_5 as i64))?;
        return Ok(_t6);
        let _t7 = mq.toBigDecimal(qsign, scale)?;
        Ok(_t7)
    }

    #[cfg_attr(any(), java_method(name = "needIncrement", descriptor = "(JIILjava/math/MutableBigInteger;J)Z", access = "private static"))]
    // java: needIncrement(JIILjava/math/MutableBigInteger;J)Z
    pub fn needIncrement__l_i_i_mutabl_l(ldivisor: i64, arg_1: i32, roundingMode: i32, qsign: Object, mq: i64) -> Result<bool> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut cmpFracHalf: i32 = 1i32;
        let _t0: i32 = BigDecimal::longCompareMagnitude((2i64).wrapping_mul(local_5), ldivisor)?;
        cmpFracHalf = _t0;
        let _t1 = mq.isOdd()?;
        let _t2: bool = BigDecimal::commonNeedIncrement(roundingMode, qsign, cmpFracHalf, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigInteger;", access = "private static"))]
    // java: divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigInteger;
    pub fn divideAndRound__bigint_bigint_i(bdividend: Object, bdivisor: Object, roundingMode: i32) -> Result<Object> {
        let mut mdividend: MutableBigInteger = MutableBigInteger::new(bdividend.mag.get())?;
        let mut mq: MutableBigInteger = MutableBigInteger::new()?;
        let mut mdivisor: MutableBigInteger = MutableBigInteger::new(bdivisor.mag.get())?;
        let _t0 = mdividend.divide(mdivisor, mq)?;
        let mut mr: Object = _t0;
        let _t1 = mr.isZero()?;
        let mut isRemainderZero: i32 = _t1;
        let mut qsign: i32 = bdividend.signum.get() == bdivisor.signum.get();
        let _t2: bool = BigDecimal::needIncrement(mdivisor, roundingMode, qsign, mq, mr)?;
        mq.add(MutableBigInteger::ONE())?;
        let _t3 = mq.toBigInteger(qsign)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divideAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;III)Ljava/math/BigDecimal;
    pub fn divideAndRound__bigint_bigint_i_i_i(bdividend: Object, bdivisor: Object, scale: i32, roundingMode: i32, preferredScale: i32) -> Result<Object> {
        let mut mdividend: MutableBigInteger = MutableBigInteger::new(bdividend.mag.get())?;
        let mut mq: MutableBigInteger = MutableBigInteger::new()?;
        let mut mdivisor: MutableBigInteger = MutableBigInteger::new(bdivisor.mag.get())?;
        let _t0 = mdividend.divide(mdivisor, mq)?;
        let mut mr: Object = _t0;
        let _t1 = mr.isZero()?;
        let mut isRemainderZero: i32 = _t1;
        let mut qsign: i32 = bdividend.signum.get() == bdivisor.signum.get();
        let _t2: bool = BigDecimal::needIncrement(mdivisor, roundingMode, qsign, mq, mr)?;
        mq.add(MutableBigInteger::ONE())?;
        let _t3 = mq.toBigDecimal(qsign, scale)?;
        return Ok(_t3);
        let _t4 = mq.toCompactValue(qsign)?;
        let mut compactVal: i64 = _t4;
        /* TODO: lcmp  */
        let _t5: Object = BigDecimal::createAndStripZerosToMatchScale(compactVal, scale, (preferredScale as i64))?;
        return Ok(_t5);
        let _t6 = mq.toBigInteger(qsign)?;
        let mut intVal: Object = _t6;
        let _t7: Object = BigDecimal::createAndStripZerosToMatchScale(intVal, scale, (preferredScale as i64))?;
        return Ok(_t7);
        let _t8 = mq.toBigDecimal(qsign, scale)?;
        Ok(_t8)
    }

    #[cfg_attr(any(), java_method(name = "needIncrement", descriptor = "(Ljava/math/MutableBigInteger;IILjava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Z", access = "private static"))]
    // java: needIncrement(Ljava/math/MutableBigInteger;IILjava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Z
    pub fn needIncrement__mutabl_i_i_mutabl_mutabl(mdivisor: Object, roundingMode: i32, qsign: i32, mq: Object, mr: Object) -> Result<bool> {
        let _t0 = mr.isZero()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = mr.compareHalf(mdivisor)?;
        let mut cmpFracHalf: i32 = _t1;
        let _t2 = mq.isOdd()?;
        let _t3: bool = BigDecimal::commonNeedIncrement(roundingMode, qsign, cmpFracHalf, _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "createAndStripZerosToMatchScale", descriptor = "(Ljava/math/BigInteger;IJ)Ljava/math/BigDecimal;", access = "private static"))]
    // java: createAndStripZerosToMatchScale(Ljava/math/BigInteger;IJ)Ljava/math/BigDecimal;
    pub fn createAndStripZerosToMatchScale__bigint_i_l(intVal: Object, scale: i32, preferredScale: i64) -> Result<Object> {
        loop {
            let _t0 = intVal.compareMagnitude(BigInteger::TEN())?;
            if _t0<0i32 { break; }
            /* TODO: lcmp  */
            let _t0 = intVal.testBit(0i32)?;
            let _t1 = intVal.divideAndRemainder(BigInteger::TEN())?;
            let mut qr: Vec<Object> = _t1;
            let _t2 = qr[1i32 as usize].clone().signum()?;
            intVal = qr[0i32 as usize].clone();
            let _t3: i32 = BigDecimal::checkScale(intVal, ((scale as i64)).wrapping_sub(1i64))?;
            scale = _t3;
        }
        let _t0: Object = BigDecimal::valueOf(intVal, scale, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "createAndStripZerosToMatchScale", descriptor = "(JIJ)Ljava/math/BigDecimal;", access = "private static"))]
    // java: createAndStripZerosToMatchScale(JIJ)Ljava/math/BigDecimal;
    pub fn createAndStripZerosToMatchScale__l_i_l(compactVal: i64, arg_1: i32, scale: i64) -> Result<Object> {
        loop {
            let _t0: i64 = (compactVal).abs();
            /* TODO: lcmp  */
            if 10i64<0i32 { break; }
            /* TODO: lcmp  */
            /* TODO: land  */
            /* TODO: lcmp  */
            /* TODO: lrem  */
            let mut r: i64 = 10i64;
            /* TODO: lcmp  */
            compactVal = (compactVal/10i64);
            let _t0: i32 = BigDecimal::checkScale(compactVal, ((scale as i64)).wrapping_sub(1i64))?;
            scale = _t0;
        }
        let _t0: Object = BigDecimal::valueOf(compactVal, scale)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stripZerosToMatchScale", descriptor = "(Ljava/math/BigInteger;JII)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn stripZerosToMatchScale(intVal: Object, intCompact: i64, arg_2: i32, scale: i32) -> Result<Object> {
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::createAndStripZerosToMatchScale(intCompact, scale, (local_4 as i64))?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::createAndStripZerosToMatchScale(intVal, scale, (local_4 as i64))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(JJ)J", access = "private static"))]
    // java: add(JJ)J
    pub fn add__l_l(xs: i64, arg_1: i64) -> Result<i64> {
        let mut sum: i64 = (xs).wrapping_add(local_2);
        /* TODO: lxor  */
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok(sum);
        Ok(9223372036854775808i64)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(JJI)Ljava/math/BigDecimal;", access = "private static"))]
    // java: add(JJI)Ljava/math/BigDecimal;
    pub fn add__l_l_i(xs: i64, arg_1: i64, ys: i32) -> Result<Object> {
        let _t0: i64 = BigDecimal::add(xs, ys)?;
        let mut sum: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::valueOf(sum, local_4)?;
        return Ok(_t1);
        let _t2 = xs.add(ys)?;
        Ok(BigDecimal::new(_t2, local_4)?)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(JIJI)Ljava/math/BigDecimal;", access = "private static"))]
    // java: add(JIJI)Ljava/math/BigDecimal;
    pub fn add__l_i_l_i(xs: i64, arg_1: i32, scale1: i64, ys: i32) -> Result<Object> {
        let mut sdiff: i64 = ((scale1 as i64)).wrapping_sub((local_5 as i64));
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::add(xs, ys, scale1)?;
        return Ok(_t0);
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t1: i32 = BigDecimal::checkScale(xs, sdiff)?;
        let mut raise: i32 = _t1;
        let _t2: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        let mut scaledX: i64 = _t2;
        /* TODO: lcmp  */
        let _t3: Object = BigDecimal::add(scaledX, ys, local_5)?;
        return Ok(_t3);
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let _t5 = _t4.add(ys)?;
        let mut bigsum: Object = _t5;
        /* TODO: lxor  */
        /* TODO: lcmp  */
        let _t6: Object = BigDecimal::valueOf(bigsum, local_5, 0i32)?;
        return Ok(_t6);
        let _t7: i32 = BigDecimal::checkScale(ys, sdiff)?;
        raise = _t7;
        let _t8: i64 = BigDecimal::longMultiplyPowerTen(ys, raise)?;
        scaledX = _t8;
        /* TODO: lcmp  */
        let _t9: Object = BigDecimal::add(xs, scaledX, scale1)?;
        return Ok(_t9);
        let _t10: Object = BigDecimal::bigMultiplyPowerTen(ys, raise)?;
        let _t11 = _t10.add(xs)?;
        bigsum = _t11;
        /* TODO: lxor  */
        /* TODO: lcmp  */
        let _t12: Object = BigDecimal::valueOf(bigsum, scale1, 0i32)?;
        Ok(_t12)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(JILjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private static"))]
    // java: add(JILjava/math/BigInteger;I)Ljava/math/BigDecimal;
    pub fn add__l_i_bigint_i(xs: i64, arg_1: i32, scale1: Object, snd: i32) -> Result<Object> {
        let mut rscale: i32 = scale1;
        let mut sdiff: i64 = ((rscale as i64)).wrapping_sub((local_4 as i64));
        let _t0: i32 = Long::signum(xs)?;
        let mut sameSigns: i32 = _t0 == snd.signum.get();
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t1: i32 = BigDecimal::checkScale(xs, sdiff)?;
        let mut raise: i32 = _t1;
        rscale = local_4;
        let _t2: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        let mut scaledX: i64 = _t2;
        /* TODO: lcmp  */
        let _t3: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let _t4 = snd.add(_t3)?;
        let mut sum: Object = _t4;
        let _t5 = snd.add(scaledX)?;
        sum = _t5;
        let _t6: i32 = BigDecimal::checkScale(snd, sdiff)?;
        raise = _t6;
        let _t7: Object = BigDecimal::bigMultiplyPowerTen(snd, raise)?;
        snd = _t7;
        let _t8 = snd.add(xs)?;
        sum = _t8;
        let _t9: Object = BigDecimal::valueOf(sum, rscale, 0i32)?;
        Ok(_t9)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private static"))]
    // java: add(Ljava/math/BigInteger;ILjava/math/BigInteger;I)Ljava/math/BigDecimal;
    pub fn add__bigint_i_bigint_i(fst: Object, scale1: i32, snd: Object, scale2: i32) -> Result<Object> {
        let mut rscale: i32 = scale1;
        let mut sdiff: i64 = ((rscale as i64)).wrapping_sub((scale2 as i64));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lneg  */
        let _t0: i32 = BigDecimal::checkScale(fst, sdiff)?;
        let mut raise: i32 = _t0;
        rscale = scale2;
        let _t1: Object = BigDecimal::bigMultiplyPowerTen(fst, raise)?;
        fst = _t1;
        let _t2: i32 = BigDecimal::checkScale(snd, sdiff)?;
        raise = _t2;
        let _t3: Object = BigDecimal::bigMultiplyPowerTen(snd, raise)?;
        snd = _t3;
        let _t4 = fst.add(snd)?;
        raise = _t4;
        let _t5: Object = BigDecimal::valueOf(raise, rscale, 0i32)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "bigMultiplyPowerTen", descriptor = "(JI)Ljava/math/BigInteger;", access = "private static"))]
    // java: bigMultiplyPowerTen(JI)Ljava/math/BigInteger;
    pub fn bigMultiplyPowerTen__l_i(value: i64, arg_1: i32) -> Result<Object> {
        return Ok(value);
        let _t0: Object = BigDecimal::bigTenToThe(local_2)?;
        let _t1 = _t0.multiply(value)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "bigMultiplyPowerTen", descriptor = "(Ljava/math/BigInteger;I)Ljava/math/BigInteger;", access = "private static"))]
    // java: bigMultiplyPowerTen(Ljava/math/BigInteger;I)Ljava/math/BigInteger;
    pub fn bigMultiplyPowerTen__bigint_i(value: Object, n: i32) -> Result<Object> {
        return Ok(value);
        let _t0 = value.multiply(BigDecimal::LONG_TEN_POWERS_TABLE()[n as usize])?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::bigTenToThe(n)?;
        let _t2 = value.multiply(_t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "divideSmallFastPath", descriptor = "(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn divideSmallFastPath(xs: i64, arg_1: i32, xscale: i64, ys: i32, arg_4: i64, yscale: Object) -> Result<Object> {
        let mut mcp: i32 = local_8.precision.get();
        let mut roundingMode: i32 = local_8.roundingMode.get().oldMode.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let mut xraise: i32 = (yscale).wrapping_sub(xscale);
        let _t0: i64 = BigDecimal::longMultiplyPowerTen(xs, xraise)?;
        let mut scaledX: i64 = _t0;
        let _t1: i32 = BigDecimal::longCompareMagnitude(scaledX, ys)?;
        let mut cmp: i32 = _t1;
        yscale = yscale.wrapping_sub(1i32);
        let _t2: i32 = BigDecimal::checkScaleNonZero((((local_6).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        let mut scl: i32 = _t2;
        let _t3: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let _t4: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let mut raise: i32 = _t4;
        let _t5: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        /* TODO: dup2  */
        let mut scaledXs: i64 = _t5;
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        let mut quotient: i64 = 9223372036854775808i64;
        let _t6: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t7: Object = BigDecimal::multiplyDivideAndRound(BigDecimal::LONG_TEN_POWERS_TABLE()[(mcp).wrapping_sub(1i32) as usize], scaledX, ys, scl, roundingMode, _t6)?;
        quotient = _t7;
        let _t8: Object = BigDecimal::bigMultiplyPowerTen(scaledX, (mcp).wrapping_sub(1i32))?;
        let mut rb: Object = _t8;
        let _t9: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t10: Object = BigDecimal::divideAndRound(rb, ys, scl, roundingMode, _t9)?;
        quotient = _t10;
        let _t11: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t12: Object = BigDecimal::divideAndRound(scaledXs, ys, scl, roundingMode, _t11)?;
        quotient = _t12;
        let _t13: i32 = BigDecimal::checkScaleNonZero(((xscale as i64)).wrapping_sub((mcp as i64)))?;
        raise = _t13;
        let _t14: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t15: Object = BigDecimal::divideAndRound(xs, ys, scl, roundingMode, _t14)?;
        quotient = _t15;
        let _t16: i32 = BigDecimal::checkScaleNonZero(((raise as i64)).wrapping_sub((yscale as i64)))?;
        scaledXs = _t16;
        let _t17: i64 = BigDecimal::longMultiplyPowerTen(ys, scaledXs)?;
        /* TODO: dup2  */
        let mut scaledYs: i64 = _t17;
        /* TODO: lcmp  */
        let _t18: Object = BigDecimal::bigMultiplyPowerTen(ys, scaledXs)?;
        let mut rb: Object = _t18;
        let _t19: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t20: Object = BigDecimal::divideAndRound(xs, rb, scl, roundingMode, _t19)?;
        quotient = _t20;
        let _t21: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t22: Object = BigDecimal::divideAndRound(xs, scaledYs, scl, roundingMode, _t21)?;
        quotient = _t22;
        let _t23: i32 = BigDecimal::checkScaleNonZero((((local_6).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        scl = _t23;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t24: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t25: Object = BigDecimal::roundedTenPower(ys != 0i64<0i32, mcp, scl, _t24)?;
        quotient = _t25;
        let _t26: i64 = BigDecimal::longMultiplyPowerTen(scaledX, mcp)?;
        /* TODO: dup2  */
        raise = _t26;
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        quotient = 9223372036854775808i64;
        let _t27: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t28: Object = BigDecimal::multiplyDivideAndRound(BigDecimal::LONG_TEN_POWERS_TABLE()[mcp as usize], scaledX, ys, scl, roundingMode, _t27)?;
        quotient = _t28;
        let _t29: Object = BigDecimal::bigMultiplyPowerTen(scaledX, mcp)?;
        scaledYs = _t29;
        let _t30: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t31: Object = BigDecimal::divideAndRound(scaledYs, ys, scl, roundingMode, _t30)?;
        quotient = _t31;
        let _t32: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t33: Object = BigDecimal::divideAndRound(raise, ys, scl, roundingMode, _t32)?;
        quotient = _t33;
        let _t34: Object = BigDecimal::doRound(quotient, local_8)?;
        Ok(_t34)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(JIJIJLjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divide__l_i_l_i_l_mathco(xs: i64, arg_1: i32, xscale: i64, ys: i32, arg_4: i64, yscale: Object) -> Result<Object> {
        let mut mcp: i32 = local_8.precision.get();
        let _t0: Object = BigDecimal::divideSmallFastPath(xs, xscale, ys, yscale, local_6, local_8)?;
        return Ok(_t0);
        let _t1: i32 = BigDecimal::compareMagnitudeNormalized(xs, xscale, ys, yscale)?;
        yscale = yscale.wrapping_sub(1i32);
        let mut roundingMode: i32 = local_8.roundingMode.get().oldMode.get();
        let _t2: i32 = BigDecimal::checkScaleNonZero((((local_6).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        let mut scl: i32 = _t2;
        let _t3: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let _t4: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let mut raise: i32 = _t4;
        let _t5: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        /* TODO: dup2  */
        let mut scaledXs: i64 = _t5;
        /* TODO: lcmp  */
        let _t6: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let mut rb: Object = _t6;
        let _t7: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t8: Object = BigDecimal::divideAndRound(rb, ys, scl, roundingMode, _t7)?;
        let mut quotient: Object = _t8;
        let _t9: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t10: Object = BigDecimal::divideAndRound(scaledXs, ys, scl, roundingMode, _t9)?;
        quotient = _t10;
        let _t11: i32 = BigDecimal::checkScaleNonZero(((xscale as i64)).wrapping_sub((mcp as i64)))?;
        raise = _t11;
        let _t12: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t13: Object = BigDecimal::divideAndRound(xs, ys, scl, roundingMode, _t12)?;
        quotient = _t13;
        let _t14: i32 = BigDecimal::checkScaleNonZero(((raise as i64)).wrapping_sub((yscale as i64)))?;
        scaledXs = _t14;
        let _t15: i64 = BigDecimal::longMultiplyPowerTen(ys, scaledXs)?;
        /* TODO: dup2  */
        let mut scaledYs: i64 = _t15;
        /* TODO: lcmp  */
        let _t16: Object = BigDecimal::bigMultiplyPowerTen(ys, scaledXs)?;
        let mut rb: Object = _t16;
        let _t17: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t18: Object = BigDecimal::divideAndRound(xs, rb, scl, roundingMode, _t17)?;
        quotient = _t18;
        let _t19: i32 = BigDecimal::checkScaleNonZero(local_6)?;
        let _t20: Object = BigDecimal::divideAndRound(xs, scaledYs, scl, roundingMode, _t19)?;
        quotient = _t20;
        let _t21: Object = BigDecimal::doRound(quotient, local_8)?;
        Ok(_t21)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;IJIJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(Ljava/math/BigInteger;IJIJLjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divide__bigint_i_l_i_l_mathco(xs: Object, xscale: i32, ys: i64, arg_3: i32, yscale: i64, preferredScale: Object) -> Result<Object> {
        let _t0: i32 = BigDecimal::compareMagnitudeNormalized(ys, yscale, xs, xscale)?;
        yscale = yscale.wrapping_sub(1i32);
        let mut mcp: i32 = local_7.precision.get();
        let mut roundingMode: i32 = local_7.roundingMode.get().oldMode.get();
        let _t1: i32 = BigDecimal::checkScaleNonZero((((preferredScale).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        let mut scl: i32 = _t1;
        let _t2: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let _t3: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let mut raise: i32 = _t3;
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let mut rb: Object = _t4;
        let _t5: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t6: Object = BigDecimal::divideAndRound(rb, ys, scl, roundingMode, _t5)?;
        let mut quotient: Object = _t6;
        let _t7: i32 = BigDecimal::checkScaleNonZero(((xscale as i64)).wrapping_sub((mcp as i64)))?;
        raise = _t7;
        let _t8: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t9: Object = BigDecimal::divideAndRound(xs, ys, scl, roundingMode, _t8)?;
        quotient = _t9;
        let _t10: i32 = BigDecimal::checkScaleNonZero(((raise as i64)).wrapping_sub((yscale as i64)))?;
        rb = _t10;
        let _t11: i64 = BigDecimal::longMultiplyPowerTen(ys, rb)?;
        /* TODO: dup2  */
        let mut scaledYs: i64 = _t11;
        /* TODO: lcmp  */
        let _t12: Object = BigDecimal::bigMultiplyPowerTen(ys, rb)?;
        let mut rb: Object = _t12;
        let _t13: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t14: Object = BigDecimal::divideAndRound(xs, rb, scl, roundingMode, _t13)?;
        quotient = _t14;
        let _t15: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t16: Object = BigDecimal::divideAndRound(xs, scaledYs, scl, roundingMode, _t15)?;
        quotient = _t16;
        let _t17: Object = BigDecimal::doRound(quotient, local_7)?;
        Ok(_t17)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(JILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(JILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divide__l_i_bigint_i_l_mathco(xs: i64, arg_1: i32, xscale: Object, ys: i32, yscale: i64, preferredScale: Object) -> Result<Object> {
        let _t0: i32 = BigDecimal::compareMagnitudeNormalized(xs, xscale, ys, yscale)?;
        yscale = yscale.wrapping_sub(1i32);
        let mut mcp: i32 = local_7.precision.get();
        let mut roundingMode: i32 = local_7.roundingMode.get().oldMode.get();
        let _t1: i32 = BigDecimal::checkScaleNonZero((((preferredScale).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        let mut scl: i32 = _t1;
        let _t2: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let _t3: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let mut raise: i32 = _t3;
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let mut rb: Object = _t4;
        let _t5: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t6: Object = BigDecimal::divideAndRound(rb, ys, scl, roundingMode, _t5)?;
        let mut quotient: Object = _t6;
        let _t7: i32 = BigDecimal::checkScaleNonZero(((xscale as i64)).wrapping_sub((mcp as i64)))?;
        raise = _t7;
        let _t8: i32 = BigDecimal::checkScaleNonZero(((raise as i64)).wrapping_sub((yscale as i64)))?;
        rb = _t8;
        let _t9: Object = BigDecimal::bigMultiplyPowerTen(ys, rb)?;
        let mut rb: Object = _t9;
        let _t10: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t11: Object = BigDecimal::divideAndRound(xs, rb, scl, roundingMode, _t10)?;
        quotient = _t11;
        let _t12: Object = BigDecimal::doRound(quotient, local_7)?;
        Ok(_t12)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(Ljava/math/BigInteger;ILjava/math/BigInteger;IJLjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn divide__bigint_i_bigint_i_l_mathco(xs: Object, xscale: i32, ys: Object, yscale: i32, preferredScale: i64, arg_5: Object) -> Result<Object> {
        let _t0: i32 = BigDecimal::compareMagnitudeNormalized(xs, xscale, ys, yscale)?;
        yscale = yscale.wrapping_sub(1i32);
        let mut mcp: i32 = local_6.precision.get();
        let mut roundingMode: i32 = local_6.roundingMode.get().oldMode.get();
        let _t1: i32 = BigDecimal::checkScaleNonZero((((preferredScale).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64))).wrapping_add((mcp as i64)))?;
        let mut scl: i32 = _t1;
        let _t2: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let _t3: i32 = BigDecimal::checkScaleNonZero((((mcp as i64)).wrapping_add((yscale as i64))).wrapping_sub((xscale as i64)))?;
        let mut raise: i32 = _t3;
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(xs, raise)?;
        let mut rb: Object = _t4;
        let _t5: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t6: Object = BigDecimal::divideAndRound(rb, ys, scl, roundingMode, _t5)?;
        let mut quotient: Object = _t6;
        let _t7: i32 = BigDecimal::checkScaleNonZero(((xscale as i64)).wrapping_sub((mcp as i64)))?;
        raise = _t7;
        let _t8: i32 = BigDecimal::checkScaleNonZero(((raise as i64)).wrapping_sub((yscale as i64)))?;
        rb = _t8;
        let _t9: Object = BigDecimal::bigMultiplyPowerTen(ys, rb)?;
        let mut rb: Object = _t9;
        let _t10: i32 = BigDecimal::checkScaleNonZero(preferredScale)?;
        let _t11: Object = BigDecimal::divideAndRound(xs, rb, scl, roundingMode, _t10)?;
        quotient = _t11;
        let _t12: Object = BigDecimal::doRound(quotient, local_6)?;
        Ok(_t12)
    }

    #[cfg_attr(any(), java_method(name = "multiplyDivideAndRound", descriptor = "(JJJIII)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn multiplyDivideAndRound(dividend0: i64, arg_1: i64, dividend1: i64, arg_3: i32, divisor: i32, arg_5: i32) -> Result<Object> {
        let _t0: i32 = Long::signum(dividend0)?;
        let _t1: i32 = Long::signum(dividend1)?;
        let _t2: i32 = Long::signum(divisor)?;
        let mut qsign: i32 = ((_t0).wrapping_mul(_t1)).wrapping_mul(_t2);
        let _t3: i64 = (dividend0).abs();
        dividend0 = _t3;
        let _t4: i64 = (dividend1).abs();
        dividend1 = _t4;
        let _t5: i64 = (divisor).abs();
        divisor = _t5;
        /* TODO: lushr  */
        let mut d0_hi: i64 = 32i32;
        /* TODO: land  */
        let mut d0_lo: i64 = 4294967295i64;
        /* TODO: lushr  */
        let mut d1_hi: i64 = 32i32;
        /* TODO: land  */
        let mut d1_lo: i64 = 4294967295i64;
        let mut product: i64 = (d0_lo).wrapping_mul(d1_lo);
        /* TODO: land  */
        let mut d0: i64 = 4294967295i64;
        /* TODO: lushr  */
        let mut d1: i64 = 32i32;
        product = ((d0_hi).wrapping_mul(d1_lo)).wrapping_add(d1);
        /* TODO: land  */
        d1 = 4294967295i64;
        /* TODO: lushr  */
        let mut d2: i64 = 32i32;
        product = ((d0_lo).wrapping_mul(d1_hi)).wrapping_add(d1);
        /* TODO: land  */
        d1 = 4294967295i64;
        /* TODO: lushr  */
        d2 = (product).wrapping_add(32i32);
        /* TODO: lushr  */
        let mut d3: i64 = 32i32;
        /* TODO: land  */
        d2 = 4294967295i64;
        product = ((d0_hi).wrapping_mul(d1_hi)).wrapping_add(d2);
        /* TODO: land  */
        d2 = 4294967295i64;
        /* TODO: lushr  */
        /* TODO: land  */
        d3 = 4294967295i64;
        let _t6: i64 = BigDecimal::make64(d3, d2)?;
        let mut dividendHi: i64 = _t6;
        let _t7: i64 = BigDecimal::make64(d1, d0)?;
        let mut dividendLo: i64 = _t7;
        let _t8: Object = BigDecimal::divideAndRound128(dividendHi, dividendLo, divisor, qsign, local_6, local_7, local_8)?;
        Ok(_t8)
    }

    #[cfg_attr(any(), java_method(name = "divideAndRound128", descriptor = "(JJJIIII)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn divideAndRound128(dividendHi: i64, arg_1: i64, dividendLo: i64, arg_3: i32, divisor: i32, arg_5: i32, sign: i32) -> Result<Object> {
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        return Ok(divisor);
        let _t0: i32 = Long::numberOfLeadingZeros(divisor)?;
        let mut shift: i32 = _t0;
        /* TODO: lshl  */
        divisor = shift;
        /* TODO: lushr  */
        let mut v1: i64 = 32i32;
        /* TODO: land  */
        let mut v0: i64 = 4294967295i64;
        /* TODO: lshl  */
        let mut tmp: i64 = shift;
        /* TODO: lushr  */
        let mut u1: i64 = 32i32;
        /* TODO: land  */
        let mut u0: i64 = 4294967295i64;
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: lor  */
        tmp = (64i32).wrapping_sub(shift);
        /* TODO: land  */
        let mut u2: i64 = 4294967295i64;
        /* TODO: lcmp  */
        let mut q1: i64 = tmp;
        let mut r_tmp: i64 = 0i64;
        /* TODO: lcmp  */
        q1 = (tmp/v1);
        r_tmp = (tmp).wrapping_sub((q1).wrapping_mul(v1));
        let _t1: Vec<i64> = BigDecimal::divRemNegativeLong(tmp, v1)?;
        let mut rq: Vec<i64> = _t1;
        q1 = rq[1i32 as usize];
        r_tmp = rq[0i32 as usize];
        /* TODO: lcmp  */
        let _t2: i64 = BigDecimal::make64(r_tmp, u1)?;
        let _t3: bool = BigDecimal::unsignedLongCompare((q1).wrapping_mul(v0), _t2)?;
        q1 = (q1).wrapping_sub(1i64);
        r_tmp = (r_tmp).wrapping_add(v1);
        /* TODO: lcmp  */
        let _t4: i64 = BigDecimal::mulsub(u2, u1, v1, v0, q1)?;
        tmp = _t4;
        /* TODO: land  */
        u1 = 4294967295i64;
        /* TODO: lcmp  */
        rq = tmp;
        r_tmp = 0i64;
        /* TODO: lcmp  */
        rq = (tmp/v1);
        r_tmp = (tmp).wrapping_sub((rq).wrapping_mul(v1));
        let _t5: Vec<i64> = BigDecimal::divRemNegativeLong(tmp, v1)?;
        let mut rq: Vec<i64> = _t5;
        rq = rq[1i32 as usize];
        r_tmp = rq[0i32 as usize];
        /* TODO: lcmp  */
        let _t6: i64 = BigDecimal::make64(r_tmp, u0)?;
        let _t7: bool = BigDecimal::unsignedLongCompare((rq).wrapping_mul(v0), _t6)?;
        rq = (rq).wrapping_sub(1i64);
        r_tmp = (r_tmp).wrapping_add(v1);
        /* TODO: lcmp  */
        let mut _arr8: Vec<i32> = vec![0i32; 2i32 as usize];
        _arr8[0i32 as usize] = (q1 as i32);
        _arr8[1i32 as usize] = (rq as i32);
        rq = MutableBigInteger::new(_arr8)?;
        let _t9 = rq.toBigDecimal(sign, local_7)?;
        return Ok(_t9);
        let _t10: i64 = BigDecimal::mulsub(u1, u0, v1, v0, rq)?;
        /* TODO: lushr  */
        let mut r: i64 = shift;
        /* TODO: lcmp  */
        /* TODO: lushr  */
        let _t11: bool = BigDecimal::needIncrement(shift, local_8, sign, &rq, r)?;
        rq.add(MutableBigInteger::ONE())?;
        let _t12 = rq.toBigDecimal(sign, local_7)?;
        return Ok(_t12);
        let _t13 = rq.toBigInteger(sign)?;
        let mut intVal: Object = _t13;
        let _t14: Object = BigDecimal::createAndStripZerosToMatchScale(intVal, local_7, (local_9 as i64))?;
        return Ok(_t14);
        let _t15 = rq.toBigDecimal(sign, local_7)?;
        return Ok(_t15);
        let _t16: i64 = BigDecimal::make64(q1, rq)?;
        rq = _t16;
        rq = (rq).wrapping_mul((sign as i64));
        let _t17: Object = BigDecimal::valueOf(rq, local_7)?;
        return Ok(_t17);
        let _t18: i64 = BigDecimal::mulsub(u1, u0, v1, v0, rq)?;
        /* TODO: lushr  */
        let mut r: i64 = shift;
        /* TODO: lcmp  */
        /* TODO: lushr  */
        let _t19: bool = BigDecimal::needIncrement(shift, local_8, sign, rq, r)?;
        let mut increment: i32 = _t19;
        let _t20: Object = BigDecimal::valueOf(rq, local_7)?;
        return Ok(_t20);
        let _t21: Object = BigDecimal::createAndStripZerosToMatchScale(rq, local_7, (local_9 as i64))?;
        return Ok(_t21);
        let _t22: Object = BigDecimal::valueOf(rq, local_7)?;
        Ok(_t22)
    }

    #[cfg_attr(any(), java_method(name = "roundedTenPower", descriptor = "(IIII)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn roundedTenPower(qsign: i32, raise: i32, scale: i32, preferredScale: i32) -> Result<Object> {
        let mut diff: i32 = (scale).wrapping_sub(preferredScale);
        let _t0: Object = BigDecimal::scaledTenPow((raise).wrapping_sub(diff), qsign, preferredScale)?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::valueOf((qsign as i64), (scale).wrapping_sub(raise))?;
        return Ok(_t1);
        let _t2: Object = BigDecimal::scaledTenPow(raise, qsign, scale)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "scaledTenPow", descriptor = "(III)Ljava/math/BigDecimal;", access = "static"))]
    pub fn scaledTenPow(n: i32, sign: i32, scale: i32) -> Result<Object> {
        let _t0: Object = BigDecimal::valueOf(((sign as i64)).wrapping_mul(BigDecimal::LONG_TEN_POWERS_TABLE()[n as usize]), scale)?;
        return Ok(_t0);
        let _t1: Object = BigDecimal::bigTenToThe(n)?;
        let mut unscaledVal: Object = _t1;
        let _t2 = unscaledVal.negate()?;
        unscaledVal = _t2;
        Ok(BigDecimal::new(unscaledVal, 9223372036854775808i64, scale, (n).wrapping_add(1i32))?)
    }

    #[cfg_attr(any(), java_method(name = "divRemNegativeLong", descriptor = "(JJ)[J", access = "private static"))]
    pub fn divRemNegativeLong(n: i64, arg_1: i64) -> Result<Vec<i64>> {
        /* TODO: lcmp  */
        String::new().append(&String::from("Non-negative numerator"))?;
        String::new().append(&n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lushr  */
        /* TODO: lushr  */
        let mut q: i64 = (local_2/1i32);
        let mut r: i64 = (n).wrapping_sub((q).wrapping_mul(local_2));
        loop {
            /* TODO: lcmp  */
            if 0i64>=0i32 { break; }
            r = (r).wrapping_add(local_2);
            q = (q).wrapping_sub(1i64);
        }
        loop {
            /* TODO: lcmp  */
            if local_2<0i32 { break; }
            r = (r).wrapping_sub(local_2);
            q = (q).wrapping_add(1i64);
        }
        let mut _arr0: Vec<i64> = vec![0i64; 2i32 as usize];
        _arr0[0i32 as usize] = r;
        _arr0[1i32 as usize] = q;
        Ok(_arr0)
    }

    #[cfg_attr(any(), java_method(name = "make64", descriptor = "(JJ)J", access = "private static"))]
    pub fn make64(hi: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lshl  */
        /* TODO: lor  */
        Ok(local_2)
    }

    #[cfg_attr(any(), java_method(name = "mulsub", descriptor = "(JJJJJ)J", access = "private static"))]
    pub fn mulsub(u1: i64, arg_1: i64, u0: i64, arg_3: i64, v1: i64) -> Result<i64> {
        let mut tmp: i64 = (u0).wrapping_sub((local_8).wrapping_mul(local_6));
        /* TODO: lushr  */
        /* TODO: land  */
        let _t0: i64 = BigDecimal::make64(tmp, 4294967295i64)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "unsignedLongCompare", descriptor = "(JJ)Z", access = "private static"))]
    pub fn unsignedLongCompare(one: i64, arg_1: i64) -> Result<bool> {
        /* TODO: lcmp  */
        Ok((local_2).wrapping_add(9223372036854775808i64)>0i32)
    }

    #[cfg_attr(any(), java_method(name = "unsignedLongCompareEq", descriptor = "(JJ)Z", access = "private static"))]
    pub fn unsignedLongCompareEq(one: i64, arg_1: i64) -> Result<bool> {
        /* TODO: lcmp  */
        Ok((local_2).wrapping_add(9223372036854775808i64)>=0i32)
    }

    #[cfg_attr(any(), java_method(name = "compareMagnitudeNormalized", descriptor = "(JIJI)I", access = "private static"))]
    // java: compareMagnitudeNormalized(JIJI)I
    pub fn compareMagnitudeNormalized__l_i_l_i(xs: i64, arg_1: i32, xscale: i64, ys: i32) -> Result<i32> {
        let mut sdiff: i32 = (xscale).wrapping_sub(local_5);
        let _t0: i64 = BigDecimal::longMultiplyPowerTen(xs, (sdiff).wrapping_neg())?;
        xs = _t0;
        let _t1: i64 = BigDecimal::longMultiplyPowerTen(ys, sdiff)?;
        ys = _t1;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t2: i32 = BigDecimal::longCompareMagnitude(xs, ys)?;
        return Ok(-1i32);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "compareMagnitudeNormalized", descriptor = "(JILjava/math/BigInteger;I)I", access = "private static"))]
    // java: compareMagnitudeNormalized(JILjava/math/BigInteger;I)I
    pub fn compareMagnitudeNormalized__l_i_bigint_i(xs: i64, arg_1: i32, xscale: Object, ys: i32) -> Result<i32> {
        /* TODO: lcmp  */
        return Ok(-1i32);
        let mut sdiff: i32 = (xscale).wrapping_sub(local_4);
        let _t0: i64 = BigDecimal::longMultiplyPowerTen(xs, (sdiff).wrapping_neg())?;
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::bigMultiplyPowerTen(xs, (sdiff).wrapping_neg())?;
        let _t2 = _t1.compareMagnitude(ys)?;
        return Ok(_t2);
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "compareMagnitudeNormalized", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;I)I", access = "private static"))]
    // java: compareMagnitudeNormalized(Ljava/math/BigInteger;ILjava/math/BigInteger;I)I
    pub fn compareMagnitudeNormalized__bigint_i_bigint_i(xs: Object, xscale: i32, ys: Object, yscale: i32) -> Result<i32> {
        let mut sdiff: i32 = (xscale).wrapping_sub(yscale);
        let _t0: Object = BigDecimal::bigMultiplyPowerTen(xs, (sdiff).wrapping_neg())?;
        let _t1 = _t0.compareMagnitude(ys)?;
        return Ok(_t1);
        let _t2: Object = BigDecimal::bigMultiplyPowerTen(ys, sdiff)?;
        let _t3 = xs.compareMagnitude(_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(JJ)J", access = "private static"))]
    // java: multiply(JJ)J
    pub fn multiply__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut product: i64 = (x).wrapping_mul(local_2);
        let _t0: i64 = (x).abs();
        let mut ax: i64 = _t0;
        let _t1: i64 = (local_2).abs();
        let mut ay: i64 = _t1;
        /* TODO: lor  */
        /* TODO: lushr  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(product);
        Ok(9223372036854775808i64)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(JJI)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiply(JJI)Ljava/math/BigDecimal;
    pub fn multiply__l_l_i(x: i64, arg_1: i64, y: i32) -> Result<Object> {
        let _t0: i64 = BigDecimal::multiply(x, y)?;
        let mut product: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::valueOf(product, local_4)?;
        return Ok(_t1);
        let _t2 = x.multiply(y)?;
        Ok(BigDecimal::new(_t2, 9223372036854775808i64, local_4, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(JLjava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiply(JLjava/math/BigInteger;I)Ljava/math/BigDecimal;
    pub fn multiply__l_bigint_i(x: i64, arg_1: Object, y: i32) -> Result<Object> {
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::zeroValueOf(local_3)?;
        return Ok(_t0);
        let _t1 = y.multiply(x)?;
        Ok(BigDecimal::new(_t1, 9223372036854775808i64, local_3, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiply(Ljava/math/BigInteger;Ljava/math/BigInteger;I)Ljava/math/BigDecimal;
    pub fn multiply__bigint_bigint_i(x: Object, y: Object, scale: i32) -> Result<Object> {
        let _t0 = x.multiply(y)?;
        Ok(BigDecimal::new(_t0, 9223372036854775808i64, scale, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "multiplyAndRound", descriptor = "(JJILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiplyAndRound(JJILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn multiplyAndRound__l_l_i_mathco(x: i64, arg_1: i64, y: i32, arg_3: Object) -> Result<Object> {
        let _t0: i64 = BigDecimal::multiply(x, y)?;
        let mut product: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: Object = BigDecimal::doRound(product, local_4, local_5)?;
        return Ok(_t1);
        let mut rsign: i32 = 1i32;
        /* TODO: lcmp  */
        /* TODO: lneg  */
        x = x;
        rsign = -1i32;
        /* TODO: lcmp  */
        /* TODO: lneg  */
        y = y;
        rsign = (rsign).wrapping_mul(-1i32);
        /* TODO: lushr  */
        let mut m0_hi: i64 = 32i32;
        /* TODO: land  */
        let mut m0_lo: i64 = 4294967295i64;
        /* TODO: lushr  */
        let mut m1_hi: i64 = 32i32;
        /* TODO: land  */
        let mut m1_lo: i64 = 4294967295i64;
        product = (m0_lo).wrapping_mul(m1_lo);
        /* TODO: land  */
        let mut m0: i64 = 4294967295i64;
        /* TODO: lushr  */
        let mut m1: i64 = 32i32;
        product = ((m0_hi).wrapping_mul(m1_lo)).wrapping_add(m1);
        /* TODO: land  */
        m1 = 4294967295i64;
        /* TODO: lushr  */
        let mut m2: i64 = 32i32;
        product = ((m0_lo).wrapping_mul(m1_hi)).wrapping_add(m1);
        /* TODO: land  */
        m1 = 4294967295i64;
        /* TODO: lushr  */
        m2 = (product).wrapping_add(32i32);
        /* TODO: lushr  */
        let mut m3: i64 = 32i32;
        /* TODO: land  */
        m2 = 4294967295i64;
        product = ((m0_hi).wrapping_mul(m1_hi)).wrapping_add(m2);
        /* TODO: land  */
        m2 = 4294967295i64;
        /* TODO: lushr  */
        /* TODO: land  */
        m3 = 4294967295i64;
        let _t2: i64 = BigDecimal::make64(m3, m2)?;
        let mut mHi: i64 = _t2;
        let _t3: i64 = BigDecimal::make64(m1, m0)?;
        let mut mLo: i64 = _t3;
        let _t4: Object = BigDecimal::doRound128(mHi, mLo, rsign, local_4, local_5)?;
        let mut res: Object = _t4;
        return Ok(res);
        let _t5 = x.multiply((y).wrapping_mul((rsign as i64)))?;
        res = BigDecimal::new(_t5, 9223372036854775808i64, local_4, 0i32)?;
        let _t6: Object = BigDecimal::doRound(res, local_5)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "multiplyAndRound", descriptor = "(JLjava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiplyAndRound(JLjava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn multiplyAndRound__l_bigint_i_mathco(x: i64, arg_1: Object, y: i32, scale: Object) -> Result<Object> {
        /* TODO: lcmp  */
        let _t0: Object = BigDecimal::zeroValueOf(scale)?;
        return Ok(_t0);
        let _t1 = y.multiply(x)?;
        let _t2: Object = BigDecimal::doRound(_t1, scale, local_4)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "multiplyAndRound", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    // java: multiplyAndRound(Ljava/math/BigInteger;Ljava/math/BigInteger;ILjava/math/MathContext;)Ljava/math/BigDecimal;
    pub fn multiplyAndRound__bigint_bigint_i_mathco(x: Object, y: Object, scale: i32, mc: Object) -> Result<Object> {
        let _t0 = x.multiply(y)?;
        let _t1: Object = BigDecimal::doRound(_t0, scale, mc)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "doRound128", descriptor = "(JJIILjava/math/MathContext;)Ljava/math/BigDecimal;", access = "private static"))]
    pub fn doRound128(hi: i64, arg_1: i64, lo: i32, arg_3: i32, sign: Object) -> Result<Object> {
        let mut mcp: i32 = local_6.precision.get();
        /* TODO: aconst_null  */
        let mut res: i32 = todo!("stack underflow");
        let _t0: i32 = BigDecimal::precision(hi, lo)?;
        let mut drop: i32 = (_t0).wrapping_sub(mcp);
        let _t1: i32 = BigDecimal::checkScaleNonZero(((local_5 as i64)).wrapping_sub((drop as i64)))?;
        let mut scale: i32 = _t1;
        let _t2: Object = BigDecimal::divideAndRound128(hi, lo, BigDecimal::LONG_TEN_POWERS_TABLE()[drop as usize], sign, scale, local_6.roundingMode.get().oldMode.get(), scale)?;
        res = _t2;
        let _t3: Object = BigDecimal::doRound(res, local_6)?;
        return Ok(_t3);
        /* TODO: aconst_null  */
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "precision", descriptor = "(JJ)I", access = "private static"))]
    // java: precision(JJ)I
    pub fn precision__l_l(hi: i64, arg_1: i64) -> Result<i32> {
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t0: i32 = BigDecimal::longDigitLength(local_2)?;
        return Ok(_t0);
        let _t1: bool = BigDecimal::unsignedLongCompareEq(local_2, BigDecimal::LONGLONG_TEN_POWERS_TABLE()[0i32 as usize].clone()[1i32 as usize])?;
        return Ok(19i32);
        let _t2: i32 = Long::numberOfLeadingZeros(hi)?;
        let mut r: i32 = (((((128i32).wrapping_sub(_t2)).wrapping_add(1i32)).wrapping_mul(1233i32) as u32>>(12i32&0x1f)) as i32);
        let mut idx: i32 = (r).wrapping_sub(19i32);
        let _t3: bool = BigDecimal::longLongCompareMagnitude(hi, local_2, BigDecimal::LONGLONG_TEN_POWERS_TABLE()[idx as usize].clone()[0i32 as usize], BigDecimal::LONGLONG_TEN_POWERS_TABLE()[idx as usize].clone()[1i32 as usize])?;
        Ok((r).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "longLongCompareMagnitude", descriptor = "(JJJJ)Z", access = "private static"))]
    pub fn longLongCompareMagnitude(hi0: i64, arg_1: i64, lo0: i64, arg_3: i64) -> Result<bool> {
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(local_4<0i32);
        /* TODO: lcmp  */
        Ok((local_6).wrapping_add(9223372036854775808i64)<0i32)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(JIJIII)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(JIJIII)Ljava/math/BigDecimal;
    pub fn divide__l_i_l_i_i_i(dividend: i64, arg_1: i32, dividendScale: i64, divisor: i32, arg_4: i32, divisorScale: i32) -> Result<Object> {
        let _t0: i32 = BigDecimal::checkScale(dividend, ((local_6 as i64)).wrapping_add((divisorScale as i64)))?;
        let mut newScale: i32 = (local_6).wrapping_add(divisorScale);
        let mut raise: i32 = (newScale).wrapping_sub(dividendScale);
        let mut xs: i64 = dividend;
        let _t1: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        /* TODO: dup2  */
        xs = _t1;
        /* TODO: lcmp  */
        let _t2: Object = BigDecimal::divideAndRound(xs, divisor, local_6, local_7, local_6)?;
        return Ok(_t2);
        let _t3: Object = BigDecimal::multiplyDivideAndRound(BigDecimal::LONG_TEN_POWERS_TABLE()[raise as usize], dividend, divisor, local_6, local_7, local_6)?;
        let mut q: Object = _t3;
        return Ok(q);
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(dividend, raise)?;
        xs = _t4;
        let _t5: Object = BigDecimal::divideAndRound(xs, divisor, local_6, local_7, local_6)?;
        return Ok(_t5);
        let _t6: i32 = BigDecimal::checkScale(divisor, ((dividendScale as i64)).wrapping_sub((local_6 as i64)))?;
        newScale = _t6;
        raise = (newScale).wrapping_sub(divisorScale);
        xs = divisor;
        let _t7: i64 = BigDecimal::longMultiplyPowerTen(xs, raise)?;
        /* TODO: dup2  */
        xs = _t7;
        /* TODO: lcmp  */
        let _t8: Object = BigDecimal::divideAndRound(dividend, xs, local_6, local_7, local_6)?;
        return Ok(_t8);
        let _t9: Object = BigDecimal::bigMultiplyPowerTen(divisor, raise)?;
        xs = _t9;
        let _t10: Object = BigDecimal::divideAndRound(dividend, xs, local_6, local_7, local_6)?;
        Ok(_t10)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;IJIII)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(Ljava/math/BigInteger;IJIII)Ljava/math/BigDecimal;
    pub fn divide__bigint_i_l_i_i_i(dividend: Object, dividendScale: i32, divisor: i64, arg_3: i32, divisorScale: i32, scale: i32) -> Result<Object> {
        let _t0: i32 = BigDecimal::checkScale(dividend, ((scale as i64)).wrapping_add((divisorScale as i64)))?;
        let mut newScale: i32 = (scale).wrapping_add(divisorScale);
        let mut raise: i32 = (newScale).wrapping_sub(dividendScale);
        let _t1: Object = BigDecimal::bigMultiplyPowerTen(dividend, raise)?;
        let mut scaledDividend: Object = _t1;
        let _t2: Object = BigDecimal::divideAndRound(scaledDividend, divisor, scale, local_6, scale)?;
        return Ok(_t2);
        let _t3: i32 = BigDecimal::checkScale(divisor, ((dividendScale as i64)).wrapping_sub((scale as i64)))?;
        newScale = _t3;
        raise = (newScale).wrapping_sub(divisorScale);
        scaledDividend = divisor;
        let _t4: i64 = BigDecimal::longMultiplyPowerTen(scaledDividend, raise)?;
        /* TODO: dup2  */
        scaledDividend = _t4;
        /* TODO: lcmp  */
        let _t5: Object = BigDecimal::divideAndRound(dividend, scaledDividend, scale, local_6, scale)?;
        return Ok(_t5);
        let _t6: Object = BigDecimal::bigMultiplyPowerTen(divisor, raise)?;
        scaledDividend = _t6;
        let _t7: Object = BigDecimal::divideAndRound(dividend, scaledDividend, scale, local_6, scale)?;
        Ok(_t7)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(JILjava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(JILjava/math/BigInteger;III)Ljava/math/BigDecimal;
    pub fn divide__l_i_bigint_i_i_i(dividend: i64, arg_1: i32, dividendScale: Object, divisor: i32, divisorScale: i32, scale: i32) -> Result<Object> {
        let _t0: i32 = BigDecimal::checkScale(dividend, ((scale as i64)).wrapping_add((divisorScale as i64)))?;
        let mut newScale: i32 = (scale).wrapping_add(divisorScale);
        let mut raise: i32 = (newScale).wrapping_sub(dividendScale);
        let _t1: Object = BigDecimal::bigMultiplyPowerTen(dividend, raise)?;
        let mut scaledDividend: Object = _t1;
        let _t2: Object = BigDecimal::divideAndRound(scaledDividend, divisor, scale, local_6, scale)?;
        return Ok(_t2);
        let _t3: i32 = BigDecimal::checkScale(divisor, ((dividendScale as i64)).wrapping_sub((scale as i64)))?;
        newScale = _t3;
        raise = (newScale).wrapping_sub(divisorScale);
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(divisor, raise)?;
        scaledDividend = _t4;
        let _t5: Object = BigDecimal::divideAndRound(dividend, scaledDividend, scale, local_6, scale)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "divide", descriptor = "(Ljava/math/BigInteger;ILjava/math/BigInteger;III)Ljava/math/BigDecimal;", access = "private static"))]
    // java: divide(Ljava/math/BigInteger;ILjava/math/BigInteger;III)Ljava/math/BigDecimal;
    pub fn divide__bigint_i_bigint_i_i_i(dividend: Object, dividendScale: i32, divisor: Object, divisorScale: i32, scale: i32, roundingMode: i32) -> Result<Object> {
        let _t0: i32 = BigDecimal::checkScale(dividend, ((scale as i64)).wrapping_add((divisorScale as i64)))?;
        let mut newScale: i32 = (scale).wrapping_add(divisorScale);
        let mut raise: i32 = (newScale).wrapping_sub(dividendScale);
        let _t1: Object = BigDecimal::bigMultiplyPowerTen(dividend, raise)?;
        let mut scaledDividend: Object = _t1;
        let _t2: Object = BigDecimal::divideAndRound(scaledDividend, divisor, scale, roundingMode, scale)?;
        return Ok(_t2);
        let _t3: i32 = BigDecimal::checkScale(divisor, ((dividendScale as i64)).wrapping_sub((scale as i64)))?;
        newScale = _t3;
        raise = (newScale).wrapping_sub(divisorScale);
        let _t4: Object = BigDecimal::bigMultiplyPowerTen(divisor, raise)?;
        scaledDividend = _t4;
        let _t5: Object = BigDecimal::divideAndRound(dividend, scaledDividend, scale, roundingMode, scale)?;
        Ok(_t5)
    }
}
