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

impl From<DualPivotQuicksort_RunMerger> for RecursiveTask<Object> {
    fn from(v: DualPivotQuicksort_RunMerger) -> RecursiveTask<Object> { v.__into_super() }
}

impl From<DualPivotQuicksort_RunMerger> for ForkJoinTask<Object> {
    fn from(v: DualPivotQuicksort_RunMerger) -> ForkJoinTask<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/DualPivotQuicksort$RunMerger"]
    #[super_class       = "java/util/concurrent/RecursiveTask"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/util/concurrent/RecursiveTask<Ljava/lang/Object;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DualPivotQuicksort.java"]
    #[inner_classes     = "java/util/DualPivotQuicksort$RunMerger:java/util/DualPivotQuicksort:RunMerger:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "RecursiveTask<Object>"]
    #[superclass_fields(status: i32, aux: Object, result: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/DualPivotQuicksort$RunMerger;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future;java/util/concurrent/RecursiveTask"]

    pub struct DualPivotQuicksort_RunMerger {
        #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub a: Object,
        #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub b: Object,
        #[cfg_attr(any(), java_field(name = "run", descriptor = "[I", access = "private", modifiers = "final", is_static = false))]
        pub run: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "aim", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub aim: i32,
        #[cfg_attr(any(), java_field(name = "lo", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub lo: i32,
        #[cfg_attr(any(), java_field(name = "hi", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub hi: i32,
    }

    impl DualPivotQuicksort_RunMerger {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "20180818"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            20180818i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;II[III)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut a: Object, mut b: Object, mut offset: i32, mut aim: i32, mut run: Rc<RefCell<Vec<i32>>>, mut lo: i32, mut hi: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(RecursiveTask::new()?);
            this.__set_a(Clone::clone(&a));
            this.__set_b(Clone::clone(&b));
            this.__set_offset(offset);
            this.__set_aim(aim);
            this.__set_run(Clone::clone(&run));
            this.__set_lo(lo);
            this.__set_hi(hi);
            Ok(this)
        }

        #[java_method(name = "compute", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(&self) -> Result<Object> {
            panic!("stub: java/util/DualPivotQuicksort$RunMerger.compute:()Ljava/lang/Object;")
        }

        #[java_method(name = "forkMe", descriptor = "()Ljava/util/DualPivotQuicksort$RunMerger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forkMe(&self) -> Result<DualPivotQuicksort_RunMerger> {
            let this = self;
            let _t0 = this.__super().__super().fork()?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "getDestination", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDestination(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.__super().__super().join()?;
            let _t1 = this.__super().getRawResult()?;
            Ok(_t1)
        }
    }
}
