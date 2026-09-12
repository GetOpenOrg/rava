#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/OutputStream$1",
    super_class = "java/io/OutputStream",
    interfaces  = "",
    access      = "",
    source      = "OutputStream.java",
))]
pub struct OutputStream_1 {
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
}

impl OutputStream_1 {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { closed: Field::new(false) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        Ok(this)
    }

    // java: ensureOpen()V
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: write(I)V
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        Ok(())
    }

    // java: write([BII)V
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        this.ensureOpen()?;
        Ok(())
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.closed.set(1i32);
        Ok(())
    }
}
