#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/Writer",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Appendable,java/io/Closeable,java/io/Flushable",
    access      = "public abstract",
    source      = "Writer.java",
))]
pub struct Writer {
    #[cfg_attr(any(), java_field(name = "writeBuffer", descriptor = "[C", access = "private"))]
    pub writeBuffer: Field<Vec<u16>>,
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/lang/Object;", access = "protected"))]
    pub lock: Field<Object>,
}

impl Writer {
    #[cfg_attr(any(), java_method(name = "nullWriter", descriptor = "()Ljava/io/Writer;", access = "public static"))]
    pub fn nullWriter() -> Result<Object> {
        Ok(Writer_1::new()?)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { writeBuffer: Field::new(Default::default()), lock: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.lock.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;)V"))]
    // java: <init>(Ljava/io/Writer;)V
    pub fn new__writer(writer: Object) -> Result<Self> {
        let this = Self { writeBuffer: Field::new(Default::default()), lock: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = writer.getClass()?;
        let mut clazz: Object = _t0;
        let _t1 = this.getClass()?;
        let _t2: Object = InternalLock::newLockOr(writer)?;
        this.lock.set(_t2);
        this.lock.set(writer);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "protected"))]
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(lock: Object) -> Result<Self> {
        let this = Self { writeBuffer: Field::new(Default::default()), lock: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom(String::from("athrow")));
        this.lock.set(lock);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, c: i32) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implWrite(c)?;
        locker.unlock()?;
        let mut local_4: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_4 = lock;
        /* TODO: monitorenter  */
        this.implWrite(c)?;
        /* TODO: monitorexit  */
        let mut local_5: bool = local_4;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private"))]
    // java: implWrite(I)V
    pub fn implWrite__i(&self, c: i32) -> Result<()> {
        let this = self;
        let mut _arr0: Vec<u16> = vec![0u16; 1024i32 as usize];
        this.writeBuffer.set(_arr0);
        /* TODO: i2c  */
        this.writeBuffer.get()[0i32 as usize] = c;
        this.write(this.writeBuffer.get(), 0i32, 1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([C)V", access = "public"))]
    // java: write([C)V
    pub fn write__arr_c(&self, cbuf: Vec<u16>) -> Result<()> {
        let this = self;
        this.write(cbuf, 0i32, (cbuf.len() as i32))?;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "write", descriptor = "([CII)V", access = "public abstract"))]
    pub fn write__arr_c_i_i(&self, arg0: Vec<u16>, arg1: i32, arg2: i32) -> Result<()> {
        todo!("abstract java/io/Writer.write")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: write(Ljava/lang/String;)V
    pub fn write__str(&self, str: String) -> Result<()> {
        let this = self;
        let _t0 = str.length()?;
        this.write(str, 0i32, _t0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public"))]
    // java: write(Ljava/lang/String;II)V
    pub fn write__str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implWrite(str, off, len)?;
        locker.unlock()?;
        let mut local_6: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_6 = lock;
        /* TODO: monitorenter  */
        this.implWrite(str, off, len)?;
        /* TODO: monitorexit  */
        let mut local_7: bool = local_6;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    // java: implWrite(Ljava/lang/String;II)V
    pub fn implWrite__str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        let this = self;
        let mut _arr0: Vec<u16> = vec![0u16; 1024i32 as usize];
        this.writeBuffer.set(_arr0);
        let mut cbuf: Vec<u16> = this.writeBuffer.get();
        let mut _arr1: Vec<u16> = vec![0u16; len as usize];
        cbuf = _arr1;
        str.getChars(off, (off).wrapping_add(len), cbuf, 0i32)?;
        this.write(cbuf, 0i32, len)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;)Ljava/io/Writer;
    pub fn append__seq(&self, csq: Object) -> Result<Object> {
        let this = self;
        this.write(String::from_owned(format!("{}", csq)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;II)Ljava/io/Writer;
    pub fn append__seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        csq = String::from("null");
        let _t0 = csq.subSequence(start, end)?;
        let _t1 = this.append(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/io/Writer;", access = "public"))]
    // java: append(C)Ljava/io/Writer;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        this.write(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "flush", descriptor = "()V", access = "public abstract"))]
    pub fn flush(&self) -> Result<()> {
        todo!("abstract java/io/Writer.flush")
    }

    #[cfg_attr(any(), java_native(name = "close", descriptor = "()V", access = "public abstract"))]
    pub fn close(&self) -> Result<()> {
        todo!("abstract java/io/Writer.close")
    }
}
