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

impl From<BigInteger_RecursiveOp> for RecursiveTask<Object> {
    fn from(v: BigInteger_RecursiveOp) -> RecursiveTask<Object> { v.__into_super() }
}

impl From<BigInteger_RecursiveOp> for ForkJoinTask<Object> {
    fn from(v: BigInteger_RecursiveOp) -> ForkJoinTask<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/math/BigInteger$RecursiveOp"]
    #[super_class       = "java/util/concurrent/RecursiveTask"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "Ljava/util/concurrent/RecursiveTask<Ljava/math/BigInteger;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BigInteger.java"]
    #[inner_classes     = "java/math/BigInteger$RecursiveOp:java/math/BigInteger:RecursiveOp:1034;java/math/BigInteger$RecursiveOp$RecursiveMultiply:java/math/BigInteger$RecursiveOp:RecursiveMultiply:26;java/math/BigInteger$RecursiveOp$RecursiveSquare:java/math/BigInteger$RecursiveOp:RecursiveSquare:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "RecursiveTask<Object>"]
    #[superclass_fields(status: i32, aux: Object, result: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/math/BigInteger$RecursiveOp;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future;java/util/concurrent/RecursiveTask"]

    pub struct BigInteger_RecursiveOp {
        #[cfg_attr(any(), java_field(name = "parallel", descriptor = "Z", access = "package", modifiers = "final", is_static = false))]
        pub parallel: bool,
        #[cfg_attr(any(), java_field(name = "depth", descriptor = "B", access = "package", modifiers = "final", is_static = false))]
        pub depth: i8,
    }

    impl BigInteger_RecursiveOp {
        #[cfg_attr(any(), java_field(name = "PARALLEL_FORK_DEPTH_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: PARALLEL_FORK_DEPTH_THRESHOLD:I
        pub fn PARALLEL_FORK_DEPTH_THRESHOLD() -> i32 {
            panic!("stub: java/math/BigInteger$RecursiveOp.PARALLEL_FORK_DEPTH_THRESHOLD:I")
        }

        #[java_method(name = "calculateMaximumDepth", descriptor = "(I)I", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn calculateMaximumDepth(parallelism: i32) -> Result<i32> {
            panic!("stub: java/math/BigInteger$RecursiveOp.calculateMaximumDepth:(I)I")
        }

        #[java_method(name = "<init>", descriptor = "(ZI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut parallel: bool, mut depth: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(RecursiveTask::new()?);
            this.__set_parallel(parallel);
            this.__set_depth(((((depth) as i8 as i32)) as i8));
            Ok(this)
        }

        #[java_method(name = "getParallelForkDepthThreshold", descriptor = "()I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParallelForkDepthThreshold() -> Result<i32> {
            panic!("stub: java/math/BigInteger$RecursiveOp.getParallelForkDepthThreshold:()I")
        }

        #[java_method(name = "forkOrInvoke", descriptor = "()Ljava/util/concurrent/RecursiveTask;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/RecursiveTask<Ljava/math/BigInteger;>;")]
        pub fn forkOrInvoke(&self) -> Result<RecursiveTask<Object>> {
            panic!("stub: java/math/BigInteger$RecursiveOp.forkOrInvoke:()Ljava/util/concurrent/RecursiveTask;")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/math/BigInteger;Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask<Ljava/math/BigInteger;>;")]
        pub fn multiply(mut a: BigInteger, mut b: BigInteger, mut parallel: bool, mut depth: i32) -> Result<RecursiveTask<Object>> {
            let _t0 = BigInteger_RecursiveOp_RecursiveMultiply::new(Clone::clone(&a), Clone::clone(&b), parallel, depth)?.__super().forkOrInvoke()?;
            Ok(_t0)
        }

        #[java_method(name = "square", descriptor = "(Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/math/BigInteger;ZI)Ljava/util/concurrent/RecursiveTask<Ljava/math/BigInteger;>;")]
        pub fn square(mut a: BigInteger, mut parallel: bool, mut depth: i32) -> Result<RecursiveTask<Object>> {
            let _t0 = BigInteger_RecursiveOp_RecursiveSquare::new(Clone::clone(&a), parallel, depth)?.__super().forkOrInvoke()?;
            Ok(_t0)
        }
    }
}
