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

impl From<DualPivotQuicksort_Merger> for CountedCompleter<Object> {
    fn from(v: DualPivotQuicksort_Merger) -> CountedCompleter<Object> { v.__into_super() }
}

impl From<DualPivotQuicksort_Merger> for ForkJoinTask<Object> {
    fn from(v: DualPivotQuicksort_Merger) -> ForkJoinTask<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/DualPivotQuicksort$Merger"]
    #[super_class       = "java/util/concurrent/CountedCompleter"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/util/concurrent/CountedCompleter<Ljava/lang/Void;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DualPivotQuicksort.java"]
    #[inner_classes     = "java/util/DualPivotQuicksort$Merger:java/util/DualPivotQuicksort:Merger:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CountedCompleter<Object>"]
    #[superclass_fields(status: i32, aux: Object, completer: CountedCompleter<Object>, pending: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/DualPivotQuicksort$Merger;java/util/concurrent/CountedCompleter;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future"]

    pub struct DualPivotQuicksort_Merger {
        #[cfg_attr(any(), java_field(name = "dst", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub dst: Object,
        #[cfg_attr(any(), java_field(name = "a1", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub a1: Object,
        #[cfg_attr(any(), java_field(name = "a2", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub a2: Object,
        #[cfg_attr(any(), java_field(name = "k", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub k: i32,
        #[cfg_attr(any(), java_field(name = "lo1", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub lo1: i32,
        #[cfg_attr(any(), java_field(name = "hi1", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub hi1: i32,
        #[cfg_attr(any(), java_field(name = "lo2", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub lo2: i32,
        #[cfg_attr(any(), java_field(name = "hi2", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub hi2: i32,
    }

    impl DualPivotQuicksort_Merger {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "20180818"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            20180818i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V")]
        pub fn new(mut parent: CountedCompleter<Object>, mut dst: Object, mut k: i32, mut a1: Object, mut lo1: i32, mut hi1: i32, mut a2: Object, mut lo2: i32, mut hi2: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(CountedCompleter::new_counte(Clone::clone(&parent))?);
            this.__set_dst(Clone::clone(&dst));
            this.__set_k(k);
            this.__set_a1(Clone::clone(&a1));
            this.__set_lo1(lo1);
            this.__set_hi1(hi1);
            this.__set_a2(Clone::clone(&a2));
            this.__set_lo2(lo2);
            this.__set_hi2(hi2);
            Ok(this)
        }

        #[java_method(name = "compute", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(&self) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort$Merger.compute:()V")
        }

        #[java_method(name = "forkMerger", descriptor = "(Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forkMerger(&self, mut dst: Object, mut k: i32, mut a1: Object, mut lo1: i32, mut hi1: i32, mut a2: Object, mut lo2: i32, mut hi2: i32) -> Result<()> {
            let this = self;
            this.__super().addToPendingCount(1i32)?;
            let _t0 = DualPivotQuicksort_Merger::new(Clone::clone(&this).into(), Clone::clone(&dst), k, Clone::clone(&a1), lo1, hi1, Clone::clone(&a2), lo2, hi2)?.__super().__super().fork()?;
            Ok(())
        }
    }
}
