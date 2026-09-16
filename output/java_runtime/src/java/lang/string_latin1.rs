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
    binary_name       = "java/lang/StringLatin1",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "StringLatin1.java",
    inner_classes     = "java/lang/StringLatin1$LinesSpliterator:java/lang/StringLatin1:LinesSpliterator:26;java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8",
    all_supertypes    = "java/lang/Object;java/lang/StringLatin1",
)]
#[derive(Clone, Default, PartialEq)]
pub struct StringLatin1;

impl StringLatin1 {
    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/StringLatin1.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn charAt(value: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<u16> {
        panic!("stub: java/lang/StringLatin1.charAt:([BI)C")
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn canEncode_c(cp: u16) -> Result<bool> {
        panic!("stub: java/lang/StringLatin1.canEncode:(C)Z")
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: canEncode(I)Z
    pub fn canEncode_i(mut cp: i32) -> Result<bool> {
        Ok((if (cp>=0) { cp <= 255i32 } else { (0i32 != 0) }))
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn length(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.length:([B)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAt(value: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.codePointAt:([BII)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "([BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBefore(value: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.codePointBefore:([BI)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointCount(value: Rc<RefCell<Vec<i8>>>, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.codePointCount:([BII)I")
    }

    #[cfg_attr(any(), java_method(name = "toChars", descriptor = "([B)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toChars(value: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<u16>>>> {
        panic!("stub: java/lang/StringLatin1.toChars:([B)[C")
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn inflate_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringLatin1.inflate:([BII)[B")
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "([BII[CI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getChars(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<u16>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/StringLatin1.getChars:([BII[CI)V")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "([BII[BI)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytes(value: Rc<RefCell<Vec<i8>>>, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/StringLatin1.getBytes:([BII[BI)V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "([B[B)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
        panic!("stub: java/lang/StringLatin1.equals:([B[B)Z")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareTo:([B[B)I")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo_arr_b_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareTo:([B[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToUTF16_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareToUTF16:([B[B)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16", descriptor = "([B[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToUTF16_arr_b_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareToUTF16:([B[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToUTF16Values", descriptor = "([B[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToUTF16Values(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>, len1: i32, len2: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareToUTF16Values:([B[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToCI", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToCI(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareToCI:([B[B)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToCI_UTF16", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToCI_UTF16(value: Rc<RefCell<Vec<i8>>>, other: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.compareToCI_UTF16:([B[B)I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.hashCode:([B)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "indexOfChar", descriptor = "([BIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_arr_b_arr_b(value: Rc<RefCell<Vec<i8>>>, str: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.indexOf:([B[B)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_arr_b_i_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, valueCount: i32, str: Rc<RefCell<Vec<i8>>>, strCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.indexOf:([BI[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_arr_b_i_arr_b_i_i(src: Rc<RefCell<Vec<i8>>>, srcCount: i32, tgt: Rc<RefCell<Vec<i8>>>, tgtCount: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.lastIndexOf:([BI[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_arr_b_i_i(value: Rc<RefCell<Vec<i8>>>, ch: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.lastIndexOf:([BII)I")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BCC)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replace_arr_b_c_c(value: Rc<RefCell<Vec<i8>>>, oldChar: u16, newChar: u16) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.replace:([BCC)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "([BI[BI[BI)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replace_arr_b_i_arr_b_i_arr_b_i(value: Rc<RefCell<Vec<i8>>>, valLen: i32, targ: Rc<RefCell<Vec<i8>>>, targLen: i32, repl: Rc<RefCell<Vec<i8>>>, replLen: i32) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.replace:([BI[BI[BI)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn regionMatchesCI(value: Rc<RefCell<Vec<i8>>>, toffset: i32, other: Rc<RefCell<Vec<i8>>>, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringLatin1.regionMatchesCI:([BI[BII)Z")
    }

    #[cfg_attr(any(), java_method(name = "regionMatchesCI_UTF16", descriptor = "([BI[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn regionMatchesCI_UTF16(value: Rc<RefCell<Vec<i8>>>, toffset: i32, other: Rc<RefCell<Vec<i8>>>, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/StringLatin1.regionMatchesCI_UTF16:([BI[BII)Z")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCase(str: String, value: Rc<RefCell<Vec<i8>>>, locale: Object) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.toLowerCase:(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCaseEx(str: String, value: Rc<RefCell<Vec<i8>>>, first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.toLowerCaseEx:(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCase(str: String, value: Rc<RefCell<Vec<i8>>>, locale: Object) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.toUpperCase:(Ljava/lang/String;[BLjava/util/Locale;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCaseEx", descriptor = "(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCaseEx(str: String, value: Rc<RefCell<Vec<i8>>>, first: i32, locale: Object, localeDependent: bool) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.toUpperCaseEx:(Ljava/lang/String;[BILjava/util/Locale;Z)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "trim", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn trim(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.trim:([B)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "indexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOfNonWhitespace(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.indexOfNonWhitespace:([B)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfNonWhitespace", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOfNonWhitespace(value: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
        panic!("stub: java/lang/StringLatin1.lastIndexOfNonWhitespace:([B)I")
    }

    #[cfg_attr(any(), java_method(name = "strip", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn strip(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.strip:([B)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "stripLeading", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stripLeading(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.stripLeading:([B)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "stripTrailing", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stripTrailing(value: Rc<RefCell<Vec<i8>>>) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.stripTrailing:([B)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "lines", descriptor = "([B)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([B)Ljava/util/stream/Stream<Ljava/lang/String;>;"))]
    pub fn lines(value: Rc<RefCell<Vec<i8>>>) -> Result<Object> {
        panic!("stub: java/lang/StringLatin1.lines:([B)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "putChar", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn putChar(val: Rc<RefCell<Vec<i8>>>, index: i32, c: i32) -> Result<()> {
        panic!("stub: java/lang/StringLatin1.putChar:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "getChar", descriptor = "([BI)C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getChar(val: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<u16> {
        panic!("stub: java/lang/StringLatin1.getChar:([BI)C")
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "([III)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toBytes_arr_i_i_i(val: Rc<RefCell<Vec<i32>>>, off: i32, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringLatin1.toBytes:([III)[B")
    }

    #[cfg_attr(any(), java_method(name = "toBytes", descriptor = "(C)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toBytes_c(c: u16) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/StringLatin1.toBytes:(C)[B")
    }

    #[cfg_attr(any(), java_method(name = "newString", descriptor = "([BII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newString(val: Rc<RefCell<Vec<i8>>>, index: i32, len: i32) -> Result<String> {
        panic!("stub: java/lang/StringLatin1.newString:([BII)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "fillNull", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn fillNull(val: Rc<RefCell<Vec<i8>>>, index: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/StringLatin1.fillNull:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BI[CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn inflate_arr_b_i_arr_c_i_i(src: Rc<RefCell<Vec<i8>>>, srcOff: i32, dst: Rc<RefCell<Vec<u16>>>, dstOff: i32, len: i32) -> Result<()> {
        panic!("stub: java/lang/StringLatin1.inflate:([BI[CII)V")
    }

    #[cfg_attr(any(), java_method(name = "inflate", descriptor = "([BI[BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: inflate([BI[BII)V
    pub fn inflate_arr_b_i_arr_b_i_i(mut src: Rc<RefCell<Vec<i8>>>, mut srcOff: i32, mut dst: Rc<RefCell<Vec<i8>>>, mut dstOff: i32, mut len: i32) -> Result<()> {
        StringUTF16::inflate(Clone::clone(&src), srcOff, Clone::clone(&dst), dstOff, len)?;
        Ok(())
    }
}
