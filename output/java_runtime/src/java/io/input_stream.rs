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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/InputStream"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Closeable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "InputStream.java"]
    #[inner_classes     = "java/io/InputStream$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Closeable;java/io/InputStream;java/lang/Object"]

    pub struct InputStream;

    impl InputStream {
        #[cfg_attr(any(), java_field(name = "MAX_SKIP_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2048"))]
        // static field: MAX_SKIP_BUFFER_SIZE:I
        pub fn MAX_SKIP_BUFFER_SIZE() -> i32 {
            2048
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16384"))]
        // static field: DEFAULT_BUFFER_SIZE:I
        pub fn DEFAULT_BUFFER_SIZE() -> i32 {
            16384
        }

        #[cfg_attr(any(), java_field(name = "MAX_BUFFER_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2147483639"))]
        // static field: MAX_BUFFER_SIZE:I
        pub fn MAX_BUFFER_SIZE() -> i32 {
            2147483639
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "nullInputStream", descriptor = "()Ljava/io/InputStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nullInputStream() -> Result<InputStream> {
            panic!("stub: java/io/InputStream.nullInputStream:()Ljava/io/InputStream;")
        }

        #[java_method(name = "read", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read(&self) -> Result<i32> {
            panic!("stub: java/io/InputStream.read:()I")
        }

        #[java_method(name = "read", descriptor = "([B)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b(&self, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/io/InputStream.read:([B)I")
        }

        #[java_method(name = "read", descriptor = "([BII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: read([BII)I
        pub fn read_arr_b_i_i(&self, mut b: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<i32> {
            let this = self;
            let _t0: i32 = Objects::checkFromIndexSize_i_i_i(off, len, (b.borrow().len() as i32))?;
            if (len==0) {
                return Ok(0i32);
            }
            let _t1 = this.read()?;
            let mut c: i32 = _t1;
            if c == -1i32 {
                return Ok(-1i32);
            }
            b.borrow_mut()[off as usize] = (((c) as i8 as i32)) as i8;
            let mut i: i32 = 1i32;
            loop {
                if i >= len { break; }
                let _t2 = this.read()?;
                c = _t2;
                if c == -1i32 {
                    break;
                }
                b.borrow_mut()[(off).wrapping_add(i) as usize] = (((c) as i8 as i32)) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(i)
        }

        #[java_method(name = "readAllBytes", descriptor = "()[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readAllBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/io/InputStream.readAllBytes:()[B")
        }

        #[java_method(name = "readNBytes", descriptor = "(I)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readNBytes_i(&self, len: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/io/InputStream.readNBytes:(I)[B")
        }

        #[java_method(name = "readNBytes", descriptor = "([BII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readNBytes_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<i32> {
            panic!("stub: java/io/InputStream.readNBytes:([BII)I")
        }

        #[java_method(name = "skip", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn skip(&self, n: i64) -> Result<i64> {
            panic!("stub: java/io/InputStream.skip:(J)J")
        }

        #[java_method(name = "skipNBytes", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn skipNBytes(&self, n: i64) -> Result<()> {
            panic!("stub: java/io/InputStream.skipNBytes:(J)V")
        }

        #[java_method(name = "available", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn available(&self) -> Result<i32> {
            panic!("stub: java/io/InputStream.available:()I")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/io/InputStream.close:()V")
        }

        #[java_method(name = "mark", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self, readlimit: i32) -> Result<()> {
            panic!("stub: java/io/InputStream.mark:(I)V")
        }

        #[java_method(name = "reset", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn reset(&self) -> Result<()> {
            panic!("stub: java/io/InputStream.reset:()V")
        }

        #[java_method(name = "markSupported", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn markSupported(&self) -> Result<bool> {
            panic!("stub: java/io/InputStream.markSupported:()Z")
        }

        #[java_method(name = "transferTo", descriptor = "(Ljava/io/OutputStream;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn transferTo(&self, out: OutputStream) -> Result<i64> {
            panic!("stub: java/io/InputStream.transferTo:(Ljava/io/OutputStream;)J")
        }
    }
}
