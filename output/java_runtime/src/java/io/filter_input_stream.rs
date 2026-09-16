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

impl From<FilterInputStream> for InputStream {
    fn from(v: FilterInputStream) -> InputStream { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/FilterInputStream"]
    #[super_class       = "java/io/InputStream"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FilterInputStream.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "InputStream"]
    #[all_supertypes    = "java/io/Closeable;java/io/FilterInputStream;java/io/InputStream;java/lang/Object"]

    pub struct FilterInputStream {
        #[cfg_attr(any(), java_field(name = "in", descriptor = "Ljava/io/InputStream;", access = "protected", modifiers = "volatile", is_static = false))]
        pub in_: InputStream,
    }

    impl FilterInputStream {
        #[java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut in_: InputStream) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(InputStream::new()?);
            this.__set_in_(Clone::clone(&in_));
            Ok(this)
        }

        #[java_method(name = "read", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read(&self) -> Result<i32> {
            panic!("stub: java/io/FilterInputStream.read:()I")
        }

        #[java_method(name = "read", descriptor = "([B)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b(&self, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/io/FilterInputStream.read:([B)I")
        }

        #[java_method(name = "read", descriptor = "([BII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<i32> {
            panic!("stub: java/io/FilterInputStream.read:([BII)I")
        }

        #[java_method(name = "skip", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn skip(&self, n: i64) -> Result<i64> {
            panic!("stub: java/io/FilterInputStream.skip:(J)J")
        }

        #[java_method(name = "available", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn available(&self) -> Result<i32> {
            panic!("stub: java/io/FilterInputStream.available:()I")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/io/FilterInputStream.close:()V")
        }

        #[java_method(name = "mark", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self, readlimit: i32) -> Result<()> {
            panic!("stub: java/io/FilterInputStream.mark:(I)V")
        }

        #[java_method(name = "reset", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn reset(&self) -> Result<()> {
            panic!("stub: java/io/FilterInputStream.reset:()V")
        }

        #[java_method(name = "markSupported", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn markSupported(&self) -> Result<bool> {
            panic!("stub: java/io/FilterInputStream.markSupported:()Z")
        }
    }
}
