#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/io/OutputStream",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Closeable,java/io/Flushable",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "OutputStream.java",
    inner_classes     = "java/io/OutputStream$1:::0",
    all_supertypes    = "java/io/Closeable;java/io/Flushable;java/io/OutputStream;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct OutputStream;

impl OutputStream {
    #[java_rta_macros::java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new() -> Result<Self> {
        panic!("stub: java/io/OutputStream.<init>:()V")
    }

    #[java_rta_macros::java_method(name = "nullOutputStream", descriptor = "()Ljava/io/OutputStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn nullOutputStream() -> Result<OutputStream> {
        panic!("stub: java/io/OutputStream.nullOutputStream:()Ljava/io/OutputStream;")
    }

    #[java_rta_macros::java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn write_i(&self, arg0: i32) -> Result<()> {
        panic!("stub: java/io/OutputStream.write:(I)V")
    }

    #[java_rta_macros::java_method(name = "write", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn write_arr_b(&self, b: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/io/OutputStream.write:([B)V")
    }

    #[java_rta_macros::java_method(name = "write", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn write_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/OutputStream.write:([BII)V")
    }

    #[java_rta_macros::java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[java_rta_macros::java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn close(&self) -> Result<()> {
        panic!("stub: java/io/OutputStream.close:()V")
    }
}
