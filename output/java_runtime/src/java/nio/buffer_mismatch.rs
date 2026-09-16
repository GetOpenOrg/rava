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
use crate::jdk::internal::misc::ScopedMemoryAccess;
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/BufferMismatch"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BufferMismatch.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/BufferMismatch"]

    pub struct BufferMismatch;

    impl BufferMismatch {
        #[cfg_attr(any(), java_field(name = "SCOPED_MEMORY_ACCESS", descriptor = "Ljdk/internal/misc/ScopedMemoryAccess;", access = "package", modifiers = "static final", is_static = true))]
        // static field: SCOPED_MEMORY_ACCESS:Ljdk/internal/misc/ScopedMemoryAccess;
        pub fn SCOPED_MEMORY_ACCESS() -> ScopedMemoryAccess {
            panic!("stub: java/nio/BufferMismatch.SCOPED_MEMORY_ACCESS:Ljdk/internal/misc/ScopedMemoryAccess;")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/nio/BufferMismatch.<init>:()V")
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/ByteBuffer;ILjava/nio/ByteBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mismatch(Ljava/nio/ByteBuffer;ILjava/nio/ByteBuffer;II)I
        pub fn mismatch_bytebu_i_bytebu_i_i(mut a: ByteBuffer, mut aOff: i32, mut b: ByteBuffer, mut bOff: i32, mut length: i32) -> Result<i32> {
            let mut i: i32 = 0i32;
            let _t0 = a.get_i(aOff)?;
            let _t1 = b.get_i(bOff)?;
            if (_t0 as i32) != (_t1 as i32) {
                return Ok(0i32);
            }
            let _t2 = a.__super().session()?;
            let _t3 = b.__super().session()?;
            let _t4 = a.base()?;
            let _t5 = b.base()?;
            let _t6 = BufferMismatch::SCOPED_MEMORY_ACCESS().vectorizedMismatch(Clone::clone(&_t2), Clone::clone(&_t3), Clone::clone(&_t4), (a.__get_address()).wrapping_add((aOff as i64)), Clone::clone(&_t5), (b.__get_address()).wrapping_add((bOff as i64)), length, ArraysSupport::LOG2_ARRAY_BYTE_INDEX_SCALE())?;
            i = _t6;
            if (i>=0) {
                return Ok(i);
            }
            i = (length).wrapping_sub((i^-1i32));
            loop {
                if i >= length { break; }
                let _t7 = a.get_i((aOff).wrapping_add(i))?;
                let _t8 = b.get_i((bOff).wrapping_add(i))?;
                if (_t7 as i32) != (_t8 as i32) {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/CharBuffer;ILjava/nio/CharBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mismatch(Ljava/nio/CharBuffer;ILjava/nio/CharBuffer;II)I
        pub fn mismatch_charbu_i_charbu_i_i(mut a: CharBuffer, mut aOff: i32, mut b: CharBuffer, mut bOff: i32, mut length: i32) -> Result<i32> {
            let mut i: i32 = 0i32;
            let _t0 = a.charRegionOrder()?;
            let _t1 = b.charRegionOrder()?;
            let _t2 = a.charRegionOrder()?;
            let _t3 = b.charRegionOrder()?;
            let _t4 = a.get_i(aOff)?;
            let _t5 = b.get_i(bOff)?;
            if (_t4 as i32) != (_t5 as i32) {
                return Ok(0i32);
            }
            let _t6 = a.__super().session()?;
            let _t7 = b.__super().session()?;
            let _t8 = a.base()?;
            let _t9 = b.base()?;
            let _t10 = BufferMismatch::SCOPED_MEMORY_ACCESS().vectorizedMismatch(Clone::clone(&_t6), Clone::clone(&_t7), Clone::clone(&_t8), (a.__get_address()).wrapping_add(((aOff<<(ArraysSupport::LOG2_ARRAY_CHAR_INDEX_SCALE()&0x1f)) as i64)), Clone::clone(&_t9), (b.__get_address()).wrapping_add(((bOff<<(ArraysSupport::LOG2_ARRAY_CHAR_INDEX_SCALE()&0x1f)) as i64)), length, ArraysSupport::LOG2_ARRAY_CHAR_INDEX_SCALE())?;
            i = _t10;
            if (i>=0) {
                return Ok(i);
            }
            i = (length).wrapping_sub((i^-1i32));
            loop {
                if i >= length { break; }
                let _t11 = a.get_i((aOff).wrapping_add(i))?;
                let _t12 = b.get_i((bOff).wrapping_add(i))?;
                if (_t11 as i32) != (_t12 as i32) {
                    return Ok(i);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(-1i32)
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/ShortBuffer;ILjava/nio/ShortBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_shortb_i_shortb_i_i(a: Object, aOff: i32, b: Object, bOff: i32, length: i32) -> Result<i32> {
            panic!("stub: java/nio/BufferMismatch.mismatch:(Ljava/nio/ShortBuffer;ILjava/nio/ShortBuffer;II)I")
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/IntBuffer;ILjava/nio/IntBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_intbuf_i_intbuf_i_i(a: Object, aOff: i32, b: Object, bOff: i32, length: i32) -> Result<i32> {
            panic!("stub: java/nio/BufferMismatch.mismatch:(Ljava/nio/IntBuffer;ILjava/nio/IntBuffer;II)I")
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/FloatBuffer;ILjava/nio/FloatBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_floatb_i_floatb_i_i(a: Object, aOff: i32, b: Object, bOff: i32, length: i32) -> Result<i32> {
            panic!("stub: java/nio/BufferMismatch.mismatch:(Ljava/nio/FloatBuffer;ILjava/nio/FloatBuffer;II)I")
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/LongBuffer;ILjava/nio/LongBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_longbu_i_longbu_i_i(a: Object, aOff: i32, b: Object, bOff: i32, length: i32) -> Result<i32> {
            panic!("stub: java/nio/BufferMismatch.mismatch:(Ljava/nio/LongBuffer;ILjava/nio/LongBuffer;II)I")
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/DoubleBuffer;ILjava/nio/DoubleBuffer;II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_double_i_double_i_i(a: Object, aOff: i32, b: Object, bOff: i32, length: i32) -> Result<i32> {
            panic!("stub: java/nio/BufferMismatch.mismatch:(Ljava/nio/DoubleBuffer;ILjava/nio/DoubleBuffer;II)I")
        }
    }
}
