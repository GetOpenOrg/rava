#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/ByteBuffer",
    super_class = "java/nio/Buffer",
    interfaces  = "java/lang/Comparable",
    access      = "public abstract",
    source      = "ByteBuffer.java",
))]
pub struct ByteBuffer {
    #[cfg_attr(any(), java_field(name = "hb", descriptor = "[B", access = "final"))]
    pub hb: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "isReadOnly", descriptor = "Z"))]
    pub isReadOnly: Field<bool>,
    #[cfg_attr(any(), java_field(name = "bigEndian", descriptor = "Z"))]
    pub bigEndian: Field<bool>,
    #[cfg_attr(any(), java_field(name = "nativeByteOrder", descriptor = "Z"))]
    pub nativeByteOrder: Field<bool>,
}

impl ByteBuffer {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IIII[BILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>(IIII[BILjava/lang/foreign/MemorySegment;)V
    pub fn new__i_i_i_i_arr_b_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, hb: Vec<i8>, offset: i32, segment: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false), bigEndian: Field::new(false), nativeByteOrder: Field::new(false) };
        /* invokespecial Method java/nio/Buffer.<init>:(IIIILjava/lang/foreign/MemorySegment;)V */
        this.bigEndian.set(1i32);
        let _t0: Object = ByteOrder::nativeOrder()?;
        _t0.nativeByteOrder.set(/* if_acmpne */ true);
        this.hb.set(hb);
        this.offset.set(offset);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IIIILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>(IIIILjava/lang/foreign/MemorySegment;)V
    pub fn new__i_i_i_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, segment: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false), bigEndian: Field::new(false), nativeByteOrder: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/nio/ByteBuffer.<init>:(IIII[BILjava/lang/foreign/MemorySegment;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([BJILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>([BJILjava/lang/foreign/MemorySegment;)V
    pub fn new__arr_b_l_i_memory(hb: Vec<i8>, addr: i64, arg_2: i32, cap: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false), bigEndian: Field::new(false), nativeByteOrder: Field::new(false) };
        /* invokespecial Method java/nio/Buffer.<init>:(JILjava/lang/foreign/MemorySegment;)V */
        this.bigEndian.set(1i32);
        let _t0: Object = ByteOrder::nativeOrder()?;
        _t0.nativeByteOrder.set(/* if_acmpne */ true);
        this.hb.set(hb);
        this.offset.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "base", descriptor = "()Ljava/lang/Object;"))]
    pub fn base(&self) -> Result<Object> {
        let this = self;
        Ok(this.hb.get())
    }

    #[cfg_attr(any(), java_method(name = "allocateDirect", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public static"))]
    pub fn allocateDirect(capacity: i32) -> Result<Object> {
        Ok(DirectByteBuffer::new(capacity)?)
    }

    #[cfg_attr(any(), java_method(name = "allocate", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public static"))]
    pub fn allocate(capacity: i32) -> Result<Object> {
        let _t0: Object = ByteBuffer::createCapacityException(capacity)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        let mut _obj1: HeapByteBuffer = HeapByteBuffer::new(HeapByteBuffer::new(), capacity, capacity)?;
        Ok(_obj1)
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public static"))]
    // java: wrap([BII)Ljava/nio/ByteBuffer;
    pub fn wrap__arr_b_i_i(array: &[i8], offset: i32, length: i32) -> Result<Object> {
        /* TODO: aconst_null  */
        let mut _obj0: HeapByteBuffer = HeapByteBuffer::new(HeapByteBuffer::new(), array, offset, length)?;
        return Ok(_obj0);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public static"))]
    // java: wrap([B)Ljava/nio/ByteBuffer;
    pub fn wrap__arr_b(array: &[i8]) -> Result<Object> {
        let _t0: Object = ByteBuffer::wrap(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "slice", descriptor = "()Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn slice(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.slice")
    }

    #[cfg_attr(any(), java_native(name = "slice", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn slice__i_i(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.slice")
    }

    #[cfg_attr(any(), java_native(name = "duplicate", descriptor = "()Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn duplicate(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.duplicate")
    }

    #[cfg_attr(any(), java_native(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn asReadOnlyBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asReadOnlyBuffer")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "()B", access = "public abstract"))]
    pub fn get(&self) -> Result<i8> {
        todo!("abstract java/nio/ByteBuffer.get")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(B)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn put__b(&self, arg0: i8) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.put")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(I)B", access = "public abstract"))]
    pub fn get__i(&self, arg0: i32) -> Result<i8> {
        todo!("abstract java/nio/ByteBuffer.get")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(IB)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn put__i_b(&self, arg0: i32, arg1: i8) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.put")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: get([BII)Ljava/nio/ByteBuffer;
    pub fn get__arr_b_i_i(&self, dst: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(offset, length, (dst.len() as i32))?;
        let _t1 = this.position()?;
        let mut pos: i32 = _t1;
        let _t2 = this.limit()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = this.getArray(pos, dst, offset, length)?;
        let _t4 = this.position((pos).wrapping_add(length))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: get([B)Ljava/nio/ByteBuffer;
    pub fn get__arr_b(&self, dst: Vec<i8>) -> Result<Object> {
        let this = self;
        let _t0 = this.get(dst, 0i32, (dst.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: get(I[BII)Ljava/nio/ByteBuffer;
    pub fn get__i_arr_b_i_i(&self, index: i32, dst: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.limit()?;
        let _t1: i32 = Objects::checkFromIndexSize(index, length, _t0)?;
        let _t2: i32 = Objects::checkFromIndexSize(offset, length, (dst.len() as i32))?;
        let _t3 = this.getArray(index, dst, offset, length)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I[B)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: get(I[B)Ljava/nio/ByteBuffer;
    pub fn get__i_arr_b(&self, index: i32, dst: Vec<i8>) -> Result<Object> {
        let this = self;
        let _t0 = this.get(index, dst, 0i32, (dst.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getArray", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "private"))]
    pub fn getArray(&self, index: i32, dst: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        /* TODO: lshl  */
        /* TODO: lcmp  */
        /* TODO: lshl  */
        let mut bufAddr: i64 = ((index as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut dstOffset: i64 = ((offset as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut len: i64 = 0i32;
        let _t0 = this.session()?;
        /* TODO: aconst_null  */
        let _t1 = this.base()?;
        (length as i64).copyMemory(ByteBuffer::SCOPED_MEMORY_ACCESS(), _t0, _t1, bufAddr, dst, dstOffset, len)?;
        Reference::reachabilityFence(this)?;
        let mut local_11: i64 = ByteBuffer::ARRAY_BASE_OFFSET();
        Reference::reachabilityFence(this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        bufAddr = (offset).wrapping_add(length);
        let mut i: i32 = offset;
        dstOffset = index;
        loop {
            if i >= bufAddr { break; }
            let _t0 = this.get(dstOffset)?;
            dst[i as usize] = _t0;
            i = i.wrapping_add(1i32);
            dstOffset = dstOffset.wrapping_add(1i32);
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: put(Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;
    pub fn put__bytebu(&self, src: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ByteBuffer::createSameBufferException()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = src.position()?;
        let mut srcPos: i32 = _t2;
        let _t3 = src.limit()?;
        let mut srcLim: i32 = _t3;
        let mut srcRem: i32 = 0i32;
        let _t4 = this.position()?;
        let mut pos: i32 = _t4;
        let _t5 = this.limit()?;
        let mut lim: i32 = _t5;
        let mut rem: i32 = 0i32;
        return Err(JvmError::Custom(String::from("athrow")));
        this.putBuffer(pos, src, srcPos, srcRem)?;
        let _t6 = this.position((pos).wrapping_add(srcRem))?;
        let _t7 = src.position((srcPos).wrapping_add(srcRem))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: put(ILjava/nio/ByteBuffer;II)Ljava/nio/ByteBuffer;
    pub fn put__i_bytebu_i_i(&self, index: i32, src: Object, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.limit()?;
        let _t1: i32 = Objects::checkFromIndexSize(index, length, _t0)?;
        let _t2 = src.limit()?;
        let _t3: i32 = Objects::checkFromIndexSize(offset, length, _t2)?;
        let _t4 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.putBuffer(index, src, offset, length)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "putBuffer", descriptor = "(ILjava/nio/ByteBuffer;II)V"))]
    pub fn putBuffer(&self, pos: i32, src: Object, srcPos: i32, n: i32) -> Result<()> {
        let this = self;
        let _t0 = src.base()?;
        let mut srcBase: Object = _t0;
        let _t1 = src.isDirect()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.base()?;
        let mut base: Object = _t2;
        let _t3 = this.isDirect()?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lshl  */
        let mut srcAddr: i64 = ((srcPos as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut addr: i64 = ((pos as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut len: i64 = 0i32;
        let _t4 = src.session()?;
        let _t5 = this.session()?;
        ByteBuffer::SCOPED_MEMORY_ACCESS().copyMemory(_t4, _t5, srcBase, srcAddr, base, addr, len)?;
        Reference::reachabilityFence(src)?;
        Reference::reachabilityFence(this)?;
        let mut local_13: i64 = (n as i64);
        Reference::reachabilityFence(src)?;
        Reference::reachabilityFence(this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "([BII)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: put([BII)Ljava/nio/ByteBuffer;
    pub fn put__arr_b_i_i(&self, src: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: i32 = Objects::checkFromIndexSize(offset, length, (src.len() as i32))?;
        let _t2 = this.position()?;
        let mut pos: i32 = _t2;
        let _t3 = this.limit()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = this.putArray(pos, src, offset, length)?;
        let _t5 = this.position((pos).wrapping_add(length))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "([B)Ljava/nio/ByteBuffer;", access = "public final"))]
    // java: put([B)Ljava/nio/ByteBuffer;
    pub fn put__arr_b(&self, src: Vec<i8>) -> Result<Object> {
        let this = self;
        let _t0 = this.put(src, 0i32, (src.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(I[BII)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: put(I[BII)Ljava/nio/ByteBuffer;
    pub fn put__i_arr_b_i_i(&self, index: i32, src: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.limit()?;
        let _t2: i32 = Objects::checkFromIndexSize(index, length, _t1)?;
        let _t3: i32 = Objects::checkFromIndexSize(offset, length, (src.len() as i32))?;
        let _t4 = this.putArray(index, src, offset, length)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(I[B)Ljava/nio/ByteBuffer;", access = "public"))]
    // java: put(I[B)Ljava/nio/ByteBuffer;
    pub fn put__i_arr_b(&self, index: i32, src: Vec<i8>) -> Result<Object> {
        let this = self;
        let _t0 = this.put(index, src, 0i32, (src.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "putArray", descriptor = "(I[BII)Ljava/nio/ByteBuffer;"))]
    pub fn putArray(&self, index: i32, src: Vec<i8>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        /* TODO: lshl  */
        /* TODO: lcmp  */
        /* TODO: lshl  */
        let mut bufAddr: i64 = ((index as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut srcOffset: i64 = ((offset as i64)).wrapping_add(0i32);
        /* TODO: lshl  */
        let mut len: i64 = 0i32;
        /* TODO: aconst_null  */
        let _t0 = this.session()?;
        let _t1 = this.base()?;
        (length as i64).copyMemory(ByteBuffer::SCOPED_MEMORY_ACCESS(), _t0, src, srcOffset, _t1, bufAddr, len)?;
        Reference::reachabilityFence(this)?;
        let mut local_11: i64 = ByteBuffer::ARRAY_BASE_OFFSET();
        Reference::reachabilityFence(this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        bufAddr = (offset).wrapping_add(length);
        let mut i: i32 = offset;
        srcOffset = index;
        loop {
            if i >= bufAddr { break; }
            let _t0 = this.put(srcOffset, src[i as usize])?;
            i = i.wrapping_add(1i32);
            srcOffset = srcOffset.wrapping_add(1i32);
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasArray", descriptor = "()Z", access = "public final"))]
    pub fn hasArray(&self) -> Result<bool> {
        let this = self;
        Ok(this.isReadOnly.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "array", descriptor = "()[B", access = "public final"))]
    pub fn array(&self) -> Result<Vec<i8>> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this.hb.get())
    }

    #[cfg_attr(any(), java_method(name = "arrayOffset", descriptor = "()I", access = "public final"))]
    pub fn arrayOffset(&self) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this.offset.get())
    }

    #[cfg_attr(any(), java_method(name = "position", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn position(&self, newPosition: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::position(newPosition)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "limit", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn limit(&self, newLimit: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::limit(newLimit)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "mark", descriptor = "()Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn mark(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::mark()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn reset(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::reset()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn clear(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::clear()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "flip", descriptor = "()Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn flip(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::flip()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "rewind", descriptor = "()Ljava/nio/ByteBuffer;", access = "public"))]
    pub fn rewind(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::rewind()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "compact", descriptor = "()Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn compact(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.compact")
    }

    #[cfg_attr(any(), java_native(name = "isDirect", descriptor = "()Z", access = "public abstract"))]
    pub fn isDirect(&self) -> Result<bool> {
        todo!("abstract java/nio/ByteBuffer.isDirect")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = _t0.getName()?;
        String::new().append(&_t1)?;
        String::new().append(&String::from("[pos="))?;
        let _t2 = this.position()?;
        String::new().append(&_t2)?;
        String::new().append(&String::from("lim="))?;
        let _t3 = this.limit()?;
        String::new().append(&_t3)?;
        String::new().append(&String::from("cap="))?;
        let _t4 = this.capacity()?;
        String::new().append(&_t4)?;
        String::new().append(&String::from("]"))?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut h: i32 = 1i32;
        let _t0 = this.position()?;
        let mut p: i32 = _t0;
        let _t1 = this.limit()?;
        let mut i: i32 = (_t1).wrapping_sub(1i32);
        loop {
            if i < p { break; }
            let _t0 = this.get(i)?;
            h = ((31i32).wrapping_mul(h)).wrapping_add(_t0);
            i = i.wrapping_sub(1i32);
        }
        Ok(h)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, ob: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        return Ok(0i32);
        let mut that: Object = ob;
        let _t0 = this.position()?;
        let mut thisPos: i32 = _t0;
        let _t1 = this.limit()?;
        let mut thisRem: i32 = (_t1).wrapping_sub(thisPos);
        let _t2 = that.position()?;
        let mut thatPos: i32 = _t2;
        let _t3 = that.limit()?;
        let mut thatRem: i32 = (_t3).wrapping_sub(thatPos);
        return Ok(0i32);
        let _t4: i32 = BufferMismatch::mismatch(this, thisPos, that, thatPos, thisRem)?;
        Ok(_t4<0i32)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public"))]
    pub fn compareTo(&self, that: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.position()?;
        let mut thisPos: i32 = _t0;
        let _t1 = this.limit()?;
        let mut thisRem: i32 = (_t1).wrapping_sub(thisPos);
        let _t2 = that.position()?;
        let mut thatPos: i32 = _t2;
        let _t3 = that.limit()?;
        let mut thatRem: i32 = (_t3).wrapping_sub(thatPos);
        let _t4: i32 = (thisRem).min(thatRem);
        let mut length: i32 = _t4;
        return Ok(-1i32);
        let _t5: i32 = BufferMismatch::mismatch(this, thisPos, that, thatPos, length)?;
        let mut i: i32 = _t5;
        let _t6 = this.get((thisPos).wrapping_add(i))?;
        let _t7 = that.get((thatPos).wrapping_add(i))?;
        let _t8: i32 = ByteBuffer::compare(_t6, _t7)?;
        return Ok(_t8);
        Ok((thisRem).wrapping_sub(thatRem))
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(BB)I", access = "private static"))]
    pub fn compare(x: i8, y: i8) -> Result<i32> {
        let _t0: i32 = Byte::compare(x, y)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public"))]
    pub fn mismatch(&self, that: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.position()?;
        let mut thisPos: i32 = _t0;
        let _t1 = this.limit()?;
        let mut thisRem: i32 = (_t1).wrapping_sub(thisPos);
        let _t2 = that.position()?;
        let mut thatPos: i32 = _t2;
        let _t3 = that.limit()?;
        let mut thatRem: i32 = (_t3).wrapping_sub(thatPos);
        let _t4: i32 = (thisRem).min(thatRem);
        let mut length: i32 = _t4;
        return Ok(-1i32);
        let _t5: i32 = BufferMismatch::mismatch(this, thisPos, that, thatPos, length)?;
        let mut r: i32 = _t5;
        Ok(r)
    }

    #[cfg_attr(any(), java_method(name = "order", descriptor = "()Ljava/nio/ByteOrder;", access = "public final"))]
    // java: order()Ljava/nio/ByteOrder;
    pub fn order(&self) -> Result<Object> {
        let this = self;
        Ok(ByteOrder::LITTLE_ENDIAN())
    }

    #[cfg_attr(any(), java_method(name = "order", descriptor = "(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;", access = "public final"))]
    // java: order(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;
    pub fn order__byteor(&self, bo: Object) -> Result<Object> {
        let this = self;
        bo.bigEndian.set(/* if_acmpne */ true);
        let _t0: Object = ByteOrder::nativeOrder()?;
        this.bigEndian.get().nativeByteOrder.set(_t0 == /* if_acmpne */ true);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "alignmentOffset", descriptor = "(II)I", access = "public final"))]
    pub fn alignmentOffset(&self, index: i32, unitSize: i32) -> Result<i32> {
        let this = self;
        String::new().append(&String::from("Index less than zero:"))?;
        String::new().append(&index)?;
        return Err(JvmError::Custom(String::from("athrow")));
        String::new().append(&String::from("Unit size not a power of two:"))?;
        String::new().append(&unitSize)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.isDirect()?;
        String::new().append(&String::from("Unit size unsupported for non-direct buffers:"))?;
        String::new().append(&unitSize)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: land  */
        Ok((((unitSize).wrapping_sub(1i32) as i64) as i32))
    }

    #[cfg_attr(any(), java_method(name = "alignedSlice", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public final"))]
    pub fn alignedSlice(&self, unitSize: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.position()?;
        let mut pos: i32 = _t0;
        let _t1 = this.limit()?;
        let mut lim: i32 = _t1;
        let _t2 = this.alignmentOffset(pos, unitSize)?;
        let mut pos_mod: i32 = _t2;
        let _t3 = this.alignmentOffset(lim, unitSize)?;
        let mut lim_mod: i32 = _t3;
        let mut aligned_pos: i32 = pos;
        let mut aligned_lim: i32 = (lim).wrapping_sub(lim_mod);
        aligned_lim = pos;
        aligned_pos = pos;
        let _t4 = this.slice(aligned_pos, (aligned_lim).wrapping_sub(aligned_pos))?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_native(name = "getChar", descriptor = "()C", access = "public abstract"))]
    pub fn getChar(&self) -> Result<u16> {
        todo!("abstract java/nio/ByteBuffer.getChar")
    }

    #[cfg_attr(any(), java_native(name = "putChar", descriptor = "(C)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putChar__c(&self, arg0: u16) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putChar")
    }

    #[cfg_attr(any(), java_native(name = "getChar", descriptor = "(I)C", access = "public abstract"))]
    pub fn getChar__i(&self, arg0: i32) -> Result<u16> {
        todo!("abstract java/nio/ByteBuffer.getChar")
    }

    #[cfg_attr(any(), java_native(name = "putChar", descriptor = "(IC)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putChar__i_c(&self, arg0: i32, arg1: u16) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putChar")
    }

    #[cfg_attr(any(), java_native(name = "asCharBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn asCharBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asCharBuffer")
    }

    #[cfg_attr(any(), java_native(name = "getShort", descriptor = "()S", access = "public abstract"))]
    pub fn getShort(&self) -> Result<i16> {
        todo!("abstract java/nio/ByteBuffer.getShort")
    }

    #[cfg_attr(any(), java_native(name = "putShort", descriptor = "(S)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putShort__s(&self, arg0: i16) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putShort")
    }

    #[cfg_attr(any(), java_native(name = "getShort", descriptor = "(I)S", access = "public abstract"))]
    pub fn getShort__i(&self, arg0: i32) -> Result<i16> {
        todo!("abstract java/nio/ByteBuffer.getShort")
    }

    #[cfg_attr(any(), java_native(name = "putShort", descriptor = "(IS)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putShort__i_s(&self, arg0: i32, arg1: i16) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putShort")
    }

    #[cfg_attr(any(), java_native(name = "asShortBuffer", descriptor = "()Ljava/nio/ShortBuffer;", access = "public abstract"))]
    pub fn asShortBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asShortBuffer")
    }

    #[cfg_attr(any(), java_native(name = "getInt", descriptor = "()I", access = "public abstract"))]
    pub fn getInt(&self) -> Result<i32> {
        todo!("abstract java/nio/ByteBuffer.getInt")
    }

    #[cfg_attr(any(), java_native(name = "putInt", descriptor = "(I)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putInt__i(&self, arg0: i32) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putInt")
    }

    #[cfg_attr(any(), java_native(name = "getInt", descriptor = "(I)I", access = "public abstract"))]
    pub fn getInt__i(&self, arg0: i32) -> Result<i32> {
        todo!("abstract java/nio/ByteBuffer.getInt")
    }

    #[cfg_attr(any(), java_native(name = "putInt", descriptor = "(II)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putInt__i_i(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putInt")
    }

    #[cfg_attr(any(), java_native(name = "asIntBuffer", descriptor = "()Ljava/nio/IntBuffer;", access = "public abstract"))]
    pub fn asIntBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asIntBuffer")
    }

    #[cfg_attr(any(), java_native(name = "getLong", descriptor = "()J", access = "public abstract"))]
    pub fn getLong(&self) -> Result<i64> {
        todo!("abstract java/nio/ByteBuffer.getLong")
    }

    #[cfg_attr(any(), java_native(name = "putLong", descriptor = "(J)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putLong__l(&self, arg0: i64) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putLong")
    }

    #[cfg_attr(any(), java_native(name = "getLong", descriptor = "(I)J", access = "public abstract"))]
    pub fn getLong__i(&self, arg0: i32) -> Result<i64> {
        todo!("abstract java/nio/ByteBuffer.getLong")
    }

    #[cfg_attr(any(), java_native(name = "putLong", descriptor = "(IJ)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putLong__i_l(&self, arg0: i32, arg1: i64) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putLong")
    }

    #[cfg_attr(any(), java_native(name = "asLongBuffer", descriptor = "()Ljava/nio/LongBuffer;", access = "public abstract"))]
    pub fn asLongBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asLongBuffer")
    }

    #[cfg_attr(any(), java_native(name = "getFloat", descriptor = "()F", access = "public abstract"))]
    pub fn getFloat(&self) -> Result<f32> {
        todo!("abstract java/nio/ByteBuffer.getFloat")
    }

    #[cfg_attr(any(), java_native(name = "putFloat", descriptor = "(F)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putFloat__f(&self, arg0: f32) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putFloat")
    }

    #[cfg_attr(any(), java_native(name = "getFloat", descriptor = "(I)F", access = "public abstract"))]
    pub fn getFloat__i(&self, arg0: i32) -> Result<f32> {
        todo!("abstract java/nio/ByteBuffer.getFloat")
    }

    #[cfg_attr(any(), java_native(name = "putFloat", descriptor = "(IF)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putFloat__i_f(&self, arg0: i32, arg1: f32) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putFloat")
    }

    #[cfg_attr(any(), java_native(name = "asFloatBuffer", descriptor = "()Ljava/nio/FloatBuffer;", access = "public abstract"))]
    pub fn asFloatBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asFloatBuffer")
    }

    #[cfg_attr(any(), java_native(name = "getDouble", descriptor = "()D", access = "public abstract"))]
    pub fn getDouble(&self) -> Result<f64> {
        todo!("abstract java/nio/ByteBuffer.getDouble")
    }

    #[cfg_attr(any(), java_native(name = "putDouble", descriptor = "(D)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putDouble__d(&self, arg0: f64) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putDouble")
    }

    #[cfg_attr(any(), java_native(name = "getDouble", descriptor = "(I)D", access = "public abstract"))]
    pub fn getDouble__i(&self, arg0: i32) -> Result<f64> {
        todo!("abstract java/nio/ByteBuffer.getDouble")
    }

    #[cfg_attr(any(), java_native(name = "putDouble", descriptor = "(ID)Ljava/nio/ByteBuffer;", access = "public abstract"))]
    pub fn putDouble__i_d(&self, arg0: i32, arg1: f64) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.putDouble")
    }

    #[cfg_attr(any(), java_native(name = "asDoubleBuffer", descriptor = "()Ljava/nio/DoubleBuffer;", access = "public abstract"))]
    pub fn asDoubleBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/ByteBuffer.asDoubleBuffer")
    }
}
