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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    // java: <init>(I)V
    pub fn new__i(capacity: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(str: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/CharSequence;)V", access = "public"))]
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new__seq(seq: CharSequence) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/StringBuilder;)I", access = "public"))]
    pub fn compareTo(&self, another: StringBuilder) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::compareTo(another)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Ljava/lang/Object;)Ljava/lang/StringBuilder;
    pub fn append__obj(&self, obj: Object) -> Result<StringBuilder> {
        let this = self;
        this.append(&String::from_owned(format!("{}", obj)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    pub fn append__str(&self, str: String) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;
    pub fn append__string(&self, sb: StringBuffer) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(sb)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
    pub fn append__seq(&self, s: CharSequence) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(s)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    pub fn append__seq_i_i(&self, s: CharSequence, start: i32, end: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(s, start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([C)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append([C)Ljava/lang/StringBuilder;
    pub fn append__arr_c(&self, str: Vec<u16>) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(&str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([CII)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append([CII)Ljava/lang/StringBuilder;
    pub fn append__arr_c_i_i(&self, str: Vec<u16>, offset: i32, len: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(&str, offset, len)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Z)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(Z)Ljava/lang/StringBuilder;
    pub fn append__z(&self, b: bool) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(b)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(C)Ljava/lang/StringBuilder;
    pub fn append__c(&self, c: u16) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(I)Ljava/lang/StringBuilder;
    pub fn append__i(&self, i: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(i)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(J)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(J)Ljava/lang/StringBuilder;
    pub fn append__l(&self, lng: i64) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(lng)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(F)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(F)Ljava/lang/StringBuilder;
    pub fn append__f(&self, f: f32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(f)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(D)Ljava/lang/StringBuilder;", access = "public"))]
    // java: append(D)Ljava/lang/StringBuilder;
    pub fn append__d(&self, d: f64) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::append(d)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::appendCodePoint(codePoint)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "delete", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn delete(&self, start: i32, end: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::delete(start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn deleteCharAt(&self, index: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::deleteCharAt(index)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::replace(start, end, str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(I[CII)Ljava/lang/StringBuilder;
    pub fn insert__i_arr_c_i_i(&self, index: i32, str: Vec<u16>, offset: i32, len: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(index, &str, offset, len)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(ILjava/lang/Object;)Ljava/lang/StringBuilder;
    pub fn insert__i_obj(&self, offset: i32, obj: Object) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, obj)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(ILjava/lang/String;)Ljava/lang/StringBuilder;
    pub fn insert__i_str(&self, offset: i32, str: String) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[C)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(I[C)Ljava/lang/StringBuilder;
    pub fn insert__i_arr_c(&self, offset: i32, str: Vec<u16>) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, &str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;
    pub fn insert__i_seq(&self, dstOffset: i32, s: CharSequence) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(dstOffset, s)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;
    pub fn insert__i_seq_i_i(&self, dstOffset: i32, s: CharSequence, start: i32, end: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(dstOffset, s, start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IZ)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(IZ)Ljava/lang/StringBuilder;
    pub fn insert__i_z(&self, offset: i32, b: bool) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, b)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IC)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(IC)Ljava/lang/StringBuilder;
    pub fn insert__i_c(&self, offset: i32, c: u16) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(II)Ljava/lang/StringBuilder;
    pub fn insert__i_i(&self, offset: i32, i: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, i)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IJ)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(IJ)Ljava/lang/StringBuilder;
    pub fn insert__i_l(&self, offset: i32, l: i64) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, l)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IF)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(IF)Ljava/lang/StringBuilder;
    pub fn insert__i_f(&self, offset: i32, f: f32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, f)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ID)Ljava/lang/StringBuilder;", access = "public"))]
    // java: insert(ID)Ljava/lang/StringBuilder;
    pub fn insert__i_d(&self, offset: i32, d: f64) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::insert(offset, d)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    // java: indexOf(Ljava/lang/String;)I
    pub fn indexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public"))]
    // java: indexOf(Ljava/lang/String;I)I
    pub fn indexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    // java: lastIndexOf(Ljava/lang/String;)I
    pub fn lastIndexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public"))]
    // java: lastIndexOf(Ljava/lang/String;I)I
    pub fn lastIndexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "()Ljava/lang/StringBuilder;", access = "public"))]
    pub fn reverse(&self) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::reverse()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    // java: repeat(II)Ljava/lang/StringBuilder;
    pub fn repeat__i_i(&self, codePoint: i32, count: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::repeat(codePoint, count)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;", access = "public"))]
    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;
    pub fn repeat__seq_i(&self, cs: CharSequence, count: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0: AbstractStringBuilder = AbstractStringBuilder::repeat(cs, count)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: ObjectOutputStream) -> Result<()> {
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

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: ObjectInputStream) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let mut c: i32 = _t0;
        let _t1 = s.readObject()?;
        let mut val: Object = _t1;
        return Err(JvmError::Custom(String::from("athrow")));
        this.initBytes(val, 0i32, (val.len() as i32))?;
        this.count.set(c);
        Ok(())
    }
}
