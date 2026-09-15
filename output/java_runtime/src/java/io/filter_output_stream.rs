#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/io/FilterOutputStream",
    super_class       = "java/io/OutputStream",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "FilterOutputStream.java",
    all_supertypes    = "java/io/Closeable;java/io/FilterOutputStream;java/io/Flushable;java/io/OutputStream;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct FilterOutputStream {
    pub _super: OutputStream,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/io/OutputStream;", access = "protected", modifiers = "", is_static = false))]
    pub out: JField<OutputStream>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private", modifiers = "volatile", is_static = false))]
    pub closed: JField<bool>,
    #[cfg_attr(any(), java_field(name = "closeLock", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
    pub closeLock: JField<Object>,
}

impl FilterOutputStream {
    pub fn as_output_stream(&self) -> &OutputStream { &self._super }
    pub fn into_output_stream(self) -> OutputStream { self._super }
}

impl From<FilterOutputStream> for OutputStream {
    fn from(v: FilterOutputStream) -> OutputStream { v._super }
}

impl FilterOutputStream {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(out: OutputStream) -> Result<Self> {
        panic!("stub: java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_i(&self, b: i32) -> Result<()> {
        panic!("stub: java/io/FilterOutputStream.write:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_b(&self, b: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/io/FilterOutputStream.write:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/FilterOutputStream.write:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flush(&self) -> Result<()> {
        panic!("stub: java/io/FilterOutputStream.flush:()V")
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn close(&self) -> Result<()> {
        panic!("stub: java/io/FilterOutputStream.close:()V")
    }
}
