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
    #[binary_name       = "java/lang/StringLatin1"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringLatin1.java"]
    #[inner_classes     = "java/lang/StringLatin1$LinesSpliterator:java/lang/StringLatin1:LinesSpliterator:26;java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/StringLatin1"]

    pub struct StringLatin1;

    impl StringLatin1 {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/StringLatin1.<init>:()V")
        }

        #[java_method(name = "charAt", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(mut value: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<u16> {
            String::checkIndex(index, (value.borrow().len() as i32))?;
            Ok(((((((value.borrow()[index as usize] as i32)&255i32)) as u16 as i32)) as u16))
        }

        #[java_method(name = "canEncode", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: canEncode(C)Z
        pub fn canEncode_c(mut cp: u16) -> Result<bool> {
            Ok((cp as i32) <= 255i32)
        }

        #[java_method(name = "canEncode", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: canEncode(I)Z
        pub fn canEncode_i(mut cp: i32) -> Result<bool> {
            Ok((if (cp>=0) { cp <= 255i32 } else { (0i32 != 0) }))
        }

        #[java_method(name = "length", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            Ok((value.borrow().len() as i32))
        }

        #[java_method(name = "codePointAt", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAt(value: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.codePointAt:([BII)I")
        }

        #[java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore(value: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.codePointBefore:([BI)I")
        }

        #[java_method(name = "codePointCount", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCount(value: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.codePointCount:([BII)I")
        }

        #[java_method(name = "toChars", descriptor = "([B)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toChars(mut value: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<u16>>>> {
            let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; (value.borrow().len() as i32) as usize]));
            let mut dst: Rc<RefCell<Vec<u16>>> = _arr0;
            StringLatin1::inflate_arr_b_i_arr_c_i_i(Clone::clone(&value), 0i32, Clone::clone(&dst), 0i32, (value.borrow().len() as i32))?;
            Ok(dst)
        }

        #[java_method(name = "inflate", descriptor = "([BII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inflate_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/StringLatin1.inflate:([BII)[B")
        }

        #[java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChars(mut value: Rc<RefCell<Vec<i8>>>, mut srcBegin: i32, mut srcEnd: i32, mut dst: Rc<RefCell<Vec<u16>>>, mut dstBegin: i32) -> Result<()> {
            StringLatin1::inflate_arr_b_i_arr_c_i_i(Clone::clone(&value), srcBegin, Clone::clone(&dst), dstBegin, (srcEnd).wrapping_sub(srcBegin))?;
            Ok(())
        }

        #[java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytes(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
            panic!("stub: java/lang/StringLatin1.getBytes:([BII[BI)V")
        }

        #[java_method(name = "equals", descriptor = "([B[B)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
            let mut i: i32 = 0i32;
            loop {
                if i >= (value.borrow().len() as i32) { break; }
                if (value.borrow()[i as usize] as i32) != (other.borrow()[i as usize] as i32) {
                    return Ok((0i32 != 0i32));
                }
                i = i.wrapping_add(1i32);
            }
            return Ok((1i32 != 0i32));
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "compareTo", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareTo([B[B)I
        pub fn compareTo_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut len1 = (value.borrow().len() as i32);
            let mut len2 = (other.borrow().len() as i32);
            let _t0: i32 = StringLatin1::compareTo_arr_b_arr_b_i_i(Clone::clone(&value), Clone::clone(&other), len1, len2)?;
            Ok(_t0)
        }

        #[java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareTo([B[BII)I
        pub fn compareTo_arr_b_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            let _t0: i32 = Math::min_i_i(len1, len2)?;
            let mut lim: i32 = _t0;
            let _t1: i32 = ArraysSupport::mismatch_arr_b_arr_b_i(Clone::clone(&value), Clone::clone(&other), lim)?;
            let mut k: i32 = _t1;
            let mut _merged4: i32;
            if (k<0) {
                _merged4 = (len1).wrapping_sub(len2);
            } else {
                let _t2: u16 = StringLatin1::getChar(Clone::clone(&value), k)?;
                let _t3: u16 = StringLatin1::getChar(Clone::clone(&other), k)?;
                _merged4 = ((_t2 as i32)).wrapping_sub((_t3 as i32));
            }
            Ok(_merged4)
        }

        #[java_method(name = "compareToUTF16", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareToUTF16([B[B)I
        pub fn compareToUTF16_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let _t0: i32 = StringLatin1::length(Clone::clone(&value))?;
            let mut len1: i32 = _t0;
            let _t1: i32 = StringUTF16::length(Clone::clone(&other))?;
            let mut len2: i32 = _t1;
            let _t2: i32 = StringLatin1::compareToUTF16Values(Clone::clone(&value), Clone::clone(&other), len1, len2)?;
            Ok(_t2)
        }

        #[java_method(name = "compareToUTF16", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: compareToUTF16([B[BII)I
        pub fn compareToUTF16_arr_b_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            let _t0: i32 = StringLatin1::length(Clone::clone(&value))?;
            String::checkOffset(len1, _t0)?;
            let _t1: i32 = StringUTF16::length(Clone::clone(&other))?;
            String::checkOffset(len2, _t1)?;
            let _t2: i32 = StringLatin1::compareToUTF16Values(Clone::clone(&value), Clone::clone(&other), len1, len2)?;
            Ok(_t2)
        }

        #[java_method(name = "compareToUTF16Values", descriptor = "([B[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToUTF16Values(mut value: Rc<RefCell<Vec<i8>>>, mut other: Rc<RefCell<Vec<i8>>>, mut len1: i32, mut len2: i32) -> Result<i32> {
            let _t0: i32 = Math::min_i_i(len1, len2)?;
            let mut lim: i32 = _t0;
            let mut k: i32 = 0i32;
            loop {
                if k >= lim { break; }
                let _t1: u16 = StringLatin1::getChar(Clone::clone(&value), k)?;
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

        #[java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToCI(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.compareToCI:([B[B)I")
        }

        #[java_method(name = "compareToCI_UTF16", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToCI_UTF16(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.compareToCI_UTF16:([B[B)I")
        }

        #[java_method(name = "hashCode", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.hashCode:([B)I")
        }

        #[java_method(name = "indexOf", descriptor = "([BIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([BIII)I
        pub fn indexOf_arr_b_i_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut toIndex: i32) -> Result<i32> {
            let _t0: bool = StringLatin1::canEncode_i(ch)?;
            if !(_t0) {
                return Ok(-1i32);
            }
            let _t1: i32 = Math::max_i_i(fromIndex, 0i32)?;
            fromIndex = _t1;
            let _t2: i32 = Math::min_i_i(toIndex, (value.borrow().len() as i32))?;
            toIndex = _t2;
            if fromIndex >= toIndex {
                return Ok(-1i32);
            }
            let _t3: i32 = StringLatin1::indexOfChar(Clone::clone(&value), ch, fromIndex, toIndex)?;
            Ok(_t3)
        }

        #[java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfChar(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut max: i32) -> Result<i32> {
            let mut c = ((ch) as i8 as i32);
            let mut i: i32 = fromIndex;
            loop {
                if i >= max { break; }
                if (value.borrow()[i as usize] as i32) == c {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "indexOf", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([B[B)I
        pub fn indexOf_arr_b_arr_b(mut value: Rc<RefCell<Vec<i8>>>, mut str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            if ((str.borrow().len() as i32)==0) {
                return Ok(0i32);
            }
            if ((value.borrow().len() as i32)==0) {
                return Ok(-1i32);
            }
            let _t0: i32 = StringLatin1::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), (value.borrow().len() as i32), Clone::clone(&str), (str.borrow().len() as i32), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf([BI[BII)I
        pub fn indexOf_arr_b_i_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut valueCount: i32, mut str: Rc<RefCell<Vec<i8>>>, mut strCount: i32, mut fromIndex: i32) -> Result<i32> {
            let mut first = (str.borrow()[0i32 as usize] as i32);
            let mut max = (valueCount).wrapping_sub(strCount);
            let mut i: i32 = fromIndex;
            loop {
                if i > max { break; }
                loop {
                    i = i.wrapping_add(1i32);
                    if i > max { break; }
                    if (value.borrow()[i as usize] as i32) != first {
                        continue;
                    }
                    break;
                }
                let mut j = (i).wrapping_add(1i32);
                let mut end = ((j).wrapping_add(strCount)).wrapping_sub(1i32);
                let mut k: i32 = 1i32;
                loop {
                    if j >= end { break; }
                    if (value.borrow()[j as usize] as i32) == (str.borrow()[k as usize] as i32) {
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

        #[java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_arr_b_i_arr_b_i_i(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/StringLatin1.lastIndexOf:([BI[BII)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: lastIndexOf([BII)I
        pub fn lastIndexOf_arr_b_i_i(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
            let _t0: bool = StringLatin1::canEncode_i(ch)?;
            if !(_t0) {
                return Ok(-1i32);
            }
            let _t1: i32 = Math::min_i_i(fromIndex, ((value.borrow().len() as i32)).wrapping_sub(1i32))?;
            let mut off: i32 = _t1;
            loop {
                if (off<0) { break; }
                if (value.borrow()[off as usize] as i32) == ((ch) as i8 as i32) {
                    return Ok(off);
                }
                off = off.wrapping_sub(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace([BCC)Ljava/lang/String;
        pub fn replace_arr_b_c_c(mut value: Rc<RefCell<Vec<i8>>>, mut oldChar: u16, mut newChar: u16) -> Result<String> {
            let _t0: bool = StringLatin1::canEncode_c(oldChar)?;
            let mut len = (value.borrow().len() as i32);
            let mut i: i32 = -1i32;
            loop {
                i = i.wrapping_add(1i32);
                if (value.borrow()[i as usize] as i32) == ((oldChar) as i8 as i32) { break; }
            }
            let _t1: bool = StringLatin1::canEncode_c(newChar)?;
            let _t2: Rc<RefCell<Vec<i8>>> = StringConcatHelper::newArray((len as i64))?;
            let mut buf: Rc<RefCell<Vec<i8>>> = _t2;
            let mut j: i32 = 0i32;
            loop {
                if j >= i { break; }
                buf.borrow_mut()[j as usize] = ((value.borrow()[j as usize] as i32)) as i8;
                j = j.wrapping_add(1i32);
            }
            loop {
                if i >= len { break; }
                j = (value.borrow()[i as usize] as i32);
                buf.borrow_mut()[i as usize] = ((if j == ((oldChar) as i8 as i32) { ((newChar) as i8 as i32) } else { j })) as i8;
                i = i.wrapping_add(1i32);
            }
            return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
            let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(len)?;
            buf = _t3;
            StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&value), 0i32, Clone::clone(&buf), 0i32, i)?;
            loop {
                if i >= len { break; }
                j = ((((value.borrow()[i as usize] as i32)&255i32)) as u16 as i32);
                StringUTF16::putChar(Clone::clone(&buf), i, ((if j == (oldChar as i32) { newChar } else { (j as u16) }) as i32))?;
                i = i.wrapping_add(1i32);
            }
            return Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?);
            Ok(Default::default())
        }

        #[java_method(name = "replace", descriptor = "([BI[BI[BI)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace([BI[BI[BI)Ljava/lang/String;
        pub fn replace_arr_b_i_arr_b_i_arr_b_i(mut value: Rc<RefCell<Vec<i8>>>, mut valLen: i32, mut targ: Rc<RefCell<Vec<i8>>>, mut targLen: i32, mut repl: Rc<RefCell<Vec<i8>>>, mut replLen: i32) -> Result<String> {
            if (targLen<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut p: i32 = 0i32;
            let _t0: i32 = StringLatin1::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, 0i32)?;
            let mut i: i32 = _t0;
            if (i<0) {
                return Ok(Default::default());
            }
            let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 16i32 as usize]));
            let mut pos: Rc<RefCell<Vec<i32>>> = _arr1;
            pos.borrow_mut()[0i32 as usize] = i;
            i = (i).wrapping_add(targLen);
            loop {
                let _t2: i32 = StringLatin1::indexOf_arr_b_i_arr_b_i_i(Clone::clone(&value), valLen, Clone::clone(&targ), targLen, i)?;
                let mut j: i32 = _t2;
                if (j<=0) { break; }
                p = p.wrapping_add(1i32);
                if p == (pos.borrow().len() as i32) {
                    let _t2: i32 = ArraysSupport::newLength(p, 1i32, (p>>((1i32&0x1f))))?;
                    let _t3: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&pos), _t2)?;
                    pos = _t3;
                }
                pos.borrow_mut()[p as usize] = j;
                i = (j).wrapping_add(targLen);
            }
            p = p.wrapping_add(1i32);
            let _t2: i32 = Math::multiplyExact_i_i(p, (replLen).wrapping_sub(targLen))?;
            let _t3: i32 = Math::addExact_i_i(valLen, _t2)?;
            let mut resultLen: i32 = _t3;
            if (resultLen==0) {
                return Ok(String::from(""));
            }
            let _t4: Rc<RefCell<Vec<i8>>> = StringConcatHelper::newArray((resultLen as i64))?;
            let mut ignored: Rc<RefCell<Vec<i8>>> = _t4;
            let mut posFrom: i32 = 0i32;
            let mut posTo: i32 = 0i32;
            let mut q: i32 = 0i32;
            loop {
                if q >= p { break; }
                let mut nextPos = pos.borrow()[q as usize];
                loop {
                    if posFrom >= nextPos { break; }
                    posTo = posTo.wrapping_add(1i32);
                    posFrom = posFrom.wrapping_add(1i32);
                    ignored.borrow_mut()[posTo as usize] = ((value.borrow()[posFrom as usize] as i32)) as i8;
                }
                posFrom = (posFrom).wrapping_add(targLen);
                let mut k: i32 = 0i32;
                loop {
                    if k >= replLen { break; }
                    posTo = posTo.wrapping_add(1i32);
                    ignored.borrow_mut()[posTo as usize] = ((repl.borrow()[k as usize] as i32)) as i8;
                    k = k.wrapping_add(1i32);
                }
                q = q.wrapping_add(1i32);
            }
            loop {
                if posFrom >= valLen { break; }
                posTo = posTo.wrapping_add(1i32);
                posFrom = posFrom.wrapping_add(1i32);
                ignored.borrow_mut()[posTo as usize] = ((value.borrow()[posFrom as usize] as i32)) as i8;
            }
            Ok(String::new_arr_b_b(Clone::clone(&ignored), ((0i32) as i8))?)
        }

        #[java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionMatchesCI(mut value: Rc<RefCell<Vec<i8>>>, mut toffset: i32, mut other: Rc<RefCell<Vec<i8>>>, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let mut last = (toffset).wrapping_add(len);
            loop {
                if toffset >= last { break; }
                toffset = toffset.wrapping_add(1i32);
                let mut b1 = (value.borrow()[toffset as usize] as i32);
                ooffset = ooffset.wrapping_add(1i32);
                let mut b2 = (other.borrow()[ooffset as usize] as i32);
                let _t0: bool = CharacterDataLatin1::equalsIgnoreCase(((b1) as i8), ((b2) as i8))?;
                if _t0 {
                    continue;
                }
                break;
            }
            return Ok((0i32 != 0i32));
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "regionMatchesCI_UTF16", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn regionMatchesCI_UTF16(mut value: Rc<RefCell<Vec<i8>>>, mut toffset: i32, mut other: Rc<RefCell<Vec<i8>>>, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let mut last = (toffset).wrapping_add(len);
            loop {
                if toffset >= last { break; }
                toffset = toffset.wrapping_add(1i32);
                let mut c1 = ((((value.borrow()[toffset as usize] as i32)&255i32)) as u16 as i32);
                ooffset = ooffset.wrapping_add(1i32);
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&other), ooffset)?;
                let mut c2: u16 = _t0;
                if c1 == (c2 as i32) {
                    continue;
                }
                let _t1 = CharacterDataLatin1::instance().toUpperCase(c1)?;
                let mut u1 = ((_t1) as u16 as i32);
                let _t2: u16 = Character::toUpperCase_c(c2)?;
                let mut u2: u16 = _t2;
                if u1 == (u2 as i32) {
                    continue;
                }
                let _t3: u16 = Character::toLowerCase_c(((u1) as u16))?;
                let _t4: u16 = Character::toLowerCase_c(u2)?;
                if (_t3 as i32) == (_t4 as i32) {
                    continue;
                }
                break;
            }
            return Ok((0i32 != 0i32));
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut locale: Locale) -> Result<String> {
            if _is_jnull(&locale) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut len = (value.borrow().len() as i32);
            let mut first: i32 = 0i32;
            loop {
                if first >= len { break; }
                let mut cp = ((value.borrow()[first as usize] as i32)&255i32);
                let _t0 = CharacterDataLatin1::instance().toLowerCase(cp)?;
                if cp != _t0 {
                    break;
                }
                first = first.wrapping_add(1i32);
            }
            if first == len {
                return Ok(str);
            }
            let _t0 = locale.getLanguage()?;
            let mut cp: String = _t0;
            if Object::from_any(cp.clone()) == Object::from_any(String::from("lt").clone()) {
                let _t1: String = StringLatin1::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), first, Clone::clone(&locale), (1i32 != 0i32))?;
                return Ok(_t1);
            }
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; len as usize]));
            let mut result: Rc<RefCell<Vec<i8>>> = _arr1;
            System::arraycopy(Object::from_any(value.clone()), 0i32, Object::from_any(result.clone()), 0i32, first)?;
            let mut i: i32 = first;
            loop {
                if i >= len { break; }
                let mut cp = ((value.borrow()[i as usize] as i32)&255i32);
                let _t2 = CharacterDataLatin1::instance().toLowerCase(cp)?;
                cp = _t2;
                let _t3: bool = StringLatin1::canEncode_i(cp)?;
                if !(_t3) {
                    let _t4: String = StringLatin1::toLowerCaseEx(Clone::clone(&str), Clone::clone(&value), first, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t4);
                }
                result.borrow_mut()[i as usize] = (((cp) as i8 as i32)) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(String::new_arr_b_b(Clone::clone(&result), ((0i32) as i8))?)
        }

        #[java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCaseEx(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut first: i32, mut locale: Locale, mut localeDependent: bool) -> Result<String> {
            let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor((value.borrow().len() as i32))?;
            let mut result: Rc<RefCell<Vec<i8>>> = _t0;
            let mut resultOffset: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= first { break; }
                resultOffset = resultOffset.wrapping_add(1i32);
                StringUTF16::putChar(Clone::clone(&result), resultOffset, ((value.borrow()[i as usize] as i32)&255i32))?;
                i = i.wrapping_add(1i32);
            }
            i = first;
            let mut lowerCharArray: Rc<RefCell<Vec<u16>>> = Default::default();
            let mut lowerChar: i32 = Default::default();
            loop {
                if i >= (value.borrow().len() as i32) { break; }
                let mut srcChar = ((value.borrow()[i as usize] as i32)&255i32);
                if localeDependent {
                    let _t1: i32 = ConditionalSpecialCasing::toLowerCaseEx(Clone::clone(&str), i, Clone::clone(&locale))?;
                    lowerChar = _t1;
                } else {
                    let _t1 = CharacterDataLatin1::instance().toLowerCase(srcChar)?;
                    let mut lowerChar: i32 = _t1;
                }
                let _t1: bool = Character::isBmpCodePoint(lowerChar)?;
                if _t1 {
                    resultOffset = resultOffset.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&result), resultOffset, lowerChar)?;
                } else {
                    if lowerChar == -1i32 {
                        let _t2: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::toLowerCaseCharArray(Clone::clone(&str), i, Clone::clone(&locale))?;
                        lowerCharArray = _t2;
                    } else {
                        let _t2: Rc<RefCell<Vec<u16>>> = Character::toChars_i(lowerChar)?;
                        let mut lowerCharArray: Rc<RefCell<Vec<u16>>> = _t2;
                    }
                    let mut mapLen = (lowerCharArray.borrow().len() as i32);
                    if mapLen > 1i32 {
                        let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(((((result.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(1i32))?;
                        let mut result2: Rc<RefCell<Vec<i8>>> = _t2;
                        System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(result2.clone()), 0i32, (resultOffset<<(1i32&0x1f)))?;
                        result = result2;
                    }
                    let mut result2: i32 = 0i32;
                    loop {
                        if result2 >= mapLen { break; }
                        resultOffset = resultOffset.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&result), resultOffset, (lowerCharArray.borrow()[result2 as usize] as i32))?;
                        result2 = result2.wrapping_add(1i32);
                    }
                }
                i = i.wrapping_add(1i32);
            }
            let _t1: String = StringUTF16::newString(Clone::clone(&result), 0i32, resultOffset)?;
            Ok(_t1)
        }

        #[java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut locale: Locale) -> Result<String> {
            if _is_jnull(&locale) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut len = (value.borrow().len() as i32);
            let mut first: i32 = 0i32;
            loop {
                if first >= len { break; }
                let mut cp = ((value.borrow()[first as usize] as i32)&255i32);
                let _t0 = CharacterDataLatin1::instance().toUpperCaseEx(cp)?;
                if cp != _t0 {
                    break;
                }
                first = first.wrapping_add(1i32);
            }
            if first == len {
                return Ok(str);
            }
            let _t0 = locale.getLanguage()?;
            let mut cp: String = _t0;
            if Object::from_any(cp.clone()) == Object::from_any(String::from("lt").clone()) {
                let _t1: String = StringLatin1::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), first, Clone::clone(&locale), (1i32 != 0i32))?;
                return Ok(_t1);
            }
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; len as usize]));
            let mut result: Rc<RefCell<Vec<i8>>> = _arr1;
            System::arraycopy(Object::from_any(value.clone()), 0i32, Object::from_any(result.clone()), 0i32, first)?;
            let mut i: i32 = first;
            loop {
                if i >= len { break; }
                let mut cp = ((value.borrow()[i as usize] as i32)&255i32);
                let _t2 = CharacterDataLatin1::instance().toUpperCaseEx(cp)?;
                cp = _t2;
                let _t3: bool = StringLatin1::canEncode_i(cp)?;
                if !(_t3) {
                    let _t4: String = StringLatin1::toUpperCaseEx(Clone::clone(&str), Clone::clone(&value), first, Clone::clone(&locale), (0i32 != 0i32))?;
                    return Ok(_t4);
                }
                result.borrow_mut()[i as usize] = (((cp) as i8 as i32)) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(String::new_arr_b_b(Clone::clone(&result), ((0i32) as i8))?)
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(mut str: String, mut value: Rc<RefCell<Vec<i8>>>, mut first: i32, mut locale: Locale, mut localeDependent: bool) -> Result<String> {
            let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor((value.borrow().len() as i32))?;
            let mut result: Rc<RefCell<Vec<i8>>> = _t0;
            let mut resultOffset: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= first { break; }
                resultOffset = resultOffset.wrapping_add(1i32);
                StringUTF16::putChar(Clone::clone(&result), resultOffset, ((value.borrow()[i as usize] as i32)&255i32))?;
                i = i.wrapping_add(1i32);
            }
            i = first;
            let mut upperChar: i32 = Default::default();
            loop {
                if i >= (value.borrow().len() as i32) { break; }
                let mut srcChar = ((value.borrow()[i as usize] as i32)&255i32);
                if localeDependent {
                    let _t1: i32 = ConditionalSpecialCasing::toUpperCaseEx(Clone::clone(&str), i, Clone::clone(&locale))?;
                    upperChar = _t1;
                } else {
                    let _t1 = CharacterDataLatin1::instance().toUpperCaseEx(srcChar)?;
                    let mut upperChar: i32 = _t1;
                }
                let _t1: bool = Character::isBmpCodePoint(upperChar)?;
                if _t1 {
                    resultOffset = resultOffset.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&result), resultOffset, upperChar)?;
                } else {
        let mut upperCharArray: Rc<RefCell<Vec<u16>>> = Default::default();
                    if upperChar == -1i32 {
                        if localeDependent {
                            let _t2: Rc<RefCell<Vec<u16>>> = ConditionalSpecialCasing::toUpperCaseCharArray(Clone::clone(&str), i, Clone::clone(&locale))?;
                            upperCharArray = _t2;
                        } else {
                            let _t2 = CharacterDataLatin1::instance().toUpperCaseCharArray(srcChar)?;
                            upperCharArray = _t2;
                        }
                    } else {
                        let _t2: Rc<RefCell<Vec<u16>>> = Character::toChars_i(upperChar)?;
                        upperCharArray = _t2;
                    }
                    let mut mapLen = (upperCharArray.borrow().len() as i32);
                    if mapLen > 1i32 {
                        let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(((((result.borrow().len() as i32)>>((1i32&0x1f)))).wrapping_add(mapLen)).wrapping_sub(1i32))?;
                        let mut result2: Rc<RefCell<Vec<i8>>> = _t2;
                        System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(result2.clone()), 0i32, (resultOffset<<(1i32&0x1f)))?;
                        result = result2;
                    }
                    let mut result2: i32 = 0i32;
                    loop {
                        if result2 >= mapLen { break; }
                        resultOffset = resultOffset.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&result), resultOffset, (upperCharArray.borrow()[result2 as usize] as i32))?;
                        result2 = result2.wrapping_add(1i32);
                    }
                }
                i = i.wrapping_add(1i32);
            }
            let _t1: String = StringUTF16::newString(Clone::clone(&result), 0i32, resultOffset)?;
            Ok(_t1)
        }

        #[java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trim(mut value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            let mut len = (value.borrow().len() as i32);
            let mut st: i32 = 0i32;
            loop {
                if st >= len { break; }
                if ((value.borrow()[st as usize] as i32)&255i32) <= 32i32 {
                    st = st.wrapping_add(1i32);
                    continue;
                }
                break;
            }
            loop {
                if st >= len { break; }
                if ((value.borrow()[(len).wrapping_sub(1i32) as usize] as i32)&255i32) <= 32i32 {
                    len = len.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut _merged1: String;
            if len < (value.borrow().len() as i32) {
                let _t0: String = StringLatin1::newString(Clone::clone(&value), st, (len).wrapping_sub(st))?;
                _merged1 = _t0;
            } else {
                _merged1 = Default::default();
            }
            Ok(_merged1)
        }

        #[java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfNonWhitespace(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut length = (value.borrow().len() as i32);
            let mut left: i32 = 0i32;
            loop {
                if left >= length { break; }
                let _t0: u16 = StringLatin1::getChar(Clone::clone(&value), left)?;
                let mut ch: u16 = _t0;
                if (ch as i32) != 32i32 {
                    if (ch as i32) != 9i32 {
                        let _t1 = CharacterDataLatin1::instance().isWhitespace((ch as i32))?;
                        if !(_t1) {
                            break;
                        }
                    } else {
                        left = left.wrapping_add(1i32);
                        continue;
                    }
                } else {
                    left = left.wrapping_add(1i32);
                    continue;
                }
            }
            Ok(left)
        }

        #[java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOfNonWhitespace(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut length = (value.borrow().len() as i32);
            let mut right: i32 = length;
            loop {
                if 0i32 >= right { break; }
                let _t0: u16 = StringLatin1::getChar(Clone::clone(&value), (right).wrapping_sub(1i32))?;
                let mut ch: u16 = _t0;
                if (ch as i32) != 32i32 {
                    if (ch as i32) != 9i32 {
                        let _t1 = CharacterDataLatin1::instance().isWhitespace((ch as i32))?;
                        if !(_t1) {
                            break;
                        }
                    } else {
                        right = right.wrapping_sub(1i32);
                        continue;
                    }
                } else {
                    right = right.wrapping_sub(1i32);
                    continue;
                }
            }
            Ok(right)
        }

        #[java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn strip(mut value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            let _t0: i32 = StringLatin1::indexOfNonWhitespace(Clone::clone(&value))?;
            let mut left: i32 = _t0;
            if left == (value.borrow().len() as i32) {
                return Ok(String::from(""));
            }
            let _t1: i32 = StringLatin1::lastIndexOfNonWhitespace(Clone::clone(&value))?;
            let mut right: i32 = _t1;
            let mut ifChanged = (right < (value.borrow().len() as i32)) as i32;
            let mut _merged3: String;
            if (ifChanged!=0) {
                let _t2: String = StringLatin1::newString(Clone::clone(&value), left, (right).wrapping_sub(left))?;
                _merged3 = _t2;
            } else {
                _merged3 = Default::default();
            }
            Ok(_merged3)
        }

        #[java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripLeading(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            panic!("stub: java/lang/StringLatin1.stripLeading:([B)Ljava/lang/String;")
        }

        #[java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripTrailing(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            panic!("stub: java/lang/StringLatin1.stripTrailing:([B)Ljava/lang/String;")
        }

        #[java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([B)Ljava/util/stream/Stream<Ljava/lang/String;>;")]
        pub fn lines(value: Rc<RefCell<Vec<i8>>>) -> Result<Object> {
            panic!("stub: java/lang/StringLatin1.lines:([B)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "putChar", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar(val: Rc<RefCell<Vec<i8>>>, index: i32, c: i32) -> Result<()> {
            panic!("stub: java/lang/StringLatin1.putChar:([BII)V")
        }

        #[java_method(name = "getChar", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<u16> {
            Ok(((((((val.borrow()[index as usize] as i32)&255i32)) as u16 as i32)) as u16))
        }

        #[java_method(name = "toBytes", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBytes_arr_i_i_i(val: Rc<RefCell<Vec<i32>>>, off: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/StringLatin1.toBytes:([III)[B")
        }

        #[java_method(name = "toBytes", descriptor = "(C)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBytes(C)[B
        pub fn toBytes_c(mut c: u16) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; 1i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = (((c) as i8 as i32)) as i8;
            Ok(_arr0)
        }

        #[java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newString(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut len: i32) -> Result<String> {
            if (len==0) {
                return Ok(String::from(""));
            }
            let _t0: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), index, (index).wrapping_add(len))?;
            Ok(String::new_arr_b_b(Clone::clone(&_t0), ((0i32) as i8))?)
        }

        #[java_method(name = "fillNull", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fillNull(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32, mut end: i32) -> Result<()> {
            Arrays::fill_arr_b_i_i_b(Clone::clone(&val), index, end, ((0i32) as i8))?;
            Ok(())
        }

        #[java_method(name = "inflate", descriptor = "([BI[CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: inflate([BI[CII)V
        pub fn inflate_arr_b_i_arr_c_i_i(mut src: Rc<RefCell<Vec<i8>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<u16>>>, mut dstOff: i32, mut len: i32) -> Result<()> {
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                dstOff = dstOff.wrapping_add(1i32);
                srcOff = srcOff.wrapping_add(1i32);
                dst.borrow_mut()[dstOff as usize] = (((((src.borrow()[srcOff as usize] as i32)&255i32)) as u16 as i32)) as u16;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: inflate([BI[BII)V
        pub fn inflate_arr_b_i_arr_b_i_i(mut src: Rc<RefCell<Vec<i8>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32, mut len: i32) -> Result<()> {
            StringUTF16::inflate(Clone::clone(&src), srcOff, Clone::clone(&dst), dstOff, len)?;
            Ok(())
        }
    }
}
