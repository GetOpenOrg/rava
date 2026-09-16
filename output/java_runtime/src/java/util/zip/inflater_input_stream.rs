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

impl From<InflaterInputStream> for FilterInputStream {
    fn from(v: InflaterInputStream) -> FilterInputStream { v.__into_super() }
}

impl From<InflaterInputStream> for InputStream {
    fn from(v: InflaterInputStream) -> InputStream { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/zip/InflaterInputStream"]
    #[super_class       = "java/io/FilterInputStream"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "InflaterInputStream.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "FilterInputStream"]
    #[superclass_fields(in_: InputStream)]
    #[all_supertypes    = "java/io/Closeable;java/io/FilterInputStream;java/io/InputStream;java/lang/Object;java/util/zip/InflaterInputStream"]

    pub struct InflaterInputStream {
        #[cfg_attr(any(), java_field(name = "inf", descriptor = "Ljava/util/zip/Inflater;", access = "protected", modifiers = "", is_static = false))]
        pub inf: Inflater,
        #[cfg_attr(any(), java_field(name = "buf", descriptor = "[B", access = "protected", modifiers = "", is_static = false))]
        pub buf: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "len", descriptor = "I", access = "protected", modifiers = "", is_static = false))]
        pub len: i32,
        #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub closed: bool,
        #[cfg_attr(any(), java_field(name = "reachEOF", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub reachEOF: bool,
        #[cfg_attr(any(), java_field(name = "usesDefaultInflater", descriptor = "Z", is_static = false))]
        pub usesDefaultInflater: bool,
        #[cfg_attr(any(), java_field(name = "singleByteBuf", descriptor = "[B", access = "private", modifiers = "", is_static = false))]
        pub singleByteBuf: Rc<RefCell<Vec<i8>>>,
    }

    impl InflaterInputStream {
        #[java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn ensureOpen(&self) -> Result<()> {
            panic!("stub: java/util/zip/InflaterInputStream.ensureOpen:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;Ljava/util/zip/Inflater;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;I)V
        pub fn new_inputs_inflat_i(mut in_: InputStream, mut inf: Inflater, mut size: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(FilterInputStream::new(Clone::clone(&in_))?);
            this.__set_closed((0i32 != 0i32));
            this.__set_reachEOF((0i32 != 0i32));
            this.__set_usesDefaultInflater((0i32 != 0i32));
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; 1i32 as usize]));
            this.__set_singleByteBuf(Clone::clone(&_arr0));
            if _is_jnull(&inf) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (size<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_inf(Clone::clone(&inf));
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; size as usize]));
            this.__set_buf(Clone::clone(&_arr1));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;Ljava/util/zip/Inflater;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/io/InputStream;Ljava/util/zip/Inflater;)V
        pub fn new_inputs_inflat(mut in_: InputStream, mut inf: Inflater) -> Result<Self> {
            let mut this = Self::default();
            this = InflaterInputStream::new_inputs_inflat_i(Clone::clone(&in_), Clone::clone(&inf), 512i32)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/io/InputStream;)V
        pub fn new_inputs(mut in_: InputStream) -> Result<Self> {
            let mut this = Self::default();
            this = InflaterInputStream::new_inputs_inflat(Clone::clone(&in_), Clone::clone(&(if !_is_jnull(&in_) { Inflater::new()? } else { Default::default() })))?;
            this.__set_usesDefaultInflater((1i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "read", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read(&self) -> Result<i32> {
            panic!("stub: java/util/zip/InflaterInputStream.read:()I")
        }

        #[java_method(name = "read", descriptor = "([BII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<i32> {
            panic!("stub: java/util/zip/InflaterInputStream.read:([BII)I")
        }

        #[java_method(name = "available", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn available(&self) -> Result<i32> {
            panic!("stub: java/util/zip/InflaterInputStream.available:()I")
        }

        #[java_method(name = "skip", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn skip(&self, n: i64) -> Result<i64> {
            panic!("stub: java/util/zip/InflaterInputStream.skip:(J)J")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/util/zip/InflaterInputStream.close:()V")
        }

        #[java_method(name = "fill", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn fill(&self) -> Result<()> {
            panic!("stub: java/util/zip/InflaterInputStream.fill:()V")
        }

        #[java_method(name = "markSupported", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn markSupported(&self) -> Result<bool> {
            panic!("stub: java/util/zip/InflaterInputStream.markSupported:()Z")
        }

        #[java_method(name = "mark", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self, readlimit: i32) -> Result<()> {
            panic!("stub: java/util/zip/InflaterInputStream.mark:(I)V")
        }

        #[java_method(name = "reset", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn reset(&self) -> Result<()> {
            panic!("stub: java/util/zip/InflaterInputStream.reset:()V")
        }
    }
}
