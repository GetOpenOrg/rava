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

impl From<CharBuffer> for Buffer {
    fn from(v: CharBuffer) -> Buffer { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/CharBuffer"]
    #[super_class       = "java/nio/Buffer"]
    #[interfaces        = "java/lang/Comparable,java/lang/Appendable,java/lang/CharSequence,java/lang/Readable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "Ljava/nio/Buffer;Ljava/lang/Comparable<Ljava/nio/CharBuffer;>;Ljava/lang/Appendable;Ljava/lang/CharSequence;Ljava/lang/Readable;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharBuffer.java"]
    #[inner_classes     = "java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Buffer"]
    #[superclass_fields(mark: i32, position: i32, limit: i32, capacity: i32, address: i64, segment: Object)]
    #[all_supertypes    = "java/lang/Appendable;java/lang/CharSequence;java/lang/Comparable;java/lang/Object;java/lang/Readable;java/nio/Buffer;java/nio/CharBuffer"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct CharBuffer {
        #[cfg_attr(any(), java_field(name = "hb", descriptor = "[C", access = "package", modifiers = "final", is_static = false))]
        pub hb: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "isReadOnly", descriptor = "Z", is_static = false))]
        pub isReadOnly: bool,
    }

    impl CharBuffer {
        #[cfg_attr(any(), java_field(name = "ARRAY_BASE_OFFSET", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BASE_OFFSET:J
        pub fn ARRAY_BASE_OFFSET() -> i64 {
            panic!("stub: java/nio/CharBuffer.ARRAY_BASE_OFFSET:J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(IIII[CILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IIII[CILjava/lang/foreign/MemorySegment;)V
        pub fn new_i_i_i_i_arr_c_i_memory(mut mark: i32, mut pos: i32, mut lim: i32, mut cap: i32, mut hb: Rc<RefCell<Vec<u16>>>, mut offset: i32, mut segment: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Buffer::new_i_i_i_i_memory(mark, pos, lim, cap, Clone::clone(&segment))?);
            this.__set_hb(Clone::clone(&hb));
            this.__set_offset(offset);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(IIIILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/CharBuffer.<init>:(IIIILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "<init>", descriptor = "([CJILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_l_i_memory(hb: Rc<RefCell<Vec<u16>>>, addr: i64, arg2: i32, cap: Object) -> Result<Self> {
            panic!("stub: java/nio/CharBuffer.<init>:([CJILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "base", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn base(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(this.__get_hb().clone()))
        }

        #[java_method(name = "allocate", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allocate(capacity: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.allocate:(I)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "wrap", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn wrap_arr_c_i_i(array: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.wrap:([CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "wrap", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn wrap_arr_c(array: Rc<RefCell<Vec<u16>>>) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.wrap:([C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "read", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read(&self, target: CharBuffer) -> Result<i32> {
            panic!("stub: java/nio/CharBuffer.read:(Ljava/nio/CharBuffer;)I")
        }

        #[java_method(name = "wrap", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn wrap_seq_i_i(csq: Object, start: i32, end: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.wrap:(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "wrap", descriptor = "(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn wrap_seq(csq: Object) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.wrap:(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "slice", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.slice:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "slice", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice_i_i(&self, arg0: i32, arg1: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.slice:(II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "duplicate", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn duplicate(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.duplicate:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn asReadOnlyBuffer(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.asReadOnlyBuffer:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "()C", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn get(&self) -> Result<u16> {
            panic!("stub: java/nio/CharBuffer.get:()C")
        }

        #[java_method(name = "put", descriptor = "(C)Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn put_c(&self, arg0: u16) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:(C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I)C", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn get_i(&self, arg0: i32) -> Result<u16> {
            panic!("stub: java/nio/CharBuffer.get:(I)C")
        }

        #[java_method(name = "getUnchecked", descriptor = "(I)C", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getUnchecked(&self, arg0: i32) -> Result<u16> {
            panic!("stub: java/nio/CharBuffer.getUnchecked:(I)C")
        }

        #[java_method(name = "put", descriptor = "(IC)Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn put_i_c(&self, arg0: i32, arg1: u16) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:(IC)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_c_i_i(&self, dst: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.get:([CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_c(&self, dst: Rc<RefCell<Vec<u16>>>) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.get:([C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_c_i_i(&self, index: i32, dst: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.get:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[C)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_c(&self, index: i32, dst: Rc<RefCell<Vec<u16>>>) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.get:(I[C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "getArray", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getArray(&self, index: i32, dst: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.getArray:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_charbu(&self, src: CharBuffer) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: put(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;
        pub fn put_i_charbu_i_i(&self, mut index: i32, mut src: CharBuffer, mut offset: i32, mut length: i32) -> Result<CharBuffer> {
            let this = self;
            let _t0 = this.__super().limit()?;
            let _t1: i32 = Objects::checkFromIndexSize_i_i_i(index, length, _t0)?;
            let _t2 = src.__super().limit()?;
            let _t3: i32 = Objects::checkFromIndexSize_i_i_i(offset, length, _t2)?;
            let _t4 = this.__super().isReadOnly()?;
            if _t4 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.putBuffer(index, Clone::clone(&src), offset, length)?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "putBuffer", descriptor = "(ILjava/nio/CharBuffer;II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putBuffer(&self, mut pos: i32, mut src: CharBuffer, mut srcPos: i32, mut n: i32) -> Result<()> {
            let this = self;
            let _t0 = src.base()?;
            let mut srcBase: Object = _t0;
            let _t1 = src.isAddressable()?;
            if _t1 {
                let _t2 = this.base()?;
                let mut base: Object = _t2;
                let _t3 = this.isDirect()?;
                if !(_t3) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let mut srcAddr = (src.__get_address()).wrapping_add(((srcPos as i64)).wrapping_shl((1i32&0x3f) as u32));
                let mut addr = (this.__get_address()).wrapping_add(((pos as i64)).wrapping_shl((1i32&0x3f) as u32));
                let mut len = ((n as i64)).wrapping_shl((1i32&0x3f) as u32);
                let _t4 = this.order()?;
                let _t5 = src.order()?;
                if Object::from_any(_t4.clone()) != Object::from_any(_t5.clone()) {
                    let _t6 = src.__super().session()?;
                    let _t7 = this.__super().session()?;
                    CharBuffer::SCOPED_MEMORY_ACCESS().copySwapMemory(Clone::clone(&_t6), Clone::clone(&_t7), Clone::clone(&srcBase), srcAddr, Clone::clone(&base), addr, len, 2i64)?;
                } else {
                    let _t6 = src.__super().session()?;
                    let _t7 = this.__super().session()?;
                    CharBuffer::SCOPED_MEMORY_ACCESS().copyMemory(Clone::clone(&_t6), Clone::clone(&_t7), Clone::clone(&srcBase), srcAddr, Clone::clone(&base), addr, len)?;
                }
                Reference::<Object>::reachabilityFence(Object::from_any(src.clone()))?;
                Reference::<Object>::reachabilityFence(Object::from_any(Clone::clone(self)))?;
            } else {
                let _vdispatch2: bool = if let Some(__f) = Object::default().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(src.clone()))? } else { Default::default() };
                if !(_vdispatch2) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let mut base = (pos).wrapping_add(n);
                let mut srcAddr: i32 = pos;
                let mut j: i32 = srcPos;
                loop {
                    if srcAddr >= base { break; }
                    let _t3 = src.get_i(j)?;
                    let _t4 = this.put_i_c(srcAddr, _t3)?;
                    srcAddr = srcAddr.wrapping_add(1i32);
                    j = j.wrapping_add(1i32);
                }
            }
            Ok(())
        }

        #[java_method(name = "put", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_arr_c_i_i(&self, src: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:([CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_arr_c(&self, src: Rc<RefCell<Vec<u16>>>) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:([C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_c_i_i(&self, index: i32, src: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(I[C)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_c(&self, index: i32, src: Rc<RefCell<Vec<u16>>>) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.put:(I[C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "putArray", descriptor = "(I[CII)Ljava/nio/CharBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putArray(&self, index: i32, src: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.putArray:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/String;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: put(Ljava/lang/String;II)Ljava/nio/CharBuffer;
        pub fn put_str_i_i(&self, mut src: String, mut start: i32, mut end: i32) -> Result<CharBuffer> {
            let this = self;
            let _t0 = src.length()?;
            let _t1: i32 = Objects::checkFromIndexSize_i_i_i(start, (end).wrapping_sub(start), _t0)?;
            let _t2 = this.__super().isReadOnly()?;
            if _t2 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t3 = this.__super().remaining()?;
            if (end).wrapping_sub(start) > _t3 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut i: i32 = start;
            loop {
                if i >= end { break; }
                let _t4 = src.charAt(i)?;
                let _t5 = this.put_c(_t4)?;
                i = i.wrapping_add(1i32);
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/String;)Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: put(Ljava/lang/String;)Ljava/nio/CharBuffer;
        pub fn put_str(&self, mut src: String) -> Result<CharBuffer> {
            let this = self;
            let _t0 = src.length()?;
            let _t1 = this.put_str_i_i(Clone::clone(&src), 0i32, _t0)?;
            Ok(_t1)
        }

        #[java_method(name = "hasArray", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasArray(&self) -> Result<bool> {
            panic!("stub: java/nio/CharBuffer.hasArray:()Z")
        }

        #[java_method(name = "array", descriptor = "()[C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn array(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/nio/CharBuffer.array:()[C")
        }

        #[java_method(name = "arrayOffset", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn arrayOffset(&self) -> Result<i32> {
            panic!("stub: java/nio/CharBuffer.arrayOffset:()I")
        }

        #[java_method(name = "position", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn position(&self, mut newPosition: i32) -> Result<CharBuffer> {
            let this = self;
            let _t0 = this.__super().position_i(newPosition)?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "limit", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn limit(&self, newLimit: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.limit:(I)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "mark", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.mark:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "reset", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.reset:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "clear", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.clear:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "flip", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flip(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.flip:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "rewind", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rewind(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.rewind:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "compact", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn compact(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.compact:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "isDirect", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isDirect(&self) -> Result<bool> {
            panic!("stub: java/nio/CharBuffer.isDirect:()Z")
        }

        #[java_method(name = "isAddressable", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAddressable(&self) -> Result<bool> {
            let this = self;
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, ob: Object) -> Result<bool> {
            panic!("stub: java/nio/CharBuffer.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut that: CharBuffer) -> Result<i32> {
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
            let _t5: i32 = BufferMismatch::mismatch_charbu_i_charbu_i_i(Clone::clone(this), thisPos, Clone::clone(&that), thatPos, length)?;
            let mut i: i32 = _t5;
            if (i>=0) {
                let _t6 = this.get_i((thisPos).wrapping_add(i))?;
                let _t7 = that.get_i((thatPos).wrapping_add(i))?;
                let _t8: i32 = CharBuffer::compare(_t6, _t7)?;
                return Ok(_t8);
            }
            Ok((thisRem).wrapping_sub(thatRem))
        }

        #[java_method(name = "compare", descriptor = "(CC)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: u16, mut y: u16) -> Result<i32> {
            let _t0: i32 = Character::compare(x, y)?;
            Ok(_t0)
        }

        #[java_method(name = "mismatch", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch(&self, that: CharBuffer) -> Result<i32> {
            panic!("stub: java/nio/CharBuffer.mismatch:(Ljava/nio/CharBuffer;)I")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString()Ljava/lang/String;
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__super().position()?;
            let _t1 = this.__super().limit()?;
            let _t2 = this.toString_i_i(_t0, _t1)?;
            Ok(_t2)
        }

        #[java_method(name = "toString", descriptor = "(II)Ljava/lang/String;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toString_i_i(&self, arg0: i32, arg1: i32) -> Result<String> {
            panic!("stub: java/nio/CharBuffer.toString:(II)Ljava/lang/String;")
        }

        #[java_method(name = "length", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__super().remaining()?;
            Ok(_t0)
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/nio/CharBuffer.isEmpty:()Z")
        }

        #[java_method(name = "charAt", descriptor = "(I)C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(&self, mut index: i32) -> Result<u16> {
            let this = self;
            let _t0 = this.__super().position()?;
            let _t1 = this.__super().checkIndex(index, 1i32)?;
            let _t2 = this.get_i((_t0).wrapping_add(_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "subSequence", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn subSequence(&self, arg0: i32, arg1: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.subSequence:(II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_seq(&self, csq: Object) -> Result<CharBuffer> {
            panic!("stub: java/nio/CharBuffer.append:(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;
        pub fn append_seq_i_i(&self, mut csq: Object, mut start: i32, mut end: i32) -> Result<CharBuffer> {
            let this = self;
            let mut cb = (csq).downcast::<CharBuffer>();
            let _vdispatch0: i32 = if let Some(_d) = csq.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = csq.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = csq.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = csq.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = csq.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = csq.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = csq.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _t1: i32 = Objects::checkFromToIndex_i_i_i(start, end, _vdispatch0)?;
            let mut length = (end).wrapping_sub(start);
            let _t2 = this.__super().position()?;
            let mut pos: i32 = _t2;
            let _t3 = this.__super().limit()?;
            let mut lim: i32 = _t3;
            let mut rem = (if pos <= lim { (lim).wrapping_sub(pos) } else { 0i32 });
            if length > rem {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4 = this.put_i_charbu_i_i(pos, Clone::clone(&cb), start, length)?;
            let _t5 = this.position((pos).wrapping_add(length))?;
            return Ok(Clone::clone(this));
            let mut cb = (if _is_jnull(&csq) { String::from("null") } else { (csq).downcast::<String>() });
            let _t6 = cb.subSequence(start, end)?;
            let _vdispatch7: String = if let Some(_d) = _t6.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _t8 = this.put_str(Clone::clone(&_vdispatch7))?;
            Ok(_t8)
        }

        #[java_method(name = "append", descriptor = "(C)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(C)Ljava/nio/CharBuffer;
        pub fn append_c(&self, mut c: u16) -> Result<CharBuffer> {
            let this = self;
            let _t0 = this.put_c(c)?;
            Ok(_t0)
        }

        #[java_method(name = "order", descriptor = "()Ljava/nio/ByteOrder;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn order(&self) -> Result<ByteOrder> {
            panic!("stub: java/nio/CharBuffer.order:()Ljava/nio/ByteOrder;")
        }

        #[java_method(name = "charRegionOrder", descriptor = "()Ljava/nio/ByteOrder;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn charRegionOrder(&self) -> Result<ByteOrder> {
            panic!("stub: java/nio/CharBuffer.charRegionOrder:()Ljava/nio/ByteOrder;")
        }

        #[java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn chars(&self) -> Result<Object> {
            panic!("stub: java/nio/CharBuffer.chars:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePoints(&self) -> Result<Object> {
            panic!("stub: java/nio/CharBuffer.codePoints:()Ljava/util/stream/IntStream;")
        }
    }
}
