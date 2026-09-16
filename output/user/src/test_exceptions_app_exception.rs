#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::r#ref::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::math::*;
use java_runtime::java::nio::*;
use java_runtime::java::nio::charset::*;
use java_runtime::java::security::*;
use java_runtime::java::text::*;
use java_runtime::java::text::spi::*;
use java_runtime::java::time::*;
use java_runtime::java::time::chrono::*;
use java_runtime::java::time::temporal::*;
use java_runtime::java::time::zone::*;
use java_runtime::java::util::*;
use java_runtime::java::util::concurrent::*;
use java_runtime::java::util::concurrent::atomic::*;
use java_runtime::java::util::concurrent::locks::*;
use java_runtime::java::util::function::*;
use java_runtime::java::util::regex::*;
use java_runtime::java::util::spi::*;
use java_runtime::java::util::stream::*;
use java_runtime::java::util::zip::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::reflect::generics::factory::*;
use java_runtime::sun::reflect::generics::repository::*;
use java_runtime::sun::reflect::generics::scope::*;
use java_runtime::sun::reflect::misc::*;
use java_runtime::sun::security::action::*;
use java_runtime::sun::security::util::*;
use java_runtime::sun::text::*;
use java_runtime::sun::util::*;
use java_runtime::sun::util::calendar::*;
use java_runtime::sun::util::locale::*;
use java_runtime::sun::util::locale::provider::*;
use java_runtime::sun::util::spi::*;
use java_runtime::java::text::Normalizer;
use crate::test_exceptions::TestExceptions;

impl From<TestExceptions_AppException> for RuntimeException {
    fn from(v: TestExceptions_AppException) -> RuntimeException { v.__into_super() }
}

impl From<TestExceptions_AppException> for Exception {
    fn from(v: TestExceptions_AppException) -> Exception { v.__into_super().__into_super() }
}

impl From<TestExceptions_AppException> for Throwable {
    fn from(v: TestExceptions_AppException) -> Throwable { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestExceptions$AppException"]
    #[super_class       = "java/lang/RuntimeException"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestExceptions.java"]
    #[inner_classes     = "TestExceptions$AppException:TestExceptions:AppException:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "RuntimeException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "TestExceptions$AppException;java/io/Serializable;java/lang/Exception;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable"]

    pub struct TestExceptions_AppException {
        #[cfg_attr(any(), java_field(name = "code", descriptor = "I", is_static = false))]
        pub code: i32,
    }

    impl TestExceptions_AppException {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut msg: String, mut code: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(RuntimeException::new_str(Clone::clone(&msg))?);
            this.__set_code(code);
            Ok(this)
        }
    }
}
