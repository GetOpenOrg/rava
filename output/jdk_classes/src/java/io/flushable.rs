#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/Flushable",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Flushable.java",
))]
pub struct Flushable;

impl Flushable {
    #[cfg_attr(any(), java_native(name = "flush", descriptor = "()V", access = "public abstract"))]
    pub fn flush(&self) -> Result<()> {
        todo!("abstract java/io/Flushable.flush")
    }
}
