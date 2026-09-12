#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/CharBuffer",
    super_class = "java/nio/Buffer",
    interfaces  = "java/lang/Comparable,java/lang/Appendable,java/lang/CharSequence,java/lang/Readable",
    access      = "public abstract",
    source      = "CharBuffer.java",
))]
pub struct CharBuffer {
    #[cfg_attr(any(), java_field(name = "hb", descriptor = "[C", access = "final"))]
    pub hb: Field<Vec<u16>>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "isReadOnly", descriptor = "Z"))]
    pub isReadOnly: Field<bool>,
}

impl CharBuffer {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IIII[CILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>(IIII[CILjava/lang/foreign/MemorySegment;)V
    pub fn new__i_i_i_i_arr_c_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, hb: Vec<u16>, offset: i32, segment: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false) };
        /* invokespecial Method java/nio/Buffer.<init>:(IIIILjava/lang/foreign/MemorySegment;)V */
        this.hb.set(hb);
        this.offset.set(offset);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IIIILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>(IIIILjava/lang/foreign/MemorySegment;)V
    pub fn new__i_i_i_i_memory(mark: i32, pos: i32, lim: i32, cap: i32, segment: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/nio/CharBuffer.<init>:(IIII[CILjava/lang/foreign/MemorySegment;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([CJILjava/lang/foreign/MemorySegment;)V"))]
    // java: <init>([CJILjava/lang/foreign/MemorySegment;)V
    pub fn new__arr_c_l_i_memory(hb: Vec<u16>, addr: i64, arg_2: i32, cap: Object) -> Result<Self> {
        let this = Self { hb: Field::new(Default::default()), offset: Field::new(0), isReadOnly: Field::new(false) };
        /* invokespecial Method java/nio/Buffer.<init>:(JILjava/lang/foreign/MemorySegment;)V */
        this.hb.set(hb);
        this.offset.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "base", descriptor = "()Ljava/lang/Object;"))]
    pub fn base(&self) -> Result<Object> {
        let this = self;
        Ok(this.hb.get())
    }

    #[cfg_attr(any(), java_method(name = "allocate", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public static"))]
    pub fn allocate(capacity: i32) -> Result<Object> {
        let _t0: Object = CharBuffer::createCapacityException(capacity)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        let mut _obj1: HeapCharBuffer = HeapCharBuffer::new(HeapCharBuffer::new(), capacity, capacity)?;
        Ok(_obj1)
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public static"))]
    // java: wrap([CII)Ljava/nio/CharBuffer;
    pub fn wrap__arr_c_i_i(array: &[u16], offset: i32, length: i32) -> Result<Object> {
        /* TODO: aconst_null  */
        let mut _obj0: HeapCharBuffer = HeapCharBuffer::new(HeapCharBuffer::new(), array, offset, length)?;
        return Ok(_obj0);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public static"))]
    // java: wrap([C)Ljava/nio/CharBuffer;
    pub fn wrap__arr_c(array: &[u16]) -> Result<Object> {
        let _t0: Object = CharBuffer::wrap(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public"))]
    pub fn read(&self, target: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.limit()?;
        let mut limit: i32 = _t0;
        let _t1 = this.position()?;
        let mut pos: i32 = _t1;
        let mut remaining: i32 = (limit).wrapping_sub(pos);
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(-1i32);
        let _t2 = target.remaining()?;
        let mut targetRemaining: i32 = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(0i32);
        let _t3: i32 = (remaining).min(targetRemaining);
        let mut n: i32 = _t3;
        let _t4 = this.limit((pos).wrapping_add(n))?;
        let _t5 = target.put(this)?;
        let _t6 = this.limit(limit)?;
        let mut local_7: i32 = n;
        let _t7 = this.limit(limit)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "public static"))]
    // java: wrap(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;
    pub fn wrap__seq_i_i(csq: Object, start: i32, end: i32) -> Result<Object> {
        return Ok(StringCharBuffer::new(csq, start, end)?);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "wrap", descriptor = "(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;", access = "public static"))]
    // java: wrap(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;
    pub fn wrap__seq(csq: Object) -> Result<Object> {
        let _t0 = csq.length()?;
        let _t1: Object = CharBuffer::wrap(csq, 0i32, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_native(name = "slice", descriptor = "()Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn slice(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.slice")
    }

    #[cfg_attr(any(), java_native(name = "slice", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn slice__i_i(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.slice")
    }

    #[cfg_attr(any(), java_native(name = "duplicate", descriptor = "()Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn duplicate(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.duplicate")
    }

    #[cfg_attr(any(), java_native(name = "asReadOnlyBuffer", descriptor = "()Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn asReadOnlyBuffer(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.asReadOnlyBuffer")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "()C", access = "public abstract"))]
    pub fn get(&self) -> Result<u16> {
        todo!("abstract java/nio/CharBuffer.get")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(C)Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn put__c(&self, arg0: u16) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.put")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(I)C", access = "public abstract"))]
    pub fn get__i(&self, arg0: i32) -> Result<u16> {
        todo!("abstract java/nio/CharBuffer.get")
    }

    #[cfg_attr(any(), java_native(name = "getUnchecked", descriptor = "(I)C", access = "abstract"))]
    pub fn getUnchecked(&self, arg0: i32) -> Result<u16> {
        todo!("abstract java/nio/CharBuffer.getUnchecked")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(IC)Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn put__i_c(&self, arg0: i32, arg1: u16) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.put")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public"))]
    // java: get([CII)Ljava/nio/CharBuffer;
    pub fn get__arr_c_i_i(&self, dst: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
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

    #[cfg_attr(any(), java_method(name = "get", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public"))]
    // java: get([C)Ljava/nio/CharBuffer;
    pub fn get__arr_c(&self, dst: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0 = this.get(dst, 0i32, (dst.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public"))]
    // java: get(I[CII)Ljava/nio/CharBuffer;
    pub fn get__i_arr_c_i_i(&self, index: i32, dst: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.limit()?;
        let _t1: i32 = Objects::checkFromIndexSize(index, length, _t0)?;
        let _t2: i32 = Objects::checkFromIndexSize(offset, length, (dst.len() as i32))?;
        let _t3 = this.getArray(index, dst, offset, length)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I[C)Ljava/nio/CharBuffer;", access = "public"))]
    // java: get(I[C)Ljava/nio/CharBuffer;
    pub fn get__i_arr_c(&self, index: i32, dst: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0 = this.get(index, dst, 0i32, (dst.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getArray", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "private"))]
    pub fn getArray(&self, index: i32, dst: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.isAddressable()?;
        /* TODO: lshl  */
        /* TODO: lcmp  */
        /* TODO: lshl  */
        let mut bufAddr: i64 = ((index as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut dstOffset: i64 = ((offset as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut len: i64 = 1i32;
        let _t1 = this.order()?;
        let _t2: Object = ByteOrder::nativeOrder()?;
        let _t3 = this.session()?;
        /* TODO: aconst_null  */
        let _t4 = this.base()?;
        _t2.copySwapMemory(CharBuffer::SCOPED_MEMORY_ACCESS(), _t3, _t4, bufAddr, dst, dstOffset, len, 2i64)?;
        let _t5 = this.session()?;
        /* TODO: aconst_null  */
        let _t6 = this.base()?;
        _t1.copyMemory(CharBuffer::SCOPED_MEMORY_ACCESS(), _t5, _t6, bufAddr, dst, dstOffset, len)?;
        Reference::reachabilityFence(this)?;
        let mut local_11: i64 = (length as i64);
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

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put(Ljava/nio/CharBuffer;)Ljava/nio/CharBuffer;
    pub fn put__charbu(&self, src: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = CharBuffer::createSameBufferException()?;
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

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put(ILjava/nio/CharBuffer;II)Ljava/nio/CharBuffer;
    pub fn put__i_charbu_i_i(&self, index: i32, src: Object, offset: i32, length: i32) -> Result<Object> {
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

    #[cfg_attr(any(), java_method(name = "putBuffer", descriptor = "(ILjava/nio/CharBuffer;II)V"))]
    pub fn putBuffer(&self, pos: i32, src: Object, srcPos: i32, n: i32) -> Result<()> {
        let this = self;
        let _t0 = src.base()?;
        let mut srcBase: Object = _t0;
        let _t1 = src.isAddressable()?;
        let _t2 = this.base()?;
        let mut base: Object = _t2;
        let _t3 = this.isDirect()?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lshl  */
        let mut srcAddr: i64 = ((srcPos as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut addr: i64 = ((pos as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut len: i64 = 1i32;
        let _t4 = this.order()?;
        let _t5 = src.order()?;
        let _t6 = src.session()?;
        let _t7 = this.session()?;
        CharBuffer::SCOPED_MEMORY_ACCESS().copySwapMemory(_t6, _t7, srcBase, srcAddr, base, addr, len, 2i64)?;
        let _t8 = src.session()?;
        let _t9 = this.session()?;
        CharBuffer::SCOPED_MEMORY_ACCESS().copyMemory(_t8, _t9, srcBase, srcAddr, base, addr, len)?;
        Reference::reachabilityFence(src)?;
        Reference::reachabilityFence(this)?;
        let mut local_13: Object = _t5;
        Reference::reachabilityFence(src)?;
        Reference::reachabilityFence(this)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = 76i32.isInstance(src)?;
        return Err(JvmError::Custom(String::from("athrow")));
        base = (pos).wrapping_add(n);
        srcAddr = pos;
        let mut j: i32 = srcPos;
        loop {
            if srcAddr >= base { break; }
            let _t0 = src.get(j)?;
            let _t1 = this.put(srcAddr, _t0)?;
            srcAddr = srcAddr.wrapping_add(1i32);
            j = j.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "([CII)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put([CII)Ljava/nio/CharBuffer;
    pub fn put__arr_c_i_i(&self, src: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
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

    #[cfg_attr(any(), java_method(name = "put", descriptor = "([C)Ljava/nio/CharBuffer;", access = "public final"))]
    // java: put([C)Ljava/nio/CharBuffer;
    pub fn put__arr_c(&self, src: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0 = this.put(src, 0i32, (src.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(I[CII)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put(I[CII)Ljava/nio/CharBuffer;
    pub fn put__i_arr_c_i_i(&self, index: i32, src: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.limit()?;
        let _t2: i32 = Objects::checkFromIndexSize(index, length, _t1)?;
        let _t3: i32 = Objects::checkFromIndexSize(offset, length, (src.len() as i32))?;
        let _t4 = this.putArray(index, src, offset, length)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(I[C)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put(I[C)Ljava/nio/CharBuffer;
    pub fn put__i_arr_c(&self, index: i32, src: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0 = this.put(index, src, 0i32, (src.len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "putArray", descriptor = "(I[CII)Ljava/nio/CharBuffer;"))]
    pub fn putArray(&self, index: i32, src: Vec<u16>, offset: i32, length: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.isAddressable()?;
        /* TODO: lshl  */
        /* TODO: lcmp  */
        /* TODO: lshl  */
        let mut bufAddr: i64 = ((index as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut srcOffset: i64 = ((offset as i64)).wrapping_add(1i32);
        /* TODO: lshl  */
        let mut len: i64 = 1i32;
        let _t1 = this.order()?;
        let _t2: Object = ByteOrder::nativeOrder()?;
        /* TODO: aconst_null  */
        let _t3 = this.session()?;
        let _t4 = this.base()?;
        _t2.copySwapMemory(CharBuffer::SCOPED_MEMORY_ACCESS(), _t3, src, srcOffset, _t4, bufAddr, len, 2i64)?;
        /* TODO: aconst_null  */
        let _t5 = this.session()?;
        let _t6 = this.base()?;
        _t1.copyMemory(CharBuffer::SCOPED_MEMORY_ACCESS(), _t5, src, srcOffset, _t6, bufAddr, len)?;
        Reference::reachabilityFence(this)?;
        let mut local_11: i64 = (length as i64);
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

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/String;II)Ljava/nio/CharBuffer;", access = "public"))]
    // java: put(Ljava/lang/String;II)Ljava/nio/CharBuffer;
    pub fn put__str_i_i(&self, src: String, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0 = src.length()?;
        let _t1: i32 = Objects::checkFromIndexSize(start, (end).wrapping_sub(start), _t0)?;
        let _t2 = this.isReadOnly()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = this.remaining()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut i: i32 = start;
        loop {
            if i >= end { break; }
            let _t0 = src.charAt(i)?;
            let _t1 = this.put(_t0)?;
            i = i.wrapping_add(1i32);
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/String;)Ljava/nio/CharBuffer;", access = "public final"))]
    // java: put(Ljava/lang/String;)Ljava/nio/CharBuffer;
    pub fn put__str(&self, src: String) -> Result<Object> {
        let this = self;
        let _t0 = src.length()?;
        let _t1 = this.put(src, 0i32, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "hasArray", descriptor = "()Z", access = "public final"))]
    pub fn hasArray(&self) -> Result<bool> {
        let this = self;
        Ok(this.isReadOnly.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "array", descriptor = "()[C", access = "public final"))]
    pub fn array(&self) -> Result<Vec<u16>> {
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

    #[cfg_attr(any(), java_method(name = "position", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn position(&self, newPosition: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::position(newPosition)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "limit", descriptor = "(I)Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn limit(&self, newLimit: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::limit(newLimit)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "mark", descriptor = "()Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn mark(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::mark()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn reset(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::reset()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn clear(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::clear()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "flip", descriptor = "()Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn flip(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::flip()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "rewind", descriptor = "()Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn rewind(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Buffer::rewind()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "compact", descriptor = "()Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn compact(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.compact")
    }

    #[cfg_attr(any(), java_native(name = "isDirect", descriptor = "()Z", access = "public abstract"))]
    pub fn isDirect(&self) -> Result<bool> {
        todo!("abstract java/nio/CharBuffer.isDirect")
    }

    #[cfg_attr(any(), java_method(name = "isAddressable", descriptor = "()Z"))]
    pub fn isAddressable(&self) -> Result<bool> {
        let this = self;
        Ok(1i32)
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

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public"))]
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
        let _t8: i32 = CharBuffer::compare(_t6, _t7)?;
        return Ok(_t8);
        Ok((thisRem).wrapping_sub(thatRem))
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(CC)I", access = "private static"))]
    pub fn compare(x: u16, y: u16) -> Result<i32> {
        let _t0: i32 = Character::compare(x, y)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "mismatch", descriptor = "(Ljava/nio/CharBuffer;)I", access = "public"))]
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

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.position()?;
        let _t1 = this.limit()?;
        let _t2 = this.toString(_t0, _t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "toString", descriptor = "(II)Ljava/lang/String;", access = "abstract"))]
    pub fn toString__i_i(&self, arg0: i32, arg1: i32) -> Result<String> {
        todo!("abstract java/nio/CharBuffer.toString")
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "()I", access = "public final"))]
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.remaining()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public final"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.remaining()?;
        Ok(_t0==0i32)
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "(I)C", access = "public final"))]
    pub fn charAt(&self, index: i32) -> Result<u16> {
        let this = self;
        let _t0 = this.position()?;
        let _t1 = this.checkIndex(index, 1i32)?;
        let _t2 = this.get((_t0).wrapping_add(_t1))?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "subSequence", descriptor = "(II)Ljava/nio/CharBuffer;", access = "public abstract"))]
    pub fn subSequence(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.subSequence")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;
    pub fn append__seq(&self, csq: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.put(String::from("null"))?;
        return Ok(_t0);
        let mut cb: Object = csq;
        let _t1 = this.put(cb)?;
        return Ok(_t1);
        let _t2 = csq.toString()?;
        let _t3 = this.put(_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;II)Ljava/nio/CharBuffer;
    pub fn append__seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let mut cb: Object = csq;
        let _t0 = csq.length()?;
        let _t1: i32 = Objects::checkFromToIndex(start, end, _t0)?;
        let mut length: i32 = (end).wrapping_sub(start);
        let _t2 = this.position()?;
        let mut pos: i32 = _t2;
        let _t3 = this.limit()?;
        let mut lim: i32 = _t3;
        let mut rem: i32 = 0i32;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = this.put(pos, cb, start, length)?;
        let _t5 = this.position((pos).wrapping_add(length))?;
        return Ok(this);
        cb = csq;
        let _t6 = cb.subSequence(start, end)?;
        let _t7 = _t6.toString()?;
        let _t8 = this.put(_t7)?;
        Ok(_t8)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/nio/CharBuffer;", access = "public"))]
    // java: append(C)Ljava/nio/CharBuffer;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        let _t0 = this.put(c)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "order", descriptor = "()Ljava/nio/ByteOrder;", access = "public abstract"))]
    pub fn order(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.order")
    }

    #[cfg_attr(any(), java_native(name = "charRegionOrder", descriptor = "()Ljava/nio/ByteOrder;", access = "abstract"))]
    pub fn charRegionOrder(&self) -> Result<Object> {
        todo!("abstract java/nio/CharBuffer.charRegionOrder")
    }

    #[cfg_attr(any(), java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public"))]
    pub fn chars(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 278 */
        let _t0: Object = StreamSupport::intStream(this, 16464i32, 0i32)?;
        Ok(_t0)
    }
}
