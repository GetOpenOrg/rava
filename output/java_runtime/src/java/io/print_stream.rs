#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::InternalLock;

#[java_rta_macros::java_class(
    binary_name       = "java/io/PrintStream",
    super_class       = "java/io/FilterOutputStream",
    interfaces        = "java/lang/Appendable,java/io/Closeable",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "PrintStream.java",
    inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/io/PrintStream$1:::0",
    all_supertypes    = "java/io/Closeable;java/io/FilterOutputStream;java/io/Flushable;java/io/OutputStream;java/io/PrintStream;java/lang/Appendable;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct PrintStream {
    pub _super: FilterOutputStream,
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private", modifiers = "final", is_static = false))]
    pub lock: JField<InternalLock>,
    #[cfg_attr(any(), java_field(name = "autoFlush", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
    pub autoFlush: JField<bool>,
    #[cfg_attr(any(), java_field(name = "trouble", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
    pub trouble: JField<bool>,
    #[cfg_attr(any(), java_field(name = "formatter", descriptor = "Ljava/util/Formatter;", access = "private", modifiers = "", is_static = false))]
    pub formatter: JField<Object>,
    #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private", modifiers = "final", is_static = false))]
    pub charset: JField<Object>,
    #[cfg_attr(any(), java_field(name = "textOut", descriptor = "Ljava/io/BufferedWriter;", access = "private", modifiers = "", is_static = false))]
    pub textOut: JField<BufferedWriter>,
    #[cfg_attr(any(), java_field(name = "charOut", descriptor = "Ljava/io/OutputStreamWriter;", access = "private", modifiers = "", is_static = false))]
    pub charOut: JField<OutputStreamWriter>,
    #[cfg_attr(any(), java_field(name = "closing", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
    pub closing: JField<bool>,
}

impl PrintStream {
    pub fn as_filter_output_stream(&self) -> &FilterOutputStream { &self._super }
    pub fn into_filter_output_stream(self) -> FilterOutputStream { self._super }
    pub fn as_output_stream(&self) -> &OutputStream { &self._super._super }
    pub fn into_output_stream(self) -> OutputStream { self._super._super }
}

impl From<PrintStream> for FilterOutputStream {
    fn from(v: PrintStream) -> FilterOutputStream { v._super }
}

impl From<PrintStream> for OutputStream {
    fn from(v: PrintStream) -> OutputStream { v._super._super }
}

impl PrintStream {
    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/lang/String;)TT;"))]
    pub fn requireNonNull(obj: Object, message: String) -> Result<Object> {
        panic!("stub: java/io/PrintStream.requireNonNull:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn toCharset(csn: String) -> Result<Object> {
        panic!("stub: java/io/PrintStream.toCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/io/OutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_z_output(autoFlush: bool, out: OutputStream) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_z_charse_output(autoFlush: bool, charset: Object, out: OutputStream) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_output(out: OutputStream) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_output_z(out: OutputStream, autoFlush: bool) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;Z)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException"))]
    pub fn new_output_z_str(out: OutputStream, autoFlush: bool, encoding: String) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_output_z_charse(out: OutputStream, autoFlush: bool, charset: Object) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException"))]
    pub fn new_str(fileName: String) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException"))]
    pub fn new_str_str(fileName: String, csn: String) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn new_str_charse(fileName: String, charset: Object) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/lang/String;Ljava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException"))]
    pub fn new_file(file: Object) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException"))]
    pub fn new_file_str(file: Object, csn: String) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn new_file_charse(file: Object, charset: Object) -> Result<Self> {
        panic!("stub: java/io/PrintStream.<init>:(Ljava/io/File;Ljava/nio/charset/Charset;)V")
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        if _is_jnull(&this._super.out.get()) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn flush(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.flush:()V")
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn implFlush(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.implFlush:()V")
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn close(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.close:()V")
    }

    #[cfg_attr(any(), java_method(name = "implClose", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn implClose(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.implClose:()V")
    }

    #[cfg_attr(any(), java_method(name = "checkError", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkError(&self) -> Result<bool> {
        panic!("stub: java/io/PrintStream.checkError:()Z")
    }

    #[cfg_attr(any(), java_method(name = "setError", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn setError(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.setError:()V")
    }

    #[cfg_attr(any(), java_method(name = "clearError", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clearError(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.clearError:()V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn write_i(&self, b: i32) -> Result<()> {
        panic!("stub: java/io/PrintStream.write:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_i(&self, b: i32) -> Result<()> {
        panic!("stub: java/io/PrintStream.implWrite:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn write_arr_b_i_i(&self, buf: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/PrintStream.write:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([BII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_arr_b_i_i(&self, buf: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/PrintStream.implWrite:([BII)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_b(&self, buf: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.write:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "writeBytes", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn writeBytes(&self, buf: Rc<RefCell<Vec<i8>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.writeBytes:([B)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn write_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.write:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.implWrite:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "writeln", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn writeln_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.writeln:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWriteln_arr_c(&self, buf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.implWriteln:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: write(Ljava/lang/String;)V
    pub fn write_str(&self, mut s: String) -> Result<()> {
        let this = self;
        if !_is_jnull(&this.lock.get()) {
            this.lock.get().lock()?;
            this.implWrite_str(Clone::clone(&s))?;
            this.lock.get().unlock()?;
            let mut x = (panic!("stack underflow") as i32);
            this.lock.get().unlock()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut x = this.clone();
        this.implWrite_str(Clone::clone(&s))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    // java: implWrite(Ljava/lang/String;)V
    pub fn implWrite_str(&self, mut s: String) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get()._super.write_str(Clone::clone(&s))?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        let _t0 = s.indexOf_i(10i32)?;
        if (_t0>=0) {
            this._super.out.get().flush()?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeln", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: writeln(Ljava/lang/String;)V
    pub fn writeln_str(&self, mut s: String) -> Result<()> {
        let this = self;
        if !_is_jnull(&this.lock.get()) {
            this.lock.get().lock()?;
            this.implWriteln_str(Clone::clone(&s))?;
            this.lock.get().unlock()?;
            let mut x = (panic!("stack underflow") as i32);
            this.lock.get().unlock()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut x = this.clone();
        this.implWriteln_str(Clone::clone(&s))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    // java: implWriteln(Ljava/lang/String;)V
    pub fn implWriteln_str(&self, mut s: String) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get()._super.write_str(Clone::clone(&s))?;
        this.textOut.get().newLine()?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        if this.autoFlush.get() {
            this._super.out.get().flush()?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newLine", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn newLine(&self) -> Result<()> {
        let this = self;
        if !_is_jnull(&this.lock.get()) {
            this.lock.get().lock()?;
            this.implNewLine()?;
            this.lock.get().unlock()?;
            let mut x = (panic!("stack underflow") as i32);
            this.lock.get().unlock()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut x = this.clone();
        this.implNewLine()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implNewLine", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implNewLine(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().newLine()?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        if this.autoFlush.get() {
            this._super.out.get().flush()?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_z(&self, b: bool) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:(Z)V")
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_c(&self, c: u16) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:(C)V")
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: print(I)V
    pub fn print_i(&self, mut i: i32) -> Result<()> {
        let this = self;
        this.write_str(Clone::clone(&String::from_owned(format!("{}", i))))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: print(J)V
    pub fn print_l(&self, mut l: i64) -> Result<()> {
        let this = self;
        this.write_str(Clone::clone(&String::from_owned(format!("{}", l))))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_f(&self, f: f32) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:(F)V")
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_d(&self, d: f64) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:(D)V")
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_arr_c(&self, s: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: print(Ljava/lang/String;)V
    pub fn print_str(&self, mut s: String) -> Result<()> {
        let this = self;
        this.write_str(Clone::clone(&String::from_owned(format!("{}", s))))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn print_obj(&self, obj: Object) -> Result<()> {
        panic!("stub: java/io/PrintStream.print:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println(&self) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:()V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println_z(&self, x: bool) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:(Z)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println_c(&self, x: u16) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:(C)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println_f(&self, x: f32) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:(F)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println_d(&self, x: f64) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:(D)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn println_arr_c(&self, x: Rc<RefCell<Vec<u16>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.println:([C)V")
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "printf", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn printf_str_arr_obj(&self, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "printf", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn printf_locale_str_arr_obj(&self, l: Object, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.printf:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn format_str_arr_obj(&self, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.format:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFormat_str_arr_obj(&self, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.implFormat:(Ljava/lang/String;[Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn format_locale_str_arr_obj(&self, l: Object, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.format:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFormat_locale_str_arr_obj(&self, l: Object, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("stub: java/io/PrintStream.implFormat:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_seq(&self, csq: Object) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.append:(Ljava/lang/CharSequence;)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.append:(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/io/PrintStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_c(&self, c: u16) -> Result<PrintStream> {
        panic!("stub: java/io/PrintStream.append:(C)Ljava/io/PrintStream;")
    }

    #[cfg_attr(any(), java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn charset(&self) -> Result<Object> {
        panic!("stub: java/io/PrintStream.charset:()Ljava/nio/charset/Charset;")
    }
}
