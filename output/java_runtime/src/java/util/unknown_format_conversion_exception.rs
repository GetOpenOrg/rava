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

impl From<UnknownFormatConversionException> for IllegalFormatException {
    fn from(v: UnknownFormatConversionException) -> IllegalFormatException { v.__into_super() }
}

impl From<UnknownFormatConversionException> for IllegalArgumentException {
    fn from(v: UnknownFormatConversionException) -> IllegalArgumentException { v.__into_super().__into_super() }
}

impl From<UnknownFormatConversionException> for RuntimeException {
    fn from(v: UnknownFormatConversionException) -> RuntimeException { v.__into_super().__into_super().__into_super() }
}

impl From<UnknownFormatConversionException> for Exception {
    fn from(v: UnknownFormatConversionException) -> Exception { v.__into_super().__into_super().__into_super().__into_super() }
}

impl From<UnknownFormatConversionException> for Throwable {
    fn from(v: UnknownFormatConversionException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/UnknownFormatConversionException"]
    #[super_class       = "java/util/IllegalFormatException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "UnknownFormatConversionException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalFormatException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/IllegalFormatException;java/util/UnknownFormatConversionException"]

    pub struct UnknownFormatConversionException {
        #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub s: String,
    }

    impl UnknownFormatConversionException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "19060418"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            19060418i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut s: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalFormatException::new()?);
            if _is_jnull(&s) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_s(Clone::clone(&s));
            Ok(this)
        }

        #[java_method(name = "getConversion", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getConversion(&self) -> Result<String> {
            panic!("stub: java/util/UnknownFormatConversionException.getConversion:()Ljava/lang/String;")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/util/UnknownFormatConversionException.getMessage:()Ljava/lang/String;")
        }
    }
}
