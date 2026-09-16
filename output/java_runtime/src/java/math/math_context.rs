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
    #[binary_name       = "java/math/MathContext"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "MathContext.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/math/MathContext"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct MathContext {
        #[cfg_attr(any(), java_field(name = "precision", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub precision: i32,
        #[cfg_attr(any(), java_field(name = "roundingMode", descriptor = "Ljava/math/RoundingMode;", access = "package", modifiers = "final", is_static = false))]
        pub roundingMode: RoundingMode,
    }

    impl MathContext {
        #[cfg_attr(any(), java_field(name = "DEFAULT_ROUNDINGMODE", descriptor = "Ljava/math/RoundingMode;", access = "private", modifiers = "static final", is_static = true))]
        // static field: DEFAULT_ROUNDINGMODE:Ljava/math/RoundingMode;
        pub fn DEFAULT_ROUNDINGMODE() -> RoundingMode {
            panic!("stub: java/math/MathContext.DEFAULT_ROUNDINGMODE:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "MIN_DIGITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: MIN_DIGITS:I
        pub fn MIN_DIGITS() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "5579720004786848255"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5579720004786848255i64
        }

        #[cfg_attr(any(), java_field(name = "UNLIMITED", descriptor = "Ljava/math/MathContext;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNLIMITED:Ljava/math/MathContext;
        pub fn UNLIMITED() -> MathContext {
            panic!("stub: java/math/MathContext.UNLIMITED:Ljava/math/MathContext;")
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL32", descriptor = "Ljava/math/MathContext;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DECIMAL32:Ljava/math/MathContext;
        pub fn DECIMAL32() -> MathContext {
            panic!("stub: java/math/MathContext.DECIMAL32:Ljava/math/MathContext;")
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL64", descriptor = "Ljava/math/MathContext;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DECIMAL64:Ljava/math/MathContext;
        pub fn DECIMAL64() -> MathContext {
            panic!("stub: java/math/MathContext.DECIMAL64:Ljava/math/MathContext;")
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL128", descriptor = "Ljava/math/MathContext;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DECIMAL128:Ljava/math/MathContext;
        pub fn DECIMAL128() -> MathContext {
            panic!("stub: java/math/MathContext.DECIMAL128:Ljava/math/MathContext;")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut setPrecision: i32) -> Result<Self> {
            let mut this = Self::default();
            this = MathContext::new_i_roundi(setPrecision, Clone::clone(&MathContext::DEFAULT_ROUNDINGMODE()))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(ILjava/math/RoundingMode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(ILjava/math/RoundingMode;)V
        pub fn new_i_roundi(mut setPrecision: i32, mut setRoundingMode: RoundingMode) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            if (setPrecision<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if _is_jnull(&setRoundingMode) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_precision(setPrecision);
            this.__set_roundingMode(Clone::clone(&setRoundingMode));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(val: String) -> Result<Self> {
            panic!("stub: java/math/MathContext.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getPrecision", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPrecision(&self) -> Result<i32> {
            panic!("stub: java/math/MathContext.getPrecision:()I")
        }

        #[java_method(name = "getRoundingMode", descriptor = "()Ljava/math/RoundingMode;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRoundingMode(&self) -> Result<RoundingMode> {
            panic!("stub: java/math/MathContext.getRoundingMode:()Ljava/math/RoundingMode;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, x: Object) -> Result<bool> {
            panic!("stub: java/math/MathContext.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/math/MathContext.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
