#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::util::ArraysSupport;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/StringUTF16",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "StringUTF16.java",
    inner_classes     = "java/lang/StringUTF16$LinesSpliterator:java/lang/StringUTF16:LinesSpliterator:26;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8",
    all_supertypes    = "java/lang/Object;java/lang/StringUTF16",
)]
#[derive(Clone, Default, PartialEq)]
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

    #[java_rta_macros::java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/StringUTF16.<init>:()V")
    }

    #[java_rta_macros::java_method(name = "newBytesFor", descriptor = "(I)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn newBytesFor(mut len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        let _t0: i32 = StringUTF16::newBytesLength(len)?;
        let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; _t0 as usize]));
        Ok(_arr1)
    }

    #[java_rta_macros::java_method(name = "newBytesLength", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "putChar", descriptor = "([BII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "getChar", descriptor = "([BI)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getChar(mut val: Rc<RefCell<Vec<i8>>>, mut index: i32) -> Result<u16> {
        let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
        if index >= _t0 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        index = (index<<(1i32&0x1f));
        index = index.wrapping_add(1i32);
        Ok(((((((((val.borrow()[index as usize] as i32)&255i32)<<(StringUTF16::HI_BYTE_SHIFT()&0x1f))|(((val.borrow()[index as usize] as i32)&255i32)<<(StringUTF16::LO_BYTE_SHIFT()&0x1f)))) as u16 as i32)) as u16))
    }

    #[java_rta_macros::java_method(name = "length", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn length(mut value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        Ok(((value.borrow().len() as i32)>>((1i32&0x1f))))
    }

    #[java_rta_macros::java_method(name = "codePointAt", descriptor = "([BIIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointAt_arr_b_i_i_z(value: Rc<RefCell<Vec<i8>>>, index: i32, end: i32, checked: bool) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointAt:([BIIZ)I")
    }

    #[java_rta_macros::java_method(name = "codePointAt", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointAt_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointAt:([BII)I")
    }

    #[java_rta_macros::java_method(name = "codePointBefore", descriptor = "([BIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointBefore_arr_b_i_z(value: Rc<RefCell<Vec<i8>>>, index: i32, checked: bool) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointBefore:([BIZ)I")
    }

    #[java_rta_macros::java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointBefore_arr_b_i(value: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointBefore:([BI)I")
    }

    #[java_rta_macros::java_method(name = "codePointCount", descriptor = "([BIIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointCount_arr_b_i_i_z(value: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32, checked: bool) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointCount:([BIIZ)I")
    }

    #[java_rta_macros::java_method(name = "codePointCount", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointCount_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointCount:([BII)I")
    }

    #[java_rta_macros::java_method(name = "toChars", descriptor = "([B)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toChars(value: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<u16>>>> {
        panic!("stub: java/lang/StringUTF16.toChars:([B)[C")
    }

    #[java_rta_macros::java_method(name = "toBytes", descriptor = "([CII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toBytes_arr_c_i_i(value: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.toBytes:([CII)[B")
    }

    #[java_rta_macros::java_method(name = "coderFromArrayLen", descriptor = "([BI)B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn coderFromArrayLen(mut value: Rc<RefCell<Vec<i8>>>, mut len: i32) -> Result<i8> {
        Ok((((((((len).wrapping_sub((value.borrow().len() as i32)) as u32>>(31i32&0x1f)) as i32)) as i8 as i32)) as i8))
    }

    #[java_rta_macros::java_method(name = "compress", descriptor = "([CII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compress_arr_c_i_i(val: Rc<RefCell<Vec<u16>>>, off: i32, count: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.compress:([CII)[B")
    }

    #[java_rta_macros::java_method(name = "compress", descriptor = "([BII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "compress", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compress_arr_i_i_i(val: Rc<RefCell<Vec<i32>>>, off: i32, count: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.compress:([III)[B")
    }

    #[java_rta_macros::java_method(name = "extractCodepoints", descriptor = "([III[BI)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn extractCodepoints(val: Rc<RefCell<Vec<i32>>>, off: i32, end: i32, dst: Rc<RefCell<Vec<i8>>>, dstOff: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.extractCodepoints:([III[BI)[B")
    }

    #[java_rta_macros::java_method(name = "computeCodePointSize", descriptor = "([III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn computeCodePointSize(val: Rc<RefCell<Vec<i32>>>, off: i32, end: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.computeCodePointSize:([III)I")
    }

    #[java_rta_macros::java_method(name = "compress", descriptor = "([CI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compress_arr_c_i_arr_b_i_i(src: Rc<RefCell<Vec<u16>>>, srcOff: i32, dst: Rc<RefCell<Vec<i8>>>, dstOff: i32, len: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compress:([CI[BII)I")
    }

    #[java_rta_macros::java_method(name = "compress", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "toBytes", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toBytes_arr_i_i_i(val: Rc<RefCell<Vec<i32>>>, index: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.toBytes:([III)[B")
    }

    #[java_rta_macros::java_method(name = "toBytes", descriptor = "(C)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toBytes_c(c: u16) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.toBytes:(C)[B")
    }

    #[java_rta_macros::java_method(name = "toBytesSupplementary", descriptor = "(I)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toBytesSupplementary(cp: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringUTF16.toBytesSupplementary:(I)[B")
    }

    #[java_rta_macros::java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getChars_arr_b_i_i_arr_c_i(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<u16>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.getChars:([BII[CI)V")
    }

    #[java_rta_macros::java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getBytes(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.getBytes:([BII[BI)V")
    }

    #[java_rta_macros::java_method(name = "equals", descriptor = "([B[B)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn equals(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
        panic!("stub: java/lang/StringUTF16.equals:([B[B)Z")
    }

    #[java_rta_macros::java_method(name = "compareTo", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareTo_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareTo:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareTo_arr_b_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareTo:([B[BII)I")
    }

    #[java_rta_macros::java_method(name = "compareValues", descriptor = "([B[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareValues(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareValues:([B[BII)I")
    }

    #[java_rta_macros::java_method(name = "compareToLatin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareToLatin1_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareToLatin1:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "compareToLatin1", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareToLatin1_arr_b_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareToLatin1:([B[BII)I")
    }

    #[java_rta_macros::java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareToCI(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareToCI:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "compareToCIImpl", descriptor = "([BII[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareToCIImpl(value: Rc<RefCell<Vec<i8>>>, toffset: i32, tlen: i32, other: Rc<RefCell<Vec<i8>>>, ooffset: i32, olen: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareToCIImpl:([BII[BII)I")
    }

    #[java_rta_macros::java_method(name = "compareCodePointCI", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareCodePointCI(cp1: i32, cp2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareCodePointCI:(II)I")
    }

    #[java_rta_macros::java_method(name = "codePointIncluding", descriptor = "([BIIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointIncluding(ba: Rc<RefCell<Vec<i8>>>, cp: i32, index: i32, start: i32, end: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointIncluding:([BIIII)I")
    }

    #[java_rta_macros::java_method(name = "compareToCI_Latin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareToCI_Latin1(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.compareToCI_Latin1:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "hashCode", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn hashCode(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.hashCode:([B)I")
    }

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "([BIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOf_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOf:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOf_arr_b_i_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, valueCount: i32, str: Rc<RefCell<Vec<i8>>>, strCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOf:([BI[BII)I")
    }

    #[java_rta_macros::java_method(name = "indexOfUnsafe", descriptor = "([BI[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfUnsafe(value: Rc<RefCell<Vec<i8>>>, valueCount: i32, str: Rc<RefCell<Vec<i8>>>, strCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOfUnsafe:([BI[BII)I")
    }

    #[java_rta_macros::java_method(name = "indexOfLatin1", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfLatin1_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOfLatin1:([B[B)I")
    }

    #[java_rta_macros::java_method(name = "indexOfLatin1", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfLatin1_arr_b_i_arr_b_i_i(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOfLatin1:([BI[BII)I")
    }

    #[java_rta_macros::java_method(name = "indexOfLatin1Unsafe", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfLatin1Unsafe(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOfLatin1Unsafe:([BI[BII)I")
    }

    #[java_rta_macros::java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfChar(mut value: Rc<RefCell<Vec<i8>>>, mut ch: i32, mut fromIndex: i32, mut max: i32) -> Result<i32> {
        StringUTF16::checkBoundsBeginEnd(fromIndex, max, Clone::clone(&value))?;
        let _t0: i32 = StringUTF16::indexOfCharUnsafe(Clone::clone(&value), ch, fromIndex, max)?;
        Ok(_t0)
    }

    #[java_rta_macros::java_method(name = "indexOfCharUnsafe", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "indexOfSupplementary", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOf_arr_b_i_arr_b_i_i(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.lastIndexOf:([BI[BII)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOf_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, ch: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.lastIndexOf:([BII)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOfSupplementary", descriptor = "([BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOfSupplementary(value: Rc<RefCell<Vec<i8>>>, ch: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.lastIndexOfSupplementary:([BII)I")
    }

    #[java_rta_macros::java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn replace_arr_b_c_c(value: Rc<RefCell<Vec<i8>>>, oldChar: u16, newChar: u16) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.replace:([BCC)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "replace", descriptor = "([BIZ[BIZ[BIZ)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn replace_arr_b_i_z_arr_b_i_z_arr_b_i_z(value: Rc<RefCell<Vec<i8>>>, valLen: i32, valLat1: bool, targ: Rc<RefCell<Vec<i8>>>, targLen: i32, targLat1: bool, repl: Rc<RefCell<Vec<i8>>>, replLen: i32, replLat1: bool) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.replace:([BIZ[BIZ[BIZ)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn regionMatchesCI(value: Rc<RefCell<Vec<i8>>>, toffset: i32, other: Rc<RefCell<Vec<i8>>>, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringUTF16.regionMatchesCI:([BI[BII)Z")
    }

    #[java_rta_macros::java_method(name = "regionMatchesCI_Latin1", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn regionMatchesCI_Latin1(value: Rc<RefCell<Vec<i8>>>, toffset: i32, other: Rc<RefCell<Vec<i8>>>, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringUTF16.regionMatchesCI_Latin1:([BI[BII)Z")
    }

    #[java_rta_macros::java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toLowerCase(str: String, value: Rc<RefCell<Vec<i8>>>, locale: Object) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.toLowerCase:(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toLowerCaseEx(str: String, value: Rc<RefCell<Vec<i8>>>, result: Rc<RefCell<Vec<i8>>>, first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.toLowerCaseEx:(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toUpperCase(str: String, value: Rc<RefCell<Vec<i8>>>, locale: Object) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.toUpperCase:(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toUpperCaseEx(str: String, value: Rc<RefCell<Vec<i8>>>, result: Rc<RefCell<Vec<i8>>>, first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.toUpperCaseEx:(Ljava/lang/String;[B[BILjava/util/Locale;Z)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn trim(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.trim:([B)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOfNonWhitespace(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.indexOfNonWhitespace:([B)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOfNonWhitespace(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.lastIndexOfNonWhitespace:([B)I")
    }

    #[java_rta_macros::java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn strip(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.strip:([B)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn stripLeading(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.stripLeading:([B)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn stripTrailing(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.stripTrailing:([B)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([B)Ljava/util/stream/Stream<Ljava/lang/String;>;")]
    pub fn lines(value: Rc<RefCell<Vec<i8>>>) -> Result<Object> {
        panic!("stub: java/lang/StringUTF16.lines:([B)Ljava/util/stream/Stream;")
    }

    #[java_rta_macros::java_method(name = "putChars", descriptor = "([BI[CII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putChars(val: Rc<RefCell<Vec<i8>>>, index: i32, str: Rc<RefCell<Vec<u16>>>, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.putChars:([BI[CII)V")
    }

    #[java_rta_macros::java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn newString(val: Rc<RefCell<Vec<i8>>>, index: i32, len: i32) -> Result<String> {
        panic!("stub: java/lang/StringUTF16.newString:([BII)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "fillNull", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn fillNull(val: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.fillNull:([BII)V")
    }

    #[java_rta_macros::java_method(name = "putCharSB", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharSB(val: Rc<RefCell<Vec<i8>>>, index: i32, c: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.putCharSB:([BII)V")
    }

    #[java_rta_macros::java_method(name = "putCharsSB", descriptor = "([BI[CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharsSB_arr_b_i_arr_c_i_i(val: Rc<RefCell<Vec<i8>>>, index: i32, ca: Rc<RefCell<Vec<u16>>>, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.putCharsSB:([BI[CII)V")
    }

    #[java_rta_macros::java_method(name = "putCharsSB", descriptor = "([BILjava/lang/CharSequence;II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharsSB_arr_b_i_seq_i_i(val: Rc<RefCell<Vec<i8>>>, index: i32, s: Object, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.putCharsSB:([BILjava/lang/CharSequence;II)V")
    }

    #[java_rta_macros::java_method(name = "codePointAtSB", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointAtSB(val: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointAtSB:([BII)I")
    }

    #[java_rta_macros::java_method(name = "codePointBeforeSB", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointBeforeSB(val: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointBeforeSB:([BI)I")
    }

    #[java_rta_macros::java_method(name = "codePointCountSB", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointCountSB(val: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.codePointCountSB:([BII)I")
    }

    #[java_rta_macros::java_method(name = "getChars", descriptor = "(III[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "getChars", descriptor = "(JII[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getChars_l_i_i_arr_b(l: i64, arg1: i32, begin: i32, end: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.getChars:(JII[B)I")
    }

    #[java_rta_macros::java_method(name = "contentEquals", descriptor = "([B[BI)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn contentEquals_arr_b_arr_b_i(v1: Rc<RefCell<Vec<i8>>>, v2: Rc<RefCell<Vec<i8>>>, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringUTF16.contentEquals:([B[BI)Z")
    }

    #[java_rta_macros::java_method(name = "contentEquals", descriptor = "([BLjava/lang/CharSequence;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn contentEquals_arr_b_seq_i(value: Rc<RefCell<Vec<i8>>>, cs: Object, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringUTF16.contentEquals:([BLjava/lang/CharSequence;I)Z")
    }

    #[java_rta_macros::java_method(name = "putCharsAt", descriptor = "([BICCCC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "putCharsAt", descriptor = "([BICCCCC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharsAt_arr_b_i_c_c_c_c_c(value: Rc<RefCell<Vec<i8>>>, i: i32, c1: u16, c2: u16, c3: u16, c4: u16, c5: u16) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.putCharsAt:([BICCCCC)I")
    }

    #[java_rta_macros::java_method(name = "charAt", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn charAt(value: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<u16> {
        panic!("stub: java/lang/StringUTF16.charAt:([BI)C")
    }

    #[java_rta_macros::java_method(name = "reverse", descriptor = "([BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn reverse(val: Rc<RefCell<Vec<i8>>>, count: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.reverse:([BI)V")
    }

    #[java_rta_macros::java_method(name = "reverseAllValidSurrogatePairs", descriptor = "([BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn reverseAllValidSurrogatePairs(val: Rc<RefCell<Vec<i8>>>, count: i32) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.reverseAllValidSurrogatePairs:([BI)V")
    }

    #[java_rta_macros::java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "lastIndexOfLatin1", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOfLatin1(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.lastIndexOfLatin1:([BI[BII)I")
    }

    #[cfg_attr(any(), java_native(name = "isBigEndian", descriptor = "()Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isBigEndian() -> Result<bool> {
        panic!("native: java/lang/StringUTF16.isBigEndian:()Z")
    }

    #[java_rta_macros::java_method(name = "getChars", descriptor = "(II[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
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

    #[java_rta_macros::java_method(name = "getChars", descriptor = "(JI[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getChars_l_i_arr_b(i: i64, arg1: i32, index: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringUTF16.getChars:(JI[B)I")
    }

    #[java_rta_macros::java_method(name = "checkIndex", descriptor = "(I[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkIndex(off: i32, val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.checkIndex:(I[B)V")
    }

    #[java_rta_macros::java_method(name = "checkOffset", descriptor = "(I[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkOffset(off: i32, val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/lang/StringUTF16.checkOffset:(I[B)V")
    }

    #[java_rta_macros::java_method(name = "checkBoundsBeginEnd", descriptor = "(II[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkBoundsBeginEnd(mut begin: i32, mut end: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
        String::checkBoundsBeginEnd(begin, end, _t0)?;
        Ok(())
    }

    #[java_rta_macros::java_method(name = "checkBoundsOffCount", descriptor = "(II[B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkBoundsOffCount(mut offset: i32, mut count: i32, mut val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        let _t0: i32 = StringUTF16::length(Clone::clone(&val))?;
        let _t1: i32 = String::checkBoundsOffCount(offset, count, _t0)?;
        Ok(())
    }
}
