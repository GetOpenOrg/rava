#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringLatin1",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "StringLatin1.java",
))]
pub struct StringLatin1;

impl StringLatin1 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "([BI)C", access = "public static"))]
    pub fn charAt(value: &[i8], index: i32) -> Result<u16> {
        String::checkIndex(index, (value.len() as i32))?;
        /* TODO: i2c  */
        Ok((value[index as usize]&255i32))
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(C)Z", access = "public static"))]
    // java: canEncode(C)Z
    pub fn canEncode__c(cp: u16) -> Result<bool> {
        Ok(cp <= 255i32)
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(I)Z", access = "public static"))]
    // java: canEncode(I)Z
    pub fn canEncode__i(cp: i32) -> Result<bool> {
        Ok(cp <= 255i32)
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "([B)I", access = "public static"))]
    pub fn length(value: &[i8]) -> Result<i32> {
        Ok((value.len() as i32))
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([BII)I", access = "public static"))]
    pub fn codePointAt(value: &[i8], index: i32, end: i32) -> Result<i32> {
        Ok((value[index as usize]&255i32))
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public static"))]
    pub fn codePointBefore(value: &[i8], index: i32) -> Result<i32> {
        Ok((value[(index).wrapping_sub(1i32) as usize]&255i32))
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "([BII)I", access = "public static"))]
    pub fn codePointCount(value: &[i8], beginIndex: i32, endIndex: i32) -> Result<i32> {
        Ok((endIndex).wrapping_sub(beginIndex))
    }

    #[cfg_attr(any(), java_method(name = "toChars", descriptor = "([B)[C", access = "public static"))]
    pub fn toChars(value: &[i8]) -> Result<Vec<u16>> {
        let mut _arr0: Vec<u16> = vec![0u16; (value.len() as i32) as usize];
        let mut dst: Vec<u16> = _arr0;
        StringLatin1::inflate(&value, 0i32, &dst, 0i32, (value.len() as i32))?;
        Ok(dst)
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BII)[B", access = "public static"))]
    // java: inflate([BII)[B
    pub fn inflate__arr_b_i_i(value: &[i8], off: i32, len: i32) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = StringUTF16::newBytesFor(len)?;
        let mut ret: Vec<i8> = _t0;
        StringLatin1::inflate(&value, off, &ret, 0i32, len)?;
        Ok(ret)
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public static"))]
    pub fn getChars(value: &[i8], srcBegin: i32, srcEnd: i32, dst: &[u16], dstBegin: i32) -> Result<()> {
        StringLatin1::inflate(&value, srcBegin, &dst, dstBegin, (srcEnd).wrapping_sub(srcBegin))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public static"))]
    pub fn getBytes(value: &[i8], srcBegin: i32, srcEnd: i32, dst: &[i8], dstBegin: i32) -> Result<()> {
        System::arraycopy(&value, srcBegin, &dst, dstBegin, (srcEnd).wrapping_sub(srcBegin))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "([B[B)Z", access = "public static"))]
    pub fn equals(value: &[i8], other: &[i8]) -> Result<bool> {
        let mut i: i32 = 0i32;
        loop {
            if i >= (value.len() as i32) { break; }
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[B)I", access = "public static"))]
    // java: compareTo([B[B)I
    pub fn compareTo__arr_b_arr_b(value: &[i8], other: &[i8]) -> Result<i32> {
        let mut len1: i32 = (value.len() as i32);
        let mut len2: i32 = (other.len() as i32);
        let _t0: i32 = StringLatin1::compareTo(&value, &other, len1, len2)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public static"))]
    // java: compareTo([B[BII)I
    pub fn compareTo__arr_b_arr_b_i_i(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        let _t0: i32 = (len1).min(len2);
        let mut lim: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&value, &other, lim)?;
        let mut k: i32 = _t1;
        let _t2: u16 = StringLatin1::getChar(&value, k)?;
        let _t3: u16 = StringLatin1::getChar(&other, k)?;
        Ok((_t2).wrapping_sub(_t3))
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16", descriptor = "([B[B)I", access = "public static"))]
    // java: compareToUTF16([B[B)I
    pub fn compareToUTF16__arr_b_arr_b(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringLatin1::length(&value)?;
        let mut len1: i32 = _t0;
        let _t1: i32 = StringUTF16::length(&other)?;
        let mut len2: i32 = _t1;
        let _t2: i32 = StringLatin1::compareToUTF16Values(&value, &other, len1, len2)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16", descriptor = "([B[BII)I", access = "public static"))]
    // java: compareToUTF16([B[BII)I
    pub fn compareToUTF16__arr_b_arr_b_i_i(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        let _t0: i32 = StringLatin1::length(&value)?;
        String::checkOffset(len1, _t0)?;
        let _t1: i32 = StringUTF16::length(&other)?;
        String::checkOffset(len2, _t1)?;
        let _t2: i32 = StringLatin1::compareToUTF16Values(&value, &other, len1, len2)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16Values", descriptor = "([B[BII)I", access = "private static"))]
    pub fn compareToUTF16Values(value: &[i8], other: &[i8], len1: i32, len2: i32) -> Result<i32> {
        let _t0: i32 = (len1).min(len2);
        let mut lim: i32 = _t0;
        let mut k: i32 = 0i32;
        loop {
            if k >= lim { break; }
            let _t0: u16 = StringLatin1::getChar(&value, k)?;
            let mut c1: i32 = _t0;
            let _t1: u16 = StringUTF16::getChar(&other, k)?;
            let mut c2: i32 = _t1;
            return Ok((c1).wrapping_sub(c2));
            k = k.wrapping_add(1i32);
        }
        Ok((len1).wrapping_sub(len2))
    }

    #[cfg_attr(any(), java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public static"))]
    pub fn compareToCI(value: &[i8], other: &[i8]) -> Result<i32> {
        let mut len1: i32 = (value.len() as i32);
        let mut len2: i32 = (other.len() as i32);
        let _t0: i32 = (len1).min(len2);
        let mut lim: i32 = _t0;
        let mut k: i32 = 0i32;
        loop {
            if k >= lim { break; }
            let _t0: u16 = StringLatin1::getChar(&value, k)?;
            let _t1 = CharacterDataLatin1::instance().toUpperCase(_t0)?;
            /* TODO: i2c  */
            let mut c1: i32 = _t1;
            let _t2: u16 = StringLatin1::getChar(&other, k)?;
            let _t3 = CharacterDataLatin1::instance().toUpperCase(_t2)?;
            /* TODO: i2c  */
            let mut c2: i32 = _t3;
            let _t4: u16 = Character::toLowerCase(c1)?;
            c1 = _t4;
            let _t5: u16 = Character::toLowerCase(c2)?;
            c2 = _t5;
            return Ok((c1).wrapping_sub(c2));
            k = k.wrapping_add(1i32);
        }
        Ok((len1).wrapping_sub(len2))
    }

    #[cfg_attr(any(), java_method(name = "compareToCI_UTF16", descriptor = "([B[B)I", access = "public static"))]
    pub fn compareToCI_UTF16(value: &[i8], other: &[i8]) -> Result<i32> {
        let _t0: i32 = StringLatin1::length(&value)?;
        let mut len1: i32 = _t0;
        let _t1: i32 = StringUTF16::length(&other)?;
        let mut len2: i32 = _t1;
        let _t2: i32 = (len1).min(len2);
        let mut lim: i32 = _t2;
        let mut k: i32 = 0i32;
        loop {
            if k >= lim { break; }
            let _t0: u16 = StringLatin1::getChar(&value, k)?;
            let mut c1: i32 = _t0;
            let _t1: u16 = StringUTF16::getChar(&other, k)?;
            let mut c2: i32 = _t1;
            let _t2 = CharacterDataLatin1::instance().toUpperCase(c1)?;
            /* TODO: i2c  */
            c1 = _t2;
            let _t3: u16 = Character::toUpperCase(c2)?;
            c2 = _t3;
            let _t4: u16 = Character::toLowerCase(c1)?;
            c1 = _t4;
            let _t5: u16 = Character::toLowerCase(c2)?;
            c2 = _t5;
            return Ok((c1).wrapping_sub(c2));
            k = k.wrapping_add(1i32);
        }
        Ok((len1).wrapping_sub(len2))
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "([B)I", access = "public static"))]
    pub fn hashCode(value: &[i8]) -> Result<i32> {
        /* TODO: lookupswitch default:42 0:28 1:32 */
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&value, 0i32, (value.len() as i32), 0i32, 4i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BIII)I", access = "public static"))]
    // java: indexOf([BIII)I
    pub fn indexOf__arr_b_i_i_i(value: &[i8], ch: i32, fromIndex: i32, toIndex: i32) -> Result<i32> {
        let _t0: bool = StringLatin1::canEncode(ch)?;
        return Ok(-1i32);
        let _t1: i32 = (fromIndex).max(0i32);
        fromIndex = _t1;
        let _t2: i32 = (toIndex).min((value.len() as i32));
        toIndex = _t2;
        return Ok(-1i32);
        let _t3: i32 = StringLatin1::indexOfChar(&value, ch, fromIndex, toIndex)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private static"))]
    pub fn indexOfChar(value: &[i8], ch: i32, fromIndex: i32, max: i32) -> Result<i32> {
        /* TODO: i2b  */
        let mut c: i32 = ch;
        let mut i: i32 = fromIndex;
        loop {
            if i >= max { break; }
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([B[B)I", access = "public static"))]
    // java: indexOf([B[B)I
    pub fn indexOf__arr_b_arr_b(value: &[i8], str: &[i8]) -> Result<i32> {
        return Ok(0i32);
        return Ok(-1i32);
        let _t0: i32 = StringLatin1::indexOf(&value, (value.len() as i32), &str, (str.len() as i32), 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public static"))]
    // java: indexOf([BI[BII)I
    pub fn indexOf__arr_b_i_arr_b_i_i(value: &[i8], valueCount: i32, str: &[i8], strCount: i32, fromIndex: i32) -> Result<i32> {
        let mut first: i32 = str[0i32 as usize];
        let mut max: i32 = (valueCount).wrapping_sub(strCount);
        let mut i: i32 = fromIndex;
        loop {
            if i > max { break; }
            i = i.wrapping_add(1i32);
            let mut j: i32 = (i).wrapping_add(1i32);
            let mut end: i32 = ((j).wrapping_add(strCount)).wrapping_sub(1i32);
            let mut k: i32 = 1i32;
            j = j.wrapping_add(1i32);
            k = k.wrapping_add(1i32);
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public static"))]
    // java: lastIndexOf([BI[BII)I
    pub fn lastIndexOf__arr_b_i_arr_b_i_i(src: &[i8], srcCount: i32, tgt: &[i8], tgtCount: i32, fromIndex: i32) -> Result<i32> {
        let mut min: i32 = (tgtCount).wrapping_sub(1i32);
        let mut i: i32 = (min).wrapping_add(fromIndex);
        let mut strLastIndex: i32 = (tgtCount).wrapping_sub(1i32);
        /* TODO: i2c  */
        let mut strLastChar: i32 = (tgt[strLastIndex as usize]&255i32);
        loop {
            i = i.wrapping_sub(1i32);
            return Ok(-1i32);
            let mut j: i32 = (i).wrapping_sub(1i32);
            let mut start: i32 = (j).wrapping_sub(strLastIndex);
            let mut k: i32 = (strLastIndex).wrapping_sub(1i32);
            if j <= start { break; }
            j = j.wrapping_sub(1i32);
            k = k.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
        }
        Ok((start).wrapping_add(1i32))
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public static"))]
    // java: lastIndexOf([BII)I
    pub fn lastIndexOf__arr_b_i_i(value: &[i8], ch: i32, fromIndex: i32) -> Result<i32> {
        let _t0: bool = StringLatin1::canEncode(ch)?;
        return Ok(-1i32);
        let _t1: i32 = (fromIndex).min(((value.len() as i32)).wrapping_sub(1i32));
        let mut off: i32 = _t1;
        loop {
            if off<0i32 { break; }
            /* TODO: i2b  */
            return Ok(off);
            off = off.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public static"))]
    // java: replace([BCC)Ljava/lang/String;
    pub fn replace__arr_b_c_c(value: &[i8], oldChar: u16, newChar: u16) -> Result<String> {
        let _t0: bool = StringLatin1::canEncode(oldChar)?;
        let mut len: i32 = (value.len() as i32);
        let mut i: i32 = -1i32;
        i = i.wrapping_add(1i32);
        /* TODO: i2b  */
        let _t1: bool = StringLatin1::canEncode(newChar)?;
        let _t2: Vec<i8> = StringConcatHelper::newArray((len as i64))?;
        let mut buf: Vec<i8> = _t2;
        let mut j: i32 = 0i32;
        loop {
            if j >= i { break; }
            buf[j as usize] = value[j as usize];
            j = j.wrapping_add(1i32);
        }
        loop {
            if i >= len { break; }
            j = value[i as usize];
            /* TODO: i2b  */
            /* TODO: i2b  */
            oldChar[newChar as usize] = j;
            i = i.wrapping_add(1i32);
        }
        return Ok(String::new(buf, 0i32)?);
        let _t3: Vec<i8> = StringUTF16::newBytesFor(len)?;
        buf = _t3;
        StringLatin1::inflate(&value, 0i32, &buf, 0i32, i)?;
        loop {
            if i >= len { break; }
            /* TODO: i2c  */
            j = (value[i as usize]&255i32);
            StringUTF16::putChar(oldChar, newChar, j)?;
            i = i.wrapping_add(1i32);
        }
        return Ok(String::new(buf, 1i32)?);
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BI[BI[BI)Ljava/lang/String;", access = "public static"))]
    // java: replace([BI[BI[BI)Ljava/lang/String;
    pub fn replace__arr_b_i_arr_b_i_arr_b_i(value: &[i8], valLen: i32, targ: &[i8], targLen: i32, repl: &[i8], replLen: i32) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut p: i32 = 0i32;
        let _t0: i32 = StringLatin1::indexOf(&value, valLen, &targ, targLen, 0i32)?;
        let mut i: i32 = _t0;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let mut _arr1: Vec<i32> = vec![0i32; 16i32 as usize];
        let mut pos: Vec<i32> = _arr1;
        pos[0i32 as usize] = i;
        i = (i).wrapping_add(targLen);
        loop {
            let _t0: i32 = StringLatin1::indexOf(&value, valLen, &targ, targLen, i)?;
            let mut j: i32 = _t0;
            if _t0<=0i32 { break; }
            p = p.wrapping_add(1i32);
            let _t0: i32 = ArraysSupport::newLength(p, 1i32, (p>>((1i32&0x1f))))?;
            let _t1: Vec<i32> = Arrays::copyOf(&pos, _t0)?;
            pos = _t1;
            pos[p as usize] = j;
            i = (j).wrapping_add(targLen);
        }
        p = p.wrapping_add(1i32);
        let _t2: i32 = (p).abs();
        let _t3: i32 = (valLen).abs();
        let mut resultLen: i32 = _t3;
        let mut ignored: i32 = valLen;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(String::from(""));
        let _t4: Vec<i8> = StringConcatHelper::newArray((resultLen as i64))?;
        ignored = _t4;
        let mut posFrom: i32 = 0i32;
        let mut posTo: i32 = 0i32;
        let mut q: i32 = 0i32;
        loop {
            if q >= p { break; }
            let mut nextPos: i32 = pos[q as usize];
            posTo = posTo.wrapping_add(1i32);
            posFrom = posFrom.wrapping_add(1i32);
            ignored[posTo as usize] = value[posFrom as usize];
            posFrom = (posFrom).wrapping_add(targLen);
            let mut k: i32 = 0i32;
            posTo = posTo.wrapping_add(1i32);
            ignored[posTo as usize] = repl[k as usize];
            k = k.wrapping_add(1i32);
            q = q.wrapping_add(1i32);
        }
        loop {
            if posFrom >= valLen { break; }
            posTo = posTo.wrapping_add(1i32);
            posFrom = posFrom.wrapping_add(1i32);
            ignored[posTo as usize] = value[posFrom as usize];
        }
        Ok(String::new(ignored, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public static"))]
    pub fn regionMatchesCI(value: &[i8], toffset: i32, other: &[i8], ooffset: i32, len: i32) -> Result<bool> {
        let mut last: i32 = (toffset).wrapping_add(len);
        loop {
            if toffset >= last { break; }
            toffset = toffset.wrapping_add(1i32);
            let mut b1: i32 = value[toffset as usize];
            ooffset = ooffset.wrapping_add(1i32);
            let mut b2: i32 = other[ooffset as usize];
            let _t0: bool = CharacterDataLatin1::equalsIgnoreCase(b1, b2)?;
        }
        return Ok(0i32);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI_UTF16", descriptor = "([BI[BII)Z", access = "public static"))]
    pub fn regionMatchesCI_UTF16(value: &[i8], toffset: i32, other: &[i8], ooffset: i32, len: i32) -> Result<bool> {
        let mut last: i32 = (toffset).wrapping_add(len);
        loop {
            if toffset >= last { break; }
            toffset = toffset.wrapping_add(1i32);
            /* TODO: i2c  */
            let mut c1: i32 = (value[toffset as usize]&255i32);
            ooffset = ooffset.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&other, ooffset)?;
            let mut c2: i32 = _t0;
            let _t1 = CharacterDataLatin1::instance().toUpperCase(c1)?;
            /* TODO: i2c  */
            let mut u1: i32 = _t1;
            let _t2: u16 = Character::toUpperCase(c2)?;
            let mut u2: i32 = _t2;
            let _t3: u16 = Character::toLowerCase(u1)?;
            let _t4: u16 = Character::toLowerCase(u2)?;
        }
        return Ok(0i32);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public static"))]
    pub fn toLowerCase(str: String, value: &[i8], locale: Object) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut len: i32 = (value.len() as i32);
        let mut first: i32 = 0i32;
        loop {
            if first >= len { break; }
            let mut cp: i32 = (value[first as usize]&255i32);
            let _t0 = CharacterDataLatin1::instance().toLowerCase(cp)?;
            first = first.wrapping_add(1i32);
        }
        return Ok(str);
        let _t0 = locale.getLanguage()?;
        cp = _t0;
        let _t1: String = StringLatin1::toLowerCaseEx(str, &value, first, locale, 1i32)?;
        return Ok(_t1);
        let mut _arr2: Vec<i8> = vec![0i8; len as usize];
        let mut result: Vec<i8> = _arr2;
        System::arraycopy(&value, 0i32, &result, 0i32, first)?;
        let mut i: i32 = first;
        loop {
            if i >= len { break; }
            let mut cp: i32 = (value[i as usize]&255i32);
            let _t0 = CharacterDataLatin1::instance().toLowerCase(cp)?;
            cp = _t0;
            let _t1: bool = StringLatin1::canEncode(cp)?;
            let _t2: String = StringLatin1::toLowerCaseEx(str, &value, first, locale, 0i32)?;
            return Ok(_t2);
            /* TODO: i2b  */
            result[i as usize] = cp;
            i = i.wrapping_add(1i32);
        }
        Ok(String::new(result, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private static"))]
    pub fn toLowerCaseEx(str: String, value: &[i8], first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        let _t0: Vec<i8> = StringUTF16::newBytesFor((value.len() as i32))?;
        let mut result: Vec<i8> = _t0;
        let mut resultOffset: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= first { break; }
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, (value[i as usize]&255i32))?;
            i = i.wrapping_add(1i32);
        }
        i = first;
        loop {
            if i >= (value.len() as i32) { break; }
            let mut srcChar: i32 = (value[i as usize]&255i32);
            let _t0: i32 = ConditionalSpecialCasing::toLowerCaseEx(str, i, locale)?;
            let mut lowerChar: i32 = _t0;
            let _t1 = CharacterDataLatin1::instance().toLowerCase(srcChar)?;
            lowerChar = _t1;
            let _t2: bool = Character::isBmpCodePoint(lowerChar)?;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, lowerChar)?;
            let _t3: Vec<u16> = ConditionalSpecialCasing::toLowerCaseCharArray(str, i, locale)?;
            let mut lowerCharArray: Vec<u16> = _t3;
            let _t4: Vec<u16> = Character::toChars(lowerChar)?;
            lowerCharArray = _t4;
            let mut mapLen: i32 = (lowerCharArray.len() as i32);
            let _t5: Vec<i8> = StringUTF16::newBytesFor(((((result.len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(1i32))?;
            let mut result2: Vec<i8> = _t5;
            System::arraycopy(&result, 0i32, &result2, 0i32, (resultOffset<<(1i32&0x1f)))?;
            result = result2;
            result2 = 0i32;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, lowerCharArray[result2 as usize])?;
            result2 = result2.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        let _t1: String = StringUTF16::newString(&result, 0i32, resultOffset)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public static"))]
    pub fn toUpperCase(str: String, value: &[i8], locale: Object) -> Result<String> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut len: i32 = (value.len() as i32);
        let mut first: i32 = 0i32;
        loop {
            if first >= len { break; }
            let mut cp: i32 = (value[first as usize]&255i32);
            let _t0 = CharacterDataLatin1::instance().toUpperCaseEx(cp)?;
            first = first.wrapping_add(1i32);
        }
        return Ok(str);
        let _t0 = locale.getLanguage()?;
        cp = _t0;
        let _t1: String = StringLatin1::toUpperCaseEx(str, &value, first, locale, 1i32)?;
        return Ok(_t1);
        let mut _arr2: Vec<i8> = vec![0i8; len as usize];
        let mut result: Vec<i8> = _arr2;
        System::arraycopy(&value, 0i32, &result, 0i32, first)?;
        let mut i: i32 = first;
        loop {
            if i >= len { break; }
            let mut cp: i32 = (value[i as usize]&255i32);
            let _t0 = CharacterDataLatin1::instance().toUpperCaseEx(cp)?;
            cp = _t0;
            let _t1: bool = StringLatin1::canEncode(cp)?;
            let _t2: String = StringLatin1::toUpperCaseEx(str, &value, first, locale, 0i32)?;
            return Ok(_t2);
            /* TODO: i2b  */
            result[i as usize] = cp;
            i = i.wrapping_add(1i32);
        }
        Ok(String::new(result, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private static"))]
    pub fn toUpperCaseEx(str: String, value: &[i8], first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        let _t0: Vec<i8> = StringUTF16::newBytesFor((value.len() as i32))?;
        let mut result: Vec<i8> = _t0;
        let mut resultOffset: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= first { break; }
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, (value[i as usize]&255i32))?;
            i = i.wrapping_add(1i32);
        }
        i = first;
        loop {
            if i >= (value.len() as i32) { break; }
            let mut srcChar: i32 = (value[i as usize]&255i32);
            let _t0: i32 = ConditionalSpecialCasing::toUpperCaseEx(str, i, locale)?;
            let mut upperChar: i32 = _t0;
            let _t1 = CharacterDataLatin1::instance().toUpperCaseEx(srcChar)?;
            upperChar = _t1;
            let _t2: bool = Character::isBmpCodePoint(upperChar)?;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, upperChar)?;
            let _t3: Vec<u16> = ConditionalSpecialCasing::toUpperCaseCharArray(str, i, locale)?;
            let mut upperCharArray: Vec<u16> = _t3;
            let _t4 = CharacterDataLatin1::instance().toUpperCaseCharArray(srcChar)?;
            upperCharArray = _t4;
            let _t5: Vec<u16> = Character::toChars(upperChar)?;
            upperCharArray = _t5;
            let mut mapLen: i32 = (upperCharArray.len() as i32);
            let _t6: Vec<i8> = StringUTF16::newBytesFor(((((result.len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(1i32))?;
            let mut result2: Vec<i8> = _t6;
            System::arraycopy(&result, 0i32, &result2, 0i32, (resultOffset<<(1i32&0x1f)))?;
            result = result2;
            result2 = 0i32;
            resultOffset = resultOffset.wrapping_add(1i32);
            StringUTF16::putChar(&result, resultOffset, upperCharArray[result2 as usize])?;
            result2 = result2.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        let _t1: String = StringUTF16::newString(&result, 0i32, resultOffset)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn trim(value: &[i8]) -> Result<String> {
        let mut len: i32 = (value.len() as i32);
        let mut st: i32 = 0i32;
        loop {
            if st >= len { break; }
            st = st.wrapping_add(1i32);
        }
        loop {
            if st >= len { break; }
            len = len.wrapping_sub(1i32);
        }
        let _t0: String = StringLatin1::newString(&value, st, (len).wrapping_sub(st))?;
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public static"))]
    pub fn indexOfNonWhitespace(value: &[i8]) -> Result<i32> {
        let mut length: i32 = (value.len() as i32);
        let mut left: i32 = 0i32;
        loop {
            if left >= length { break; }
            let _t0: u16 = StringLatin1::getChar(&value, left)?;
            let mut ch: i32 = _t0;
            let _t1 = CharacterDataLatin1::instance().isWhitespace(ch)?;
            left = left.wrapping_add(1i32);
        }
        Ok(left)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public static"))]
    pub fn lastIndexOfNonWhitespace(value: &[i8]) -> Result<i32> {
        let mut length: i32 = (value.len() as i32);
        let mut right: i32 = length;
        loop {
            if 0i32 >= right { break; }
            let _t0: u16 = StringLatin1::getChar(&value, (right).wrapping_sub(1i32))?;
            let mut ch: i32 = _t0;
            let _t1 = CharacterDataLatin1::instance().isWhitespace(ch)?;
            right = right.wrapping_sub(1i32);
        }
        Ok(right)
    }

    #[cfg_attr(any(), java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn strip(value: &[i8]) -> Result<String> {
        let _t0: i32 = StringLatin1::indexOfNonWhitespace(&value)?;
        let mut left: i32 = _t0;
        return Ok(String::from(""));
        let _t1: i32 = StringLatin1::lastIndexOfNonWhitespace(&value)?;
        let mut right: i32 = _t1;
        let mut ifChanged: i32 = right < (value.len() as i32);
        let _t2: String = StringLatin1::newString(&value, left, (right).wrapping_sub(left))?;
        /* TODO: aconst_null  */
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn stripLeading(value: &[i8]) -> Result<String> {
        let _t0: i32 = StringLatin1::indexOfNonWhitespace(&value)?;
        let mut left: i32 = _t0;
        let _t1: String = StringLatin1::newString(&value, left, ((value.len() as i32)).wrapping_sub(left))?;
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public static"))]
    pub fn stripTrailing(value: &[i8]) -> Result<String> {
        let _t0: i32 = StringLatin1::lastIndexOfNonWhitespace(&value)?;
        let mut right: i32 = _t0;
        let _t1: String = StringLatin1::newString(&value, 0i32, right)?;
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "static"))]
    pub fn lines(value: &[i8]) -> Result<Object> {
        let _t0: Object = StringLatin1$LinesSpliterator::spliterator(&value)?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "putChar", descriptor = "([BII)V", access = "public static"))]
    pub fn putChar(val: &[i8], index: i32, c: i32) -> Result<()> {
        /* TODO: i2b  */
        val[index as usize] = c;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getChar", descriptor = "([BI)C", access = "public static"))]
    pub fn getChar(val: &[i8], index: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok((val[index as usize]&255i32))
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "([III)[B", access = "public static"))]
    // java: toBytes([III)[B
    pub fn toBytes__arr_i_i_i(val: &[i32], off: i32, len: i32) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; len as usize];
        let mut ret: Vec<i8> = _arr0;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            off = off.wrapping_add(1i32);
            let mut cp: i32 = val[off as usize];
            let _t0: bool = StringLatin1::canEncode(cp)?;
            /* TODO: aconst_null  */
            return Ok(_t0);
            /* TODO: i2b  */
            ret[i as usize] = cp;
            i = i.wrapping_add(1i32);
        }
        Ok(ret)
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "(C)[B", access = "public static"))]
    // java: toBytes(C)[B
    pub fn toBytes__c(c: u16) -> Result<Vec<i8>> {
        let mut _arr0: Vec<i8> = vec![0i8; 1i32 as usize];
        /* TODO: i2b  */
        _arr0[0i32 as usize] = c;
        Ok(_arr0)
    }

    #[cfg_attr(any(), java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public static"))]
    pub fn newString(val: &[i8], index: i32, len: i32) -> Result<String> {
        return Ok(String::from(""));
        let _t0: Vec<i8> = Arrays::copyOfRange(&val, index, (index).wrapping_add(len))?;
        Ok(String::new(_t0, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "fillNull", descriptor = "([BII)V", access = "public static"))]
    pub fn fillNull(val: &[i8], index: i32, end: i32) -> Result<()> {
        Arrays::fill(&val, index, end, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BI[CII)V", access = "public static"))]
    // java: inflate([BI[CII)V
    pub fn inflate__arr_b_i_arr_c_i_i(src: &[i8], srcOff: i32, dst: &[u16], dstOff: i32, len: i32) -> Result<()> {
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            dstOff = dstOff.wrapping_add(1i32);
            srcOff = srcOff.wrapping_add(1i32);
            /* TODO: i2c  */
            dst[dstOff as usize] = (src[srcOff as usize]&255i32);
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public static"))]
    // java: inflate([BI[BII)V
    pub fn inflate__arr_b_i_arr_b_i_i(src: &[i8], srcOff: i32, dst: &[i8], dstOff: i32, len: i32) -> Result<()> {
        StringUTF16::inflate(&src, srcOff, &dst, dstOff, len)?;
        Ok(())
    }
}
