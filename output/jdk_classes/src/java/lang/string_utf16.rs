#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringUTF16",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "StringUTF16.java",
))]
pub struct StringUTF16;

impl StringUTF16 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "newBytesFor", descriptor = "(I)[B", access = "public static"))]
    pub fn newBytesFor(len: i32) -> Result<Vec<i8>> {
        let _t0: i32 = StringUTF16::newBytesLength(len)?;
        let mut _arr1: Vec<i8> = vec![0i8; _t0 as usize];
        Ok(_arr1)
    }

    #[cfg_attr(any(), java_method(name = "newBytesLength", descriptor = "(I)I", access = "public static"))]
    pub fn newBytesLength(len: i32) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        String::new().append(&String::from("UTF16 String size is"))?;
        String::new().append(&len)?;
        String::new().append(&String::from(", should be less than"))?;
        String::new().append(&1073741823i32)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok((len<<(1i32&0x1f)))
    }

    #[cfg_attr(any(), java_method(name = "putChar", descriptor = "([BII)V", access = "static"))]
    pub fn putChar(val: &[i8], index: i32, c: i32) -> Result<()> {
        let _t0: i32 = StringUTF16::length(&val)?;
        return Err(JvmError::Custom(String::from("athrow")));
        index = (index<<(1i32&0x1f));
        index = index.wrapping_add(1i32);
        /* TODO: i2b  */
        val[index as usize] = (c>>((StringUTF16::HI_BYTE_SHIFT()&0x1f)));
        /* TODO: i2b  */
        val[index as usize] = (c>>((StringUTF16::LO_BYTE_SHIFT()&0x1f)));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getChar", descriptor = "([BI)C", access = "static"))]
    pub fn getChar(val: &[i8], index: i32) -> Result<u16> {
        let _t0: i32 = StringUTF16::length(&val)?;
        return Err(JvmError::Custom(String::from("athrow")));
        index = (index<<(1i32&0x1f));
        index = index.wrapping_add(1i32);
        /* TODO: i2c  */
        Ok((((val[index as usize]&255i32)<<(StringUTF16::HI_BYTE_SHIFT()&0x1f))|((val[index as usize]&255i32)<<(StringUTF16::LO_BYTE_SHIFT()&0x1f))))
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "([B)I", access = "public static"))]
    pub fn length(value: &[i8]) -> Result<i32> {
        Ok(((value.len() as i32)>>((1i32&0x1f))))
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([BIIZ)I", access = "private static"))]
    // java: codePointAt([BIIZ)I
    pub fn codePointAt__arr_b_i_i_z(value: &[i8], index: i32, end: i32, checked: bool) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        StringUTF16::checkIndex(index, &value)?;
        let _t0: u16 = StringUTF16::getChar(&value, index)?;
        let mut c1: i32 = _t0;
        let _t1: bool = Character::isHighSurrogate(c1)?;
        index = index.wrapping_add(1i32);
        StringUTF16::checkIndex(index, &value)?;
        let _t2: u16 = StringUTF16::getChar(&value, index)?;
        let mut c2: i32 = _t2;
        let _t3: bool = Character::isLowSurrogate(c2)?;
        let _t4: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t4);
        Ok(c1)
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([BII)I", access = "public static"))]
    // java: codePointAt([BII)I
    pub fn codePointAt__arr_b_i_i(value: &[i8], index: i32, end: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointAt(&value, index, end, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([BIZ)I", access = "private static"))]
    // java: codePointBefore([BIZ)I
    pub fn codePointBefore__arr_b_i_z(value: &[i8], index: i32, checked: bool) -> Result<i32> {
        index = index.wrapping_sub(1i32);
        StringUTF16::checkIndex(index, &value)?;
        let _t0: u16 = StringUTF16::getChar(&value, index)?;
        let mut c2: i32 = _t0;
        let _t1: bool = Character::isLowSurrogate(c2)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::checkIndex(index, &value)?;
        let _t2: u16 = StringUTF16::getChar(&value, index)?;
        let mut c1: i32 = _t2;
        let _t3: bool = Character::isHighSurrogate(c1)?;
        let _t4: i32 = Character::toCodePoint(c1, c2)?;
        return Ok(_t4);
        Ok(c2)
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public static"))]
    // java: codePointBefore([BI)I
    pub fn codePointBefore__arr_b_i(value: &[i8], index: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointBefore(&value, index, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "([BIIZ)I", access = "private static"))]
    // java: codePointCount([BIIZ)I
    pub fn codePointCount__arr_b_i_i_z(value: &[i8], beginIndex: i32, endIndex: i32, checked: bool) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut count: i32 = (endIndex).wrapping_sub(beginIndex);
        let mut i: i32 = beginIndex;
        StringUTF16::checkBoundsBeginEnd(i, endIndex, &value)?;
        loop {
            if i >= (endIndex).wrapping_sub(1i32) { break; }
            i = i.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let _t1: bool = Character::isHighSurrogate(_t0)?;
            let _t2: u16 = StringUTF16::getChar(&value, i)?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            count = count.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(count)
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "([BII)I", access = "public static"))]
    // java: codePointCount([BII)I
    pub fn codePointCount__arr_b_i_i(value: &[i8], beginIndex: i32, endIndex: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointCount(&value, beginIndex, endIndex, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toChars", descriptor = "([B)[C", access = "public static"))]
    pub fn toChars(value: &[i8]) -> Result<Vec<u16>> {
        let mut _arr0: Vec<u16> = vec![0u16; ((value.len() as i32)>>((1i32&0x1f))) as usize];
        let mut dst: Vec<u16> = _arr0;
        StringUTF16::getChars(&value, 0i32, (dst.len() as i32), &dst, 0i32)?;
        Ok(dst)
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "([CII)[B", access = "public static"))]
    // java: toBytes([CII)[B
    pub fn toBytes__arr_c_i_i(value: &[u16], off: i32, len: i32) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = StringUTF16::newBytesFor(len)?;
        let mut val: Vec<i8> = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            StringUTF16::putChar(&val, i, value[off as usize])?;
            off = off.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "coderFromArrayLen", descriptor = "([BI)B", access = "static"))]
    pub fn coderFromArrayLen(value: &[i8], len: i32) -> Result<i8> {
        /* TODO: i2b  */
        Ok((((len).wrapping_sub((value.len() as i32)) as u32>>(31i32&0x1f)) as i32))
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "([CII)[B", access = "public static"))]
    // java: compress([CII)[B
    pub fn compress__arr_c_i_i(val: &[u16], off: i32, count: i32) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; count as usize];
        let mut latin1: Vec<i8> = _arr0;
        let _t1: i32 = StringUTF16::compress(&val, off, &latin1, 0i32, count)?;
        let mut ndx: i32 = _t1;
        let _t2: Vec<i8> = StringUTF16::toBytes(&val, off, count)?;
        let mut utf16: Vec<i8> = _t2;
        let _t3: u16 = StringUTF16::getChar(&utf16, ndx)?;
        let _t4: i32 = StringUTF16::compress(&utf16, 0i32, &latin1, 0i32, count)?;
        return Ok(utf16);
        Ok(latin1)
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "([BII)[B", access = "public static"))]
    // java: compress([BII)[B
    pub fn compress__arr_b_i_i(val: &[i8], off: i32, count: i32) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; count as usize];
        let mut latin1: Vec<i8> = _arr0;
        let _t1: i32 = StringUTF16::compress(&val, off, &latin1, 0i32, count)?;
        let mut ndx: i32 = _t1;
        let _t2: i32 = StringUTF16::newBytesLength((off).wrapping_add(count))?;
        let _t3: Vec<i8> = Arrays::copyOfRange(&val, (off<<(1i32&0x1f)), _t2)?;
        let mut utf16: Vec<i8> = _t3;
        let _t4: u16 = StringUTF16::getChar(&utf16, ndx)?;
        let _t5: i32 = StringUTF16::compress(&utf16, 0i32, &latin1, 0i32, count)?;
        return Ok(utf16);
        Ok(latin1)
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "([III)[B", access = "public static"))]
    // java: compress([III)[B
    pub fn compress__arr_i_i_i(val: &[i32], off: i32, count: i32) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; count as usize];
        let mut latin1: Vec<i8> = _arr0;
        let mut end: i32 = (off).wrapping_add(count);
        let mut ndx: i32 = 0i32;
        loop {
            if ndx >= count { break; }
            let mut cp: i32 = val[off as usize];
            /* TODO: i2b  */
            latin1[ndx as usize] = cp;
            let _t0: i32 = StringUTF16::computeCodePointSize(&val, off, end)?;
            let mut estSize: i32 = (ndx).wrapping_add(_t0);
            let _t1: Vec<i8> = StringUTF16::newBytesFor(estSize)?;
            let mut utf16: Vec<i8> = _t1;
            StringLatin1::inflate(&latin1, 0i32, &utf16, 0i32, ndx)?;
            StringUTF16::putChar(&utf16, ndx, cp)?;
            off = off.wrapping_add(1i32);
            let mut i: i32 = (ndx).wrapping_add(1i32);
            StringUTF16::putChar(&utf16, i, val[off as usize])?;
            i = i.wrapping_add(1i32);
            off = off.wrapping_add(1i32);
            let _t2: Vec<i8> = StringUTF16::extractCodepoints(&val, off, end, &utf16, ndx)?;
            utf16 = _t2;
            let _t3: u16 = StringUTF16::getChar(&utf16, ndx)?;
            let _t4: i32 = StringUTF16::compress(&utf16, 0i32, &latin1, 0i32, count)?;
            return Ok(latin1);
            return Ok(utf16);
            ndx = ndx.wrapping_add(1i32);
            off = off.wrapping_add(1i32);
        }
        Ok(latin1)
    }

    #[cfg_attr(any(), java_method(name = "extractCodepoints", descriptor = "([III[BI)[B", access = "private static"))]
    pub fn extractCodepoints(val: &[i32], off: i32, end: i32, dst: &[i8], dstOff: i32) -> Result<Vec<i8>> {
        loop {
            if off >= end { break; }
            let mut codePoint: i32 = val[off as usize];
            let _t0: i32 = Character::charCount(codePoint)?;
            let mut dstLimit: i32 = ((dstOff).wrapping_add(_t0)).wrapping_add(((end).wrapping_sub(off)).wrapping_sub(1i32));
            let mut maxRemaining: i32 = (dstLimit).wrapping_add(((end).wrapping_sub(off)).wrapping_sub(1i32));
            let _t1: i32 = StringUTF16::newBytesLength(maxRemaining)?;
            let _t2: Vec<i8> = Arrays::copyOf(&dst, _t1)?;
            dst = _t2;
            let _t3: bool = Character::isBmpCodePoint(codePoint)?;
            dstOff = dstOff.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dstOff, codePoint)?;
            dstOff = dstOff.wrapping_add(1i32);
            let _t4: u16 = Character::highSurrogate(codePoint)?;
            StringUTF16::putChar(&dst, dstOff, _t4)?;
            dstOff = dstOff.wrapping_add(1i32);
            let _t5: u16 = Character::lowSurrogate(codePoint)?;
            StringUTF16::putChar(&dst, dstOff, _t5)?;
            off = off.wrapping_add(1i32);
            codePoint = val[off as usize];
        }
        let _t0: i32 = StringUTF16::newBytesLength(dstOff)?;
        let _t1: Vec<i8> = Arrays::copyOf(&dst, _t0)?;
        return Ok(_t1);
        Ok(dst)
    }

    #[cfg_attr(any(), java_method(name = "computeCodePointSize", descriptor = "([III)I", access = "private static"))]
    pub fn computeCodePointSize(val: &[i32], off: i32, end: i32) -> Result<i32> {
        let mut n: i32 = (end).wrapping_sub(off);
        loop {
            if off >= end { break; }
            off = off.wrapping_add(1i32);
            let mut codePoint: i32 = val[off as usize];
            let _t0: bool = Character::isBmpCodePoint(codePoint)?;
            let _t1: bool = Character::isValidCodePoint(codePoint)?;
            n = n.wrapping_add(1i32);
            let _t2: String = Integer::toString(codePoint)?;
            return Err(JvmError::Custom(String::from("athrow")));
        }
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "([CI[BII)I", access = "public static"))]
    // java: compress([CI[BII)I
    pub fn compress__arr_c_i_arr_b_i_i(src: &[u16], srcOff: i32, dst: &[i8], dstOff: i32, len: i32) -> Result<i32> {
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let mut c: i32 = src[srcOff as usize];
            return Ok(i);
            /* TODO: i2b  */
            dst[dstOff as usize] = c;
            srcOff = srcOff.wrapping_add(1i32);
            dstOff = dstOff.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(len)
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "([BI[BII)I", access = "public static"))]
    // java: compress([BI[BII)I
    pub fn compress__arr_b_i_arr_b_i_i(src: &[i8], srcOff: i32, dst: &[i8], dstOff: i32, len: i32) -> Result<i32> {
        StringUTF16::checkBoundsOffCount(srcOff, len, &src)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&src, srcOff)?;
            let mut c: i32 = _t0;
            return Ok(i);
            /* TODO: i2b  */
            dst[dstOff as usize] = c;
            srcOff = srcOff.wrapping_add(1i32);
            dstOff = dstOff.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(len)
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "([III)[B", access = "public static"))]
    // java: toBytes([III)[B
    pub fn toBytes__arr_i_i_i(val: &[i32], index: i32, len: i32) -> Result<Vec<i8>> {
        let mut end: i32 = (index).wrapping_add(len);
        let _t0: i32 = StringUTF16::computeCodePointSize(&val, index, end)?;
        let mut n: i32 = _t0;
        let _t1: Vec<i8> = StringUTF16::newBytesFor(n)?;
        let mut buf: Vec<i8> = _t1;
        let _t2: Vec<i8> = StringUTF16::extractCodepoints(&val, index, end, &buf, 0i32)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "(C)[B", access = "public static"))]
    // java: toBytes(C)[B
    pub fn toBytes__c(c: u16) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; 2i32 as usize];
        let mut result: Vec<i8> = _arr0;
        StringUTF16::putChar(&result, 0i32, c)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "toBytesSupplementary", descriptor = "(I)[B", access = "static"))]
    pub fn toBytesSupplementary(cp: i32) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; 4i32 as usize];
        let mut result: Vec<i8> = _arr0;
        let _t1: u16 = Character::highSurrogate(cp)?;
        StringUTF16::putChar(&result, 0i32, _t1)?;
        let _t2: u16 = Character::lowSurrogate(cp)?;
        StringUTF16::putChar(&result, 1i32, _t2)?;
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public static"))]
    // java: getChars([BII[CI)V
    pub fn getChars__arr_b_i_i_arr_c_i(value: &[i8], srcBegin: i32, srcEnd: i32, dst: &[u16], dstBegin: i32) -> Result<()> {
        StringUTF16::checkBoundsOffCount(srcBegin, (srcEnd).wrapping_sub(srcBegin), &value)?;
        let mut i: i32 = srcBegin;
        loop {
            if i >= srcEnd { break; }
            dstBegin = dstBegin.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            dst[dstBegin as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public static"))]
    pub fn getBytes(value: &[i8], srcBegin: i32, srcEnd: i32, dst: &[i8], dstBegin: i32) -> Result<()> {
        srcBegin = (srcBegin<<(1i32&0x1f));
        srcEnd = (srcEnd<<(1i32&0x1f));
        let mut i: i32 = (srcBegin).wrapping_add((1i32>>((StringUTF16::LO_BYTE_SHIFT()&0x1f))));
        loop {
            if i >= srcEnd { break; }
            dstBegin = dstBegin.wrapping_add(1i32);
            dst[dstBegin as usize] = value[i as usize];
            i = i.wrapping_add(2i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "([B[B)Z", access = "public static"))]
    pub fn equals(value: &[i8], other: &[i8]) -> Result<bool> {
        let mut len: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let _t1: u16 = StringUTF16::getChar(&other, i)?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[B)I", access = "public static"))]
    // java: compareTo([B[B)I
    pub fn compareTo__arr_b_arr_b(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringUTF16::length(&value)?;
        let mut len1: i32 = _t0;
        let _t1: i32 = StringUTF16::length(&other)?;
        let mut len2: i32 = _t1;
        let _t2: i32 = StringUTF16::compareValues(&value, &other, len1, len2)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public static"))]
    // java: compareTo([B[BII)I
    pub fn compareTo__arr_b_arr_b_i_i(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        StringUTF16::checkOffset(len1, &value)?;
        StringUTF16::checkOffset(len2, &other)?;
        let _t0: i32 = StringUTF16::compareValues(&value, &other, len1, len2)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "compareValues", descriptor = "([B[BII)I", access = "private static"))]
    pub fn compareValues(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        let _t0: i32 = (len1).min(len2);
        let mut lim: i32 = _t0;
        let mut k: i32 = 0i32;
        loop {
            if k >= lim { break; }
            let _t0: u16 = StringUTF16::getChar(&value, k)?;
            let mut c1: i32 = _t0;
            let _t1: u16 = StringUTF16::getChar(&other, k)?;
            let mut c2: i32 = _t1;
            return Ok((c1).wrapping_sub(c2));
            k = k.wrapping_add(1i32);
        }
        Ok((len1).wrapping_sub(len2))
    }

    #[cfg_attr(any(), java_method(name = "compareToLatin1", descriptor = "([B[B)I", access = "public static"))]
    // java: compareToLatin1([B[B)I
    pub fn compareToLatin1__arr_b_arr_b(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringLatin1::compareToUTF16(&other, &value)?;
        Ok((_t0).wrapping_neg())
    }

    #[cfg_attr(any(), java_method(name = "compareToLatin1", descriptor = "([B[BII)I", access = "public static"))]
    // java: compareToLatin1([B[BII)I
    pub fn compareToLatin1__arr_b_arr_b_i_i(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        let _t0: i32 = StringLatin1::compareToUTF16(&other, &value, len2, len1)?;
        Ok((_t0).wrapping_neg())
    }

    #[cfg_attr(any(), java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public static"))]
    pub fn compareToCI(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringUTF16::length(&value)?;
        let _t1: i32 = StringUTF16::length(&other)?;
        let _t2: i32 = StringUTF16::compareToCIImpl(&value, 0i32, _t0, &other, 0i32, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "compareToCIImpl", descriptor = "([BII[BII)I", access = "private static"))]
    pub fn compareToCIImpl(value: &[i8], toffset: i32, tlen: i32, other: &[i8], ooffset: i32, olen: i32) -> Result<i32> {
        let mut tlast: i32 = (toffset).wrapping_add(tlen);
        let mut olast: i32 = (ooffset).wrapping_add(olen);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = StringUTF16::length(&value)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: i32 = StringUTF16::length(&other)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut k1: i32 = toffset;
        let mut k2: i32 = ooffset;
        loop {
            if k1 >= tlast { break; }
            let _t0: u16 = StringUTF16::getChar(&value, k1)?;
            let mut cp1: i32 = _t0;
            let _t1: u16 = StringUTF16::getChar(&other, k2)?;
            let mut cp2: i32 = _t1;
            let _t2: i32 = StringUTF16::compareCodePointCI(cp1, cp2)?;
            let _t3: i32 = StringUTF16::codePointIncluding(&value, cp1, k1, toffset, tlast)?;
            cp1 = _t3;
            k1 = k1.wrapping_add(1i32);
            cp1 = (cp1).wrapping_neg();
            let _t4: i32 = StringUTF16::codePointIncluding(&other, cp2, k2, ooffset, olast)?;
            cp2 = _t4;
            k2 = k2.wrapping_add(1i32);
            cp2 = (cp2).wrapping_neg();
            let _t5: i32 = StringUTF16::compareCodePointCI(cp1, cp2)?;
            let mut diff: i32 = _t5;
            return Ok(diff);
            k1 = k1.wrapping_add(1i32);
            k2 = k2.wrapping_add(1i32);
        }
        Ok((tlen).wrapping_sub(olen))
    }

    #[cfg_attr(any(), java_method(name = "compareCodePointCI", descriptor = "(II)I", access = "private static"))]
    pub fn compareCodePointCI(cp1: i32, cp2: i32) -> Result<i32> {
        let _t0: i32 = Character::toUpperCase(cp1)?;
        cp1 = _t0;
        let _t1: i32 = Character::toUpperCase(cp2)?;
        cp2 = _t1;
        let _t2: i32 = Character::toLowerCase(cp1)?;
        cp1 = _t2;
        let _t3: i32 = Character::toLowerCase(cp2)?;
        cp2 = _t3;
        return Ok((cp1).wrapping_sub(cp2));
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "codePointIncluding", descriptor = "([BIIII)I", access = "private static"))]
    pub fn codePointIncluding(ba: &[i8], cp: i32, index: i32, start: i32, end: i32) -> Result<i32> {
        /* TODO: i2c  */
        let _t0: bool = Character::isSurrogate(cp)?;
        return Ok(cp);
        /* TODO: i2c  */
        let _t1: bool = Character::isLowSurrogate(cp)?;
        let _t2: u16 = StringUTF16::getChar(&ba, (index).wrapping_sub(1i32))?;
        let mut c: i32 = _t2;
        let _t3: bool = Character::isHighSurrogate(c)?;
        /* TODO: i2c  */
        let _t4: i32 = Character::toCodePoint(c, cp)?;
        return Ok(_t4);
        let _t5: u16 = StringUTF16::getChar(&ba, (index).wrapping_add(1i32))?;
        c = _t5;
        let _t6: bool = Character::isLowSurrogate(c)?;
        /* TODO: i2c  */
        let _t7: i32 = Character::toCodePoint(cp, c)?;
        return Ok((_t7).wrapping_neg());
        Ok(cp)
    }

    #[cfg_attr(any(), java_method(name = "compareToCI_Latin1", descriptor = "([B[B)I", access = "public static"))]
    pub fn compareToCI_Latin1(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringLatin1::compareToCI_UTF16(&other, &value)?;
        Ok((_t0).wrapping_neg())
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "([B)I", access = "public static"))]
    pub fn hashCode(value: &[i8]) -> Result<i32> {
        /* TODO: lookupswitch default:40 0:28 2:32 */
        let _t0: u16 = StringUTF16::getChar(&value, 0i32)?;
        let _t1: i32 = ArraysSupport::vectorizedHashCode(&value, 0i32, ((value.len() as i32)>>((1i32&0x1f))), 0i32, 5i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BIII)I", access = "public static"))]
    // java: indexOf([BIII)I
    pub fn indexOf__arr_b_i_i_i(value: &[i8], ch: i32, fromIndex: i32, toIndex: i32) -> Result<i32> {
        let _t0: i32 = (fromIndex).max(0i32);
        fromIndex = _t0;
        let _t1: i32 = (toIndex).min(((value.len() as i32)>>((1i32&0x1f))));
        toIndex = _t1;
        return Ok(-1i32);
        let _t2: i32 = StringUTF16::indexOfChar(&value, ch, fromIndex, toIndex)?;
        return Ok(_t2);
        let _t3: i32 = StringUTF16::indexOfSupplementary(&value, ch, fromIndex, toIndex)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([B[B)I", access = "public static"))]
    // java: indexOf([B[B)I
    pub fn indexOf__arr_b_arr_b(value: &[i8], str: &[i8]) -> Result<i32> {
        return Ok(0i32);
        return Ok(-1i32);
        let _t0: i32 = StringUTF16::length(&value)?;
        let _t1: i32 = StringUTF16::length(&str)?;
        let _t2: i32 = StringUTF16::indexOfUnsafe(&value, _t0, &str, _t1, 0i32)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public static"))]
    // java: indexOf([BI[BII)I
    pub fn indexOf__arr_b_i_arr_b_i_i(value: &[i8], valueCount: i32, str: &[i8], strCount: i32, fromIndex: i32) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(fromIndex, valueCount, &value)?;
        StringUTF16::checkBoundsBeginEnd(0i32, strCount, &str)?;
        let _t0: i32 = StringUTF16::indexOfUnsafe(&value, valueCount, &str, strCount, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOfUnsafe", descriptor = "([BI[BII)I", access = "private static"))]
    pub fn indexOfUnsafe(value: &[i8], valueCount: i32, str: &[i8], strCount: i32, fromIndex: i32) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = StringUTF16::length(&str)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: u16 = StringUTF16::getChar(&str, 0i32)?;
        let mut first: i32 = _t1;
        let mut max: i32 = (valueCount).wrapping_sub(strCount);
        let mut i: i32 = fromIndex;
        loop {
            if i > max { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            i = i.wrapping_add(1i32);
            let _t1: u16 = StringUTF16::getChar(&value, i)?;
            let mut j: i32 = (i).wrapping_add(1i32);
            let mut end: i32 = ((j).wrapping_add(strCount)).wrapping_sub(1i32);
            let mut k: i32 = 1i32;
            let _t2: u16 = StringUTF16::getChar(&value, j)?;
            let _t3: u16 = StringUTF16::getChar(&str, k)?;
            j = j.wrapping_add(1i32);
            k = k.wrapping_add(1i32);
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "indexOfLatin1", descriptor = "([B[B)I", access = "public static"))]
    // java: indexOfLatin1([B[B)I
    pub fn indexOfLatin1__arr_b_arr_b(value: &[i8], str: &[i8]) -> Result<i32> {
        return Ok(0i32);
        let _t0: i32 = StringUTF16::length(&value)?;
        return Ok(-1i32);
        let _t1: i32 = StringUTF16::length(&value)?;
        let _t2: i32 = StringUTF16::indexOfLatin1Unsafe(&value, _t1, &str, (str.len() as i32), 0i32)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "indexOfLatin1", descriptor = "([BI[BII)I", access = "public static"))]
    // java: indexOfLatin1([BI[BII)I
    pub fn indexOfLatin1__arr_b_i_arr_b_i_i(src: &[i8], srcCount: i32, tgt: &[i8], tgtCount: i32, fromIndex: i32) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(fromIndex, srcCount, &src)?;
        String::checkBoundsBeginEnd(0i32, tgtCount, (tgt.len() as i32))?;
        let _t0: i32 = StringUTF16::indexOfLatin1Unsafe(&src, srcCount, &tgt, tgtCount, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOfLatin1Unsafe", descriptor = "([BI[BII)I", access = "public static"))]
    pub fn indexOfLatin1Unsafe(src: &[i8], srcCount: i32, tgt: &[i8], tgtCount: i32, fromIndex: i32) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: i2c  */
        let mut first: i32 = (tgt[0i32 as usize]&255i32);
        let mut max: i32 = (srcCount).wrapping_sub(tgtCount);
        let mut i: i32 = fromIndex;
        loop {
            if i > max { break; }
            let _t0: u16 = StringUTF16::getChar(&src, i)?;
            i = i.wrapping_add(1i32);
            let _t1: u16 = StringUTF16::getChar(&src, i)?;
            let mut j: i32 = (i).wrapping_add(1i32);
            let mut end: i32 = ((j).wrapping_add(tgtCount)).wrapping_sub(1i32);
            let mut k: i32 = 1i32;
            let _t2: u16 = StringUTF16::getChar(&src, j)?;
            j = j.wrapping_add(1i32);
            k = k.wrapping_add(1i32);
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private static"))]
    pub fn indexOfChar(value: &[i8], ch: i32, fromIndex: i32, max: i32) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(fromIndex, max, &value)?;
        let _t0: i32 = StringUTF16::indexOfCharUnsafe(&value, ch, fromIndex, max)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOfCharUnsafe", descriptor = "([BIII)I", access = "private static"))]
    pub fn indexOfCharUnsafe(value: &[i8], ch: i32, fromIndex: i32, max: i32) -> Result<i32> {
        let mut i: i32 = fromIndex;
        loop {
            if i >= max { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "indexOfSupplementary", descriptor = "([BIII)I", access = "private static"))]
    pub fn indexOfSupplementary(value: &[i8], ch: i32, fromIndex: i32, max: i32) -> Result<i32> {
        let _t0: bool = Character::isValidCodePoint(ch)?;
        let _t1: u16 = Character::highSurrogate(ch)?;
        let mut hi: i32 = _t1;
        let _t2: u16 = Character::lowSurrogate(ch)?;
        let mut lo: i32 = _t2;
        StringUTF16::checkBoundsBeginEnd(fromIndex, max, &value)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= (max).wrapping_sub(1i32) { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let _t1: u16 = StringUTF16::getChar(&value, (i).wrapping_add(1i32))?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public static"))]
    // java: lastIndexOf([BI[BII)I
    pub fn lastIndexOf__arr_b_i_arr_b_i_i(src: &[i8], srcCount: i32, tgt: &[i8], tgtCount: i32, fromIndex: i32) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: i32 = StringUTF16::length(&tgt)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut min: i32 = (tgtCount).wrapping_sub(1i32);
        let mut i: i32 = (min).wrapping_add(fromIndex);
        let mut strLastIndex: i32 = (tgtCount).wrapping_sub(1i32);
        StringUTF16::checkIndex(strLastIndex, &tgt)?;
        let _t1: u16 = StringUTF16::getChar(&tgt, strLastIndex)?;
        let mut strLastChar: i32 = _t1;
        StringUTF16::checkIndex(i, &src)?;
        loop {
            let _t0: u16 = StringUTF16::getChar(&src, i)?;
            i = i.wrapping_sub(1i32);
            return Ok(-1i32);
            let mut j: i32 = (i).wrapping_sub(1i32);
            let mut start: i32 = (j).wrapping_sub(strLastIndex);
            let mut k: i32 = (strLastIndex).wrapping_sub(1i32);
            if j <= start { break; }
            j = j.wrapping_sub(1i32);
            let _t0: u16 = StringUTF16::getChar(&src, j)?;
            k = k.wrapping_sub(1i32);
            let _t1: u16 = StringUTF16::getChar(&tgt, k)?;
            i = i.wrapping_sub(1i32);
        }
        Ok((start).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public static"))]
    // java: lastIndexOf([BII)I
    pub fn lastIndexOf__arr_b_i_i(value: &[i8], ch: i32, fromIndex: i32) -> Result<i32> {
        let _t0: i32 = (fromIndex).min((((value.len() as i32)>>((1i32&0x1f)))).wrapping_sub(1i32));
        let mut i: i32 = _t0;
        loop {
            if i<0i32 { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        return Ok(-1i32);
        let _t1: i32 = StringUTF16::lastIndexOfSupplementary(&value, ch, fromIndex)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfSupplementary", descriptor = "([BII)I", access = "private static"))]
    pub fn lastIndexOfSupplementary(value: &[i8], ch: i32, fromIndex: i32) -> Result<i32> {
        let _t0: bool = Character::isValidCodePoint(ch)?;
        let _t1: u16 = Character::highSurrogate(ch)?;
        let mut hi: i32 = _t1;
        let _t2: u16 = Character::lowSurrogate(ch)?;
        let mut lo: i32 = _t2;
        let _t3: i32 = (fromIndex).min((((value.len() as i32)>>((1i32&0x1f)))).wrapping_sub(2i32));
        let mut i: i32 = _t3;
        loop {
            if i<0i32 { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let _t1: u16 = StringUTF16::getChar(&value, (i).wrapping_add(1i32))?;
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public static"))]
    // java: replace([BCC)Ljava/lang/String;
    pub fn replace__arr_b_c_c(value: &[i8], oldChar: u16, newChar: u16) -> Result<String> {
        let mut len: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut i: i32 = -1i32;
        i = i.wrapping_add(1i32);
        let _t0: u16 = StringUTF16::getChar(&value, i)?;
        let mut _arr1: Vec<i8> = vec![0i8; (value.len() as i32) as usize];
        let mut buf: Vec<i8> = _arr1;
        let mut j: i32 = 0i32;
        loop {
            if j >= i { break; }
            let _t0: u16 = StringUTF16::getChar(&value, j)?;
            StringUTF16::putChar(&buf, j, _t0)?;
            j = j.wrapping_add(1i32);
        }
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            j = _t0;
            StringUTF16::putChar(oldChar, newChar, j)?;
            i = i.wrapping_add(1i32);
        }
        let _t2: bool = StringLatin1::canEncode(oldChar)?;
        let _t3: bool = StringLatin1::canEncode(newChar)?;
        let _t4: Vec<i8> = StringUTF16::compress(&buf, 0i32, len)?;
        j = _t4;
        let _t5: i8 = StringUTF16::coderFromArrayLen(j, len)?;
        let mut coder: i32 = _t5;
        return Ok(String::new(j, coder)?);
        return Ok(String::new(buf, 1i32)?);
        /* TODO: aconst_null  */
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BIZ[BIZ[BIZ)Ljava/lang/String;", access = "public static"))]
    // java: replace([BIZ[BIZ[BIZ)Ljava/lang/String;
    pub fn replace__arr_b_i_z_arr_b_i_z_arr_b_i_z(value: &[i8], valLen: i32, valLat1: bool, targ: &[i8], targLen: i32, targLat1: bool, repl: &[i8], replLen: i32, replLat1: bool) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        return Ok(targLat1);
        let _t0: i32 = StringLatin1::indexOf(&value, &targ)?;
        let _t1: i32 = StringUTF16::indexOfLatin1(&value, &targ)?;
        let _t2: i32 = StringUTF16::indexOf(&value, &targ)?;
        let mut i: i32 = _t2;
        /* TODO: aconst_null  */
        return Ok(i);
        let mut p: i32 = 0i32;
        let mut _arr3: Vec<i32> = vec![0i32; 16i32 as usize];
        let mut pos: Vec<i32> = _arr3;
        pos[0i32 as usize] = i;
        i = (i).wrapping_add(targLen);
        loop {
            let _t0: i32 = StringLatin1::indexOf(&value, valLen, &targ, targLen, i)?;
            let _t1: i32 = StringUTF16::indexOfLatin1(&value, valLen, &targ, targLen, i)?;
            let _t2: i32 = StringUTF16::indexOf(&value, valLen, &targ, targLen, i)?;
            let mut j: i32 = _t2;
            if _t2<=0i32 { break; }
            p = p.wrapping_add(1i32);
            let _t0: i32 = ArraysSupport::newLength(p, 1i32, (p>>((1i32&0x1f))))?;
            let _t1: Vec<i32> = Arrays::copyOf(&pos, _t0)?;
            pos = _t1;
            pos[p as usize] = j;
            i = (j).wrapping_add(targLen);
        }
        p = p.wrapping_add(1i32);
        let _t4: i32 = (p).abs();
        let _t5: i32 = (valLen).abs();
        let mut resultLen: i32 = _t5;
        let mut ignored: i32 = _t1;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(String::from(""));
        let _t6: Vec<i8> = StringUTF16::newBytesFor(resultLen)?;
        ignored = _t6;
        let mut posFrom: i32 = 0i32;
        let mut posTo: i32 = 0i32;
        let mut q: i32 = 0i32;
        loop {
            if q >= p { break; }
            let mut nextPos: i32 = pos[q as usize];
            posFrom = posFrom.wrapping_add(1i32);
            /* TODO: i2c  */
            let mut c: i32 = (value[posFrom as usize]&255i32);
            posTo = posTo.wrapping_add(1i32);
            StringUTF16::putChar(ignored, posTo, c)?;
            posTo = posTo.wrapping_add(1i32);
            posFrom = posFrom.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&value, posFrom)?;
            StringUTF16::putChar(ignored, posTo, _t0)?;
            posFrom = (posFrom).wrapping_add(targLen);
            c = 0i32;
            /* TODO: i2c  */
            let mut c: i32 = (repl[c as usize]&255i32);
            posTo = posTo.wrapping_add(1i32);
            StringUTF16::putChar(ignored, posTo, c)?;
            c = c.wrapping_add(1i32);
            c = 0i32;
            posTo = posTo.wrapping_add(1i32);
            let _t1: u16 = StringUTF16::getChar(&repl, c)?;
            StringUTF16::putChar(ignored, posTo, _t1)?;
            c = c.wrapping_add(1i32);
            q = q.wrapping_add(1i32);
        }
        loop {
            if posFrom >= valLen { break; }
            posFrom = posFrom.wrapping_add(1i32);
            /* TODO: i2c  */
            q = (value[posFrom as usize]&255i32);
            posTo = posTo.wrapping_add(1i32);
            StringUTF16::putChar(ignored, posTo, q)?;
        }
        loop {
            if posFrom >= valLen { break; }
            posTo = posTo.wrapping_add(1i32);
            posFrom = posFrom.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&value, posFrom)?;
            StringUTF16::putChar(ignored, posTo, _t0)?;
        }
        let _t7: Vec<i8> = StringUTF16::compress(ignored, 0i32, resultLen)?;
        q = _t7;
        let _t8: i8 = StringUTF16::coderFromArrayLen(q, resultLen)?;
        nextPos = _t8;
        return Ok(String::new(q, nextPos)?);
        Ok(String::new(ignored, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public static"))]
    pub fn regionMatchesCI(value: &[i8], toffset: i32, other: &[i8], ooffset: i32, len: i32) -> Result<bool> {
        let _t0: i32 = StringUTF16::compareToCIImpl(&value, toffset, len, &other, ooffset, len)?;
        Ok(_t0==0i32)
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI_Latin1", descriptor = "([BI[BII)Z", access = "public static"))]
    pub fn regionMatchesCI_Latin1(value: &[i8], toffset: i32, other: &[i8], ooffset: i32, len: i32) -> Result<bool> {
        let _t0: bool = StringLatin1::regionMatchesCI_UTF16(&other, ooffset, &value, toffset, len)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public static"))]
    pub fn toLowerCase(str: String, value: &[i8], locale: Object) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut hasSurr: i32 = 0i32;
        let mut len: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut first: i32 = 0i32;
        loop {
            if first >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, first)?;
            let mut cp: i32 = _t0;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(cp)?;
            hasSurr = 1i32;
            let _t2: i32 = Character::toLowerCase(cp)?;
            first = first.wrapping_add(1i32);
        }
        return Ok(str);
        let mut _arr0: Vec<i8> = vec![0i8; (value.len() as i32) as usize];
        cp = _arr0;
        System::arraycopy(&value, 0i32, cp, 0i32, (first<<(1i32&0x1f)))?;
        let _t1 = locale.getLanguage()?;
        let mut lang: String = _t1;
        let _t2: String = StringUTF16::toLowerCaseEx(str, &value, cp, first, locale, 1i32)?;
        return Ok(_t2);
        let _t3: String = StringUTF16::toLowerCaseEx(str, &value, cp, first, locale, 0i32)?;
        return Ok(_t3);
        let mut bits: i32 = 0i32;
        let mut i: i32 = first;
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let mut cp: i32 = _t0;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(cp)?;
            let _t2: String = StringUTF16::toLowerCaseEx(str, &value, cp, i, locale, 0i32)?;
            return Ok(_t2);
            let _t3: String = StringUTF16::toLowerCaseEx(str, &value, cp, i, locale, 1i32)?;
            return Ok(_t3);
            let _t4: i32 = Character::toLowerCase(cp)?;
            cp = _t4;
            let _t5: bool = Character::isBmpCodePoint(cp)?;
            let _t6: String = StringUTF16::toLowerCaseEx(str, &value, cp, i, locale, 0i32)?;
            return Ok(_t6);
            bits = (bits|cp);
            StringUTF16::putChar(cp, i, cp)?;
            i = i.wrapping_add(1i32);
        }
        return Ok(String::new(cp, 1i32)?);
        let _t4: String = StringUTF16::newString(cp, 0i32, len)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private static"))]
    pub fn toLowerCaseEx(str: String, value: &[i8], result: &[i8], first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut resultOffset: i32 = first;
        let mut length: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut i: i32 = first;
        loop {
            if i >= length { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let mut srcChar: i32 = _t0;
            let mut srcCount: i32 = 1i32;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(srcChar)?;
            let _t2: i32 = StringUTF16::codePointAt(&value, i, length)?;
            srcChar = _t2;
            let _t3: i32 = Character::charCount(srcChar)?;
            srcCount = _t3;
            let _t4: i32 = ConditionalSpecialCasing::toLowerCaseEx(str, i, locale)?;
            let mut lowerChar: i32 = _t4;
            let _t5: i32 = Character::toLowerCase(srcChar)?;
            lowerChar = _t5;
            let _t6: bool = Character::isBmpCodePoint(lowerChar)?;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, lowerChar)?;
            let _t7: Vec<u16> = ConditionalSpecialCasing::toLowerCaseCharArray(str, i, locale)?;
            let mut lowerCharArray: Vec<u16> = _t7;
            let _t8: Vec<u16> = Character::toChars(lowerChar)?;
            lowerCharArray = _t8;
            let mut mapLen: i32 = (lowerCharArray.len() as i32);
            let _t9: Vec<i8> = StringUTF16::newBytesFor(((((result.len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(srcCount))?;
            let mut result2: Vec<i8> = _t9;
            System::arraycopy(&result, 0i32, &result2, 0i32, (resultOffset<<(1i32&0x1f)))?;
            result = result2;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t10: i32 = StringUTF16::length(&result)?;
            return Err(JvmError::Custom(String::from("athrow")));
            result2 = 0i32;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, lowerCharArray[result2 as usize])?;
            result2 = result2.wrapping_add(1i32);
            i = (i).wrapping_add(srcCount);
        }
        let _t0: String = StringUTF16::newString(&result, 0i32, resultOffset)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public static"))]
    pub fn toUpperCase(str: String, value: &[i8], locale: Object) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut hasSurr: i32 = 0i32;
        let mut len: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut first: i32 = 0i32;
        loop {
            if first >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, first)?;
            let mut cp: i32 = _t0;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(cp)?;
            hasSurr = 1i32;
            let _t2: i32 = Character::toUpperCaseEx(cp)?;
            first = first.wrapping_add(1i32);
        }
        return Ok(str);
        let mut _arr0: Vec<i8> = vec![0i8; (value.len() as i32) as usize];
        cp = _arr0;
        System::arraycopy(&value, 0i32, cp, 0i32, (first<<(1i32&0x1f)))?;
        let _t1 = locale.getLanguage()?;
        let mut lang: String = _t1;
        let _t2: String = StringUTF16::toUpperCaseEx(str, &value, cp, first, locale, 1i32)?;
        return Ok(_t2);
        let _t3: String = StringUTF16::toUpperCaseEx(str, &value, cp, first, locale, 0i32)?;
        return Ok(_t3);
        let mut bits: i32 = 0i32;
        let mut i: i32 = first;
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let mut cp: i32 = _t0;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(cp)?;
            let _t2: String = StringUTF16::toUpperCaseEx(str, &value, cp, i, locale, 0i32)?;
            return Ok(_t2);
            let _t3: i32 = Character::toUpperCaseEx(cp)?;
            cp = _t3;
            let _t4: bool = Character::isBmpCodePoint(cp)?;
            let _t5: String = StringUTF16::toUpperCaseEx(str, &value, cp, i, locale, 0i32)?;
            return Ok(_t5);
            bits = (bits|cp);
            StringUTF16::putChar(cp, i, cp)?;
            i = i.wrapping_add(1i32);
        }
        return Ok(String::new(cp, 1i32)?);
        let _t4: String = StringUTF16::newString(cp, 0i32, len)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private static"))]
    pub fn toUpperCaseEx(str: String, value: &[i8], result: &[i8], first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut resultOffset: i32 = first;
        let mut length: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut i: i32 = first;
        loop {
            if i >= length { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let mut srcChar: i32 = _t0;
            let mut srcCount: i32 = 1i32;
            /* TODO: i2c  */
            let _t1: bool = Character::isSurrogate(srcChar)?;
            let _t2: i32 = StringUTF16::codePointAt(&value, i, length)?;
            srcChar = _t2;
            let _t3: i32 = Character::charCount(srcChar)?;
            srcCount = _t3;
            let _t4: i32 = ConditionalSpecialCasing::toUpperCaseEx(str, i, locale)?;
            let mut upperChar: i32 = _t4;
            let _t5: i32 = Character::toUpperCaseEx(srcChar)?;
            upperChar = _t5;
            let _t6: bool = Character::isBmpCodePoint(upperChar)?;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, upperChar)?;
            let _t7: Vec<u16> = ConditionalSpecialCasing::toUpperCaseCharArray(str, i, locale)?;
            let mut upperCharArray: Vec<u16> = _t7;
            let _t8: Vec<u16> = Character::toUpperCaseCharArray(srcChar)?;
            upperCharArray = _t8;
            let _t9: Vec<u16> = Character::toChars(upperChar)?;
            upperCharArray = _t9;
            let mut mapLen: i32 = (upperCharArray.len() as i32);
            let _t10: Vec<i8> = StringUTF16::newBytesFor(((((result.len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(srcCount))?;
            let mut result2: Vec<i8> = _t10;
            System::arraycopy(&result, 0i32, &result2, 0i32, (resultOffset<<(1i32&0x1f)))?;
            result = result2;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t11: i32 = StringUTF16::length(&result)?;
            return Err(JvmError::Custom(String::from("athrow")));
            result2 = 0i32;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, upperCharArray[result2 as usize])?;
            result2 = result2.wrapping_add(1i32);
            i = (i).wrapping_add(srcCount);
        }
        let _t0: String = StringUTF16::newString(&result, 0i32, resultOffset)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn trim(value: &[i8]) -> Result<String> {
        let mut length: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut len: i32 = length;
        let mut st: i32 = 0i32;
        loop {
            if st >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, st)?;
            st = st.wrapping_add(1i32);
        }
        loop {
            if st >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, (len).wrapping_sub(1i32))?;
            len = len.wrapping_sub(1i32);
        }
        let _t0: Vec<i8> = Arrays::copyOfRange(&value, (st<<(1i32&0x1f)), (len<<(1i32&0x1f)))?;
        /* TODO: aconst_null  */
        Ok(String::new(_t0, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public static"))]
    pub fn indexOfNonWhitespace(value: &[i8]) -> Result<i32> {
        let mut length: i32 = ((value.len() as i32)>>((1i32&0x1f)));
        let mut left: i32 = 0i32;
        loop {
            if left >= length { break; }
            let _t0: i32 = StringUTF16::codePointAt(&value, left, length)?;
            let mut codepoint: i32 = _t0;
            let _t1: bool = Character::isWhitespace(codepoint)?;
            let _t2: i32 = Character::charCount(codepoint)?;
            left = (left).wrapping_add(_t2);
        }
        Ok(left)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public static"))]
    pub fn lastIndexOfNonWhitespace(value: &[i8]) -> Result<i32> {
        let mut length: i32 = (((value.len() as i32) as u32>>(1i32&0x1f)) as i32);
        let mut right: i32 = length;
        loop {
            if 0i32 >= right { break; }
            let _t0: i32 = StringUTF16::codePointBefore(&value, right)?;
            let mut codepoint: i32 = _t0;
            let _t1: bool = Character::isWhitespace(codepoint)?;
            let _t2: i32 = Character::charCount(codepoint)?;
            right = (right).wrapping_sub(_t2);
        }
        Ok(right)
    }

    #[cfg_attr(any(), java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn strip(value: &[i8]) -> Result<String> {
        let mut length: i32 = (((value.len() as i32) as u32>>(1i32&0x1f)) as i32);
        let _t0: i32 = StringUTF16::indexOfNonWhitespace(&value)?;
        let mut left: i32 = _t0;
        return Ok(String::from(""));
        let _t1: i32 = StringUTF16::lastIndexOfNonWhitespace(&value)?;
        let mut right: i32 = _t1;
        let mut ifChanged: i32 = right < length;
        let _t2: String = StringUTF16::newString(&value, left, (right).wrapping_sub(left))?;
        /* TODO: aconst_null  */
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn stripLeading(value: &[i8]) -> Result<String> {
        let mut length: i32 = (((value.len() as i32) as u32>>(1i32&0x1f)) as i32);
        let _t0: i32 = StringUTF16::indexOfNonWhitespace(&value)?;
        let mut left: i32 = _t0;
        let _t1: String = StringUTF16::newString(&value, left, (length).wrapping_sub(left))?;
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn stripTrailing(value: &[i8]) -> Result<String> {
        let mut length: i32 = (((value.len() as i32) as u32>>(1i32&0x1f)) as i32);
        let _t0: i32 = StringUTF16::lastIndexOfNonWhitespace(&value)?;
        let mut right: i32 = _t0;
        let _t1: String = StringUTF16::newString(&value, 0i32, right)?;
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "static"))]
    pub fn lines(value: &[i8]) -> Result<Object> {
        let _t0: Object = StringUTF16$LinesSpliterator::spliterator(&value)?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "putChars", descriptor = "([BI[CII)V", access = "private static"))]
    pub fn putChars(val: &[i8], index: i32, str: &[u16], off: i32, end: i32) -> Result<()> {
        loop {
            if off >= end { break; }
            index = index.wrapping_add(1i32);
            off = off.wrapping_add(1i32);
            StringUTF16::putChar(&val, index, str[off as usize])?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public static"))]
    pub fn newString(val: &[i8], index: i32, len: i32) -> Result<String> {
        return Ok(String::from(""));
        let _t0: Vec<i8> = StringUTF16::compress(&val, index, len)?;
        let mut res: Vec<i8> = _t0;
        let _t1: i8 = StringUTF16::coderFromArrayLen(&res, len)?;
        let mut coder: i32 = _t1;
        return Ok(String::new(res, coder)?);
        res = (index).wrapping_add(len);
        let _t2: Vec<i8> = Arrays::copyOfRange(&val, (index<<(1i32&0x1f)), (res<<(1i32&0x1f)))?;
        Ok(String::new(_t2, 1i32)?)
    }

    #[cfg_attr(any(), java_method(name = "fillNull", descriptor = "([BII)V", access = "public static"))]
    pub fn fillNull(val: &[i8], index: i32, end: i32) -> Result<()> {
        Arrays::fill(&val, (index<<(1i32&0x1f)), (end<<(1i32&0x1f)), 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putCharSB", descriptor = "([BII)V", access = "public static"))]
    pub fn putCharSB(val: &[i8], index: i32, c: i32) -> Result<()> {
        StringUTF16::checkIndex(index, &val)?;
        StringUTF16::putChar(&val, index, c)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putCharsSB", descriptor = "([BI[CII)V", access = "public static"))]
    // java: putCharsSB([BI[CII)V
    pub fn putCharsSB__arr_b_i_arr_c_i_i(val: &[i8], index: i32, ca: &[u16], off: i32, end: i32) -> Result<()> {
        StringUTF16::checkBoundsBeginEnd(index, ((index).wrapping_add(end)).wrapping_sub(off), &val)?;
        StringUTF16::putChars(&val, index, &ca, off, end)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putCharsSB", descriptor = "([BILjava/lang/CharSequence;II)V", access = "public static"))]
    // java: putCharsSB([BILjava/lang/CharSequence;II)V
    pub fn putCharsSB__arr_b_i_seq_i_i(val: &[i8], index: i32, s: Object, off: i32, end: i32) -> Result<()> {
        StringUTF16::checkBoundsBeginEnd(index, ((index).wrapping_add(end)).wrapping_sub(off), &val)?;
        let mut i: i32 = off;
        loop {
            if i >= end { break; }
            index = index.wrapping_add(1i32);
            let _t0 = s.charAt(i)?;
            StringUTF16::putChar(&val, index, _t0)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "codePointAtSB", descriptor = "([BII)I", access = "public static"))]
    pub fn codePointAtSB(val: &[i8], index: i32, end: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointAt(&val, index, end, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointBeforeSB", descriptor = "([BI)I", access = "public static"))]
    pub fn codePointBeforeSB(val: &[i8], index: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointBefore(&val, index, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointCountSB", descriptor = "([BII)I", access = "public static"))]
    pub fn codePointCountSB(val: &[i8], beginIndex: i32, endIndex: i32) -> Result<i32> {
        let _t0: i32 = StringUTF16::codePointCount(&val, beginIndex, endIndex, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(III[B)I", access = "public static"))]
    // java: getChars(III[B)I
    pub fn getChars__i_i_i_arr_b(i: i32, begin: i32, end: i32, value: &[i8]) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(begin, end, &value)?;
        let _t0: i32 = StringUTF16::getChars(i, end, &value)?;
        let mut pos: i32 = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(pos)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(JII[B)I", access = "public static"))]
    // java: getChars(JII[B)I
    pub fn getChars__l_i_i_arr_b(l: i64, arg_1: i32, begin: i32, end: &[i8]) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(begin, &end, local_4)?;
        let _t0: i32 = StringUTF16::getChars(l, &end, local_4)?;
        let mut pos: i32 = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(pos)
    }

    #[cfg_attr(any(), java_method(name = "contentEquals", descriptor = "([B[BI)Z", access = "public static"))]
    // java: contentEquals([B[BI)Z
    pub fn contentEquals__arr_b_arr_b_i(v1: &[i8], v2: &[i8], len: i32) -> Result<bool> {
        StringUTF16::checkBoundsOffCount(0i32, len, &v2)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            /* TODO: i2c  */
            let _t0: u16 = StringUTF16::getChar(&v2, i)?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "contentEquals", descriptor = "([BLjava/lang/CharSequence;I)Z", access = "public static"))]
    // java: contentEquals([BLjava/lang/CharSequence;I)Z
    pub fn contentEquals__arr_b_seq_i(value: &[i8], cs: Object, len: i32) -> Result<bool> {
        StringUTF16::checkOffset(len, &value)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            let _t0: u16 = StringUTF16::getChar(&value, i)?;
            let _t1 = cs.charAt(i)?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "putCharsAt", descriptor = "([BICCCC)I", access = "public static"))]
    // java: putCharsAt([BICCCC)I
    pub fn putCharsAt__arr_b_i_c_c_c_c(value: &[i8], i: i32, c1: u16, c2: u16, c3: u16, c4: u16) -> Result<i32> {
        let mut end: i32 = (i).wrapping_add(4i32);
        StringUTF16::checkBoundsBeginEnd(i, end, &value)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c1)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c2)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c3)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c4)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(end)
    }

    #[cfg_attr(any(), java_method(name = "putCharsAt", descriptor = "([BICCCCC)I", access = "public static"))]
    // java: putCharsAt([BICCCCC)I
    pub fn putCharsAt__arr_b_i_c_c_c_c_c(value: &[i8], i: i32, c1: u16, c2: u16, c3: u16, c4: u16, c5: u16) -> Result<i32> {
        let mut end: i32 = (i).wrapping_add(5i32);
        StringUTF16::checkBoundsBeginEnd(i, end, &value)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c1)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c2)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c3)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c4)?;
        i = i.wrapping_add(1i32);
        StringUTF16::putChar(&value, i, c5)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(end)
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "([BI)C", access = "public static"))]
    pub fn charAt(value: &[i8], index: i32) -> Result<u16> {
        StringUTF16::checkIndex(index, &value)?;
        let _t0: u16 = StringUTF16::getChar(&value, index)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "([BI)V", access = "public static"))]
    pub fn reverse(val: &[i8], count: i32) -> Result<()> {
        StringUTF16::checkOffset(count, &val)?;
        let mut n: i32 = (count).wrapping_sub(1i32);
        let mut hasSurrogates: i32 = 0i32;
        let mut j: i32 = ((n).wrapping_sub(1i32)>>((1i32&0x1f)));
        loop {
            if j<0i32 { break; }
            let mut k: i32 = (n).wrapping_sub(j);
            let _t0: u16 = StringUTF16::getChar(&val, j)?;
            let mut cj: i32 = _t0;
            let _t1: u16 = StringUTF16::getChar(&val, k)?;
            let mut ck: i32 = _t1;
            StringUTF16::putChar(&val, j, ck)?;
            StringUTF16::putChar(&val, k, cj)?;
            let _t2: bool = Character::isSurrogate(cj)?;
            let _t3: bool = Character::isSurrogate(ck)?;
            hasSurrogates = 1i32;
            j = j.wrapping_sub(1i32);
        }
        StringUTF16::reverseAllValidSurrogatePairs(&val, count)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "reverseAllValidSurrogatePairs", descriptor = "([BI)V", access = "private static"))]
    pub fn reverseAllValidSurrogatePairs(val: &[i8], count: i32) -> Result<()> {
        let mut i: i32 = 0i32;
        loop {
            if i >= (count).wrapping_sub(1i32) { break; }
            let _t0: u16 = StringUTF16::getChar(&val, i)?;
            let mut c2: i32 = _t0;
            let _t1: bool = Character::isLowSurrogate(c2)?;
            let _t2: u16 = StringUTF16::getChar(&val, (i).wrapping_add(1i32))?;
            let mut c1: i32 = _t2;
            let _t3: bool = Character::isHighSurrogate(c1)?;
            i = i.wrapping_add(1i32);
            StringUTF16::putChar(&val, i, c1)?;
            StringUTF16::putChar(&val, i, c2)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public static"))]
    pub fn inflate(src: &[i8], srcOff: i32, dst: &[i8], dstOff: i32, len: i32) -> Result<()> {
        StringUTF16::checkBoundsOffCount(dstOff, len, &dst)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            dstOff = dstOff.wrapping_add(1i32);
            srcOff = srcOff.wrapping_add(1i32);
            StringUTF16::putChar(&dst, dstOff, (src[srcOff as usize]&255i32))?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfLatin1", descriptor = "([BI[BII)I", access = "public static"))]
    pub fn lastIndexOfLatin1(src: &[i8], srcCount: i32, tgt: &[i8], tgtCount: i32, fromIndex: i32) -> Result<i32> {
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut min: i32 = (tgtCount).wrapping_sub(1i32);
        let mut i: i32 = (min).wrapping_add(fromIndex);
        let mut strLastIndex: i32 = (tgtCount).wrapping_sub(1i32);
        /* TODO: i2c  */
        let mut strLastChar: i32 = (tgt[strLastIndex as usize]&255i32);
        StringUTF16::checkIndex(i, &src)?;
        loop {
            let _t0: u16 = StringUTF16::getChar(&src, i)?;
            i = i.wrapping_sub(1i32);
            return Ok(-1i32);
            let mut j: i32 = (i).wrapping_sub(1i32);
            let mut start: i32 = (j).wrapping_sub(strLastIndex);
            let mut k: i32 = (strLastIndex).wrapping_sub(1i32);
            if j <= start { break; }
            j = j.wrapping_sub(1i32);
            let _t0: u16 = StringUTF16::getChar(&src, j)?;
            k = k.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
        }
        Ok((start).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_native(name = "isBigEndian", descriptor = "()Z", access = "private static native"))]
    pub fn isBigEndian() -> Result<bool> {
        todo!("native java/lang/StringUTF16.isBigEndian")
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(II[B)I", access = "static"))]
    // java: getChars(II[B)I
    pub fn getChars__i_i_arr_b(i: i32, index: i32, buf: &[i8]) -> Result<i32> {
        let mut charPos: i32 = index;
        let mut negative: i32 = i<0i32;
        i = (i).wrapping_neg();
        loop {
            if i > -100i32 { break; }
            let mut q: i32 = (i/100i32);
            let mut r: i32 = ((q).wrapping_mul(100i32)).wrapping_sub(i);
            i = q;
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(&buf, charPos, Integer::DigitOnes()[r as usize])?;
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(&buf, charPos, Integer::DigitTens()[r as usize])?;
        }
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, Integer::DigitOnes()[(i).wrapping_neg() as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, Integer::DigitTens()[(i).wrapping_neg() as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(&buf, charPos, 45i32)?;
        Ok(charPos)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(JI[B)I", access = "static"))]
    // java: getChars(JI[B)I
    pub fn getChars__l_i_arr_b(i: i64, arg_1: i32, index: &[i8]) -> Result<i32> {
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
            StringUTF16::putChar(local_3, charPos, Integer::DigitOnes()[r as usize])?;
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(local_3, charPos, Integer::DigitTens()[r as usize])?;
        }
        let mut i2: i32 = (i as i32);
        loop {
            if i2 > -100i32 { break; }
            let mut q2: i32 = (i2/100i32);
            r = ((q2).wrapping_mul(100i32)).wrapping_sub(i2);
            i2 = q2;
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(local_3, charPos, Integer::DigitOnes()[r as usize])?;
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(local_3, charPos, Integer::DigitTens()[r as usize])?;
        }
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(local_3, charPos, Integer::DigitOnes()[(i2).wrapping_neg() as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(local_3, charPos, Integer::DigitTens()[(i2).wrapping_neg() as usize])?;
        charPos = charPos.wrapping_sub(1i32);
        StringUTF16::putChar(local_3, charPos, 45i32)?;
        Ok(charPos)
    }

    #[cfg_attr(any(), java_method(name = "checkIndex", descriptor = "(I[B)V", access = "public static"))]
    pub fn checkIndex(off: i32, val: &[i8]) -> Result<()> {
        let _t0: i32 = StringUTF16::length(&val)?;
        String::checkIndex(off, _t0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkOffset", descriptor = "(I[B)V", access = "public static"))]
    pub fn checkOffset(off: i32, val: &[i8]) -> Result<()> {
        let _t0: i32 = StringUTF16::length(&val)?;
        String::checkOffset(off, _t0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkBoundsBeginEnd", descriptor = "(II[B)V", access = "public static"))]
    pub fn checkBoundsBeginEnd(begin: i32, end: i32, val: &[i8]) -> Result<()> {
        let _t0: i32 = StringUTF16::length(&val)?;
        String::checkBoundsBeginEnd(begin, end, _t0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkBoundsOffCount", descriptor = "(II[B)V", access = "public static"))]
    pub fn checkBoundsOffCount(offset: i32, count: i32, val: &[i8]) -> Result<()> {
        let _t0: i32 = StringUTF16::length(&val)?;
        let _t1: i32 = String::checkBoundsOffCount(offset, count, _t0)?;
        Ok(())
    }
}
