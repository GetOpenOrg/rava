#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

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
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    pub fn new(capacity: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(str: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/CharSequence;)V", access = "public"))]
    pub fn new(seq: JvmObject) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/StringBuilder;)I", access = "public"))]
    pub fn compareTo(&self, another: JvmObject) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::compareTo(another)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, obj: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&String::from_owned(format!("{}", obj)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, str: String) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, sb: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(sb)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, s: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(s)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, s: JvmObject, start: i32, end: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(s, start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([C)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, str: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([CII)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, str: JvmObject, offset: i32, len: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(str, offset, len)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Z)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, b: bool) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(b)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, c: u16) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, i: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(i)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(J)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, lng: i64) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(lng)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(F)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, f: f32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(f)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(D)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn append(&self, d: f64) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::append(d)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::appendCodePoint(codePoint)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "delete", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn delete(&self, start: i32, end: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::delete(start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn deleteCharAt(&self, index: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::deleteCharAt(index)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::replace(start, end, str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, index: i32, str: JvmObject, offset: i32, len: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(index, str, offset, len)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, obj: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, obj)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, str: String) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[C)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, str: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, str)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, dstOffset: i32, s: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(dstOffset, s)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, dstOffset: i32, s: JvmObject, start: i32, end: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(dstOffset, s, start, end)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IZ)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, b: bool) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, b)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IC)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, c: u16) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, i: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, i)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IJ)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, l: i64) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, l)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IF)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, f: f32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, f)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ID)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn insert(&self, offset: i32, d: f64) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::insert(offset, d)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    pub fn indexOf(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public"))]
    pub fn indexOf(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::indexOf(str, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    pub fn lastIndexOf(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public"))]
    pub fn lastIndexOf(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::lastIndexOf(str, fromIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "()Ljava/lang/StringBuilder;", access = "public"))]
    pub fn reverse(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::reverse()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn repeat(&self, codePoint: i32, count: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::repeat(codePoint, count)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;", access = "public"))]
    pub fn repeat(&self, cs: JvmObject, count: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::repeat(cs, count)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        s.defaultWriteObject()?;
        s.writeInt(this.count.get())?;
        let _t0 = this.capacity()?;
        let mut _arr1: Vec<u16> = vec![0u16; _t0 as usize];
        let mut val: Vec<u16> = _arr1;
        let _t2 = this.isLatin1()?;
        StringLatin1::getChars(this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        StringUTF16::getChars(this.value.get(), 0i32, this.count.get(), &val, 0i32)?;
        s.writeObject(val)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let mut c: i32 = _t0;
        let _t1 = s.readObject()?;
        let mut val: JvmObject = _t1;
        panic!("{}", /* StreamCorruptedException::new(String::from("count value invalid"))? */);
        this.initBytes(val, 0i32, (val.len() as i32))?;
        this.count.set(c);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn repeat(&self, arg_0: JvmObject, arg_1: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.repeat(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn repeat(&self, arg_0: i32, arg_1: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.repeat(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public"))]
    pub fn codePoints(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::codePoints()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public"))]
    pub fn chars(&self) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::chars()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "()Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn reverse(&self) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.reverse()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ID)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: f64) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IF)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: f32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IJ)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: i64) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IC)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: u16) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IZ)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: bool) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: JvmObject, arg_2: i32, arg_3: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1, arg_2, arg_3)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[C)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: String) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn insert(&self, arg_0: i32, arg_1: JvmObject, arg_2: i32, arg_3: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.insert(arg_0, arg_1, arg_2, arg_3)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "substring", descriptor = "(II)Ljava/lang/String;", access = "public"))]
    pub fn substring(&self, arg_0: i32, arg_1: i32) -> Result<String> {
        let this = self;
        let _t0: String = AbstractStringBuilder::substring(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "subSequence", descriptor = "(II)Ljava/lang/CharSequence;", access = "public"))]
    pub fn subSequence(&self, arg_0: i32, arg_1: i32) -> Result<JvmObject> {
        let this = self;
        let _t0: JvmObject = AbstractStringBuilder::subSequence(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "substring", descriptor = "(I)Ljava/lang/String;", access = "public"))]
    pub fn substring(&self, arg_0: i32) -> Result<String> {
        let this = self;
        let _t0: String = AbstractStringBuilder::substring(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn replace(&self, arg_0: i32, arg_1: i32, arg_2: String) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.replace(arg_0, arg_1, arg_2)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn deleteCharAt(&self, arg_0: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.deleteCharAt(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn appendCodePoint(&self, arg_0: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.appendCodePoint(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "delete", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn delete(&self, arg_0: i32, arg_1: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.delete(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(D)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: f64) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(F)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: f32) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(J)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: i64) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: i32) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: u16) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Z)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: bool) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([CII)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject, arg_1: i32, arg_2: i32) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([C)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject, arg_1: i32, arg_2: i32) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: String) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "setCharAt", descriptor = "(IC)V", access = "public"))]
    pub fn setCharAt(&self, arg_0: i32, arg_1: u16) -> Result<()> {
        let this = self;
        AbstractStringBuilder::setCharAt(arg_0, arg_1)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(II[CI)V", access = "public"))]
    pub fn getChars(&self, arg_0: i32, arg_1: i32, arg_2: JvmObject, arg_3: i32) -> Result<()> {
        let this = self;
        AbstractStringBuilder::getChars(arg_0, arg_1, arg_2, arg_3)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "offsetByCodePoints", descriptor = "(II)I", access = "public"))]
    pub fn offsetByCodePoints(&self, arg_0: i32, arg_1: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::offsetByCodePoints(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointCount", descriptor = "(II)I", access = "public"))]
    pub fn codePointCount(&self, arg_0: i32, arg_1: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointCount(arg_0, arg_1)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointBefore", descriptor = "(I)I", access = "public"))]
    pub fn codePointBefore(&self, arg_0: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointBefore(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "codePointAt", descriptor = "(I)I", access = "public"))]
    pub fn codePointAt(&self, arg_0: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::codePointAt(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "(I)C", access = "public"))]
    pub fn charAt(&self, arg_0: i32) -> Result<u16> {
        let this = self;
        let _t0: u16 = AbstractStringBuilder::charAt(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "setLength", descriptor = "(I)V", access = "public"))]
    pub fn setLength(&self, arg_0: i32) -> Result<()> {
        let this = self;
        AbstractStringBuilder::setLength(arg_0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "trimToSize", descriptor = "()V", access = "public"))]
    pub fn trimToSize(&self) -> Result<()> {
        let this = self;
        AbstractStringBuilder::trimToSize()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ensureCapacity", descriptor = "(I)V", access = "public"))]
    pub fn ensureCapacity(&self, arg_0: i32) -> Result<()> {
        let this = self;
        AbstractStringBuilder::ensureCapacity(arg_0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "capacity", descriptor = "()I", access = "public"))]
    pub fn capacity(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::capacity()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "length", descriptor = "()I", access = "public"))]
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = AbstractStringBuilder::length()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: u16) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject, arg_1: i32, arg_2: i32) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.append(&arg_0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn compareTo(&self, arg_0: JvmObject) -> Result<i32> {
        let this = self;
        let _t0 = this.compareTo(arg_0)?;
        Ok(_t0)
    }
}
