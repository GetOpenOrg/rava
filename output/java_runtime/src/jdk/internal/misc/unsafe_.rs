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
use crate::jdk::internal::misc::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/misc/Unsafe"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Unsafe.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/misc/Unsafe"]

    pub struct Unsafe;

    impl Unsafe {
        #[cfg_attr(any(), java_field(name = "theUnsafe", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: theUnsafe:Ljdk/internal/misc/Unsafe;
        pub fn theUnsafe() -> Unsafe {
            panic!("stub: jdk/internal/misc/Unsafe.theUnsafe:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "INVALID_FIELD_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-1"))]
        // static field: INVALID_FIELD_OFFSET:I
        pub fn INVALID_FIELD_OFFSET() -> i32 {
            -1
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_BOOLEAN_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BOOLEAN_BASE_OFFSET:I
        pub fn ARRAY_BOOLEAN_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_BOOLEAN_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_BYTE_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BYTE_BASE_OFFSET:I
        pub fn ARRAY_BYTE_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_BYTE_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_SHORT_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_SHORT_BASE_OFFSET:I
        pub fn ARRAY_SHORT_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_SHORT_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_CHAR_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_CHAR_BASE_OFFSET:I
        pub fn ARRAY_CHAR_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_CHAR_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_INT_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_INT_BASE_OFFSET:I
        pub fn ARRAY_INT_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_INT_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_LONG_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_LONG_BASE_OFFSET:I
        pub fn ARRAY_LONG_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_LONG_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_FLOAT_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_FLOAT_BASE_OFFSET:I
        pub fn ARRAY_FLOAT_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_FLOAT_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_DOUBLE_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_DOUBLE_BASE_OFFSET:I
        pub fn ARRAY_DOUBLE_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_DOUBLE_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_OBJECT_BASE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_OBJECT_BASE_OFFSET:I
        pub fn ARRAY_OBJECT_BASE_OFFSET() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_OBJECT_BASE_OFFSET:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_BOOLEAN_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BOOLEAN_INDEX_SCALE:I
        pub fn ARRAY_BOOLEAN_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_BOOLEAN_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_BYTE_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BYTE_INDEX_SCALE:I
        pub fn ARRAY_BYTE_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_BYTE_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_SHORT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_SHORT_INDEX_SCALE:I
        pub fn ARRAY_SHORT_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_SHORT_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_CHAR_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_CHAR_INDEX_SCALE:I
        pub fn ARRAY_CHAR_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_CHAR_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_INT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_INT_INDEX_SCALE:I
        pub fn ARRAY_INT_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_INT_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_LONG_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_LONG_INDEX_SCALE:I
        pub fn ARRAY_LONG_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_LONG_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_FLOAT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_FLOAT_INDEX_SCALE:I
        pub fn ARRAY_FLOAT_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_FLOAT_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_DOUBLE_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_DOUBLE_INDEX_SCALE:I
        pub fn ARRAY_DOUBLE_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_DOUBLE_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_OBJECT_INDEX_SCALE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ARRAY_OBJECT_INDEX_SCALE:I
        pub fn ARRAY_OBJECT_INDEX_SCALE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ARRAY_OBJECT_INDEX_SCALE:I")
        }

        #[cfg_attr(any(), java_field(name = "ADDRESS_SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true))]
        // static field: ADDRESS_SIZE:I
        pub fn ADDRESS_SIZE() -> i32 {
            panic!("stub: jdk/internal/misc/Unsafe.ADDRESS_SIZE:I")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.registerNatives:()V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/misc/Unsafe.<init>:()V")
        }

        #[java_method(name = "getUnsafe", descriptor = "()Ljdk/internal/misc/Unsafe;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnsafe() -> Result<Unsafe> {
            panic!("stub: jdk/internal/misc/Unsafe.getUnsafe:()Ljdk/internal/misc/Unsafe;")
        }

        #[native]
        #[java_native(name = "getInt", descriptor = "(Ljava/lang/Object;J)I", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getInt_obj_l(&self, arg0: Object, arg1: i64) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.getInt:(Ljava/lang/Object;J)I")
        }

        #[native]
        #[java_native(name = "putInt", descriptor = "(Ljava/lang/Object;JI)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putInt_obj_l_i(&self, arg0: Object, arg1: i64, arg2: i32) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putInt:(Ljava/lang/Object;JI)V")
        }

        #[native]
        #[java_native(name = "getReference", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getReference(&self, arg0: Object, arg1: i64) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.getReference:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "putReference", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putReference(&self, arg0: Object, arg1: i64, arg2: Object) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putReference:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[native]
        #[java_native(name = "getBoolean", descriptor = "(Ljava/lang/Object;J)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getBoolean(&self, arg0: Object, arg1: i64) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.getBoolean:(Ljava/lang/Object;J)Z")
        }

        #[native]
        #[java_native(name = "putBoolean", descriptor = "(Ljava/lang/Object;JZ)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putBoolean(&self, arg0: Object, arg1: i64, arg2: bool) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putBoolean:(Ljava/lang/Object;JZ)V")
        }

        #[native]
        #[java_native(name = "getByte", descriptor = "(Ljava/lang/Object;J)B", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getByte_obj_l(&self, arg0: Object, arg1: i64) -> Result<i8> {
            panic!("native: jdk/internal/misc/Unsafe.getByte:(Ljava/lang/Object;J)B")
        }

        #[native]
        #[java_native(name = "putByte", descriptor = "(Ljava/lang/Object;JB)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putByte_obj_l_b(&self, arg0: Object, arg1: i64, arg2: i8) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putByte:(Ljava/lang/Object;JB)V")
        }

        #[native]
        #[java_native(name = "getShort", descriptor = "(Ljava/lang/Object;J)S", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getShort_obj_l(&self, arg0: Object, arg1: i64) -> Result<i16> {
            panic!("native: jdk/internal/misc/Unsafe.getShort:(Ljava/lang/Object;J)S")
        }

        #[native]
        #[java_native(name = "putShort", descriptor = "(Ljava/lang/Object;JS)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putShort_obj_l_s(&self, arg0: Object, arg1: i64, arg2: i16) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putShort:(Ljava/lang/Object;JS)V")
        }

        #[native]
        #[java_native(name = "getChar", descriptor = "(Ljava/lang/Object;J)C", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getChar_obj_l(&self, arg0: Object, arg1: i64) -> Result<u16> {
            panic!("native: jdk/internal/misc/Unsafe.getChar:(Ljava/lang/Object;J)C")
        }

        #[native]
        #[java_native(name = "putChar", descriptor = "(Ljava/lang/Object;JC)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putChar_obj_l_c(&self, arg0: Object, arg1: i64, arg2: u16) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putChar:(Ljava/lang/Object;JC)V")
        }

        #[native]
        #[java_native(name = "getLong", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getLong_obj_l(&self, arg0: Object, arg1: i64) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.getLong:(Ljava/lang/Object;J)J")
        }

        #[native]
        #[java_native(name = "putLong", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putLong_obj_l_l(&self, arg0: Object, arg1: i64, arg2: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putLong:(Ljava/lang/Object;JJ)V")
        }

        #[native]
        #[java_native(name = "getFloat", descriptor = "(Ljava/lang/Object;J)F", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat_obj_l(&self, arg0: Object, arg1: i64) -> Result<f32> {
            panic!("native: jdk/internal/misc/Unsafe.getFloat:(Ljava/lang/Object;J)F")
        }

        #[native]
        #[java_native(name = "putFloat", descriptor = "(Ljava/lang/Object;JF)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putFloat_obj_l_f(&self, arg0: Object, arg1: i64, arg2: f32) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putFloat:(Ljava/lang/Object;JF)V")
        }

        #[native]
        #[java_native(name = "getDouble", descriptor = "(Ljava/lang/Object;J)D", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble_obj_l(&self, arg0: Object, arg1: i64) -> Result<f64> {
            panic!("native: jdk/internal/misc/Unsafe.getDouble:(Ljava/lang/Object;J)D")
        }

        #[native]
        #[java_native(name = "putDouble", descriptor = "(Ljava/lang/Object;JD)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putDouble_obj_l_d(&self, arg0: Object, arg1: i64, arg2: f64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putDouble:(Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getAddress", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAddress_obj_l(&self, o: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAddress:(Ljava/lang/Object;J)J")
        }

        #[java_method(name = "putAddress", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putAddress_obj_l_l(&self, o: Object, offset: i64, arg2: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putAddress:(Ljava/lang/Object;JJ)V")
        }

        #[native]
        #[java_native(name = "getUncompressedObject", descriptor = "(J)Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getUncompressedObject(&self, arg0: i64) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.getUncompressedObject:(J)Ljava/lang/Object;")
        }

        #[java_method(name = "getByte", descriptor = "(J)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByte_l(&self, address: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getByte:(J)B")
        }

        #[java_method(name = "putByte", descriptor = "(JB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByte_l_b(&self, address: i64, arg1: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putByte:(JB)V")
        }

        #[java_method(name = "getShort", descriptor = "(J)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShort_l(&self, address: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getShort:(J)S")
        }

        #[java_method(name = "putShort", descriptor = "(JS)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShort_l_s(&self, address: i64, arg1: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShort:(JS)V")
        }

        #[java_method(name = "getChar", descriptor = "(J)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar_l(&self, address: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getChar:(J)C")
        }

        #[java_method(name = "putChar", descriptor = "(JC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar_l_c(&self, address: i64, arg1: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putChar:(JC)V")
        }

        #[java_method(name = "getInt", descriptor = "(J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt_l(&self, address: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getInt:(J)I")
        }

        #[java_method(name = "putInt", descriptor = "(JI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putInt_l_i(&self, address: i64, arg1: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putInt:(JI)V")
        }

        #[java_method(name = "getLong", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong_l(&self, address: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getLong:(J)J")
        }

        #[java_method(name = "putLong", descriptor = "(JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLong_l_l(&self, address: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLong:(JJ)V")
        }

        #[java_method(name = "getFloat", descriptor = "(J)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat_l(&self, address: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getFloat:(J)F")
        }

        #[java_method(name = "putFloat", descriptor = "(JF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloat_l_f(&self, address: i64, arg1: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putFloat:(JF)V")
        }

        #[java_method(name = "getDouble", descriptor = "(J)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble_l(&self, address: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getDouble:(J)D")
        }

        #[java_method(name = "putDouble", descriptor = "(JD)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDouble_l_d(&self, address: i64, arg1: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putDouble:(JD)V")
        }

        #[java_method(name = "getAddress", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAddress_l(&self, address: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAddress:(J)J")
        }

        #[java_method(name = "putAddress", descriptor = "(JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putAddress_l_l(&self, address: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putAddress:(JJ)V")
        }

        #[java_method(name = "invalidInput", descriptor = "()Ljava/lang/RuntimeException;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn invalidInput(&self) -> Result<RuntimeException> {
            panic!("stub: jdk/internal/misc/Unsafe.invalidInput:()Ljava/lang/RuntimeException;")
        }

        #[java_method(name = "is32BitClean", descriptor = "(J)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn is32BitClean(&self, value: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.is32BitClean:(J)Z")
        }

        #[java_method(name = "checkSize", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkSize(&self, size: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkSize:(J)V")
        }

        #[java_method(name = "checkNativeAddress", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkNativeAddress(&self, address: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkNativeAddress:(J)V")
        }

        #[java_method(name = "checkOffset", descriptor = "(Ljava/lang/Object;J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkOffset(&self, o: Object, offset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkOffset:(Ljava/lang/Object;J)V")
        }

        #[java_method(name = "checkPointer", descriptor = "(Ljava/lang/Object;J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPointer(&self, o: Object, offset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkPointer:(Ljava/lang/Object;J)V")
        }

        #[java_method(name = "checkPrimitiveArray", descriptor = "(Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn checkPrimitiveArray(&self, c: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkPrimitiveArray:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "checkPrimitivePointer", descriptor = "(Ljava/lang/Object;J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPrimitivePointer(&self, o: Object, offset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkPrimitivePointer:(Ljava/lang/Object;J)V")
        }

        #[java_method(name = "alignToHeapWordSize", descriptor = "(J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn alignToHeapWordSize(&self, bytes: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.alignToHeapWordSize:(J)J")
        }

        #[java_method(name = "allocateMemory", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allocateMemory(&self, bytes: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.allocateMemory:(J)J")
        }

        #[java_method(name = "allocateMemoryChecks", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allocateMemoryChecks(&self, bytes: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.allocateMemoryChecks:(J)V")
        }

        #[java_method(name = "reallocateMemory", descriptor = "(JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reallocateMemory(&self, address: i64, arg1: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.reallocateMemory:(JJ)J")
        }

        #[java_method(name = "reallocateMemoryChecks", descriptor = "(JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reallocateMemoryChecks(&self, address: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.reallocateMemoryChecks:(JJ)V")
        }

        #[java_method(name = "setMemory", descriptor = "(Ljava/lang/Object;JJB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMemory_obj_l_l_b(&self, o: Object, offset: i64, arg2: i64, bytes: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.setMemory:(Ljava/lang/Object;JJB)V")
        }

        #[java_method(name = "setMemory", descriptor = "(JJB)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMemory_l_l_b(&self, address: i64, arg1: i64, bytes: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.setMemory:(JJB)V")
        }

        #[java_method(name = "setMemoryChecks", descriptor = "(Ljava/lang/Object;JJB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMemoryChecks(&self, o: Object, offset: i64, arg2: i64, bytes: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.setMemoryChecks:(Ljava/lang/Object;JJB)V")
        }

        #[java_method(name = "copyMemory", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemory_obj_l_obj_l_l(&self, srcBase: Object, srcOffset: i64, arg2: Object, destBase: i64, destOffset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copyMemory:(Ljava/lang/Object;JLjava/lang/Object;JJ)V")
        }

        #[java_method(name = "copyMemory", descriptor = "(JJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemory_l_l_l(&self, srcAddress: i64, arg1: i64, destAddress: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copyMemory:(JJJ)V")
        }

        #[java_method(name = "copyMemoryChecks", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemoryChecks(&self, srcBase: Object, srcOffset: i64, arg2: Object, destBase: i64, destOffset: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copyMemoryChecks:(Ljava/lang/Object;JLjava/lang/Object;JJ)V")
        }

        #[java_method(name = "copySwapMemory", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemory_obj_l_obj_l_l_l(&self, srcBase: Object, srcOffset: i64, arg2: Object, destBase: i64, destOffset: i64, arg5: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copySwapMemory:(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
        }

        #[java_method(name = "copySwapMemoryChecks", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemoryChecks(&self, srcBase: Object, srcOffset: i64, arg2: Object, destBase: i64, destOffset: i64, arg5: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copySwapMemoryChecks:(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
        }

        #[java_method(name = "copySwapMemory", descriptor = "(JJJJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemory_l_l_l_l(&self, srcAddress: i64, arg1: i64, destAddress: i64, arg3: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.copySwapMemory:(JJJJ)V")
        }

        #[java_method(name = "freeMemory", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn freeMemory(&self, address: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.freeMemory:(J)V")
        }

        #[java_method(name = "freeMemoryChecks", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn freeMemoryChecks(&self, address: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.freeMemoryChecks:(J)V")
        }

        #[java_method(name = "writebackMemory", descriptor = "(JJ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writebackMemory(&self, address: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.writebackMemory:(JJ)V")
        }

        #[java_method(name = "checkWritebackMemory", descriptor = "(JJ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkWritebackMemory(&self, address: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkWritebackMemory:(JJ)V")
        }

        #[java_method(name = "checkWritebackEnabled", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkWritebackEnabled(&self) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.checkWritebackEnabled:()V")
        }

        #[native]
        #[java_native(name = "writeback0", descriptor = "(J)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn writeback0(&self, arg0: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.writeback0:(J)V")
        }

        #[native]
        #[java_native(name = "writebackPreSync0", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn writebackPreSync0(&self) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.writebackPreSync0:()V")
        }

        #[native]
        #[java_native(name = "writebackPostSync0", descriptor = "()V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn writebackPostSync0(&self) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.writebackPostSync0:()V")
        }

        #[java_method(name = "objectFieldOffset", descriptor = "(Ljava/lang/reflect/Field;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn objectFieldOffset_field(&self, f: Object) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.objectFieldOffset:(Ljava/lang/reflect/Field;)J")
        }

        #[java_method(name = "objectFieldOffset", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;)J")]
        pub fn objectFieldOffset_class_str(&self, c: Object, name: String) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.objectFieldOffset:(Ljava/lang/Class;Ljava/lang/String;)J")
        }

        #[java_method(name = "staticFieldOffset", descriptor = "(Ljava/lang/reflect/Field;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn staticFieldOffset(&self, f: Object) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.staticFieldOffset:(Ljava/lang/reflect/Field;)J")
        }

        #[java_method(name = "staticFieldBase", descriptor = "(Ljava/lang/reflect/Field;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn staticFieldBase(&self, f: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.staticFieldBase:(Ljava/lang/reflect/Field;)Ljava/lang/Object;")
        }

        #[java_method(name = "shouldBeInitialized", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn shouldBeInitialized(&self, c: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.shouldBeInitialized:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "ensureClassInitialized", descriptor = "(Ljava/lang/Class;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn ensureClassInitialized(&self, c: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.ensureClassInitialized:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "arrayBaseOffset", descriptor = "(Ljava/lang/Class;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)I")]
        pub fn arrayBaseOffset(&self, arrayClass: Object) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.arrayBaseOffset:(Ljava/lang/Class;)I")
        }

        #[java_method(name = "arrayIndexScale", descriptor = "(Ljava/lang/Class;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)I")]
        pub fn arrayIndexScale(&self, arrayClass: Object) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.arrayIndexScale:(Ljava/lang/Class;)I")
        }

        #[java_method(name = "addressSize", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addressSize(&self) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.addressSize:()I")
        }

        #[java_method(name = "pageSize", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pageSize(&self) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.pageSize:()I")
        }

        #[java_method(name = "dataCacheLineFlushSize", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dataCacheLineFlushSize(&self) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.dataCacheLineFlushSize:()I")
        }

        #[java_method(name = "dataCacheLineAlignDown", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dataCacheLineAlignDown(&self, address: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.dataCacheLineAlignDown:(J)J")
        }

        #[java_method(name = "isWritebackEnabled", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWritebackEnabled() -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.isWritebackEnabled:()Z")
        }

        #[java_method(name = "defineClass", descriptor = "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class<*>;")]
        pub fn defineClass(&self, name: String, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32, loader: ClassLoader, protectionDomain: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.defineClass:(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "defineClass0", descriptor = "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class<*>;")]
        pub fn defineClass0(&self, arg0: String, arg1: Rc<RefCell<Vec<i8>>>, arg2: i32, arg3: i32, arg4: ClassLoader, arg5: Object) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.defineClass0:(Ljava/lang/String;[BIILjava/lang/ClassLoader;Ljava/security/ProtectionDomain;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "allocateInstance", descriptor = "(Ljava/lang/Class;)Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InstantiationException", generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/Object;")]
        pub fn allocateInstance(&self, arg0: Object) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.allocateInstance:(Ljava/lang/Class;)Ljava/lang/Object;")
        }

        #[java_method(name = "allocateUninitializedArray", descriptor = "(Ljava/lang/Class;I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;I)Ljava/lang/Object;")]
        pub fn allocateUninitializedArray(&self, componentType: Object, length: i32) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.allocateUninitializedArray:(Ljava/lang/Class;I)Ljava/lang/Object;")
        }

        #[java_method(name = "allocateUninitializedArray0", descriptor = "(Ljava/lang/Class;I)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;I)Ljava/lang/Object;")]
        pub fn allocateUninitializedArray0(&self, componentType: Object, length: i32) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.allocateUninitializedArray0:(Ljava/lang/Class;I)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "throwException", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn throwException(&self, arg0: Throwable) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.throwException:(Ljava/lang/Throwable;)V")
        }

        #[native]
        #[java_native(name = "compareAndSetReference", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetReference(&self, arg0: Object, arg1: i64, arg2: Object, arg3: Object) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[native]
        #[java_native(name = "compareAndExchangeReference", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeReference(&self, arg0: Object, arg1: i64, arg2: Object, arg3: Object) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndExchangeReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "compareAndExchangeReferenceAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeReferenceAcquire(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeReferenceAcquire:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "compareAndExchangeReferenceRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeReferenceRelease(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeReferenceRelease:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "weakCompareAndSetReferencePlain", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetReferencePlain(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetReferencePlain:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetReferenceAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetReferenceAcquire(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetReferenceAcquire:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetReferenceRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetReferenceRelease(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetReferenceRelease:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetReference", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetReference(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[native]
        #[java_native(name = "compareAndSetInt", descriptor = "(Ljava/lang/Object;JII)Z", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetInt(&self, arg0: Object, arg1: i64, arg2: i32, arg3: i32) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndSetInt:(Ljava/lang/Object;JII)Z")
        }

        #[native]
        #[java_native(name = "compareAndExchangeInt", descriptor = "(Ljava/lang/Object;JII)I", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeInt(&self, arg0: Object, arg1: i64, arg2: i32, arg3: i32) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndExchangeInt:(Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntAcquire", descriptor = "(Ljava/lang/Object;JII)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntAcquire(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeIntAcquire:(Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "compareAndExchangeIntRelease", descriptor = "(Ljava/lang/Object;JII)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeIntRelease(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeIntRelease:(Ljava/lang/Object;JII)I")
        }

        #[java_method(name = "weakCompareAndSetIntPlain", descriptor = "(Ljava/lang/Object;JII)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntPlain(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetIntPlain:(Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntAcquire", descriptor = "(Ljava/lang/Object;JII)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntAcquire(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetIntAcquire:(Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetIntRelease", descriptor = "(Ljava/lang/Object;JII)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetIntRelease(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetIntRelease:(Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "weakCompareAndSetInt", descriptor = "(Ljava/lang/Object;JII)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetInt(&self, o: Object, offset: i64, arg2: i32, expected: i32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetInt:(Ljava/lang/Object;JII)Z")
        }

        #[java_method(name = "compareAndExchangeByte", descriptor = "(Ljava/lang/Object;JBB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeByte(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeByte:(Ljava/lang/Object;JBB)B")
        }

        #[java_method(name = "compareAndSetByte", descriptor = "(Ljava/lang/Object;JBB)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetByte(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetByte:(Ljava/lang/Object;JBB)Z")
        }

        #[java_method(name = "weakCompareAndSetByte", descriptor = "(Ljava/lang/Object;JBB)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetByte(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetByte:(Ljava/lang/Object;JBB)Z")
        }

        #[java_method(name = "weakCompareAndSetByteAcquire", descriptor = "(Ljava/lang/Object;JBB)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetByteAcquire(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetByteAcquire:(Ljava/lang/Object;JBB)Z")
        }

        #[java_method(name = "weakCompareAndSetByteRelease", descriptor = "(Ljava/lang/Object;JBB)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetByteRelease(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetByteRelease:(Ljava/lang/Object;JBB)Z")
        }

        #[java_method(name = "weakCompareAndSetBytePlain", descriptor = "(Ljava/lang/Object;JBB)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetBytePlain(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetBytePlain:(Ljava/lang/Object;JBB)Z")
        }

        #[java_method(name = "compareAndExchangeByteAcquire", descriptor = "(Ljava/lang/Object;JBB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeByteAcquire(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeByteAcquire:(Ljava/lang/Object;JBB)B")
        }

        #[java_method(name = "compareAndExchangeByteRelease", descriptor = "(Ljava/lang/Object;JBB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeByteRelease(&self, o: Object, offset: i64, arg2: i8, expected: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeByteRelease:(Ljava/lang/Object;JBB)B")
        }

        #[java_method(name = "compareAndExchangeShort", descriptor = "(Ljava/lang/Object;JSS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeShort(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeShort:(Ljava/lang/Object;JSS)S")
        }

        #[java_method(name = "compareAndSetShort", descriptor = "(Ljava/lang/Object;JSS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetShort(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetShort:(Ljava/lang/Object;JSS)Z")
        }

        #[java_method(name = "weakCompareAndSetShort", descriptor = "(Ljava/lang/Object;JSS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetShort(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetShort:(Ljava/lang/Object;JSS)Z")
        }

        #[java_method(name = "weakCompareAndSetShortAcquire", descriptor = "(Ljava/lang/Object;JSS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetShortAcquire(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetShortAcquire:(Ljava/lang/Object;JSS)Z")
        }

        #[java_method(name = "weakCompareAndSetShortRelease", descriptor = "(Ljava/lang/Object;JSS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetShortRelease(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetShortRelease:(Ljava/lang/Object;JSS)Z")
        }

        #[java_method(name = "weakCompareAndSetShortPlain", descriptor = "(Ljava/lang/Object;JSS)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetShortPlain(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetShortPlain:(Ljava/lang/Object;JSS)Z")
        }

        #[java_method(name = "compareAndExchangeShortAcquire", descriptor = "(Ljava/lang/Object;JSS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeShortAcquire(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeShortAcquire:(Ljava/lang/Object;JSS)S")
        }

        #[java_method(name = "compareAndExchangeShortRelease", descriptor = "(Ljava/lang/Object;JSS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeShortRelease(&self, o: Object, offset: i64, arg2: i16, expected: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeShortRelease:(Ljava/lang/Object;JSS)S")
        }

        #[java_method(name = "s2c", descriptor = "(S)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn s2c(&self, s: i16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.s2c:(S)C")
        }

        #[java_method(name = "c2s", descriptor = "(C)S", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn c2s(&self, s: u16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.c2s:(C)S")
        }

        #[java_method(name = "compareAndSetChar", descriptor = "(Ljava/lang/Object;JCC)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetChar(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetChar:(Ljava/lang/Object;JCC)Z")
        }

        #[java_method(name = "compareAndExchangeChar", descriptor = "(Ljava/lang/Object;JCC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeChar(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeChar:(Ljava/lang/Object;JCC)C")
        }

        #[java_method(name = "compareAndExchangeCharAcquire", descriptor = "(Ljava/lang/Object;JCC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeCharAcquire(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeCharAcquire:(Ljava/lang/Object;JCC)C")
        }

        #[java_method(name = "compareAndExchangeCharRelease", descriptor = "(Ljava/lang/Object;JCC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeCharRelease(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeCharRelease:(Ljava/lang/Object;JCC)C")
        }

        #[java_method(name = "weakCompareAndSetChar", descriptor = "(Ljava/lang/Object;JCC)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetChar(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetChar:(Ljava/lang/Object;JCC)Z")
        }

        #[java_method(name = "weakCompareAndSetCharAcquire", descriptor = "(Ljava/lang/Object;JCC)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetCharAcquire(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetCharAcquire:(Ljava/lang/Object;JCC)Z")
        }

        #[java_method(name = "weakCompareAndSetCharRelease", descriptor = "(Ljava/lang/Object;JCC)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetCharRelease(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetCharRelease:(Ljava/lang/Object;JCC)Z")
        }

        #[java_method(name = "weakCompareAndSetCharPlain", descriptor = "(Ljava/lang/Object;JCC)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetCharPlain(&self, o: Object, offset: i64, arg2: u16, expected: u16) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetCharPlain:(Ljava/lang/Object;JCC)Z")
        }

        #[java_method(name = "byte2bool", descriptor = "(B)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byte2bool(&self, b: i8) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.byte2bool:(B)Z")
        }

        #[java_method(name = "bool2byte", descriptor = "(Z)B", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bool2byte(&self, b: bool) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.bool2byte:(Z)B")
        }

        #[java_method(name = "compareAndSetBoolean", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetBoolean(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetBoolean:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "compareAndExchangeBoolean", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeBoolean(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeBoolean:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "compareAndExchangeBooleanAcquire", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeBooleanAcquire(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeBooleanAcquire:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "compareAndExchangeBooleanRelease", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeBooleanRelease(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeBooleanRelease:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetBoolean", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetBoolean(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetBoolean:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetBooleanAcquire", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetBooleanAcquire(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetBooleanAcquire:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetBooleanRelease", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetBooleanRelease(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetBooleanRelease:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetBooleanPlain", descriptor = "(Ljava/lang/Object;JZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetBooleanPlain(&self, o: Object, offset: i64, arg2: bool, expected: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetBooleanPlain:(Ljava/lang/Object;JZZ)Z")
        }

        #[java_method(name = "compareAndSetFloat", descriptor = "(Ljava/lang/Object;JFF)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetFloat(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetFloat:(Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "compareAndExchangeFloat", descriptor = "(Ljava/lang/Object;JFF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloat(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeFloat:(Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatAcquire", descriptor = "(Ljava/lang/Object;JFF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatAcquire(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeFloatAcquire:(Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "compareAndExchangeFloatRelease", descriptor = "(Ljava/lang/Object;JFF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeFloatRelease(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeFloatRelease:(Ljava/lang/Object;JFF)F")
        }

        #[java_method(name = "weakCompareAndSetFloatPlain", descriptor = "(Ljava/lang/Object;JFF)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatPlain(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetFloatPlain:(Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatAcquire", descriptor = "(Ljava/lang/Object;JFF)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatAcquire(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetFloatAcquire:(Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloatRelease", descriptor = "(Ljava/lang/Object;JFF)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloatRelease(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetFloatRelease:(Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "weakCompareAndSetFloat", descriptor = "(Ljava/lang/Object;JFF)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetFloat(&self, o: Object, offset: i64, arg2: f32, expected: f32) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetFloat:(Ljava/lang/Object;JFF)Z")
        }

        #[java_method(name = "compareAndSetDouble", descriptor = "(Ljava/lang/Object;JDD)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetDouble(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetDouble:(Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "compareAndExchangeDouble", descriptor = "(Ljava/lang/Object;JDD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDouble(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeDouble:(Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleAcquire", descriptor = "(Ljava/lang/Object;JDD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleAcquire(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeDoubleAcquire:(Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "compareAndExchangeDoubleRelease", descriptor = "(Ljava/lang/Object;JDD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeDoubleRelease(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeDoubleRelease:(Ljava/lang/Object;JDD)D")
        }

        #[java_method(name = "weakCompareAndSetDoublePlain", descriptor = "(Ljava/lang/Object;JDD)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoublePlain(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetDoublePlain:(Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleAcquire", descriptor = "(Ljava/lang/Object;JDD)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleAcquire(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetDoubleAcquire:(Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDoubleRelease", descriptor = "(Ljava/lang/Object;JDD)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDoubleRelease(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetDoubleRelease:(Ljava/lang/Object;JDD)Z")
        }

        #[java_method(name = "weakCompareAndSetDouble", descriptor = "(Ljava/lang/Object;JDD)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetDouble(&self, o: Object, offset: i64, arg2: f64, expected: f64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetDouble:(Ljava/lang/Object;JDD)Z")
        }

        #[native]
        #[java_native(name = "compareAndSetLong", descriptor = "(Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSetLong(&self, arg0: Object, arg1: i64, arg2: i64, arg3: i64) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndSetLong:(Ljava/lang/Object;JJJ)Z")
        }

        #[native]
        #[java_native(name = "compareAndExchangeLong", descriptor = "(Ljava/lang/Object;JJJ)J", access = "public", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLong(&self, arg0: Object, arg1: i64, arg2: i64, arg3: i64) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.compareAndExchangeLong:(Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongAcquire", descriptor = "(Ljava/lang/Object;JJJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongAcquire(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeLongAcquire:(Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "compareAndExchangeLongRelease", descriptor = "(Ljava/lang/Object;JJJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeLongRelease(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeLongRelease:(Ljava/lang/Object;JJJ)J")
        }

        #[java_method(name = "weakCompareAndSetLongPlain", descriptor = "(Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongPlain(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetLongPlain:(Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongAcquire", descriptor = "(Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongAcquire(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetLongAcquire:(Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLongRelease", descriptor = "(Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLongRelease(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetLongRelease:(Ljava/lang/Object;JJJ)Z")
        }

        #[java_method(name = "weakCompareAndSetLong", descriptor = "(Ljava/lang/Object;JJJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetLong(&self, o: Object, offset: i64, arg2: i64, expected: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetLong:(Ljava/lang/Object;JJJ)Z")
        }

        #[native]
        #[java_native(name = "getReferenceVolatile", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getReferenceVolatile(&self, arg0: Object, arg1: i64) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.getReferenceVolatile:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "putReferenceVolatile", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putReferenceVolatile(&self, arg0: Object, arg1: i64, arg2: Object) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putReferenceVolatile:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[native]
        #[java_native(name = "getIntVolatile", descriptor = "(Ljava/lang/Object;J)I", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getIntVolatile(&self, arg0: Object, arg1: i64) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.getIntVolatile:(Ljava/lang/Object;J)I")
        }

        #[native]
        #[java_native(name = "putIntVolatile", descriptor = "(Ljava/lang/Object;JI)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putIntVolatile(&self, arg0: Object, arg1: i64, arg2: i32) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putIntVolatile:(Ljava/lang/Object;JI)V")
        }

        #[native]
        #[java_native(name = "getBooleanVolatile", descriptor = "(Ljava/lang/Object;J)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getBooleanVolatile(&self, arg0: Object, arg1: i64) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.getBooleanVolatile:(Ljava/lang/Object;J)Z")
        }

        #[native]
        #[java_native(name = "putBooleanVolatile", descriptor = "(Ljava/lang/Object;JZ)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putBooleanVolatile(&self, arg0: Object, arg1: i64, arg2: bool) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putBooleanVolatile:(Ljava/lang/Object;JZ)V")
        }

        #[native]
        #[java_native(name = "getByteVolatile", descriptor = "(Ljava/lang/Object;J)B", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getByteVolatile(&self, arg0: Object, arg1: i64) -> Result<i8> {
            panic!("native: jdk/internal/misc/Unsafe.getByteVolatile:(Ljava/lang/Object;J)B")
        }

        #[native]
        #[java_native(name = "putByteVolatile", descriptor = "(Ljava/lang/Object;JB)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putByteVolatile(&self, arg0: Object, arg1: i64, arg2: i8) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putByteVolatile:(Ljava/lang/Object;JB)V")
        }

        #[native]
        #[java_native(name = "getShortVolatile", descriptor = "(Ljava/lang/Object;J)S", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getShortVolatile(&self, arg0: Object, arg1: i64) -> Result<i16> {
            panic!("native: jdk/internal/misc/Unsafe.getShortVolatile:(Ljava/lang/Object;J)S")
        }

        #[native]
        #[java_native(name = "putShortVolatile", descriptor = "(Ljava/lang/Object;JS)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putShortVolatile(&self, arg0: Object, arg1: i64, arg2: i16) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putShortVolatile:(Ljava/lang/Object;JS)V")
        }

        #[native]
        #[java_native(name = "getCharVolatile", descriptor = "(Ljava/lang/Object;J)C", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getCharVolatile(&self, arg0: Object, arg1: i64) -> Result<u16> {
            panic!("native: jdk/internal/misc/Unsafe.getCharVolatile:(Ljava/lang/Object;J)C")
        }

        #[native]
        #[java_native(name = "putCharVolatile", descriptor = "(Ljava/lang/Object;JC)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putCharVolatile(&self, arg0: Object, arg1: i64, arg2: u16) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putCharVolatile:(Ljava/lang/Object;JC)V")
        }

        #[native]
        #[java_native(name = "getLongVolatile", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getLongVolatile(&self, arg0: Object, arg1: i64) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.getLongVolatile:(Ljava/lang/Object;J)J")
        }

        #[native]
        #[java_native(name = "putLongVolatile", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putLongVolatile(&self, arg0: Object, arg1: i64, arg2: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putLongVolatile:(Ljava/lang/Object;JJ)V")
        }

        #[native]
        #[java_native(name = "getFloatVolatile", descriptor = "(Ljava/lang/Object;J)F", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatVolatile(&self, arg0: Object, arg1: i64) -> Result<f32> {
            panic!("native: jdk/internal/misc/Unsafe.getFloatVolatile:(Ljava/lang/Object;J)F")
        }

        #[native]
        #[java_native(name = "putFloatVolatile", descriptor = "(Ljava/lang/Object;JF)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatVolatile(&self, arg0: Object, arg1: i64, arg2: f32) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putFloatVolatile:(Ljava/lang/Object;JF)V")
        }

        #[native]
        #[java_native(name = "getDoubleVolatile", descriptor = "(Ljava/lang/Object;J)D", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleVolatile(&self, arg0: Object, arg1: i64) -> Result<f64> {
            panic!("native: jdk/internal/misc/Unsafe.getDoubleVolatile:(Ljava/lang/Object;J)D")
        }

        #[native]
        #[java_native(name = "putDoubleVolatile", descriptor = "(Ljava/lang/Object;JD)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleVolatile(&self, arg0: Object, arg1: i64, arg2: f64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.putDoubleVolatile:(Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getReferenceAcquire", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getReferenceAcquire(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getReferenceAcquire:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "getBooleanAcquire", descriptor = "(Ljava/lang/Object;J)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBooleanAcquire(&self, o: Object, offset: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getBooleanAcquire:(Ljava/lang/Object;J)Z")
        }

        #[java_method(name = "getByteAcquire", descriptor = "(Ljava/lang/Object;J)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteAcquire(&self, o: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getByteAcquire:(Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getShortAcquire", descriptor = "(Ljava/lang/Object;J)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortAcquire(&self, o: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getShortAcquire:(Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getCharAcquire", descriptor = "(Ljava/lang/Object;J)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharAcquire(&self, o: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getCharAcquire:(Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getIntAcquire", descriptor = "(Ljava/lang/Object;J)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntAcquire(&self, o: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getIntAcquire:(Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getFloatAcquire", descriptor = "(Ljava/lang/Object;J)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatAcquire(&self, o: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getFloatAcquire:(Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getLongAcquire", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongAcquire(&self, o: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getLongAcquire:(Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getDoubleAcquire", descriptor = "(Ljava/lang/Object;J)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleAcquire(&self, o: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getDoubleAcquire:(Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putReferenceRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putReferenceRelease(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putReferenceRelease:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "putBooleanRelease", descriptor = "(Ljava/lang/Object;JZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putBooleanRelease(&self, o: Object, offset: i64, arg2: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putBooleanRelease:(Ljava/lang/Object;JZ)V")
        }

        #[java_method(name = "putByteRelease", descriptor = "(Ljava/lang/Object;JB)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putByteRelease:(Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putShortRelease", descriptor = "(Ljava/lang/Object;JS)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShortRelease:(Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putCharRelease", descriptor = "(Ljava/lang/Object;JC)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putCharRelease:(Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putIntRelease", descriptor = "(Ljava/lang/Object;JI)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntRelease:(Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putFloatRelease", descriptor = "(Ljava/lang/Object;JF)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatRelease(&self, o: Object, offset: i64, arg2: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putFloatRelease:(Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putLongRelease", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongRelease:(Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putDoubleRelease", descriptor = "(Ljava/lang/Object;JD)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleRelease(&self, o: Object, offset: i64, arg2: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putDoubleRelease:(Ljava/lang/Object;JD)V")
        }

        #[java_method(name = "getReferenceOpaque", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getReferenceOpaque(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getReferenceOpaque:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "getBooleanOpaque", descriptor = "(Ljava/lang/Object;J)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBooleanOpaque(&self, o: Object, offset: i64) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getBooleanOpaque:(Ljava/lang/Object;J)Z")
        }

        #[java_method(name = "getByteOpaque", descriptor = "(Ljava/lang/Object;J)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getByteOpaque(&self, o: Object, offset: i64) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getByteOpaque:(Ljava/lang/Object;J)B")
        }

        #[java_method(name = "getShortOpaque", descriptor = "(Ljava/lang/Object;J)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortOpaque(&self, o: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getShortOpaque:(Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getCharOpaque", descriptor = "(Ljava/lang/Object;J)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharOpaque(&self, o: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getCharOpaque:(Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getIntOpaque", descriptor = "(Ljava/lang/Object;J)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntOpaque(&self, o: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getIntOpaque:(Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getFloatOpaque", descriptor = "(Ljava/lang/Object;J)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloatOpaque(&self, o: Object, offset: i64) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getFloatOpaque:(Ljava/lang/Object;J)F")
        }

        #[java_method(name = "getLongOpaque", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongOpaque(&self, o: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getLongOpaque:(Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getDoubleOpaque", descriptor = "(Ljava/lang/Object;J)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDoubleOpaque(&self, o: Object, offset: i64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getDoubleOpaque:(Ljava/lang/Object;J)D")
        }

        #[java_method(name = "putReferenceOpaque", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putReferenceOpaque(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putReferenceOpaque:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "putBooleanOpaque", descriptor = "(Ljava/lang/Object;JZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putBooleanOpaque(&self, o: Object, offset: i64, arg2: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putBooleanOpaque:(Ljava/lang/Object;JZ)V")
        }

        #[java_method(name = "putByteOpaque", descriptor = "(Ljava/lang/Object;JB)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putByteOpaque(&self, o: Object, offset: i64, arg2: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putByteOpaque:(Ljava/lang/Object;JB)V")
        }

        #[java_method(name = "putShortOpaque", descriptor = "(Ljava/lang/Object;JS)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortOpaque(&self, o: Object, offset: i64, arg2: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShortOpaque:(Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putCharOpaque", descriptor = "(Ljava/lang/Object;JC)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharOpaque(&self, o: Object, offset: i64, arg2: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putCharOpaque:(Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putIntOpaque", descriptor = "(Ljava/lang/Object;JI)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntOpaque(&self, o: Object, offset: i64, arg2: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntOpaque:(Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putFloatOpaque", descriptor = "(Ljava/lang/Object;JF)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloatOpaque(&self, o: Object, offset: i64, arg2: f32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putFloatOpaque:(Ljava/lang/Object;JF)V")
        }

        #[java_method(name = "putLongOpaque", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongOpaque(&self, o: Object, offset: i64, arg2: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongOpaque:(Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putDoubleOpaque", descriptor = "(Ljava/lang/Object;JD)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDoubleOpaque(&self, o: Object, offset: i64, arg2: f64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putDoubleOpaque:(Ljava/lang/Object;JD)V")
        }

        #[native]
        #[java_native(name = "unpark", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn unpark(&self, arg0: Object) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.unpark:(Ljava/lang/Object;)V")
        }

        #[native]
        #[java_native(name = "park", descriptor = "(ZJ)V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn park(&self, arg0: bool, arg1: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.park:(ZJ)V")
        }

        #[java_method(name = "getLoadAverage", descriptor = "([DI)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLoadAverage(&self, loadavg: Rc<RefCell<Vec<f64>>>, nelems: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getLoadAverage:([DI)I")
        }

        #[java_method(name = "getAndAddInt", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddInt(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddInt:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntRelease", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddIntRelease:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddIntAcquire", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddIntAcquire(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddIntAcquire:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndAddLong", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLong(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddLong:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongRelease", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddLongRelease:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddLongAcquire", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddLongAcquire(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddLongAcquire:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndAddByte", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByte(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddByte:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteRelease", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddByteRelease:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddByteAcquire", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddByteAcquire(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddByteAcquire:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndAddShort", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShort(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddShort:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortRelease", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddShortRelease:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddShortAcquire", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddShortAcquire(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddShortAcquire:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndAddChar", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddChar(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddChar:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharRelease", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddCharRelease:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddCharAcquire", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddCharAcquire(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddCharAcquire:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndAddFloat", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloat(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddFloat:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatRelease", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatRelease(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddFloatRelease:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddFloatAcquire", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddFloatAcquire(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddFloatAcquire:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndAddDouble", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDouble(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddDouble:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleRelease", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleRelease(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddDoubleRelease:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndAddDoubleAcquire", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAddDoubleAcquire(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddDoubleAcquire:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetInt", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetInt(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetInt:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntRelease", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetIntRelease:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetIntAcquire", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetIntAcquire(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetIntAcquire:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndSetLong", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLong(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetLong:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongRelease", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetLongRelease:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetLongAcquire", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetLongAcquire(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetLongAcquire:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndSetReference", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetReference(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getAndSetReferenceRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetReferenceRelease(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetReferenceRelease:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getAndSetReferenceAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetReferenceAcquire(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetReferenceAcquire:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getAndSetByte", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetByte(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetByte:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndSetByteRelease", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetByteRelease:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndSetByteAcquire", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetByteAcquire(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetByteAcquire:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndSetBoolean", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetBoolean(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetBoolean:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndSetBooleanRelease", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetBooleanRelease(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetBooleanRelease:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndSetBooleanAcquire", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetBooleanAcquire(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetBooleanAcquire:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndSetShort", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetShort(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetShort:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndSetShortRelease", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetShortRelease:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndSetShortAcquire", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetShortAcquire(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetShortAcquire:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndSetChar", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetChar(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetChar:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndSetCharRelease", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetCharRelease:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndSetCharAcquire", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetCharAcquire(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetCharAcquire:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndSetFloat", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloat(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetFloat:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatRelease", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatRelease(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetFloatRelease:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetFloatAcquire", descriptor = "(Ljava/lang/Object;JF)F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetFloatAcquire(&self, o: Object, offset: i64, arg2: f32) -> Result<f32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetFloatAcquire:(Ljava/lang/Object;JF)F")
        }

        #[java_method(name = "getAndSetDouble", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDouble(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetDouble:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleRelease", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleRelease(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetDoubleRelease:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndSetDoubleAcquire", descriptor = "(Ljava/lang/Object;JD)D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSetDoubleAcquire(&self, o: Object, offset: i64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetDoubleAcquire:(Ljava/lang/Object;JD)D")
        }

        #[java_method(name = "getAndBitwiseOrBoolean", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrBoolean(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrBoolean:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseOrBooleanRelease", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrBooleanRelease(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrBooleanRelease:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseOrBooleanAcquire", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrBooleanAcquire(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrBooleanAcquire:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseAndBoolean", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndBoolean(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndBoolean:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseAndBooleanRelease", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndBooleanRelease(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndBooleanRelease:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseAndBooleanAcquire", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndBooleanAcquire(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndBooleanAcquire:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseXorBoolean", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorBoolean(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorBoolean:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseXorBooleanRelease", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorBooleanRelease(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorBooleanRelease:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseXorBooleanAcquire", descriptor = "(Ljava/lang/Object;JZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorBooleanAcquire(&self, o: Object, offset: i64, arg2: bool) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorBooleanAcquire:(Ljava/lang/Object;JZ)Z")
        }

        #[java_method(name = "getAndBitwiseOrByte", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByte(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrByte:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteRelease", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrByteRelease:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrByteAcquire", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrByteAcquire(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrByteAcquire:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByte", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByte(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndByte:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteRelease", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndByteRelease:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseAndByteAcquire", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndByteAcquire(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndByteAcquire:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByte", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByte(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorByte:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteRelease", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteRelease(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorByteRelease:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseXorByteAcquire", descriptor = "(Ljava/lang/Object;JB)B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorByteAcquire(&self, o: Object, offset: i64, arg2: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorByteAcquire:(Ljava/lang/Object;JB)B")
        }

        #[java_method(name = "getAndBitwiseOrChar", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrChar(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrChar:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharRelease", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrCharRelease:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrCharAcquire", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrCharAcquire(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrCharAcquire:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndChar", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndChar(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndChar:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharRelease", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndCharRelease:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseAndCharAcquire", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndCharAcquire(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndCharAcquire:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorChar", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorChar(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorChar:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharRelease", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharRelease(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorCharRelease:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseXorCharAcquire", descriptor = "(Ljava/lang/Object;JC)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorCharAcquire(&self, o: Object, offset: i64, arg2: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorCharAcquire:(Ljava/lang/Object;JC)C")
        }

        #[java_method(name = "getAndBitwiseOrShort", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShort(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrShort:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortRelease", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrShortRelease:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrShortAcquire", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrShortAcquire(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrShortAcquire:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShort", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShort(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndShort:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortRelease", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndShortRelease:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseAndShortAcquire", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndShortAcquire(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndShortAcquire:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShort", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShort(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorShort:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortRelease", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortRelease(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorShortRelease:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseXorShortAcquire", descriptor = "(Ljava/lang/Object;JS)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorShortAcquire(&self, o: Object, offset: i64, arg2: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorShortAcquire:(Ljava/lang/Object;JS)S")
        }

        #[java_method(name = "getAndBitwiseOrInt", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrInt(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrInt:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntRelease", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrIntRelease:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrIntAcquire", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrIntAcquire(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrIntAcquire:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndInt", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndInt(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndInt:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntRelease", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndIntRelease:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseAndIntAcquire", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndIntAcquire(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndIntAcquire:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorInt", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorInt(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorInt:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntRelease", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntRelease(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorIntRelease:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseXorIntAcquire", descriptor = "(Ljava/lang/Object;JI)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorIntAcquire(&self, o: Object, offset: i64, arg2: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorIntAcquire:(Ljava/lang/Object;JI)I")
        }

        #[java_method(name = "getAndBitwiseOrLong", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLong(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrLong:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongRelease", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrLongRelease:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseOrLongAcquire", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseOrLongAcquire(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrLongAcquire:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLong", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLong(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndLong:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongRelease", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndLongRelease:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseAndLongAcquire", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseAndLongAcquire(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndLongAcquire:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLong", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLong(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorLong:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongRelease", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongRelease(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorLongRelease:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "getAndBitwiseXorLongAcquire", descriptor = "(Ljava/lang/Object;JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndBitwiseXorLongAcquire(&self, o: Object, offset: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseXorLongAcquire:(Ljava/lang/Object;JJ)J")
        }

        #[java_method(name = "loadFence", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadFence(&self) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.loadFence:()V")
        }

        #[java_method(name = "storeFence", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn storeFence(&self) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.storeFence:()V")
        }

        #[native]
        #[java_native(name = "fullFence", descriptor = "()V", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn fullFence(&self) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.fullFence:()V")
        }

        #[java_method(name = "loadLoadFence", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadLoadFence(&self) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.loadLoadFence:()V")
        }

        #[java_method(name = "storeStoreFence", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn storeStoreFence(&self) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.storeStoreFence:()V")
        }

        #[java_method(name = "throwIllegalAccessError", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwIllegalAccessError() -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.throwIllegalAccessError:()V")
        }

        #[java_method(name = "throwNoSuchMethodError", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwNoSuchMethodError() -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.throwNoSuchMethodError:()V")
        }

        #[java_method(name = "isBigEndian", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBigEndian(&self) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.isBigEndian:()Z")
        }

        #[java_method(name = "unalignedAccess", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unalignedAccess(&self) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.unalignedAccess:()Z")
        }

        #[java_method(name = "getLongUnaligned", descriptor = "(Ljava/lang/Object;J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongUnaligned_obj_l(&self, o: Object, offset: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getLongUnaligned:(Ljava/lang/Object;J)J")
        }

        #[java_method(name = "getLongUnaligned", descriptor = "(Ljava/lang/Object;JZ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLongUnaligned_obj_l_z(&self, o: Object, offset: i64, arg2: bool) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.getLongUnaligned:(Ljava/lang/Object;JZ)J")
        }

        #[java_method(name = "getIntUnaligned", descriptor = "(Ljava/lang/Object;J)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntUnaligned_obj_l(&self, o: Object, offset: i64) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getIntUnaligned:(Ljava/lang/Object;J)I")
        }

        #[java_method(name = "getIntUnaligned", descriptor = "(Ljava/lang/Object;JZ)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntUnaligned_obj_l_z(&self, o: Object, offset: i64, arg2: bool) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.getIntUnaligned:(Ljava/lang/Object;JZ)I")
        }

        #[java_method(name = "getShortUnaligned", descriptor = "(Ljava/lang/Object;J)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortUnaligned_obj_l(&self, o: Object, offset: i64) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getShortUnaligned:(Ljava/lang/Object;J)S")
        }

        #[java_method(name = "getShortUnaligned", descriptor = "(Ljava/lang/Object;JZ)S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortUnaligned_obj_l_z(&self, o: Object, offset: i64, arg2: bool) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.getShortUnaligned:(Ljava/lang/Object;JZ)S")
        }

        #[java_method(name = "getCharUnaligned", descriptor = "(Ljava/lang/Object;J)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharUnaligned_obj_l(&self, o: Object, offset: i64) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getCharUnaligned:(Ljava/lang/Object;J)C")
        }

        #[java_method(name = "getCharUnaligned", descriptor = "(Ljava/lang/Object;JZ)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharUnaligned_obj_l_z(&self, o: Object, offset: i64, arg2: bool) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.getCharUnaligned:(Ljava/lang/Object;JZ)C")
        }

        #[java_method(name = "putLongUnaligned", descriptor = "(Ljava/lang/Object;JJ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongUnaligned_obj_l_l(&self, o: Object, offset: i64, arg2: i64) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongUnaligned:(Ljava/lang/Object;JJ)V")
        }

        #[java_method(name = "putLongUnaligned", descriptor = "(Ljava/lang/Object;JJZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongUnaligned_obj_l_l_z(&self, o: Object, offset: i64, arg2: i64, x: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongUnaligned:(Ljava/lang/Object;JJZ)V")
        }

        #[java_method(name = "putIntUnaligned", descriptor = "(Ljava/lang/Object;JI)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntUnaligned_obj_l_i(&self, o: Object, offset: i64, arg2: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntUnaligned:(Ljava/lang/Object;JI)V")
        }

        #[java_method(name = "putIntUnaligned", descriptor = "(Ljava/lang/Object;JIZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntUnaligned_obj_l_i_z(&self, o: Object, offset: i64, arg2: i32, x: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntUnaligned:(Ljava/lang/Object;JIZ)V")
        }

        #[java_method(name = "putShortUnaligned", descriptor = "(Ljava/lang/Object;JS)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortUnaligned_obj_l_s(&self, o: Object, offset: i64, arg2: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShortUnaligned:(Ljava/lang/Object;JS)V")
        }

        #[java_method(name = "putShortUnaligned", descriptor = "(Ljava/lang/Object;JSZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortUnaligned_obj_l_s_z(&self, o: Object, offset: i64, arg2: i16, x: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShortUnaligned:(Ljava/lang/Object;JSZ)V")
        }

        #[java_method(name = "putCharUnaligned", descriptor = "(Ljava/lang/Object;JC)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharUnaligned_obj_l_c(&self, o: Object, offset: i64, arg2: u16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putCharUnaligned:(Ljava/lang/Object;JC)V")
        }

        #[java_method(name = "putCharUnaligned", descriptor = "(Ljava/lang/Object;JCZ)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharUnaligned_obj_l_c_z(&self, o: Object, offset: i64, arg2: u16, x: bool) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putCharUnaligned:(Ljava/lang/Object;JCZ)V")
        }

        #[java_method(name = "pickPos", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pickPos(top: i32, pos: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.pickPos:(II)I")
        }

        #[java_method(name = "makeLong", descriptor = "(BBBBBBBB)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeLong_b_b_b_b_b_b_b_b(i0: i8, i1: i8, i2: i8, i3: i8, i4: i8, i5: i8, i6: i8, i7: i8) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.makeLong:(BBBBBBBB)J")
        }

        #[java_method(name = "makeLong", descriptor = "(SSSS)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeLong_s_s_s_s(i0: i16, i1: i16, i2: i16, i3: i16) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.makeLong:(SSSS)J")
        }

        #[java_method(name = "makeLong", descriptor = "(II)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeLong_i_i(i0: i32, i1: i32) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.makeLong:(II)J")
        }

        #[java_method(name = "makeInt", descriptor = "(SS)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeInt_s_s(i0: i16, i1: i16) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.makeInt:(SS)I")
        }

        #[java_method(name = "makeInt", descriptor = "(BBBB)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeInt_b_b_b_b(i0: i8, i1: i8, i2: i8, i3: i8) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.makeInt:(BBBB)I")
        }

        #[java_method(name = "makeShort", descriptor = "(BB)S", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn makeShort(i0: i8, i1: i8) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.makeShort:(BB)S")
        }

        #[java_method(name = "pick", descriptor = "(BB)B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pick_b_b(le: i8, be: i8) -> Result<i8> {
            panic!("stub: jdk/internal/misc/Unsafe.pick:(BB)B")
        }

        #[java_method(name = "pick", descriptor = "(SS)S", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pick_s_s(le: i16, be: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.pick:(SS)S")
        }

        #[java_method(name = "pick", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pick_i_i(le: i32, be: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.pick:(II)I")
        }

        #[java_method(name = "putLongParts", descriptor = "(Ljava/lang/Object;JBBBBBBBB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongParts_obj_l_b_b_b_b_b_b_b_b(&self, o: Object, offset: i64, arg2: i8, i0: i8, i1: i8, i2: i8, i3: i8, i4: i8, i5: i8, i6: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongParts:(Ljava/lang/Object;JBBBBBBBB)V")
        }

        #[java_method(name = "putLongParts", descriptor = "(Ljava/lang/Object;JSSSS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongParts_obj_l_s_s_s_s(&self, o: Object, offset: i64, arg2: i16, i0: i16, i1: i16, i2: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongParts:(Ljava/lang/Object;JSSSS)V")
        }

        #[java_method(name = "putLongParts", descriptor = "(Ljava/lang/Object;JII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLongParts_obj_l_i_i(&self, o: Object, offset: i64, arg2: i32, i0: i32) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putLongParts:(Ljava/lang/Object;JII)V")
        }

        #[java_method(name = "putIntParts", descriptor = "(Ljava/lang/Object;JSS)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntParts_obj_l_s_s(&self, o: Object, offset: i64, arg2: i16, i0: i16) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntParts:(Ljava/lang/Object;JSS)V")
        }

        #[java_method(name = "putIntParts", descriptor = "(Ljava/lang/Object;JBBBB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putIntParts_obj_l_b_b_b_b(&self, o: Object, offset: i64, arg2: i8, i0: i8, i1: i8, i2: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putIntParts:(Ljava/lang/Object;JBBBB)V")
        }

        #[java_method(name = "putShortParts", descriptor = "(Ljava/lang/Object;JBB)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShortParts(&self, o: Object, offset: i64, arg2: i8, i0: i8) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putShortParts:(Ljava/lang/Object;JBB)V")
        }

        #[java_method(name = "toUnsignedInt", descriptor = "(B)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedInt_b(n: i8) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.toUnsignedInt:(B)I")
        }

        #[java_method(name = "toUnsignedInt", descriptor = "(S)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedInt_s(n: i16) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.toUnsignedInt:(S)I")
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(B)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong_b(n: i8) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.toUnsignedLong:(B)J")
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(S)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong_s(n: i16) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.toUnsignedLong:(S)J")
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(I)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong_i(n: i32) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.toUnsignedLong:(I)J")
        }

        #[java_method(name = "convEndian", descriptor = "(ZC)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convEndian_z_c(big: bool, n: u16) -> Result<u16> {
            panic!("stub: jdk/internal/misc/Unsafe.convEndian:(ZC)C")
        }

        #[java_method(name = "convEndian", descriptor = "(ZS)S", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convEndian_z_s(big: bool, n: i16) -> Result<i16> {
            panic!("stub: jdk/internal/misc/Unsafe.convEndian:(ZS)S")
        }

        #[java_method(name = "convEndian", descriptor = "(ZI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convEndian_z_i(big: bool, n: i32) -> Result<i32> {
            panic!("stub: jdk/internal/misc/Unsafe.convEndian:(ZI)I")
        }

        #[java_method(name = "convEndian", descriptor = "(ZJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convEndian_z_l(big: bool, n: i64) -> Result<i64> {
            panic!("stub: jdk/internal/misc/Unsafe.convEndian:(ZJ)J")
        }

        #[native]
        #[java_native(name = "allocateMemory0", descriptor = "(J)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn allocateMemory0(&self, arg0: i64) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.allocateMemory0:(J)J")
        }

        #[native]
        #[java_native(name = "reallocateMemory0", descriptor = "(JJ)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn reallocateMemory0(&self, arg0: i64, arg1: i64) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.reallocateMemory0:(JJ)J")
        }

        #[native]
        #[java_native(name = "freeMemory0", descriptor = "(J)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn freeMemory0(&self, arg0: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.freeMemory0:(J)V")
        }

        #[native]
        #[java_native(name = "setMemory0", descriptor = "(Ljava/lang/Object;JJB)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setMemory0(&self, arg0: Object, arg1: i64, arg2: i64, arg3: i8) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.setMemory0:(Ljava/lang/Object;JJB)V")
        }

        #[native]
        #[java_native(name = "copyMemory0", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJ)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn copyMemory0(&self, arg0: Object, arg1: i64, arg2: Object, arg3: i64, arg4: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.copyMemory0:(Ljava/lang/Object;JLjava/lang/Object;JJ)V")
        }

        #[native]
        #[java_native(name = "copySwapMemory0", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn copySwapMemory0(&self, arg0: Object, arg1: i64, arg2: Object, arg3: i64, arg4: i64, arg5: i64) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.copySwapMemory0:(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
        }

        #[native]
        #[java_native(name = "objectFieldOffset0", descriptor = "(Ljava/lang/reflect/Field;)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn objectFieldOffset0(&self, arg0: Object) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.objectFieldOffset0:(Ljava/lang/reflect/Field;)J")
        }

        #[native]
        #[java_native(name = "objectFieldOffset1", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;)J")]
        pub fn objectFieldOffset1(&self, arg0: Object, arg1: String) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.objectFieldOffset1:(Ljava/lang/Class;Ljava/lang/String;)J")
        }

        #[native]
        #[java_native(name = "staticFieldOffset0", descriptor = "(Ljava/lang/reflect/Field;)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn staticFieldOffset0(&self, arg0: Object) -> Result<i64> {
            panic!("native: jdk/internal/misc/Unsafe.staticFieldOffset0:(Ljava/lang/reflect/Field;)J")
        }

        #[native]
        #[java_native(name = "staticFieldBase0", descriptor = "(Ljava/lang/reflect/Field;)Ljava/lang/Object;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn staticFieldBase0(&self, arg0: Object) -> Result<Object> {
            panic!("native: jdk/internal/misc/Unsafe.staticFieldBase0:(Ljava/lang/reflect/Field;)Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "shouldBeInitialized0", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn shouldBeInitialized0(&self, arg0: Object) -> Result<bool> {
            panic!("native: jdk/internal/misc/Unsafe.shouldBeInitialized0:(Ljava/lang/Class;)Z")
        }

        #[native]
        #[java_native(name = "ensureClassInitialized0", descriptor = "(Ljava/lang/Class;)V", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn ensureClassInitialized0(&self, arg0: Object) -> Result<()> {
            panic!("native: jdk/internal/misc/Unsafe.ensureClassInitialized0:(Ljava/lang/Class;)V")
        }

        #[native]
        #[java_native(name = "arrayBaseOffset0", descriptor = "(Ljava/lang/Class;)I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)I")]
        pub fn arrayBaseOffset0(&self, arg0: Object) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.arrayBaseOffset0:(Ljava/lang/Class;)I")
        }

        #[native]
        #[java_native(name = "arrayIndexScale0", descriptor = "(Ljava/lang/Class;)I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)I")]
        pub fn arrayIndexScale0(&self, arg0: Object) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.arrayIndexScale0:(Ljava/lang/Class;)I")
        }

        #[native]
        #[java_native(name = "getLoadAverage0", descriptor = "([DI)I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getLoadAverage0(&self, arg0: Rc<RefCell<Vec<f64>>>, arg1: i32) -> Result<i32> {
            panic!("native: jdk/internal/misc/Unsafe.getLoadAverage0:([DI)I")
        }

        #[java_method(name = "invokeCleaner", descriptor = "(Ljava/nio/ByteBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn invokeCleaner(&self, directBuffer: ByteBuffer) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.invokeCleaner:(Ljava/nio/ByteBuffer;)V")
        }

        #[java_method(name = "getObject", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getObject(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getObject:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "getObjectVolatile", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getObjectVolatile(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getObjectVolatile:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "getObjectAcquire", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getObjectAcquire(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getObjectAcquire:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "getObjectOpaque", descriptor = "(Ljava/lang/Object;J)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getObjectOpaque(&self, o: Object, offset: i64) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getObjectOpaque:(Ljava/lang/Object;J)Ljava/lang/Object;")
        }

        #[java_method(name = "putObject", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn putObject(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putObject:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "putObjectVolatile", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn putObjectVolatile(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putObjectVolatile:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "putObjectOpaque", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn putObjectOpaque(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putObjectOpaque:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "putObjectRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn putObjectRelease(&self, o: Object, offset: i64, arg2: Object) -> Result<()> {
            panic!("stub: jdk/internal/misc/Unsafe.putObjectRelease:(Ljava/lang/Object;JLjava/lang/Object;)V")
        }

        #[java_method(name = "getAndSetObject", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getAndSetObject(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetObject:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getAndSetObjectAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getAndSetObjectAcquire(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetObjectAcquire:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getAndSetObjectRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getAndSetObjectRelease(&self, o: Object, offset: i64, arg2: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetObjectRelease:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "compareAndSetObject", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn compareAndSetObject(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetObject:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareAndExchangeObject", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn compareAndExchangeObject(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeObject:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "compareAndExchangeObjectAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn compareAndExchangeObjectAcquire(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeObjectAcquire:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "compareAndExchangeObjectRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn compareAndExchangeObjectRelease(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<Object> {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeObjectRelease:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "weakCompareAndSetObject", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSetObject(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetObject:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetObjectAcquire", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSetObjectAcquire(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetObjectAcquire:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetObjectPlain", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSetObjectPlain(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetObjectPlain:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "weakCompareAndSetObjectRelease", descriptor = "(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSetObjectRelease(&self, o: Object, offset: i64, arg2: Object, expected: Object) -> Result<bool> {
            panic!("stub: jdk/internal/misc/Unsafe.weakCompareAndSetObjectRelease:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z")
        }
    }
}
