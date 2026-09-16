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

impl From<StringIndexOutOfBoundsException> for IndexOutOfBoundsException {
    fn from(v: StringIndexOutOfBoundsException) -> IndexOutOfBoundsException { v.__into_super() }
}

impl From<StringIndexOutOfBoundsException> for RuntimeException {
    fn from(v: StringIndexOutOfBoundsException) -> RuntimeException { v.__into_super().__into_super() }
}

impl From<StringIndexOutOfBoundsException> for Exception {
    fn from(v: StringIndexOutOfBoundsException) -> Exception { v.__into_super().__into_super().__into_super() }
}

impl From<StringIndexOutOfBoundsException> for Throwable {
    fn from(v: StringIndexOutOfBoundsException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/StringIndexOutOfBoundsException"]
    #[super_class       = "java/lang/IndexOutOfBoundsException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringIndexOutOfBoundsException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IndexOutOfBoundsException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IndexOutOfBoundsException;java/lang/Object;java/lang/RuntimeException;java/lang/StringIndexOutOfBoundsException;java/lang/Throwable"]

    pub struct StringIndexOutOfBoundsException;

    impl StringIndexOutOfBoundsException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-6762910422159637258"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -6762910422159637258i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IndexOutOfBoundsException::new()?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/StringIndexOutOfBoundsException.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut index: i32) -> Result<Self> {
            let mut this = Self::default();
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("String index out of range: ")))?;
            let _t1 = _t0.append_i(index)?;
            let _t2 = _t1.toString()?;
            this = Self::__new_with_super(IndexOutOfBoundsException::new_str(Clone::clone(&_t2))?);
            Ok(this)
        }
    }
}
