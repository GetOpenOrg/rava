#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::InternalLock;

impl From<OutputStreamWriter> for Writer {
    fn from(v: OutputStreamWriter) -> Writer { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/OutputStreamWriter"]
    #[super_class       = "java/io/Writer"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "OutputStreamWriter.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Writer"]
    #[superclass_fields(writeBuffer: Rc<RefCell<Vec<u16>>>, lock: Object)]
    #[all_supertypes    = "java/io/Closeable;java/io/Flushable;java/io/OutputStreamWriter;java/io/Writer;java/lang/Appendable;java/lang/Object"]

    pub struct OutputStreamWriter {
        #[cfg_attr(any(), java_field(name = "se", descriptor = "Lsun/nio/cs/StreamEncoder;", access = "private", modifiers = "final", is_static = false))]
        pub se: StreamEncoder,
    }

    impl OutputStreamWriter {
        #[java_method(name = "lockFor", descriptor = "(Ljava/io/OutputStreamWriter;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lockFor(writer: OutputStreamWriter) -> Result<Object> {
            panic!("stub: java/io/OutputStreamWriter.lockFor:(Ljava/io/OutputStreamWriter;)Ljava/lang/Object;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_output_str(out: OutputStream, charsetName: String) -> Result<Self> {
            panic!("stub: java/io/OutputStreamWriter.<init>:(Ljava/io/OutputStream;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output(out: OutputStream) -> Result<Self> {
            panic!("stub: java/io/OutputStreamWriter.<init>:(Ljava/io/OutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output_charse(out: OutputStream, cs: Charset) -> Result<Self> {
            panic!("stub: java/io/OutputStreamWriter.<init>:(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output_charse_1(out: OutputStream, enc: CharsetEncoder) -> Result<Self> {
            panic!("stub: java/io/OutputStreamWriter.<init>:(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V")
        }

        #[java_method(name = "getEncoding", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEncoding(&self) -> Result<String> {
            panic!("stub: java/io/OutputStreamWriter.getEncoding:()Ljava/lang/String;")
        }

        #[java_method(name = "flushBuffer", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn flushBuffer(&self) -> Result<()> {
            let this = self;
            this.__get_se().flushBuffer()?;
            Ok(())
        }

        #[java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_i(&self, c: i32) -> Result<()> {
            panic!("stub: java/io/OutputStreamWriter.write:(I)V")
        }

        #[java_method(name = "write", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/io/OutputStreamWriter.write:([CII)V")
        }

        #[java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/io/OutputStreamWriter.write:(Ljava/lang/String;II)V")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn append_seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Writer> {
            panic!("stub: java/io/OutputStreamWriter.append:(Ljava/lang/CharSequence;II)Ljava/io/Writer;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn append_seq(&self, csq: Object) -> Result<Writer> {
            panic!("stub: java/io/OutputStreamWriter.append:(Ljava/lang/CharSequence;)Ljava/io/Writer;")
        }

        #[java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn flush(&self) -> Result<()> {
            panic!("stub: java/io/OutputStreamWriter.flush:()V")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/io/OutputStreamWriter.close:()V")
        }
    }
}
