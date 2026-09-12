#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/String",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable,java/lang/Comparable,java/lang/CharSequence,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "String.java",
))]
pub struct String {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "[B", access = "private final"))]
    pub value: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "coder", descriptor = "B", access = "private final"))]
    pub coder: Field<i8>,
    #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private"))]
    pub hash: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hashIsZero", descriptor = "Z", access = "private"))]
    pub hashIsZero: Field<bool>,
}

impl String {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(String::from("").value.get());
        this.coder.set(String::from("").coder.get());
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(original: String) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(original.value.get());
        this.coder.set(original.coder.get());
        this.hash.set(original.hash.get());
        this.hashIsZero.set(original.hashIsZero.get());
        Ok(this)
    }

    // java: <init>([C)V
    // java: <init>([C)V
    pub fn new__arr_c(value: Vec<u16>) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/String.<init>:([CIILjava/lang/Void;)V */
        Ok(this)
    }

    // java: <init>([CII)V
    // java: <init>([CII)V
    pub fn new__arr_c_i_i(value: Vec<u16>, offset: i32, count: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = String::rangeCheck(&value, offset, count)?;
        /* invokespecial Method java/lang/String.<init>:([CIILjava/lang/Void;)V */
        Ok(this)
    }

    // java: rangeCheck([CII)Ljava/lang/Void;
    pub fn rangeCheck(value: &[u16], offset: i32, count: i32) -> Result<Object> {
        let _t0: i32 = String::checkBoundsOffCount(offset, count, (value.len() as i32))?;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    // java: <init>([III)V
    // java: <init>([III)V
    pub fn new__arr_i_i_i(codePoints: Vec<i32>, offset: i32, count: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: i32 = String::checkBoundsOffCount(offset, count, (codePoints.len() as i32))?;
        this.value.set(String::from("").value.get());
        this.coder.set(String::from("").coder.get());
        return Ok(());
        let _t1: Vec<i8> = StringUTF16::compress(&codePoints, offset, count)?;
        let mut val: Vec<i8> = _t1;
        let _t2: i8 = StringUTF16::coderFromArrayLen(&val, count)?;
        this.coder.set(_t2);
        this.value.set(val);
        return Ok(());
        this.coder.set(1i32);
        let _t3: Vec<i8> = StringUTF16::toBytes(&codePoints, offset, count)?;
        this.value.set(_t3);
        Ok(this)
    }

    // java: <init>([BIII)V
    // java: <init>([BIII)V
    pub fn new__arr_b_i_i_i(ascii: Vec<i8>, hibyte: i32, offset: i32, count: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: i32 = String::checkBoundsOffCount(offset, count, (ascii.len() as i32))?;
        this.value.set(String::from("").value.get());
        this.coder.set(String::from("").coder.get());
        return Ok(());
        /* TODO: i2b  */
        let _t1: Vec<i8> = Arrays::copyOfRange(&ascii, offset, (offset).wrapping_add(count))?;
        this.value.set(_t1);
        this.coder.set(0i32);
        hibyte = (hibyte<<(8i32&0x1f));
        let _t2: Vec<i8> = StringUTF16::newBytesFor(count)?;
        let mut val: Vec<i8> = _t2;
        let mut i: i32 = 0i32;
        loop {
            if i >= count { break; }
            offset = offset.wrapping_add(1i32);
            StringUTF16::putChar(&val, i, (hibyte|(ascii[offset as usize]&255i32)))?;
            i = i.wrapping_add(1i32);
        }
        this.value.set(val);
        this.coder.set(1i32);
        Ok(this)
    }

    // java: <init>([BI)V
    // java: <init>([BI)V
    pub fn new__arr_b_i(ascii: Vec<i8>, hibyte: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/String.<init>:([BIII)V */
        Ok(this)
    }

    // java: <init>([BIILjava/lang/String;)V
    // java: <init>([BIILjava/lang/String;)V
    pub fn new__arr_b_i_i_str(bytes: Vec<i8>, offset: i32, length: i32, charsetName: String) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = String::lookupCharset(charsetName)?;
        let _t1: i32 = String::checkBoundsOffCount(offset, length, (bytes.len() as i32))?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>([BIILjava/nio/charset/Charset;)V
    // java: <init>([BIILjava/nio/charset/Charset;)V
    pub fn new__arr_b_i_i_charse(bytes: Vec<i8>, offset: i32, length: i32, charset: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = Objects::requireNonNull(charset)?;
        let _t1: i32 = String::checkBoundsOffCount(offset, length, (bytes.len() as i32))?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>(Ljava/nio/charset/Charset;[BII)V
    // java: <init>(Ljava/nio/charset/Charset;[BII)V
    pub fn new__charse_arr_b_i_i(charset: Object, bytes: Vec<i8>, offset: i32, length: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(String::from("").value.get());
        this.coder.set(String::from("").coder.get());
        let _t0: i32 = StringCoding::countPositives(&bytes, offset, length)?;
        let mut dp: i32 = _t0;
        let _t1: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        this.value.set(_t1);
        this.coder.set(0i32);
        return Ok(());
        let _t2: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        let mut latin1: Vec<i8> = _t2;
        let mut sp: i32 = dp;
        loop {
            if sp >= length { break; }
            sp = sp.wrapping_add(1i32);
            let mut b1: i32 = latin1[sp as usize];
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            latin1[dp as usize] = b1;
            let mut b2: i32 = latin1[sp as usize];
            dp = dp.wrapping_add(1i32);
            let _t0: u16 = String::decode2(b1, b2)?;
            /* TODO: i2b  */
            latin1[dp as usize] = _t0;
            sp = sp.wrapping_add(1i32);
        }
        sp = sp.wrapping_sub(1i32);
        let _t3: Vec<i8> = Arrays::copyOf(&latin1, dp)?;
        latin1 = _t3;
        this.value.set(latin1);
        this.coder.set(0i32);
        return Ok(());
        let _t4: Vec<i8> = StringUTF16::newBytesFor(length)?;
        b1 = _t4;
        StringLatin1::inflate(&latin1, 0i32, b1, 0i32, dp)?;
        let _t5: i32 = String::decodeUTF8_UTF16(&latin1, sp, length, b1, dp, 1i32)?;
        dp = _t5;
        let _t6: Vec<i8> = Arrays::copyOf(b1, (dp<<(1i32&0x1f)))?;
        b1 = _t6;
        this.value.set(b1);
        this.coder.set(1i32);
        let _t7: Vec<i8> = StringUTF16::newBytesFor(length)?;
        dp = _t7;
        let _t8: i32 = String::decodeUTF8_UTF16(&bytes, offset, (offset).wrapping_add(length), dp, 0i32, 1i32)?;
        latin1 = _t8;
        let _t9: Vec<i8> = Arrays::copyOf(dp, (latin1<<(1i32&0x1f)))?;
        dp = _t9;
        this.value.set(dp);
        this.coder.set(1i32);
        let _t10: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        this.value.set(_t10);
        this.coder.set(0i32);
        let _t11: Vec<i8> = StringLatin1::inflate(&bytes, offset, length)?;
        this.value.set(_t11);
        this.coder.set(1i32);
        let _t12: bool = StringCoding::hasNegatives(&bytes, offset, length)?;
        let _t13: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        this.value.set(_t13);
        this.coder.set(0i32);
        let _t14: Vec<i8> = StringUTF16::newBytesFor(length)?;
        dp = _t14;
        latin1 = 0i32;
        loop {
            if latin1 >= length { break; }
            offset = offset.wrapping_add(1i32);
            sp = bytes[offset as usize];
            latin1 = latin1.wrapping_add(1i32);
            /* TODO: i2c  */
            StringUTF16::putChar(sp, sp, 65533i32)?;
        }
        this.value.set(dp);
        this.coder.set(1i32);
        let _t15 = charset.newDecoder()?;
        dp = _t15;
        latin1 = dp;
        let _t16 = latin1.isASCIICompatible()?;
        let _t17: bool = StringCoding::hasNegatives(&bytes, offset, length)?;
        let _t18: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        this.value.set(_t18);
        this.coder.set(0i32);
        return Ok(());
        let _t19: Vec<i8> = StringLatin1::inflate(&bytes, offset, length)?;
        this.value.set(_t19);
        this.coder.set(1i32);
        return Ok(());
        let _t20 = latin1.isLatin1Decodable()?;
        let mut _arr21: Vec<i8> = vec![0i8; length as usize];
        sp = _arr21;
        let _t22 = latin1.decodeToLatin1(bytes, offset, length, sp)?;
        this.value.set(sp);
        this.coder.set(0i32);
        return Ok(());
        let _t23 = dp.maxCharsPerByte()?;
        let _t24: i32 = String::scale(length, _t23)?;
        sp = _t24;
        let _t25 = dp.onMalformedInput(CodingErrorAction::REPLACE())?;
        let _t26 = _t25.onUnmappableCharacter(CodingErrorAction::REPLACE())?;
        let mut _arr27: Vec<u16> = vec![0u16; sp as usize];
        b1 = _arr27;
        let _t28 = latin1.decode(bytes, offset, length, b1)?;
        b2 = _t28;
        let _t29: Vec<i8> = StringUTF16::compress(b1, 0i32, b2)?;
        let mut val: Vec<i8> = _t29;
        let _t30: i8 = StringUTF16::coderFromArrayLen(&val, b2)?;
        this.coder.set(_t30);
        this.value.set(val);
        return Ok(());
        this.coder.set(1i32);
        let _t31: Vec<i8> = StringUTF16::toBytes(b1, 0i32, b2)?;
        this.value.set(_t31);
        return Ok(());
        let _t32 = dp.maxCharsPerByte()?;
        let _t33: i32 = String::scale(length, _t32)?;
        latin1 = _t33;
        let _t34 = dp.onMalformedInput(CodingErrorAction::REPLACE())?;
        let _t35 = _t34.onUnmappableCharacter(CodingErrorAction::REPLACE())?;
        let mut _arr36: Vec<u16> = vec![0u16; latin1 as usize];
        sp = _arr36;
        let _t37 = charset.getClass()?;
        let _t38 = _t37.getClassLoader0()?;
        let _t39: Object = System::getSecurityManager()?;
        let _t40: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        bytes = _t40;
        offset = 0i32;
        let _t41: i32 = String::decodeWithDecoder(dp, sp, &bytes, offset, length)?;
        b1 = _t41;
        b2 = _t39;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t42: Vec<i8> = StringUTF16::compress(sp, 0i32, b1)?;
        b2 = _t42;
        let _t43: i8 = StringUTF16::coderFromArrayLen(b2, b1)?;
        this.coder.set(_t43);
        this.value.set(b2);
        return Ok(());
        this.coder.set(1i32);
        let _t44: Vec<i8> = StringUTF16::toBytes(sp, 0i32, b1)?;
        this.value.set(_t44);
        Ok(this)
    }

    // java: newStringUTF8NoRepl([BIIZ)Ljava/lang/String;
    pub fn newStringUTF8NoRepl(bytes: &[i8], offset: i32, length: i32, noShare: bool) -> Result<String> {
        let _t0: i32 = String::checkBoundsOffCount(offset, length, (bytes.len() as i32))?;
        return Ok(String::from(""));
        let _t1: i32 = StringCoding::countPositives(&bytes, offset, length)?;
        let mut dp: i32 = _t1;
        let mut sl: i32 = (offset).wrapping_add(length);
        let _t2: Vec<i8> = Arrays::copyOfRange(&bytes, offset, (offset).wrapping_add(length))?;
        return Ok(String::new(_t2, 0i32)?);
        return Ok(String::new(bytes, 0i32)?);
        let mut _arr3: Vec<i8> = vec![0i8; length as usize];
        let mut dst: Vec<i8> = _arr3;
        System::arraycopy(&bytes, offset, &dst, 0i32, dp)?;
        offset = (offset).wrapping_add(dp);
        loop {
            if offset >= sl { break; }
            offset = offset.wrapping_add(1i32);
            let mut b1: i32 = bytes[offset as usize];
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = b1;
            let mut b2: i32 = bytes[offset as usize];
            dp = dp.wrapping_add(1i32);
            let _t0: u16 = String::decode2(b1, b2)?;
            /* TODO: i2b  */
            dst[dp as usize] = _t0;
            offset = offset.wrapping_add(1i32);
        }
        offset = offset.wrapping_sub(1i32);
        let _t4: Vec<i8> = Arrays::copyOf(&dst, dp)?;
        dst = _t4;
        return Ok(String::new(dst, 0i32)?);
        let _t5: Vec<i8> = StringUTF16::newBytesFor(length)?;
        dst = _t5;
        let _t6: Vec<i8> = StringUTF16::newBytesFor(length)?;
        b1 = _t6;
        StringLatin1::inflate(&dst, 0i32, b1, 0i32, dp)?;
        dst = b1;
        let _t7: i32 = String::decodeUTF8_UTF16(&bytes, offset, sl, &dst, dp, 0i32)?;
        dp = _t7;
        let _t8: Vec<i8> = StringUTF16::newBytesFor(length)?;
        dst = _t8;
        let _t9: i32 = String::decodeUTF8_UTF16(&bytes, offset, (offset).wrapping_add(length), &dst, 0i32, 0i32)?;
        dp = _t9;
        let _t10: Vec<i8> = Arrays::copyOf(&dst, (dp<<(1i32&0x1f)))?;
        dst = _t10;
        Ok(String::new(dst, 1i32)?)
    }

    // java: newStringNoRepl([BLjava/nio/charset/Charset;)Ljava/lang/String;
    pub fn newStringNoRepl(src: &[i8], cs: Object) -> Result<String> {
        let _t0: String = String::newStringNoRepl1(&src, cs)?;
        return Ok(_t0);
        let mut e: i32 = todo!("stack underflow");
        let _t1 = e.getCause()?;
        let mut cause: Object = _t1;
        let mut mie: Object = cause;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: newStringNoRepl1([BLjava/nio/charset/Charset;)Ljava/lang/String;
    pub fn newStringNoRepl1(src: &[i8], cs: Object) -> Result<String> {
        let mut len: i32 = (src.len() as i32);
        return Ok(String::from(""));
        let _t0: String = String::newStringUTF8NoRepl(&src, 0i32, (src.len() as i32), 0i32)?;
        return Ok(_t0);
        return Ok(String::new(src, 0i32)?);
        let _t1: Vec<i8> = StringLatin1::inflate(&src, 0i32, (src.len() as i32))?;
        return Ok(String::new(_t1, 1i32)?);
        let _t2: bool = StringCoding::hasNegatives(&src, 0i32, (src.len() as i32))?;
        return Ok(String::new(src, 0i32)?);
        let _t3: Vec<i8> = StringLatin1::inflate(&src, 0i32, (src.len() as i32))?;
        return Ok(String::new(_t3, 1i32)?);
        String::throwMalformed(&src)?;
        let _t4 = cs.newDecoder()?;
        let mut cd: Object = _t4;
        let mut ad: Object = cd;
        let _t5 = ad.isASCIICompatible()?;
        let _t6: bool = StringCoding::hasNegatives(&src, 0i32, (src.len() as i32))?;
        return Ok(String::new(src, 0i32)?);
        return Ok(String::new(src, 0i32, (src.len() as i32), ISO_8859_1::INSTANCE())?);
        let _t7 = cd.maxCharsPerByte()?;
        let _t8: i32 = String::scale(len, _t7)?;
        ad = _t8;
        let mut _arr9: Vec<u16> = vec![0u16; ad as usize];
        let mut ca: Vec<u16> = _arr9;
        let _t10 = cs.getClass()?;
        let _t11 = _t10.getClassLoader0()?;
        let _t12: Object = System::getSecurityManager()?;
        let _t13: Vec<i8> = Arrays::copyOf(&src, len)?;
        src = _t13;
        let _t14: i32 = String::decodeWithDecoder(cd, &ca, &src, 0i32, (src.len() as i32))?;
        let mut caLen: i32 = _t14;
        let mut x: Object = _t12;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t15: Vec<i8> = StringUTF16::compress(&ca, 0i32, caLen)?;
        x = _t15;
        let _t16: i8 = StringUTF16::coderFromArrayLen(x, caLen)?;
        let mut coder: i32 = _t16;
        return Ok(String::new(x, coder)?);
        let _t17: Vec<i8> = StringUTF16::toBytes(&ca, 0i32, caLen)?;
        Ok(String::new(_t17, 1i32)?)
    }

    // java: safeTrim([BIZ)[B
    pub fn safeTrim(ba: &[i8], len: i32, isTrusted: bool) -> Result<Vec<i8>> {
        let _t0: Object = System::getSecurityManager()?;
        return Ok(ba);
        let _t1: Vec<i8> = Arrays::copyOf(&ba, len)?;
        Ok(_t1)
    }

    // java: scale(IF)I
    pub fn scale(len: i32, expansionFactor: f32) -> Result<i32> {
        Ok((((len as f64)*(expansionFactor as f64)) as i32))
    }

    // java: lookupCharset(Ljava/lang/String;)Ljava/nio/charset/Charset;
    pub fn lookupCharset(csn: String) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(csn)?;
        let _t1: Object = Charset::forName(csn)?;
        return Ok(_t1);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: encode(Ljava/nio/charset/Charset;B[B)[B
    pub fn encode(cs: Object, coder: i8, val: &[i8]) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = String::encodeUTF8(coder, &val, 1i32)?;
        return Ok(_t0);
        let _t1: Vec<i8> = String::encode8859_1(coder, &val)?;
        return Ok(_t1);
        let _t2: Vec<i8> = String::encodeASCII(coder, &val)?;
        return Ok(_t2);
        let _t3: Vec<i8> = String::encodeWithEncoder(cs, coder, &val, 1i32)?;
        Ok(_t3)
    }

    // java: encodeWithEncoder(Ljava/nio/charset/Charset;B[BZ)[B
    pub fn encodeWithEncoder(cs: Object, coder: i8, val: &[i8], doReplace: bool) -> Result<Vec<i8>> {
        let _t0 = cs.newEncoder()?;
        let mut ce: Object = _t0;
        let mut len: i32 = ((val.len() as i32)>>((coder&0x1f)));
        let _t1 = ce.maxBytesPerChar()?;
        let _t2: i32 = String::scale(len, _t1)?;
        let mut en: i32 = _t2;
        let mut ae: Object = ce;
        let _t3 = ae.isASCIICompatible()?;
        let _t4: bool = StringCoding::hasNegatives(&val, 0i32, (val.len() as i32))?;
        let _t5 = val.clone()?;
        return Ok(_t5);
        let mut _arr6: Vec<i8> = vec![0i8; en as usize];
        let mut ba: Vec<i8> = _arr6;
        return Ok(ba);
        let _t7 = ae.encodeFromLatin1(val, 0i32, len, ba)?;
        let _t8 = ae.encodeFromUTF16(val, 0i32, len, ba)?;
        let mut blen: i32 = _t8;
        let _t9: Vec<i8> = String::safeTrim(&ba, blen, 1i32)?;
        return Ok(_t9);
        let mut _arr10: Vec<i8> = vec![0i8; en as usize];
        ae = _arr10;
        return Ok(ae);
        let _t11 = ce.onMalformedInput(CodingErrorAction::REPLACE())?;
        let _t12 = _t11.onUnmappableCharacter(CodingErrorAction::REPLACE())?;
        let _t13: Vec<u16> = StringLatin1::toChars(&val)?;
        let _t14: Vec<u16> = StringUTF16::toChars(&val)?;
        ba = _t14;
        let _t15: Object = ByteBuffer::wrap(ae)?;
        blen = _t15;
        let _t16: Object = CharBuffer::wrap(&ba, 0i32, len)?;
        let mut cb: Object = _t16;
        let _t17 = ce.encode(cb, blen, 1i32)?;
        let mut cr: Object = _t17;
        let _t18 = cr.isUnderflow()?;
        cr.throwException()?;
        let _t19 = ce.flush(blen)?;
        cr = _t19;
        let _t20 = cr.isUnderflow()?;
        cr.throwException()?;
        cr = _t20;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t21 = blen.position()?;
        let _t22 = cs.getClass()?;
        let _t23 = _t22.getClassLoader0()?;
        let _t24: Vec<i8> = String::safeTrim(ae, _t21, _t23.is_none())?;
        Ok(_t24)
    }

    // java: getBytesUTF8NoRepl(Ljava/lang/String;)[B
    pub fn getBytesUTF8NoRepl(s: String) -> Result<Vec<i8>> {
        let _t0 = s.coder()?;
        let _t1 = s.value()?;
        let _t2: Vec<i8> = String::encodeUTF8(_t0, &_t1, 0i32)?;
        Ok(_t2)
    }

    // java: isASCII([B)Z
    pub fn isASCII(src: &[i8]) -> Result<bool> {
        let _t0: bool = StringCoding::hasNegatives(&src, 0i32, (src.len() as i32))?;
        Ok(_t0==0i32)
    }

    // java: getBytesNoRepl(Ljava/lang/String;Ljava/nio/charset/Charset;)[B
    pub fn getBytesNoRepl(s: String, cs: Object) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = String::getBytesNoRepl1(s, cs)?;
        return Ok(_t0);
        let mut e: i32 = todo!("stack underflow");
        let _t1 = e.getCause()?;
        let mut cause: Object = _t1;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: getBytesNoRepl1(Ljava/lang/String;Ljava/nio/charset/Charset;)[B
    pub fn getBytesNoRepl1(s: String, cs: Object) -> Result<Vec<i8>> {
        let _t0 = s.value()?;
        let mut val: Vec<i8> = _t0;
        let _t1 = s.coder()?;
        let mut coder: i32 = _t1;
        let _t2: bool = String::isASCII(&val)?;
        return Ok(val);
        let _t3: Vec<i8> = String::encodeUTF8(coder, &val, 0i32)?;
        return Ok(_t3);
        return Ok(val);
        let _t4: Vec<i8> = String::encode8859_1(coder, &val, 0i32)?;
        return Ok(_t4);
        let _t5: bool = String::isASCII(&val)?;
        return Ok(val);
        String::throwUnmappable(&val)?;
        let _t6: Vec<i8> = String::encodeWithEncoder(cs, coder, &val, 0i32)?;
        Ok(_t6)
    }

    // java: encodeASCII(B[B)[B
    pub fn encodeASCII(coder: i8, val: &[i8]) -> Result<Vec<i8>> {
        let _t0: i32 = StringCoding::countPositives(&val, 0i32, (val.len() as i32))?;
        let mut positives: i32 = _t0;
        let _t1 = val.clone()?;
        let mut dst: Object = _t1;
        String::replaceNegatives(dst, positives)?;
        return Ok(dst);
        positives = ((val.len() as i32)>>((1i32&0x1f)));
        let mut _arr2: Vec<i8> = vec![0i8; positives as usize];
        dst = _arr2;
        let mut dp: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= positives { break; }
            let _t0: u16 = StringUTF16::getChar(&val, i)?;
            let mut c: i32 = _t0;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = c;
            let _t1: bool = Character::isHighSurrogate(c)?;
            let _t2: u16 = StringUTF16::getChar(&val, (i).wrapping_add(1i32))?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            i = i.wrapping_add(1i32);
            dp = dp.wrapping_add(1i32);
            dst[dp as usize] = 63i32;
            i = i.wrapping_add(1i32);
        }
        return Ok(dst);
        let _t3: Vec<i8> = Arrays::copyOf(dst, dp)?;
        Ok(_t3)
    }

    // java: replaceNegatives([BI)V
    pub fn replaceNegatives(val: &[i8], fromIndex: i32) -> Result<()> {
        let mut i: i32 = fromIndex;
        loop {
            if i >= (val.len() as i32) { break; }
            val[i as usize] = 63i32;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: encode8859_1(B[B)[B
    // java: encode8859_1(B[B)[B
    pub fn encode8859_1__b_arr_b(coder: i8, val: &[i8]) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = String::encode8859_1(coder, &val, 1i32)?;
        Ok(_t0)
    }

    // java: encode8859_1(B[BZ)[B
    // java: encode8859_1(B[BZ)[B
    pub fn encode8859_1__b_arr_b_z(coder: i8, val: &[i8], doReplace: bool) -> Result<Vec<i8>> {
        let _t0 = val.clone()?;
        return Ok(_t0);
        let mut len: i32 = ((val.len() as i32)>>((1i32&0x1f)));
        let mut _arr1: Vec<i8> = vec![0i8; len as usize];
        let mut dst: Vec<i8> = _arr1;
        let mut dp: i32 = 0i32;
        let mut sp: i32 = 0i32;
        let mut sl: i32 = len;
        loop {
            if sp >= sl { break; }
            let _t0: i32 = StringCoding::implEncodeISOArray(&val, sp, &dst, dp, len)?;
            let mut ret: i32 = _t0;
            sp = (sp).wrapping_add(ret);
            dp = (dp).wrapping_add(ret);
            String::throwUnmappable(sp)?;
            sp = sp.wrapping_add(1i32);
            let _t1: u16 = StringUTF16::getChar(&val, sp)?;
            let mut c: i32 = _t1;
            let _t2: bool = Character::isHighSurrogate(c)?;
            let _t3: u16 = StringUTF16::getChar(&val, sp)?;
            let _t4: bool = Character::isLowSurrogate(_t3)?;
            sp = sp.wrapping_add(1i32);
            dp = dp.wrapping_add(1i32);
            dst[dp as usize] = 63i32;
            len = (sl).wrapping_sub(sp);
        }
        return Ok(dst);
        let _t2: Vec<i8> = Arrays::copyOf(&dst, dp)?;
        Ok(_t2)
    }

    // java: decodeASCII([BI[CII)I
    pub fn decodeASCII(sa: &[i8], sp: i32, da: &[u16], dp: i32, len: i32) -> Result<i32> {
        let _t0: i32 = StringCoding::countPositives(&sa, sp, len)?;
        let mut count: i32 = _t0;
        loop {
            if count >= len { break; }
            count = count.wrapping_add(1i32);
        }
        StringLatin1::inflate(&sa, sp, &da, dp, count)?;
        Ok(count)
    }

    // java: isNotContinuation(I)Z
    pub fn isNotContinuation(b: i32) -> Result<bool> {
        Ok((b&192i32) != 128i32)
    }

    // java: isMalformed3(III)Z
    pub fn isMalformed3(b1: i32, b2: i32, b3: i32) -> Result<bool> {
        Ok((b3&192i32) != 128i32)
    }

    // java: isMalformed3_2(II)Z
    pub fn isMalformed3_2(b1: i32, b2: i32) -> Result<bool> {
        Ok((b2&192i32) != 128i32)
    }

    // java: isMalformed4(III)Z
    pub fn isMalformed4(b2: i32, b3: i32, b4: i32) -> Result<bool> {
        Ok((b4&192i32) != 128i32)
    }

    // java: isMalformed4_2(II)Z
    pub fn isMalformed4_2(b1: i32, b2: i32) -> Result<bool> {
        Ok((b2&192i32) != 128i32)
    }

    // java: isMalformed4_3(I)Z
    pub fn isMalformed4_3(b3: i32) -> Result<bool> {
        Ok((b3&192i32) != 128i32)
    }

    // java: decode2(II)C
    pub fn decode2(b1: i32, b2: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok((((b1<<(6i32&0x1f))^b2)^3968i32))
    }

    // java: decode3(III)C
    pub fn decode3(b1: i32, b2: i32, b3: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok((((b1<<(12i32&0x1f))^(b2<<(6i32&0x1f)))^(b3^383i32)))
    }

    // java: decode4(IIII)I
    pub fn decode4(b1: i32, b2: i32, b3: i32, b4: i32) -> Result<i32> {
        Ok(((((b1<<(18i32&0x1f))^(b2<<(12i32&0x1f)))^(b3<<(6i32&0x1f)))^(b4^384i32)))
    }

    // java: decodeUTF8_UTF16([BII[BIZ)I
    pub fn decodeUTF8_UTF16(src: &[i8], sp: i32, sl: i32, dst: &[i8], dp: i32, doReplace: bool) -> Result<i32> {
        loop {
            if sp >= sl { break; }
            sp = sp.wrapping_add(1i32);
            let mut b1: i32 = src[sp as usize];
            dp = dp.wrapping_add(1i32);
            /* TODO: i2c  */
            StringUTF16::putChar(&dst, dp, b1)?;
            sp = sp.wrapping_add(1i32);
            let mut b2: i32 = src[sp as usize];
            let _t0: bool = String::isNotContinuation(b2)?;
            String::throwMalformed((sp).wrapping_sub(1i32), 1i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            sp = sp.wrapping_sub(1i32);
            dp = dp.wrapping_add(1i32);
            let _t1: u16 = String::decode2(b1, b2)?;
            StringUTF16::putChar(&dst, dp, _t1)?;
            String::throwMalformed(sp, 1i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            sp = sp.wrapping_add(1i32);
            b2 = src[sp as usize];
            sp = sp.wrapping_add(1i32);
            let mut b3: i32 = src[sp as usize];
            let _t2: bool = String::isMalformed3(b1, b2, b3)?;
            String::throwMalformed((sp).wrapping_sub(3i32), 3i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            sp = sp.wrapping_sub(3i32);
            let _t3: i32 = String::malformed3(&src, sp)?;
            sp = (sp).wrapping_add(_t3);
            let _t4: u16 = String::decode3(b1, b2, b3)?;
            let mut c: i32 = _t4;
            let _t5: bool = Character::isSurrogate(c)?;
            String::throwMalformed((sp).wrapping_sub(3i32), 3i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, c)?;
            let _t6: bool = String::isMalformed3_2(b1, src[sp as usize])?;
            String::throwMalformed((sp).wrapping_sub(1i32), 2i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            String::throwMalformed(sp, 1i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            sp = sp.wrapping_add(1i32);
            b2 = src[sp as usize];
            sp = sp.wrapping_add(1i32);
            b3 = src[sp as usize];
            sp = sp.wrapping_add(1i32);
            c = src[sp as usize];
            let _t7: i32 = String::decode4(b1, b2, b3, c)?;
            let mut uc: i32 = _t7;
            let _t8: bool = String::isMalformed4(b2, b3, c)?;
            let _t9: bool = Character::isSupplementaryCodePoint(uc)?;
            String::throwMalformed((sp).wrapping_sub(4i32), 4i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            sp = sp.wrapping_sub(4i32);
            let _t10: i32 = String::malformed4(&src, sp)?;
            sp = (sp).wrapping_add(_t10);
            dp = dp.wrapping_add(1i32);
            let _t11: u16 = Character::highSurrogate(uc)?;
            StringUTF16::putChar(&dst, dp, _t11)?;
            dp = dp.wrapping_add(1i32);
            let _t12: u16 = Character::lowSurrogate(uc)?;
            StringUTF16::putChar(&dst, dp, _t12)?;
            b1 = (b1&255i32);
            let _t13: bool = String::isMalformed4_2(b1, (src[sp as usize]&255i32))?;
            String::throwMalformed((sp).wrapping_sub(1i32), 1i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            String::throwMalformed((sp).wrapping_sub(1i32), 1i32)?;
            sp = sp.wrapping_add(1i32);
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
            let _t14: bool = String::isMalformed4_3(src[sp as usize])?;
            String::throwMalformed((sp).wrapping_sub(1i32), 1i32)?;
            dp = dp.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dp, 65533i32)?;
        }
        Ok(dp)
    }

    // java: decodeWithDecoder(Ljava/nio/charset/CharsetDecoder;[C[BII)I
    pub fn decodeWithDecoder(cd: Object, dst: &[u16], src: &[i8], offset: i32, length: i32) -> Result<i32> {
        let _t0: Object = ByteBuffer::wrap(&src, offset, length)?;
        let mut bb: Object = _t0;
        let _t1: Object = CharBuffer::wrap(&dst, 0i32, (dst.len() as i32))?;
        let mut cb: Object = _t1;
        let _t2 = cd.decode(bb, cb, 1i32)?;
        let mut cr: Object = _t2;
        let _t3 = cr.isUnderflow()?;
        cr.throwException()?;
        let _t4 = cd.flush(cb)?;
        cr = _t4;
        let _t5 = cr.isUnderflow()?;
        cr.throwException()?;
        let _t6 = cb.position()?;
        Ok(_t6)
    }

    // java: malformed3([BI)I
    pub fn malformed3(src: &[i8], sp: i32) -> Result<i32> {
        sp = sp.wrapping_add(1i32);
        let mut b1: i32 = src[sp as usize];
        let mut b2: i32 = src[sp as usize];
        let _t0: bool = String::isNotContinuation(b2)?;
        Ok(_t0==0i32)
    }

    // java: malformed4([BI)I
    pub fn malformed4(src: &[i8], sp: i32) -> Result<i32> {
        sp = sp.wrapping_add(1i32);
        let mut b1: i32 = (src[sp as usize]&255i32);
        sp = sp.wrapping_add(1i32);
        let mut b2: i32 = (src[sp as usize]&255i32);
        let _t0: bool = String::isNotContinuation(b2)?;
        return Ok(1i32);
        let _t1: bool = String::isNotContinuation(src[sp as usize])?;
        return Ok(2i32);
        Ok(3i32)
    }

    // java: throwMalformed(II)V
    // java: throwMalformed(II)V
    pub fn throwMalformed__i_i(off: i32, nb: i32) -> Result<()> {
        String::new().append(&String::from("malformed input off :"))?;
        String::new().append(&off)?;
        String::new().append(&String::from(", length :"))?;
        String::new().append(&nb)?;
        let mut msg: String = String::new();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: throwMalformed([B)V
    // java: throwMalformed([B)V
    pub fn throwMalformed__arr_b(val: &[i8]) -> Result<()> {
        let _t0: i32 = StringCoding::countPositives(&val, 0i32, (val.len() as i32))?;
        let mut dp: i32 = _t0;
        String::throwMalformed(dp, 1i32)?;
        Ok(())
    }

    // java: throwUnmappable(I)V
    // java: throwUnmappable(I)V
    pub fn throwUnmappable__i(off: i32) -> Result<()> {
        String::new().append(&String::from("malformed input off :"))?;
        String::new().append(&off)?;
        String::new().append(&String::from(", length : 1"))?;
        let mut msg: String = String::new();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: throwUnmappable([B)V
    // java: throwUnmappable([B)V
    pub fn throwUnmappable__arr_b(val: &[i8]) -> Result<()> {
        let _t0: i32 = StringCoding::countPositives(&val, 0i32, (val.len() as i32))?;
        let mut dp: i32 = _t0;
        String::throwUnmappable(dp)?;
        Ok(())
    }

    // java: encodeUTF8(B[BZ)[B
    pub fn encodeUTF8(coder: i8, val: &[i8], doReplace: bool) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = String::encodeUTF8_UTF16(&val, doReplace)?;
        return Ok(_t0);
        let _t1: bool = StringCoding::hasNegatives(&val, 0i32, (val.len() as i32))?;
        let _t2 = val.clone()?;
        return Ok(_t2);
        let mut dp: i32 = 0i32;
        let _t3: Vec<i8> = StringUTF16::newBytesFor((val.len() as i32))?;
        let mut dst: Vec<i8> = _t3;
        let mut local_5: Vec<i8> = val;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut c: i32 = local_5[local_7 as usize];
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (192i32|((c&255i32)>>((6i32&0x1f))));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|(c&63i32));
            dp = dp.wrapping_add(1i32);
            dst[dp as usize] = c;
            local_7 = local_7.wrapping_add(1i32);
        }
        return Ok(dst);
        let _t4: Vec<i8> = Arrays::copyOf(&dst, dp)?;
        Ok(_t4)
    }

    // java: encodeUTF8_UTF16([BZ)[B
    pub fn encodeUTF8_UTF16(val: &[i8], doReplace: bool) -> Result<Vec<i8>> {
        let mut dp: i32 = 0i32;
        let mut sp: i32 = 0i32;
        let mut sl: i32 = ((val.len() as i32)>>((1i32&0x1f)));
        let mut _arr0: Vec<i8> = vec![0i8; (sl).wrapping_mul(3i32) as usize];
        let mut dst: Vec<i8> = _arr0;
        loop {
            if sp >= sl { break; }
            let _t0: u16 = StringUTF16::getChar(&val, sp)?;
            let mut c: i32 = _t0;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = c;
            sp = sp.wrapping_add(1i32);
        }
        loop {
            if sp >= sl { break; }
            sp = sp.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&val, sp)?;
            c = _t0;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = c;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (192i32|(c>>((6i32&0x1f))));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|(c&63i32));
            let _t1: bool = Character::isSurrogate(c)?;
            let mut uc: i32 = -1i32;
            let _t2: bool = Character::isHighSurrogate(c)?;
            let _t3: u16 = StringUTF16::getChar(&val, sp)?;
            let mut c2: i32 = _t3;
            let _t4: bool = Character::isLowSurrogate(_t3)?;
            let _t5: i32 = Character::toCodePoint(c, c2)?;
            uc = _t5;
            dp = dp.wrapping_add(1i32);
            dst[dp as usize] = 63i32;
            String::throwUnmappable((sp).wrapping_sub(1i32))?;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (240i32|(uc>>((18i32&0x1f))));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|((uc>>((12i32&0x1f)))&63i32));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|((uc>>((6i32&0x1f)))&63i32));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|(uc&63i32));
            sp = sp.wrapping_add(1i32);
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (224i32|(c>>((12i32&0x1f))));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|((c>>((6i32&0x1f)))&63i32));
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            dst[dp as usize] = (128i32|(c&63i32));
        }
        return Ok(dst);
        let _t1: Vec<i8> = Arrays::copyOf(&dst, dp)?;
        Ok(_t1)
    }

    // java: <init>([BLjava/lang/String;)V
    // java: <init>([BLjava/lang/String;)V
    pub fn new__arr_b_str(bytes: Vec<i8>, charsetName: String) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = String::lookupCharset(charsetName)?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>([BLjava/nio/charset/Charset;)V
    // java: <init>([BLjava/nio/charset/Charset;)V
    pub fn new__arr_b_charse(bytes: Vec<i8>, charset: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = Objects::requireNonNull(charset)?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>([BII)V
    // java: <init>([BII)V
    pub fn new__arr_b_i_i(bytes: Vec<i8>, offset: i32, length: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = Charset::defaultCharset()?;
        let _t1: i32 = String::checkBoundsOffCount(offset, length, (bytes.len() as i32))?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>([B)V
    // java: <init>([B)V
    pub fn new__arr_b(bytes: Vec<i8>) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        let _t0: Object = Charset::defaultCharset()?;
        /* invokespecial Method java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/StringBuffer;)V
    // java: <init>(Ljava/lang/StringBuffer;)V
    pub fn new__string(buffer: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/String.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/StringBuilder;)V
    // java: <init>(Ljava/lang/StringBuilder;)V
    pub fn new__sb(builder: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/String.<init>:(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V */
        Ok(this)
    }

    // java: length()I
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.coder()?;
        Ok(((this.value.get().len() as i32)>>((_t0&0x1f))))
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok((this.value.get().len() as i32)==0i32)
    }

    // java: charAt(I)C
    pub fn charAt(&self, index: i32) -> Result<u16> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: u16 = StringLatin1::charAt(&this.value.get(), index)?;
        return Ok(_t1);
        let _t2: u16 = StringUTF16::charAt(&this.value.get(), index)?;
        Ok(_t2)
    }

    // java: codePointAt(I)I
    pub fn codePointAt(&self, index: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        String::checkIndex(index, (this.value.get().len() as i32))?;
        return Ok((this.value.get()[index as usize]&255i32));
        let mut length: i32 = ((this.value.get().len() as i32)>>((1i32&0x1f)));
        String::checkIndex(index, length)?;
        let _t1: i32 = StringUTF16::codePointAt(&this.value.get(), index, length)?;
        Ok(_t1)
    }

    // java: codePointBefore(I)I
    pub fn codePointBefore(&self, index: i32) -> Result<i32> {
        let this = self;
        let mut i: i32 = (index).wrapping_sub(1i32);
        let _t0 = this.length()?;
        String::checkIndex(i, _t0)?;
        let _t1 = this.isLatin1()?;
        return Ok((this.value.get()[i as usize]&255i32));
        let _t2: i32 = StringUTF16::codePointBefore(&this.value.get(), index)?;
        Ok(_t2)
    }

    // java: codePointCount(II)I
    pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.length()?;
        let _t1: i32 = Objects::checkFromToIndex(beginIndex, endIndex, _t0)?;
        let _t2 = this.isLatin1()?;
        return Ok((endIndex).wrapping_sub(beginIndex));
        let _t3: i32 = StringUTF16::codePointCount(&this.value.get(), beginIndex, endIndex)?;
        Ok(_t3)
    }

    // java: offsetByCodePoints(II)I
    pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = Character::offsetByCodePoints(this, index, codePointOffset)?;
        Ok(_t0)
    }

    // java: getChars(II[CI)V
    pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Vec<u16>, dstBegin: i32) -> Result<()> {
        let this = self;
        let _t0 = this.length()?;
        String::checkBoundsBeginEnd(srcBegin, srcEnd, _t0)?;
        let _t1: i32 = String::checkBoundsOffCount(dstBegin, (srcEnd).wrapping_sub(srcBegin), (dst.len() as i32))?;
        let _t2 = this.isLatin1()?;
        StringLatin1::getChars(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        StringUTF16::getChars(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        Ok(())
    }

    // java: getBytes(II[BI)V
    // java: getBytes(II[BI)V
    pub fn getBytes__i_i_arr_b_i(&self, srcBegin: i32, srcEnd: i32, dst: Vec<i8>, dstBegin: i32) -> Result<()> {
        let this = self;
        let _t0 = this.length()?;
        String::checkBoundsBeginEnd(srcBegin, srcEnd, _t0)?;
        let _t1: Object = Objects::requireNonNull(&dst)?;
        let _t2: i32 = String::checkBoundsOffCount(dstBegin, (srcEnd).wrapping_sub(srcBegin), (dst.len() as i32))?;
        let _t3 = this.isLatin1()?;
        StringLatin1::getBytes(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        StringUTF16::getBytes(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        Ok(())
    }

    // java: getBytes(Ljava/lang/String;)[B
    // java: getBytes(Ljava/lang/String;)[B
    pub fn getBytes__str(&self, charsetName: String) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Object = String::lookupCharset(charsetName)?;
        let _t1 = this.coder()?;
        let _t2: Vec<i8> = String::encode(_t0, _t1, &this.value.get())?;
        Ok(_t2)
    }

    // java: getBytes(Ljava/nio/charset/Charset;)[B
    // java: getBytes(Ljava/nio/charset/Charset;)[B
    pub fn getBytes__charse(&self, charset: Object) -> Result<Vec<i8>> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.coder()?;
        let _t1: Vec<i8> = String::encode(charset, _t0, &this.value.get())?;
        Ok(_t1)
    }

    // java: getBytes()[B
    // java: getBytes()[B
    pub fn getBytes(&self) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Object = Charset::defaultCharset()?;
        let _t1 = this.coder()?;
        let _t2: Vec<i8> = String::encode(_t0, _t1, &this.value.get())?;
        Ok(_t2)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, anObject: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut aString: Object = anObject;
        let _t0: bool = StringLatin1::equals(&this.value.get(), &aString.value.get())?;
        Ok(_t0!=0i32)
    }

    // java: contentEquals(Ljava/lang/StringBuffer;)Z
    // java: contentEquals(Ljava/lang/StringBuffer;)Z
    pub fn contentEquals__string(&self, sb: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.contentEquals(sb)?;
        Ok(_t0)
    }

    // java: nonSyncContentEquals(Ljava/lang/AbstractStringBuilder;)Z
    pub fn nonSyncContentEquals(&self, sb: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.length()?;
        let mut len: i32 = _t0;
        let _t1 = sb.length()?;
        return Ok(0i32);
        let mut v1: Vec<i8> = this.value.get();
        let _t2 = sb.getValue()?;
        let mut v2: Vec<i8> = _t2;
        let _t3 = this.coder()?;
        let mut coder: i32 = _t3;
        let _t4 = sb.getCoder()?;
        let _t5: i32 = ArraysSupport::mismatch(&v1, &v2, (v1.len() as i32))?;
        return Ok(_t5<0i32);
        return Ok(0i32);
        let _t6: bool = StringUTF16::contentEquals(&v1, &v2, len)?;
        Ok(_t6)
    }

    // java: contentEquals(Ljava/lang/CharSequence;)Z
    // java: contentEquals(Ljava/lang/CharSequence;)Z
    pub fn contentEquals__seq(&self, cs: Object) -> Result<bool> {
        let this = self;
        let mut n: Object = cs;
        /* TODO: monitorenter  */
        let _t0 = this.nonSyncContentEquals(cs)?;
        /* TODO: monitorexit  */
        return Ok(n);
        let mut val: bool = _t0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = this.nonSyncContentEquals(cs)?;
        return Ok(_t1);
        let _t2 = this.equals(cs)?;
        return Ok(_t2);
        let _t3 = cs.length()?;
        n = _t3;
        let _t4 = this.length()?;
        return Ok(0i32);
        val = this.value.get();
        let _t5 = this.isLatin1()?;
        let mut i: i32 = 0i32;
        loop {
            if i >= n { break; }
            let _t0 = cs.charAt(i)?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        let _t6: bool = StringUTF16::contentEquals(val, cs, n)?;
        return Ok(0i32);
        Ok(1i32)
    }

    // java: equalsIgnoreCase(Ljava/lang/String;)Z
    pub fn equalsIgnoreCase(&self, anotherString: String) -> Result<bool> {
        let this = self;
        let _t0 = anotherString.length()?;
        let _t1 = this.length()?;
        let _t2 = this.length()?;
        let _t3 = this.regionMatches(1i32, 0i32, anotherString, 0i32, _t2)?;
        Ok(_t3!=0i32)
    }

    // java: compareTo(Ljava/lang/String;)I
    pub fn compareTo(&self, anotherString: String) -> Result<i32> {
        let this = self;
        let mut v1: Vec<i8> = this.value.get();
        let mut v2: Vec<i8> = anotherString.value.get();
        let _t0 = this.coder()?;
        let mut coder: i32 = _t0;
        let _t1 = anotherString.coder()?;
        let _t2: i32 = StringLatin1::compareTo(&v1, &v2)?;
        let _t3: i32 = StringUTF16::compareTo(&v1, &v2)?;
        return Ok(_t3);
        let _t4: i32 = StringLatin1::compareToUTF16(&v1, &v2)?;
        let _t5: i32 = StringUTF16::compareToLatin1(&v1, &v2)?;
        Ok(_t5)
    }

    // java: compareToIgnoreCase(Ljava/lang/String;)I
    pub fn compareToIgnoreCase(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = String::CASE_INSENSITIVE_ORDER().compare(this, str)?;
        Ok(_t0)
    }

    // java: regionMatches(ILjava/lang/String;II)Z
    // java: regionMatches(ILjava/lang/String;II)Z
    pub fn regionMatches__i_str_i_i(&self, toffset: i32, other: String, ooffset: i32, len: i32) -> Result<bool> {
        let this = self;
        let _t0 = this.length()?;
        /* TODO: lcmp  */
        let _t1 = other.length()?;
        /* TODO: lcmp  */
        return Ok(0i32);
        return Ok(1i32);
        let mut tv: Vec<i8> = this.value.get();
        let mut ov: Vec<i8> = other.value.get();
        let _t2 = this.coder()?;
        let mut coder: i32 = _t2;
        let _t3 = other.coder()?;
        toffset = (toffset<<(1i32&0x1f));
        ooffset = (ooffset<<(1i32&0x1f));
        len = (len<<(1i32&0x1f));
        let _t4: i32 = ArraysSupport::mismatch(&tv, toffset, &ov, ooffset, len)?;
        return Ok(_t4<0i32);
        len = len.wrapping_sub(1i32);
        toffset = toffset.wrapping_add(1i32);
        let _t5: u16 = StringLatin1::getChar(&tv, toffset)?;
        ooffset = ooffset.wrapping_add(1i32);
        let _t6: u16 = StringUTF16::getChar(&ov, ooffset)?;
        return Ok(0i32);
        len = len.wrapping_sub(1i32);
        toffset = toffset.wrapping_add(1i32);
        let _t7: u16 = StringUTF16::getChar(&tv, toffset)?;
        ooffset = ooffset.wrapping_add(1i32);
        let _t8: u16 = StringLatin1::getChar(&ov, ooffset)?;
        return Ok(0i32);
        Ok(1i32)
    }

    // java: regionMatches(ZILjava/lang/String;II)Z
    // java: regionMatches(ZILjava/lang/String;II)Z
    pub fn regionMatches__z_i_str_i_i(&self, ignoreCase: bool, toffset: i32, other: String, ooffset: i32, len: i32) -> Result<bool> {
        let this = self;
        let _t0 = this.regionMatches(toffset, other, ooffset, len)?;
        return Ok(_t0);
        let _t1 = this.length()?;
        /* TODO: lcmp  */
        let _t2 = other.length()?;
        /* TODO: lcmp  */
        return Ok(0i32);
        let mut tv: Vec<i8> = this.value.get();
        let mut ov: Vec<i8> = other.value.get();
        let _t3 = this.coder()?;
        let mut coder: i32 = _t3;
        let _t4 = other.coder()?;
        let _t5: bool = StringLatin1::regionMatchesCI(&tv, toffset, &ov, ooffset, len)?;
        let _t6: bool = StringUTF16::regionMatchesCI(&tv, toffset, &ov, ooffset, len)?;
        return Ok(_t6);
        let _t7: bool = StringLatin1::regionMatchesCI_UTF16(&tv, toffset, &ov, ooffset, len)?;
        let _t8: bool = StringUTF16::regionMatchesCI_Latin1(&tv, toffset, &ov, ooffset, len)?;
        Ok(_t8)
    }

    // java: startsWith(Ljava/lang/String;I)Z
    // java: startsWith(Ljava/lang/String;I)Z
    pub fn startsWith__str_i(&self, prefix: String, toffset: i32) -> Result<bool> {
        let this = self;
        let _t0 = this.length()?;
        let _t1 = prefix.length()?;
        return Ok(0i32);
        let mut ta: Vec<i8> = this.value.get();
        let mut pa: Vec<i8> = prefix.value.get();
        let mut po: i32 = 0i32;
        let mut pc: i32 = (pa.len() as i32);
        let _t2 = this.coder()?;
        let mut coder: i32 = _t2;
        let _t3 = prefix.coder()?;
        toffset = (toffset<<(1i32&0x1f));
        let _t4: i32 = ArraysSupport::mismatch(&ta, toffset, &pa, 0i32, pc)?;
        return Ok(_t4<0i32);
        return Ok(0i32);
        toffset = toffset.wrapping_add(1i32);
        let _t5: u16 = StringUTF16::getChar(&ta, toffset)?;
        po = po.wrapping_add(1i32);
        return Ok(0i32);
        Ok(1i32)
    }

    // java: startsWith(Ljava/lang/String;)Z
    // java: startsWith(Ljava/lang/String;)Z
    pub fn startsWith__str(&self, prefix: String) -> Result<bool> {
        let this = self;
        let _t0 = this.startsWith(prefix, 0i32)?;
        Ok(_t0)
    }

    // java: endsWith(Ljava/lang/String;)Z
    pub fn endsWith(&self, suffix: String) -> Result<bool> {
        let this = self;
        let _t0 = this.length()?;
        let _t1 = suffix.length()?;
        let _t2 = this.startsWith(suffix, (_t0).wrapping_sub(_t1))?;
        Ok(_t2)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut h: i32 = this.hash.get();
        let _t0 = this.isLatin1()?;
        let _t1: i32 = StringLatin1::hashCode(&this.value.get())?;
        let _t2: i32 = StringUTF16::hashCode(&this.value.get())?;
        h = _t2;
        this.hashIsZero.set(1i32);
        this.hash.set(h);
        Ok(h)
    }

    // java: indexOf(I)I
    // java: indexOf(I)I
    pub fn indexOf__i(&self, ch: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.indexOf(ch, 0i32)?;
        Ok(_t0)
    }

    // java: indexOf(II)I
    // java: indexOf(II)I
    pub fn indexOf__i_i(&self, ch: i32, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1 = this.length()?;
        let _t2: i32 = StringLatin1::indexOf(&this.value.get(), ch, fromIndex, _t1)?;
        let _t3 = this.length()?;
        let _t4: i32 = StringUTF16::indexOf(&this.value.get(), ch, fromIndex, _t3)?;
        Ok(_t4)
    }

    // java: indexOf(III)I
    // java: indexOf(III)I
    pub fn indexOf__i_i_i(&self, ch: i32, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.length()?;
        String::checkBoundsBeginEnd(beginIndex, endIndex, _t0)?;
        let _t1 = this.isLatin1()?;
        let _t2: i32 = StringLatin1::indexOf(&this.value.get(), ch, beginIndex, endIndex)?;
        let _t3: i32 = StringUTF16::indexOf(&this.value.get(), ch, beginIndex, endIndex)?;
        Ok(_t3)
    }

    // java: lastIndexOf(I)I
    // java: lastIndexOf(I)I
    pub fn lastIndexOf__i(&self, ch: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.length()?;
        let _t1 = this.lastIndexOf(ch, (_t0).wrapping_sub(1i32))?;
        Ok(_t1)
    }

    // java: lastIndexOf(II)I
    // java: lastIndexOf(II)I
    pub fn lastIndexOf__i_i(&self, ch: i32, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: i32 = StringLatin1::lastIndexOf(&this.value.get(), ch, fromIndex)?;
        let _t2: i32 = StringUTF16::lastIndexOf(&this.value.get(), ch, fromIndex)?;
        Ok(_t2)
    }

    // java: indexOf(Ljava/lang/String;)I
    // java: indexOf(Ljava/lang/String;)I
    pub fn indexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = this.coder()?;
        let mut coder: i32 = _t0;
        let _t1 = str.coder()?;
        let _t2 = this.isLatin1()?;
        let _t3: i32 = StringLatin1::indexOf(&this.value.get(), &str.value.get())?;
        let _t4: i32 = StringUTF16::indexOf(&this.value.get(), &str.value.get())?;
        return Ok(_t4);
        return Ok(-1i32);
        let _t5: i32 = StringUTF16::indexOfLatin1(&this.value.get(), &str.value.get())?;
        Ok(_t5)
    }

    // java: indexOf(Ljava/lang/String;I)I
    // java: indexOf(Ljava/lang/String;I)I
    pub fn indexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.coder()?;
        let _t1 = this.length()?;
        let _t2: i32 = String::indexOf(&this.value.get(), _t0, _t1, str, fromIndex)?;
        Ok(_t2)
    }

    // java: indexOf(Ljava/lang/String;II)I
    // java: indexOf(Ljava/lang/String;II)I
    pub fn indexOf__str_i_i(&self, str: String, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = str.length()?;
        let _t1 = str.charAt(0i32)?;
        let _t2 = this.indexOf(_t1, beginIndex, endIndex)?;
        return Ok(_t2);
        let _t3 = this.length()?;
        String::checkBoundsBeginEnd(beginIndex, endIndex, _t3)?;
        let _t4 = this.coder()?;
        let _t5: i32 = String::indexOf(&this.value.get(), _t4, endIndex, str, beginIndex)?;
        Ok(_t5)
    }

    // java: indexOf([BBILjava/lang/String;I)I
    // java: indexOf([BBILjava/lang/String;I)I
    pub fn indexOf__arr_b_b_i_str_i(src: &[i8], srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
        let _t0: i32 = ((fromIndex as i64)).abs();
        fromIndex = _t0;
        let _t1 = tgtStr.length()?;
        let mut tgtCount: i32 = _t1;
        return Ok(-1i32);
        return Ok(fromIndex);
        let mut tgt: Vec<i8> = tgtStr.value.get();
        let _t2 = tgtStr.coder()?;
        let mut tgtCoder: i32 = _t2;
        let _t3: i32 = StringLatin1::indexOf(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        let _t4: i32 = StringUTF16::indexOf(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        return Ok(_t4);
        return Ok(-1i32);
        let _t5: i32 = StringUTF16::indexOfLatin1(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        Ok(_t5)
    }

    // java: lastIndexOf(Ljava/lang/String;)I
    // java: lastIndexOf(Ljava/lang/String;)I
    pub fn lastIndexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = this.length()?;
        let _t1 = this.lastIndexOf(str, _t0)?;
        Ok(_t1)
    }

    // java: lastIndexOf(Ljava/lang/String;I)I
    // java: lastIndexOf(Ljava/lang/String;I)I
    pub fn lastIndexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.coder()?;
        let _t1 = this.length()?;
        let _t2: i32 = String::lastIndexOf(&this.value.get(), _t0, _t1, str, fromIndex)?;
        Ok(_t2)
    }

    // java: lastIndexOf([BBILjava/lang/String;I)I
    // java: lastIndexOf([BBILjava/lang/String;I)I
    pub fn lastIndexOf__arr_b_b_i_str_i(src: &[i8], srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
        let mut tgt: Vec<i8> = tgtStr.value.get();
        let _t0 = tgtStr.coder()?;
        let mut tgtCoder: i32 = _t0;
        let _t1 = tgtStr.length()?;
        let mut tgtCount: i32 = _t1;
        let mut rightIndex: i32 = (srcCount).wrapping_sub(tgtCount);
        fromIndex = rightIndex;
        return Ok(-1i32);
        return Ok(fromIndex);
        let _t2: i32 = StringLatin1::lastIndexOf(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        let _t3: i32 = StringUTF16::lastIndexOf(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        return Ok(_t3);
        return Ok(-1i32);
        let _t4: i32 = StringUTF16::lastIndexOfLatin1(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        Ok(_t4)
    }

    // java: substring(I)Ljava/lang/String;
    // java: substring(I)Ljava/lang/String;
    pub fn substring__i(&self, beginIndex: i32) -> Result<String> {
        let this = self;
        let _t0 = this.length()?;
        let _t1 = this.substring(beginIndex, _t0)?;
        Ok(_t1)
    }

    // java: substring(II)Ljava/lang/String;
    // java: substring(II)Ljava/lang/String;
    pub fn substring__i_i(&self, beginIndex: i32, endIndex: i32) -> Result<String> {
        let this = self;
        let _t0 = this.length()?;
        let mut length: i32 = _t0;
        String::checkBoundsBeginEnd(beginIndex, endIndex, length)?;
        return Ok(this);
        let mut subLen: i32 = (endIndex).wrapping_sub(beginIndex);
        let _t1 = this.isLatin1()?;
        let _t2: String = StringLatin1::newString(&this.value.get(), beginIndex, subLen)?;
        let _t3: String = StringUTF16::newString(&this.value.get(), beginIndex, subLen)?;
        Ok(_t3)
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, beginIndex: i32, endIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.substring(beginIndex, endIndex)?;
        Ok(_t0)
    }

    // java: concat(Ljava/lang/String;)Ljava/lang/String;
    pub fn concat(&self, str: String) -> Result<String> {
        let this = self;
        let _t0 = str.isEmpty()?;
        return Ok(this);
        let _t1: String = StringConcatHelper::simpleConcat(this, str)?;
        Ok(_t1)
    }

    // java: replace(CC)Ljava/lang/String;
    // java: replace(CC)Ljava/lang/String;
    pub fn replace__c_c(&self, oldChar: u16, newChar: u16) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::replace(&this.value.get(), oldChar, newChar)?;
        let _t2: String = StringUTF16::replace(&this.value.get(), oldChar, newChar)?;
        let mut ret: String = _t2;
        return Ok(ret);
        Ok(this)
    }

    // java: matches(Ljava/lang/String;)Z
    pub fn matches(&self, regex: String) -> Result<bool> {
        let this = self;
        let _t0: bool = Pattern::matches(regex, this)?;
        Ok(_t0)
    }

    // java: contains(Ljava/lang/CharSequence;)Z
    pub fn contains(&self, s: Object) -> Result<bool> {
        let this = self;
        let _t0 = s.toString()?;
        let _t1 = this.indexOf(_t0)?;
        Ok(_t1>=0i32)
    }

    // java: replaceFirst(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn replaceFirst(&self, regex: String, replacement: String) -> Result<String> {
        let this = self;
        let _t0: Object = Pattern::compile(regex)?;
        let _t1 = _t0.matcher(this)?;
        let _t2 = _t1.replaceFirst(replacement)?;
        Ok(_t2)
    }

    // java: replaceAll(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn replaceAll(&self, regex: String, replacement: String) -> Result<String> {
        let this = self;
        let _t0: Object = Pattern::compile(regex)?;
        let _t1 = _t0.matcher(this)?;
        let _t2 = _t1.replaceAll(replacement)?;
        Ok(_t2)
    }

    // java: replace(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;
    // java: replace(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;
    pub fn replace__seq_seq(&self, target: Object, replacement: Object) -> Result<String> {
        let this = self;
        let _t0 = target.toString()?;
        let mut trgtStr: String = _t0;
        let _t1 = replacement.toString()?;
        let mut replStr: String = _t1;
        let _t2 = this.length()?;
        let mut thisLen: i32 = _t2;
        let _t3 = trgtStr.length()?;
        let mut trgtLen: i32 = _t3;
        let _t4 = replStr.length()?;
        let mut replLen: i32 = _t4;
        let _t5 = trgtStr.charAt(0i32)?;
        let _t6 = replStr.charAt(0i32)?;
        let _t7 = this.replace(_t5, _t6)?;
        return Ok(_t7);
        let _t8 = this.isLatin1()?;
        let mut thisIsLatin1: i32 = _t8;
        let _t9 = trgtStr.isLatin1()?;
        let mut trgtIsLatin1: i32 = _t9;
        let _t10 = replStr.isLatin1()?;
        let mut replIsLatin1: i32 = _t10;
        let _t11: String = StringLatin1::replace(&this.value.get(), thisLen, &trgtStr.value.get(), trgtLen, &replStr.value.get(), replLen)?;
        let _t12: String = StringUTF16::replace(&this.value.get(), thisLen, thisIsLatin1, &trgtStr.value.get(), trgtLen, trgtIsLatin1, &replStr.value.get(), replLen, replIsLatin1)?;
        let mut ret: String = _t12;
        return Ok(ret);
        return Ok(this);
        let _t13: i32 = (thisLen).abs();
        let _t14: i32 = (_t13).abs();
        let _t15: i32 = (thisLen).abs();
        thisIsLatin1 = _t15;
        trgtIsLatin1 = ret;
        return Err(JvmError::Custom("athrow".to_owned()));
        trgtIsLatin1 = String::new();
        trgtIsLatin1.append(&replStr)?;
        replIsLatin1 = 0i32;
        loop {
            if replIsLatin1 >= thisLen { break; }
            let _t0 = this.charAt(replIsLatin1)?;
            trgtIsLatin1.append(&_t0)?;
            trgtIsLatin1.append(&replStr)?;
            replIsLatin1 = replIsLatin1.wrapping_add(1i32);
        }
        Ok(trgtIsLatin1)
    }

    // java: split(Ljava/lang/String;I)[Ljava/lang/String;
    // java: split(Ljava/lang/String;I)[Ljava/lang/String;
    pub fn split__str_i(&self, regex: String, limit: i32) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(regex, limit, 0i32)?;
        Ok(_t0)
    }

    // java: splitWithDelimiters(Ljava/lang/String;I)[Ljava/lang/String;
    pub fn splitWithDelimiters(&self, regex: String, limit: i32) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(regex, limit, 1i32)?;
        Ok(_t0)
    }

    // java: split(Ljava/lang/String;IZ)[Ljava/lang/String;
    // java: split(Ljava/lang/String;IZ)[Ljava/lang/String;
    pub fn split__str_i_z(&self, regex: String, limit: i32, withDelimiters: bool) -> Result<Vec<String>> {
        let this = self;
        let mut ch: i32 = 0i32;
        let _t0 = regex.length()?;
        let _t1 = regex.charAt(0i32)?;
        ch = _t1;
        let _t2 = String::from(".$|()[{^?*+\\").indexOf(_t1)?;
        let _t3 = regex.length()?;
        let _t4 = regex.charAt(0i32)?;
        let _t5 = regex.charAt(1i32)?;
        ch = _t5;
        let _t6 = this.split(ch, limit, withDelimiters)?;
        return Ok(_t6);
        let _t7: Object = Pattern::compile(regex)?;
        let mut pattern: Object = _t7;
        let _t8 = pattern.splitWithDelimiters(this, limit)?;
        let _t9 = pattern.split(this, limit)?;
        Ok(_t9)
    }

    // java: split(CIZ)[Ljava/lang/String;
    // java: split(CIZ)[Ljava/lang/String;
    pub fn split__c_i_z(&self, ch: u16, limit: i32, withDelimiters: bool) -> Result<Vec<String>> {
        let this = self;
        let mut matchCount: i32 = 0i32;
        let mut off: i32 = 0i32;
        let mut limited: i32 = limit>0i32;
        let mut list: ArrayList<_> = ArrayList::<_>::new()?;
        /* TODO: aconst_null  */
        let mut del: String = String::from_owned(format!("{}", ch));
        loop {
            let _t0 = this.indexOf(ch, off)?;
            let mut next: i32 = _t0;
            if _t0 == -1i32 { break; }
            let _t0 = this.substring(off, next)?;
            let _t1 = list.add(_t0)?;
            let _t2 = list.add(del)?;
            off = (next).wrapping_add(1i32);
            matchCount = matchCount.wrapping_add(1i32);
        }
        let _t0 = this.length()?;
        let mut last: i32 = _t0;
        let _t1 = this.substring(off, last)?;
        let _t2 = list.add(_t1)?;
        off = last;
        matchCount = matchCount.wrapping_add(1i32);
        let mut _arr3: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr3[0i32 as usize] = this;
        return Ok(_arr3);
        let _t4 = this.length()?;
        let _t5 = this.substring(off, _t4)?;
        let _t6 = list.add(_t5)?;
        let _t7 = list.size()?;
        last = _t7;
        loop {
            if last<=0i32 { break; }
            let _t0 = list.get((last).wrapping_sub(1i32))?;
            let _t1 = _t0.isEmpty()?;
            last = last.wrapping_sub(1i32);
        }
        let mut _arr8: Vec<Object> = Vec::with_capacity(last as usize);
        let mut result: Vec<Object> = _arr8;
        let _t9 = list.subList(0i32, last)?;
        let _t10 = _t9.toArray(result)?;
        Ok(_t10)
    }

    // java: split(Ljava/lang/String;)[Ljava/lang/String;
    // java: split(Ljava/lang/String;)[Ljava/lang/String;
    pub fn split__str(&self, regex: String) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(regex, 0i32, 0i32)?;
        Ok(_t0)
    }

    // java: join(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;
    // java: join(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;
    pub fn join__seq_arr_seq(delimiter: Object, elements: &[Object]) -> Result<String> {
        let _t0 = delimiter.toString()?;
        let mut delim: String = _t0;
        let mut _arr1: Vec<Object> = Vec::with_capacity((elements.len() as i32) as usize);
        let mut elems: Vec<Object> = _arr1;
        let mut i: i32 = 0i32;
        loop {
            if i >= (elements.len() as i32) { break; }
            elems[i as usize] = String::from_owned(format!("{}", elements[i as usize].clone()));
            i = i.wrapping_add(1i32);
        }
        let _t2: String = String::join(String::from(""), String::from(""), delim, &elems, (elems.len() as i32))?;
        Ok(_t2)
    }

    // java: join(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;
    // java: join(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;
    pub fn join__str_str_str_arr_str_i(prefix: String, suffix: String, delimiter: String, elements: &[String], size: i32) -> Result<String> {
        let _t0 = prefix.coder()?;
        let _t1 = suffix.coder()?;
        let mut icoder: i32 = (_t0|_t1);
        let _t2 = prefix.length()?;
        let _t3 = suffix.length()?;
        let mut len: i64 = ((_t2 as i64)).wrapping_add((_t3 as i64));
        let _t4 = delimiter.length()?;
        len = (len).wrapping_add((((size).wrapping_sub(1i32) as i64)).wrapping_mul((_t4 as i64)));
        let _t5 = delimiter.coder()?;
        icoder = (icoder|_t5);
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let mut el: String = elements[i as usize].clone();
            let _t0 = el.length()?;
            len = (len).wrapping_add((_t0 as i64));
            let _t1 = el.coder()?;
            icoder = (icoder|_t1);
            i = i.wrapping_add(1i32);
        }
        /* TODO: i2b  */
        i = icoder;
        /* TODO: lcmp  */
        /* TODO: lshl  */
        /* TODO: dup2  */
        len = i;
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t6: Vec<i8> = StringConcatHelper::newArray(len)?;
        el = _t6;
        let mut off: i32 = 0i32;
        prefix.getBytes(el, off, i)?;
        let _t7 = prefix.length()?;
        off = (off).wrapping_add(_t7);
        let mut el: String = elements[0i32 as usize].clone();
        el.getBytes(el, off, i)?;
        let _t8 = el.length()?;
        off = (off).wrapping_add(_t8);
        let mut i: i32 = 1i32;
        loop {
            if i >= size { break; }
            delimiter.getBytes(el, off, i)?;
            let _t0 = delimiter.length()?;
            off = (off).wrapping_add(_t0);
            el = elements[i as usize].clone();
            el.getBytes(el, off, i)?;
            let _t1 = el.length()?;
            off = (off).wrapping_add(_t1);
            i = i.wrapping_add(1i32);
        }
        suffix.getBytes(el, off, i)?;
        Ok(String::new(el, i)?)
    }

    // java: join(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;
    // java: join(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;
    pub fn join__seq_iter(delimiter: Object, elements: Object) -> Result<String> {
        let _t0: Object = Objects::requireNonNull(delimiter)?;
        let _t1: Object = Objects::requireNonNull(elements)?;
        let _t2 = delimiter.toString()?;
        let mut delim: String = _t2;
        let mut _arr3: Vec<Object> = Vec::with_capacity(8i32 as usize);
        let mut elems: Vec<Object> = _arr3;
        let mut size: i32 = 0i32;
        let _t4 = elements.iterator()?;
        let mut local_5: Object = _t4;
        loop {
            let _t0 = local_5.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_5.next()?;
            let mut cs: Object = _t0;
            let _t1: Vec<Object> = Arrays::copyOf(&elems, ((elems.len() as i32)<<(1i32&0x1f)))?;
            elems = _t1;
            size = size.wrapping_add(1i32);
            elems[size as usize] = String::from_owned(format!("{}", cs));
        }
        let _t5: String = String::join(String::from(""), String::from(""), delim, &elems, size)?;
        Ok(_t5)
    }

    // java: toLowerCase(Ljava/util/Locale;)Ljava/lang/String;
    // java: toLowerCase(Ljava/util/Locale;)Ljava/lang/String;
    pub fn toLowerCase__locale(&self, locale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::toLowerCase(this, &this.value.get(), locale)?;
        let _t2: String = StringUTF16::toLowerCase(this, &this.value.get(), locale)?;
        Ok(_t2)
    }

    // java: toLowerCase()Ljava/lang/String;
    // java: toLowerCase()Ljava/lang/String;
    pub fn toLowerCase(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault()?;
        let _t1 = this.toLowerCase(_t0)?;
        Ok(_t1)
    }

    // java: toUpperCase(Ljava/util/Locale;)Ljava/lang/String;
    // java: toUpperCase(Ljava/util/Locale;)Ljava/lang/String;
    pub fn toUpperCase__locale(&self, locale: Object) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::toUpperCase(this, &this.value.get(), locale)?;
        let _t2: String = StringUTF16::toUpperCase(this, &this.value.get(), locale)?;
        Ok(_t2)
    }

    // java: toUpperCase()Ljava/lang/String;
    // java: toUpperCase()Ljava/lang/String;
    pub fn toUpperCase(&self) -> Result<String> {
        let this = self;
        let _t0: Object = Locale::getDefault()?;
        let _t1 = this.toUpperCase(_t0)?;
        Ok(_t1)
    }

    // java: trim()Ljava/lang/String;
    pub fn trim(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::trim(&this.value.get())?;
        let _t2: String = StringUTF16::trim(&this.value.get())?;
        let mut ret: String = _t2;
        Ok(ret)
    }

    // java: strip()Ljava/lang/String;
    pub fn strip(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::strip(&this.value.get())?;
        let _t2: String = StringUTF16::strip(&this.value.get())?;
        let mut ret: String = _t2;
        Ok(ret)
    }

    // java: stripLeading()Ljava/lang/String;
    pub fn stripLeading(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::stripLeading(&this.value.get())?;
        let _t2: String = StringUTF16::stripLeading(&this.value.get())?;
        let mut ret: String = _t2;
        Ok(ret)
    }

    // java: stripTrailing()Ljava/lang/String;
    pub fn stripTrailing(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: String = StringLatin1::stripTrailing(&this.value.get())?;
        let _t2: String = StringUTF16::stripTrailing(&this.value.get())?;
        let mut ret: String = _t2;
        Ok(ret)
    }

    // java: isBlank()Z
    pub fn isBlank(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOfNonWhitespace()?;
        let _t1 = this.length()?;
        Ok(_t0 == _t1)
    }

    // java: lines()Ljava/util/stream/Stream;
    pub fn lines(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: Object = StringLatin1::lines(&this.value.get())?;
        let _t2: Object = StringUTF16::lines(&this.value.get())?;
        Ok(_t2)
    }

    // java: indent(I)Ljava/lang/String;
    pub fn indent(&self, n: i32) -> Result<String> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Ok(String::from(""));
        let _t1 = this.lines()?;
        let mut stream: Object = _t1;
        let _t2 = String::from("").repeat(n)?;
        let mut spaces: String = _t2;
        /* TODO: invokedynamic 893 */
        let _t3 = stream.map(spaces)?;
        stream = _t3;
        /* TODO: invokedynamic 906 */
        let _t4 = 905i32.map(stream)?;
        stream = _t4;
        /* TODO: invokedynamic 909 */
        let _t5 = stream.map(n)?;
        stream = _t5;
        let _t6: Object = Collectors::joining(String::from(""), String::from(""), String::from(""))?;
        let _t7 = stream.collect(_t6)?;
        Ok(_t7)
    }

    // java: indexOfNonWhitespace()I
    pub fn indexOfNonWhitespace(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: i32 = StringLatin1::indexOfNonWhitespace(&this.value.get())?;
        let _t2: i32 = StringUTF16::indexOfNonWhitespace(&this.value.get())?;
        Ok(_t2)
    }

    // java: lastIndexOfNonWhitespace()I
    pub fn lastIndexOfNonWhitespace(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: i32 = StringLatin1::lastIndexOfNonWhitespace(&this.value.get())?;
        let _t2: i32 = StringUTF16::lastIndexOfNonWhitespace(&this.value.get())?;
        Ok(_t2)
    }

    // java: stripIndent()Ljava/lang/String;
    pub fn stripIndent(&self) -> Result<String> {
        let this = self;
        let _t0 = this.length()?;
        let mut length: i32 = _t0;
        return Ok(String::from(""));
        let _t1 = this.charAt((length).wrapping_sub(1i32))?;
        let mut lastChar: i32 = _t1;
        let mut optOut: i32 = lastChar == 13i32;
        let _t2 = this.lines()?;
        let _t3 = _t2.toList()?;
        let mut lines: Object = _t3;
        let _t4: i32 = String::outdent(lines)?;
        let mut outdent: i32 = _t4;
        let _t5 = lines.stream()?;
        /* TODO: invokedynamic 942 */
        let _t6 = _t5.map(outdent)?;
        let _t7: Object = Collectors::joining(optOut, String::from(""), String::from(""))?;
        let _t8 = String::from("").collect(_t7)?;
        Ok(_t8)
    }

    // java: outdent(Ljava/util/List;)I
    pub fn outdent(lines: Object) -> Result<i32> {
        let mut outdent: i32 = 943i32;
        let _t0 = lines.iterator()?;
        let mut lastLine: Object = _t0;
        loop {
            let _t0 = lastLine.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = lastLine.next()?;
            let mut line: Object = _t0;
            let _t1 = line.indexOfNonWhitespace()?;
            let mut leadingWhitespace: i32 = _t1;
            let _t2 = line.length()?;
            let _t3: i32 = Integer::min(outdent, leadingWhitespace)?;
            outdent = _t3;
        }
        let _t1 = lines.size()?;
        let _t2 = lines.get((_t1).wrapping_sub(1i32))?;
        lastLine = _t2;
        let _t3 = lastLine.isBlank()?;
        let _t4 = lastLine.length()?;
        let _t5: i32 = Integer::min(outdent, _t4)?;
        outdent = _t5;
        Ok(outdent)
    }

    // java: translateEscapes()Ljava/lang/String;
    pub fn translateEscapes(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isEmpty()?;
        return Ok(String::from(""));
        let _t1 = this.toCharArray()?;
        let mut chars: Vec<u16> = _t1;
        let mut length: i32 = (chars.len() as i32);
        let mut from: i32 = 0i32;
        let mut to: i32 = 0i32;
        loop {
            if from >= length { break; }
            from = from.wrapping_add(1i32);
            let mut ch: i32 = chars[from as usize];
            from = from.wrapping_add(1i32);
            ch = 0i32;
            /* TODO: lookupswitch default:372 10:350 13:353 34:266 39:266 48:269 49:269 50:269 51:269 52:269 53:269 54:269 55:269 92:266 98:224 102:231 110:238 114:245 115:252 116:259 */
            ch = 8i32;
            ch = 12i32;
            ch = 10i32;
            ch = 13i32;
            ch = 32i32;
            ch = 9i32;
            let _t0: i32 = Integer::min((2i32).wrapping_add(1i32), length)?;
            let mut limit: i32 = _t0;
            let mut code: i32 = (ch).wrapping_sub(48i32);
            ch = chars[from as usize];
            from = from.wrapping_add(1i32);
            code = ((code<<(3i32&0x1f))|(ch).wrapping_sub(48i32));
            /* TODO: i2c  */
            ch = code;
            from = from.wrapping_add(1i32);
            let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
            let _t2: Object = Character::valueOf(ch)?;
            _arr1[0i32 as usize] = _t2;
            _arr1[1i32 as usize] = ch;
            let _t3: String = String::format(String::from("Invalid escape sequence: \\%c \\u%04X"), &_arr1)?;
            let mut msg: String = _t3;
            return Err(JvmError::Custom("athrow".to_owned()));
            to = to.wrapping_add(1i32);
            chars[to as usize] = ch;
        }
        Ok(String::new(chars, 0i32, to)?)
    }

    // java: transform(Ljava/util/function/Function;)Ljava/lang/Object;
    pub fn transform(&self, f: Object) -> Result<Object> {
        let this = self;
        let _t0 = f.apply(this)?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(this)
    }

    // java: chars()Ljava/util/stream/IntStream;
    pub fn chars(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: Object = StreamSupport::intStream(StringUTF16_CharsSpliterator::new(this.value.get(), 1024i32)?, 0i32)?;
        Ok(_t1)
    }

    // java: codePoints()Ljava/util/stream/IntStream;
    pub fn codePoints(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: Object = StreamSupport::intStream(StringUTF16_CodePointsSpliterator::new(this.value.get(), 1024i32)?, 0i32)?;
        Ok(_t1)
    }

    // java: toCharArray()[C
    pub fn toCharArray(&self) -> Result<Vec<u16>> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1: Vec<u16> = StringLatin1::toChars(&this.value.get())?;
        let _t2: Vec<u16> = StringUTF16::toChars(&this.value.get())?;
        Ok(_t2)
    }

    // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
    // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
    pub fn format__str_arr_obj(format: String, args: &[Object]) -> Result<String> {
        let _t0 = Formatter::new()?.format(format, args)?;
        let _t1 = _t0.toString()?;
        Ok(_t1)
    }

    // java: format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
    // java: format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
    pub fn format__locale_str_arr_obj(l: Object, format: String, args: &[Object]) -> Result<String> {
        let _t0 = Formatter::new(l)?.format(format, args)?;
        let _t1 = _t0.toString()?;
        Ok(_t1)
    }

    // java: formatted([Ljava/lang/Object;)Ljava/lang/String;
    pub fn formatted(&self, args: Vec<Object>) -> Result<String> {
        let this = self;
        let _t0 = Formatter::new()?.format(this, args)?;
        let _t1 = _t0.toString()?;
        Ok(_t1)
    }

    // java: valueOf(Ljava/lang/Object;)Ljava/lang/String;
    // java: valueOf(Ljava/lang/Object;)Ljava/lang/String;
    pub fn valueOf__obj(obj: Object) -> Result<String> {
        let _t0 = obj.toString()?;
        Ok(_t0)
    }

    // java: valueOf([C)Ljava/lang/String;
    // java: valueOf([C)Ljava/lang/String;
    pub fn valueOf__arr_c(data: &[u16]) -> Result<String> {
        Ok(String::new(data)?)
    }

    // java: valueOf([CII)Ljava/lang/String;
    // java: valueOf([CII)Ljava/lang/String;
    pub fn valueOf__arr_c_i_i(data: &[u16], offset: i32, count: i32) -> Result<String> {
        Ok(String::new(data, offset, count)?)
    }

    // java: copyValueOf([CII)Ljava/lang/String;
    // java: copyValueOf([CII)Ljava/lang/String;
    pub fn copyValueOf__arr_c_i_i(data: &[u16], offset: i32, count: i32) -> Result<String> {
        Ok(String::new(data, offset, count)?)
    }

    // java: copyValueOf([C)Ljava/lang/String;
    // java: copyValueOf([C)Ljava/lang/String;
    pub fn copyValueOf__arr_c(data: &[u16]) -> Result<String> {
        Ok(String::new(data)?)
    }

    // java: valueOf(Z)Ljava/lang/String;
    // java: valueOf(Z)Ljava/lang/String;
    pub fn valueOf__z(b: bool) -> Result<String> {
        Ok(String::from("false"))
    }

    // java: valueOf(C)Ljava/lang/String;
    // java: valueOf(C)Ljava/lang/String;
    pub fn valueOf__c(c: u16) -> Result<String> {
        let _t0: bool = StringLatin1::canEncode(c)?;
        let _t1: Vec<i8> = StringLatin1::toBytes(c)?;
        return Ok(String::new(_t1, 0i32)?);
        let _t2: Vec<i8> = StringUTF16::toBytes(c)?;
        Ok(String::new(_t2, 1i32)?)
    }

    // java: valueOf(I)Ljava/lang/String;
    // java: valueOf(I)Ljava/lang/String;
    pub fn valueOf__i(i: i32) -> Result<String> {
        let _t0: String = Integer::toString(i)?;
        Ok(_t0)
    }

    // java: valueOf(J)Ljava/lang/String;
    // java: valueOf(J)Ljava/lang/String;
    pub fn valueOf__l(l: i64) -> Result<String> {
        let _t0: String = Long::toString(l)?;
        Ok(_t0)
    }

    // java: valueOf(F)Ljava/lang/String;
    // java: valueOf(F)Ljava/lang/String;
    pub fn valueOf__f(f: f32) -> Result<String> {
        let _t0: String = Float::toString(f)?;
        Ok(_t0)
    }

    // java: valueOf(D)Ljava/lang/String;
    // java: valueOf(D)Ljava/lang/String;
    pub fn valueOf__d(d: f64) -> Result<String> {
        let _t0: String = Double::toString(d)?;
        Ok(_t0)
    }

    // java: intern()Ljava/lang/String;
    pub fn intern(&self) -> Result<String> {
        todo!("native java/lang/String.intern")
    }

    // java: repeat(I)Ljava/lang/String;
    pub fn repeat(&self, count: i32) -> Result<String> {
        let this = self;
        String::new().append(&String::from("count is negative:"))?;
        String::new().append(&count)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(this);
        let mut len: i32 = (this.value.get().len() as i32);
        return Ok(String::from(""));
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut _arr0: Vec<i8> = vec![0i8; count as usize];
        let mut single: Vec<i8> = _arr0;
        Arrays::fill(&single, this.value.get()[0i32 as usize])?;
        return Ok(String::new(single, this.coder.get())?);
        single = (len).wrapping_mul(count);
        let mut _arr1: Vec<i8> = vec![0i8; single as usize];
        let mut multiple: Vec<i8> = _arr1;
        System::arraycopy(&this.value.get(), 0i32, &multiple, 0i32, len)?;
        String::repeatCopyRest(&multiple, 0i32, &single, len)?;
        Ok(String::new(multiple, this.coder.get())?)
    }

    // java: repeatCopyRest([BIII)V
    pub fn repeatCopyRest(buffer: &[i8], offset: i32, limit: i32, copied: i32) -> Result<()> {
        loop {
            if copied >= (limit).wrapping_sub(copied) { break; }
            System::arraycopy(&buffer, offset, &buffer, (offset).wrapping_add(copied), copied)?;
            copied = (copied<<(1i32&0x1f));
        }
        System::arraycopy(&buffer, offset, &buffer, (offset).wrapping_add(copied), (limit).wrapping_sub(copied))?;
        Ok(())
    }

    // java: getBytes([BIB)V
    // java: getBytes([BIB)V
    pub fn getBytes__arr_b_i_b(&self, dst: Vec<i8>, dstBegin: i32, coder: i8) -> Result<()> {
        let this = self;
        let _t0 = this.coder()?;
        System::arraycopy(&this.value.get(), 0i32, &dst, (dstBegin<<(coder&0x1f)), (this.value.get().len() as i32))?;
        StringLatin1::inflate(&this.value.get(), 0i32, &dst, dstBegin, (this.value.get().len() as i32))?;
        Ok(())
    }

    // java: getBytes([BIIBI)V
    // java: getBytes([BIIBI)V
    pub fn getBytes__arr_b_i_i_b_i(&self, dst: Vec<i8>, srcPos: i32, dstBegin: i32, coder: i8, length: i32) -> Result<()> {
        let this = self;
        let _t0 = this.coder()?;
        System::arraycopy(&this.value.get(), (srcPos<<(coder&0x1f)), &dst, (dstBegin<<(coder&0x1f)), (length<<(coder&0x1f)))?;
        StringLatin1::inflate(&this.value.get(), srcPos, &dst, dstBegin, length)?;
        Ok(())
    }

    // java: <init>([CIILjava/lang/Void;)V
    // java: <init>([CIILjava/lang/Void;)V
    pub fn new__arr_c_i_i_void(value: Vec<u16>, off: i32, len: i32, sig: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(String::from("").value.get());
        this.coder.set(String::from("").coder.get());
        return Ok(());
        let _t0: Vec<i8> = StringUTF16::compress(&value, off, len)?;
        let mut val: Vec<i8> = _t0;
        let _t1: i8 = StringUTF16::coderFromArrayLen(&val, len)?;
        this.coder.set(_t1);
        this.value.set(val);
        return Ok(());
        this.coder.set(1i32);
        let _t2: Vec<i8> = StringUTF16::toBytes(&value, off, len)?;
        this.value.set(_t2);
        Ok(this)
    }

    // java: <init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V
    // java: <init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V
    pub fn new__abstra_void(asb: Object, sig: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = asb.getValue()?;
        let mut val: Vec<i8> = _t0;
        let _t1 = asb.length()?;
        let mut length: i32 = _t1;
        let _t2 = asb.isLatin1()?;
        this.coder.set(0i32);
        let _t3: Vec<i8> = Arrays::copyOfRange(&val, 0i32, length)?;
        this.value.set(_t3);
        let _t4: Vec<i8> = StringUTF16::compress(&val, 0i32, length)?;
        this.value.set(_t4);
        let _t5: i8 = StringUTF16::coderFromArrayLen(&this.value.get(), length)?;
        this.coder.set(_t5);
        return Ok(());
        this.coder.set(1i32);
        let _t6: Vec<i8> = Arrays::copyOfRange(&val, 0i32, (length<<(1i32&0x1f)))?;
        this.value.set(_t6);
        Ok(this)
    }

    // java: <init>([BB)V
    // java: <init>([BB)V
    pub fn new__arr_b_b(value: Vec<i8>, coder: i8) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), hash: Field::new(0), hashIsZero: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(value);
        this.coder.set(coder);
        Ok(this)
    }

    // java: coder()B
    pub fn coder(&self) -> Result<i8> {
        let this = self;
        Ok(1i32)
    }

    // java: value()[B
    pub fn value(&self) -> Result<Vec<i8>> {
        let this = self;
        Ok(this.value.get())
    }

    // java: isLatin1()Z
    pub fn isLatin1(&self) -> Result<bool> {
        let this = self;
        Ok(this.coder.get()==0i32)
    }

    // java: checkIndex(II)V
    pub fn checkIndex(index: i32, length: i32) -> Result<()> {
        let _t0: i32 = Preconditions::checkIndex(index, length, Preconditions::SIOOBE_FORMATTER())?;
        Ok(())
    }

    // java: checkOffset(II)V
    pub fn checkOffset(offset: i32, length: i32) -> Result<()> {
        let _t0: i32 = Preconditions::checkFromToIndex(offset, length, length, Preconditions::SIOOBE_FORMATTER())?;
        Ok(())
    }

    // java: checkBoundsOffCount(III)I
    pub fn checkBoundsOffCount(offset: i32, count: i32, length: i32) -> Result<i32> {
        let _t0: i32 = Preconditions::checkFromIndexSize(offset, count, length, Preconditions::SIOOBE_FORMATTER())?;
        Ok(_t0)
    }

    // java: checkBoundsBeginEnd(III)V
    pub fn checkBoundsBeginEnd(begin: i32, end: i32, length: i32) -> Result<()> {
        let _t0: i32 = Preconditions::checkFromToIndex(begin, end, length, Preconditions::SIOOBE_FORMATTER())?;
        Ok(())
    }

    // java: valueOfCodePoint(I)Ljava/lang/String;
    pub fn valueOfCodePoint(codePoint: i32) -> Result<String> {
        let _t0: bool = StringLatin1::canEncode(codePoint)?;
        /* TODO: i2c  */
        let _t1: Vec<i8> = StringLatin1::toBytes(codePoint)?;
        return Ok(String::new(_t1, 0i32)?);
        let _t2: bool = Character::isBmpCodePoint(codePoint)?;
        /* TODO: i2c  */
        let _t3: Vec<i8> = StringUTF16::toBytes(codePoint)?;
        return Ok(String::new(_t3, 1i32)?);
        let _t4: bool = Character::isSupplementaryCodePoint(codePoint)?;
        let _t5: Vec<i8> = StringUTF16::toBytesSupplementary(codePoint)?;
        return Ok(String::new(_t5, 1i32)?);
        let mut _arr6: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr6[0i32 as usize] = codePoint;
        let _t7: String = String::format(String::from("Not a valid Unicode code point: 0x%X"), &_arr6)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(this)?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/String;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<String> {
        let this = self;
        Ok(this)
    }
}
