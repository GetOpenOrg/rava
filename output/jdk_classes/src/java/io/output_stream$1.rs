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
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self { closed: Field::new(false) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public"))]
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        this.ensureOpen()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.closed.set(1i32);
        Ok(())
    }
}
