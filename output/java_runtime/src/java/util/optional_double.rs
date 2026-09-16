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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/OptionalDouble"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "OptionalDouble.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/OptionalDouble"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct OptionalDouble {
        #[cfg_attr(any(), java_field(name = "isPresent", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub isPresent: bool,
        #[cfg_attr(any(), java_field(name = "value", descriptor = "D", access = "private", modifiers = "final", is_static = false))]
        pub value: f64,
    }

    impl OptionalDouble {
        #[cfg_attr(any(), java_field(name = "EMPTY", descriptor = "Ljava/util/OptionalDouble;", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY:Ljava/util/OptionalDouble;
        pub fn EMPTY() -> OptionalDouble {
            panic!("stub: java/util/OptionalDouble.EMPTY:Ljava/util/OptionalDouble;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_isPresent((0i32 != 0i32));
            this.__set_value(f64::NAN);
            Ok(this)
        }

        #[java_method(name = "empty", descriptor = "()Ljava/util/OptionalDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn empty() -> Result<OptionalDouble> {
            Ok(OptionalDouble::EMPTY())
        }

        #[java_method(name = "<init>", descriptor = "(D)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(D)V
        pub fn new_d(mut value: f64) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_isPresent((1i32 != 0i32));
            this.__set_value(value);
            Ok(this)
        }

        #[java_method(name = "of", descriptor = "(D)Ljava/util/OptionalDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(mut value: f64) -> Result<OptionalDouble> {
            Ok(OptionalDouble::new_d(value)?)
        }

        #[java_method(name = "getAsDouble", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAsDouble(&self) -> Result<f64> {
            let this = self;
            if !(this.__get_isPresent()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(this.__get_value())
        }

        #[java_method(name = "isPresent", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPresent(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_isPresent())
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/OptionalDouble.isEmpty:()Z")
        }

        #[java_method(name = "ifPresent", descriptor = "(Ljava/util/function/DoubleConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ifPresent(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/OptionalDouble.ifPresent:(Ljava/util/function/DoubleConsumer;)V")
        }

        #[java_method(name = "ifPresentOrElse", descriptor = "(Ljava/util/function/DoubleConsumer;Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ifPresentOrElse(&self, action: Object, emptyAction: Object) -> Result<()> {
            panic!("stub: java/util/OptionalDouble.ifPresentOrElse:(Ljava/util/function/DoubleConsumer;Ljava/lang/Runnable;)V")
        }

        #[java_method(name = "stream", descriptor = "()Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream(&self) -> Result<Object> {
            panic!("stub: java/util/OptionalDouble.stream:()Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "orElse", descriptor = "(D)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn orElse(&self, other: f64) -> Result<f64> {
            panic!("stub: java/util/OptionalDouble.orElse:(D)D")
        }

        #[java_method(name = "orElseGet", descriptor = "(Ljava/util/function/DoubleSupplier;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn orElseGet(&self, supplier: Object) -> Result<f64> {
            panic!("stub: java/util/OptionalDouble.orElseGet:(Ljava/util/function/DoubleSupplier;)D")
        }

        #[java_method(name = "orElseThrow", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn orElseThrow(&self) -> Result<f64> {
            panic!("stub: java/util/OptionalDouble.orElseThrow:()D")
        }

        #[java_method(name = "orElseThrow", descriptor = "(Ljava/util/function/Supplier;)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Throwable", generic_signature = "<X:Ljava/lang/Throwable;>(Ljava/util/function/Supplier<+TX;>;)D^TX;")]
        pub fn orElseThrow_suppli(&self, exceptionSupplier: Object) -> Result<f64> {
            panic!("stub: java/util/OptionalDouble.orElseThrow:(Ljava/util/function/Supplier;)D")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/OptionalDouble.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
