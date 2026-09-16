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
    binary_name       = "sun/nio/cs/StreamEncoder",
    super_class       = "java/io/Writer",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "StreamEncoder.java",
    all_supertypes    = "java/io/Closeable;java/io/Flushable;java/io/Writer;java/lang/Appendable;java/lang/Object;sun/nio/cs/StreamEncoder",
)]
#[derive(Clone, Default, PartialEq)]
pub struct StreamEncoder {
    pub _super: Writer,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private", modifiers = "volatile", is_static = false))]
    pub closed: JField<bool>,
    #[cfg_attr(any(), java_field(name = "cs", descriptor = "Ljava/nio/charset/Charset;", access = "private", modifiers = "final", is_static = false))]
    pub cs: JField<Object>,
    #[cfg_attr(any(), java_field(name = "encoder", descriptor = "Ljava/nio/charset/CharsetEncoder;", access = "private", modifiers = "final", is_static = false))]
    pub encoder: JField<Object>,
    #[cfg_attr(any(), java_field(name = "bb", descriptor = "Ljava/nio/ByteBuffer;", access = "private", modifiers = "", is_static = false))]
    pub bb: JField<Object>,
    #[cfg_attr(any(), java_field(name = "maxBufferCapacity", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
    pub maxBufferCapacity: JField<i32>,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/io/OutputStream;", access = "private", modifiers = "final", is_static = false))]
    pub out: JField<OutputStream>,
    #[cfg_attr(any(), java_field(name = "ch", descriptor = "Ljava/nio/channels/WritableByteChannel;", access = "private", modifiers = "final", is_static = false))]
    pub ch: JField<Object>,
    #[cfg_attr(any(), java_field(name = "haveLeftoverChar", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
    pub haveLeftoverChar: JField<bool>,
    #[cfg_attr(any(), java_field(name = "leftoverChar", descriptor = "C", access = "private", modifiers = "", is_static = false))]
    pub leftoverChar: JField<u16>,
    #[cfg_attr(any(), java_field(name = "lcb", descriptor = "Ljava/nio/CharBuffer;", access = "private", modifiers = "", is_static = false))]
    pub lcb: JField<Object>,
}

impl StreamEncoder {
    pub fn as_writer(&self) -> &Writer { &self._super }
    pub fn into_writer(self) -> Writer { self._super }
}

impl From<StreamEncoder> for Writer {
    fn from(v: StreamEncoder) -> Writer { v._super }
}

impl StreamEncoder {
    #[cfg_attr(any(), java_field(name = "INITIAL_BYTE_BUFFER_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
    // static field: INITIAL_BYTE_BUFFER_CAPACITY:I
    pub fn INITIAL_BYTE_BUFFER_CAPACITY() -> i32 {
        512
    }

    #[cfg_attr(any(), java_field(name = "MAX_BYTE_BUFFER_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8192"))]
    // static field: MAX_BYTE_BUFFER_CAPACITY:I
    pub fn MAX_BYTE_BUFFER_CAPACITY() -> i32 {
        8192
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn ensureOpen(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.ensureOpen:()V")
    }

    #[cfg_attr(any(), java_method(name = "forOutputStreamWriter", descriptor = "(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/lang/String;)Lsun/nio/cs/StreamEncoder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn forOutputStreamWriter_output_obj_str(out: OutputStream, lock: Object, charsetName: String) -> Result<StreamEncoder> {
        panic!("stub: sun/nio/cs/StreamEncoder.forOutputStreamWriter:(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/lang/String;)Lsun/nio/cs/StreamEncoder;")
    }

    #[cfg_attr(any(), java_method(name = "forOutputStreamWriter", descriptor = "(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)Lsun/nio/cs/StreamEncoder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn forOutputStreamWriter_output_obj_charse(out: OutputStream, lock: Object, cs: Object) -> Result<StreamEncoder> {
        panic!("stub: sun/nio/cs/StreamEncoder.forOutputStreamWriter:(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)Lsun/nio/cs/StreamEncoder;")
    }

    #[cfg_attr(any(), java_method(name = "forOutputStreamWriter", descriptor = "(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)Lsun/nio/cs/StreamEncoder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn forOutputStreamWriter_output_obj_charse_1(out: OutputStream, lock: Object, enc: Object) -> Result<StreamEncoder> {
        panic!("stub: sun/nio/cs/StreamEncoder.forOutputStreamWriter:(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)Lsun/nio/cs/StreamEncoder;")
    }

    #[cfg_attr(any(), java_method(name = "forEncoder", descriptor = "(Ljava/nio/channels/WritableByteChannel;Ljava/nio/charset/CharsetEncoder;I)Lsun/nio/cs/StreamEncoder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn forEncoder(ch: Object, enc: Object, minBufferCap: i32) -> Result<StreamEncoder> {
        panic!("stub: sun/nio/cs/StreamEncoder.forEncoder:(Ljava/nio/channels/WritableByteChannel;Ljava/nio/charset/CharsetEncoder;I)Lsun/nio/cs/StreamEncoder;")
    }

    #[cfg_attr(any(), java_method(name = "getEncoding", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getEncoding(&self) -> Result<String> {
        panic!("stub: sun/nio/cs/StreamEncoder.getEncoding:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "flushBuffer", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flushBuffer(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.flushBuffer:()V")
    }

    #[cfg_attr(any(), java_method(name = "lockedFlushBuffer", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn lockedFlushBuffer(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.lockedFlushBuffer:()V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_i(&self, c: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.write:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.write:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "lockedWrite", descriptor = "([CII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn lockedWrite_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.lockedWrite:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.write:(Ljava/lang/String;II)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/nio/CharBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_charbu(&self, cb: Object) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.write:(Ljava/nio/CharBuffer;)V")
    }

    #[cfg_attr(any(), java_method(name = "lockedWrite", descriptor = "(Ljava/nio/CharBuffer;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn lockedWrite_charbu(&self, cb: Object) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.lockedWrite:(Ljava/nio/CharBuffer;)V")
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flush(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.flush:()V")
    }

    #[cfg_attr(any(), java_method(name = "lockedFlush", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn lockedFlush(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.lockedFlush:()V")
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn close(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.close:()V")
    }

    #[cfg_attr(any(), java_method(name = "lockedClose", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn lockedClose(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.lockedClose:()V")
    }

    #[cfg_attr(any(), java_method(name = "isOpen", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isOpen(&self) -> Result<bool> {
        panic!("stub: sun/nio/cs/StreamEncoder.isOpen:()Z")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_output_obj_charse(out: OutputStream, lock: Object, cs: Object) -> Result<Self> {
        panic!("stub: sun/nio/cs/StreamEncoder.<init>:(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_output_obj_charse_1(out: OutputStream, lock: Object, enc: Object) -> Result<Self> {
        panic!("stub: sun/nio/cs/StreamEncoder.<init>:(Ljava/io/OutputStream;Ljava/lang/Object;Ljava/nio/charset/CharsetEncoder;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/channels/WritableByteChannel;Ljava/nio/charset/CharsetEncoder;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_writab_charse_i(ch: Object, enc: Object, mbc: i32) -> Result<Self> {
        panic!("stub: sun/nio/cs/StreamEncoder.<init>:(Ljava/nio/channels/WritableByteChannel;Ljava/nio/charset/CharsetEncoder;I)V")
    }

    #[cfg_attr(any(), java_method(name = "writeBytes", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn writeBytes(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.writeBytes:()V")
    }

    #[cfg_attr(any(), java_method(name = "flushLeftoverChar", descriptor = "(Ljava/nio/CharBuffer;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flushLeftoverChar(&self, cb: Object, endOfInput: bool) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.flushLeftoverChar:(Ljava/nio/CharBuffer;Z)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([CII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.implWrite:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/nio/CharBuffer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_charbu(&self, cb: Object) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.implWrite:(Ljava/nio/CharBuffer;)V")
    }

    #[cfg_attr(any(), java_method(name = "growByteBufferIfNeeded", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn growByteBufferIfNeeded(&self, len: i32) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.growByteBufferIfNeeded:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "implFlushBuffer", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFlushBuffer(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.implFlushBuffer:()V")
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFlush(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.implFlush:()V")
    }

    #[cfg_attr(any(), java_method(name = "implClose", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implClose(&self) -> Result<()> {
        panic!("stub: sun/nio/cs/StreamEncoder.implClose:()V")
    }

    #[cfg_attr(any(), java_method(name = "encodingName", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn encodingName(&self) -> Result<String> {
        panic!("stub: sun/nio/cs/StreamEncoder.encodingName:()Ljava/lang/String;")
    }
}
