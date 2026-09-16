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

impl From<IllegalFormatConversionException> for IllegalFormatException {
    fn from(v: IllegalFormatConversionException) -> IllegalFormatException { v.__into_super() }
}

impl From<IllegalFormatConversionException> for IllegalArgumentException {
    fn from(v: IllegalFormatConversionException) -> IllegalArgumentException { v.__into_super().__into_super() }
}

impl From<IllegalFormatConversionException> for RuntimeException {
    fn from(v: IllegalFormatConversionException) -> RuntimeException { v.__into_super().__into_super().__into_super() }
}

impl From<IllegalFormatConversionException> for Exception {
    fn from(v: IllegalFormatConversionException) -> Exception { v.__into_super().__into_super().__into_super().__into_super() }
}

impl From<IllegalFormatConversionException> for Throwable {
    fn from(v: IllegalFormatConversionException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/IllegalFormatConversionException"]
    #[super_class       = "java/util/IllegalFormatException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IllegalFormatConversionException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalFormatException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/IllegalFormatConversionException;java/util/IllegalFormatException"]

    pub struct IllegalFormatConversionException {
        #[cfg_attr(any(), java_field(name = "c", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub c: u16,
        #[cfg_attr(any(), java_field(name = "arg", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
        pub arg: Class<Object>,
    }

    impl IllegalFormatConversionException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "17000126"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            17000126i64
        }

        #[java_method(name = "<init>", descriptor = "(CLjava/lang/Class;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(CLjava/lang/Class<*>;)V")]
        pub fn new(mut c: u16, mut arg: Object) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalFormatException::new()?);
            if _is_jnull(&arg) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_c(c);
            this.__set_arg(Clone::clone(&arg));
            Ok(this)
        }

        #[java_method(name = "getConversion", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getConversion(&self) -> Result<u16> {
            panic!("stub: java/util/IllegalFormatConversionException.getConversion:()C")
        }

        #[java_method(name = "getArgumentClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getArgumentClass(&self) -> Result<Object> {
            panic!("stub: java/util/IllegalFormatConversionException.getArgumentClass:()Ljava/lang/Class;")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/util/IllegalFormatConversionException.getMessage:()Ljava/lang/String;")
        }
    }
}
