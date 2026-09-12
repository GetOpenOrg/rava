#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StringBuilder",
    super_class = "java/lang/AbstractStringBuilder",
    interfaces  = "java/lang/Appendable,java/io/Serializable,java/lang/Comparable,java/lang/CharSequence",
    access      = "public final",
    source      = "StringBuilder.java",
))]
pub struct StringBuilder;

impl StringBuilder {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(capacity: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(str: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/CharSequence;)V
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new__seq(seq: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V */
        Ok(this)
    }

    // java: compareTo(Ljava/lang/StringBuilder;)I
    pub fn compareTo(&self, another: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::compareTo(another)?;
        Ok(_t0)
    }

    // java: append(Ljava/lang/Object;)Ljava/lang/StringBuilder;
    // java: append(Ljava/lang/Object;)Ljava/lang/StringBuilder;
    pub fn append__obj(&self, obj: Object) -> Result<Object> {
        let this = self;
        this.append(&String::from_owned(format!("{}", obj)))?;
        Ok(this)
    }

    // java: append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    // java: append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    pub fn append__str(&self, str: String) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(str)?;
        Ok(this)
    }

    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;
    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;
    pub fn append__string(&self, sb: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(sb)?;
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
    // java: append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
    pub fn append__seq(&self, s: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(s)?;
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    pub fn append__seq_i_i(&self, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(s, start, end)?;
        Ok(this)
    }

    // java: append([C)Ljava/lang/StringBuilder;
    // java: append([C)Ljava/lang/StringBuilder;
    pub fn append__arr_c(&self, str: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(&str)?;
        Ok(this)
    }

    // java: append([CII)Ljava/lang/StringBuilder;
    // java: append([CII)Ljava/lang/StringBuilder;
    pub fn append__arr_c_i_i(&self, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(&str, offset, len)?;
        Ok(this)
    }

    // java: append(Z)Ljava/lang/StringBuilder;
    // java: append(Z)Ljava/lang/StringBuilder;
    pub fn append__z(&self, b: bool) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(b)?;
        Ok(this)
    }

    // java: append(C)Ljava/lang/StringBuilder;
    // java: append(C)Ljava/lang/StringBuilder;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(c)?;
        Ok(this)
    }

    // java: append(I)Ljava/lang/StringBuilder;
    // java: append(I)Ljava/lang/StringBuilder;
    pub fn append__i(&self, i: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(i)?;
        Ok(this)
    }

    // java: append(J)Ljava/lang/StringBuilder;
    // java: append(J)Ljava/lang/StringBuilder;
    pub fn append__l(&self, lng: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(lng)?;
        Ok(this)
    }

    // java: append(F)Ljava/lang/StringBuilder;
    // java: append(F)Ljava/lang/StringBuilder;
    pub fn append__f(&self, f: f32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(f)?;
        Ok(this)
    }

    // java: append(D)Ljava/lang/StringBuilder;
    // java: append(D)Ljava/lang/StringBuilder;
    pub fn append__d(&self, d: f64) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::append(d)?;
        Ok(this)
    }

    // java: appendCodePoint(I)Ljava/lang/StringBuilder;
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::appendCodePoint(codePoint)?;
        Ok(this)
    }

    // java: delete(II)Ljava/lang/StringBuilder;
    pub fn delete(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::delete(start, end)?;
        Ok(this)
    }

    // java: deleteCharAt(I)Ljava/lang/StringBuilder;
    pub fn deleteCharAt(&self, index: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::deleteCharAt(index)?;
        Ok(this)
    }

    // java: replace(IILjava/lang/String;)Ljava/lang/StringBuilder;
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::replace(start, end, str)?;
        Ok(this)
    }

    // java: insert(I[CII)Ljava/lang/StringBuilder;
    // java: insert(I[CII)Ljava/lang/StringBuilder;
    pub fn insert__i_arr_c_i_i(&self, index: i32, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(index, &str, offset, len)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/Object;)Ljava/lang/StringBuilder;
    // java: insert(ILjava/lang/Object;)Ljava/lang/StringBuilder;
    pub fn insert__i_obj(&self, offset: i32, obj: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, obj)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/String;)Ljava/lang/StringBuilder;
    // java: insert(ILjava/lang/String;)Ljava/lang/StringBuilder;
    pub fn insert__i_str(&self, offset: i32, str: String) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, str)?;
        Ok(this)
    }

    // java: insert(I[C)Ljava/lang/StringBuilder;
    // java: insert(I[C)Ljava/lang/StringBuilder;
    pub fn insert__i_arr_c(&self, offset: i32, str: Vec<u16>) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, &str)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;
    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;
    pub fn insert__i_seq(&self, dstOffset: i32, s: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(dstOffset, s)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    pub fn insert__i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(dstOffset, s, start, end)?;
        Ok(this)
    }

    // java: insert(IZ)Ljava/lang/StringBuilder;
    // java: insert(IZ)Ljava/lang/StringBuilder;
    pub fn insert__i_z(&self, offset: i32, b: bool) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, b)?;
        Ok(this)
    }

    // java: insert(IC)Ljava/lang/StringBuilder;
    // java: insert(IC)Ljava/lang/StringBuilder;
    pub fn insert__i_c(&self, offset: i32, c: u16) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, c)?;
        Ok(this)
    }

    // java: insert(II)Ljava/lang/StringBuilder;
    // java: insert(II)Ljava/lang/StringBuilder;
    pub fn insert__i_i(&self, offset: i32, i: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, i)?;
        Ok(this)
    }

    // java: insert(IJ)Ljava/lang/StringBuilder;
    // java: insert(IJ)Ljava/lang/StringBuilder;
    pub fn insert__i_l(&self, offset: i32, l: i64) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, l)?;
        Ok(this)
    }

    // java: insert(IF)Ljava/lang/StringBuilder;
    // java: insert(IF)Ljava/lang/StringBuilder;
    pub fn insert__i_f(&self, offset: i32, f: f32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::insert(offset, f)?;
        Ok(this)
    }

    // java: insert(ID)Ljava/lang/StringBuilder;
    // java: insert(ID)Ljava/lang/StringBuilder;
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
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str)?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/String;I)I
    // java: lastIndexOf(Ljava/lang/String;I)I
    pub fn lastIndexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str, fromIndex)?;
        Ok(_t0)
    }

    // java: reverse()Ljava/lang/StringBuilder;
    pub fn reverse(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::reverse()?;
        Ok(this)
    }

    // java: repeat(II)Ljava/lang/StringBuilder;
    // java: repeat(II)Ljava/lang/StringBuilder;
    pub fn repeat__i_i(&self, codePoint: i32, count: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::repeat(codePoint, count)?;
        Ok(this)
    }

    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;
    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;
    pub fn repeat__seq_i(&self, cs: Object, count: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = AbstractStringBuilder::repeat(cs, count)?;
        Ok(this)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::new(this)?)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultWriteObject()?;
        s.writeInt(this.count.get())?;
        let _t0 = this.capacity()?;
        let mut _arr1: Vec<u16> = vec![0u16; _t0 as usize];
        let mut val: Vec<u16> = _arr1;
        let _t2 = this.isLatin1()?;
        StringLatin1::getChars(&this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        StringUTF16::getChars(&this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        s.writeObject(val)?;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let mut c: i32 = _t0;
        let _t1 = s.readObject()?;
        let mut val: Object = _t1;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.initBytes(val, 0i32, (val.len() as i32))?;
        this.count.set(c);
        Ok(())
    }
}
