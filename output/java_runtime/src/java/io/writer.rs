#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::InternalLock;

#[java_rta_macros::java_class(
    binary_name       = "java/io/Writer",
    super_class       = "java/lang/Object",
    interfaces        = "java/lang/Appendable,java/io/Closeable,java/io/Flushable",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Writer.java",
    inner_classes     = "java/io/Writer$1:::0",
    all_supertypes    = "java/io/Closeable;java/io/Flushable;java/io/Writer;java/lang/Appendable;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Writer {
    #[cfg_attr(any(), java_field(name = "writeBuffer", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
    pub writeBuffer: JField<Rc<RefCell<Vec<u16>>>>,
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/lang/Object;", access = "protected", modifiers = "", is_static = false))]
    pub lock: JField<Object>,
}

impl Writer {
    #[cfg_attr(any(), java_field(name = "WRITE_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1024"))]
    // static field: WRITE_BUFFER_SIZE:I
    pub fn WRITE_BUFFER_SIZE() -> i32 {
        1024
    }

    #[cfg_attr(any(), java_method(name = "nullWriter", descriptor = "()Ljava/io/Writer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nullWriter() -> Result<Writer> {
        panic!("stub: java/io/Writer.nullWriter:()Ljava/io/Writer;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/io/Writer.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_writer(writer: Writer) -> Result<Self> {
        panic!("stub: java/io/Writer.<init>:(Ljava/io/Writer;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_obj(lock: Object) -> Result<Self> {
        panic!("stub: java/io/Writer.<init>:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_i(&self, c: i32) -> Result<()> {
        panic!("stub: java/io/Writer.write:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_i(&self, c: i32) -> Result<()> {
        panic!("stub: java/io/Writer.implWrite:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_c(&self, cbuf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/Writer.write:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([CII)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_c_i_i(&self, arg0: Rc<RefCell<Vec<u16>>>, arg1: i32, arg2: i32) -> Result<()> {
        panic!("stub: java/io/Writer.write:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_str(&self, str: String) -> Result<()> {
        panic!("stub: java/io/Writer.write:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/Writer.write:(Ljava/lang/String;II)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/Writer.implWrite:(Ljava/lang/String;II)V")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn append_seq(&self, csq: Object) -> Result<Writer> {
        panic!("stub: java/io/Writer.append:(Ljava/lang/CharSequence;)Ljava/io/Writer;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn append_seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Writer> {
        panic!("stub: java/io/Writer.append:(Ljava/lang/CharSequence;II)Ljava/io/Writer;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn append_c(&self, c: u16) -> Result<Writer> {
        panic!("stub: java/io/Writer.append:(C)Ljava/io/Writer;")
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flush(&self) -> Result<()> {
        panic!("stub: java/io/Writer.flush:()V")
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn close(&self) -> Result<()> {
        panic!("stub: java/io/Writer.close:()V")
    }
}
