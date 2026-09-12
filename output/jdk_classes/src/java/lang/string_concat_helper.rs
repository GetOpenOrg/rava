#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringConcatHelper",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "StringConcatHelper.java",
))]
pub struct StringConcatHelper;

impl StringConcatHelper {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "coder", descriptor = "(C)J", access = "static"))]
    pub fn coder(value: u16) -> Result<i64> {
        let _t0: bool = StringLatin1::canEncode(value)?;
        Ok(4294967296i64)
    }

    #[cfg_attr(any(), java_method(name = "checkOverflow", descriptor = "(J)J", access = "private static"))]
    pub fn checkOverflow(lengthCoder: i64) -> Result<i64> {
        return Ok(lengthCoder);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JZ)J", access = "static"))]
    // java: mix(JZ)J
    pub fn mix__l_z(lengthCoder: i64, arg_1: bool) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::checkOverflow((lengthCoder).wrapping_add((local_2==0i32 as i64)))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JC)J", access = "static"))]
    // java: mix(JC)J
    pub fn mix__l_c(lengthCoder: i64, arg_1: u16) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::checkOverflow((lengthCoder).wrapping_add(1i64))?;
        let _t1: i64 = StringConcatHelper::coder(local_2)?;
        /* TODO: lor  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JI)J", access = "static"))]
    // java: mix(JI)J
    pub fn mix__l_i(lengthCoder: i64, arg_1: i32) -> Result<i64> {
        let _t0: i32 = Integer::stringSize(local_2)?;
        let _t1: i64 = StringConcatHelper::checkOverflow((lengthCoder).wrapping_add((_t0 as i64)))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JJ)J", access = "static"))]
    // java: mix(JJ)J
    pub fn mix__l_l(lengthCoder: i64, arg_1: i64) -> Result<i64> {
        let _t0: i32 = Long::stringSize(local_2)?;
        let _t1: i64 = StringConcatHelper::checkOverflow((lengthCoder).wrapping_add((_t0 as i64)))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JLjava/lang/String;)J", access = "static"))]
    // java: mix(JLjava/lang/String;)J
    pub fn mix__l_str(lengthCoder: i64, arg_1: String) -> Result<i64> {
        let _t0 = local_2.length()?;
        lengthCoder = (lengthCoder).wrapping_add((_t0 as i64));
        let _t1 = local_2.coder()?;
        /* TODO: lor  */
        lengthCoder = 4294967296i64;
        let _t2: i64 = StringConcatHelper::checkOverflow(lengthCoder)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "mix", descriptor = "(JLjdk/internal/util/FormatConcatItem;)J", access = "static"))]
    // java: mix(JLjdk/internal/util/FormatConcatItem;)J
    pub fn mix__l_format(lengthCoder: i64, arg_1: Object) -> Result<i64> {
        let _t0 = local_2.mix(lengthCoder)?;
        lengthCoder = _t0;
        let _t1: i64 = StringConcatHelper::checkOverflow(lengthCoder)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BZ)J", access = "private static"))]
    // java: prepend(J[BZ)J
    pub fn prepend__l_arr_b_z(indexCoder: i64, arg_1: &[i8], buf: bool) -> Result<i64> {
        let mut index: i32 = (indexCoder as i32);
        /* TODO: lcmp  */
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 101i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 117i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 114i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 116i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 101i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 115i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 108i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 97i32;
        index = index.wrapping_sub(1i32);
        buf[index as usize] = 102i32;
        return Ok((index as i64));
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 101i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 117i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 114i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 116i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 101i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 115i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 108i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 97i32)?;
        index = index.wrapping_sub(1i32);
        StringUTF16::putChar(buf, index, 102i32)?;
        /* TODO: lor  */
        Ok(4294967296i64)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BZLjava/lang/String;)J", access = "static"))]
    // java: prepend(J[BZLjava/lang/String;)J
    pub fn prepend__l_arr_b_z_str(indexCoder: i64, arg_1: &[i8], buf: bool, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_4)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BC)J", access = "private static"))]
    // java: prepend(J[BC)J
    pub fn prepend__l_arr_b_c(indexCoder: i64, arg_1: &[i8], buf: u16) -> Result<i64> {
        /* TODO: lcmp  */
        /* TODO: dup2  */
        indexCoder = (indexCoder).wrapping_sub(1i64);
        /* TODO: i2b  */
        4294967296i64[(buf as i32) as usize] = (local_3&255i32);
        /* TODO: dup2  */
        indexCoder = (indexCoder).wrapping_sub(1i64);
        StringUTF16::putChar(indexCoder, (buf as i32), local_3)?;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BCLjava/lang/String;)J", access = "static"))]
    // java: prepend(J[BCLjava/lang/String;)J
    pub fn prepend__l_arr_b_c_str(indexCoder: i64, arg_1: &[i8], buf: u16, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_4)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BI)J", access = "private static"))]
    // java: prepend(J[BI)J
    pub fn prepend__l_arr_b_i(indexCoder: i64, arg_1: &[i8], buf: i32) -> Result<i64> {
        /* TODO: lcmp  */
        let _t0: i32 = Integer::getChars(local_3, (indexCoder as i32), buf)?;
        return Ok((_t0 as i64));
        let _t1: i32 = StringUTF16::getChars(local_3, (indexCoder as i32), buf)?;
        /* TODO: lor  */
        Ok(4294967296i64)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BILjava/lang/String;)J", access = "static"))]
    // java: prepend(J[BILjava/lang/String;)J
    pub fn prepend__l_arr_b_i_str(indexCoder: i64, arg_1: &[i8], buf: i32, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_4)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BJ)J", access = "private static"))]
    // java: prepend(J[BJ)J
    pub fn prepend__l_arr_b_l(indexCoder: i64, arg_1: &[i8], buf: i64) -> Result<i64> {
        /* TODO: lcmp  */
        let _t0: i32 = Long::getChars(local_3, (indexCoder as i32), buf)?;
        return Ok((_t0 as i64));
        let _t1: i32 = StringUTF16::getChars(local_3, (indexCoder as i32), buf)?;
        /* TODO: lor  */
        Ok(4294967296i64)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BJLjava/lang/String;)J", access = "static"))]
    // java: prepend(J[BJLjava/lang/String;)J
    pub fn prepend__l_arr_b_l_str(indexCoder: i64, arg_1: &[i8], buf: i64, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_5)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BLjava/lang/String;)J", access = "private static"))]
    // java: prepend(J[BLjava/lang/String;)J
    pub fn prepend__l_arr_b_str(indexCoder: i64, arg_1: &[i8], buf: String) -> Result<i64> {
        let _t0 = local_3.length()?;
        indexCoder = (indexCoder).wrapping_sub((_t0 as i64));
        /* TODO: lcmp  */
        local_3.getBytes(buf, (indexCoder as i32), 0i32)?;
        local_3.getBytes(buf, (indexCoder as i32), 1i32)?;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BLjava/lang/String;Ljava/lang/String;)J", access = "static"))]
    // java: prepend(J[BLjava/lang/String;Ljava/lang/String;)J
    pub fn prepend__l_arr_b_str_str(indexCoder: i64, arg_1: &[i8], buf: String, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_4)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BLjdk/internal/util/FormatConcatItem;)J", access = "private static"))]
    // java: prepend(J[BLjdk/internal/util/FormatConcatItem;)J
    pub fn prepend__l_arr_b_format(indexCoder: i64, arg_1: &[i8], buf: Object) -> Result<i64> {
        let _t0 = local_3.prepend(indexCoder, buf)?;
        return Ok(_t0);
        let mut ex: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
        ex = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "prepend", descriptor = "(J[BLjdk/internal/util/FormatConcatItem;Ljava/lang/String;)J", access = "static"))]
    // java: prepend(J[BLjdk/internal/util/FormatConcatItem;Ljava/lang/String;)J
    pub fn prepend__l_arr_b_format_str(indexCoder: i64, arg_1: &[i8], buf: Object, value: String) -> Result<i64> {
        let _t0: i64 = StringConcatHelper::prepend(indexCoder, buf, value)?;
        indexCoder = _t0;
        let _t1: i64 = StringConcatHelper::prepend(indexCoder, buf, local_4)?;
        indexCoder = _t1;
        Ok(indexCoder)
    }

    #[cfg_attr(any(), java_method(name = "newString", descriptor = "([BJ)Ljava/lang/String;", access = "static"))]
    pub fn newString(buf: &[i8], indexCoder: i64) -> Result<String> {
        /* TODO: lcmp  */
        return Ok(String::new(buf, 0i32)?);
        /* TODO: lcmp  */
        return Ok(String::new(buf, 1i32)?);
        String::new().append(&String::from("Storage is not completely initialized,"))?;
        String::new().append(&(indexCoder as i32))?;
        String::new().append(&String::from("bytes left"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "simpleConcat", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;", access = "static"))]
    pub fn simpleConcat(first: Object, second: Object) -> Result<String> {
        let _t0: String = StringConcatHelper::stringOf(first)?;
        let mut s1: String = _t0;
        let _t1: String = StringConcatHelper::stringOf(second)?;
        let mut s2: String = _t1;
        let _t2 = s1.isEmpty()?;
        return Ok(String::new(s2)?);
        let _t3 = s2.isEmpty()?;
        return Ok(String::new(s1)?);
        let _t4: i64 = StringConcatHelper::initialCoder()?;
        let _t5: i64 = StringConcatHelper::mix(_t4, s1)?;
        let mut indexCoder: i64 = _t5;
        let _t6: i64 = StringConcatHelper::mix(indexCoder, s2)?;
        indexCoder = _t6;
        let _t7: Vec<i8> = StringConcatHelper::newArray(indexCoder)?;
        let mut buf: Vec<i8> = _t7;
        let _t8: i64 = StringConcatHelper::prepend(indexCoder, &buf, s2)?;
        indexCoder = _t8;
        let _t9: i64 = StringConcatHelper::prepend(indexCoder, &buf, s1)?;
        indexCoder = _t9;
        let _t10: String = StringConcatHelper::newString(&buf, indexCoder)?;
        Ok(_t10)
    }

    #[cfg_attr(any(), java_method(name = "newStringOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "static"))]
    pub fn newStringOf(arg: Object) -> Result<String> {
        let _t0: String = StringConcatHelper::stringOf(arg)?;
        Ok(String::new(_t0)?)
    }

    #[cfg_attr(any(), java_method(name = "stringOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "static"))]
    pub fn stringOf(value: Object) -> Result<String> {
        let _t0 = value.toString()?;
        let mut s: String = _t0;
        Ok(s)
    }

    #[cfg_attr(any(), java_method(name = "newArrayWithSuffix", descriptor = "(Ljava/lang/String;J)[B", access = "static"))]
    pub fn newArrayWithSuffix(suffix: String, indexCoder: i64) -> Result<Vec<i8>> {
        let _t0 = suffix.length()?;
        let _t1: Vec<i8> = StringConcatHelper::newArray((indexCoder).wrapping_add((_t0 as i64)))?;
        let mut buf: Vec<i8> = _t1;
        /* TODO: lcmp  */
        suffix.getBytes(buf, (indexCoder as i32), 0i32)?;
        suffix.getBytes(buf, (indexCoder as i32), 1i32)?;
        Ok(buf)
    }

    #[cfg_attr(any(), java_method(name = "newArray", descriptor = "(J)[B", access = "static"))]
    pub fn newArray(indexCoder: i64) -> Result<Vec<i8>> {
        /* TODO: lshr  */
        /* TODO: i2b  */
        let mut coder: i32 = (32i32 as i32);
        let mut index: i32 = ((indexCoder as i32)<<(coder&0x1f));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = StringConcatHelper::UNSAFE().allocateUninitializedArray(Byte::TYPE(), index)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "initialCoder", descriptor = "()J", access = "static"))]
    pub fn initialCoder() -> Result<i64> {
        Ok(4294967296i64)
    }

    #[cfg_attr(any(), java_method(name = "getCharLatin1", descriptor = "([BI)C", access = "static"))]
    pub fn getCharLatin1(buffer: &[i8], index: i32) -> Result<u16> {
        /* TODO: i2c  */
        Ok(buffer[index as usize])
    }

    #[cfg_attr(any(), java_method(name = "getCharUTF16", descriptor = "([BI)C", access = "static"))]
    pub fn getCharUTF16(buffer: &[i8], index: i32) -> Result<u16> {
        let _t0: u16 = StringUTF16::getChar(&buffer, index)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "putCharLatin1", descriptor = "([BII)V", access = "static"))]
    pub fn putCharLatin1(buffer: &[i8], index: i32, ch: i32) -> Result<()> {
        /* TODO: i2b  */
        buffer[index as usize] = ch;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putCharUTF16", descriptor = "([BII)V", access = "static"))]
    pub fn putCharUTF16(buffer: &[i8], index: i32, ch: i32) -> Result<()> {
        StringUTF16::putChar(&buffer, index, ch)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "selectGetChar", descriptor = "(J)Ljava/lang/invoke/MethodHandle;", access = "static"))]
    pub fn selectGetChar(indexCoder: i64) -> Result<Object> {
        /* TODO: lcmp  */
        Ok(StringConcatHelper$LateInit::GETCHAR_UTF16_MH())
    }

    #[cfg_attr(any(), java_method(name = "selectPutChar", descriptor = "(J)Ljava/lang/invoke/MethodHandle;", access = "static"))]
    pub fn selectPutChar(indexCoder: i64) -> Result<Object> {
        /* TODO: lcmp  */
        Ok(StringConcatHelper$LateInit::PUTCHAR_UTF16_MH())
    }

    #[cfg_attr(any(), java_method(name = "lookupStatic", descriptor = "(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;", access = "static"))]
    pub fn lookupStatic(name: String, methodType: Object) -> Result<Object> {
        let _t0: Object = MethodHandles::lookup()?;
        let _t1 = _t0.findStatic(15i32, name, methodType)?;
        return Ok(_t1);
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }
}
