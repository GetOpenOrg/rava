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

impl From<Comparators_NaturalOrderComparator> for Enum<Object> {
    fn from(v: Comparators_NaturalOrderComparator) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Comparators$NaturalOrderComparator"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = "java/util/Comparator"]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/util/Comparators$NaturalOrderComparator;>;Ljava/util/Comparator<Ljava/lang/Comparable<Ljava/lang/Object;>;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Comparators.java"]
    #[inner_classes     = "java/util/Comparators$NaturalOrderComparator:java/util/Comparators:NaturalOrderComparator:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/util/Comparator;java/util/Comparators$NaturalOrderComparator"]

    pub struct Comparators_NaturalOrderComparator;

    impl Comparators_NaturalOrderComparator {
        #[cfg_attr(any(), java_field(name = "INSTANCE", descriptor = "Ljava/util/Comparators$NaturalOrderComparator;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSTANCE:Ljava/util/Comparators$NaturalOrderComparator;
        pub fn INSTANCE() -> Comparators_NaturalOrderComparator {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.INSTANCE:Ljava/util/Comparators$NaturalOrderComparator;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/util/Comparators$NaturalOrderComparator;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/util/Comparators$NaturalOrderComparator;
        pub fn _VALUES() -> Rc<RefCell<Vec<Comparators_NaturalOrderComparator>>> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.$VALUES:[Ljava/util/Comparators$NaturalOrderComparator;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/util/Comparators$NaturalOrderComparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Comparators_NaturalOrderComparator>>>> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.values:()[Ljava/util/Comparators$NaturalOrderComparator;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/util/Comparators$NaturalOrderComparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<Comparators_NaturalOrderComparator> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.valueOf:(Ljava/lang/String;)Ljava/util/Comparators$NaturalOrderComparator;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "compare", descriptor = "(Ljava/lang/Comparable;Ljava/lang/Comparable;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Comparable<Ljava/lang/Object;>;Ljava/lang/Comparable<Ljava/lang/Object;>;)I")]
        pub fn compare(&self, c1: Object, c2: Object) -> Result<i32> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.compare:(Ljava/lang/Comparable;Ljava/lang/Comparable;)I")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<Ljava/lang/Comparable<Ljava/lang/Object;>;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.reversed:()Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparing", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TT;>;)Ljava/util/Comparator<TT;>;")]
        // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
        pub fn thenComparing_compar(&self, mut other: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&other))?;
            let __lam_cap13_0 = other;
            let __lam_cap13_1 = this;
            let __lam_13: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<i32> { Comparator::lambda_thenComparing_36697e65_1(__lam_cap13_0.clone(), __lam_cap13_1.clone(), _la0, _la1) });
            Ok(Object::from_any(__lam_13))
        }

        #[java_method(name = "thenComparing", descriptor = "(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;Ljava/util/Comparator<-TU;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparing_functi_compar(&self, keyExtractor: Object, keyComparator: Object) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.thenComparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparing", descriptor = "(Ljava/util/function/Function;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U::Ljava/lang/Comparable<-TU;>;>(Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparing_functi(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.thenComparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingInt(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.thenComparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingLong(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.thenComparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingDouble(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Comparators$NaturalOrderComparator.thenComparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
        }
    }
}
