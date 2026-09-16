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
use crate::jdk::internal::misc::Unsafe;

impl From<HeapByteBuffer> for ByteBuffer {
    fn from(v: HeapByteBuffer) -> ByteBuffer { v.__into_super() }
}

impl From<HeapByteBuffer> for Buffer {
    fn from(v: HeapByteBuffer) -> Buffer { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/HeapByteBuffer"]
    #[super_class       = "java/nio/ByteBuffer"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HeapByteBuffer.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ByteBuffer"]
    #[superclass_fields(mark: i32, position: i32, limit: i32, capacity: i32, address: i64, segment: Object, hb: Rc<RefCell<Vec<i8>>>, offset: i32, isReadOnly: bool, bigEndian: bool, nativeByteOrder: bool)]
    #[all_supertypes    = "java/lang/Comparable;java/lang/Object;java/nio/Buffer;java/nio/ByteBuffer;java/nio/HeapByteBuffer"]

    pub struct HeapByteBuffer;

    impl HeapByteBuffer {
        #[cfg_attr(any(), java_field(name = "ARRAY_BASE_OFFSET", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BASE_OFFSET:J
        pub fn ARRAY_BASE_OFFSET() -> i64 {
            panic!("stub: java/nio/HeapByteBuffer.ARRAY_BASE_OFFSET:J")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_INDEX_SCALE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_INDEX_SCALE:J
        pub fn ARRAY_INDEX_SCALE() -> i64 {
            panic!("stub: java/nio/HeapByteBuffer.ARRAY_INDEX_SCALE:J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(IILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_memory(cap: i32, lim: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/HeapByteBuffer.<init>:(IILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "<init>", descriptor = "([BIILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([BIILjava/lang/foreign/MemorySegment;)V
        pub fn new_arr_b_i_i_memory(mut buf: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32, mut segment: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ByteBuffer::new_i_i_i_i_arr_b_i_memory(-1i32, off, (off).wrapping_add(len), (buf.borrow().len() as i32), Clone::clone(&buf), 0i32, Clone::clone(&segment))?);
            this.__set_address(HeapByteBuffer::ARRAY_BASE_OFFSET());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([BIIIIILjava/lang/foreign/MemorySegment;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_i_i_i_i_i_memory(buf: Rc<RefCell<Vec<i8>>>, mark: i32, pos: i32, lim: i32, cap: i32, off: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/HeapByteBuffer.<init>:([BIIIIILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "slice", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn slice(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.slice:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "slice", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn slice_i_i(&self, index: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.slice:(II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "duplicate", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn duplicate(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.duplicate:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asReadOnlyBuffer(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.asReadOnlyBuffer:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "ix", descriptor = "(I)I", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ix(&self, i: i32) -> Result<i32> {
            panic!("stub: java/nio/HeapByteBuffer.ix:(I)I")
        }

        #[java_method(name = "byteOffset", descriptor = "(J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteOffset(&self, i: i64) -> Result<i64> {
            panic!("stub: java/nio/HeapByteBuffer.byteOffset:(J)J")
        }

        #[java_method(name = "get", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<i8> {
            panic!("stub: java/nio/HeapByteBuffer.get:()B")
        }

        #[java_method(name = "get", descriptor = "(I)B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i(&self, i: i32) -> Result<i8> {
            panic!("stub: java/nio/HeapByteBuffer.get:(I)B")
        }

        #[java_method(name = "get", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_b_i_i(&self, dst: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.get:([BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_b_i_i(&self, index: i32, dst: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.get:(I[BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "isDirect", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDirect(&self) -> Result<bool> {
            panic!("stub: java/nio/HeapByteBuffer.isDirect:()Z")
        }

        #[java_method(name = "isReadOnly", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isReadOnly(&self) -> Result<bool> {
            panic!("stub: java/nio/HeapByteBuffer.isReadOnly:()Z")
        }

        #[java_method(name = "put", descriptor = "(B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_b(&self, x: i8) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:(B)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(IB)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_b(&self, i: i32, x: i8) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:(IB)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_arr_b_i_i(&self, src: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:([BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_bytebu(&self, src: ByteBuffer) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_bytebu_i_i(&self, index: i32, src: ByteBuffer, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_b_i_i(&self, index: i32, src: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.put:(I[BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "compact", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compact(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.compact:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "_get", descriptor = "(I)B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn _get(&self, i: i32) -> Result<i8> {
            panic!("stub: java/nio/HeapByteBuffer._get:(I)B")
        }

        #[java_method(name = "_put", descriptor = "(IB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn _put(&self, i: i32, b: i8) -> Result<()> {
            panic!("stub: java/nio/HeapByteBuffer._put:(IB)V")
        }

        #[java_method(name = "getChar", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar(&self) -> Result<u16> {
            panic!("stub: java/nio/HeapByteBuffer.getChar:()C")
        }

        #[java_method(name = "getChar", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChar_i(&self, i: i32) -> Result<u16> {
            panic!("stub: java/nio/HeapByteBuffer.getChar:(I)C")
        }

        #[java_method(name = "putChar", descriptor = "(C)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar_c(&self, x: u16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putChar:(C)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putChar", descriptor = "(IC)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putChar_i_c(&self, i: i32, x: u16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putChar:(IC)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asCharBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asCharBuffer(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.asCharBuffer:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "getShort", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShort(&self) -> Result<i16> {
            panic!("stub: java/nio/HeapByteBuffer.getShort:()S")
        }

        #[java_method(name = "getShort", descriptor = "(I)S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShort_i(&self, i: i32) -> Result<i16> {
            panic!("stub: java/nio/HeapByteBuffer.getShort:(I)S")
        }

        #[java_method(name = "putShort", descriptor = "(S)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShort_s(&self, x: i16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putShort:(S)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putShort", descriptor = "(IS)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putShort_i_s(&self, i: i32, x: i16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putShort:(IS)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asShortBuffer", descriptor = "()Ljava/nio/ShortBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asShortBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/HeapByteBuffer.asShortBuffer:()Ljava/nio/ShortBuffer;")
        }

        #[java_method(name = "getInt", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt(&self) -> Result<i32> {
            panic!("stub: java/nio/HeapByteBuffer.getInt:()I")
        }

        #[java_method(name = "getInt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt_i(&self, i: i32) -> Result<i32> {
            panic!("stub: java/nio/HeapByteBuffer.getInt:(I)I")
        }

        #[java_method(name = "putInt", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putInt_i(&self, x: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putInt:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putInt", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putInt_i_i(&self, i: i32, x: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putInt:(II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asIntBuffer", descriptor = "()Ljava/nio/IntBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asIntBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/HeapByteBuffer.asIntBuffer:()Ljava/nio/IntBuffer;")
        }

        #[java_method(name = "getLong", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self) -> Result<i64> {
            panic!("stub: java/nio/HeapByteBuffer.getLong:()J")
        }

        #[java_method(name = "getLong", descriptor = "(I)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong_i(&self, i: i32) -> Result<i64> {
            panic!("stub: java/nio/HeapByteBuffer.getLong:(I)J")
        }

        #[java_method(name = "putLong", descriptor = "(J)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLong_l(&self, x: i64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putLong:(J)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putLong", descriptor = "(IJ)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putLong_i_l(&self, i: i32, x: i64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putLong:(IJ)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asLongBuffer", descriptor = "()Ljava/nio/LongBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asLongBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/HeapByteBuffer.asLongBuffer:()Ljava/nio/LongBuffer;")
        }

        #[java_method(name = "getFloat", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat(&self) -> Result<f32> {
            panic!("stub: java/nio/HeapByteBuffer.getFloat:()F")
        }

        #[java_method(name = "getFloat", descriptor = "(I)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFloat_i(&self, i: i32) -> Result<f32> {
            panic!("stub: java/nio/HeapByteBuffer.getFloat:(I)F")
        }

        #[java_method(name = "putFloat", descriptor = "(F)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloat_f(&self, x: f32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putFloat:(F)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putFloat", descriptor = "(IF)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putFloat_i_f(&self, i: i32, x: f32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putFloat:(IF)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asFloatBuffer", descriptor = "()Ljava/nio/FloatBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asFloatBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/HeapByteBuffer.asFloatBuffer:()Ljava/nio/FloatBuffer;")
        }

        #[java_method(name = "getDouble", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble(&self) -> Result<f64> {
            panic!("stub: java/nio/HeapByteBuffer.getDouble:()D")
        }

        #[java_method(name = "getDouble", descriptor = "(I)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDouble_i(&self, i: i32) -> Result<f64> {
            panic!("stub: java/nio/HeapByteBuffer.getDouble:(I)D")
        }

        #[java_method(name = "putDouble", descriptor = "(D)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDouble_d(&self, x: f64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putDouble:(D)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putDouble", descriptor = "(ID)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putDouble_i_d(&self, i: i32, x: f64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/HeapByteBuffer.putDouble:(ID)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asDoubleBuffer", descriptor = "()Ljava/nio/DoubleBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asDoubleBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/HeapByteBuffer.asDoubleBuffer:()Ljava/nio/DoubleBuffer;")
        }
    }
}
