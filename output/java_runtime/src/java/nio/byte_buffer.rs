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

impl From<ByteBuffer> for Buffer {
    fn from(v: ByteBuffer) -> Buffer { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/ByteBuffer"]
    #[super_class       = "java/nio/Buffer"]
    #[interfaces        = "java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "Ljava/nio/Buffer;Ljava/lang/Comparable<Ljava/nio/ByteBuffer;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ByteBuffer.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Buffer"]
    #[superclass_fields(mark: i32, position: i32, limit: i32, capacity: i32, address: i64, segment: Object)]
    #[all_supertypes    = "java/lang/Comparable;java/lang/Object;java/nio/Buffer;java/nio/ByteBuffer"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ByteBuffer {
        #[cfg_attr(any(), java_field(name = "hb", descriptor = "[B", access = "package", modifiers = "final", is_static = false))]
        pub hb: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "isReadOnly", descriptor = "Z", is_static = false))]
        pub isReadOnly: bool,
        #[cfg_attr(any(), java_field(name = "bigEndian", descriptor = "Z", is_static = false))]
        pub bigEndian: bool,
        #[cfg_attr(any(), java_field(name = "nativeByteOrder", descriptor = "Z", is_static = false))]
        pub nativeByteOrder: bool,
    }

    impl ByteBuffer {
        #[cfg_attr(any(), java_field(name = "ARRAY_BASE_OFFSET", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BASE_OFFSET:J
        pub fn ARRAY_BASE_OFFSET() -> i64 {
            panic!("stub: java/nio/ByteBuffer.ARRAY_BASE_OFFSET:J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(IIII[BILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IIII[BILjava/lang/foreign/MemorySegment;)V
        pub fn new_i_i_i_i_arr_b_i_memory(mut mark: i32, mut pos: i32, mut lim: i32, mut cap: i32, mut hb: Rc<RefCell<Vec<i8>>>, mut offset: i32, mut segment: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Buffer::new_i_i_i_i_memory(mark, pos, lim, cap, Clone::clone(&segment))?);
            this.__set_bigEndian((1i32 != 0i32));
            let _t0: ByteOrder = ByteOrder::nativeOrder()?;
            this.__set_nativeByteOrder(Object::from_any(_t0.clone()) == Object::from_any(ByteOrder::BIG_ENDIAN().clone()));
            this.__set_hb(Clone::clone(&hb));
            this.__set_offset(offset);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(IIIILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/ByteBuffer.<init>:(IIIILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "<init>", descriptor = "([BJILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_l_i_memory(hb: Rc<RefCell<Vec<i8>>>, addr: i64, arg2: i32, cap: Object) -> Result<Self> {
            panic!("stub: java/nio/ByteBuffer.<init>:([BJILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "base", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn base(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(this.__get_hb().clone()))
        }

        #[java_method(name = "allocateDirect", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allocateDirect(capacity: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.allocateDirect:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "allocate", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allocate(capacity: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.allocate:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "wrap", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn wrap_arr_b_i_i(array: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.wrap:([BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "wrap", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: wrap([B)Ljava/nio/ByteBuffer;
        pub fn wrap_arr_b(mut array: Rc<RefCell<Vec<i8>>>) -> Result<ByteBuffer> {
            let _t0: ByteBuffer = ByteBuffer::wrap_arr_b_i_i(Clone::clone(&array), 0i32, (array.borrow().len() as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "slice", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.slice:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "slice", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice_i_i(&self, arg0: i32, arg1: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.slice:(II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "duplicate", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn duplicate(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.duplicate:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asReadOnlyBuffer(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.asReadOnlyBuffer:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "()B", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn get(&self) -> Result<i8> {
            panic!("stub: java/nio/ByteBuffer.get:()B")
        }

        #[java_method(name = "put", descriptor = "(B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn put_b(&self, arg0: i8) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(B)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I)B", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn get_i(&self, arg0: i32) -> Result<i8> {
            panic!("stub: java/nio/ByteBuffer.get:(I)B")
        }

        #[java_method(name = "put", descriptor = "(IB)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn put_i_b(&self, arg0: i32, arg1: i8) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(IB)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_b_i_i(&self, dst: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.get:([BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_b(&self, dst: Rc<RefCell<Vec<i8>>>) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.get:([B)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_b_i_i(&self, index: i32, dst: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.get:(I[BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_b(&self, index: i32, dst: Rc<RefCell<Vec<i8>>>) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.get:(I[B)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getArray", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getArray(&self, index: i32, dst: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.getArray:(I[BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_bytebu(&self, src: ByteBuffer) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_bytebu_i_i(&self, index: i32, src: ByteBuffer, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putBuffer", descriptor = "(ILjava/nio/ByteBuffer;II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putBuffer(&self, pos: i32, src: ByteBuffer, srcPos: i32, n: i32) -> Result<()> {
            panic!("stub: java/nio/ByteBuffer.putBuffer:(ILjava/nio/ByteBuffer;II)V")
        }

        #[java_method(name = "put", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: put([BII)Ljava/nio/ByteBuffer;
        pub fn put_arr_b_i_i(&self, mut src: Rc<RefCell<Vec<i8>>>, mut offset: i32, mut length: i32) -> Result<ByteBuffer> {
            let this = self;
            let _t0 = this.__super().isReadOnly()?;
            if _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: i32 = Objects::checkFromIndexSize_i_i_i(offset, length, (src.borrow().len() as i32))?;
            let _t2 = this.__super().position()?;
            let mut pos: i32 = _t2;
            let _t3 = this.__super().limit()?;
            if length > (_t3).wrapping_sub(pos) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4 = this.putArray(pos, Clone::clone(&src), offset, length)?;
            let _t5 = this.position((pos).wrapping_add(length))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "put", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: put([B)Ljava/nio/ByteBuffer;
        pub fn put_arr_b(&self, mut src: Rc<RefCell<Vec<i8>>>) -> Result<ByteBuffer> {
            let this = self;
            let _t0 = this.put_arr_b_i_i(Clone::clone(&src), 0i32, (src.borrow().len() as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "put", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_b_i_i(&self, index: i32, src: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(I[BII)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "put", descriptor = "(I[B)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_b(&self, index: i32, src: Rc<RefCell<Vec<i8>>>) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.put:(I[B)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "putArray", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putArray(&self, mut index: i32, mut src: Rc<RefCell<Vec<i8>>>, mut offset: i32, mut length: i32) -> Result<ByteBuffer> {
            let this = self;
            if (((((length as i64)).wrapping_shl((0i32&0x3f) as u32)>(6i64)) as i32-((((length as i64)).wrapping_shl((0i32&0x3f) as u32))<(6i64)) as i32)>0) {
                let mut bufAddr = (this.__get_address()).wrapping_add(((index as i64)).wrapping_shl((0i32&0x3f) as u32));
                let mut srcOffset = (ByteBuffer::ARRAY_BASE_OFFSET()).wrapping_add(((offset as i64)).wrapping_shl((0i32&0x3f) as u32));
                let mut len = ((length as i64)).wrapping_shl((0i32&0x3f) as u32);
                let _t0 = this.__super().session()?;
                let _t1 = this.base()?;
                ByteBuffer::SCOPED_MEMORY_ACCESS().copyMemory(Clone::clone(&Object::default()), Clone::clone(&_t0), Object::from_any(src.clone()), srcOffset, Clone::clone(&_t1), bufAddr, len)?;
                Reference::<Object>::reachabilityFence(Object::from_any(Clone::clone(self)))?;
                let mut local_11 = (panic!("stack underflow") as i32);
                Reference::<Object>::reachabilityFence(Object::from_any(Clone::clone(self)))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut bufAddr = (offset).wrapping_add(length);
            let mut i: i32 = offset;
            let mut srcOffset: i32 = index;
            loop {
                if i >= bufAddr { break; }
                let _t0 = this.put_i_b(srcOffset, (((src.borrow()[i as usize] as i32)) as i8))?;
                i = i.wrapping_add(1i32);
                srcOffset = srcOffset.wrapping_add(1i32);
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "hasArray", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasArray(&self) -> Result<bool> {
            panic!("stub: java/nio/ByteBuffer.hasArray:()Z")
        }

        #[java_method(name = "array", descriptor = "()[B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn array(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/nio/ByteBuffer.array:()[B")
        }

        #[java_method(name = "arrayOffset", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn arrayOffset(&self) -> Result<i32> {
            panic!("stub: java/nio/ByteBuffer.arrayOffset:()I")
        }

        #[java_method(name = "position", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn position(&self, mut newPosition: i32) -> Result<ByteBuffer> {
            let this = self;
            let _t0 = this.__super().position_i(newPosition)?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "limit", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn limit(&self, newLimit: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.limit:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "mark", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.mark:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "reset", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.reset:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "clear", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.clear:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "flip", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flip(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.flip:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "rewind", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rewind(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.rewind:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "compact", descriptor = "()Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn compact(&self) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.compact:()Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "isDirect", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isDirect(&self) -> Result<bool> {
            panic!("stub: java/nio/ByteBuffer.isDirect:()Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, ob: Object) -> Result<bool> {
            panic!("stub: java/nio/ByteBuffer.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut that: ByteBuffer) -> Result<i32> {
            let this = self;
            let _t0 = this.__super().position()?;
            let mut thisPos: i32 = _t0;
            let _t1 = this.__super().limit()?;
            let mut thisRem = (_t1).wrapping_sub(thisPos);
            let _t2 = that.__super().position()?;
            let mut thatPos: i32 = _t2;
            let _t3 = that.__super().limit()?;
            let mut thatRem = (_t3).wrapping_sub(thatPos);
            let _t4: i32 = Math::min_i_i(thisRem, thatRem)?;
            let mut length: i32 = _t4;
            if (length<0) {
                return Ok(-1i32);
            }
            let _t5: i32 = BufferMismatch::mismatch_bytebu_i_bytebu_i_i(Clone::clone(this), thisPos, Clone::clone(&that), thatPos, length)?;
            let mut i: i32 = _t5;
            if (i>=0) {
                let _t6 = this.get_i((thisPos).wrapping_add(i))?;
                let _t7 = that.get_i((thatPos).wrapping_add(i))?;
                let _t8: i32 = ByteBuffer::compare(_t6, _t7)?;
                return Ok(_t8);
            }
            Ok((thisRem).wrapping_sub(thatRem))
        }

        #[java_method(name = "compare", descriptor = "(BB)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: i8, mut y: i8) -> Result<i32> {
            let _t0: i32 = Byte::compare(x, y)?;
            Ok(_t0)
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch(&self, that: ByteBuffer) -> Result<i32> {
            panic!("stub: java/nio/ByteBuffer.mismatch:(Ljava/nio/ByteBuffer;)I")
        }

        #[java_method(name = "order", descriptor = "()Ljava/nio/ByteOrder;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn order(&self) -> Result<ByteOrder> {
            panic!("stub: java/nio/ByteBuffer.order:()Ljava/nio/ByteOrder;")
        }

        #[java_method(name = "order", descriptor = "(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn order_byteor(&self, bo: ByteOrder) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.order:(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "alignmentOffset", descriptor = "(II)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn alignmentOffset(&self, index: i32, unitSize: i32) -> Result<i32> {
            panic!("stub: java/nio/ByteBuffer.alignmentOffset:(II)I")
        }

        #[java_method(name = "alignedSlice", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn alignedSlice(&self, unitSize: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.alignedSlice:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getChar", descriptor = "()C", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getChar(&self) -> Result<u16> {
            panic!("stub: java/nio/ByteBuffer.getChar:()C")
        }

        #[java_method(name = "putChar", descriptor = "(C)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putChar_c(&self, arg0: u16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putChar:(C)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getChar", descriptor = "(I)C", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getChar_i(&self, arg0: i32) -> Result<u16> {
            panic!("stub: java/nio/ByteBuffer.getChar:(I)C")
        }

        #[java_method(name = "putChar", descriptor = "(IC)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putChar_i_c(&self, arg0: i32, arg1: u16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putChar:(IC)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asCharBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asCharBuffer(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/ByteBuffer.asCharBuffer:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "getShort", descriptor = "()S", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getShort(&self) -> Result<i16> {
            panic!("stub: java/nio/ByteBuffer.getShort:()S")
        }

        #[java_method(name = "putShort", descriptor = "(S)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putShort_s(&self, arg0: i16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putShort:(S)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getShort", descriptor = "(I)S", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getShort_i(&self, arg0: i32) -> Result<i16> {
            panic!("stub: java/nio/ByteBuffer.getShort:(I)S")
        }

        #[java_method(name = "putShort", descriptor = "(IS)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putShort_i_s(&self, arg0: i32, arg1: i16) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putShort:(IS)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asShortBuffer", descriptor = "()Ljava/nio/ShortBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asShortBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/ByteBuffer.asShortBuffer:()Ljava/nio/ShortBuffer;")
        }

        #[java_method(name = "getInt", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getInt(&self) -> Result<i32> {
            panic!("stub: java/nio/ByteBuffer.getInt:()I")
        }

        #[java_method(name = "putInt", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putInt_i(&self, arg0: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putInt:(I)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getInt", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getInt_i(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/nio/ByteBuffer.getInt:(I)I")
        }

        #[java_method(name = "putInt", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putInt_i_i(&self, arg0: i32, arg1: i32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putInt:(II)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asIntBuffer", descriptor = "()Ljava/nio/IntBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asIntBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/ByteBuffer.asIntBuffer:()Ljava/nio/IntBuffer;")
        }

        #[java_method(name = "getLong", descriptor = "()J", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getLong(&self) -> Result<i64> {
            panic!("stub: java/nio/ByteBuffer.getLong:()J")
        }

        #[java_method(name = "putLong", descriptor = "(J)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putLong_l(&self, arg0: i64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putLong:(J)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getLong", descriptor = "(I)J", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getLong_i(&self, arg0: i32) -> Result<i64> {
            panic!("stub: java/nio/ByteBuffer.getLong:(I)J")
        }

        #[java_method(name = "putLong", descriptor = "(IJ)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putLong_i_l(&self, arg0: i32, arg1: i64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putLong:(IJ)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asLongBuffer", descriptor = "()Ljava/nio/LongBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asLongBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/ByteBuffer.asLongBuffer:()Ljava/nio/LongBuffer;")
        }

        #[java_method(name = "getFloat", descriptor = "()F", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getFloat(&self) -> Result<f32> {
            panic!("stub: java/nio/ByteBuffer.getFloat:()F")
        }

        #[java_method(name = "putFloat", descriptor = "(F)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putFloat_f(&self, arg0: f32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putFloat:(F)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getFloat", descriptor = "(I)F", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getFloat_i(&self, arg0: i32) -> Result<f32> {
            panic!("stub: java/nio/ByteBuffer.getFloat:(I)F")
        }

        #[java_method(name = "putFloat", descriptor = "(IF)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putFloat_i_f(&self, arg0: i32, arg1: f32) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putFloat:(IF)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asFloatBuffer", descriptor = "()Ljava/nio/FloatBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asFloatBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/ByteBuffer.asFloatBuffer:()Ljava/nio/FloatBuffer;")
        }

        #[java_method(name = "getDouble", descriptor = "()D", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDouble(&self) -> Result<f64> {
            panic!("stub: java/nio/ByteBuffer.getDouble:()D")
        }

        #[java_method(name = "putDouble", descriptor = "(D)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putDouble_d(&self, arg0: f64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putDouble:(D)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "getDouble", descriptor = "(I)D", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDouble_i(&self, arg0: i32) -> Result<f64> {
            panic!("stub: java/nio/ByteBuffer.getDouble:(I)D")
        }

        #[java_method(name = "putDouble", descriptor = "(ID)Ljava/nio/ByteBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn putDouble_i_d(&self, arg0: i32, arg1: f64) -> Result<ByteBuffer> {
            panic!("stub: java/nio/ByteBuffer.putDouble:(ID)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "asDoubleBuffer", descriptor = "()Ljava/nio/DoubleBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asDoubleBuffer(&self) -> Result<Object> {
            panic!("stub: java/nio/ByteBuffer.asDoubleBuffer:()Ljava/nio/DoubleBuffer;")
        }
    }
}
