#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringCoding",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "StringCoding.java",
))]
pub struct StringCoding;

impl StringCoding {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasNegatives", descriptor = "([BII)Z", access = "public static"))]
    pub fn hasNegatives(ba: &[i8], off: i32, len: i32) -> Result<bool> {
        let _t0: i32 = StringCoding::countPositives(&ba, off, len)?;
        Ok(_t0 != len)
    }

    #[cfg_attr(any(), java_method(name = "countPositives", descriptor = "([BII)I", access = "public static"))]
    pub fn countPositives(ba: &[i8], off: i32, len: i32) -> Result<i32> {
        let mut limit: i32 = (off).wrapping_add(len);
        let mut i: i32 = off;
        loop {
            if i >= limit { break; }
            return Ok((i).wrapping_sub(off));
            i = i.wrapping_add(1i32);
        }
        Ok(len)
    }

    #[cfg_attr(any(), java_method(name = "implEncodeISOArray", descriptor = "([BI[BII)I", access = "public static"))]
    pub fn implEncodeISOArray(sa: &[i8], sp: i32, da: &[i8], dp: i32, len: i32) -> Result<i32> {
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            sp = sp.wrapping_add(1i32);
            let _t0: u16 = StringUTF16::getChar(&sa, sp)?;
            let mut c: i32 = _t0;
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            da[dp as usize] = c;
            i = i.wrapping_add(1i32);
        }
        Ok(i)
    }

    #[cfg_attr(any(), java_method(name = "implEncodeAsciiArray", descriptor = "([CI[BII)I", access = "public static"))]
    pub fn implEncodeAsciiArray(sa: &[u16], sp: i32, da: &[i8], dp: i32, len: i32) -> Result<i32> {
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            sp = sp.wrapping_add(1i32);
            let mut c: i32 = sa[sp as usize];
            dp = dp.wrapping_add(1i32);
            /* TODO: i2b  */
            da[dp as usize] = c;
            i = i.wrapping_add(1i32);
        }
        Ok(i)
    }
}
