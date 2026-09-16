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

impl From<AssertionError> for Error {
    fn from(v: AssertionError) -> Error { v.__into_super() }
}

impl From<AssertionError> for Throwable {
    fn from(v: AssertionError) -> Throwable { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/AssertionError"]
    #[super_class       = "java/lang/Error"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AssertionError.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Error"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/AssertionError;java/lang/Error;java/lang/Object;java/lang/Throwable"]

    pub struct AssertionError;

    impl AssertionError {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-5013299493970297370"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -5013299493970297370i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Error::new()?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut detailMessage: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Error::new_str(Clone::clone(&detailMessage))?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/Object;)V
        pub fn new_obj(mut detailMessage: Object) -> Result<Self> {
            let mut this = Self::default();
            this = AssertionError::new_str(Clone::clone(&String::from_owned(format!("{}", detailMessage))))?;
            if (detailMessage.is_instance_of("java/lang/Throwable")) {
                let _t0 = this.__super().__super().initCause(Clone::clone(&(detailMessage).downcast::<Throwable>()))?;
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_z(detailMessage: bool) -> Result<Self> {
            panic!("stub: java/lang/AssertionError.<init>:(Z)V")
        }

        #[java_method(name = "<init>", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_c(detailMessage: u16) -> Result<Self> {
            panic!("stub: java/lang/AssertionError.<init>:(C)V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut detailMessage: i32) -> Result<Self> {
            let mut this = Self::default();
            this = AssertionError::new_str(Clone::clone(&String::from_owned(format!("{}", detailMessage))))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(J)V
        pub fn new_l(mut detailMessage: i64) -> Result<Self> {
            let mut this = Self::default();
            this = AssertionError::new_str(Clone::clone(&String::from_owned(format!("{}", detailMessage))))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_f(detailMessage: f32) -> Result<Self> {
            panic!("stub: java/lang/AssertionError.<init>:(F)V")
        }

        #[java_method(name = "<init>", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_d(detailMessage: f64) -> Result<Self> {
            panic!("stub: java/lang/AssertionError.<init>:(D)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
            panic!("stub: java/lang/AssertionError.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
        }
    }
}
