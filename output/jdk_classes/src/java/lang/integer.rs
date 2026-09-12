#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

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
    // java: toString(II)Ljava/lang/String;
    // java: toString(II)Ljava/lang/String;
    pub fn toString__i_i(i: i32, radix: i32) -> Result<String> {
        radix = 10i32;
        let _t0: String = Integer::toString__i(i)?;
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

    // java: toStringUTF16(II)Ljava/lang/String;
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

    // java: toUnsignedString(II)Ljava/lang/String;
    // java: toUnsignedString(II)Ljava/lang/String;
    pub fn toUnsignedString__i_i(i: i32, radix: i32) -> Result<String> {
        let _t0: i64 = Integer::toUnsignedLong(i)?;
        let _t1: String = Long::toUnsignedString__l_i(_t0, radix)?;
        Ok(_t1)
    }

    // java: toHexString(I)Ljava/lang/String;
    pub fn toHexString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 4i32)?;
        Ok(_t0)
    }

    // java: toOctalString(I)Ljava/lang/String;
    pub fn toOctalString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 3i32)?;
        Ok(_t0)
    }

    // java: toBinaryString(I)Ljava/lang/String;
    pub fn toBinaryString(i: i32) -> Result<String> {
        let _t0: String = Integer::toUnsignedString0(i, 1i32)?;
        Ok(_t0)
    }

    // java: toUnsignedString0(II)Ljava/lang/String;
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

    // java: formatUnsignedInt(II[BI)V
    pub fn formatUnsignedInt(val: i32, shift: i32, buf: &[i8], len: i32) -> Result<()> {
        let mut charPos: i32 = len;
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        /* TODO: i2b  */
        buf[charPos as usize] = Integer::digits()[(val&mask) as usize];
        val = ((val as u32>>(shift&0x1f)) as i32);
        Ok(())
    }

    // java: formatUnsignedIntUTF16(II[BI)V
    pub fn formatUnsignedIntUTF16(val: i32, shift: i32, buf: &[i8], len: i32) -> Result<()> {
        let mut charPos: i32 = len;
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, Integer::digits()[(val&mask) as usize])?;
        val = ((val as u32>>(shift&0x1f)) as i32);
        Ok(())
    }

    // java: toString(I)Ljava/lang/String;
    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(i: i32) -> Result<String> {
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

    // java: toUnsignedString(I)Ljava/lang/String;
    // java: toUnsignedString(I)Ljava/lang/String;
    pub fn toUnsignedString__i(i: i32) -> Result<String> {
        let _t0: i64 = Integer::toUnsignedLong(i)?;
        let _t1: String = Long::toString__l(_t0)?;
        Ok(_t1)
    }

    // java: getChars(II[B)I
    pub fn getChars(i: i32, index: i32, buf: &[i8]) -> Result<i32> {
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

    // java: stringSize(I)I
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

    // java: parseInt(Ljava/lang/String;I)I
    // java: parseInt(Ljava/lang/String;I)I
    pub fn parseInt__str_i(s: String, radix: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("less than Character.MIN_RADIX"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("greater than Character.MAX_RADIX"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut negative: i32 = 0i32;
        let mut i: i32 = 0i32;
        let _t0 = s.length()?;
        let mut len: i32 = _t0;
        let mut limit: i32 = -2147483647i32;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        negative = 1i32;
        limit = -2147483648i32;
        let _t2: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        i = i.wrapping_add(1i32);
        let mut multmin: i32 = (limit/radix);
        let mut result: i32 = 0i32;
        loop {
            if i >= len { break; }
            i = i.wrapping_add(1i32);
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit__c_i(_t0, radix)?;
            let mut digit: i32 = _t1;
            let _t2: Object = NumberFormatException::forInputString(s, radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_mul(radix);
            let _t3: Object = NumberFormatException::forInputString(s, radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_sub(digit);
        }
        return Ok((result).wrapping_neg());
        let _t4: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseInt(Ljava/lang/CharSequence;III)I
    // java: parseInt(Ljava/lang/CharSequence;III)I
    pub fn parseInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(s)?;
        let _t1 = s.length()?;
        let _t2: i32 = Objects::checkFromToIndex__i_i_i(beginIndex, endIndex, _t1)?;
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("less than Character.MIN_RADIX"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("radix"))?;
        String::new().append(&radix)?;
        String::new().append(&String::from("greater than Character.MAX_RADIX"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut negative: i32 = 0i32;
        let mut i: i32 = beginIndex;
        let mut limit: i32 = -2147483647i32;
        let _t3 = s.charAt(i)?;
        let mut firstChar: i32 = _t3;
        negative = 1i32;
        limit = -2147483648i32;
        let _t4: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        i = i.wrapping_add(1i32);
        let _t5: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut multmin: i32 = (limit/radix);
        let mut result: i32 = 0i32;
        loop {
            if i >= endIndex { break; }
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit__c_i(_t0, radix)?;
            let mut digit: i32 = _t1;
            let _t2: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_mul(radix);
            let _t3: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            i = i.wrapping_add(1i32);
            result = (result).wrapping_sub(digit);
        }
        return Ok((result).wrapping_neg());
        let _t6: Object = NumberFormatException::forInputString(String::from(""), radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseInt(Ljava/lang/String;)I
    // java: parseInt(Ljava/lang/String;)I
    pub fn parseInt__str(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseInt__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: parseUnsignedInt(Ljava/lang/String;I)I
    // java: parseUnsignedInt(Ljava/lang/String;I)I
    pub fn parseUnsignedInt__str_i(s: String, radix: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = s.length()?;
        let mut len: i32 = _t0;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = s;
        let _t3: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr2)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4: i32 = Integer::parseInt__str_i(s, radix)?;
        return Ok(_t4);
        let _t5: i64 = Long::parseLong__str_i(s, radix)?;
        let mut ell: i64 = _t5;
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok((ell as i32));
        let mut _arr6: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr6[0i32 as usize] = s;
        let _t7: String = String::format(String::from("String value %s exceeds range of unsigned int."), &_arr6)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t8: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseUnsignedInt(Ljava/lang/CharSequence;III)I
    // java: parseUnsignedInt(Ljava/lang/CharSequence;III)I
    pub fn parseUnsignedInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(s)?;
        let _t1 = s.length()?;
        let _t2: i32 = Objects::checkFromToIndex__i_i_i(beginIndex, endIndex, _t1)?;
        let mut start: i32 = beginIndex;
        let mut len: i32 = (endIndex).wrapping_sub(beginIndex);
        let _t3 = s.charAt(start)?;
        let mut firstChar: i32 = _t3;
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr4[0i32 as usize] = s;
        let _t5: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t6: i32 = Integer::parseInt__seq_i_i_i(s, start, (start).wrapping_add(len), radix)?;
        return Ok(_t6);
        let _t7: i64 = Long::parseLong__seq_i_i_i(s, start, (start).wrapping_add(len), radix)?;
        let mut ell: i64 = _t7;
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok((ell as i32));
        let mut _arr8: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr8[0i32 as usize] = s;
        let _t9: String = String::format(String::from("String value %s exceeds range of unsigned int."), &_arr8)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseUnsignedInt(Ljava/lang/String;)I
    // java: parseUnsignedInt(Ljava/lang/String;)I
    pub fn parseUnsignedInt__str(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseUnsignedInt__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Integer;
    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i32> {
        let _t0: i32 = Integer::parseInt__str_i(s, radix)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Integer;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn valueOf__str(s: String) -> Result<i32> {
        let _t0: i32 = Integer::parseInt__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(I)Ljava/lang/Integer;
    // java: valueOf(I)Ljava/lang/Integer;
    pub fn valueOf__i(i: i32) -> Result<i32> {
        return Ok(Integer_IntegerCache::cache()[(i).wrapping_add(128i32) as usize].clone());
        Ok(Integer::new(i)?)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(value: i32) -> Result<Self> {
        let this = Self { value: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { value: Field::new(0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: i32 = Integer::parseInt__str_i(s, 10i32)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        /* TODO: i2b  */
        Ok(this.value.get())
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
        /* TODO: i2s  */
        Ok(this.value.get())
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        Ok(this.value.get())
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        Ok((this.value.get() as i64))
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        Ok((this.value.get() as f32))
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        Ok((this.value.get() as f64))
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Integer::toString__i(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Integer::hashCode__i(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(I)I
    // java: hashCode(I)I
    pub fn hashCode__i(value: i32) -> Result<i32> {
        Ok(value)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(this.value.get() == obj);
        Ok(0i32)
    }

    // java: getInteger(Ljava/lang/String;)Ljava/lang/Integer;
    // java: getInteger(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn getInteger__str(nm: String) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Integer::getInteger__str_int(todo!("stack underflow"), nm)?;
        Ok(_t0)
    }

    // java: getInteger(Ljava/lang/String;I)Ljava/lang/Integer;
    // java: getInteger(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn getInteger__str_i(nm: String, val: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let _t0: i32 = Integer::getInteger__str_int(todo!("stack underflow"), nm)?;
        let mut result: i32 = _t0;
        Ok(result)
    }

    // java: getInteger(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;
    // java: getInteger(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;
    pub fn getInteger__str_int(nm: String, val: i32) -> Result<i32> {
        /* TODO: aconst_null  */
        let mut v: i32 = todo!("stack underflow");
        let _t0: String = System::getProperty(nm)?;
        v = _t0;
        let mut local_3: i32 = todo!("stack underflow");
        let _t1: i32 = Integer::decode(v)?;
        return Ok(_t1);
        local_3 = v;
        Ok(val)
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn decode(nm: String) -> Result<i32> {
        let mut radix: i32 = 10i32;
        let mut index: i32 = 0i32;
        let mut negative: i32 = 0i32;
        let _t0 = nm.isEmpty()?;
        return Err(JvmError::Custom("athrow".to_owned()));
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
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t9 = nm.length()?;
        let _t10: i32 = Integer::parseInt__seq_i_i_i(nm, index, _t9, radix)?;
        let mut result: i32 = _t10;
        result = result;
        let mut e: i32 = (result).wrapping_neg();
        String::new().append(&String::from("-"))?;
        let _t11 = nm.substring(index)?;
        String::new().append(&_t11)?;
        let _t12 = nm.substring(index)?;
        let mut constant: String = _t12;
        let _t13: i32 = Integer::parseInt__str_i(constant, radix)?;
        result = _t13;
        Ok(result)
    }

    // java: compareTo(Ljava/lang/Integer;)I
    pub fn compareTo(&self, anotherInteger: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = Integer::compare(this.value.get(), anotherInteger.value.get())?;
        Ok(_t0)
    }

    // java: compare(II)I
    pub fn compare(x: i32, y: i32) -> Result<i32> {
        Ok(x != y)
    }

    // java: compareUnsigned(II)I
    pub fn compareUnsigned(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = Integer::compare((x).wrapping_add(-2147483648i32), (y).wrapping_add(-2147483648i32))?;
        Ok(_t0)
    }

    // java: toUnsignedLong(I)J
    pub fn toUnsignedLong(x: i32) -> Result<i64> {
        /* TODO: land  */
        Ok(4294967295i64)
    }

    // java: divideUnsigned(II)I
    pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        let _t0: i64 = Integer::toUnsignedLong(dividend)?;
        let _t1: i64 = Integer::toUnsignedLong(divisor)?;
        Ok(((_t0/_t1) as i32))
    }

    // java: remainderUnsigned(II)I
    pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        let _t0: i64 = Integer::toUnsignedLong(dividend)?;
        let _t1: i64 = Integer::toUnsignedLong(divisor)?;
        /* TODO: lrem  */
        Ok((_t1 as i32))
    }

    // java: highestOneBit(I)I
    pub fn highestOneBit(i: i32) -> Result<i32> {
        let _t0: i32 = Integer::numberOfLeadingZeros(i)?;
        Ok((i&((-2147483648i32 as u32>>(_t0&0x1f)) as i32)))
    }

    // java: lowestOneBit(I)I
    pub fn lowestOneBit(i: i32) -> Result<i32> {
        Ok((i&(i).wrapping_neg()))
    }

    // java: numberOfLeadingZeros(I)I
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

    // java: numberOfTrailingZeros(I)I
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

    // java: bitCount(I)I
    pub fn bitCount(i: i32) -> Result<i32> {
        i = (i).wrapping_sub((((i as u32>>(1i32&0x1f)) as i32)&265i32));
        i = ((i&266i32)).wrapping_add((((i as u32>>(2i32&0x1f)) as i32)&266i32));
        i = ((i).wrapping_add(((i as u32>>(4i32&0x1f)) as i32))&267i32);
        i = (i).wrapping_add(((i as u32>>(8i32&0x1f)) as i32));
        i = (i).wrapping_add(((i as u32>>(16i32&0x1f)) as i32));
        Ok((i&63i32))
    }

    // java: rotateLeft(II)I
    pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
        Ok(((i<<(distance&0x1f))|((i as u32>>((distance).wrapping_neg()&0x1f)) as i32)))
    }

    // java: rotateRight(II)I
    pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
        Ok((((i as u32>>(distance&0x1f)) as i32)|(i<<((distance).wrapping_neg()&0x1f))))
    }

    // java: reverse(I)I
    pub fn reverse(i: i32) -> Result<i32> {
        i = (((i&265i32)<<(1i32&0x1f))|(((i as u32>>(1i32&0x1f)) as i32)&265i32));
        i = (((i&266i32)<<(2i32&0x1f))|(((i as u32>>(2i32&0x1f)) as i32)&266i32));
        i = (((i&267i32)<<(4i32&0x1f))|(((i as u32>>(4i32&0x1f)) as i32)&267i32));
        let _t0: i32 = Integer::reverseBytes(i)?;
        Ok(_t0)
    }

    // java: compress(II)I
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

    // java: expand(II)I
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

    // java: parallelSuffix(I)I
    pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
        let mut maskPrefix: i32 = (maskCount^(maskCount<<(1i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(2i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(4i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(8i32&0x1f)));
        maskPrefix = (maskPrefix^(maskPrefix<<(16i32&0x1f)));
        Ok(maskPrefix)
    }

    // java: signum(I)I
    pub fn signum(i: i32) -> Result<i32> {
        Ok(((i>>((31i32&0x1f)))|(((i).wrapping_neg() as u32>>(31i32&0x1f)) as i32)))
    }

    // java: reverseBytes(I)I
    pub fn reverseBytes(i: i32) -> Result<i32> {
        Ok(((((i<<(24i32&0x1f))|((i&274i32)<<(8i32&0x1f)))|(((i as u32>>(8i32&0x1f)) as i32)&274i32))|((i as u32>>(24i32&0x1f)) as i32)))
    }

    // java: sum(II)I
    pub fn sum(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_add(b))
    }

    // java: max(II)I
    pub fn max(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).max(b);
        Ok(_t0)
    }

    // java: min(II)I
    pub fn min(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).min(b);
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(this)?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i32> {
        let this = self;
        Ok(this)
    }
}
