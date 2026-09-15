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
use crate::jdk::internal::misc::InternalLock;
use crate::jdk::internal::misc::VM;

#[java_rta_macros::java_class(
    binary_name       = "java/io/BufferedWriter",
    super_class       = "java/io/Writer",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "BufferedWriter.java",
    all_supertypes    = "java/io/BufferedWriter;java/io/Closeable;java/io/Flushable;java/io/Writer;java/lang/Appendable;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct BufferedWriter {
    pub _super: Writer,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/io/Writer;", access = "private", modifiers = "", is_static = false))]
    pub out: JField<Writer>,
    #[cfg_attr(any(), java_field(name = "cb", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
    pub cb: JField<Rc<RefCell<Vec<u16>>>>,
    #[cfg_attr(any(), java_field(name = "nChars", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub nChars: JField<i32>,
    #[cfg_attr(any(), java_field(name = "nextChar", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub nextChar: JField<i32>,
    #[cfg_attr(any(), java_field(name = "maxChars", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
    pub maxChars: JField<i32>,
}

impl BufferedWriter {
    pub fn as_writer(&self) -> &Writer { &self._super }
    pub fn into_writer(self) -> Writer { self._super }
}

impl From<BufferedWriter> for Writer {
    fn from(v: BufferedWriter) -> Writer { v._super }
}

impl BufferedWriter {
    #[cfg_attr(any(), java_field(name = "DEFAULT_INITIAL_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
    // static field: DEFAULT_INITIAL_BUFFER_SIZE:I
    pub fn DEFAULT_INITIAL_BUFFER_SIZE() -> i32 {
        512
    }

    #[cfg_attr(any(), java_field(name = "DEFAULT_MAX_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8192"))]
    // static field: DEFAULT_MAX_BUFFER_SIZE:I
    pub fn DEFAULT_MAX_BUFFER_SIZE() -> i32 {
        8192
    }

    #[cfg_attr(any(), java_method(name = "initialBufferSize", descriptor = "()I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn initialBufferSize() -> Result<i32> {
        panic!("stub: java/io/BufferedWriter.initialBufferSize:()I")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_writer_i_i(out: Writer, initialSize: i32, maxSize: i32) -> Result<Self> {
        panic!("stub: java/io/BufferedWriter.<init>:(Ljava/io/Writer;II)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_writer(out: Writer) -> Result<Self> {
        panic!("stub: java/io/BufferedWriter.<init>:(Ljava/io/Writer;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_writer_i(out: Writer, sz: i32) -> Result<Self> {
        panic!("stub: java/io/BufferedWriter.<init>:(Ljava/io/Writer;I)V")
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        if _is_jnull(&this.out.get()) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "growIfNeeded", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn growIfNeeded(&self, len: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.growIfNeeded:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "flushBuffer", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flushBuffer(&self) -> Result<()> {
        let this = self;
        let mut lock = this._super.lock.get();
        if (lock.is_instance_of("jdk/internal/misc/InternalLock")) {
            let mut locker = (lock).downcast::<InternalLock>();
            locker.lock()?;
            this.implFlushBuffer()?;
            locker.unlock()?;
            let mut local_3 = (panic!("stack underflow") as i32);
            locker.unlock()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let mut local_3: Object = lock;
        this.implFlushBuffer()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implFlushBuffer", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFlushBuffer(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        if (this.nextChar.get()==0) {
            return Ok(());
        }
        this.out.get().write_arr_c_i_i(Clone::clone(&this.cb.get()), 0i32, this.nextChar.get())?;
        this.nextChar.set(0i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_i(&self, c: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.write:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_i(&self, c: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.implWrite:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min(&self, a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/io/BufferedWriter.min:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.write:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([CII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_arr_c_i_i(&self, cbuf: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.implWrite:([CII)V")
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn write_str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.write:(Ljava/lang/String;II)V")
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implWrite_str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.implWrite:(Ljava/lang/String;II)V")
    }

    #[cfg_attr(any(), java_method(name = "newLine", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn newLine(&self) -> Result<()> {
        let this = self;
        let _t0: String = System::lineSeparator()?;
        this._super.write_str(Clone::clone(&_t0))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn flush(&self) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.flush:()V")
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implFlush(&self) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.implFlush:()V")
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn close(&self) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.close:()V")
    }

    #[cfg_attr(any(), java_method(name = "implClose", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn implClose(&self) -> Result<()> {
        panic!("stub: java/io/BufferedWriter.implClose:()V")
    }
}
