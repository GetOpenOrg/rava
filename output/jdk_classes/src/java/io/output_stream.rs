#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/OutputStream",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Closeable,java/io/Flushable",
    access      = "public abstract",
    source      = "OutputStream.java",
))]
pub struct OutputStream;

impl OutputStream {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: nullOutputStream()Ljava/io/OutputStream;
    pub fn nullOutputStream() -> Result<Object> {
        Ok(OutputStream_1::new()?)
    }

    // java: write(I)V
    pub fn write__i(&self, arg0: i32) -> Result<()> {
        todo!("abstract java/io/OutputStream.write")
    }

    // java: write([B)V
    // java: write([B)V
    pub fn write__arr_b(&self, b: Vec<i8>) -> Result<()> {
        let this = self;
        this.write(b, 0i32, (b.len() as i32))?;
        Ok(())
    }

    // java: write([BII)V
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            this.write(b[(off).wrapping_add(i) as usize])?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: flush()V
    pub fn flush(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        Ok(())
    }
}
