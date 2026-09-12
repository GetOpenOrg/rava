#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringBuffer",
    super_class = "java/lang/AbstractStringBuilder",
    interfaces  = "java/lang/Appendable,java/io/Serializable,java/lang/Comparable,java/lang/CharSequence",
    access      = "public final",
    source      = "StringBuffer.java",
))]
pub struct StringBuffer {
    #[cfg_attr(any(), java_field(name = "toStringCache", descriptor = "Ljava/lang/String;", access = "private"))]
    pub toStringCache: Field<String>,
}

impl StringBuffer {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { toStringCache: Field::new(String::new()) };
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(capacity: i32) -> Result<Self> {
        let this = Self { toStringCache: Field::new(String::new()) };
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(str: String) -> Result<Self> {
        let this = Self { toStringCache: Field::new(String::new()) };
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/CharSequence;)V
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new__seq(seq: Object) -> Result<Self> {
        let this = Self { toStringCache: Field::new(String::new()) };
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V */
        Ok(this)
    }

    // java: compareTo(Ljava/lang/StringBuffer;)I
    pub fn compareTo(&self, another: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::compareTo(another)?;
        Ok(_t0)
    }

    // java: length()I
    pub fn length(&self) -> Result<i32> {
        let this = self;
        Ok(this.count.get())
    }

    // java: capacity()I
    pub fn capacity(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::capacity()?;
        Ok(_t0)
    }

    // java: ensureCapacity(I)V
    pub fn ensureCapacity(&self, minimumCapacity: i32) -> Result<()> {
        let this = self;
        AbstractStringBuilder::ensureCapacity(minimumCapacity)?;
        Ok(())
    }

    // java: trimToSize()V
    pub fn trimToSize(&self) -> Result<()> {
        let this = self;
        AbstractStringBuilder::trimToSize()?;
        Ok(())
    }

    // java: setLength(I)V
    pub fn setLength(&self, newLength: i32) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        AbstractStringBuilder::setLength(newLength)?;
        Ok(())
    }

    // java: charAt(I)C
    pub fn charAt(&self, index: i32) -> Result<u16> {
        let this = self;
        let _t0: u16 = AbstractStringBuilder::charAt(index)?;
        Ok(_t0)
    }

    // java: codePointAt(I)I
    pub fn codePointAt(&self, index: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointAt(index)?;
        Ok(_t0)
    }

    // java: codePointBefore(I)I
    pub fn codePointBefore(&self, index: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointBefore(index)?;
        Ok(_t0)
    }

    // java: codePointCount(II)I
    pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointCount(beginIndex, endIndex)?;
        Ok(_t0)
    }

    // java: offsetByCodePoints(II)I
    pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::offsetByCodePoints(index, codePointOffset)?;
        Ok(_t0)
    }

    // java: getChars(II[CI)V
    pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Vec<u16>, dstBegin: i32) -> Result<()> {
        let this = self;
        AbstractStringBuilder::getChars(srcBegin, srcEnd, &dst, dstBegin)?;
        Ok(())
    }

    // java: setCharAt(IC)V
    pub fn setCharAt(&self, index: i32, ch: u16) -> Result<()> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        AbstractStringBuilder::setCharAt(index, ch)?;
        Ok(())
    }

    // java: append(Ljava/lang/Object;)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/Object;)Ljava/lang/StringBuffer;
    pub fn append__obj(&self, obj: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(String::from_owned(format!("{}", obj)))?;
        Ok(this)
    }

    // java: append(Ljava/lang/String;)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/String;)Ljava/lang/StringBuffer;
    pub fn append__str(&self, str: String) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(str)?;
        Ok(this)
    }

    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;
    pub fn append__string(&self, sb: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(sb)?;
        Ok(this)
    }

    // java: append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/StringBuffer;
    pub fn append__abstra(&self, asb: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(asb)?;
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/CharSequence;)Ljava/lang/StringBuffer;
    pub fn append__seq(&self, s: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(s)?;
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuffer;
    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuffer;
    pub fn append__seq_i_i(&self, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(s, start, end)?;
        Ok(this)
    }

    // java: append([C)Ljava/lang/StringBuffer;
    // java: append([C)Ljava/lang/StringBuffer;
    pub fn append__arr_c(&self, str: Vec<u16>) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(&str)?;
        Ok(this)
    }

    // java: append([CII)Ljava/lang/StringBuffer;
    // java: append([CII)Ljava/lang/StringBuffer;
    pub fn append__arr_c_i_i(&self, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(&str, offset, len)?;
        Ok(this)
    }

    // java: append(Z)Ljava/lang/StringBuffer;
    // java: append(Z)Ljava/lang/StringBuffer;
    pub fn append__z(&self, b: bool) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(b)?;
        Ok(this)
    }

    // java: append(C)Ljava/lang/StringBuffer;
    // java: append(C)Ljava/lang/StringBuffer;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(c)?;
        Ok(this)
    }

    // java: append(I)Ljava/lang/StringBuffer;
    // java: append(I)Ljava/lang/StringBuffer;
    pub fn append__i(&self, i: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(i)?;
        Ok(this)
    }

    // java: appendCodePoint(I)Ljava/lang/StringBuffer;
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::appendCodePoint(codePoint)?;
        Ok(this)
    }

    // java: append(J)Ljava/lang/StringBuffer;
    // java: append(J)Ljava/lang/StringBuffer;
    pub fn append__l(&self, lng: i64) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(lng)?;
        Ok(this)
    }

    // java: append(F)Ljava/lang/StringBuffer;
    // java: append(F)Ljava/lang/StringBuffer;
    pub fn append__f(&self, f: f32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(f)?;
        Ok(this)
    }

    // java: append(D)Ljava/lang/StringBuffer;
    // java: append(D)Ljava/lang/StringBuffer;
    pub fn append__d(&self, d: f64) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::append(d)?;
        Ok(this)
    }

    // java: delete(II)Ljava/lang/StringBuffer;
    pub fn delete(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::delete(start, end)?;
        Ok(this)
    }

    // java: deleteCharAt(I)Ljava/lang/StringBuffer;
    pub fn deleteCharAt(&self, index: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::deleteCharAt(index)?;
        Ok(this)
    }

    // java: replace(IILjava/lang/String;)Ljava/lang/StringBuffer;
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::replace(start, end, str)?;
        Ok(this)
    }

    // java: substring(I)Ljava/lang/String;
    // java: substring(I)Ljava/lang/String;
    pub fn substring__i(&self, start: i32) -> Result<String> {
        let this = self;
        let _t0 = this.substring(start, this.count.get())?;
        Ok(_t0)
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0: String = AbstractStringBuilder::substring(start, end)?;
        Ok(_t0)
    }

    // java: substring(II)Ljava/lang/String;
    // java: substring(II)Ljava/lang/String;
    pub fn substring__i_i(&self, start: i32, end: i32) -> Result<String> {
        let this = self;
        let _t0: String = AbstractStringBuilder::substring(start, end)?;
        Ok(_t0)
    }

    // java: insert(I[CII)Ljava/lang/StringBuffer;
    // java: insert(I[CII)Ljava/lang/StringBuffer;
    pub fn insert__i_arr_c_i_i(&self, index: i32, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(index, &str, offset, len)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/Object;)Ljava/lang/StringBuffer;
    // java: insert(ILjava/lang/Object;)Ljava/lang/StringBuffer;
    pub fn insert__i_obj(&self, offset: i32, obj: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(offset, String::from_owned(format!("{}", obj)))?;
        Ok(this)
    }

    // java: insert(ILjava/lang/String;)Ljava/lang/StringBuffer;
    // java: insert(ILjava/lang/String;)Ljava/lang/StringBuffer;
    pub fn insert__i_str(&self, offset: i32, str: String) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(offset, str)?;
        Ok(this)
    }

    // java: insert(I[C)Ljava/lang/StringBuffer;
    // java: insert(I[C)Ljava/lang/StringBuffer;
    pub fn insert__i_arr_c(&self, offset: i32, str: Vec<u16>) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(offset, &str)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;
    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuffer;
    pub fn insert__i_seq(&self, dstOffset: i32, s: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(dstOffset, s)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuffer;
    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuffer;
    pub fn insert__i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(dstOffset, s, start, end)?;
        Ok(this)
    }

    // java: insert(IZ)Ljava/lang/StringBuffer;
    // java: insert(IZ)Ljava/lang/StringBuffer;
    pub fn insert__i_z(&self, offset: i32, b: bool) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, b)?;
        Ok(this)
    }

    // java: insert(IC)Ljava/lang/StringBuffer;
    // java: insert(IC)Ljava/lang/StringBuffer;
    pub fn insert__i_c(&self, offset: i32, c: u16) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::insert(offset, c)?;
        Ok(this)
    }

    // java: insert(II)Ljava/lang/StringBuffer;
    // java: insert(II)Ljava/lang/StringBuffer;
    pub fn insert__i_i(&self, offset: i32, i: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, i)?;
        Ok(this)
    }

    // java: insert(IJ)Ljava/lang/StringBuffer;
    // java: insert(IJ)Ljava/lang/StringBuffer;
    pub fn insert__i_l(&self, offset: i32, l: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, l)?;
        Ok(this)
    }

    // java: insert(IF)Ljava/lang/StringBuffer;
    // java: insert(IF)Ljava/lang/StringBuffer;
    pub fn insert__i_f(&self, offset: i32, f: f32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, f)?;
        Ok(this)
    }

    // java: insert(ID)Ljava/lang/StringBuffer;
    // java: insert(ID)Ljava/lang/StringBuffer;
    pub fn insert__i_d(&self, offset: i32, d: f64) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, d)?;
        Ok(this)
    }

    // java: indexOf(Ljava/lang/String;)I
    // java: indexOf(Ljava/lang/String;)I
    pub fn indexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str)?;
        Ok(_t0)
    }

    // java: indexOf(Ljava/lang/String;I)I
    // java: indexOf(Ljava/lang/String;I)I
    pub fn indexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str, fromIndex)?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/String;)I
    // java: lastIndexOf(Ljava/lang/String;)I
    pub fn lastIndexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = this.lastIndexOf(str, this.count.get())?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/String;I)I
    // java: lastIndexOf(Ljava/lang/String;I)I
    pub fn lastIndexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str, fromIndex)?;
        Ok(_t0)
    }

    // java: reverse()Ljava/lang/StringBuffer;
    pub fn reverse(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::reverse()?;
        Ok(this)
    }

    // java: repeat(II)Ljava/lang/StringBuffer;
    // java: repeat(II)Ljava/lang/StringBuffer;
    pub fn repeat__i_i(&self, codePoint: i32, count: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::repeat(codePoint, count)?;
        Ok(this)
    }

    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/StringBuffer;
    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/StringBuffer;
    pub fn repeat__seq_i(&self, cs: Object, count: i32) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        todo!("stack underflow").toStringCache.set(this);
        let _t0: Object = AbstractStringBuilder::repeat(cs, count)?;
        Ok(this)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        /* TODO: aconst_null  */
        let mut _obj0: String = String::new(String::new(), this)?;
        this.toStringCache.set(_obj0);
        return Ok(_obj0);
        Ok(String::new(this.toStringCache.get())?)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.putFields()?;
        let mut fields: Object = _t0;
        let _t1 = this.capacity()?;
        let mut _arr2: Vec<u16> = vec![0u16; _t1 as usize];
        let mut val: Vec<u16> = _arr2;
        let _t3 = this.isLatin1()?;
        StringLatin1::getChars(&this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        StringUTF16::getChars(&this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        fields.put(String::from("value"), val)?;
        fields.put(String::from("count"), this.count.get())?;
        fields.put(String::from("shared"), 0i32)?;
        s.writeFields()?;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let mut fields: Object = _t0;
        /* TODO: aconst_null  */
        let _t1 = todo!("stack underflow").get(fields, String::from("value"))?;
        let mut val: Object = _t1;
        let _t2 = fields.get(String::from("count"), 0i32)?;
        let mut c: i32 = _t2;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.initBytes(val, 0i32, (val.len() as i32))?;
        this.count.set(c);
        Ok(())
    }

    // java: getBytes([BIB)V
    pub fn getBytes(&self, dst: Vec<i8>, dstBegin: i32, coder: i8) -> Result<()> {
        let this = self;
        AbstractStringBuilder::getBytes(&dst, dstBegin, coder)?;
        Ok(())
    }
}
