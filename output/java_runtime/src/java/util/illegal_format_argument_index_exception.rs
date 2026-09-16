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

impl From<IllegalFormatArgumentIndexException> for IllegalFormatException {
    fn from(v: IllegalFormatArgumentIndexException) -> IllegalFormatException { v.__into_super() }
}

impl From<IllegalFormatArgumentIndexException> for IllegalArgumentException {
    fn from(v: IllegalFormatArgumentIndexException) -> IllegalArgumentException { v.__into_super().__into_super() }
}

impl From<IllegalFormatArgumentIndexException> for RuntimeException {
    fn from(v: IllegalFormatArgumentIndexException) -> RuntimeException { v.__into_super().__into_super().__into_super() }
}

impl From<IllegalFormatArgumentIndexException> for Exception {
    fn from(v: IllegalFormatArgumentIndexException) -> Exception { v.__into_super().__into_super().__into_super().__into_super() }
}

impl From<IllegalFormatArgumentIndexException> for Throwable {
    fn from(v: IllegalFormatArgumentIndexException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/IllegalFormatArgumentIndexException"]
    #[super_class       = "java/util/IllegalFormatException"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IllegalFormatArgumentIndexException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalFormatException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/IllegalFormatArgumentIndexException;java/util/IllegalFormatException"]

    pub struct IllegalFormatArgumentIndexException {
        #[cfg_attr(any(), java_field(name = "illegalIndex", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub illegalIndex: i32,
    }

    impl IllegalFormatArgumentIndexException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4191767811181838112"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            4191767811181838112i64
        }

        #[java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut index: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalFormatException::new()?);
            this.__set_illegalIndex(index);
            Ok(this)
        }

        #[java_method(name = "getIndex", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIndex(&self) -> Result<i32> {
            panic!("stub: java/util/IllegalFormatArgumentIndexException.getIndex:()I")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/util/IllegalFormatArgumentIndexException.getMessage:()Ljava/lang/String;")
        }
    }
}
