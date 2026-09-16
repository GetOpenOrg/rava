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
    #[binary_name       = "java/util/stream/Collectors"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Collectors.java"]
    #[inner_classes     = "java/util/stream/Collectors$CollectorImpl:java/util/stream/Collectors:CollectorImpl:24;java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/util/stream/Collectors$1PairBox::PairBox:0;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/stream/Collectors$Partition:java/util/stream/Collectors:Partition:26;java/util/stream/Collectors$1OptionalBox::OptionalBox:0;java/util/stream/Collectors$Partition$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/Collectors"]

    pub struct Collectors;

    impl Collectors {
        #[cfg_attr(any(), java_field(name = "CH_CONCURRENT_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_CONCURRENT_ID:Ljava/util/Set;
        pub fn CH_CONCURRENT_ID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_CONCURRENT_ID:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "CH_CONCURRENT_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_CONCURRENT_NOID:Ljava/util/Set;
        pub fn CH_CONCURRENT_NOID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_CONCURRENT_NOID:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "CH_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_ID:Ljava/util/Set;
        pub fn CH_ID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_ID:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "CH_UNORDERED_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_UNORDERED_ID:Ljava/util/Set;
        pub fn CH_UNORDERED_ID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_UNORDERED_ID:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "CH_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_NOID:Ljava/util/Set;
        pub fn CH_NOID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_NOID:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "CH_UNORDERED_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
        // static field: CH_UNORDERED_NOID:Ljava/util/Set;
        pub fn CH_UNORDERED_NOID() -> Object {
            panic!("stub: java/util/stream/Collectors.CH_UNORDERED_NOID:Ljava/util/Set;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/Collectors.<init>:()V")
        }

        #[java_method(name = "duplicateKeyException", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/IllegalStateException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn duplicateKeyException(k: Object, u: Object, v: Object) -> Result<IllegalStateException> {
            panic!("stub: java/util/stream/Collectors.duplicateKeyException:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/IllegalStateException;")
        }

        #[java_method(name = "uniqKeysMapMerger", descriptor = "()Ljava/util/function/BinaryOperator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;M::Ljava/util/Map<TK;TV;>;>()Ljava/util/function/BinaryOperator<TM;>;")]
        pub fn uniqKeysMapMerger() -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.uniqKeysMapMerger:()Ljava/util/function/BinaryOperator;")
        }

        #[java_method(name = "uniqKeysMapAccumulator", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/function/BiConsumer;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TV;>;)Ljava/util/function/BiConsumer<Ljava/util/Map<TK;TV;>;TT;>;")]
        pub fn uniqKeysMapAccumulator(keyMapper: Object, valueMapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.uniqKeysMapAccumulator:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/function/BiConsumer;")
        }

        #[java_method(name = "castingIdentity", descriptor = "()Ljava/util/function/Function;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<I:Ljava/lang/Object;R:Ljava/lang/Object;>()Ljava/util/function/Function<TI;TR;>;")]
        pub fn castingIdentity() -> Result<Object> {
            let __lam_28: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Collectors::lambda_castingIdentity_2(_la0) });
            Ok(Object::from_any(__lam_28))
        }

        #[java_method(name = "toCollection", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;C::Ljava/util/Collection<TT;>;>(Ljava/util/function/Supplier<TC;>;)Ljava/util/stream/Collector<TT;*TC;>;")]
        pub fn toCollection(collectionFactory: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toCollection:(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toList", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/List<TT;>;>;")]
        pub fn toList() -> Result<Object> {
            let __lam_46: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { ArrayList::<init>() });
            let __lam_50: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { List::add(_la0, _la1) });
            let __lam_51: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { Collectors::lambda_toList_4(_la0, _la1) });
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&Object::from_any(__lam_46)), Clone::clone(&Object::from_any(__lam_50)), Clone::clone(&Object::from_any(__lam_51)), Clone::clone(&Collectors::CH_ID()))?.clone()))
        }

        #[java_method(name = "toUnmodifiableList", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/List<TT;>;>;")]
        pub fn toUnmodifiableList() -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toUnmodifiableList:()Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toSet", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/Set<TT;>;>;")]
        pub fn toSet() -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toSet:()Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toUnmodifiableSet", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/Set<TT;>;>;")]
        pub fn toUnmodifiableSet() -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toUnmodifiableSet:()Ljava/util/stream/Collector;")
        }

        #[java_method(name = "joining", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;")]
        pub fn joining() -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.joining:()Ljava/util/stream/Collector;")
        }

        #[java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;)Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;")]
        // java: joining(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
        pub fn joining_seq(mut delimiter: Object) -> Result<Object> {
            let _t0: Object = Collectors::joining_seq_seq_seq(Clone::clone(&delimiter), Object::from_any(String::from("").clone()), Object::from_any(String::from("").clone()))?;
            Ok(_t0)
        }

        #[java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;")]
        // java: joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
        pub fn joining_seq_seq_seq(mut delimiter: Object, mut prefix: Object, mut suffix: Object) -> Result<Object> {
            let __lam_cap81_0 = suffix;
            let __lam_cap81_1 = prefix;
            let __lam_cap81_2 = delimiter;
            let __lam_81: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { Collectors::lambda_joining_11(__lam_cap81_0.clone(), __lam_cap81_1.clone(), __lam_cap81_2.clone()) });
            let __lam_84: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { StringJoiner::add(_la0, _la1) });
            let __lam_85: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { StringJoiner::merge(_la0, _la1) });
            let __lam_86: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { StringJoiner::toString(_la0) });
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::from_any(__lam_81)), Clone::clone(&Object::from_any(__lam_84)), Clone::clone(&Object::from_any(__lam_85)), Clone::clone(&Object::from_any(__lam_86)), Clone::clone(&Collectors::CH_NOID()))?.clone()))
        }

        #[java_method(name = "mapMerger", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/function/BinaryOperator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;M::Ljava/util/Map<TK;TV;>;>(Ljava/util/function/BinaryOperator<TV;>;)Ljava/util/function/BinaryOperator<TM;>;")]
        pub fn mapMerger(mut mergeFunction: Object) -> Result<Object> {
            let __lam_cap87_0 = mergeFunction;
            let __lam_87: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { Collectors::lambda_mapMerger_12(__lam_cap87_0.clone(), _la0, _la1) });
            Ok(Object::from_any(__lam_87))
        }

        #[java_method(name = "mapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;Ljava/util/stream/Collector<-TU;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;")]
        pub fn mapping(mapper: Object, downstream: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.mapping:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "flatMapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+Ljava/util/stream/Stream<+TU;>;>;Ljava/util/stream/Collector<-TU;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;")]
        pub fn flatMapping(mapper: Object, downstream: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.flatMapping:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "filtering", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;Ljava/util/stream/Collector<-TT;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;")]
        pub fn filtering(predicate: Object, downstream: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.filtering:(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "collectingAndThen", descriptor = "(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;RR:Ljava/lang/Object;>(Ljava/util/stream/Collector<TT;TA;TR;>;Ljava/util/function/Function<TR;TRR;>;)Ljava/util/stream/Collector<TT;TA;TRR;>;")]
        pub fn collectingAndThen(downstream: Object, finisher: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.collectingAndThen:(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "counting", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/lang/Long;>;")]
        pub fn counting() -> Result<Object> {
            let __lam_154: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i64>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<i64> { Collectors::lambda_counting_17(_la0) });
            let _t0: Object = Collectors::summingLong(Clone::clone(&Object::from_any(__lam_154)))?;
            Ok(_t0)
        }

        #[java_method(name = "minBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;")]
        pub fn minBy(comparator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.minBy:(Ljava/util/Comparator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "maxBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;")]
        pub fn maxBy(comparator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.maxBy:(Ljava/util/Comparator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "summingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Integer;>;")]
        pub fn summingInt(mut mapper: Object) -> Result<Object> {
            let __lam_175: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { Collectors::lambda_summingInt_18() });
            let __lam_cap176_0 = mapper;
            let __lam_176: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { Collectors::lambda_summingInt_19(__lam_cap176_0.clone(), _la0, _la1) });
            let __lam_179: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { Collectors::lambda_summingInt_20(_la0, _la1) });
            let __lam_180: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Collectors::lambda_summingInt_21(_la0) });
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::from_any(__lam_175)), Clone::clone(&Object::from_any(__lam_176)), Clone::clone(&Object::from_any(__lam_179)), Clone::clone(&Object::from_any(__lam_180)), Clone::clone(&Collectors::CH_NOID()))?.clone()))
        }

        #[java_method(name = "summingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Long;>;")]
        pub fn summingLong(mut mapper: Object) -> Result<Object> {
            let __lam_181: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { Collectors::lambda_summingLong_22() });
            let __lam_cap182_0 = mapper;
            let __lam_182: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { Collectors::lambda_summingLong_23(__lam_cap182_0.clone(), _la0, _la1) });
            let __lam_185: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { Collectors::lambda_summingLong_24(_la0, _la1) });
            let __lam_186: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Collectors::lambda_summingLong_25(_la0) });
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::from_any(__lam_181)), Clone::clone(&Object::from_any(__lam_182)), Clone::clone(&Object::from_any(__lam_185)), Clone::clone(&Object::from_any(__lam_186)), Clone::clone(&Collectors::CH_NOID()))?.clone()))
        }

        #[java_method(name = "summingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;")]
        pub fn summingDouble(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.summingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "sumWithCompensation", descriptor = "([DD)[D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sumWithCompensation(intermediateSum: Rc<RefCell<Vec<f64>>>, value: f64) -> Result<Rc<RefCell<Vec<f64>>>> {
            panic!("stub: java/util/stream/Collectors.sumWithCompensation:([DD)[D")
        }

        #[java_method(name = "computeFinalSum", descriptor = "([D)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeFinalSum(summands: Rc<RefCell<Vec<f64>>>) -> Result<f64> {
            panic!("stub: java/util/stream/Collectors.computeFinalSum:([D)D")
        }

        #[java_method(name = "averagingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;")]
        pub fn averagingInt(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.averagingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "averagingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;")]
        pub fn averagingLong(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.averagingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "averagingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;")]
        pub fn averagingDouble(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.averagingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/stream/Collector<TT;*TT;>;")]
        pub fn reducing_obj_binary(identity: Object, op: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.reducing:(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "boxSupplier", descriptor = "(Ljava/lang/Object;)Ljava/util/function/Supplier;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/function/Supplier<[TT;>;")]
        pub fn boxSupplier(identity: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.boxSupplier:(Ljava/lang/Object;)Ljava/util/function/Supplier;")
        }

        #[java_method(name = "reducing", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;")]
        pub fn reducing_binary(op: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.reducing:(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>(TU;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*TU;>;")]
        pub fn reducing_obj_functi_binary(identity: Object, mapper: Object, op: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.reducing:(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;Ljava/util/List<TT;>;>;>;")]
        // java: groupingBy(Ljava/util/function/Function;)Ljava/util/stream/Collector;
        pub fn groupingBy_functi(mut classifier: Object) -> Result<Object> {
            let _t0: Object = Collectors::toList()?;
            let _t1: Object = Collectors::groupingBy_functi_collec(Clone::clone(&classifier), Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TD;>;>;")]
        // java: groupingBy(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
        pub fn groupingBy_functi_collec(mut classifier: Object, mut downstream: Object) -> Result<Object> {
            let __lam_244: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { HashMap::<init>() });
            let _t0: Object = Collectors::groupingBy_functi_suppli_collec(Clone::clone(&classifier), Clone::clone(&Object::from_any(__lam_244)), Clone::clone(&downstream))?;
            Ok(_t0)
        }

        #[java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;D:Ljava/lang/Object;A:Ljava/lang/Object;M::Ljava/util/Map<TK;TD;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Supplier<TM;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*TM;>;")]
        // java: groupingBy(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
        pub fn groupingBy_functi_suppli_collec(mut classifier: Object, mut mapFactory: Object, mut downstream: Object) -> Result<Object> {
            let _vdispatch0: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.supplier()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.supplier()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut downstreamSupplier: Object = _vdispatch0;
            let _vdispatch1: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.accumulator()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.accumulator()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut downstreamAccumulator = (_vdispatch1).downcast::<BiConsumer<Object, Object>>();
            let __lam_cap248_0 = downstreamAccumulator;
            let __lam_cap248_1 = downstreamSupplier;
            let __lam_cap248_2 = classifier;
            let __lam_248: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { Collectors::lambda_groupingBy_53(__lam_cap248_0.clone(), __lam_cap248_1.clone(), __lam_cap248_2.clone(), _la0, _la1) });
            let mut accumulator = Object::from_any(__lam_248);
            let _vdispatch2: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.combiner()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.combiner()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _t3: Object = Collectors::mapMerger(Clone::clone(&_vdispatch2))?;
            let mut merger: Object = _t3;
            let mut mangledFactory: Object = mapFactory;
            let _vdispatch4: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.characteristics()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch5: bool = if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<Object>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(__f) = _vdispatch4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else { Default::default() };
            if _vdispatch5 {
                return Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&mangledFactory), Object::from_any(accumulator.clone()), Clone::clone(&merger), Clone::clone(&Collectors::CH_ID()))?.clone()));
            }
            let _vdispatch6: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.finisher()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.finisher()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut downstreamFinisher: Object = _vdispatch6;
            let __lam_cap254_0 = downstreamFinisher;
            let __lam_254: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Collectors::lambda_groupingBy_55(__lam_cap254_0.clone(), _la0) });
            let mut finisher = Object::from_any(__lam_254);
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&mangledFactory), Object::from_any(accumulator.clone()), Clone::clone(&merger), Clone::clone(&finisher), Clone::clone(&Collectors::CH_NOID()))?.clone()))
        }

        #[java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;Ljava/util/List<TT;>;>;>;")]
        pub fn groupingByConcurrent_functi(classifier: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TD;>;>;")]
        pub fn groupingByConcurrent_functi_collec(classifier: Object, downstream: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;M::Ljava/util/concurrent/ConcurrentMap<TK;TD;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Supplier<TM;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*TM;>;")]
        pub fn groupingByConcurrent_functi_suppli_collec(classifier: Object, mapFactory: Object, downstream: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<Ljava/lang/Boolean;Ljava/util/List<TT;>;>;>;")]
        // java: partitioningBy(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;
        pub fn partitioningBy_predic(mut predicate: Object) -> Result<Object> {
            let _t0: Object = Collectors::toList()?;
            let _t1: Object = Collectors::partitioningBy_predic_collec(Clone::clone(&predicate), Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;D:Ljava/lang/Object;A:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<Ljava/lang/Boolean;TD;>;>;")]
        // java: partitioningBy(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
        pub fn partitioningBy_predic_collec(mut predicate: Object, mut downstream: Object) -> Result<Object> {
            let _vdispatch0: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.accumulator()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.accumulator()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut downstreamAccumulator = (_vdispatch0).downcast::<BiConsumer<Object, Object>>();
            let __lam_cap276_0 = predicate;
            let __lam_cap276_1 = downstreamAccumulator;
            let __lam_276: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { Collectors::lambda_partitioningBy_62(__lam_cap276_0.clone(), __lam_cap276_1.clone(), _la0, _la1) });
            let mut accumulator = Object::from_any(__lam_276);
            let _vdispatch1: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.combiner()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.combiner()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut op: Object = _vdispatch1;
            let __lam_cap279_0 = op;
            let __lam_279: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { Collectors::lambda_partitioningBy_63(__lam_cap279_0.clone(), _la0, _la1) });
            let mut merger = Object::from_any(__lam_279);
            let __lam_cap280_0 = downstream;
            let __lam_280: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { Collectors::lambda_partitioningBy_64(__lam_cap280_0.clone()) });
            let mut supplier = Object::from_any(__lam_280);
            let _vdispatch2: Object = if let Some(_d) = downstream.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.characteristics()? } else if let Some(_d) = downstream.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = downstream.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch3: bool = if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<HashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch2.0.as_any().downcast_ref::<Object>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(__f) = _vdispatch2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else { Default::default() };
            if _vdispatch3 {
                return Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&supplier), Object::from_any(accumulator.clone()), Clone::clone(&merger), Clone::clone(&Collectors::CH_ID()))?.clone()));
            }
            let __lam_cap283_0 = downstream;
            let __lam_283: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Collectors::lambda_partitioningBy_65(__lam_cap283_0.clone(), _la0) });
            let mut finisher = Object::from_any(__lam_283);
            Ok(Object::from_any(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&supplier), Object::from_any(accumulator.clone()), Clone::clone(&merger), Clone::clone(&finisher), Clone::clone(&Collectors::CH_NOID()))?.clone()))
        }

        #[java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;")]
        pub fn toMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;")]
        pub fn toUnmodifiableMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toUnmodifiableMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;")]
        pub fn toMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;")]
        pub fn toUnmodifiableMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toUnmodifiableMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;M::Ljava/util/Map<TK;TU;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;Ljava/util/function/Supplier<TM;>;)Ljava/util/stream/Collector<TT;*TM;>;")]
        pub fn toMap_functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>;")]
        pub fn toConcurrentMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>;")]
        pub fn toConcurrentMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;M::Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;Ljava/util/function/Supplier<TM;>;)Ljava/util/stream/Collector<TT;*TM;>;")]
        pub fn toConcurrentMap_functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "summarizingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/IntSummaryStatistics;>;")]
        pub fn summarizingInt(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.summarizingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "summarizingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/LongSummaryStatistics;>;")]
        pub fn summarizingLong(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.summarizingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "summarizingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/DoubleSummaryStatistics;>;")]
        pub fn summarizingDouble(mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.summarizingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "teeing", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;R1:Ljava/lang/Object;R2:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;*TR1;>;Ljava/util/stream/Collector<-TT;*TR2;>;Ljava/util/function/BiFunction<-TR1;-TR2;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;")]
        pub fn teeing(downstream1: Object, downstream2: Object, merger: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.teeing:(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;")
        }

        #[java_method(name = "teeing0", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A1:Ljava/lang/Object;A2:Ljava/lang/Object;R1:Ljava/lang/Object;R2:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;TA1;TR1;>;Ljava/util/stream/Collector<-TT;TA2;TR2;>;Ljava/util/function/BiFunction<-TR1;-TR2;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;")]
        pub fn teeing0(downstream1: Object, downstream2: Object, merger: Object) -> Result<Object> {
            panic!("stub: java/util/stream/Collectors.teeing0:(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;")
        }
    }
}
