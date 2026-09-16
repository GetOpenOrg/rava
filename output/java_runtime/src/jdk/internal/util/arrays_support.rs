#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::util::*;

#[java_rta_macros::java_class(
    binary_name       = "jdk/internal/util/ArraysSupport",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ArraysSupport.java",
    all_supertypes    = "java/lang/Object;jdk/internal/util/ArraysSupport",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ArraysSupport;

impl ArraysSupport {
    #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "package", modifiers = "static final", is_static = true))]
    // static field: U:Ljdk/internal/misc/Unsafe;
    pub fn U() -> Object {
        panic!("stub: jdk/internal/util/ArraysSupport.U:Ljdk/internal/misc/Unsafe;")
    }

    #[cfg_attr(any(), java_field(name = "BIG_ENDIAN", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
    // static field: BIG_ENDIAN:Z
    pub fn BIG_ENDIAN() -> bool {
        panic!("stub: jdk/internal/util/ArraysSupport.BIG_ENDIAN:Z")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_BOOLEAN_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_BOOLEAN_INDEX_SCALE:I
    pub fn LOG2_ARRAY_BOOLEAN_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_BOOLEAN_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_BYTE_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_BYTE_INDEX_SCALE:I
    pub fn LOG2_ARRAY_BYTE_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_BYTE_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_CHAR_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_CHAR_INDEX_SCALE:I
    pub fn LOG2_ARRAY_CHAR_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_CHAR_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_SHORT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_SHORT_INDEX_SCALE:I
    pub fn LOG2_ARRAY_SHORT_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_SHORT_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_INT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_INT_INDEX_SCALE:I
    pub fn LOG2_ARRAY_INT_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_INT_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_LONG_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_LONG_INDEX_SCALE:I
    pub fn LOG2_ARRAY_LONG_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_LONG_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_FLOAT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_FLOAT_INDEX_SCALE:I
    pub fn LOG2_ARRAY_FLOAT_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_FLOAT_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_ARRAY_DOUBLE_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOG2_ARRAY_DOUBLE_INDEX_SCALE:I
    pub fn LOG2_ARRAY_DOUBLE_INDEX_SCALE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_ARRAY_DOUBLE_INDEX_SCALE:I")
    }

    #[cfg_attr(any(), java_field(name = "LOG2_BYTE_BIT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
    // static field: LOG2_BYTE_BIT_SIZE:I
    pub fn LOG2_BYTE_BIT_SIZE() -> i32 {
        panic!("stub: jdk/internal/util/ArraysSupport.LOG2_BYTE_BIT_SIZE:I")
    }

    #[cfg_attr(any(), java_field(name = "T_BOOLEAN", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
    // static field: T_BOOLEAN:I
    pub fn T_BOOLEAN() -> i32 {
        4
    }

    #[cfg_attr(any(), java_field(name = "T_CHAR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
    // static field: T_CHAR:I
    pub fn T_CHAR() -> i32 {
        5
    }

    #[cfg_attr(any(), java_field(name = "T_FLOAT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
    // static field: T_FLOAT:I
    pub fn T_FLOAT() -> i32 {
        6
    }

    #[cfg_attr(any(), java_field(name = "T_DOUBLE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
    // static field: T_DOUBLE:I
    pub fn T_DOUBLE() -> i32 {
        7
    }

    #[cfg_attr(any(), java_field(name = "T_BYTE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
    // static field: T_BYTE:I
    pub fn T_BYTE() -> i32 {
        8
    }

    #[cfg_attr(any(), java_field(name = "T_SHORT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
    // static field: T_SHORT:I
    pub fn T_SHORT() -> i32 {
        9
    }

    #[cfg_attr(any(), java_field(name = "T_INT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
    // static field: T_INT:I
    pub fn T_INT() -> i32 {
        10
    }

    #[cfg_attr(any(), java_field(name = "T_LONG", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
    // static field: T_LONG:I
    pub fn T_LONG() -> i32 {
        11
    }

    #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
    // static field: JLA:Ljdk/internal/access/JavaLangAccess;
    pub fn JLA() -> Object {
        panic!("stub: jdk/internal/util/ArraysSupport.JLA:Ljdk/internal/access/JavaLangAccess;")
    }

    #[cfg_attr(any(), java_field(name = "SOFT_MAX_ARRAY_LENGTH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2147483639"))]
    // static field: SOFT_MAX_ARRAY_LENGTH:I
    pub fn SOFT_MAX_ARRAY_LENGTH() -> i32 {
        2147483639
    }

    #[cfg_attr(any(), java_method(name = "exactLog2", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn exactLog2(scale: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.exactLog2:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: jdk/internal/util/ArraysSupport.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "vectorizedMismatch", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn vectorizedMismatch(a: Object, aOffset: i64, arg2: Object, b: i64, bOffset: i32, arg5: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.vectorizedMismatch:(Ljava/lang/Object;JLjava/lang/Object;JII)I")
    }

    #[cfg_attr(any(), java_method(name = "vectorizedHashCode", descriptor = "(Ljava/lang/Object;IIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn vectorizedHashCode(array: Object, fromIndex: i32, length: i32, initialValue: i32, basicType: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.vectorizedHashCode:(Ljava/lang/Object;IIII)I")
    }

    #[cfg_attr(any(), java_method(name = "signedHashCode", descriptor = "(I[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signedHashCode(result: i32, a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.signedHashCode:(I[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_i_arr_b_i_i(result: i32, a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.hashCode:(I[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I[CII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_i_arr_c_i_i(result: i32, a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.hashCode:(I[CII)I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I[SII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_i_arr_s_i_i(result: i32, a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.hashCode:(I[SII)I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I[III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_i_arr_i_i_i(result: i32, a: Rc<RefCell<Vec<i32>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.hashCode:(I[III)I")
    }

    #[cfg_attr(any(), java_method(name = "utf16hashCode", descriptor = "(I[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn utf16hashCode(result: i32, value: Rc<RefCell<Vec<i8>>>, fromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.utf16hashCode:(I[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([Z[ZI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_z_arr_z_i(a: Rc<RefCell<Vec<bool>>>, b: Rc<RefCell<Vec<bool>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([Z[ZI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([ZI[ZII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_z_i_arr_z_i_i(a: Rc<RefCell<Vec<bool>>>, aFromIndex: i32, b: Rc<RefCell<Vec<bool>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([ZI[ZII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([B[BI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_b_arr_b_i(a: Rc<RefCell<Vec<i8>>>, b: Rc<RefCell<Vec<i8>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([B[BI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_b_i_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, aFromIndex: i32, b: Rc<RefCell<Vec<i8>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([BI[BII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([C[CI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_c_arr_c_i(a: Rc<RefCell<Vec<u16>>>, b: Rc<RefCell<Vec<u16>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([C[CI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([CI[CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_c_i_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, aFromIndex: i32, b: Rc<RefCell<Vec<u16>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([CI[CII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([S[SI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_s_arr_s_i(a: Rc<RefCell<Vec<i16>>>, b: Rc<RefCell<Vec<i16>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([S[SI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([SI[SII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_s_i_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, aFromIndex: i32, b: Rc<RefCell<Vec<i16>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([SI[SII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([I[II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_i_arr_i_i(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([I[II)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([II[III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_i_i_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, aFromIndex: i32, b: Rc<RefCell<Vec<i32>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([II[III)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([F[FI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_f_arr_f_i(a: Rc<RefCell<Vec<f32>>>, b: Rc<RefCell<Vec<f32>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([F[FI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([FI[FII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_f_i_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, aFromIndex: i32, b: Rc<RefCell<Vec<f32>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([FI[FII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([J[JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_l_arr_l_i(a: Rc<RefCell<Vec<i64>>>, b: Rc<RefCell<Vec<i64>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([J[JI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([JI[JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_l_i_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, aFromIndex: i32, b: Rc<RefCell<Vec<i64>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([JI[JII)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([D[DI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_d_arr_d_i(a: Rc<RefCell<Vec<f64>>>, b: Rc<RefCell<Vec<f64>>>, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([D[DI)I")
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "([DI[DII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn mismatch_arr_d_i_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, aFromIndex: i32, b: Rc<RefCell<Vec<f64>>>, bFromIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.mismatch:([DI[DII)I")
    }

    #[cfg_attr(any(), java_method(name = "hugeLength", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hugeLength(oldLength: i32, minGrowth: i32) -> Result<i32> {
        panic!("stub: jdk/internal/util/ArraysSupport.hugeLength:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn reverse(a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: jdk/internal/util/ArraysSupport.reverse:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArrayReversed", descriptor = "(Ljava/util/Collection;[Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<*>;[TT;)[TT;"))]
    pub fn toArrayReversed(coll: Object, array: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: jdk/internal/util/ArraysSupport.toArrayReversed:(Ljava/util/Collection;[Ljava/lang/Object;)[Ljava/lang/Object;")
    }
}
