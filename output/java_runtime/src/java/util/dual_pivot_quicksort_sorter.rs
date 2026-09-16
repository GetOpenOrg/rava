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

impl From<DualPivotQuicksort_Sorter> for CountedCompleter<Object> {
    fn from(v: DualPivotQuicksort_Sorter) -> CountedCompleter<Object> { v.__into_super() }
}

impl From<DualPivotQuicksort_Sorter> for ForkJoinTask<Object> {
    fn from(v: DualPivotQuicksort_Sorter) -> ForkJoinTask<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/DualPivotQuicksort$Sorter"]
    #[super_class       = "java/util/concurrent/CountedCompleter"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/util/concurrent/CountedCompleter<Ljava/lang/Void;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DualPivotQuicksort.java"]
    #[inner_classes     = "java/util/DualPivotQuicksort$Sorter:java/util/DualPivotQuicksort:Sorter:26;java/util/DualPivotQuicksort$Merger:java/util/DualPivotQuicksort:Merger:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CountedCompleter<Object>"]
    #[superclass_fields(status: i32, aux: Object, completer: CountedCompleter<Object>, pending: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/DualPivotQuicksort$Sorter;java/util/concurrent/CountedCompleter;java/util/concurrent/ForkJoinTask;java/util/concurrent/Future"]

    pub struct DualPivotQuicksort_Sorter {
        #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub a: Object,
        #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false))]
        pub b: Object,
        #[cfg_attr(any(), java_field(name = "low", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub low: i32,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub size: i32,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub depth: i32,
    }

    impl DualPivotQuicksort_Sorter {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "20180818"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            20180818i64
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;Ljava/lang/Object;IIII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;Ljava/lang/Object;Ljava/lang/Object;IIII)V")]
        pub fn new(mut parent: CountedCompleter<Object>, mut a: Object, mut b: Object, mut low: i32, mut size: i32, mut offset: i32, mut depth: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(CountedCompleter::new_counte(Clone::clone(&parent))?);
            this.__set_a(Clone::clone(&a));
            this.__set_b(Clone::clone(&b));
            this.__set_low(low);
            this.__set_size(size);
            this.__set_offset(offset);
            this.__set_depth(depth);
            Ok(this)
        }

        #[java_method(name = "compute", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(&self) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort$Sorter.compute:()V")
        }

        #[java_method(name = "onCompletion", descriptor = "(Ljava/util/concurrent/CountedCompleter;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/concurrent/CountedCompleter<*>;)V")]
        pub fn onCompletion(&self, caller: CountedCompleter<Object>) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort$Sorter.onCompletion:(Ljava/util/concurrent/CountedCompleter;)V")
        }

        #[java_method(name = "forkSorter", descriptor = "(III)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forkSorter(&self, mut depth: i32, mut low: i32, mut high: i32) -> Result<()> {
            let this = self;
            this.__super().addToPendingCount(1i32)?;
            let mut a = this.__get_a();
            let _t0 = DualPivotQuicksort_Sorter::new(Clone::clone(&this).into(), Clone::clone(&a), Clone::clone(&this.__get_b()), low, (high).wrapping_sub(low), this.__get_offset(), depth)?.__super().__super().fork()?;
            Ok(())
        }
    }
}
