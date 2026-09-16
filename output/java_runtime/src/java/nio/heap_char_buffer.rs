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
use crate::jdk::internal::misc::Unsafe;

impl From<HeapCharBuffer> for CharBuffer {
    fn from(v: HeapCharBuffer) -> CharBuffer { v.__into_super() }
}

impl From<HeapCharBuffer> for Buffer {
    fn from(v: HeapCharBuffer) -> Buffer { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/HeapCharBuffer"]
    #[super_class       = "java/nio/CharBuffer"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HeapCharBuffer.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CharBuffer"]
    #[superclass_fields(mark: i32, position: i32, limit: i32, capacity: i32, address: i64, segment: Object, hb: Rc<RefCell<Vec<u16>>>, offset: i32, isReadOnly: bool)]
    #[all_supertypes    = "java/lang/Appendable;java/lang/CharSequence;java/lang/Comparable;java/lang/Object;java/lang/Readable;java/nio/Buffer;java/nio/CharBuffer;java/nio/HeapCharBuffer"]

    pub struct HeapCharBuffer;

    impl HeapCharBuffer {
        #[cfg_attr(any(), java_field(name = "ARRAY_BASE_OFFSET", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_BASE_OFFSET:J
        pub fn ARRAY_BASE_OFFSET() -> i64 {
            panic!("stub: java/nio/HeapCharBuffer.ARRAY_BASE_OFFSET:J")
        }

        #[cfg_attr(any(), java_field(name = "ARRAY_INDEX_SCALE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: ARRAY_INDEX_SCALE:J
        pub fn ARRAY_INDEX_SCALE() -> i64 {
            panic!("stub: java/nio/HeapCharBuffer.ARRAY_INDEX_SCALE:J")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(IILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_memory(cap: i32, lim: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/HeapCharBuffer.<init>:(IILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "<init>", descriptor = "([CIILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([CIILjava/lang/foreign/MemorySegment;)V
        pub fn new_arr_c_i_i_memory(mut buf: Rc<RefCell<Vec<u16>>>, mut off: i32, mut len: i32, mut segment: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(CharBuffer::new_i_i_i_i_arr_c_i_memory(-1i32, off, (off).wrapping_add(len), (buf.borrow().len() as i32), Clone::clone(&buf), 0i32, Clone::clone(&segment))?);
            this.__set_address(HeapCharBuffer::ARRAY_BASE_OFFSET());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([CIIIIILjava/lang/foreign/MemorySegment;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_i_i_i_i_i_memory(buf: Rc<RefCell<Vec<u16>>>, mark: i32, pos: i32, lim: i32, cap: i32, off: i32, segment: Object) -> Result<Self> {
            panic!("stub: java/nio/HeapCharBuffer.<init>:([CIIIIILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "slice", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn slice(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.slice:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "slice", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn slice_i_i(&self, index: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.slice:(II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "duplicate", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn duplicate(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.duplicate:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asReadOnlyBuffer(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.asReadOnlyBuffer:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "ix", descriptor = "(I)I", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ix(&self, i: i32) -> Result<i32> {
            panic!("stub: java/nio/HeapCharBuffer.ix:(I)I")
        }

        #[java_method(name = "get", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<u16> {
            panic!("stub: java/nio/HeapCharBuffer.get:()C")
        }

        #[java_method(name = "get", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i(&self, i: i32) -> Result<u16> {
            panic!("stub: java/nio/HeapCharBuffer.get:(I)C")
        }

        #[java_method(name = "getUnchecked", descriptor = "(I)C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnchecked(&self, i: i32) -> Result<u16> {
            panic!("stub: java/nio/HeapCharBuffer.getUnchecked:(I)C")
        }

        #[java_method(name = "get", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_arr_c_i_i(&self, dst: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.get:([CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "get", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get_i_arr_c_i_i(&self, index: i32, dst: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.get:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "isDirect", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDirect(&self) -> Result<bool> {
            panic!("stub: java/nio/HeapCharBuffer.isDirect:()Z")
        }

        #[java_method(name = "isReadOnly", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isReadOnly(&self) -> Result<bool> {
            panic!("stub: java/nio/HeapCharBuffer.isReadOnly:()Z")
        }

        #[java_method(name = "put", descriptor = "(C)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_c(&self, x: u16) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(C)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(IC)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_c(&self, i: i32, x: u16) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(IC)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_arr_c_i_i(&self, src: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:([CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_charbu(&self, src: CharBuffer) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_charbu_i_i(&self, index: i32, src: CharBuffer, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_i_arr_c_i_i(&self, index: i32, src: Rc<RefCell<Vec<u16>>>, offset: i32, length: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(I[CII)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "appendChars", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendChars(&self, csq: Object, start: i32, end: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.appendChars:(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_seq(&self, csq: Object) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.append:(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.append:(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "put", descriptor = "(Ljava/lang/String;II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn put_str_i_i(&self, src: String, start: i32, end: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.put:(Ljava/lang/String;II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "compact", descriptor = "()Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compact(&self) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.compact:()Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "toString", descriptor = "(II)Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self, start: i32, end: i32) -> Result<String> {
            panic!("stub: java/nio/HeapCharBuffer.toString:(II)Ljava/lang/String;")
        }

        #[java_method(name = "subSequence", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subSequence(&self, start: i32, end: i32) -> Result<CharBuffer> {
            panic!("stub: java/nio/HeapCharBuffer.subSequence:(II)Ljava/nio/CharBuffer;")
        }

        #[java_method(name = "order", descriptor = "()Ljava/nio/ByteOrder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn order(&self) -> Result<ByteOrder> {
            panic!("stub: java/nio/HeapCharBuffer.order:()Ljava/nio/ByteOrder;")
        }

        #[java_method(name = "charRegionOrder", descriptor = "()Ljava/nio/ByteOrder;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charRegionOrder(&self) -> Result<ByteOrder> {
            panic!("stub: java/nio/HeapCharBuffer.charRegionOrder:()Ljava/nio/ByteOrder;")
        }
    }
}
