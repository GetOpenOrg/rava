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
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/StringUTF16"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringUTF16.java"]
    #[inner_classes     = "java/lang/StringUTF16$LinesSpliterator:java/lang/StringUTF16:LinesSpliterator:26;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/StringUTF16"]

    pub struct StringUTF16;

    impl StringUTF16 {
        #[cfg_attr(any(), java_field(name = "HI_BYTE_SHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: HI_BYTE_SHIFT:I
        pub fn HI_BYTE_SHIFT() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "LO_BYTE_SHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: LO_BYTE_SHIFT:I
        pub fn LO_BYTE_SHIFT() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "MAX_LENGTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1073741823"))]
        // static field: MAX_LENGTH:I
        pub fn MAX_LENGTH() -> i32 {
            1073741823
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/StringUTF16.<init>:()V")
        }

        #[java_method(name = "newBytesFor", descriptor = "(I)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newBytesFor(mut len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0: i32 = StringUTF16::newBytesLength(len)?;
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; _t0 as usize]));
            Ok(_arr1)
        }

        #[java_method(name = "newBytesLength", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newBytesLength(mut len: i32) -> Result<i32> {
            if (len<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if len >= 1073741823i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("UTF16 String size is ")))?;
                let _t1 = _t0.append_i(len)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(", should be less than ")))?;
                let _t3 = _t2.append_i(1073741823i32)?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok((len<<(1i32&0x1f)))
        }

        #[java_method(name = "putChar", descriptor = "([BII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut c: i32) -> Result<()> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            if index >= _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            index = (index<<(1i32&0x1f));
            index = index.wrapping_add(1i32);
            val.borrow_mut()[index as usize] = ((((c>>((StringUTF16::HI_BYTE_SHIFT()&0x1f)))) as i8 as i32)) as i8;
            val.borrow_mut()[index as usize] = ((((c>>((StringUTF16::LO_BYTE_SHIFT()&0x1f)))) as i8 as i32)) as i8;
            Ok(())
        }

        #[java_method(name = "getChar", descriptor = "([BI)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<u16> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            if index >= _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            index = (index<<(1i32&0x1f));
            index = index.wrapping_add(1i32);
            Ok(((((((((val.borrow()[index as usize] as i32)&255i32)<<(StringUTF16::HI_BYTE_SHIFT()&0x1f))|(((val.borrow()[index as usize] as i32)&255i32)<<(StringUTF16::LO_BYTE_SHIFT()&0x1f)))) as u16 as i32)) as u16))
        }

        #[java_method(name = "length", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            Ok(((value.borrow().len() as i32)>>((1i32&0x1f))))
        }

        #[java_method(name = "codePointAt", descriptor = "([BIIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointAt([BIIZ)I
        pub fn codePointAt_arr_b_i_i_z(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32, mut end: i32, mut checked: bool) -> Result<i32> {
            if index >= end {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if checked {
                StringUTF16::checkIndex(index, Clone::clone(&value))?;
            }
            let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            let mut c1: u16 = _t0;
            let _t1: bool = Character::isHighSurrogate(c1)?;
            index = index.wrapping_add(1i32);
            if checked {
                StringUTF16::checkIndex(index, Clone::clone(&value))?;
            }
            let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            let mut c2: u16 = _t2;
            let _t3: bool = Character::isLowSurrogate(c2)?;
            if _t3 {
                let _t4: i32 = Character::toCodePoint(c1, c2)?;
                return Ok(_t4);
            }
            Ok((c1) as i32)
        }

        #[java_method(name = "codePointAt", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointAt([BII)I
        pub fn codePointAt_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32, mut end: i32) -> Result<i32> {
            let _t0: i32 = StringUTF16::codePointAt_arr_b_i_i_z(Clone::clone(&value), index, end, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "codePointBefore", descriptor = "([BIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointBefore([BIZ)I
        pub fn codePointBefore_arr_b_i_z(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32, mut checked: bool) -> Result<i32> {
            index = index.wrapping_sub(1i32);
            if checked {
                StringUTF16::checkIndex(index, Clone::clone(&value))?;
            }
            let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            let mut c2: u16 = _t0;
            let _t1: bool = Character::isLowSurrogate(c2)?;
            index = index.wrapping_sub(1i32);
            if checked {
                StringUTF16::checkIndex(index, Clone::clone(&value))?;
            }
            let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            let mut c1: u16 = _t2;
            let _t3: bool = Character::isHighSurrogate(c1)?;
            if _t3 {
                let _t4: i32 = Character::toCodePoint(c1, c2)?;
                return Ok(_t4);
            }
            Ok((c2) as i32)
        }

        #[java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointBefore([BI)I
        pub fn codePointBefore_arr_b_i(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<i32> {
            let _t0: i32 = StringUTF16::codePointBefore_arr_b_i_z(Clone::clone(&value), index, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "codePointCount", descriptor = "([BIIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointCount([BIIZ)I
        pub fn codePointCount_arr_b_i_i_z(mut value: Rc<RefCell<Vec<i8>>>, mut beginIndex: i32, mut endIndex: i32, mut checked: bool) -> Result<i32> {
            if beginIndex > endIndex {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut count = (endIndex).wrapping_sub(beginIndex);
            let mut i: i32 = beginIndex;
            if i < endIndex {
                StringUTF16::checkBoundsBeginEnd(i, endIndex, Clone::clone(&value))?;
            }
            loop {
                if i >= (endIndex).wrapping_sub(1i32) { break; }
                i = i.wrapping_add(1i32);
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let _t1: bool = Character::isHighSurrogate(_t0)?;
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let _t3: bool = Character::isLowSurrogate(_t2)?;
                count = count.wrapping_sub(1i32);
                i = i.wrapping_add(1i32);
            }
            Ok(count)
        }

        #[java_method(name = "codePointCount", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: codePointCount([BII)I
        pub fn codePointCount_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut beginIndex: i32, mut endIndex: i32) -> Result<i32> {
            let _t0: i32 = StringUTF16::codePointCount_arr_b_i_i_z(Clone::clone(&value), beginIndex, endIndex, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "toChars", descriptor = "([B)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars(mut value: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<u16>>>> {
            let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; ((value.borrow().len() as i32)>>((1i32&0x1f))) as usize]));
            let mut dst: Rc<RefCell<Vec<u16>>> = _arr0;
            StringUTF16::getChars_arr_b_i_i_arr_c_i(Clone::clone(&value), 0i32, (dst.borrow().len() as i32), Clone::clone(&dst), 0i32)?;
            Ok(dst)
        }

        #[java_method(name = "toBytes", descriptor = "([CII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBytes([CII)[B
        pub fn toBytes_arr_c_i_i(mut value: Rc<RefCell<Vec<u16>>>, mut off: i32, mut len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(len)?;
            let mut val: Rc<RefCell<Vec<i8>>> = _t0;
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                StringUTF16::putChar(Clone::clone(&val), i, (value.borrow()[off as usize] as i32))?;
                off = off.wrapping_add(1i32);
                i = i.wrapping_add(1i32);
            }
            Ok(val)
        }

        #[java_method(name = "coderFromArrayLen", descriptor = "([BI)B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn coderFromArrayLen(mut value: Rc<RefCell<Vec<i8>>>, mut len: i32) -> Result<i8> {
            Ok((((((((len).wrapping_sub((value.borrow().len() as i32)) as u32>>(31i32&0x1f)) as i32)) as i8 as i32)) as i8))
        }

        #[java_method(name = "compress", descriptor = "([CII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compress([CII)[B
        pub fn compress_arr_c_i_i(mut val: Rc<RefCell<Vec<u16>>>, mut off: i32, mut count: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; count as usize]));
            let mut latin1: Rc<RefCell<Vec<i8>>> = _arr0;
            let _t1: i32 = StringUTF16::compress_arr_c_i_arr_b_i_i(Clone::clone(&val), off, Clone::clone(&latin1), 0i32, count)?;
            let mut ndx: i32 = _t1;
            let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::toBytes_arr_c_i_i(Clone::clone(&val), off, count)?;
            let mut utf16: Rc<RefCell<Vec<i8>>> = _t2;
            let _t3: u16 = StringUTF16::getChar(Clone::clone(&utf16), ndx)?;
            let _t4: i32 = StringUTF16::compress_arr_b_i_arr_b_i_i(Clone::clone(&utf16), 0i32, Clone::clone(&latin1), 0i32, count)?;
            if _t4 != count {
                return Ok(utf16);
            }
            Ok(latin1)
        }

        #[java_method(name = "compress", descriptor = "([BII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compress([BII)[B
        pub fn compress_arr_b_i_i(mut val: Rc<RefCell<Vec<i8>>>, mut off: i32, mut count: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; count as usize]));
            let mut latin1: Rc<RefCell<Vec<i8>>> = _arr0;
            let _t1: i32 = StringUTF16::compress_arr_b_i_arr_b_i_i(Clone::clone(&val), off, Clone::clone(&latin1), 0i32, count)?;
            let mut ndx: i32 = _t1;
            let _t2: i32 = StringUTF16::newBytesLength((off).wrapping_add(count))?;
            let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), (off<<(1i32&0x1f)), _t2)?;
            let mut utf16: Rc<RefCell<Vec<i8>>> = _t3;
            let _t4: u16 = StringUTF16::getChar(Clone::clone(&utf16), ndx)?;
            let _t5: i32 = StringUTF16::compress_arr_b_i_arr_b_i_i(Clone::clone(&utf16), 0i32, Clone::clone(&latin1), 0i32, count)?;
            if _t5 != count {
                return Ok(utf16);
            }
            Ok(latin1)
        }

        #[java_method(name = "compress", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compress([III)[B
        pub fn compress_arr_i_i_i(mut val: Rc<RefCell<Vec<i32>>>, mut off: i32, mut count: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; count as usize]));
            let mut latin1: Rc<RefCell<Vec<i8>>> = _arr0;
            let mut end = (off).wrapping_add(count);
            let mut ndx: i32 = 0i32;
            loop {
                if ndx >= count { break; }
                let mut cp = val.borrow()[off as usize];
                if (cp>=0) {
                    if cp <= 255i32 {
                        latin1.borrow_mut()[ndx as usize] = (((cp) as i8 as i32)) as i8;
                    } else {
                        let _t1: i32 = StringUTF16::computeCodePointSize(Clone::clone(&val), off, end)?;
                        let mut estSize = (ndx).wrapping_add(_t1);
                        let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(estSize)?;
                        let mut utf16: Rc<RefCell<Vec<i8>>> = _t2;
                        if (ndx>0) {
                            StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&latin1), 0i32, Clone::clone(&utf16), 0i32, ndx)?;
                        }
                        if estSize == count {
                            StringUTF16::putChar(Clone::clone(&utf16), ndx, cp)?;
                            off = off.wrapping_add(1i32);
                            let mut i = (ndx).wrapping_add(1i32);
                            loop {
                                if i >= count { break; }
                                StringUTF16::putChar(Clone::clone(&utf16), i, val.borrow()[off as usize])?;
                                i = i.wrapping_add(1i32);
                                off = off.wrapping_add(1i32);
                            }
                        } else {
                            let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::extractCodepoints(Clone::clone(&val), off, end, Clone::clone(&utf16), ndx)?;
                            utf16 = _t3;
                            let _t4: u16 = StringUTF16::getChar(Clone::clone(&utf16), ndx)?;
                            let _t5: i32 = StringUTF16::compress_arr_b_i_arr_b_i_i(Clone::clone(&utf16), 0i32, Clone::clone(&latin1), 0i32, count)?;
                            if _t5 == count {
                                return Ok(latin1);
                            }
                        }
                        return Ok(utf16);
                    }
                } else {
                    let _t1: i32 = StringUTF16::computeCodePointSize(Clone::clone(&val), off, end)?;
                    let mut estSize = (ndx).wrapping_add(_t1);
                    let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(estSize)?;
                    let mut utf16: Rc<RefCell<Vec<i8>>> = _t2;
                    if (ndx>0) {
                        StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&latin1), 0i32, Clone::clone(&utf16), 0i32, ndx)?;
                    }
                    if estSize == count {
                        StringUTF16::putChar(Clone::clone(&utf16), ndx, cp)?;
                        off = off.wrapping_add(1i32);
                        let mut i = (ndx).wrapping_add(1i32);
                        loop {
                            if i >= count { break; }
                            StringUTF16::putChar(Clone::clone(&utf16), i, val.borrow()[off as usize])?;
                            i = i.wrapping_add(1i32);
                            off = off.wrapping_add(1i32);
                        }
                    } else {
                        let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::extractCodepoints(Clone::clone(&val), off, end, Clone::clone(&utf16), ndx)?;
                        utf16 = _t3;
                        let _t4: u16 = StringUTF16::getChar(Clone::clone(&utf16), ndx)?;
                        let _t5: i32 = StringUTF16::compress_arr_b_i_arr_b_i_i(Clone::clone(&utf16), 0i32, Clone::clone(&latin1), 0i32, count)?;
                        if _t5 == count {
                            return Ok(latin1);
                        }
                    }
                    return Ok(utf16);
                }
                ndx = ndx.wrapping_add(1i32);
                off = off.wrapping_add(1i32);
            }
            Ok(latin1)
        }

        #[java_method(name = "extractCodepoints", descriptor = "([III[BI)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn extractCodepoints(mut val: Rc<RefCell<Vec<i32>>>, mut off: i32, mut end: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            loop {
                if off >= end { break; }
                let mut codePoint = val.borrow()[off as usize];
                let _t0: i32 = Character::charCount(codePoint)?;
                let mut dstLimit = ((dstOff).wrapping_add(_t0)).wrapping_add(((end).wrapping_sub(off)).wrapping_sub(1i32));
                if dstLimit > ((dst.borrow().len() as i32)>>((1i32&0x1f))) {
                    let mut maxRemaining = (dstLimit).wrapping_add(((end).wrapping_sub(off)).wrapping_sub(1i32));
                    let _t1: i32 = StringUTF16::newBytesLength(maxRemaining)?;
                    let _t2: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), _t1)?;
                    dst = _t2;
                }
                loop {
                    let _t1: bool = Character::isBmpCodePoint(codePoint)?;
                    if _t1 {
                        dstOff = dstOff.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&dst), dstOff, codePoint)?;
                    } else {
                        dstOff = dstOff.wrapping_add(1i32);
                        let _t2: u16 = Character::highSurrogate(codePoint)?;
                        StringUTF16::putChar(Clone::clone(&dst), dstOff, (_t2 as i32))?;
                        dstOff = dstOff.wrapping_add(1i32);
                        let _t3: u16 = Character::lowSurrogate(codePoint)?;
                        StringUTF16::putChar(Clone::clone(&dst), dstOff, (_t3 as i32))?;
                    }
                    off = off.wrapping_add(1i32);
                    if (dstOff).wrapping_add(2i32) > dstLimit {
                    } else {
                        codePoint = val.borrow()[off as usize];
                        continue;
                    }
                }
            }
            if dstOff != ((dst.borrow().len() as i32)>>((1i32&0x1f))) {
                let _t0: i32 = StringUTF16::newBytesLength(dstOff)?;
                let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), _t0)?;
                return Ok(_t1);
            }
            Ok(dst)
        }

        #[java_method(name = "computeCodePointSize", descriptor = "([III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeCodePointSize(mut val: Rc<RefCell<Vec<i32>>>, mut off: i32, mut end: i32) -> Result<i32> {
            let mut n = (end).wrapping_sub(off);
            loop {
                if off >= end { break; }
                off = off.wrapping_add(1i32);
                let mut codePoint = val.borrow()[off as usize];
                let _t0: bool = Character::isBmpCodePoint(codePoint)?;
                if _t0 {
                    continue;
                }
                let _t1: bool = Character::isValidCodePoint(codePoint)?;
                if _t1 {
                    n = n.wrapping_add(1i32);
                } else {
                    let _t2: String = Integer::toString_i(codePoint)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            }
            Ok(n)
        }

        #[java_method(name = "compress", descriptor = "([CI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compress([CI[BII)I
        pub fn compress_arr_c_i_arr_b_i_i(mut src: Rc<RefCell<Vec<u16>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32, mut len: i32) -> Result<i32> {
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                let mut c = (src.borrow()[srcOff as usize] as i32);
                if c > 255i32 {
                    return Ok(i);
                }
                dst.borrow_mut()[dstOff as usize] = (((c) as i8 as i32)) as i8;
                srcOff = srcOff.wrapping_add(1i32);
                dstOff = dstOff.wrapping_add(1i32);
                i = i.wrapping_add(1i32);
            }
            Ok(len)
        }

        #[java_method(name = "compress", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compress([BI[BII)I
        pub fn compress_arr_b_i_arr_b_i_i(mut src: Rc<RefCell<Vec<i8>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32, mut len: i32) -> Result<i32> {
            StringUTF16::checkBoundsOffCount(srcOff, len, Clone::clone(&src))?;
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&src), srcOff)?;
                let mut c: u16 = _t0;
                if (c as i32) > 255i32 {
                    return Ok(i);
                }
                dst.borrow_mut()[dstOff as usize] = (((c) as i8 as i32)) as i8;
                srcOff = srcOff.wrapping_add(1i32);
                dstOff = dstOff.wrapping_add(1i32);
                i = i.wrapping_add(1i32);
            }
            Ok(len)
        }

        #[java_method(name = "toBytes", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBytes([III)[B
        pub fn toBytes_arr_i_i_i(mut val: Rc<RefCell<Vec<i32>>>, mut index: i32, mut len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut end = (index).wrapping_add(len);
            let _t0: i32 = StringUTF16::computeCodePointSize(Clone::clone(&val), index, end)?;
            let mut n: i32 = _t0;
            let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(n)?;
            let mut buf: Rc<RefCell<Vec<i8>>> = _t1;
            let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::extractCodepoints(Clone::clone(&val), index, end, Clone::clone(&buf), 0i32)?;
            Ok(_t2)
        }

        #[java_method(name = "toBytes", descriptor = "(C)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBytes(C)[B
        pub fn toBytes_c(mut c: u16) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; 2i32 as usize]));
            let mut result: Rc<RefCell<Vec<i8>>> = _arr0;
            StringUTF16::putChar(Clone::clone(&result), 0i32, (c as i32))?;
            Ok(result)
        }

        #[java_method(name = "toBytesSupplementary", descriptor = "(I)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBytesSupplementary(cp: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/StringUTF16.toBytesSupplementary:(I)[B")
        }

        #[java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getChars([BII[CI)V
        pub fn getChars_arr_b_i_i_arr_c_i(mut value: Rc<RefCell<Vec<i8>>>, mut srcBegin: i32, mut srcEnd: i32, mut dst: Rc<RefCell<Vec<u16>>>, mut dstBegin: i32) -> Result<()> {
            if srcBegin < srcEnd {
                StringUTF16::checkBoundsOffCount(srcBegin, (srcEnd).wrapping_sub(srcBegin), Clone::clone(&value))?;
            }
            let mut i: i32 = srcBegin;
            loop {
                if i >= srcEnd { break; }
                dstBegin = dstBegin.wrapping_add(1i32);
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                dst.borrow_mut()[dstBegin as usize] = (_t0) as u16;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytes(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
            panic!("stub: java/lang/StringUTF16.getBytes:([BII[BI)V")
        }

        #[java_method(name = "equals", descriptor = "([B[B)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
            panic!("stub: java/lang/StringUTF16.equals:([B[B)Z")
        }

        #[java_method(name = "compareTo", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareTo([B[B)I
        pub fn compareTo_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&value))?;
            let mut len1: i32 = _t0;
            let _t1: i32 = StringUTF16::length(Clone::clone(&other))?;
            let mut len2: i32 = _t1;
            let _t2: i32 = StringUTF16::compareValues(Clone::clone(&value), Clone::clone(&other), len1, len2)?;
            Ok(_t2)
        }

        #[java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareTo([B[BII)I
        pub fn compareTo_arr_b_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            StringUTF16::checkOffset(len1, Clone::clone(&value))?;
            StringUTF16::checkOffset(len2, Clone::clone(&other))?;
            let _t0: i32 = StringUTF16::compareValues(Clone::clone(&value), Clone::clone(&other), len1, len2)?;
            Ok(_t0)
        }

        #[java_method(name = "compareValues", descriptor = "([B[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareValues(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            let _t0: i32 = Math::min_i_i(len1, len2)?;
            let mut lim: i32 = _t0;
            let mut k: i32 = 0i32;
            loop {
                if k >= lim { break; }
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&value), k)?;
                let mut c1: u16 = _t1;
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&other), k)?;
                let mut c2: u16 = _t2;
                if (c1 as i32) != (c2 as i32) {
                    return Ok(((c1 as i32)).wrapping_sub((c2 as i32)));
                }
                k = k.wrapping_add(1i32);
            }
            Ok((len1).wrapping_sub(len2))
        }

        #[java_method(name = "compareToLatin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareToLatin1([B[B)I
        pub fn compareToLatin1_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let _t0: i32 = StringLatin1::compareToUTF16_arr_b_arr_b(Clone::clone(&other), Clone::clone(&value))?;
            Ok((_t0).wrapping_neg())
        }

        #[java_method(name = "compareToLatin1", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareToLatin1([B[BII)I
        pub fn compareToLatin1_arr_b_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            let _t0: i32 = StringLatin1::compareToUTF16_arr_b_arr_b_i_i(Clone::clone(&other), Clone::clone(&value), len2, len1)?;
            Ok((_t0).wrapping_neg())
        }

        #[java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToCI(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.compareToCI:([B[B)I")
        }

        #[java_method(name = "compareToCIImpl", descriptor = "([BII[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToCIImpl(mut value: Rc<RefCell<Vec<i8>>>, mut toffset: i32, mut tlen: i32, mut other: Rc<RefCell<Vec<i8>>>, mut ooffset: i32, mut olen: i32) -> Result<i32> {
            let mut tlast = (toffset).wrapping_add(tlen);
            let mut olast = (ooffset).wrapping_add(olen);
            if (ooffset<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i32 = StringUTF16::length(Clone::clone(&value))?;
            if tlast > _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: i32 = StringUTF16::length(Clone::clone(&other))?;
            if olast > _t1 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut k1: i32 = toffset;
            let mut k2: i32 = ooffset;
            loop {
                if k1 >= tlast { break; }
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), k1)?;
                let mut cp1: u16 = _t2;
                let _t3: u16 = StringUTF16::getChar(Clone::clone(&other), k2)?;
                let mut cp2: u16 = _t3;
                let _t4: i32 = StringUTF16::compareCodePointCI((cp1 as i32), (cp2 as i32))?;
                if (_t4==0) {
                } else {
                    let _t5: i32 = StringUTF16::codePointIncluding(Clone::clone(&value), (cp1 as i32), k1, toffset, tlast)?;
                    let mut cp1: i32 = _t5;
                    if (cp1<0) {
                        k1 = k1.wrapping_add(1i32);
                        cp1 = (cp1).wrapping_neg();
                    }
                    let _t6: i32 = StringUTF16::codePointIncluding(Clone::clone(&other), (cp2 as i32), k2, ooffset, olast)?;
                    let mut cp2: i32 = _t6;
                    if (cp2<0) {
                        k2 = k2.wrapping_add(1i32);
                        cp2 = (cp2).wrapping_neg();
                    }
                    let _t7: i32 = StringUTF16::compareCodePointCI(cp1, cp2)?;
                    let mut diff: i32 = _t7;
                    if (diff!=0) {
                        return Ok(diff);
                    }
                }
                k1 = k1.wrapping_add(1i32);
                k2 = k2.wrapping_add(1i32);
            }
            Ok((tlen).wrapping_sub(olen))
        }

        #[java_method(name = "compareCodePointCI", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareCodePointCI(mut cp1: i32, mut cp2: i32) -> Result<i32> {
            let _t0: i32 = Character::toUpperCase_i(cp1)?;
            cp1 = _t0;
            let _t1: i32 = Character::toUpperCase_i(cp2)?;
            cp2 = _t1;
            let _t2: i32 = Character::toLowerCase_i(cp1)?;
            cp1 = _t2;
            let _t3: i32 = Character::toLowerCase_i(cp2)?;
            cp2 = _t3;
            if cp1 != cp2 {
                return Ok((cp1).wrapping_sub(cp2));
            }
            Ok(0i32)
        }

        #[java_method(name = "codePointIncluding", descriptor = "([BIIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointIncluding(mut ba: Rc<RefCell<Vec<i8>>>, mut cp: i32, mut index: i32, mut start: i32, mut end: i32) -> Result<i32> {
            let _t0: bool = Character::isSurrogate(((((cp) as u16 as i32)) as u16))?;
            if !(_t0) {
                return Ok(cp);
            }
            let _t1: bool = Character::isLowSurrogate(((((cp) as u16 as i32)) as u16))?;
            if _t1 {
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&ba), (index).wrapping_sub(1i32))?;
                let mut c: u16 = _t2;
                let _t3: bool = Character::isHighSurrogate(c)?;
                if _t3 {
                    let _t4: i32 = Character::toCodePoint(c, ((((cp) as u16 as i32)) as u16))?;
                    return Ok(_t4);
                }
            } else {
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&ba), (index).wrapping_add(1i32))?;
                let mut c: u16 = _t2;
                let _t3: bool = Character::isLowSurrogate(c)?;
                if _t3 {
                    let _t4: i32 = Character::toCodePoint(((((cp) as u16 as i32)) as u16), c)?;
                    return Ok((_t4).wrapping_neg());
                }
            }
            Ok(cp)
        }

        #[java_method(name = "compareToCI_Latin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToCI_Latin1(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.compareToCI_Latin1:([B[B)I")
        }

        #[java_method(name = "hashCode", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.hashCode:([B)I")
        }

        #[java_method(name = "indexOf", descriptor = "([BIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([BIII)I
        pub fn indexOf_arr_b_i_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut toIndex: i32) -> Result<i32> {
            let _t0: i32 = Math::max_i_i(fromIndex, 0i32)?;
            fromIndex = _t0;
            let _t1: i32 = Math::min_i_i(toIndex, ((value.borrow().len() as i32)>>((1i32&0x1f))))?;
            toIndex = _t1;
            if fromIndex >= toIndex {
                return Ok(-1i32);
            }
            if ch < 65536i32 {
                let _t2: i32 = StringUTF16::indexOfChar(Clone::clone(&value), ch, fromIndex, toIndex)?;
                return Ok(_t2);
            }
            let _t2: i32 = StringUTF16::indexOfSupplementary(Clone::clone(&value), ch, fromIndex, toIndex)?;
            Ok(_t2)
        }

        #[java_method(name = "indexOf", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([B[B)I
        pub fn indexOf_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            if ((str.borrow().len() as i32)==0) {
                return Ok(0i32);
            }
            if (value.borrow().len() as i32) < (str.borrow().len() as i32) {
                return Ok(-1i32);
            }
            let _t0: i32 = StringUTF16::length(Clone::clone(&value))?;
            let _t1: i32 = StringUTF16::length(Clone::clone(&str))?;
            let _t2: i32 = StringUTF16::indexOfUnsafe(Clone::clone(&value), _t0, Clone::clone(&str), _t1, 0i32)?;
            Ok(_t2)
        }

        #[java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([BI[BII)I
        pub fn indexOf_arr_b_i_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut valueCount: i32, mut str: Rc<RefCell<Vec<i8>>>, mut strCount: i32, mut fromIndex: i32) -> Result<i32> {
            StringUTF16::checkBoundsBeginEnd(fromIndex, valueCount, Clone::clone(&value))?;
            StringUTF16::checkBoundsBeginEnd(0i32, strCount, Clone::clone(&str))?;
            let _t0: i32 = StringUTF16::indexOfUnsafe(Clone::clone(&value), valueCount, Clone::clone(&str), strCount, fromIndex)?;
            Ok(_t0)
        }

        #[java_method(name = "indexOfUnsafe", descriptor = "([BI[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfUnsafe(mut value: Rc<RefCell<Vec<i8>>>, mut valueCount: i32, mut str: Rc<RefCell<Vec<i8>>>, mut strCount: i32, mut fromIndex: i32) -> Result<i32> {
            if (fromIndex<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (strCount<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i32 = StringUTF16::length(Clone::clone(&str))?;
            if strCount > _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if valueCount < strCount {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: u16 = StringUTF16::getChar(Clone::clone(&str), 0i32)?;
            let mut first: u16 = _t1;
            let mut max = (valueCount).wrapping_sub(strCount);
            let mut i: i32 = fromIndex;
            loop {
                if i > max { break; }
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                loop {
                    i = i.wrapping_add(1i32);
                    if i > max { break; }
                    let _t3: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                    if (_t3 as i32) != (first as i32) {
                        continue;
                    }
                    break;
                }
                let mut j = (i).wrapping_add(1i32);
                let mut end = ((j).wrapping_add(strCount)).wrapping_sub(1i32);
                let mut k: i32 = 1i32;
                loop {
                    if j >= end { break; }
                    let _t3: u16 = StringUTF16::getChar(Clone::clone(&value), j)?;
                    let _t4: u16 = StringUTF16::getChar(Clone::clone(&str), k)?;
                    if (_t3 as i32) == (_t4 as i32) {
                        j = j.wrapping_add(1i32);
                        k = k.wrapping_add(1i32);
                        continue;
                    }
                    break;
                }
                if j == end {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "indexOfLatin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOfLatin1([B[B)I
        pub fn indexOfLatin1_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            if ((str.borrow().len() as i32)==0) {
                return Ok(0i32);
            }
            let _t0: i32 = StringUTF16::length(Clone::clone(&value))?;
            if _t0 < (str.borrow().len() as i32) {
                return Ok(-1i32);
            }
            let _t1: i32 = StringUTF16::length(Clone::clone(&value))?;
            let _t2: i32 = StringUTF16::indexOfLatin1Unsafe(Clone::clone(&value), _t1, Clone::clone(&str), (str.borrow().len() as i32), 0i32)?;
            Ok(_t2)
        }

        #[java_method(name = "indexOfLatin1", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOfLatin1([BI[BII)I
        pub fn indexOfLatin1_arr_b_i_arr_b_i_i(mut src: Rc<RefCell<Vec<i8>>>, mut srcCount: i32, mut tgt: Rc<RefCell<Vec<i8>>>, mut tgtCount: i32, mut fromIndex: i32) -> Result<i32> {
            StringUTF16::checkBoundsBeginEnd(fromIndex, srcCount, Clone::clone(&src))?;
            String::checkBoundsBeginEnd(0i32, tgtCount, (tgt.borrow().len() as i32))?;
            let _t0: i32 = StringUTF16::indexOfLatin1Unsafe(Clone::clone(&src), srcCount, Clone::clone(&tgt), tgtCount, fromIndex)?;
            Ok(_t0)
        }

        #[java_method(name = "indexOfLatin1Unsafe", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfLatin1Unsafe(mut src: Rc<RefCell<Vec<i8>>>, mut srcCount: i32, mut tgt: Rc<RefCell<Vec<i8>>>, mut tgtCount: i32, mut fromIndex: i32) -> Result<i32> {
            if (fromIndex<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (tgtCount<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if tgtCount > (tgt.borrow().len() as i32) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if srcCount < tgtCount {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut first = ((((tgt.borrow()[0i32 as usize] as i32)&255i32)) as u16 as i32);
            let mut max = (srcCount).wrapping_sub(tgtCount);
            let mut i: i32 = fromIndex;
            loop {
                if i > max { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&src), i)?;
                loop {
                    i = i.wrapping_add(1i32);
                    if i > max { break; }
                    let _t1: u16 = StringUTF16::getChar(Clone::clone(&src), i)?;
                    if (_t1 as i32) != first {
                        continue;
                    }
                    break;
                }
                let mut j = (i).wrapping_add(1i32);
                let mut end = ((j).wrapping_add(tgtCount)).wrapping_sub(1i32);
                let mut k: i32 = 1i32;
                loop {
                    if j >= end { break; }
                    let _t1: u16 = StringUTF16::getChar(Clone::clone(&src), j)?;
                    if (_t1 as i32) == ((tgt.borrow()[k as usize] as i32)&255i32) {
                        j = j.wrapping_add(1i32);
                        k = k.wrapping_add(1i32);
                        continue;
                    }
                    break;
                }
                if j == end {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfChar(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut max: i32) -> Result<i32> {
            StringUTF16::checkBoundsBeginEnd(fromIndex, max, Clone::clone(&value))?;
            let _t0: i32 = StringUTF16::indexOfCharUnsafe(Clone::clone(&value), ch, fromIndex, max)?;
            Ok(_t0)
        }

        #[java_method(name = "indexOfCharUnsafe", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfCharUnsafe(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut max: i32) -> Result<i32> {
            let mut i: i32 = fromIndex;
            loop {
                if i >= max { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                if (_t0 as i32) == ch {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "indexOfSupplementary", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfSupplementary(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut max: i32) -> Result<i32> {
            let _t0: bool = Character::isValidCodePoint(ch)?;
            let _t1: u16 = Character::highSurrogate(ch)?;
            let mut hi: u16 = _t1;
            let _t2: u16 = Character::lowSurrogate(ch)?;
            let mut lo: u16 = _t2;
            StringUTF16::checkBoundsBeginEnd(fromIndex, max, Clone::clone(&value))?;
            let mut i: i32 = fromIndex;
            loop {
                if i >= (max).wrapping_sub(1i32) { break; }
                let _t3: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let _t4: u16 = StringUTF16::getChar(Clone::clone(&value), (i).wrapping_add(1i32))?;
                if (_t4 as i32) == (lo as i32) {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_arr_b_i_arr_b_i_i(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.lastIndexOf:([BI[BII)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: lastIndexOf([BII)I
        pub fn lastIndexOf_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
            let _t0: i32 = Math::min_i_i(fromIndex, (((value.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_sub(1i32))?;
            let mut i: i32 = _t0;
            loop {
                if (i<0) { break; }
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                if (_t1 as i32) == ch {
                    return Ok(i);
                }
                i = i.wrapping_sub(1i32);
            }
            return Ok(-1i32);
            let _t1: i32 = StringUTF16::lastIndexOfSupplementary(Clone::clone(&value), ch, fromIndex)?;
            Ok(_t1)
        }

        #[java_method(name = "lastIndexOfSupplementary", descriptor = "([BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOfSupplementary(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
            let _t0: bool = Character::isValidCodePoint(ch)?;
            let _t1: u16 = Character::highSurrogate(ch)?;
            let mut hi: u16 = _t1;
            let _t2: u16 = Character::lowSurrogate(ch)?;
            let mut lo: u16 = _t2;
            let _t3: i32 = Math::min_i_i(fromIndex, (((value.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_sub(2i32))?;
            let mut i: i32 = _t3;
            loop {
                if (i<0) { break; }
                let _t4: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let _t5: u16 = StringUTF16::getChar(Clone::clone(&value), (i).wrapping_add(1i32))?;
                if (_t5 as i32) == (lo as i32) {
                    return Ok(i);
                }
                i = i.wrapping_sub(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace([BCC)Ljava/lang/String;
        pub fn replace_arr_b_c_c(mut value: Rc<RefCell<Vec<i8>>>, mut oldChar: u16, mut newChar: u16) -> Result<String> {
            let mut len = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut i: i32 = -1i32;
            loop {
                i = i.wrapping_add(1i32);
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                if (_t0 as i32) == (oldChar as i32) { break; }
            }
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (value.borrow().len() as i32) as usize]));
            let mut buf: Rc<RefCell<Vec<i8>>> = _arr0;
            let mut j: i32 = 0i32;
            loop {
                if j >= i { break; }
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&value), j)?;
                StringUTF16::putChar(Clone::clone(&buf), j, (_t1 as i32))?;
                j = j.wrapping_add(1i32);
            }
            loop {
                if i >= len { break; }
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let mut j: u16 = _t1;
                StringUTF16::putChar(Clone::clone(&buf), i, ((if (j as i32) == (oldChar as i32) { newChar } else { j }) as i32))?;
                i = i.wrapping_add(1i32);
            }
            let _t1: bool = StringLatin1::canEncode_c(oldChar)?;
            let _t2: bool = StringLatin1::canEncode_c(newChar)?;
            if _t2 {
                let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_b_i_i(Clone::clone(&buf), 0i32, len)?;
                let mut j: Rc<RefCell<Vec<i8>>> = _t3;
                let _t4: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&j), len)?;
                let mut coder: i8 = _t4;
                return Ok(String::new_arr_b_b(Clone::clone(&j), coder)?);
            }
            return Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?);
            Ok(Default::default())
        }

        #[java_method(name = "replace", descriptor = "([BIZ[BIZ[BIZ)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace([BIZ[BIZ[BIZ)Ljava/lang/String;
        pub fn replace_arr_b_i_z_arr_b_i_z_arr_b_i_z(mut value: Rc<RefCell<Vec<i8>>>, mut valLen: i32, mut valLat1: bool, mut targ: Rc<RefCell<Vec<i8>>>, mut targLen: i32, mut targLat1: bool, mut repl: Rc<RefCell<Vec<i8>>>, mut replLen: i32, mut replLat1: bool) -> Result<String> {
            if (targLen<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if replLat1 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if !(targLat1) {
                return Ok(Default::default());
            }
            let mut _merged4: i32;
            if String::COMPACT_STRINGS() {
                let mut _merged3: i32;
                if valLat1 {
                    let _t0: i32 = StringLatin1::indexOf_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                    _merged3 = _t0;
                } else {
                    let mut _merged2: i32;
                    if String::COMPACT_STRINGS() {
                        let mut _merged1: i32;
                        if targLat1 {
                            let _t0: i32 = StringUTF16::indexOfLatin1_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                            _merged1 = _t0;
                        } else {
                            let _t0: i32 = StringUTF16::indexOf_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                            _merged1 = _t0;
                        }
                        _merged2 = _merged1;
                    } else {
                        let _t0: i32 = StringUTF16::indexOf_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                        _merged2 = _t0;
                    }
                    _merged3 = _merged2;
                }
                _merged4 = _merged3;
            } else {
                let mut _merged2: i32;
                if String::COMPACT_STRINGS() {
                    let mut _merged1: i32;
                    if targLat1 {
                        let _t0: i32 = StringUTF16::indexOfLatin1_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                        _merged1 = _t0;
                    } else {
                        let _t0: i32 = StringUTF16::indexOf_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                        _merged1 = _t0;
                    }
                    _merged2 = _merged1;
                } else {
                    let _t0: i32 = StringUTF16::indexOf_arr_b_arr_b(Clone::clone(&value), Clone::clone(&targ))?;
                    _merged2 = _t0;
                }
                _merged4 = _merged2;
            }
            let mut i: i32 = _merged4;
            if (i<0) {
                return Ok(Default::default());
            }
            let mut p: i32 = 0i32;
            let mut _arr5: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 16i32 as usize]));
            let mut pos: Rc<RefCell<Vec<i32>>> = _arr5;
            pos.borrow_mut()[0i32 as usize] = i;
            i = (i).wrapping_add(targLen);
            loop {
                let mut _merged10: i32;
                if String::COMPACT_STRINGS() {
                    let mut _merged9: i32;
                    if valLat1 {
                        let _t6: i32 = StringLatin1::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                        _merged9 = _t6;
                    } else {
                        let mut _merged8: i32;
                        if String::COMPACT_STRINGS() {
                            let mut _merged7: i32;
                            if targLat1 {
                                let _t6: i32 = StringUTF16::indexOfLatin1_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                                _merged7 = _t6;
                            } else {
                                let _t6: i32 = StringUTF16::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                                _merged7 = _t6;
                            }
                            _merged8 = _merged7;
                        } else {
                            let _t6: i32 = StringUTF16::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                            _merged8 = _t6;
                        }
                        _merged9 = _merged8;
                    }
                    _merged10 = _merged9;
                } else {
                    let mut _merged8: i32;
                    if String::COMPACT_STRINGS() {
                        let mut _merged7: i32;
                        if targLat1 {
                            let _t6: i32 = StringUTF16::indexOfLatin1_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                            _merged7 = _t6;
                        } else {
                            let _t6: i32 = StringUTF16::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                            _merged7 = _t6;
                        }
                        _merged8 = _merged7;
                    } else {
                        let _t6: i32 = StringUTF16::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                        _merged8 = _t6;
                    }
                    _merged10 = _merged8;
                }
                let mut j: i32 = _merged10;
                if (j<=0) { break; }
                p = p.wrapping_add(1i32);
                if p == (pos.borrow().len() as i32) {
                    let _t6: i32 = ArraysSupport::newLength(p, 1i32, (p>>((1i32&0x1f))))?;
                    let _t7: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&pos), _t6)?;
                    pos = _t7;
                }
                pos.borrow_mut()[p as usize] = j;
                i = (j).wrapping_add(targLen);
            }
            p = p.wrapping_add(1i32);
            let _t6: i32 = Math::multiplyExact_i_i(p, (replLen).wrapping_sub(targLen))?;
            let _t7: i32 = Math::addExact_i_i(valLen, _t6)?;
            let mut resultLen: i32 = _t7;
            if (resultLen==0) {
                return Ok(String::from(""));
            }
            let _t8: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(resultLen)?;
            let mut ignored: Rc<RefCell<Vec<i8>>> = _t8;
            let mut posFrom: i32 = 0i32;
            let mut posTo: i32 = 0i32;
            let mut q: i32 = 0i32;
        let mut c = Default::default();
            loop {
                if q >= p { break; }
                let mut nextPos = pos.borrow()[q as usize];
                c = Default::default();
                loop {
                    if posFrom >= nextPos { break; }
                    posFrom = posFrom.wrapping_add(1i32);
                    c = ((((value.borrow()[posFrom as usize] as i32)&255i32)) as u16 as i32);
                    posTo = posTo.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&ignored), posTo, c)?;
                }
                loop {
                    if posFrom >= nextPos { break; }
                    posTo = posTo.wrapping_add(1i32);
                    posFrom = posFrom.wrapping_add(1i32);
                    let _t9: u16 = StringUTF16::getChar(Clone::clone(&value), posFrom)?;
                    StringUTF16::putChar(Clone::clone(&ignored), posTo, (_t9 as i32))?;
                }
                posFrom = (posFrom).wrapping_add(targLen);
                if String::COMPACT_STRINGS() {
                    if replLat1 {
                        c = 0i32;
                        loop {
                            if c >= replLen { break; }
                            c = ((((repl.borrow()[c as usize] as i32)&255i32)) as u16 as i32);
                            posTo = posTo.wrapping_add(1i32);
                            StringUTF16::putChar(Clone::clone(&ignored), posTo, c)?;
                            c = c.wrapping_add(1i32);
                        }
                    } else {
                        c = 0i32;
                        loop {
                            if c >= replLen { break; }
                            posTo = posTo.wrapping_add(1i32);
                            let _t9: u16 = StringUTF16::getChar(Clone::clone(&repl), c)?;
                            StringUTF16::putChar(Clone::clone(&ignored), posTo, (_t9 as i32))?;
                            c = c.wrapping_add(1i32);
                        }
                    }
                } else {
                    c = 0i32;
                    loop {
                        if c >= replLen { break; }
                        posTo = posTo.wrapping_add(1i32);
                        let _t9: u16 = StringUTF16::getChar(Clone::clone(&repl), c)?;
                        StringUTF16::putChar(Clone::clone(&ignored), posTo, (_t9 as i32))?;
                        c = c.wrapping_add(1i32);
                    }
                }
                q = q.wrapping_add(1i32);
            }
            loop {
                if posFrom >= valLen { break; }
                posFrom = posFrom.wrapping_add(1i32);
                q = ((((value.borrow()[posFrom as usize] as i32)&255i32)) as u16 as i32);
                posTo = posTo.wrapping_add(1i32);
                StringUTF16::putChar(Clone::clone(&ignored), posTo, q)?;
            }
            loop {
                if posFrom >= valLen { break; }
                posTo = posTo.wrapping_add(1i32);
                posFrom = posFrom.wrapping_add(1i32);
                let _t9: u16 = StringUTF16::getChar(Clone::clone(&value), posFrom)?;
                StringUTF16::putChar(Clone::clone(&ignored), posTo, (_t9 as i32))?;
            }
            if !(targLat1) {
                let _t9: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_b_i_i(Clone::clone(&ignored), 0i32, resultLen)?;
                let mut q: Rc<RefCell<Vec<i8>>> = _t9;
                let _t10: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&q), resultLen)?;
                let mut nextPos: i8 = _t10;
                return Ok(String::new_arr_b_b(Clone::clone(&q), nextPos)?);
            }
            Ok(String::new_arr_b_b(Clone::clone(&ignored), ((1i32) as i8))?)
        }

        #[java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionMatchesCI(mut value: Rc<RefCell<Vec<i8>>>, mut toffset: i32, mut other: Rc<RefCell<Vec<i8>>>, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let _t0: i32 = StringUTF16::compareToCIImpl(Clone::clone(&value), toffset, len, Clone::clone(&other), ooffset, len)?;
            Ok((_t0==0))
        }

        #[java_method(name = "regionMatchesCI_Latin1", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionMatchesCI_Latin1(mut value: Rc<RefCell<Vec<i8>>>, mut toffset: i32, mut other: Rc<RefCell<Vec<i8>>>, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let _t0: bool = StringLatin1::regionMatchesCI_UTF16(Clone::clone(&other), ooffset, Clone::clone(&value), toffset, len)?;
            Ok(_t0)
        }

        #[java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut locale: Locale) -> Result<String> {
            if _is_jnull(&locale) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut hasSurr: i32 = 0i32;
            let mut len = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut first: i32 = 0i32;
            loop {
                if first >= len { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), first)?;
                let mut cp: u16 = _t0;
                let _t1: bool = Character::isSurrogate(((((cp) as u16 as i32)) as u16))?;
                if _t1 {
                    hasSurr = 1i32;
                    break;
                }
                let _t2: i32 = Character::toLowerCase_i((cp as i32))?;
                if (cp as i32) != _t2 {
                    break;
                }
                first = first.wrapping_add(1i32);
            }
            if first == len {
                return Ok(str);
            }
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (value.borrow().len() as i32) as usize]));
            let mut cp: Rc<RefCell<Vec<i8>>> = _arr0;
            System::arraycopy(Object::from_any(value.clone()), 0i32, Object::from_any(cp.clone()), 0i32, (first<<(1i32&0x1f)))?;
            let _t1 = locale.getLanguage()?;
            let mut lang: String = _t1;
            if Object::from_any(lang.clone()) == Object::from_any(String::from("lt").clone()) {
                let _t2: String = StringUTF16::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), first, Clone::clone(&locale), (1i32 != 0i32))?;
                return Ok(_t2);
            }
            if (hasSurr!=0) {
                let _t2: String = StringUTF16::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), first, Clone::clone(&locale), (0i32 != 0i32))?;
                return Ok(_t2);
            }
            let mut bits: i32 = 0i32;
            let mut i: i32 = first;
            loop {
                if i >= len { break; }
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let mut cp: u16 = _t2;
                let _t3: bool = Character::isSurrogate(((((cp) as u16 as i32)) as u16))?;
                if _t3 {
                    let _t4: String = StringUTF16::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), i, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t4);
                }
                if (cp as i32) == 304i32 {
                    let _t4: String = StringUTF16::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), i, Clone::clone(&locale), (1i32 != 0i32))?;
                    return Ok(_t4);
                }
                let _t4: i32 = Character::toLowerCase_i((cp as i32))?;
                let mut cp: i32 = _t4;
                let _t5: bool = Character::isBmpCodePoint(cp)?;
                if !(_t5) {
                    let _t6: String = StringUTF16::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), i, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t6);
                }
                bits = (bits|cp);
                StringUTF16::putChar(Clone::clone(&cp), i, cp)?;
                i = i.wrapping_add(1i32);
            }
            if bits > 255i32 {
                return Ok(String::new_arr_b_b(Clone::clone(&cp), ((1i32) as i8))?);
            }
            let _t2: String = StringUTF16::newString(Clone::clone(&cp), 0i32, len)?;
            Ok(_t2)
        }

        #[java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCaseEx(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut result: Rc<RefCell<Vec<i8>>>, mut first: i32, mut locale: Locale, mut localeDependent: bool) -> Result<String> {
            if (result.borrow().len() as i32) != (value.borrow().len() as i32) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (first<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut resultOffset: i32 = first;
            let mut length = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut i: i32 = first;
            let mut lowerCharArray: Rc<RefCell<Vec<u16>>> = Default::default();
            let mut lowerChar: i32 = Default::default();
        let mut srcChar: u16 = Default::default();
            loop {
                if i >= length { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                srcChar = _t0;
                let mut srcCount: i32 = 1i32;
                let _t1: bool = Character::isSurrogate(((((srcChar) as u16 as i32)) as u16))?;
                if _t1 {
                    let _t2: i32 = StringUTF16::codePointAt_arr_b_i_i(Clone::clone(&value), i, length)?;
                    srcChar = _t2;
                    let _t3: i32 = Character::charCount(srcChar)?;
                    srcCount = _t3;
                }
                if srcChar == 304i32 {
                    let _t2: i32 = ConditionalSpecialCasing::toLowerCaseEx(Clone::clone(&str), i, Clone::clone(&locale))?;
                    lowerChar = _t2;
                } else {
                    let _t2: i32 = Character::toLowerCase_i(srcChar)?;
                    let mut lowerChar: i32 = _t2;
                }
                let _t2: bool = Character::isBmpCodePoint(lowerChar)?;
                if _t2 {
                    resultOffset = resultOffset.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&result), resultOffset, lowerChar)?;
                } else {
                    if lowerChar == -1i32 {
                        let _t3: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::toLowerCaseCharArray(Clone::clone(&str), i, Clone::clone(&locale))?;
                        lowerCharArray = _t3;
                    } else {
                        let _t3: Rc<RefCell<Vec<u16>>> = Character::toChars_i(lowerChar)?;
                        let mut lowerCharArray: Rc<RefCell<Vec<u16>>> = _t3;
                    }
                    let mut mapLen = (lowerCharArray.borrow().len() as i32);
                    if mapLen > srcCount {
                        let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(((((result.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(srcCount))?;
                        let mut result2: Rc<RefCell<Vec<i8>>> = _t3;
                        System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(result2.clone()), 0i32, (resultOffset<<(1i32&0x1f)))?;
                        result = result2;
                    }
                    if (resultOffset<0) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let _t3: i32 = StringUTF16::length(Clone::clone(&result))?;
                    if (resultOffset).wrapping_add(mapLen) > _t3 {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let mut result2: i32 = 0i32;
                    loop {
                        if result2 >= mapLen { break; }
                        resultOffset = resultOffset.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&result), resultOffset, (lowerCharArray.borrow()[result2 as usize] as i32))?;
                        result2 = result2.wrapping_add(1i32);
                    }
                }
                i = (i).wrapping_add(srcCount);
            }
            let _t0: String = StringUTF16::newString(Clone::clone(&result), 0i32, resultOffset)?;
            Ok(_t0)
        }

        #[java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut locale: Locale) -> Result<String> {
            if _is_jnull(&locale) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut hasSurr: i32 = 0i32;
            let mut len = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut first: i32 = 0i32;
            loop {
                if first >= len { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), first)?;
                let mut cp: u16 = _t0;
                let _t1: bool = Character::isSurrogate(((((cp) as u16 as i32)) as u16))?;
                if _t1 {
                    hasSurr = 1i32;
                    break;
                }
                let _t2: i32 = Character::toUpperCaseEx((cp as i32))?;
                if (cp as i32) != _t2 {
                    break;
                }
                first = first.wrapping_add(1i32);
            }
            if first == len {
                return Ok(str);
            }
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (value.borrow().len() as i32) as usize]));
            let mut cp: Rc<RefCell<Vec<i8>>> = _arr0;
            System::arraycopy(Object::from_any(value.clone()), 0i32, Object::from_any(cp.clone()), 0i32, (first<<(1i32&0x1f)))?;
            let _t1 = locale.getLanguage()?;
            let mut lang: String = _t1;
            if Object::from_any(lang.clone()) == Object::from_any(String::from("lt").clone()) {
                let _t2: String = StringUTF16::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), first, Clone::clone(&locale), (1i32 != 0i32))?;
                return Ok(_t2);
            }
            if (hasSurr!=0) {
                let _t2: String = StringUTF16::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), first, Clone::clone(&locale), (0i32 != 0i32))?;
                return Ok(_t2);
            }
            let mut bits: i32 = 0i32;
            let mut i: i32 = first;
            loop {
                if i >= len { break; }
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                let mut cp: u16 = _t2;
                let _t3: bool = Character::isSurrogate(((((cp) as u16 as i32)) as u16))?;
                if _t3 {
                    let _t4: String = StringUTF16::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), i, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t4);
                }
                let _t4: i32 = Character::toUpperCaseEx((cp as i32))?;
                let mut cp: i32 = _t4;
                let _t5: bool = Character::isBmpCodePoint(cp)?;
                if !(_t5) {
                    let _t6: String = StringUTF16::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), Clone::clone(&cp), i, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t6);
                }
                bits = (bits|cp);
                StringUTF16::putChar(Clone::clone(&cp), i, cp)?;
                i = i.wrapping_add(1i32);
            }
            if bits > 255i32 {
                return Ok(String::new_arr_b_b(Clone::clone(&cp), ((1i32) as i8))?);
            }
            let _t2: String = StringUTF16::newString(Clone::clone(&cp), 0i32, len)?;
            Ok(_t2)
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut result: Rc<RefCell<Vec<i8>>>, mut first: i32, mut locale: Locale, mut localeDependent: bool) -> Result<String> {
            if (result.borrow().len() as i32) != (value.borrow().len() as i32) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (first<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut resultOffset: i32 = first;
            let mut length = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut i: i32 = first;
            let mut upperChar: i32 = Default::default();
        let mut srcChar: u16 = Default::default();
            loop {
                if i >= length { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), i)?;
                srcChar = _t0;
                let mut srcCount: i32 = 1i32;
                let _t1: bool = Character::isSurrogate(((((srcChar) as u16 as i32)) as u16))?;
                if _t1 {
                    let _t2: i32 = StringUTF16::codePointAt_arr_b_i_i(Clone::clone(&value), i, length)?;
                    srcChar = _t2;
                    let _t3: i32 = Character::charCount(srcChar)?;
                    srcCount = _t3;
                }
                if localeDependent {
                    let _t2: i32 = ConditionalSpecialCasing::toUpperCaseEx(Clone::clone(&str), i, Clone::clone(&locale))?;
                    upperChar = _t2;
                } else {
                    let _t2: i32 = Character::toUpperCaseEx(srcChar)?;
                    let mut upperChar: i32 = _t2;
                }
                let _t2: bool = Character::isBmpCodePoint(upperChar)?;
                if _t2 {
                    resultOffset = resultOffset.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&result), resultOffset, upperChar)?;
                } else {
        let mut upperCharArray: Rc<RefCell<Vec<u16>>> = Default::default();
                    if upperChar == -1i32 {
                        if localeDependent {
                            let _t3: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::toUpperCaseCharArray(Clone::clone(&str), i, Clone::clone(&locale))?;
                            upperCharArray = _t3;
                        } else {
                            let _t3: Rc<RefCell<Vec<u16>>> = Character::toUpperCaseCharArray(srcChar)?;
                            upperCharArray = _t3;
                        }
                    } else {
                        let _t3: Rc<RefCell<Vec<u16>>> = Character::toChars_i(upperChar)?;
                        upperCharArray = _t3;
                    }
                    let mut mapLen = (upperCharArray.borrow().len() as i32);
                    if mapLen > srcCount {
                        let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(((((result.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(srcCount))?;
                        let mut result2: Rc<RefCell<Vec<i8>>> = _t3;
                        System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(result2.clone()), 0i32, (resultOffset<<(1i32&0x1f)))?;
                        result = result2;
                    }
                    if (resultOffset<0) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let _t3: i32 = StringUTF16::length(Clone::clone(&result))?;
                    if (resultOffset).wrapping_add(mapLen) > _t3 {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                    let mut result2: i32 = 0i32;
                    loop {
                        if result2 >= mapLen { break; }
                        resultOffset = resultOffset.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&result), resultOffset, (upperCharArray.borrow()[result2 as usize] as i32))?;
                        result2 = result2.wrapping_add(1i32);
                    }
                }
                i = (i).wrapping_add(srcCount);
            }
            let _t0: String = StringUTF16::newString(Clone::clone(&result), 0i32, resultOffset)?;
            Ok(_t0)
        }

        #[java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trim(mut value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            let mut length = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut len: i32 = length;
            let mut st: i32 = 0i32;
            loop {
                if st >= len { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), st)?;
                if (_t0 as i32) <= 32i32 {
                    st = st.wrapping_add(1i32);
                    continue;
                }
                break;
            }
            loop {
                if st >= len { break; }
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), (len).wrapping_sub(1i32))?;
                if (_t0 as i32) <= 32i32 {
                    len = len.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut _merged1: String;
            if len < length {
                let _t0: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&value), (st<<(1i32&0x1f)), (len<<(1i32&0x1f)))?;
                _merged1 = String::new_arr_b_b(Clone::clone(&_t0), ((1i32) as i8))?;
            } else {
                _merged1 = Default::default();
            }
            Ok(_merged1)
        }

        #[java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfNonWhitespace(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut length = ((value.borrow().len() as i32)>>((1i32&0x1f)));
            let mut left: i32 = 0i32;
            loop {
                if left >= length { break; }
                let _t0: i32 = StringUTF16::codePointAt_arr_b_i_i(Clone::clone(&value), left, length)?;
                let mut codepoint: i32 = _t0;
                if codepoint != 32i32 {
                    if codepoint != 9i32 {
                        let _t1: bool = Character::isWhitespace_i(codepoint)?;
                        if !(_t1) {
                            break;
                        }
                    } else {
                        let _t1: i32 = Character::charCount(codepoint)?;
                        left = (left).wrapping_add(_t1);
                        continue;
                    }
                } else {
                    let _t1: i32 = Character::charCount(codepoint)?;
                    left = (left).wrapping_add(_t1);
                    continue;
                }
            }
            Ok(left)
        }

        #[java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOfNonWhitespace(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut length = (((value.borrow().len() as i32) as u32>>(1i32&0x1f)) as i32);
            let mut right: i32 = length;
            loop {
                if 0i32 >= right { break; }
                let _t0: i32 = StringUTF16::codePointBefore_arr_b_i(Clone::clone(&value), right)?;
                let mut codepoint: i32 = _t0;
                if codepoint != 32i32 {
                    if codepoint != 9i32 {
                        let _t1: bool = Character::isWhitespace_i(codepoint)?;
                        if !(_t1) {
                            break;
                        }
                    } else {
                        let _t1: i32 = Character::charCount(codepoint)?;
                        right = (right).wrapping_sub(_t1);
                        continue;
                    }
                } else {
                    let _t1: i32 = Character::charCount(codepoint)?;
                    right = (right).wrapping_sub(_t1);
                    continue;
                }
            }
            Ok(right)
        }

        #[java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn strip(mut value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            let mut length = (((value.borrow().len() as i32) as u32>>(1i32&0x1f)) as i32);
            let _t0: i32 = StringUTF16::indexOfNonWhitespace(Clone::clone(&value))?;
            let mut left: i32 = _t0;
            if left == length {
                return Ok(String::from(""));
            }
            let _t1: i32 = StringUTF16::lastIndexOfNonWhitespace(Clone::clone(&value))?;
            let mut right: i32 = _t1;
            let mut ifChanged = (right < length) as i32;
            let mut _merged3: String;
            if (ifChanged!=0) {
                let _t2: String = StringUTF16::newString(Clone::clone(&value), left, (right).wrapping_sub(left))?;
                _merged3 = _t2;
            } else {
                _merged3 = Default::default();
            }
            Ok(_merged3)
        }

        #[java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripLeading(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            panic!("stub: java/lang/StringUTF16.stripLeading:([B)Ljava/lang/String;")
        }

        #[java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripTrailing(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            panic!("stub: java/lang/StringUTF16.stripTrailing:([B)Ljava/lang/String;")
        }

        #[java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([B)Ljava/util/stream/Stream<Ljava/lang/String;>;")]
        pub fn lines(value: Rc<RefCell<Vec<i8>>>) -> Result<Object> {
            panic!("stub: java/lang/StringUTF16.lines:([B)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "putChars", descriptor = "([BI[CII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChars(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut str: Rc<RefCell<Vec<u16>>>, mut off: i32, mut end: i32) -> Result<()> {
            loop {
                if off >= end { break; }
                index = index.wrapping_add(1i32);
                off = off.wrapping_add(1i32);
                StringUTF16::putChar(Clone::clone(&val), index, (str.borrow()[off as usize] as i32))?;
            }
            Ok(())
        }

        #[java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newString(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut len: i32) -> Result<String> {
            if (len==0) {
                return Ok(String::from(""));
            }
            if String::COMPACT_STRINGS() {
                let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_b_i_i(Clone::clone(&val), index, len)?;
                let mut res: Rc<RefCell<Vec<i8>>> = _t0;
                let _t1: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&res), len)?;
                let mut coder: i8 = _t1;
                return Ok(String::new_arr_b_b(Clone::clone(&res), coder)?);
            }
            let mut res = (index).wrapping_add(len);
            let _t0: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), (index<<(1i32&0x1f)), (res<<(1i32&0x1f)))?;
            Ok(String::new_arr_b_b(Clone::clone(&_t0), ((1i32) as i8))?)
        }

        #[java_method(name = "fillNull", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fillNull(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut end: i32) -> Result<()> {
            Arrays::fill_arr_b_i_i_b(Clone::clone(&val), (index<<(1i32&0x1f)), (end<<(1i32&0x1f)), ((0i32) as i8))?;
            Ok(())
        }

        #[java_method(name = "putCharSB", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharSB(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut c: i32) -> Result<()> {
            StringUTF16::checkIndex(index, Clone::clone(&val))?;
            StringUTF16::putChar(Clone::clone(&val), index, c)?;
            Ok(())
        }

        #[java_method(name = "putCharsSB", descriptor = "([BI[CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: putCharsSB([BI[CII)V
        pub fn putCharsSB_arr_b_i_arr_c_i_i(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut ca: Rc<RefCell<Vec<u16>>>, mut off: i32, mut end: i32) -> Result<()> {
            StringUTF16::checkBoundsBeginEnd(index, ((index).wrapping_add(end)).wrapping_sub(off), Clone::clone(&val))?;
            StringUTF16::putChars(Clone::clone(&val), index, Clone::clone(&ca), off, end)?;
            Ok(())
        }

        #[java_method(name = "putCharsSB", descriptor = "([BILjava/lang/CharSequence;II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: putCharsSB([BILjava/lang/CharSequence;II)V
        pub fn putCharsSB_arr_b_i_seq_i_i(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut s: Object, mut off: i32, mut end: i32) -> Result<()> {
            StringUTF16::checkBoundsBeginEnd(index, ((index).wrapping_add(end)).wrapping_sub(off), Clone::clone(&val))?;
            let mut i: i32 = off;
            loop {
                if i >= end { break; }
                index = index.wrapping_add(1i32);
                let _vdispatch0: u16 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
                StringUTF16::putChar(Clone::clone(&val), index, (_vdispatch0 as i32))?;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "codePointAtSB", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAtSB(val: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.codePointAtSB:([BII)I")
        }

        #[java_method(name = "codePointBeforeSB", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBeforeSB(val: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.codePointBeforeSB:([BI)I")
        }

        #[java_method(name = "codePointCountSB", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCountSB(val: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.codePointCountSB:([BII)I")
        }

        #[java_method(name = "getChars", descriptor = "(III[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getChars(III[B)I
        pub fn getChars_i_i_i_arr_b(mut i: i32, mut begin: i32, mut end: i32, mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            StringUTF16::checkBoundsBeginEnd(begin, end, Clone::clone(&value))?;
            let _t0: i32 = StringUTF16::getChars_i_i_arr_b(i, end, Clone::clone(&value))?;
            let mut pos: i32 = _t0;
            if begin != pos {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(pos)
        }

        #[java_method(name = "getChars", descriptor = "(JII[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getChars(JII[B)I
        pub fn getChars_l_i_i_arr_b(mut l: i64, mut begin: i32, mut end: i32, mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            StringUTF16::checkBoundsBeginEnd(begin, end, Clone::clone(&value))?;
            let _t0: i32 = StringUTF16::getChars_l_i_arr_b(l, end, Clone::clone(&value))?;
            let mut pos: i32 = _t0;
            if begin != pos {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(pos)
        }

        #[java_method(name = "contentEquals", descriptor = "([B[BI)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contentEquals_arr_b_arr_b_i(v1: Rc<RefCell<Vec<i8>>>, v2: Rc<RefCell<Vec<i8>>>, len: i32) -> Result<bool> {
            panic!("stub: java/lang/StringUTF16.contentEquals:([B[BI)Z")
        }

        #[java_method(name = "contentEquals", descriptor = "([BLjava/lang/CharSequence;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contentEquals_arr_b_seq_i(value: Rc<RefCell<Vec<i8>>>, cs: Object, len: i32) -> Result<bool> {
            panic!("stub: java/lang/StringUTF16.contentEquals:([BLjava/lang/CharSequence;I)Z")
        }

        #[java_method(name = "putCharsAt", descriptor = "([BICCCC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: putCharsAt([BICCCC)I
        pub fn putCharsAt_arr_b_i_c_c_c_c(mut value: Rc<RefCell<Vec<i8>>>, mut i: i32, mut c1: u16, mut c2: u16, mut c3: u16, mut c4: u16) -> Result<i32> {
            let mut end = (i).wrapping_add(4i32);
            StringUTF16::checkBoundsBeginEnd(i, end, Clone::clone(&value))?;
            i = i.wrapping_add(1i32);
            StringUTF16::putChar(Clone::clone(&value), i, (c1 as i32))?;
            i = i.wrapping_add(1i32);
            StringUTF16::putChar(Clone::clone(&value), i, (c2 as i32))?;
            i = i.wrapping_add(1i32);
            StringUTF16::putChar(Clone::clone(&value), i, (c3 as i32))?;
            i = i.wrapping_add(1i32);
            StringUTF16::putChar(Clone::clone(&value), i, (c4 as i32))?;
            if i != end {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(end)
        }

        #[java_method(name = "putCharsAt", descriptor = "([BICCCCC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharsAt_arr_b_i_c_c_c_c_c(value: Rc<RefCell<Vec<i8>>>, i: i32, c1: u16, c2: u16, c3: u16, c4: u16, c5: u16) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.putCharsAt:([BICCCCC)I")
        }

        #[java_method(name = "charAt", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<u16> {
            StringUTF16::checkIndex(index, Clone::clone(&value))?;
            let _t0: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            Ok(_t0)
        }

        #[java_method(name = "reverse", descriptor = "([BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverse(val: Rc<RefCell<Vec<i8>>>, count: i32) -> Result<()> {
            panic!("stub: java/lang/StringUTF16.reverse:([BI)V")
        }

        #[java_method(name = "reverseAllValidSurrogatePairs", descriptor = "([BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverseAllValidSurrogatePairs(val: Rc<RefCell<Vec<i8>>>, count: i32) -> Result<()> {
            panic!("stub: java/lang/StringUTF16.reverseAllValidSurrogatePairs:([BI)V")
        }

        #[java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inflate(mut src: Rc<RefCell<Vec<i8>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32, mut len: i32) -> Result<()> {
            StringUTF16::checkBoundsOffCount(dstOff, len, Clone::clone(&dst))?;
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                dstOff = dstOff.wrapping_add(1i32);
                srcOff = srcOff.wrapping_add(1i32);
                StringUTF16::putChar(Clone::clone(&dst), dstOff, ((src.borrow()[srcOff as usize] as i32)&255i32))?;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "lastIndexOfLatin1", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOfLatin1(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/StringUTF16.lastIndexOfLatin1:([BI[BII)I")
        }

        #[native]
        #[java_native(name = "isBigEndian", descriptor = "()Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isBigEndian() -> Result<bool> {
            panic!("native: java/lang/StringUTF16.isBigEndian:()Z")
        }

        #[java_method(name = "getChars", descriptor = "(II[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getChars(II[B)I
        pub fn getChars_i_i_arr_b(mut i: i32, mut index: i32, mut buf: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut charPos: i32 = index;
            let mut negative = ((i<0)) as i32;
            if (negative==0) {
                i = (i).wrapping_neg();
            }
            loop {
                if i > -100i32 { break; }
                let mut q = (i/100i32);
                let mut r = ((q).wrapping_mul(100i32)).wrapping_sub(i);
                i = q;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitOnes().borrow()[r as usize] as i32))?;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitTens().borrow()[r as usize] as i32))?;
            }
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitOnes().borrow()[(i).wrapping_neg() as usize] as i32))?;
            if i < -9i32 {
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitTens().borrow()[(i).wrapping_neg() as usize] as i32))?;
            }
            if (negative!=0) {
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, 45i32)?;
            }
            Ok(charPos)
        }

        #[java_method(name = "getChars", descriptor = "(JI[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getChars(JI[B)I
        pub fn getChars_l_i_arr_b(mut i: i64, mut index: i32, mut buf: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut charPos: i32 = index;
            let mut negative = ((((i>(0i64)) as i32-((i)<(0i64)) as i32)<0)) as i32;
            if (negative==0) {
                i = (i).wrapping_neg();
            }
            let mut r: i32 = Default::default();
            loop {
                if (((i>(-2147483648i64)) as i32-((i)<(-2147483648i64)) as i32)>0) { break; }
                let mut q = (i/100i64);
                r = (((q).wrapping_mul(100i64)).wrapping_sub(i) as i32);
                i = q;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitOnes().borrow()[r as usize] as i32))?;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitTens().borrow()[r as usize] as i32))?;
            }
            let mut i2: i32 = (i as i32);
            loop {
                if i2 > -100i32 { break; }
                let mut q2 = (i2/100i32);
                r = ((q2).wrapping_mul(100i32)).wrapping_sub(i2);
                i2 = q2;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitOnes().borrow()[r as usize] as i32))?;
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitTens().borrow()[r as usize] as i32))?;
            }
            charPos = charPos.wrapping_sub(1i32);
            StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitOnes().borrow()[(i2).wrapping_neg() as usize] as i32))?;
            if i2 < -9i32 {
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::DigitTens().borrow()[(i2).wrapping_neg() as usize] as i32))?;
            }
            if (negative!=0) {
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, 45i32)?;
            }
            Ok(charPos)
        }

        #[java_method(name = "checkIndex", descriptor = "(I[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIndex(mut off: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            String::checkIndex(off, _t0)?;
            Ok(())
        }

        #[java_method(name = "checkOffset", descriptor = "(I[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkOffset(mut off: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            String::checkOffset(off, _t0)?;
            Ok(())
        }

        #[java_method(name = "checkBoundsBeginEnd", descriptor = "(II[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBoundsBeginEnd(mut begin: i32, mut end: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            String::checkBoundsBeginEnd(begin, end, _t0)?;
            Ok(())
        }

        #[java_method(name = "checkBoundsOffCount", descriptor = "(II[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBoundsOffCount(mut offset: i32, mut count: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
            let _t1: i32 = String::checkBoundsOffCount(offset, count, _t0)?;
            Ok(())
        }
    }
}
