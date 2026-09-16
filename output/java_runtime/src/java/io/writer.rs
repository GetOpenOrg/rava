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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/Writer"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Appendable,java/io/Closeable,java/io/Flushable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Writer.java"]
    #[inner_classes     = "java/io/Writer$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Closeable;java/io/Flushable;java/io/Writer;java/lang/Appendable;java/lang/Object"]

    pub struct Writer {
        #[cfg_attr(any(), java_field(name = "writeBuffer", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub writeBuffer: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljava/lang/Object;", access = "protected", modifiers = "", is_static = false))]
        pub lock: Object,
    }

    impl Writer {
        #[cfg_attr(any(), java_field(name = "WRITE_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1024"))]
        // static field: WRITE_BUFFER_SIZE:I
        pub fn WRITE_BUFFER_SIZE() -> i32 {
            1024
        }

        #[java_method(name = "nullWriter", descriptor = "()Ljava/io/Writer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nullWriter() -> Result<Writer> {
            panic!("stub: java/io/Writer.nullWriter:()Ljava/io/Writer;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/io/Writer.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/Writer;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_writer(writer: Writer) -> Result<Self> {
            panic!("stub: java/io/Writer.<init>:(Ljava/io/Writer;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_obj(lock: Object) -> Result<Self> {
            panic!("stub: java/io/Writer.<init>:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "write", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: write(I)V
        pub fn write_i(&self, mut c: i32) -> Result<()> {
            let this = self;
            let mut lock = this.__get_lock();
            if (lock.is_instance_of("jdk/internal/misc/InternalLock")) {
                let mut locker = (lock).downcast::<InternalLock>();
                locker.lock()?;
                this.implWrite_i(c)?;
                locker.unlock()?;
                let mut local_4 = (panic!("stack underflow") as i32);
                locker.unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut local_4: Object = lock;
            this.implWrite_i(c)?;
            Ok(())
        }

        #[java_method(name = "implWrite", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: implWrite(I)V
        pub fn implWrite_i(&self, mut c: i32) -> Result<()> {
            let this = self;
            if _is_jnull(&this.__get_writeBuffer()) {
                let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 1024i32 as usize]));
                this.__set_writeBuffer(Clone::clone(&_arr0));
            }
            this.__get_writeBuffer().borrow_mut()[0i32 as usize] = (((c) as u16 as i32)) as u16;
            this.write_arr_c_i_i(Clone::clone(&this.__get_writeBuffer()), 0i32, 1i32)?;
            Ok(())
        }

        #[java_method(name = "write", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_arr_c(&self, cbuf: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/io/Writer.write:([C)V")
        }

        #[java_method(name = "write", descriptor = "([CII)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write_arr_c_i_i(&self, arg0: Rc<RefCell<Vec<u16>>>, arg1: i32, arg2: i32) -> Result<()> {
            panic!("stub: java/io/Writer.write:([CII)V")
        }

        #[java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: write(Ljava/lang/String;)V
        pub fn write_str(&self, mut str: String) -> Result<()> {
            let this = self;
            let _t0 = str.length()?;
            this.write_str_i_i(Clone::clone(&str), 0i32, _t0)?;
            Ok(())
        }

        #[java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: write(Ljava/lang/String;II)V
        pub fn write_str_i_i(&self, mut str: String, mut off: i32, mut len: i32) -> Result<()> {
            let this = self;
            let mut lock = this.__get_lock();
            if (lock.is_instance_of("jdk/internal/misc/InternalLock")) {
                let mut locker = (lock).downcast::<InternalLock>();
                locker.lock()?;
                this.implWrite_str_i_i(Clone::clone(&str), off, len)?;
                locker.unlock()?;
                let mut local_6 = (panic!("stack underflow") as i32);
                locker.unlock()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut local_6: Object = lock;
            this.implWrite_str_i_i(Clone::clone(&str), off, len)?;
            Ok(())
        }

        #[java_method(name = "implWrite", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: implWrite(Ljava/lang/String;II)V
        pub fn implWrite_str_i_i(&self, mut str: String, mut off: i32, mut len: i32) -> Result<()> {
            let this = self;
        let mut cbuf = Default::default();
            if len <= 1024i32 {
                if _is_jnull(&this.__get_writeBuffer()) {
                    let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 1024i32 as usize]));
                    this.__set_writeBuffer(Clone::clone(&_arr0));
                }
                cbuf = this.__get_writeBuffer();
            } else {
                let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; len as usize]));
                cbuf = _arr0;
            }
            str.getChars(off, (off).wrapping_add(len), Clone::clone(&cbuf), 0i32)?;
            this.write_arr_c_i_i(Clone::clone(&cbuf), 0i32, len)?;
            Ok(())
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: append(Ljava/lang/CharSequence;)Ljava/io/Writer;
        pub fn append_seq(&self, mut csq: Object) -> Result<Writer> {
            let this = self;
            this.write_str(Clone::clone(&String::from_owned(format!("{}", csq))))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: append(Ljava/lang/CharSequence;II)Ljava/io/Writer;
        pub fn append_seq_i_i(&self, mut csq: Object, mut start: i32, mut end: i32) -> Result<Writer> {
            let this = self;
            if _is_jnull(&csq) {
                let mut csq: String = String::from("null");
            }
            let _t0 = csq.subSequence(start, end)?;
            let _t1 = this.append_seq(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "append", descriptor = "(C)Ljava/io/Writer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: append(C)Ljava/io/Writer;
        pub fn append_c(&self, mut c: u16) -> Result<Writer> {
            let this = self;
            this.write_i((c as i32))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn flush(&self) -> Result<()> {
            panic!("stub: java/io/Writer.flush:()V")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/io/Writer.close:()V")
        }
    }
}
