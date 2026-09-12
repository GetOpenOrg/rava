#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/Closeable",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/AutoCloseable",
    access      = "public abstract",
    source      = "Closeable.java",
))]
pub struct Closeable;

impl Closeable {
    // java: close()V
    pub fn close(&self) -> Result<()> {
        todo!("abstract java/io/Closeable.close")
    }
}
