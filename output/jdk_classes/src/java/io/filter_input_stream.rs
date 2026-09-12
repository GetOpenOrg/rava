#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FilterInputStream",
    super_class = "java/io/InputStream",
    interfaces  = "",
    access      = "public",
    source      = "FilterInputStream.java",
))]
pub struct FilterInputStream {
    #[cfg_attr(any(), java_field(name = "in", descriptor = "Ljava/io/InputStream;", access = "protected"))]
    pub in_: Field<Object>,
}

impl FilterInputStream {
    // java: <init>(Ljava/io/InputStream;)V
    pub fn new(in_: Object) -> Result<Self> {
        let this = Self { in_: Field::new(Default::default()) };
        /* invokespecial Method java/io/InputStream.<init>:()V */
        this.in_.set(in_);
        Ok(this)
    }

    // java: read()I
    // java: read()I
    pub fn read(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.in_.get().read()?;
        Ok(_t0)
    }

    // java: read([B)I
    // java: read([B)I
    pub fn read__arr_b(&self, b: Vec<i8>) -> Result<i32> {
        let this = self;
        let _t0 = this.read(b, 0i32, (b.len() as i32))?;
        Ok(_t0)
    }

    // java: read([BII)I
    // java: read([BII)I
    pub fn read__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.in_.get().read(b, off, len)?;
        Ok(_t0)
    }

    // java: skip(J)J
    pub fn skip(&self, n: i64) -> Result<i64> {
        let this = self;
        let _t0 = this.in_.get().skip(n)?;
        Ok(_t0)
    }

    // java: available()I
    pub fn available(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.in_.get().available()?;
        Ok(_t0)
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.in_.get().close()?;
        Ok(())
    }

    // java: mark(I)V
    pub fn mark(&self, readlimit: i32) -> Result<()> {
        let this = self;
        this.in_.get().mark(readlimit)?;
        Ok(())
    }

    // java: reset()V
    pub fn reset(&self) -> Result<()> {
        let this = self;
        this.in_.get().reset()?;
        Ok(())
    }

    // java: markSupported()Z
    pub fn markSupported(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.in_.get().markSupported()?;
        Ok(_t0)
    }
}
