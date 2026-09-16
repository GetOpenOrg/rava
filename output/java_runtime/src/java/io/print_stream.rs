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

impl From<PrintStream> for FilterOutputStream {
    fn from(v: PrintStream) -> FilterOutputStream { v.__into_super() }
}

impl From<PrintStream> for OutputStream {
    fn from(v: PrintStream) -> OutputStream { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/PrintStream"]
    #[super_class       = "java/io/FilterOutputStream"]
    #[interfaces        = "java/lang/Appendable,java/io/Closeable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "PrintStream.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/io/PrintStream$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "FilterOutputStream"]
    #[superclass_fields(out: OutputStream, closed: bool, closeLock: Object)]
    #[all_supertypes    = "java/io/Closeable;java/io/FilterOutputStream;java/io/Flushable;java/io/OutputStream;java/io/PrintStream;java/lang/Appendable;java/lang/Object"]

    pub struct PrintStream {
        #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private", modifiers = "final", is_static = false))]
        pub lock: InternalLock,
        #[cfg_attr(any(), java_field(name = "autoFlush", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub autoFlush: bool,
        #[cfg_attr(any(), java_field(name = "trouble", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub trouble: bool,
        #[cfg_attr(any(), java_field(name = "formatter", descriptor = "Ljava/util/Formatter;", access = "private", modifiers = "", is_static = false))]
        pub formatter: Formatter,
        #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private", modifiers = "final", is_static = false))]
        pub charset: Charset,
        #[cfg_attr(any(), java_field(name = "textOut", descriptor = "Ljava/io/BufferedWriter;", access = "private", modifiers = "", is_static = false))]
        pub textOut: BufferedWriter,
        #[cfg_attr(any(), java_field(name = "charOut", descriptor = "Ljava/io/OutputStreamWriter;", access = "private", modifiers = "", is_static = false))]
        pub charOut: OutputStreamWriter,
        #[cfg_attr(any(), java_field(name = "closing", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub closing: bool,
    }

    impl PrintStream {
        #[java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/lang/String;)TT;")]
        pub fn requireNonNull(obj: Object, message: String) -> Result<Object> {
            panic!("stub: java/io/PrintStream.requireNonNull:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;")
        }

        #[java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn toCharset(csn: String) -> Result<Charset> {
            panic!("stub: java/io/PrintStream.toCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "<init>", descriptor = "(ZLjava/io/OutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_z_output(autoFlush: bool, out: OutputStream) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_z_charse_output(autoFlush: bool, charset: Charset, out: OutputStream) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output(out: OutputStream) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output_z(out: OutputStream, autoFlush: bool) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;Z)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_output_z_str(out: OutputStream, autoFlush: bool, encoding: String) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output_z_charse(out: OutputStream, autoFlush: bool, charset: Charset) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException")]
        pub fn new_str(fileName: String) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_str_str(fileName: String, csn: String) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn new_str_charse(fileName: String, charset: Charset) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;Ljava/nio/charset/Charset;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException")]
        pub fn new_file(file: Object) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_file_str(file: Object, csn: String) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn new_file_charse(file: Object, charset: Charset) -> Result<Self> {
            panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;Ljava/nio/charset/Charset;)V")
        }

        #[java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn ensureOpen(&self) -> Result<()> {
            let this = self;
            if _is_jnull(&this.__get_out()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flush(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.flush:()V")
        }

        #[java_method(name = "implFlush", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implFlush(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.implFlush:()V")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.close:()V")
        }

        #[java_method(name = "implClose", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implClose(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.implClose:()V")
        }

        #[java_method(name = "checkError", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkError(&self) -> Result<bool> {
            panic!("stub: java/io/PrintStream.checkError:()Z")
        }

        #[java_method(name = "setError", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setError(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.setError:()V")
        }

        #[java_method(name = "clearError", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearError(&self) -> Result<()> {
            panic!("stub: java/io/PrintStream.clearError:()V")
        }

        #[java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn write_i(&self, b: i32) -> Result<()> {
            panic!("stub: java/io/PrintStream.write:(I)V")
        }

        #[java_method(name = "implWrite", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implWrite_i(&self, b: i32) -> Result<()> {
            panic!("stub: java/io/PrintStream.implWrite:(I)V")
        }

        #[java_method(name = "write", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn write_arr_b_i_i(&self, buf: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/io/PrintStream.write:([BII)V")
        }

        #[java_method(name = "implWrite", descriptor = "([BII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implWrite_arr_b_i_i(&self, buf: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/io/PrintStream.implWrite:([BII)V")
        }

        #[java_method(name = "write", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_arr_b(&self, buf: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.write:([B)V")
        }

        #[java_method(name = "writeBytes", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeBytes(&self, buf: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.writeBytes:([B)V")
        }

        #[java_method(name = "write", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn write_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.write:([C)V")
        }

        #[java_method(name = "implWrite", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implWrite_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.implWrite:([C)V")
        }

        #[java_method(name = "writeln", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeln_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.writeln:([C)V")
        }

        #[java_method(name = "implWriteln", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implWriteln_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.implWriteln:([C)V")
        }

        #[java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: write(Ljava/lang/String;)V
        pub fn write_str(&self, mut s: String) -> Result<()> {
            let this = self;
            if !_is_jnull(&this.__get_lock()) {
                this.__get_lock().lock()?;
                this.implWrite_str(Clone::clone(&s))?;
                this.__get_lock().unlock()?;
                let mut x = (panic!("stack underflow") as i32);
                this.__get_lock().unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut x = this.clone();
            this.implWrite_str(Clone::clone(&s))?;
            Ok(())
        }

        #[java_method(name = "implWrite", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: implWrite(Ljava/lang/String;)V
        pub fn implWrite_str(&self, mut s: String) -> Result<()> {
            let this = self;
            this.ensureOpen()?;
            this.__get_textOut().__super().write_str(Clone::clone(&s))?;
            this.__get_textOut().flushBuffer()?;
            this.__get_charOut().flushBuffer()?;
            let _t0 = s.indexOf_i(10i32)?;
            if (_t0>=0) {
                this.__get_out().flush()?;
            }
            Ok(())
        }

        #[java_method(name = "writeln", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: writeln(Ljava/lang/String;)V
        pub fn writeln_str(&self, mut s: String) -> Result<()> {
            let this = self;
            if !_is_jnull(&this.__get_lock()) {
                this.__get_lock().lock()?;
                this.implWriteln_str(Clone::clone(&s))?;
                this.__get_lock().unlock()?;
                let mut x = (panic!("stack underflow") as i32);
                this.__get_lock().unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut x = this.clone();
            this.implWriteln_str(Clone::clone(&s))?;
            Ok(())
        }

        #[java_method(name = "implWriteln", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: implWriteln(Ljava/lang/String;)V
        pub fn implWriteln_str(&self, mut s: String) -> Result<()> {
            let this = self;
            this.ensureOpen()?;
            this.__get_textOut().__super().write_str(Clone::clone(&s))?;
            this.__get_textOut().newLine()?;
            this.__get_textOut().flushBuffer()?;
            this.__get_charOut().flushBuffer()?;
            if this.__get_autoFlush() {
                this.__get_out().flush()?;
            }
            Ok(())
        }

        #[java_method(name = "newLine", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newLine(&self) -> Result<()> {
            let this = self;
            if !_is_jnull(&this.__get_lock()) {
                this.__get_lock().lock()?;
                this.implNewLine()?;
                this.__get_lock().unlock()?;
                let mut x = (panic!("stack underflow") as i32);
                this.__get_lock().unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut x = this.clone();
            this.implNewLine()?;
            Ok(())
        }

        #[java_method(name = "implNewLine", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implNewLine(&self) -> Result<()> {
            let this = self;
            this.ensureOpen()?;
            this.__get_textOut().newLine()?;
            this.__get_textOut().flushBuffer()?;
            this.__get_charOut().flushBuffer()?;
            if this.__get_autoFlush() {
                this.__get_out().flush()?;
            }
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(Z)V
        pub fn print_z(&self, mut b: bool) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", b))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(C)V
        pub fn print_c(&self, mut c: u16) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", c))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(I)V
        pub fn print_i(&self, mut i: i32) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", i))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(J)V
        pub fn print_l(&self, mut l: i64) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", l))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn print_f(&self, f: f32) -> Result<()> {
            panic!("stub: java/io/PrintStream.print:(F)V")
        }

        #[java_method(name = "print", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(D)V
        pub fn print_d(&self, mut d: f64) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", d))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn print_arr_c(&self, s: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.print:([C)V")
        }

        #[java_method(name = "print", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(Ljava/lang/String;)V
        pub fn print_str(&self, mut s: String) -> Result<()> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", s))))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn print_obj(&self, obj: Object) -> Result<()> {
            panic!("stub: java/io/PrintStream.print:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "println", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println()V
        pub fn println(&self) -> Result<()> {
            let this = self;
            this.newLine()?;
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(Z)V
        pub fn println_z(&self, mut x: bool) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_2 = this.clone();
                this.print_z(x)?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(C)V
        pub fn println_c(&self, mut x: u16) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_2 = this.clone();
                this.print_c(x)?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(I)V
        pub fn println_i(&self, mut x: i32) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_2 = this.clone();
                this.print_i(x)?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(J)V
        pub fn println_l(&self, mut x: i64) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_3 = this.clone();
                this.print_l(x)?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn println_f(&self, x: f32) -> Result<()> {
            panic!("stub: java/io/PrintStream.println:(F)V")
        }

        #[java_method(name = "println", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(D)V
        pub fn println_d(&self, mut x: f64) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_3 = this.clone();
                this.print_d(x)?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn println_arr_c(&self, x: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.println:([C)V")
        }

        #[java_method(name = "println", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(Ljava/lang/String;)V
        pub fn println_str(&self, mut x: String) -> Result<()> {
            let this = self;
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", x))))?;
            } else {
                let mut local_2 = this.clone();
                this.print_str(Clone::clone(&x))?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "println", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: println(Ljava/lang/Object;)V
        pub fn println_obj(&self, mut x: Object) -> Result<()> {
            let this = self;
            let mut s: String = String::from_owned(format!("{}", x));
            let _t0 = this.getClass()?;
            if _t0 == Object::default() {
                this.writeln_str(Clone::clone(&String::from_owned(format!("{}", s))))?;
            } else {
                let mut local_3 = this.clone();
                this.print_str(Clone::clone(&s))?;
                this.newLine()?;
            }
            Ok(())
        }

        #[java_method(name = "printf", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: printf(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
        pub fn printf_str_arr_obj(&self, mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
            let this = self;
            let _t0 = this.format_str_arr_obj(Clone::clone(&format), Clone::clone(&args))?;
            Ok(_t0)
        }

        #[java_method(name = "printf", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printf_locale_str_arr_obj(&self, l: Locale, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
            panic!("stub: java/io/PrintStream.printf:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
        pub fn format_str_arr_obj(&self, mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
            let this = self;
            if !_is_jnull(&this.__get_lock()) {
                this.__get_lock().lock()?;
                this.implFormat_str_arr_obj(Clone::clone(&format), Clone::clone(&args))?;
                this.__get_lock().unlock()?;
                let mut x = (panic!("stack underflow") as i32);
                this.__get_lock().unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut x = this.clone();
            this.implFormat_str_arr_obj(Clone::clone(&format), Clone::clone(&args))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "implFormat", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: implFormat(Ljava/lang/String;[Ljava/lang/Object;)V
        pub fn implFormat_str_arr_obj(&self, mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            let this = self;
            this.ensureOpen()?;
            let _t0 = this.__get_formatter().locale()?;
            let _t1: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            if Object::from_any(_t0.clone()) != Object::from_any(_t1.clone()) {
                this.__set_formatter(Formatter::new_append(Object::from_any(Clone::clone(self)))?);
            }
            let _t2: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            let _t3 = this.__get_formatter().format_locale_str_arr_obj(Clone::clone(&_t2), Clone::clone(&format), Clone::clone(&args))?;
            Ok(())
        }

        #[java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_locale_str_arr_obj(&self, l: Locale, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
            panic!("stub: java/io/PrintStream.format:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
        }

        #[java_method(name = "implFormat", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn implFormat_locale_str_arr_obj(&self, l: Locale, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/io/PrintStream.implFormat:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/CharSequence;)Ljava/io/PrintStream;
        pub fn append_seq(&self, mut csq: Object) -> Result<PrintStream> {
            let this = self;
            this.print_str(Clone::clone(&String::from_owned(format!("{}", csq))))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;
        pub fn append_seq_i_i(&self, mut csq: Object, mut start: i32, mut end: i32) -> Result<PrintStream> {
            let this = self;
            if _is_jnull(&csq) {
                let mut csq: String = String::from("null");
            }
            let _t0 = csq.subSequence(start, end)?;
            let _t1 = this.append_seq(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "append", descriptor = "(C)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(C)Ljava/io/PrintStream;
        pub fn append_c(&self, mut c: u16) -> Result<PrintStream> {
            let this = self;
            this.print_c(c)?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charset(&self) -> Result<Charset> {
            panic!("stub: java/io/PrintStream.charset:()Ljava/nio/charset/Charset;")
        }
    }
}
