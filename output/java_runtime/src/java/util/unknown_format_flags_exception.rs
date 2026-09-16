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

impl From<UnknownFormatFlagsException> for IllegalFormatException {
    fn from(v: UnknownFormatFlagsException) -> IllegalFormatException { v.__into_super() }
}

impl From<UnknownFormatFlagsException> for IllegalArgumentException {
    fn from(v: UnknownFormatFlagsException) -> IllegalArgumentException { v.__into_super().__into_super() }
}

impl From<UnknownFormatFlagsException> for RuntimeException {
    fn from(v: UnknownFormatFlagsException) -> RuntimeException { v.__into_super().__into_super().__into_super() }
}

impl From<UnknownFormatFlagsException> for Exception {
    fn from(v: UnknownFormatFlagsException) -> Exception { v.__into_super().__into_super().__into_super().__into_super() }
}

impl From<UnknownFormatFlagsException> for Throwable {
    fn from(v: UnknownFormatFlagsException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/UnknownFormatFlagsException"]
    #[super_class       = "java/util/IllegalFormatException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "UnknownFormatFlagsException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalFormatException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/IllegalFormatException;java/util/UnknownFormatFlagsException"]

    pub struct UnknownFormatFlagsException {
        #[cfg_attr(any(), java_field(name = "flags", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub flags: String,
    }

    impl UnknownFormatFlagsException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "19370506"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            19370506i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut f: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalFormatException::new()?);
            if _is_jnull(&f) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_flags(Clone::clone(&f));
            Ok(this)
        }

        #[java_method(name = "getFlags", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFlags(&self) -> Result<String> {
            panic!("stub: java/util/UnknownFormatFlagsException.getFlags:()Ljava/lang/String;")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/util/UnknownFormatFlagsException.getMessage:()Ljava/lang/String;")
        }
    }
}
