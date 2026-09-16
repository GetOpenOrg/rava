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

impl From<PatternSyntaxException> for IllegalArgumentException {
    fn from(v: PatternSyntaxException) -> IllegalArgumentException { v.__into_super() }
}

impl From<PatternSyntaxException> for RuntimeException {
    fn from(v: PatternSyntaxException) -> RuntimeException { v.__into_super().__into_super() }
}

impl From<PatternSyntaxException> for Exception {
    fn from(v: PatternSyntaxException) -> Exception { v.__into_super().__into_super().__into_super() }
}

impl From<PatternSyntaxException> for Throwable {
    fn from(v: PatternSyntaxException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/PatternSyntaxException"]
    #[super_class       = "java/lang/IllegalArgumentException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "PatternSyntaxException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalArgumentException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable;java/util/regex/PatternSyntaxException"]

    pub struct PatternSyntaxException {
        #[cfg_attr(any(), java_field(name = "desc", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub desc: String,
        #[cfg_attr(any(), java_field(name = "pattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub pattern: String,
        #[cfg_attr(any(), java_field(name = "index", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub index: i32,
    }

    impl PatternSyntaxException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-3864639126226059218"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -3864639126226059218i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut desc: String, mut regex: String, mut index: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalArgumentException::new()?);
            this.__set_desc(Clone::clone(&desc));
            this.__set_pattern(Clone::clone(&regex));
            this.__set_index(index);
            Ok(this)
        }

        #[java_method(name = "getIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIndex(&self) -> Result<i32> {
            panic!("stub: java/util/regex/PatternSyntaxException.getIndex:()I")
        }

        #[java_method(name = "getDescription", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDescription(&self) -> Result<String> {
            panic!("stub: java/util/regex/PatternSyntaxException.getDescription:()Ljava/lang/String;")
        }

        #[java_method(name = "getPattern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPattern(&self) -> Result<String> {
            panic!("stub: java/util/regex/PatternSyntaxException.getPattern:()Ljava/lang/String;")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            panic!("stub: java/util/regex/PatternSyntaxException.getMessage:()Ljava/lang/String;")
        }
    }
}
