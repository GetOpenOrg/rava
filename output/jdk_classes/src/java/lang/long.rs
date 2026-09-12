#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Long",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Long.java",
))]
pub struct Long {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "J", access = "private final"))]
    pub value: Field<i64>,
}

impl Long {
    // java: toString(JI)Ljava/lang/String;
    // java: toString(JI)Ljava/lang/String;
    pub fn toString__l_i(i: i64, arg_1: i32) -> Result<String> {
        let mut radix: i32 = 10i32;
        let _t0: String = Long::toString__l(i)?;
        return Ok(_t0);
        let mut _arr1: Vec<i8> = vec![0i8; 65i32 as usize];
        let mut buf: Vec<i8> = _arr1;
        let mut charPos: i32 = 64i32;
        /* TODO: lcmp  */
        let mut negative: i32 = 0i64<0i32;
        /* TODO: lneg  */
        i = i;
        loop {
            /* TODO: lcmp  */
            if ((radix).wrapping_neg() as i64)>0i32 { break; }
            charPos = charPos.wrapping_sub(1i32);
            /* TODO: lrem  */
            /* TODO: lneg  */
            /* TODO: i2b  */
            charPos[Integer::digits() as usize] = i[((radix as i64) as i32) as usize];
            i = (i/(radix as i64));
        }
        /* TODO: lneg  */
        /* TODO: i2b  */
        buf[charPos as usize] = Integer::digits()[(i as i32) as usize];
        charPos = charPos.wrapping_sub(1i32);
        buf[charPos as usize] = 45i32;
        let _t2: String = StringLatin1::newString(&buf, charPos, (65i32).wrapping_sub(charPos))?;
        return Ok(_t2);
        let _t3: String = Long::toStringUTF16(i, radix)?;
        Ok(_t3)
    }

    // java: toStringUTF16(JI)Ljava/lang/String;
    pub fn toStringUTF16(i: i64, arg_1: i32) -> Result<String> {
        let mut _arr0: Vec<i8> = vec![0i8; 130i32 as usize];
        let mut buf: Vec<i8> = _arr0;
        let mut charPos: i32 = 64i32;
        /* TODO: lcmp  */
        let mut negative: i32 = 0i64<0i32;
        /* TODO: lneg  */
        i = i;
        loop {
            /* TODO: lcmp  */
            if ((local_2).wrapping_neg() as i64)>0i32 { break; }
            charPos = charPos.wrapping_sub(1i32);
            /* TODO: lrem  */
            /* TODO: lneg  */
            StringUTF16::putChar(charPos, &Integer::digits(), i[((local_2 as i64) as i32) as usize])?;
            i = (i/(local_2 as i64));
        }
        /* TODO: lneg  */
        StringUTF16::putChar(&buf, charPos, Integer::digits()[(i as i32) as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, 45i32)?;
        let _t1: String = StringUTF16::newString(&buf, charPos, (65i32).wrapping_sub(charPos))?;
        Ok(_t1)
    }

    // java: toUnsignedString(JI)Ljava/lang/String;
    // java: toUnsignedString(JI)Ljava/lang/String;
    pub fn toUnsignedString__l_i(i: i64, arg_1: i32) -> Result<String> {
        /* TODO: lcmp  */
        let _t0: String = Long::toString__l_i(i, local_2)?;
        return Ok(_t0);
        /* TODO: lookupswitch default:151 2:72 4:79 8:87 10:94 16:136 32:143 */
        let _t1: String = Long::toBinaryString(i)?;
        let _t2: String = Long::toUnsignedString0(i, 2i32)?;
        let _t3: String = Long::toOctalString(i)?;
        /* TODO: lushr  */
        let mut quot: i64 = (1i32/5i64);
        let mut rem: i64 = (i).wrapping_sub((quot).wrapping_mul(10i64));
        let _t4: String = Long::toString__l(quot)?;
        String::new().append(&_t4)?;
        String::new().append(&rem)?;
        let _t5: String = Long::toHexString(i)?;
        let _t6: String = Long::toUnsignedString0(i, 5i32)?;
        let _t7: Object = Long::toUnsignedBigInteger(i)?;
        let _t8 = _t7.toString(local_2)?;
        Ok(_t8)
    }

    // java: toUnsignedBigInteger(J)Ljava/math/BigInteger;
    pub fn toUnsignedBigInteger(i: i64) -> Result<Object> {
        /* TODO: lcmp  */
        return Ok(i);
        /* TODO: lushr  */
        let mut upper: i32 = (32i32 as i32);
        let mut lower: i32 = (i as i32);
        let _t0: i64 = Integer::toUnsignedLong(upper)?;
        let _t1 = _t0.shiftLeft(32i32)?;
        let _t2: i64 = Integer::toUnsignedLong(lower)?;
        let _t3 = _t1.add(_t2)?;
        Ok(_t3)
    }

    // java: toHexString(J)Ljava/lang/String;
    pub fn toHexString(i: i64) -> Result<String> {
        let _t0: String = Long::toUnsignedString0(i, 4i32)?;
        Ok(_t0)
    }

    // java: toOctalString(J)Ljava/lang/String;
    pub fn toOctalString(i: i64) -> Result<String> {
        let _t0: String = Long::toUnsignedString0(i, 3i32)?;
        Ok(_t0)
    }

    // java: toBinaryString(J)Ljava/lang/String;
    pub fn toBinaryString(i: i64) -> Result<String> {
        let _t0: String = Long::toUnsignedString0(i, 1i32)?;
        Ok(_t0)
    }

    // java: toUnsignedString0(JI)Ljava/lang/String;
    pub fn toUnsignedString0(val: i64, arg_1: i32) -> Result<String> {
        let _t0: i32 = Long::numberOfLeadingZeros(val)?;
        let mut mag: i32 = (64i32).wrapping_sub(_t0);
        let _t1: i32 = (((mag).wrapping_add((local_2).wrapping_sub(1i32))/local_2)).max(1i32);
        let mut chars: i32 = _t1;
        let mut _arr2: Vec<i8> = vec![0i8; chars as usize];
        let mut buf: Vec<i8> = _arr2;
        Long::formatUnsignedLong0(val, local_2, &buf, 0i32, chars)?;
        return Ok(String::new(buf, 0i32)?);
        let mut _arr3: Vec<i8> = vec![0i8; (chars).wrapping_mul(2i32) as usize];
        buf = _arr3;
        Long::formatUnsignedLong0UTF16(val, local_2, &buf, 0i32, chars)?;
        Ok(String::new(buf, 1i32)?)
    }

    // java: formatUnsignedLong0(JI[BII)V
    pub fn formatUnsignedLong0(val: i64, arg_1: i32, shift: &[i8], buf: i32, offset: i32) -> Result<()> {
        let mut charPos: i32 = (offset).wrapping_add(local_5);
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        /* TODO: i2b  */
        buf[charPos as usize] = Integer::digits()[((val as i32)&mask) as usize];
        /* TODO: lushr  */
        val = shift;
        Ok(())
    }

    // java: formatUnsignedLong0UTF16(JI[BII)V
    pub fn formatUnsignedLong0UTF16(val: i64, arg_1: i32, shift: &[i8], buf: i32, offset: i32) -> Result<()> {
        let mut charPos: i32 = (offset).wrapping_add(local_5);
        let mut radix: i32 = (1i32<<(shift&0x1f));
        let mut mask: i32 = (radix).wrapping_sub(1i32);
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(buf, charPos, Integer::digits()[((val as i32)&mask) as usize])?;
        /* TODO: lushr  */
        val = shift;
        Ok(())
    }

    // java: fastUUID(JJ)Ljava/lang/String;
    pub fn fastUUID(lsb: i64, arg_1: i64) -> Result<String> {
        let mut _arr0: Vec<i8> = vec![0i8; 36i32 as usize];
        let mut buf: Vec<i8> = _arr0;
        Long::formatUnsignedLong0(lsb, 4i32, &buf, 24i32, 12i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0(48i32, 4i32, &buf, 19i32, 4i32)?;
        Long::formatUnsignedLong0(local_2, 4i32, &buf, 14i32, 4i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0(16i32, 4i32, &buf, 9i32, 4i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0(32i32, 4i32, &buf, 0i32, 8i32)?;
        buf[23i32 as usize] = 45i32;
        buf[18i32 as usize] = 45i32;
        buf[13i32 as usize] = 45i32;
        buf[8i32 as usize] = 45i32;
        return Ok(String::new(buf, 0i32)?);
        let mut _arr1: Vec<i8> = vec![0i8; 72i32 as usize];
        buf = _arr1;
        Long::formatUnsignedLong0UTF16(lsb, 4i32, &buf, 24i32, 12i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0UTF16(48i32, 4i32, &buf, 19i32, 4i32)?;
        Long::formatUnsignedLong0UTF16(local_2, 4i32, &buf, 14i32, 4i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0UTF16(16i32, 4i32, &buf, 9i32, 4i32)?;
        /* TODO: lushr  */
        Long::formatUnsignedLong0UTF16(32i32, 4i32, &buf, 0i32, 8i32)?;
        StringUTF16::putChar(&buf, 23i32, 45i32)?;
        StringUTF16::putChar(&buf, 18i32, 45i32)?;
        StringUTF16::putChar(&buf, 13i32, 45i32)?;
        StringUTF16::putChar(&buf, 8i32, 45i32)?;
        Ok(String::new(buf, 1i32)?)
    }

    // java: toString(J)Ljava/lang/String;
    // java: toString(J)Ljava/lang/String;
    pub fn toString__l(i: i64) -> Result<String> {
        let _t0: i32 = Long::stringSize(i)?;
        let mut size: i32 = _t0;
        let mut _arr1: Vec<i8> = vec![0i8; size as usize];
        let mut buf: Vec<i8> = _arr1;
        let _t2: i32 = Long::getChars(i, size, &buf)?;
        return Ok(String::new(buf, 0i32)?);
        let mut _arr3: Vec<i8> = vec![0i8; (size).wrapping_mul(2i32) as usize];
        buf = _arr3;
        let _t4: i32 = StringUTF16::getChars(i, size, &buf)?;
        Ok(String::new(buf, 1i32)?)
    }

    // java: toUnsignedString(J)Ljava/lang/String;
    // java: toUnsignedString(J)Ljava/lang/String;
    pub fn toUnsignedString__l(i: i64) -> Result<String> {
        let _t0: String = Long::toUnsignedString__l_i(i, 10i32)?;
        Ok(_t0)
    }

    // java: getChars(JI[B)I
    pub fn getChars(i: i64, arg_1: i32, index: &[i8]) -> Result<i32> {
        let mut charPos: i32 = index;
        /* TODO: lcmp  */
        let mut negative: i32 = 0i64<0i32;
        /* TODO: lneg  */
        i = i;
        loop {
            /* TODO: lcmp  */
            if 18446744071562067968i64>0i32 { break; }
            let mut q: i64 = (i/100i64);
            let mut r: i32 = (((q).wrapping_mul(100i64)).wrapping_sub(i) as i32);
            i = q;
            charPos = charPos.wrapping_sub(1i32);
            local_3[charPos as usize] = Integer::DigitOnes()[r as usize];
            charPos = charPos.wrapping_sub(1i32);
            local_3[charPos as usize] = Integer::DigitTens()[r as usize];
        }
        let mut i2: i32 = (i as i32);
        loop {
            if i2 > -100i32 { break; }
            let mut q2: i32 = (i2/100i32);
            r = ((q2).wrapping_mul(100i32)).wrapping_sub(i2);
            i2 = q2;
            charPos = charPos.wrapping_sub(1i32);
            local_3[charPos as usize] = Integer::DigitOnes()[r as usize];
            charPos = charPos.wrapping_sub(1i32);
            local_3[charPos as usize] = Integer::DigitTens()[r as usize];
        }
        charPos = charPos.wrapping_sub(1i32);
        local_3[charPos as usize] = Integer::DigitOnes()[(i2).wrapping_neg() as usize];
        charPos = charPos.wrapping_sub(1i32);
        local_3[charPos as usize] = Integer::DigitTens()[(i2).wrapping_neg() as usize];
        charPos = charPos.wrapping_sub(1i32);
        local_3[charPos as usize] = 45i32;
        Ok(charPos)
    }

    // java: stringSize(J)I
    pub fn stringSize(x: i64) -> Result<i32> {
        let mut d: i32 = 1i32;
        /* TODO: lcmp  */
        d = 0i32;
        /* TODO: lneg  */
        x = x;
        let mut p: i64 = 18446744073709551606i64;
        let mut i: i32 = 1i32;
        loop {
            if i >= 19i32 { break; }
            /* TODO: lcmp  */
            return Ok((i).wrapping_add(d));
            p = (10i64).wrapping_mul(p);
            i = i.wrapping_add(1i32);
        }
        Ok((19i32).wrapping_add(d))
    }

    // java: parseLong(Ljava/lang/String;I)J
    // java: parseLong(Ljava/lang/String;I)J
    pub fn parseLong__str_i(s: String, radix: i32) -> Result<i64> {
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
        let mut limit: i64 = 9223372036854775809i64;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        negative = 1i32;
        limit = 9223372036854775808i64;
        let _t2: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        i = i.wrapping_add(1i32);
        let mut multmin: i64 = (limit/(radix as i64));
        let mut result: i64 = 0i64;
        loop {
            if i >= len { break; }
            i = i.wrapping_add(1i32);
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit__c_i(_t0, radix)?;
            let mut digit: i32 = _t1;
            /* TODO: lcmp  */
            let _t2: Object = NumberFormatException::forInputString(s, radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_mul((radix as i64));
            /* TODO: lcmp  */
            let _t3: Object = NumberFormatException::forInputString(s, radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_sub((digit as i64));
        }
        /* TODO: lneg  */
        return Ok(result);
        let _t4: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseLong(Ljava/lang/CharSequence;III)J
    // java: parseLong(Ljava/lang/CharSequence;III)J
    pub fn parseLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
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
        let mut limit: i64 = 9223372036854775809i64;
        let _t3 = s.charAt(i)?;
        let mut firstChar: i32 = _t3;
        negative = 1i32;
        limit = 9223372036854775808i64;
        let _t4: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        i = i.wrapping_add(1i32);
        let _t5: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut multmin: i64 = (limit/(radix as i64));
        let mut result: i64 = 0i64;
        loop {
            if i >= endIndex { break; }
            let _t0 = s.charAt(i)?;
            let _t1: i32 = Character::digit__c_i(_t0, radix)?;
            let mut digit: i32 = _t1;
            /* TODO: lcmp  */
            let _t2: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            result = (result).wrapping_mul((radix as i64));
            /* TODO: lcmp  */
            let _t3: Object = NumberFormatException::forCharSequence(s, beginIndex, endIndex, i)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            i = i.wrapping_add(1i32);
            result = (result).wrapping_sub((digit as i64));
        }
        /* TODO: lneg  */
        return Ok(result);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseLong(Ljava/lang/String;)J
    // java: parseLong(Ljava/lang/String;)J
    pub fn parseLong__str(s: String) -> Result<i64> {
        let _t0: i64 = Long::parseLong__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: parseUnsignedLong(Ljava/lang/String;I)J
    // java: parseUnsignedLong(Ljava/lang/String;I)J
    pub fn parseUnsignedLong__str_i(s: String, radix: i32) -> Result<i64> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = s.length()?;
        let mut len: i32 = _t0;
        let _t1 = s.charAt(0i32)?;
        let mut firstChar: i32 = _t1;
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = s;
        let _t3: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr2)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4: i64 = Long::parseLong__str_i(s, radix)?;
        return Ok(_t4);
        let _t5: i64 = Long::parseLong__seq_i_i_i(s, 0i32, (len).wrapping_sub(1i32), radix)?;
        let mut first: i64 = _t5;
        let _t6 = s.charAt((len).wrapping_sub(1i32))?;
        let _t7: i32 = Character::digit__c_i(_t6, radix)?;
        let mut second: i32 = _t7;
        String::new().append(&String::from("Bad digit at end of"))?;
        String::new().append(&s)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut result: i64 = ((first).wrapping_mul((radix as i64))).wrapping_add((second as i64));
        /* TODO: lushr  */
        let mut guard: i32 = (first).wrapping_mul((57i32 as i32));
        /* TODO: lcmp  */
        let mut _arr8: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr8[0i32 as usize] = s;
        let _t9: String = String::format(String::from("String value %s exceeds range of unsigned long."), &_arr8)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(result);
        let _t10: Object = NumberFormatException::forInputString(s, radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseUnsignedLong(Ljava/lang/CharSequence;III)J
    // java: parseUnsignedLong(Ljava/lang/CharSequence;III)J
    pub fn parseUnsignedLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        let _t0: Object = Objects::requireNonNull__obj(s)?;
        let _t1 = s.length()?;
        let _t2: i32 = Objects::checkFromToIndex__i_i_i(beginIndex, endIndex, _t1)?;
        let mut start: i32 = beginIndex;
        let mut len: i32 = (endIndex).wrapping_sub(beginIndex);
        let _t3 = s.charAt(start)?;
        let mut firstChar: i32 = _t3;
        let mut _arr4: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t5 = s.subSequence(start, (start).wrapping_add(len))?;
        _arr4[0i32 as usize] = _t5;
        let _t6: String = String::format(String::from("Illegal leading minus sign on unsigned string %s."), &_arr4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t7: i64 = Long::parseLong__seq_i_i_i(s, start, (start).wrapping_add(len), radix)?;
        return Ok(_t7);
        let _t8: i64 = Long::parseLong__seq_i_i_i(s, start, ((start).wrapping_add(len)).wrapping_sub(1i32), radix)?;
        let mut first: i64 = _t8;
        let _t9 = s.charAt(((start).wrapping_add(len)).wrapping_sub(1i32))?;
        let _t10: i32 = Character::digit__c_i(_t9, radix)?;
        let mut second: i32 = _t10;
        String::new().append(&String::from("Bad digit at end of"))?;
        let _t11 = s.subSequence(start, (start).wrapping_add(len))?;
        String::new().append(&_t11)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut result: i64 = ((first).wrapping_mul((radix as i64))).wrapping_add((second as i64));
        /* TODO: lushr  */
        let mut guard: i32 = (first).wrapping_mul((57i32 as i32));
        /* TODO: lcmp  */
        let mut _arr12: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t13 = s.subSequence(start, (start).wrapping_add(len))?;
        _arr12[0i32 as usize] = _t13;
        let _t14: String = String::format(String::from("String value %s exceeds range of unsigned long."), &_arr12)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(result);
        let _t15: Object = NumberFormatException::forInputString(String::from(""), radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: parseUnsignedLong(Ljava/lang/String;)J
    // java: parseUnsignedLong(Ljava/lang/String;)J
    pub fn parseUnsignedLong__str(s: String) -> Result<i64> {
        let _t0: i64 = Long::parseUnsignedLong__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Long;
    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Long;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i64> {
        let _t0: i64 = Long::parseLong__str_i(s, radix)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Long;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Long;
    pub fn valueOf__str(s: String) -> Result<i64> {
        let _t0: i64 = Long::parseLong__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(J)Ljava/lang/Long;
    // java: valueOf(J)Ljava/lang/Long;
    pub fn valueOf__l(l: i64) -> Result<i64> {
        let mut offset: i32 = 128i32;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok(Long_LongCache::cache()[((l as i32)).wrapping_add(128i32) as usize].clone());
        Ok(Long::new(l)?)
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Long;
    pub fn decode(nm: String) -> Result<i64> {
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
        let _t10: i64 = Long::parseLong__seq_i_i_i(nm, index, _t9, radix)?;
        let mut result: i64 = _t10;
        /* TODO: lneg  */
        result = result;
        let mut e: i64 = result;
        String::new().append(&String::from("-"))?;
        let _t11 = nm.substring(index)?;
        String::new().append(&_t11)?;
        let _t12 = nm.substring(index)?;
        let mut constant: String = _t12;
        let _t13: i64 = Long::parseLong__str_i(constant, radix)?;
        result = _t13;
        Ok(result)
    }

    // java: <init>(J)V
    // java: <init>(J)V
    pub fn new__l(value: i64) -> Result<Self> {
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
        let _t0: i64 = Long::parseLong__str_i(s, 10i32)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        /* TODO: i2b  */
        Ok((this.value.get() as i32))
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
        /* TODO: i2s  */
        Ok((this.value.get() as i32))
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        Ok((this.value.get() as i32))
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        Ok(this.value.get())
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        /* TODO: l2f  */
        Ok(this.value.get())
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        /* TODO: l2d  */
        Ok(this.value.get())
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Long::toString__l(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Long::hashCode__l(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(J)I
    // java: hashCode(J)I
    pub fn hashCode__l(value: i64) -> Result<i32> {
        /* TODO: lushr  */
        /* TODO: lxor  */
        Ok((32i32 as i32))
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(obj==0i32);
        Ok(0i32)
    }

    // java: getLong(Ljava/lang/String;)Ljava/lang/Long;
    // java: getLong(Ljava/lang/String;)Ljava/lang/Long;
    pub fn getLong__str(nm: String) -> Result<i64> {
        /* TODO: aconst_null  */
        let _t0: i64 = Long::getLong__str_lng(todo!("stack underflow"), nm)?;
        Ok(_t0)
    }

    // java: getLong(Ljava/lang/String;J)Ljava/lang/Long;
    // java: getLong(Ljava/lang/String;J)Ljava/lang/Long;
    pub fn getLong__str_l(nm: String, val: i64) -> Result<i64> {
        /* TODO: aconst_null  */
        let _t0: i64 = Long::getLong__str_lng(todo!("stack underflow"), nm)?;
        let mut result: i64 = _t0;
        Ok(result)
    }

    // java: getLong(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;
    // java: getLong(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;
    pub fn getLong__str_lng(nm: String, val: i64) -> Result<i64> {
        /* TODO: aconst_null  */
        let mut v: i32 = todo!("stack underflow");
        let _t0: String = System::getProperty(nm)?;
        v = _t0;
        let mut local_3: i32 = todo!("stack underflow");
        let _t1: i64 = Long::decode(v)?;
        return Ok(_t1);
        local_3 = v;
        Ok(val)
    }

    // java: compareTo(Ljava/lang/Long;)I
    pub fn compareTo(&self, anotherLong: i64) -> Result<i32> {
        let this = self;
        let _t0: i32 = Long::compare(this.value.get(), anotherLong.value.get())?;
        Ok(_t0)
    }

    // java: compare(JJ)I
    pub fn compare(x: i64, arg_1: i64) -> Result<i32> {
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        Ok(local_2!=0i32)
    }

    // java: compareUnsigned(JJ)I
    pub fn compareUnsigned(x: i64, arg_1: i64) -> Result<i32> {
        let _t0: i32 = Long::compare((x).wrapping_add(9223372036854775808i64), (local_2).wrapping_add(9223372036854775808i64))?;
        Ok(_t0)
    }

    // java: divideUnsigned(JJ)J
    pub fn divideUnsigned(dividend: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lcmp  */
        /* TODO: lushr  */
        /* TODO: lshl  */
        let mut q: i64 = 1i32;
        let mut r: i64 = (dividend).wrapping_sub((q).wrapping_mul(local_2));
        /* TODO: lxor  */
        /* TODO: lor  */
        /* TODO: lushr  */
        return Ok((18446744073709551615i64).wrapping_add(63i32));
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lushr  */
        Ok(63i32)
    }

    // java: remainderUnsigned(JJ)J
    pub fn remainderUnsigned(dividend: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lcmp  */
        /* TODO: lushr  */
        /* TODO: lshl  */
        let mut q: i64 = 1i32;
        let mut r: i64 = (dividend).wrapping_sub((q).wrapping_mul(local_2));
        /* TODO: lxor  */
        /* TODO: lshr  */
        /* TODO: land  */
        return Ok((63i32).wrapping_sub(local_2));
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lshr  */
        /* TODO: land  */
        Ok((63i32).wrapping_sub(local_2))
    }

    // java: highestOneBit(J)J
    pub fn highestOneBit(i: i64) -> Result<i64> {
        let _t0: i32 = Long::numberOfLeadingZeros(i)?;
        /* TODO: lushr  */
        /* TODO: land  */
        Ok(_t0)
    }

    // java: lowestOneBit(J)J
    pub fn lowestOneBit(i: i64) -> Result<i64> {
        /* TODO: lneg  */
        /* TODO: land  */
        Ok(i)
    }

    // java: numberOfLeadingZeros(J)I
    pub fn numberOfLeadingZeros(i: i64) -> Result<i32> {
        /* TODO: lushr  */
        let mut x: i32 = (32i32 as i32);
        let _t0: i32 = Integer::numberOfLeadingZeros((i as i32))?;
        let _t1: i32 = Integer::numberOfLeadingZeros(x)?;
        Ok(_t1)
    }

    // java: numberOfTrailingZeros(J)I
    pub fn numberOfTrailingZeros(i: i64) -> Result<i32> {
        let mut x: i32 = (i as i32);
        /* TODO: lushr  */
        let _t0: i32 = Integer::numberOfTrailingZeros((32i32 as i32))?;
        let _t1: i32 = Integer::numberOfTrailingZeros(x)?;
        Ok(_t1)
    }

    // java: bitCount(J)I
    pub fn bitCount(i: i64) -> Result<i32> {
        /* TODO: lushr  */
        /* TODO: land  */
        i = (1i32).wrapping_sub(6148914691236517205i64);
        /* TODO: land  */
        /* TODO: lushr  */
        /* TODO: land  */
        i = (2i32).wrapping_add(3689348814741910323i64);
        /* TODO: lushr  */
        /* TODO: land  */
        i = 1085102592571150095i64;
        /* TODO: lushr  */
        i = (i).wrapping_add(8i32);
        /* TODO: lushr  */
        i = (i).wrapping_add(16i32);
        /* TODO: lushr  */
        i = (i).wrapping_add(32i32);
        Ok(((i as i32)&127i32))
    }

    // java: rotateLeft(JI)J
    pub fn rotateLeft(i: i64, arg_1: i32) -> Result<i64> {
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: lor  */
        Ok((local_2).wrapping_neg())
    }

    // java: rotateRight(JI)J
    pub fn rotateRight(i: i64, arg_1: i32) -> Result<i64> {
        /* TODO: lushr  */
        /* TODO: lshl  */
        /* TODO: lor  */
        Ok((local_2).wrapping_neg())
    }

    // java: reverse(J)J
    pub fn reverse(i: i64) -> Result<i64> {
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: land  */
        /* TODO: lor  */
        i = 6148914691236517205i64;
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: land  */
        /* TODO: lor  */
        i = 3689348814741910323i64;
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: land  */
        /* TODO: lor  */
        i = 1085102592571150095i64;
        let _t0: i64 = Long::reverseBytes(i)?;
        Ok(_t0)
    }

    // java: compress(JJ)J
    pub fn compress(i: i64, arg_1: i64) -> Result<i64> {
        /* TODO: land  */
        i = local_2;
        /* TODO: lxor  */
        /* TODO: lshl  */
        let mut maskCount: i64 = 1i32;
        let mut j: i32 = 0i32;
        loop {
            if j >= 6i32 { break; }
            let _t0: i64 = Long::parallelSuffix(maskCount)?;
            let mut maskPrefix: i64 = _t0;
            /* TODO: land  */
            let mut maskMove: i64 = local_2;
            /* TODO: lxor  */
            /* TODO: lushr  */
            /* TODO: lor  */
            let mut mask: i64 = (1i32<<(j&0x1f));
            /* TODO: land  */
            let mut t: i64 = maskMove;
            /* TODO: lxor  */
            /* TODO: lushr  */
            /* TODO: lor  */
            i = (1i32<<(j&0x1f));
            /* TODO: lxor  */
            /* TODO: land  */
            maskCount = 18446744073709551615i64;
            j = j.wrapping_add(1i32);
        }
        Ok(i)
    }

    // java: expand(JJ)J
    pub fn expand(i: i64, arg_1: i64) -> Result<i64> {
        let mut originalMask: i64 = local_2;
        /* TODO: lxor  */
        /* TODO: lshl  */
        let mut maskCount: i64 = 1i32;
        let _t0: i64 = Long::parallelSuffix(maskCount)?;
        let mut maskPrefix: i64 = _t0;
        /* TODO: land  */
        let mut maskMove1: i64 = local_2;
        /* TODO: lxor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        let mut mask: i64 = 1i32;
        /* TODO: lxor  */
        /* TODO: land  */
        maskCount = 18446744073709551615i64;
        let _t1: i64 = Long::parallelSuffix(maskCount)?;
        maskPrefix = _t1;
        /* TODO: land  */
        let mut maskMove2: i64 = mask;
        /* TODO: lxor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        mask = 2i32;
        /* TODO: lxor  */
        /* TODO: land  */
        maskCount = 18446744073709551615i64;
        let _t2: i64 = Long::parallelSuffix(maskCount)?;
        maskPrefix = _t2;
        /* TODO: land  */
        let mut maskMove3: i64 = mask;
        /* TODO: lxor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        mask = 4i32;
        /* TODO: lxor  */
        /* TODO: land  */
        maskCount = 18446744073709551615i64;
        let _t3: i64 = Long::parallelSuffix(maskCount)?;
        maskPrefix = _t3;
        /* TODO: land  */
        let mut maskMove4: i64 = mask;
        /* TODO: lxor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        mask = 8i32;
        /* TODO: lxor  */
        /* TODO: land  */
        maskCount = 18446744073709551615i64;
        let _t4: i64 = Long::parallelSuffix(maskCount)?;
        maskPrefix = _t4;
        /* TODO: land  */
        let mut maskMove5: i64 = mask;
        /* TODO: lxor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        mask = 16i32;
        /* TODO: lxor  */
        /* TODO: land  */
        maskCount = 18446744073709551615i64;
        let _t5: i64 = Long::parallelSuffix(maskCount)?;
        maskPrefix = _t5;
        /* TODO: land  */
        let mut maskMove6: i64 = mask;
        /* TODO: lshl  */
        let mut t: i64 = 32i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove6;
        /* TODO: lshl  */
        t = 16i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove5;
        /* TODO: lshl  */
        t = 8i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove4;
        /* TODO: lshl  */
        t = 4i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove3;
        /* TODO: lshl  */
        t = 2i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove2;
        /* TODO: lshl  */
        t = 1i32;
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lor  */
        i = maskMove1;
        /* TODO: land  */
        Ok(originalMask)
    }

    // java: parallelSuffix(J)J
    pub fn parallelSuffix(maskCount: i64) -> Result<i64> {
        /* TODO: lshl  */
        /* TODO: lxor  */
        let mut maskPrefix: i64 = 1i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        maskPrefix = 2i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        maskPrefix = 4i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        maskPrefix = 8i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        maskPrefix = 16i32;
        /* TODO: lshl  */
        /* TODO: lxor  */
        maskPrefix = 32i32;
        Ok(maskPrefix)
    }

    // java: signum(J)I
    pub fn signum(i: i64) -> Result<i32> {
        /* TODO: lshr  */
        /* TODO: lneg  */
        /* TODO: lushr  */
        /* TODO: lor  */
        Ok((63i32 as i32))
    }

    // java: reverseBytes(J)J
    pub fn reverseBytes(i: i64) -> Result<i64> {
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: lushr  */
        /* TODO: land  */
        /* TODO: lor  */
        i = 71777214294589695i64;
        /* TODO: lshl  */
        /* TODO: land  */
        /* TODO: lshl  */
        /* TODO: lor  */
        /* TODO: lushr  */
        /* TODO: land  */
        /* TODO: lor  */
        /* TODO: lushr  */
        /* TODO: lor  */
        Ok(48i32)
    }

    // java: sum(JJ)J
    pub fn sum(a: i64, arg_1: i64) -> Result<i64> {
        Ok((a).wrapping_add(local_2))
    }

    // java: max(JJ)J
    pub fn max(a: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (a).max(local_2);
        Ok(_t0)
    }

    // java: min(JJ)J
    pub fn min(a: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (a).min(local_2);
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(this)?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i64> {
        let this = self;
        Ok(this)
    }
}
