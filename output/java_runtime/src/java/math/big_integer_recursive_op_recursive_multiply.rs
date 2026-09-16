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

impl From<BigInteger_RecursiveOp_RecursiveMultiply> for BigInteger_RecursiveOp {
    fn from(v: BigInteger_RecursiveOp_RecursiveMultiply) -> BigInteger_RecursiveOp { v.__into_super() }
}

impl From<BigInteger_RecursiveOp_RecursiveMultiply> for RecursiveTask<Object> {
    fn from(v: BigInteger_RecursiveOp_RecursiveMultiply) -> RecursiveTask<Object> { v.__into_super().__into_super() }
}

impl From<BigInteger_RecursiveOp_RecursiveMultiply> for ForkJoinTask<Object> {
    fn from(v: BigInteger_RecursiveOp_RecursiveMultiply) -> ForkJoinTask<Object> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/math/BigInteger$RecursiveOp$RecursiveMultiply"]
    #[super_class       = "java/math/BigInteger$RecursiveOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BigInteger.java"]
    #[inner_classes     = "java/math/BigInteger$RecursiveOp:java/math/BigInteger:RecursiveOp:1034;java/math/BigInteger$RecursiveOp$RecursiveMultiply:java/math/BigInteger$RecursiveOp:RecursiveMultiply:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "BigInteger_RecursiveOp"]
    #[superclass_fields(status: i32, aux: Object, result: Object, parallel: bool, depth: i8)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/math/BigInteger$RecursiveOp;java/math/BigInteger$RecursiveOp$RecursiveMultiply;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future;java/util/concurrent/RecursiveTask"]

    pub struct BigInteger_RecursiveOp_RecursiveMultiply {
        #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "final", is_static = false))]
        pub a: BigInteger,
        #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "final", is_static = false))]
        pub b: BigInteger,
    }

    impl BigInteger_RecursiveOp_RecursiveMultiply {
        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut a: BigInteger, mut b: BigInteger, mut parallel: bool, mut depth: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(BigInteger_RecursiveOp::new(parallel, depth)?);
            this.__set_a(Clone::clone(&a));
            this.__set_b(Clone::clone(&b));
            Ok(this)
        }

        #[java_method(name = "compute", descriptor = "()Ljava/math/BigInteger;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(&self) -> Result<BigInteger> {
            panic!("stub: java/math/BigInteger$RecursiveOp$RecursiveMultiply.compute:()Ljava/math/BigInteger;")
        }
    }
}
