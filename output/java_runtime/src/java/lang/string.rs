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
use crate::jdk::internal::util::Preconditions;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/String",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Serializable,java/lang/Comparable,java/lang/CharSequence,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/String;>;Ljava/lang/CharSequence;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "String.java",
    inner_classes     = "java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25;java/lang/String$CaseInsensitiveComparator:java/lang/String:CaseInsensitiveComparator:10",
    all_supertypes    = "java/io/Serializable;java/lang/CharSequence;java/lang/Comparable;java/lang/Object;java/lang/String;java/lang/constant/Constable;java/lang/constant/ConstantDesc",
)]
#[derive(Clone, Default, PartialEq)]
pub struct String {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
    pub value: JField<Rc<RefCell<Vec<i8>>>>,
    #[cfg_attr(any(), java_field(name = "coder", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
    pub coder: JField<i8>,
    #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub hash: JField<i32>,
    #[cfg_attr(any(), java_field(name = "hashIsZero", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
    pub hashIsZero: JField<bool>,
}

impl String {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-6849794470754667710"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -6849794470754667710i64
    }

    #[cfg_attr(any(), java_field(name = "COMPACT_STRINGS", descriptor = "Z", access = "package", modifiers = "static final", is_static = true))]
    // static field: COMPACT_STRINGS:Z
    pub fn COMPACT_STRINGS() -> bool {
        true
    }

    #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
    // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
    pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "REPL", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "65533"))]
    // static field: REPL:C
    pub fn REPL() -> u16 {
        65533
    }

    #[cfg_attr(any(), java_field(name = "CASE_INSENSITIVE_ORDER", descriptor = "Ljava/util/Comparator;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Comparator<Ljava/lang/String;>;"))]
    // static field: CASE_INSENSITIVE_ORDER:Ljava/util/Comparator;
    pub fn CASE_INSENSITIVE_ORDER() -> Object {
        panic!("stub: java/lang/String.CASE_INSENSITIVE_ORDER:Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_field(name = "LATIN1", descriptor = "B", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
    // static field: LATIN1:B
    pub fn LATIN1() -> i8 {
        0
    }

    #[cfg_attr(any(), java_field(name = "UTF16", descriptor = "B", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
    // static field: UTF16:B
    pub fn UTF16() -> i8 {
        1
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { value: JField::new(Default::default()), coder: JField::new(Default::default()), hash: JField::new(0), hashIsZero: JField::new(false), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.value.set(Clone::clone(&String::from("").value.get()));
        this.coder.set(String::from("").coder.get());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(original: String) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_c(value: Rc<RefCell<Vec<u16>>>) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_c_i_i(value: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "rangeCheck", descriptor = "([CII)Ljava/lang/Void;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rangeCheck(value: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<Object> {
        panic!("stub: java/lang/String.rangeCheck:([CII)Ljava/lang/Void;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_i_i_i(codePoints: Rc<RefCell<Vec<i32>>>, offset: i32, count: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([III)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn new_arr_b_i_i_i(ascii: Rc<RefCell<Vec<i8>>>, hibyte: i32, offset: i32, count: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BIII)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn new_arr_b_i(ascii: Rc<RefCell<Vec<i8>>>, hibyte: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BI)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BIILjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn new_arr_b_i_i_str(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, charsetName: String) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BIILjava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BIILjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_b_i_i_charse(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, charset: Object) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BIILjava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;[BII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_charse_arr_b_i_i(charset: Object, bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V")
    }

    #[cfg_attr(any(), java_method(name = "newStringUTF8NoRepl", descriptor = "([BIIZ)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newStringUTF8NoRepl(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, noShare: bool) -> Result<String> {
        panic!("stub: java/lang/String.newStringUTF8NoRepl:([BIIZ)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "newStringNoRepl", descriptor = "([BLjava/nio/charset/Charset;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException"))]
    pub fn newStringNoRepl(src: Rc<RefCell<Vec<i8>>>, cs: Object) -> Result<String> {
        panic!("stub: java/lang/String.newStringNoRepl:([BLjava/nio/charset/Charset;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "newStringNoRepl1", descriptor = "([BLjava/nio/charset/Charset;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newStringNoRepl1(src: Rc<RefCell<Vec<i8>>>, cs: Object) -> Result<String> {
        panic!("stub: java/lang/String.newStringNoRepl1:([BLjava/nio/charset/Charset;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "safeTrim", descriptor = "([BIZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn safeTrim(ba: Rc<RefCell<Vec<i8>>>, len: i32, isTrusted: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.safeTrim:([BIZ)[B")
    }

    #[cfg_attr(any(), java_method(name = "scale", descriptor = "(IF)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn scale(len: i32, expansionFactor: f32) -> Result<i32> {
        panic!("stub: java/lang/String.scale:(IF)I")
    }

    #[cfg_attr(any(), java_method(name = "lookupCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn lookupCharset(csn: String) -> Result<Object> {
        panic!("stub: java/lang/String.lookupCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
    }

    #[cfg_attr(any(), java_method(name = "encode", descriptor = "(Ljava/nio/charset/Charset;B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encode(cs: Object, coder: i8, val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encode:(Ljava/nio/charset/Charset;B[B)[B")
    }

    #[cfg_attr(any(), java_method(name = "encodeWithEncoder", descriptor = "(Ljava/nio/charset/Charset;B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encodeWithEncoder(cs: Object, coder: i8, val: Rc<RefCell<Vec<i8>>>, doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encodeWithEncoder:(Ljava/nio/charset/Charset;B[BZ)[B")
    }

    #[cfg_attr(any(), java_method(name = "getBytesUTF8NoRepl", descriptor = "(Ljava/lang/String;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytesUTF8NoRepl(s: String) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytesUTF8NoRepl:(Ljava/lang/String;)[B")
    }

    #[cfg_attr(any(), java_method(name = "isASCII", descriptor = "([B)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isASCII(src: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
        panic!("stub: java/lang/String.isASCII:([B)Z")
    }

    #[cfg_attr(any(), java_method(name = "getBytesNoRepl", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException"))]
    pub fn getBytesNoRepl(s: String, cs: Object) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytesNoRepl:(Ljava/lang/String;Ljava/nio/charset/Charset;)[B")
    }

    #[cfg_attr(any(), java_method(name = "getBytesNoRepl1", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytesNoRepl1(s: String, cs: Object) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytesNoRepl1:(Ljava/lang/String;Ljava/nio/charset/Charset;)[B")
    }

    #[cfg_attr(any(), java_method(name = "encodeASCII", descriptor = "(B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encodeASCII(coder: i8, val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encodeASCII:(B[B)[B")
    }

    #[cfg_attr(any(), java_method(name = "replaceNegatives", descriptor = "([BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replaceNegatives(val: Rc<RefCell<Vec<i8>>>, fromIndex: i32) -> Result<()> {
        panic!("stub: java/lang/String.replaceNegatives:([BI)V")
    }

    #[cfg_attr(any(), java_method(name = "encode8859_1", descriptor = "(B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encode8859_1_b_arr_b(coder: i8, val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encode8859_1:(B[B)[B")
    }

    #[cfg_attr(any(), java_method(name = "encode8859_1", descriptor = "(B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encode8859_1_b_arr_b_z(coder: i8, val: Rc<RefCell<Vec<i8>>>, doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encode8859_1:(B[BZ)[B")
    }

    #[cfg_attr(any(), java_method(name = "decodeASCII", descriptor = "([BI[CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decodeASCII(sa: Rc<RefCell<Vec<i8>>>, sp: i32, da: Rc<RefCell<Vec<u16>>>, dp: i32, len: i32) -> Result<i32> {
        panic!("stub: java/lang/String.decodeASCII:([BI[CII)I")
    }

    #[cfg_attr(any(), java_method(name = "isNotContinuation", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isNotContinuation(b: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isNotContinuation:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMalformed3", descriptor = "(III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMalformed3(b1: i32, b2: i32, b3: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isMalformed3:(III)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMalformed3_2", descriptor = "(II)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMalformed3_2(b1: i32, b2: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isMalformed3_2:(II)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMalformed4", descriptor = "(III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMalformed4(b2: i32, b3: i32, b4: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isMalformed4:(III)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMalformed4_2", descriptor = "(II)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMalformed4_2(b1: i32, b2: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isMalformed4_2:(II)Z")
    }

    #[cfg_attr(any(), java_method(name = "isMalformed4_3", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isMalformed4_3(b3: i32) -> Result<bool> {
        panic!("stub: java/lang/String.isMalformed4_3:(I)Z")
    }

    #[cfg_attr(any(), java_method(name = "decode2", descriptor = "(II)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decode2(b1: i32, b2: i32) -> Result<u16> {
        panic!("stub: java/lang/String.decode2:(II)C")
    }

    #[cfg_attr(any(), java_method(name = "decode3", descriptor = "(III)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decode3(b1: i32, b2: i32, b3: i32) -> Result<u16> {
        panic!("stub: java/lang/String.decode3:(III)C")
    }

    #[cfg_attr(any(), java_method(name = "decode4", descriptor = "(IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decode4(b1: i32, b2: i32, b3: i32, b4: i32) -> Result<i32> {
        panic!("stub: java/lang/String.decode4:(IIII)I")
    }

    #[cfg_attr(any(), java_method(name = "decodeUTF8_UTF16", descriptor = "([BII[BIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decodeUTF8_UTF16(src: Rc<RefCell<Vec<i8>>>, sp: i32, sl: i32, dst: Rc<RefCell<Vec<i8>>>, dp: i32, doReplace: bool) -> Result<i32> {
        panic!("stub: java/lang/String.decodeUTF8_UTF16:([BII[BIZ)I")
    }

    #[cfg_attr(any(), java_method(name = "decodeWithDecoder", descriptor = "(Ljava/nio/charset/CharsetDecoder;[C[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException"))]
    pub fn decodeWithDecoder(cd: Object, dst: Rc<RefCell<Vec<u16>>>, src: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<i32> {
        panic!("stub: java/lang/String.decodeWithDecoder:(Ljava/nio/charset/CharsetDecoder;[C[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "malformed3", descriptor = "([BI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn malformed3(src: Rc<RefCell<Vec<i8>>>, sp: i32) -> Result<i32> {
        panic!("stub: java/lang/String.malformed3:([BI)I")
    }

    #[cfg_attr(any(), java_method(name = "malformed4", descriptor = "([BI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn malformed4(src: Rc<RefCell<Vec<i8>>>, sp: i32) -> Result<i32> {
        panic!("stub: java/lang/String.malformed4:([BI)I")
    }

    #[cfg_attr(any(), java_method(name = "throwMalformed", descriptor = "(II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn throwMalformed_i_i(off: i32, nb: i32) -> Result<()> {
        panic!("stub: java/lang/String.throwMalformed:(II)V")
    }

    #[cfg_attr(any(), java_method(name = "throwMalformed", descriptor = "([B)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn throwMalformed_arr_b(val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/lang/String.throwMalformed:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "throwUnmappable", descriptor = "(I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn throwUnmappable_i(off: i32) -> Result<()> {
        panic!("stub: java/lang/String.throwUnmappable:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "throwUnmappable", descriptor = "([B)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn throwUnmappable_arr_b(val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/lang/String.throwUnmappable:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "encodeUTF8", descriptor = "(B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encodeUTF8(coder: i8, val: Rc<RefCell<Vec<i8>>>, doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encodeUTF8:(B[BZ)[B")
    }

    #[cfg_attr(any(), java_method(name = "encodeUTF8_UTF16", descriptor = "([BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encodeUTF8_UTF16(val: Rc<RefCell<Vec<i8>>>, doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.encodeUTF8_UTF16:([BZ)[B")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BLjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn new_arr_b_str(bytes: Rc<RefCell<Vec<i8>>>, charsetName: String) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BLjava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BLjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_b_charse(bytes: Rc<RefCell<Vec<i8>>>, charset: Object) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BLjava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_b_i_i(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_b(bytes: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/StringBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_string(buffer: Object) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:(Ljava/lang/StringBuffer;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/StringBuilder;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/StringBuilder;)V
    pub fn new_sb(mut builder: StringBuilder) -> Result<Self> {
        let mut this = Self { value: JField::new(Default::default()), coder: JField::new(Default::default()), hash: JField::new(0), hashIsZero: JField::new(false), ..Default::default() };
        this = String::new_abstra_void(Clone::clone(&builder).into(), Clone::clone(&Object::default()))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.coder()?;
        Ok(((this.value.get().borrow().len() as i32)>>(((_t0 as i32)&0x1f))))
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/lang/String.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn charAt(&self, index: i32) -> Result<u16> {
        panic!("stub: java/lang/String.charAt:(I)C")
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointAt(&self, index: i32) -> Result<i32> {
        panic!("stub: java/lang/String.codePointAt:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointBefore(&self, index: i32) -> Result<i32> {
        panic!("stub: java/lang/String.codePointBefore:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.codePointCount:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "offsetByCodePoints", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/String.offsetByCodePoints:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(II[CI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<u16>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/String.getChars:(II[CI)V")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "(II[BI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn getBytes_i_i_arr_b_i(&self, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/String.getBytes:(II[BI)V")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "(Ljava/lang/String;)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn getBytes_str(&self, charsetName: String) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytes:(Ljava/lang/String;)[B")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "(Ljava/nio/charset/Charset;)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytes_charse(&self, charset: Object) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytes:(Ljava/nio/charset/Charset;)[B")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "()[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.getBytes:()[B")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, anObject: Object) -> Result<bool> {
        panic!("stub: java/lang/String.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "contentEquals", descriptor = "(Ljava/lang/StringBuffer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contentEquals_string(&self, sb: Object) -> Result<bool> {
        panic!("stub: java/lang/String.contentEquals:(Ljava/lang/StringBuffer;)Z")
    }

    #[cfg_attr(any(), java_method(name = "nonSyncContentEquals", descriptor = "(Ljava/lang/AbstractStringBuilder;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nonSyncContentEquals(&self, sb: AbstractStringBuilder) -> Result<bool> {
        panic!("stub: java/lang/String.nonSyncContentEquals:(Ljava/lang/AbstractStringBuilder;)Z")
    }

    #[cfg_attr(any(), java_method(name = "contentEquals", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contentEquals_seq(&self, cs: Object) -> Result<bool> {
        panic!("stub: java/lang/String.contentEquals:(Ljava/lang/CharSequence;)Z")
    }

    #[cfg_attr(any(), java_method(name = "equalsIgnoreCase", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equalsIgnoreCase(&self, anotherString: String) -> Result<bool> {
        panic!("stub: java/lang/String.equalsIgnoreCase:(Ljava/lang/String;)Z")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo(&self, anotherString: String) -> Result<i32> {
        panic!("stub: java/lang/String.compareTo:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "compareToIgnoreCase", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareToIgnoreCase(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/String.compareToIgnoreCase:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "regionMatches", descriptor = "(ILjava/lang/String;II)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn regionMatches_i_str_i_i(&self, toffset: i32, other: String, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/String.regionMatches:(ILjava/lang/String;II)Z")
    }

    #[cfg_attr(any(), java_method(name = "regionMatches", descriptor = "(ZILjava/lang/String;II)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn regionMatches_z_i_str_i_i(&self, ignoreCase: bool, toffset: i32, other: String, ooffset: i32, len: i32) -> Result<bool> {
        panic!("stub: java/lang/String.regionMatches:(ZILjava/lang/String;II)Z")
    }

    #[cfg_attr(any(), java_method(name = "startsWith", descriptor = "(Ljava/lang/String;I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn startsWith_str_i(&self, prefix: String, toffset: i32) -> Result<bool> {
        panic!("stub: java/lang/String.startsWith:(Ljava/lang/String;I)Z")
    }

    #[cfg_attr(any(), java_method(name = "startsWith", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn startsWith_str(&self, prefix: String) -> Result<bool> {
        panic!("stub: java/lang/String.startsWith:(Ljava/lang/String;)Z")
    }

    #[cfg_attr(any(), java_method(name = "endsWith", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn endsWith(&self, suffix: String) -> Result<bool> {
        panic!("stub: java/lang/String.endsWith:(Ljava/lang/String;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/String.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: indexOf(I)I
    pub fn indexOf_i(&self, mut ch: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.indexOf_i_i(ch, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: indexOf(II)I
    pub fn indexOf_i_i(&self, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let mut _merged3: i32;
        if _t0 {
            let _t1 = this.length()?;
            let _t2: i32 = StringLatin1::indexOf_arr_b_i_i_i(Clone::clone(&this.value.get()), ch, fromIndex, _t1)?;
            _merged3 = _t2;
        } else {
            let _t1 = this.length()?;
            let _t2: i32 = StringUTF16::indexOf_arr_b_i_i_i(Clone::clone(&this.value.get()), ch, fromIndex, _t1)?;
            _merged3 = _t2;
        }
        Ok(_merged3)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(III)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_i_i_i(&self, ch: i32, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.indexOf:(III)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_i(&self, ch: i32) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOf:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_i_i(&self, ch: i32, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOf:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/String.indexOf:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.indexOf:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_str_i_i(&self, str: String, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.indexOf:(Ljava/lang/String;II)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "([BBILjava/lang/String;I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_arr_b_b_i_str_i(src: Rc<RefCell<Vec<i8>>>, srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.indexOf:([BBILjava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOf:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOf:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "([BBILjava/lang/String;I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_arr_b_b_i_str_i(src: Rc<RefCell<Vec<i8>>>, srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOf:([BBILjava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "substring", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn substring_i(&self, beginIndex: i32) -> Result<String> {
        panic!("stub: java/lang/String.substring:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "substring", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn substring_i_i(&self, beginIndex: i32, endIndex: i32) -> Result<String> {
        panic!("stub: java/lang/String.substring:(II)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "subSequence", descriptor = "(II)Ljava/lang/CharSequence;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn subSequence(&self, beginIndex: i32, endIndex: i32) -> Result<Object> {
        panic!("stub: java/lang/String.subSequence:(II)Ljava/lang/CharSequence;")
    }

    #[cfg_attr(any(), java_method(name = "concat", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn concat(&self, str: String) -> Result<String> {
        panic!("stub: java/lang/String.concat:(Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(CC)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replace_c_c(&self, oldChar: u16, newChar: u16) -> Result<String> {
        panic!("stub: java/lang/String.replace:(CC)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "matches", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn matches(&self, regex: String) -> Result<bool> {
        panic!("stub: java/lang/String.matches:(Ljava/lang/String;)Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contains(&self, s: Object) -> Result<bool> {
        panic!("stub: java/lang/String.contains:(Ljava/lang/CharSequence;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replaceFirst", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replaceFirst(&self, regex: String, replacement: String) -> Result<String> {
        panic!("stub: java/lang/String.replaceFirst:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replaceAll(&self, regex: String, replacement: String) -> Result<String> {
        panic!("stub: java/lang/String.replaceAll:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replace_seq_seq(&self, target: Object, replacement: Object) -> Result<String> {
        panic!("stub: java/lang/String.replace:(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/String;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn split_str_i(&self, regex: String, limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/String.split:(Ljava/lang/String;I)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "splitWithDelimiters", descriptor = "(Ljava/lang/String;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn splitWithDelimiters(&self, regex: String, limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/String.splitWithDelimiters:(Ljava/lang/String;I)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/String;IZ)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn split_str_i_z(&self, regex: String, limit: i32, withDelimiters: bool) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/String.split:(Ljava/lang/String;IZ)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(CIZ)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn split_c_i_z(&self, ch: u16, limit: i32, withDelimiters: bool) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/String.split:(CIZ)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn split_str(&self, regex: String) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/String.split:(Ljava/lang/String;)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn join_seq_arr_seq(delimiter: Object, elements: Rc<RefCell<Vec<Object>>>) -> Result<String> {
        panic!("stub: java/lang/String.join:(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn join_str_str_str_arr_str_i(prefix: String, suffix: String, delimiter: String, elements: Rc<RefCell<Vec<String>>>, size: i32) -> Result<String> {
        panic!("stub: java/lang/String.join:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "join", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;Ljava/lang/Iterable<+Ljava/lang/CharSequence;>;)Ljava/lang/String;"))]
    pub fn join_seq_iter(delimiter: Object, elements: Object) -> Result<String> {
        panic!("stub: java/lang/String.join:(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCase_locale(&self, locale: Object) -> Result<String> {
        panic!("stub: java/lang/String.toLowerCase:(Ljava/util/Locale;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toLowerCase", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toLowerCase(&self) -> Result<String> {
        panic!("stub: java/lang/String.toLowerCase:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCase_locale(&self, locale: Object) -> Result<String> {
        panic!("stub: java/lang/String.toUpperCase:(Ljava/util/Locale;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUpperCase", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUpperCase(&self) -> Result<String> {
        panic!("stub: java/lang/String.toUpperCase:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "trim", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn trim(&self) -> Result<String> {
        panic!("stub: java/lang/String.trim:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "strip", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn strip(&self) -> Result<String> {
        panic!("stub: java/lang/String.strip:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "stripLeading", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stripLeading(&self) -> Result<String> {
        panic!("stub: java/lang/String.stripLeading:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "stripTrailing", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stripTrailing(&self) -> Result<String> {
        panic!("stub: java/lang/String.stripTrailing:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "isBlank", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isBlank(&self) -> Result<bool> {
        panic!("stub: java/lang/String.isBlank:()Z")
    }

    #[cfg_attr(any(), java_method(name = "lines", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/lang/String;>;"))]
    pub fn lines(&self) -> Result<Object> {
        panic!("stub: java/lang/String.lines:()Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "indent", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indent(&self, n: i32) -> Result<String> {
        panic!("stub: java/lang/String.indent:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "indexOfNonWhitespace", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOfNonWhitespace(&self) -> Result<i32> {
        panic!("stub: java/lang/String.indexOfNonWhitespace:()I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOfNonWhitespace", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOfNonWhitespace(&self) -> Result<i32> {
        panic!("stub: java/lang/String.lastIndexOfNonWhitespace:()I")
    }

    #[cfg_attr(any(), java_method(name = "stripIndent", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn stripIndent(&self) -> Result<String> {
        panic!("stub: java/lang/String.stripIndent:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "outdent", descriptor = "(Ljava/util/List;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/lang/String;>;)I"))]
    pub fn outdent(lines: List<Object>) -> Result<i32> {
        panic!("stub: java/lang/String.outdent:(Ljava/util/List;)I")
    }

    #[cfg_attr(any(), java_method(name = "translateEscapes", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn translateEscapes(&self) -> Result<String> {
        panic!("stub: java/lang/String.translateEscapes:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "transform", descriptor = "(Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-Ljava/lang/String;+TR;>;)TR;"))]
    pub fn transform(&self, f: Object) -> Result<Object> {
        panic!("stub: java/lang/String.transform:(Ljava/util/function/Function;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/String.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn chars(&self) -> Result<Object> {
        panic!("stub: java/lang/String.chars:()Ljava/util/stream/IntStream;")
    }

    #[cfg_attr(any(), java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn codePoints(&self) -> Result<Object> {
        panic!("stub: java/lang/String.codePoints:()Ljava/util/stream/IntStream;")
    }

    #[cfg_attr(any(), java_method(name = "toCharArray", descriptor = "()[C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toCharArray(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
        panic!("stub: java/lang/String.toCharArray:()[C")
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn format_str_arr_obj(format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
        panic!("stub: java/lang/String.format:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn format_locale_str_arr_obj(l: Object, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
        panic!("stub: java/lang/String.format:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "formatted", descriptor = "([Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn formatted(&self, args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
        panic!("stub: java/lang/String.formatted:([Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: valueOf(Ljava/lang/Object;)Ljava/lang/String;
    pub fn valueOf_obj(mut obj: Object) -> Result<String> {
        let mut _merged1: String;
        if _is_jnull(&obj) {
            _merged1 = String::from("null");
        } else {
            let _t0: String = Default::default();
            _merged1 = _t0;
        }
        Ok(_merged1)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "([C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_arr_c(data: Rc<RefCell<Vec<u16>>>) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:([C)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "([CII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_arr_c_i_i(data: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:([CII)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "copyValueOf", descriptor = "([CII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn copyValueOf_arr_c_i_i(data: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<String> {
        panic!("stub: java/lang/String.copyValueOf:([CII)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "copyValueOf", descriptor = "([C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn copyValueOf_arr_c(data: Rc<RefCell<Vec<u16>>>) -> Result<String> {
        panic!("stub: java/lang/String.copyValueOf:([C)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Z)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_z(b: bool) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:(Z)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_c(c: u16) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:(C)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: valueOf(I)Ljava/lang/String;
    pub fn valueOf_i(mut i: i32) -> Result<String> {
        let _t0: String = Integer::toString_i(i)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_l(l: i64) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:(J)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_f(f: f32) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:(F)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_d(d: f64) -> Result<String> {
        panic!("stub: java/lang/String.valueOf:(D)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_native(name = "intern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn intern(&self) -> Result<String> {
        panic!("native: java/lang/String.intern:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn repeat(&self, count: i32) -> Result<String> {
        panic!("stub: java/lang/String.repeat:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "repeatCopyRest", descriptor = "([BIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn repeatCopyRest(buffer: Rc<RefCell<Vec<i8>>>, offset: i32, limit: i32, copied: i32) -> Result<()> {
        panic!("stub: java/lang/String.repeatCopyRest:([BIII)V")
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "([BIB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: getBytes([BIB)V
    pub fn getBytes_arr_b_i_b(&self, mut dst: Rc<RefCell<Vec<i8>>>, mut dstBegin: i32, mut coder: i8) -> Result<()> {
        let this = self;
        let _t0 = this.coder()?;
        if (_t0 as i32) == (coder as i32) {
            System::arraycopy(Object::from_any(this.value.get().clone()), 0i32, Object::from_any(dst.clone()), (dstBegin<<((coder as i32)&0x1f)), (this.value.get().borrow().len() as i32))?;
        } else {
            StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&this.value.get()), 0i32, Clone::clone(&dst), dstBegin, (this.value.get().borrow().len() as i32))?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getBytes", descriptor = "([BIIBI)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getBytes_arr_b_i_i_b_i(&self, dst: Rc<RefCell<Vec<i8>>>, srcPos: i32, dstBegin: i32, coder: i8, length: i32) -> Result<()> {
        panic!("stub: java/lang/String.getBytes:([BIIBI)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CIILjava/lang/Void;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_arr_c_i_i_void(value: Rc<RefCell<Vec<u16>>>, off: i32, len: i32, sig: Object) -> Result<Self> {
        panic!("stub: java/lang/String.<init>:([CIILjava/lang/Void;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V
    pub fn new_abstra_void(mut asb: AbstractStringBuilder, mut sig: Object) -> Result<Self> {
        let mut this = Self { value: JField::new(Default::default()), coder: JField::new(Default::default()), hash: JField::new(0), hashIsZero: JField::new(false), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        let _t0 = asb.getValue()?;
        let mut val: Rc<RefCell<Vec<i8>>> = _t0;
        let _t1 = asb.length()?;
        let mut length: i32 = _t1;
        let _t2 = asb.isLatin1()?;
        if _t2 {
            this.coder.set(((0i32) as i8));
            let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), 0i32, length)?;
            this.value.set(Clone::clone(&_t3));
        } else {
            if asb.maybeLatin1.get() {
                let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_b_i_i(Clone::clone(&val), 0i32, length)?;
                this.value.set(Clone::clone(&_t3));
                let _t4: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&this.value.get()), length)?;
                this.coder.set(_t4);
                return Ok(this);
            }
            this.coder.set(((1i32) as i8));
            let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), 0i32, (length<<(1i32&0x1f)))?;
            this.value.set(Clone::clone(&_t3));
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>([BB)V
    pub fn new_arr_b_b(mut value: Rc<RefCell<Vec<i8>>>, mut coder: i8) -> Result<Self> {
        let mut this = Self { value: JField::new(Default::default()), coder: JField::new(Default::default()), hash: JField::new(0), hashIsZero: JField::new(false), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.value.set(Clone::clone(&value));
        this.coder.set(coder);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "coder", descriptor = "()B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn coder(&self) -> Result<i8> {
        let this = self;
        Ok((if String::COMPACT_STRINGS() { this.coder.get() } else { (1i32 as i8) }))
    }

    #[cfg_attr(any(), java_method(name = "value", descriptor = "()[B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn value(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/String.value:()[B")
    }

    #[cfg_attr(any(), java_method(name = "isLatin1", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isLatin1(&self) -> Result<bool> {
        let this = self;
        Ok((if String::COMPACT_STRINGS() { (this.coder.get()==0) } else { (0i32 != 0) }))
    }

    #[cfg_attr(any(), java_method(name = "checkIndex", descriptor = "(II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkIndex(index: i32, length: i32) -> Result<()> {
        panic!("stub: java/lang/String.checkIndex:(II)V")
    }

    #[cfg_attr(any(), java_method(name = "checkOffset", descriptor = "(II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkOffset(offset: i32, length: i32) -> Result<()> {
        panic!("stub: java/lang/String.checkOffset:(II)V")
    }

    #[cfg_attr(any(), java_method(name = "checkBoundsOffCount", descriptor = "(III)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkBoundsOffCount(mut offset: i32, mut count: i32, mut length: i32) -> Result<i32> {
        let _t0: i32 = Preconditions::checkFromIndexSize_i_i_i_bifunc(offset, count, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "checkBoundsBeginEnd", descriptor = "(III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkBoundsBeginEnd(mut begin: i32, mut end: i32, mut length: i32) -> Result<()> {
        let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(begin, end, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "valueOfCodePoint", descriptor = "(I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOfCodePoint(codePoint: i32) -> Result<String> {
        panic!("stub: java/lang/String.valueOfCodePoint:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/String;>;"))]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/String.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<String> {
        panic!("stub: java/lang/String.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/String;")
    }
}
