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
    #[binary_name       = "java/util/Collections$ReverseComparator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Comparator,java/io/Serializable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/util/Comparator<Ljava/lang/Comparable<Ljava/lang/Object;>;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Collections.java"]
    #[inner_classes     = "java/util/Collections$ReverseComparator:java/util/Collections:ReverseComparator:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/Collections$ReverseComparator;java/util/Comparator"]

    pub struct Collections_ReverseComparator;

    impl Collections_ReverseComparator {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "7207038068494060240"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            7207038068494060240i64
        }

        #[cfg_attr(any(), java_field(name = "REVERSE_ORDER", descriptor = "Ljava/util/Collections$ReverseComparator;", access = "package", modifiers = "static final", is_static = true))]
        // static field: REVERSE_ORDER:Ljava/util/Collections$ReverseComparator;
        pub fn REVERSE_ORDER() -> Collections_ReverseComparator {
            panic!("stub: java/util/Collections$ReverseComparator.REVERSE_ORDER:Ljava/util/Collections$ReverseComparator;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Collections$ReverseComparator.<init>:()V")
        }

        #[java_method(name = "compare", descriptor = "(Ljava/lang/Comparable;Ljava/lang/Comparable;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Comparable<Ljava/lang/Object;>;Ljava/lang/Comparable<Ljava/lang/Object;>;)I")]
        pub fn compare(&self, c1: Object, c2: Object) -> Result<i32> {
            panic!("stub: java/util/Collections$ReverseComparator.compare:(Ljava/lang/Comparable;Ljava/lang/Comparable;)I")
        }

        #[java_method(name = "readResolve", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn readResolve(&self) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.readResolve:()Ljava/lang/Object;")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<Ljava/lang/Comparable<Ljava/lang/Object;>;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.reversed:()Ljava/util/Comparator;")
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
            panic!("stub: java/util/Collections$ReverseComparator.thenComparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparing", descriptor = "(Ljava/util/function/Function;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U::Ljava/lang/Comparable<-TU;>;>(Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparing_functi(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.thenComparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingInt(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.thenComparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingLong(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.thenComparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
        }

        #[java_method(name = "thenComparingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn thenComparingDouble(&self, keyExtractor: Object) -> Result<Object> {
            panic!("stub: java/util/Collections$ReverseComparator.thenComparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
        }
    }
}
