#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Integer",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Integer.java",
))]
pub struct Integer {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private final"))]
    pub value: Field<i32>,
}

impl Integer {
    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(II)Ljava/lang/String;", access = "public static"))]
    pub fn toString(i: i32, radix: i32) -> Result<String> {
        radix = 10i32;
        let _t0: String = Integer::toString(i)?;
        return Ok(_t0);
        let mut _arr1: Vec<i8> = vec![0i8; 33i32 as usize];
        let mut buf: Vec<i8> = _arr1;
        let mut negative: i32 = i<0i32;
        let mut charPos: i32 = 32i32;
        i = (i).wrapping_neg();
        loop {
            if i > (radix).wrapping_neg() { break; }
            charPos = charPos.wrapping_sub(1i32);
            /* TODO: i2b  */
            buf[charPos as usize] = Integer::digits()[((i%radix)).wrapping_neg() as usize];
            i = (i/radix);
        }
        /* TODO: i2b  */
        buf[charPos as usize] = Integer::digits()[(i).wrapping_neg() as usize];
        charPos = charPos.wrapping_sub(1i32);
        buf[charPos as usize] = 45i32;
        let _t2: String = StringLatin1::newString(&buf, charPos, (33i32).wrapping_sub(charPos))?;
        return Ok(_t2);
        let _t3: String = Integer::toStringUTF16(i, radix)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "toStringUTF16", descriptor = "(II)Ljava/lang/String;", access = "private static"))]
    pub fn toStringUTF16(i: i32, radix: i32) -> Result<String> {
        let mut _arr0: Vec<i8> = vec![0i8; 66i32 as usize];
        let mut buf: Vec<i8> = _arr0;
        let mut negative: i32 = i<0i32;
        let mut charPos: i32 = 32i32;
        i = (i).wrapping_neg();
        loop {
            if i > (radix).wrapping_neg() { break; }
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(&buf, charPos, Integer::digits()[((i%radix)).wrapping_neg() as usize])?;
            i = (i/radix);
        }
        StringUTF16::putChar(&buf, charPos, Integer::digits()[(i).wrapping_neg() as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, 45i32)?;
        let _t1: String = StringUTF16::newString(&buf, charPos, (33i32).wrapping_sub(charPos))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(II)Ljava/lang/String;", access = "public static"))]
    pub fn toUnsignedString(i: i32, radix: i32) -> Result<String> {
        let _t0: i64 = Integer::toUnsignedLong(i)?;
        let _t1: String = Long::toUnsignedString(_t0, radix)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toHexString", descriptor = "(I)Ljava/lang/String;", access = "public static"))]
    pub fn toHexString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 4i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toOctalString", descriptor = "(I)Ljava/lang/String;", access = "public static"))]
    pub fn toOctalString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 3i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toBinaryString", descriptor = "(I)Ljava/lang/String;", access = "public static"))]
    pub fn toBinaryString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString0", descriptor = "(II)Ljava/lang/String;", access = "private static"))]
    pub fn toUnsignedString0(val: i32, shift: i32) -> Result<String> {
        let _t0: i32 = Integer::numberOfLeadingZeros(val)?;
        let mut mag: i32 = (32i32).wrapping_sub(_t0);
        let _t1: i32 = (((mag).wrapping_add((shift).wrapping_sub(1i32))/shift)).max(1i32);
        let mut chars: i32 = _t1;
        let mut _arr2: Vec<i8> = vec![0i8; chars as usize];
        let mut buf: Vec<i8> = _arr2;
        Integer::formatUnsignedInt(val, shift, &buf, chars)?;
        return Ok(String::new(buf, 0i32)?);
        let mut _arr3: Vec<i8> = vec![0i8; (chars).wrapping_mul(2i32) as usize];
        buf = _arr3;
        Integer::formatUnsignedIntUTF16(val, shift, &buf, chars)?;
        Ok(String::new(buf, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedInt", descriptor = "(II[BI)V", access = "private static"))]
    pub fn formatUnsignedInt(val: i32, shift: i32, buf: JvmObject, len: i32) -> Result<()> {
        let mut charPos: i32 = len;
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        /* TODO: i2b  */
        buf[charPos as usize] = Integer::digits()[(val&mask) as usize];
        val = ((val as u32>>(shift&0x1f)) as i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedIntUTF16", descriptor = "(II[BI)V", access = "private static"))]
    pub fn formatUnsignedIntUTF16(val: i32, shift: i32, buf: JvmObject, len: i32) -> Result<()> {
        let mut charPos: i32 = len;
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(buf, charPos, Integer::digits()[(val&mask) as usize])?;
        val = ((val as u32>>(shift&0x1f)) as i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public static"))]
    pub fn toString(i: i32) -> Result<String> {
        let _t0: i32 = Integer::stringSize(i)?;
        let mut size: i32 = _t0;
        let mut _arr1: Vec<i8> = vec![0i8; size as usize];
        let mut buf: Vec<i8> = _arr1;
        let _t2: i32 = Integer::getChars(i, size, &buf)?;
        return Ok(String::new(buf, 0i32)?);
        let mut _arr3: Vec<i8> = vec![0i8; (size).wrapping_mul(2i32) as usize];
        buf = _arr3;
        let _t4: i32 = StringUTF16::getChars(i, size, &buf)?;
        Ok(String::new(buf, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(I)Ljava/lang/String;", access = "public static"))]
    pub fn toUnsignedString(i: i32) -> Result<String> {
        let _t0: i64 = Integer::toUnsignedLong(i)?;
        let _t1: String = Long::toString(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(II[B)I", access = "static"))]
    pub fn getChars(i: i32, index: i32, buf: JvmObject) -> Result<i32> {
        let mut charPos: i32 = index;
        let mut negative: i32 = i<0i32;
        i = (i).wrapping_neg();
        loop {
            if i > -100i32 { break; }
            let mut q: i32 = (i/100i32);
            let mut r: i32 = ((q).wrapping_mul(100i32)).wrapping_sub(i);
            i = q;
            charPos = charPos.wrapping_sub(1i32);
            buf[charPos as usize] = Integer::DigitOnes()[r as usize];
            charPos = charPos.wrapping_sub(1i32);
            buf[charPos as usize] = Integer::DigitTens()[r as usize];
        }
        charPos = charPos.wrapping_sub(1i32);
        buf[charPos as usize] = Integer::DigitOnes()[(i).wrapping_neg() as usize];
        charPos = charPos.wrapping_sub(1i32);
        buf[charPos as usize] = Integer::DigitTens()[(i).wrapping_neg() as usize];
        charPos = charPos.wrapping_sub(1i32);
        buf[charPos as usize] = 45i32;
        Ok(charPos)
    }

    #[cfg_attr(any(), java_method(name = "stringSize", descriptor = "(I)I", access = "static"))]
    pub fn stringSize(x: i32) -> Result<i32> {
        let mut d: i32 = 1i32;
        d = 0i32;
        x = (x).wrapping_neg();
        let mut p: i32 = -10i32;
        let mut i: i32 = 1i32;
        loop {
            if i >= 10i32 { break; }
            return Ok((i).wrapping_add(d));
            p = (10i32).wrapping_mul(p);
            i = i.wrapping_add(1i32);
        }
        Ok((10i32).wrapping_add(d))
    }

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/String;I)I", access = "public static"))]
    pub fn parseInt(s: String, radix: i32) -> Result<i32> {
        panic!("{}", /* NumberFormatException::new(String::from("Cannot parse null string"))? */);
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("less than Character.MIN_RADIX"))?;
        panic!("{}", /* NumberFormatException::new(String::new())? */);
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("greater than Character.MAX_RADIX"))?;
        panic!("{}", /* NumberFormatException::new(String::new())? */);
        let mut negative: i32 = 0i32;
        let mut i: i32 = 0i32;
        let _t0 = s.length()?;
        let mut len: i32 = _t0;
        let mut limit: i32 = -2147483647i32;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        negative = 1i32;
        limit = -2147483648i32;
        let _t2: JvmObject = NumberFormatException::forInputString(s, radix)?;
        panic!("{}", /* _t2 */);
        let _t3: JvmObject = NumberFormatException::forInputString(s, radix)?;
        panic!("{}", /* _t3 */);
        i = i.wrapping_add(1i32);
        let mut multmin: i32 = (limit/radix);
        let mut result: i32 = 0i32;
        loop {
            if i >= len { break; }
            i = i.wrapping_add(1i32);
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit(_t0, radix)?;
            let mut digit: i32 = _t1;
            let _t2: JvmObject = NumberFormatException::forInputString(s, radix)?;
            panic!("{}", /* _t2 */);
            result = (result).wrapping_mul(radix);
            let _t3: JvmObject = NumberFormatException::forInputString(s, radix)?;
            panic!("{}", /* _t3 */);
            result = (result).wrapping_sub(digit);
        }
        return Ok((result).wrapping_neg());
        let _t4: JvmObject = NumberFormatException::forInputString(s, radix)?;
        panic!("{}", /* _t4 */);
    }

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public static"))]
    pub fn parseInt(s: JvmObject, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        let _t0: JvmObject = Objects::requireNonNull(s)?;
        let _t1 = s.length()?;
        let _t2: i32 = Objects::checkFromToIndex(beginIndex, endIndex, _t1)?;
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("less than Character.MIN_RADIX"))?;
        panic!("{}", /* NumberFormatException::new(String::new())? */);
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("greater than Character.MAX_RADIX"))?;
        panic!("{}", /* NumberFormatException::new(String::new())? */);
        let mut negative: i32 = 0i32;
        let mut i: i32 = beginIndex;
        let mut limit: i32 = -2147483647i32;
        let _t3 = s.charAt(i)?;
        let mut firstChar: i32 = _t3;
        negative = 1i32;
        limit = -2147483648i32;
        let _t4: JvmObject = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        panic!("{}", /* _t4 */);
        i = i.wrapping_add(1i32);
        let _t5: JvmObject = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        panic!("{}", /* _t5 */);
        let mut multmin: i32 = (limit/radix);
        let mut result: i32 = 0i32;
        loop {
            if i >= endIndex { break; }
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit(_t0, radix)?;
            let mut digit: i32 = _t1;
            let _t2: JvmObject = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            panic!("{}", /* _t2 */);
            result = (result).wrapping_mul(radix);
            let _t3: JvmObject = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            panic!("{}", /* _t3 */);
            i = i.wrapping_add(1i32);
            result = (result).wrapping_sub(digit);
        }
        return Ok((result).wrapping_neg());
        let _t6: JvmObject = NumberFormatException::forInputString(String::from(""), radix)?;
        panic!("{}", /* _t6 */);
    }

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/String;)I", access = "public static"))]
    pub fn parseInt(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseInt(s, 10i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;I)I", access = "public static"))]
    pub fn parseUnsignedInt(s: String, radix: i32) -> Result<i32> {
        panic!("{}", /* NumberFormatException::new(String::from("Cannot parse null string"))? */);
        let _t0 = s.length()?;
        let mut len: i32 = _t0;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        let mut _arr2: Vec<JvmObject> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = s;
        let _t3: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr2)?;
        panic!("{}", /* NumberFormatException::new(_t3)? */);
        let _t4: i32 = Integer::parseInt(s, radix)?;
        return Ok(_t4);
        let _t5: i64 = Long::parseLong(s, radix)?;
        let mut ell: i64 = _t5;
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok((ell as i32));
        let mut _arr6: Vec<JvmObject> = Vec::with_capacity(1i32 as usize);
        _arr6[0i32 as usize] = s;
        let _t7: String = String::format(String::from("String value %s exceeds range of unsigned int."), &_arr6)?;
        panic!("{}", /* NumberFormatException::new(_t7)? */);
        let _t8: JvmObject = NumberFormatException::forInputString(s, radix)?;
        panic!("{}", /* _t8 */);
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public static"))]
    pub fn parseUnsignedInt(s: JvmObject, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        let _t0: JvmObject = Objects::requireNonNull(s)?;
        let _t1 = s.length()?;
        let _t2: i32 = Objects::checkFromToIndex(beginIndex, endIndex, _t1)?;
        let mut start: i32 = beginIndex;
        let mut len: i32 = (endIndex).wrapping_sub(beginIndex);
        let _t3 = s.charAt(start)?;
        let mut firstChar: i32 = _t3;
        let mut _arr4: Vec<JvmObject> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = s;
        let _t5: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr4)?;
        panic!("{}", /* NumberFormatException::new(_t5)? */);
        let _t6: i32 = Integer::parseInt(s, start, (start).wrapping_add(len), radix)?;
        return Ok(_t6);
        let _t7: i64 = Long::parseLong(s, start, (start).wrapping_add(len), radix)?;
        let mut ell: i64 = _t7;
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok((ell as i32));
        let mut _arr8: Vec<JvmObject> = Vec::with_capacity(1i32 as usize);
        _arr8[0i32 as usize] = s;
        let _t9: String = String::format(String::from("String value %s exceeds range of unsigned int."), &_arr8)?;
        panic!("{}", /* NumberFormatException::new(_t9)? */);
        panic!("{}", /* NumberFormatException::new(String::from(""))? */);
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;)I", access = "public static"))]
    pub fn parseUnsignedInt(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseUnsignedInt(s, 10i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public static"))]
    pub fn valueOf(s: String, radix: i32) -> Result<i32> {
        let _t0: i32 = Integer::parseInt(s, radix)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public static"))]
    pub fn valueOf(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseInt(s, 10i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(I)Ljava/lang/Integer;", access = "public static"))]
    pub fn valueOf(i: i32) -> Result<i32> {
        return Ok(Integer$IntegerCache::cache()[(i).wrapping_add(128i32) as usize].clone());
        Ok(Integer::new(i)?)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    pub fn new(value: i32) -> Result<Self> {
        let this = Self { value: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(s: String) -> Result<Self> {
        let this = Self { value: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: i32 = Integer::parseInt(s, 10i32)?;
        this.value.set(_t0);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "byteValue", descriptor = "()B", access = "public"))]
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        /* TODO: i2b  */
        Ok(this.value.get())
    }

    #[cfg_attr(any(), java_method(name = "shortValue", descriptor = "()S", access = "public"))]
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
        /* TODO: i2s  */
        Ok(this.value.get())
    }

    #[cfg_attr(any(), java_method(name = "intValue", descriptor = "()I", access = "public"))]
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        Ok(this.value.get())
    }

    #[cfg_attr(any(), java_method(name = "longValue", descriptor = "()J", access = "public"))]
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        Ok((this.value.get() as i64))
    }

    #[cfg_attr(any(), java_method(name = "floatValue", descriptor = "()F", access = "public"))]
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        Ok((this.value.get() as f32))
    }

    #[cfg_attr(any(), java_method(name = "doubleValue", descriptor = "()D", access = "public"))]
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        Ok((this.value.get() as f64))
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Integer::toString(this.value.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Integer::hashCode(this.value.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I)I", access = "public static"))]
    pub fn hashCode(value: i32) -> Result<i32> {
        Ok(value)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, obj: JvmObject) -> Result<bool> {
        let this = self;
        return Ok(this.value.get() == obj);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public static"))]
    pub fn getInteger(nm: String) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Integer::getInteger(/* UNDERFLOW */, nm)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public static"))]
    pub fn getInteger(nm: String, val: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Integer::getInteger(/* UNDERFLOW */, nm)?;
        let mut result: i32 = _t0;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;", access = "public static"))]
    pub fn getInteger(nm: String, val: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let mut v: i32 = /* UNDERFLOW */;
        let _t0: String = System::getProperty(nm)?;
        v = _t0;
        let mut local_3: i32 = /* UNDERFLOW */;
        let _t1: i32 = Integer::decode(v)?;
        return Ok(_t1);
        local_3 = v;
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public static"))]
    pub fn decode(nm: String) -> Result<i32> {
        let mut radix: i32 = 10i32;
        let mut index: i32 = 0i32;
        let mut negative: i32 = 0i32;
        let _t0 = nm.isEmpty()?;
        panic!("{}", /* NumberFormatException::new(String::from("Zero length string"))? */);
        let _t1 = nm.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        negative = 1i32;
        index = index.wrapping_add(1i32);
        index = index.wrapping_add(1i32);
        let _t2 = nm.startsWith(String::from("0x"), index)?;
        let _t3 = nm.startsWith(String::from("0X"), index)?;
        index = index.wrapping_add(2i32);
        radix = 16i32;
        let _t4 = nm.startsWith(String::from("#"), index)?;
        index = index.wrapping_add(1i32);
        radix = 16i32;
        let _t5 = nm.startsWith(String::from("0"), index)?;
        let _t6 = nm.length()?;
        index = index.wrapping_add(1i32);
        radix = 8i32;
        let _t7 = nm.startsWith(String::from("-"), index)?;
        let _t8 = nm.startsWith(String::from("+"), index)?;
        panic!("{}", /* NumberFormatException::new(String::from("Sign character in wrong position"))? */);
        let _t9 = nm.length()?;
        let _t10: i32 = Integer::parseInt(nm, index, _t9, radix)?;
        let mut result: i32 = _t10;
        result = result;
        let mut e: i32 = (result).wrapping_neg();
        String::new().append(&String::from("-"))?;
        let _t11 = nm.substring(index)?;
        String::new().append(&_t11)?;
        let _t12 = nm.substring(index)?;
        let mut constant: String = _t12;
        let _t13: i32 = Integer::parseInt(constant, radix)?;
        result = _t13;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Integer;)I", access = "public"))]
    pub fn compareTo(&self, anotherInteger: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = Integer::compare(this.value.get(), anotherInteger.value.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(II)I", access = "public static"))]
    pub fn compare(x: i32, y: i32) -> Result<i32> {
        Ok(x != y)
    }

    #[cfg_attr(any(), java_method(name = "compareUnsigned", descriptor = "(II)I", access = "public static"))]
    pub fn compareUnsigned(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = Integer::compare((x).wrapping_add(-2147483648i32), (y).wrapping_add(-2147483648i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedLong", descriptor = "(I)J", access = "public static"))]
    pub fn toUnsignedLong(x: i32) -> Result<i64> {
        /* TODO: land  */
        Ok(4294967295i64)
    }

    #[cfg_attr(any(), java_method(name = "divideUnsigned", descriptor = "(II)I", access = "public static"))]
    pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        let _t0: i64 = Integer::toUnsignedLong(dividend)?;
        let _t1: i64 = Integer::toUnsignedLong(divisor)?;
        Ok(((_t0/_t1) as i32))
    }

    #[cfg_attr(any(), java_method(name = "remainderUnsigned", descriptor = "(II)I", access = "public static"))]
    pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        let _t0: i64 = Integer::toUnsignedLong(dividend)?;
        let _t1: i64 = Integer::toUnsignedLong(divisor)?;
        /* TODO: lrem  */
        Ok((_t1 as i32))
    }

    #[cfg_attr(any(), java_method(name = "highestOneBit", descriptor = "(I)I", access = "public static"))]
    pub fn highestOneBit(i: i32) -> Result<i32> {
        let _t0: i32 = Integer::numberOfLeadingZeros(i)?;
        Ok((i&((-2147483648i32 as u32>>(_t0&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "lowestOneBit", descriptor = "(I)I", access = "public static"))]
    pub fn lowestOneBit(i: i32) -> Result<i32> {
        Ok((i&(i).wrapping_neg()))
    }

    #[cfg_attr(any(), java_method(name = "numberOfLeadingZeros", descriptor = "(I)I", access = "public static"))]
    pub fn numberOfLeadingZeros(i: i32) -> Result<i32> {
        return Ok(0i32);
        let mut n: i32 = 31i32;
        n = n.wrapping_sub(16i32);
        i = ((i as u32>>(16i32&0x1f)) as i32);
        n = n.wrapping_sub(8i32);
        i = ((i as u32>>(8i32&0x1f)) as i32);
        n = n.wrapping_sub(4i32);
        i = ((i as u32>>(4i32&0x1f)) as i32);
        n = n.wrapping_sub(2i32);
        i = ((i as u32>>(2i32&0x1f)) as i32);
        Ok((n).wrapping_sub(((i as u32>>(1i32&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "numberOfTrailingZeros", descriptor = "(I)I", access = "public static"))]
    pub fn numberOfTrailingZeros(i: i32) -> Result<i32> {
        i = ((i^-1i32)&(i).wrapping_sub(1i32));
        return Ok((i&32i32));
        let mut n: i32 = 1i32;
        n = n.wrapping_add(16i32);
        i = ((i as u32>>(16i32&0x1f)) as i32);
        n = n.wrapping_add(8i32);
        i = ((i as u32>>(8i32&0x1f)) as i32);
        n = n.wrapping_add(4i32);
        i = ((i as u32>>(4i32&0x1f)) as i32);
        n = n.wrapping_add(2i32);
        i = ((i as u32>>(2i32&0x1f)) as i32);
        Ok((n).wrapping_add(((i as u32>>(1i32&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "bitCount", descriptor = "(I)I", access = "public static"))]
    pub fn bitCount(i: i32) -> Result<i32> {
        i = (i).wrapping_sub((((i as u32>>(1i32&0x1f)) as i32)&265i32));
        i = ((i&266i32)).wrapping_add((((i as u32>>(2i32&0x1f)) as i32)&266i32));
        i = ((i).wrapping_add(((i as u32>>(4i32&0x1f)) as i32))&267i32);
        i = (i).wrapping_add(((i as u32>>(8i32&0x1f)) as i32));
        i = (i).wrapping_add(((i as u32>>(16i32&0x1f)) as i32));
        Ok((i&63i32))
    }

    #[cfg_attr(any(), java_method(name = "rotateLeft", descriptor = "(II)I", access = "public static"))]
    pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
        Ok(((i<<(distance&0x1f))|((i as u32>>((distance).wrapping_neg()&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "rotateRight", descriptor = "(II)I", access = "public static"))]
    pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
        Ok((((i as u32>>(distance&0x1f)) as i32)|(i<<((distance).wrapping_neg()&0x1f))))
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "(I)I", access = "public static"))]
    pub fn reverse(i: i32) -> Result<i32> {
        i = (((i&265i32)<<(1i32&0x1f))|(((i as u32>>(1i32&0x1f)) as i32)&265i32));
        i = (((i&266i32)<<(2i32&0x1f))|(((i as u32>>(2i32&0x1f)) as i32)&266i32));
        i = (((i&267i32)<<(4i32&0x1f))|(((i as u32>>(4i32&0x1f)) as i32)&267i32));
        let _t0: i32 = Integer::reverseBytes(i)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "(II)I", access = "public static"))]
    pub fn compress(i: i32, mask: i32) -> Result<i32> {
        i = (i&mask);
        let mut maskCount: i32 = ((mask^-1i32)<<(1i32&0x1f));
        let mut j: i32 = 0i32;
        loop {
            if j >= 5i32 { break; }
            let _t0: i32 = Integer::parallelSuffix(maskCount)?;
            let mut maskPrefix: i32 = _t0;
            let mut maskMove: i32 = (maskPrefix&mask);
            mask = ((mask^maskMove)|((maskMove as u32>>((1i32<<(j&0x1f))&0x1f)) as i32));
            let mut t: i32 = (i&maskMove);
            i = ((i^t)|((t as u32>>((1i32<<(j&0x1f))&0x1f)) as i32));
            maskCount = (maskCount&(maskPrefix^-1i32));
            j = j.wrapping_add(1i32);
        }
        Ok(i)
    }

    #[cfg_attr(any(), java_method(name = "expand", descriptor = "(II)I", access = "public static"))]
    pub fn expand(i: i32, mask: i32) -> Result<i32> {
        let mut originalMask: i32 = mask;
        let mut maskCount: i32 = ((mask^-1i32)<<(1i32&0x1f));
        let _t0: i32 = Integer::parallelSuffix(maskCount)?;
        let mut maskPrefix: i32 = _t0;
        let mut maskMove1: i32 = (maskPrefix&mask);
        mask = ((mask^maskMove1)|((maskMove1 as u32>>(1i32&0x1f)) as i32));
        maskCount = (maskCount&(maskPrefix^-1i32));
        let _t1: i32 = Integer::parallelSuffix(maskCount)?;
        maskPrefix = _t1;
        let mut maskMove2: i32 = (maskPrefix&mask);
        mask = ((mask^maskMove2)|((maskMove2 as u32>>(2i32&0x1f)) as i32));
        maskCount = (maskCount&(maskPrefix^-1i32));
        let _t2: i32 = Integer::parallelSuffix(maskCount)?;
        maskPrefix = _t2;
        let mut maskMove3: i32 = (maskPrefix&mask);
        mask = ((mask^maskMove3)|((maskMove3 as u32>>(4i32&0x1f)) as i32));
        maskCount = (maskCount&(maskPrefix^-1i32));
        let _t3: i32 = Integer::parallelSuffix(maskCount)?;
        maskPrefix = _t3;
        let mut maskMove4: i32 = (maskPrefix&mask);
        mask = ((mask^maskMove4)|((maskMove4 as u32>>(8i32&0x1f)) as i32));
        maskCount = (maskCount&(maskPrefix^-1i32));
        let _t4: i32 = Integer::parallelSuffix(maskCount)?;
        maskPrefix = _t4;
        let mut maskMove5: i32 = (maskPrefix&mask);
        let mut t: i32 = (i<<(16i32&0x1f));
        i = ((i&(maskMove5^-1i32))|(t&maskMove5));
        t = (i<<(8i32&0x1f));
        i = ((i&(maskMove4^-1i32))|(t&maskMove4));
        t = (i<<(4i32&0x1f));
        i = ((i&(maskMove3^-1i32))|(t&maskMove3));
        t = (i<<(2i32&0x1f));
        i = ((i&(maskMove2^-1i32))|(t&maskMove2));
        t = (i<<(1i32&0x1f));
        i = ((i&(maskMove1^-1i32))|(t&maskMove1));
        Ok((i&originalMask))
    }

    #[cfg_attr(any(), java_method(name = "parallelSuffix", descriptor = "(I)I", access = "private static"))]
    pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
        let mut maskPrefix: i32 = (maskCount^(maskCount<<(1i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(2i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(4i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(8i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(16i32&0x1f)));
        Ok(maskPrefix)
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "(I)I", access = "public static"))]
    pub fn signum(i: i32) -> Result<i32> {
        Ok(((i>>((31i32&0x1f)))|(((i).wrapping_neg() as u32>>(31i32&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "reverseBytes", descriptor = "(I)I", access = "public static"))]
    pub fn reverseBytes(i: i32) -> Result<i32> {
        Ok(((((i<<(24i32&0x1f))|((i&274i32)<<(8i32&0x1f)))|(((i as u32>>(8i32&0x1f)) as i32)&274i32))|((i as u32>>(24i32&0x1f)) as i32)))
    }

    #[cfg_attr(any(), java_method(name = "sum", descriptor = "(II)I", access = "public static"))]
    pub fn sum(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_add(b))
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(II)I", access = "public static"))]
    pub fn max(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).max(b);
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(II)I", access = "public static"))]
    pub fn min(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).min(b);
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public"))]
    pub fn describeConstable(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = Optional::of(this)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;", access = "public"))]
    pub fn resolveConstantDesc(&self, lookup: JvmObject) -> Result<i32> {
        let this = self;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn compareTo(&self, arg_0: JvmObject) -> Result<i32> {
        let this = self;
        let _t0 = this.compareTo(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;", access = "public"))]
    pub fn resolveConstantDesc(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.resolveConstantDesc(arg_0)?;
        Ok(_t0)
    }
}
