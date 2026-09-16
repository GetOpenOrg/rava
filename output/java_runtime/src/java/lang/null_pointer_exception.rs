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

impl From<NullPointerException> for RuntimeException {
    fn from(v: NullPointerException) -> RuntimeException { v.__into_super() }
}

impl From<NullPointerException> for Exception {
    fn from(v: NullPointerException) -> Exception { v.__into_super().__into_super() }
}

impl From<NullPointerException> for Throwable {
    fn from(v: NullPointerException) -> Throwable { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/NullPointerException"]
    #[super_class       = "java/lang/RuntimeException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "NullPointerException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "RuntimeException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/NullPointerException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable"]

    pub struct NullPointerException {
        #[cfg_attr(any(), java_field(name = "extendedMessageState", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub extendedMessageState: i32,
        #[cfg_attr(any(), java_field(name = "extendedMessage", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub extendedMessage: String,
    }

    impl NullPointerException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5162710183389028792"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5162710183389028792i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(RuntimeException::new()?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut s: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(RuntimeException::new_str(Clone::clone(&s))?);
            Ok(this)
        }

        #[java_method(name = "fillInStackTrace", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fillInStackTrace(&self) -> Result<Throwable> {
            panic!("stub: java/lang/NullPointerException.fillInStackTrace:()Ljava/lang/Throwable;")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/lang/NullPointerException.getMessage:()Ljava/lang/String;")
        }
    }
}
