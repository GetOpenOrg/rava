#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/stream/Collectors",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Collectors.java",
    inner_classes     = "java/util/stream/Collectors$CollectorImpl:java/util/stream/Collectors:CollectorImpl:24;java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/util/stream/Collectors$1PairBox::PairBox:0;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/stream/Collectors$Partition:java/util/stream/Collectors:Partition:26;java/util/stream/Collectors$1OptionalBox::OptionalBox:0;java/util/stream/Collectors$Partition$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/lang/Object;java/util/stream/Collectors",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Collectors;

impl Collectors {
    #[cfg_attr(any(), java_field(name = "CH_CONCURRENT_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_CONCURRENT_ID:Ljava/util/Set;
    pub fn CH_CONCURRENT_ID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_CONCURRENT_ID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_field(name = "CH_CONCURRENT_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_CONCURRENT_NOID:Ljava/util/Set;
    pub fn CH_CONCURRENT_NOID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_CONCURRENT_NOID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_field(name = "CH_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_ID:Ljava/util/Set;
    pub fn CH_ID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_ID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_field(name = "CH_UNORDERED_ID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_UNORDERED_ID:Ljava/util/Set;
    pub fn CH_UNORDERED_ID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_UNORDERED_ID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_field(name = "CH_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_NOID:Ljava/util/Set;
    pub fn CH_NOID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_NOID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_field(name = "CH_UNORDERED_NOID", descriptor = "Ljava/util/Set;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    // static field: CH_UNORDERED_NOID:Ljava/util/Set;
    pub fn CH_UNORDERED_NOID() -> Set<Object> {
        panic!("stub: java/util/stream/Collectors.CH_UNORDERED_NOID:Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/util/stream/Collectors.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "duplicateKeyException", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/IllegalStateException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn duplicateKeyException(k: Object, u: Object, v: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Collectors.duplicateKeyException:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/IllegalStateException;")
    }

    #[cfg_attr(any(), java_method(name = "uniqKeysMapMerger", descriptor = "()Ljava/util/function/BinaryOperator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;M::Ljava/util/Map<TK;TV;>;>()Ljava/util/function/BinaryOperator<TM;>;"))]
    pub fn uniqKeysMapMerger() -> Result<Object> {
        panic!("stub: java/util/stream/Collectors.uniqKeysMapMerger:()Ljava/util/function/BinaryOperator;")
    }

    #[cfg_attr(any(), java_method(name = "uniqKeysMapAccumulator", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/function/BiConsumer;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TV;>;)Ljava/util/function/BiConsumer<Ljava/util/Map<TK;TV;>;TT;>;"))]
    pub fn uniqKeysMapAccumulator(keyMapper: Object, valueMapper: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Collectors.uniqKeysMapAccumulator:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/function/BiConsumer;")
    }

    #[cfg_attr(any(), java_method(name = "castingIdentity", descriptor = "()Ljava/util/function/Function;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<I:Ljava/lang/Object;R:Ljava/lang/Object;>()Ljava/util/function/Function<TI;TR;>;"))]
    pub fn castingIdentity() -> Result<Object> {
        /* TODO: invokedynamic 28 */
        Ok(Object::default())
    }

    #[cfg_attr(any(), java_method(name = "toCollection", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;C::Ljava/util/Collection<TT;>;>(Ljava/util/function/Supplier<TC;>;)Ljava/util/stream/Collector<TT;*TC;>;"))]
    pub fn toCollection(collectionFactory: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toCollection:(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toList", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/List<TT;>;>;"))]
    pub fn toList() -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 46 */
        /* TODO: invokedynamic 50 */
        /* TODO: invokedynamic 51 */
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Collectors::CH_ID()))?))
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableList", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/List<TT;>;>;"))]
    pub fn toUnmodifiableList() -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toUnmodifiableList:()Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toSet", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/Set<TT;>;>;"))]
    pub fn toSet() -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toSet:()Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableSet", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/util/Set<TT;>;>;"))]
    pub fn toUnmodifiableSet() -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toUnmodifiableSet:()Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;"))]
    pub fn joining() -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.joining:()Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;)Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;"))]
    // java: joining(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
    pub fn joining_seq(mut delimiter: Object) -> Result<Collector<Object, Object, Object>> {
        let _t0: Collector<Object, Object, Object> = Collectors::joining_seq_seq_seq(Clone::clone(&delimiter), Object::from_any(String::from("").clone()), Object::from_any(String::from("").clone()))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector<Ljava/lang/CharSequence;*Ljava/lang/String;>;"))]
    // java: joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
    pub fn joining_seq_seq_seq(mut delimiter: Object, mut prefix: Object, mut suffix: Object) -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 81 */
        /* TODO: invokedynamic 84 */
        /* TODO: invokedynamic 85 */
        /* TODO: invokedynamic 86 */
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Collectors::CH_NOID()))?))
    }

    #[cfg_attr(any(), java_method(name = "mapMerger", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/function/BinaryOperator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;M::Ljava/util/Map<TK;TV;>;>(Ljava/util/function/BinaryOperator<TV;>;)Ljava/util/function/BinaryOperator<TM;>;"))]
    pub fn mapMerger(mut mergeFunction: Object) -> Result<Object> {
        /* TODO: invokedynamic 87 */
        Ok(Object::default())
    }

    #[cfg_attr(any(), java_method(name = "mapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;Ljava/util/stream/Collector<-TU;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;"))]
    pub fn mapping(mapper: Object, downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.mapping:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "flatMapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+Ljava/util/stream/Stream<+TU;>;>;Ljava/util/stream/Collector<-TU;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;"))]
    pub fn flatMapping(mapper: Object, downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.flatMapping:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "filtering", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;Ljava/util/stream/Collector<-TT;TA;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;"))]
    pub fn filtering(predicate: Object, downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.filtering:(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "collectingAndThen", descriptor = "(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;RR:Ljava/lang/Object;>(Ljava/util/stream/Collector<TT;TA;TR;>;Ljava/util/function/Function<TR;TRR;>;)Ljava/util/stream/Collector<TT;TA;TRR;>;"))]
    pub fn collectingAndThen(downstream: Collector<Object, Object, Object>, finisher: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.collectingAndThen:(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "counting", descriptor = "()Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Collector<TT;*Ljava/lang/Long;>;"))]
    pub fn counting() -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 154 */
        let _t0: Collector<Object, Object, Object> = Collectors::summingLong(Clone::clone(&Object::default()))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "minBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;"))]
    pub fn minBy(comparator: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.minBy:(Ljava/util/Comparator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "maxBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;"))]
    pub fn maxBy(comparator: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.maxBy:(Ljava/util/Comparator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "summingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Integer;>;"))]
    pub fn summingInt(mut mapper: Object) -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 175 */
        /* TODO: invokedynamic 176 */
        /* TODO: invokedynamic 179 */
        /* TODO: invokedynamic 180 */
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Collectors::CH_NOID()))?))
    }

    #[cfg_attr(any(), java_method(name = "summingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Long;>;"))]
    pub fn summingLong(mut mapper: Object) -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 181 */
        /* TODO: invokedynamic 182 */
        /* TODO: invokedynamic 185 */
        /* TODO: invokedynamic 186 */
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Object::default()), Clone::clone(&Collectors::CH_NOID()))?))
    }

    #[cfg_attr(any(), java_method(name = "summingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;"))]
    pub fn summingDouble(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.summingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "sumWithCompensation", descriptor = "([DD)[D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn sumWithCompensation(intermediateSum: Rc<RefCell<Vec<f64>>>, value: f64) -> Result<Rc<RefCell<Vec<f64>>>> {
        panic!("stub: java/util/stream/Collectors.sumWithCompensation:([DD)[D")
    }

    #[cfg_attr(any(), java_method(name = "computeFinalSum", descriptor = "([D)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn computeFinalSum(summands: Rc<RefCell<Vec<f64>>>) -> Result<f64> {
        panic!("stub: java/util/stream/Collectors.computeFinalSum:([D)D")
    }

    #[cfg_attr(any(), java_method(name = "averagingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;"))]
    pub fn averagingInt(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.averagingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "averagingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;"))]
    pub fn averagingLong(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.averagingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "averagingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/lang/Double;>;"))]
    pub fn averagingDouble(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.averagingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/stream/Collector<TT;*TT;>;"))]
    pub fn reducing_obj_binary(identity: Object, op: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.reducing:(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "boxSupplier", descriptor = "(Ljava/lang/Object;)Ljava/util/function/Supplier;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/function/Supplier<[TT;>;"))]
    pub fn boxSupplier(identity: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Collectors.boxSupplier:(Ljava/lang/Object;)Ljava/util/function/Supplier;")
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Optional<TT;>;>;"))]
    pub fn reducing_binary(op: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.reducing:(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>(TU;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*TU;>;"))]
    pub fn reducing_obj_functi_binary(identity: Object, mapper: Object, op: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.reducing:(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;Ljava/util/List<TT;>;>;>;"))]
    // java: groupingBy(Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn groupingBy_functi(mut classifier: Object) -> Result<Collector<Object, Object, Object>> {
        let _t0: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t1: Collector<Object, Object, Object> = Collectors::groupingBy_functi_collec(Clone::clone(&classifier), Clone::clone(&_t0))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TD;>;>;"))]
    // java: groupingBy(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingBy_functi_collec(mut classifier: Object, mut downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        /* TODO: invokedynamic 244 */
        let _t0: Collector<Object, Object, Object> = Collectors::groupingBy_functi_suppli_collec(Clone::clone(&classifier), Clone::clone(&Object::default()), Clone::clone(&downstream))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;D:Ljava/lang/Object;A:Ljava/lang/Object;M::Ljava/util/Map<TK;TD;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Supplier<TM;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*TM;>;"))]
    // java: groupingBy(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingBy_functi_suppli_collec(mut classifier: Object, mut mapFactory: Object, mut downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        let _t0 = downstream.supplier()?;
        let mut downstreamSupplier: Object = _t0;
        let _t1 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t1;
        /* TODO: invokedynamic 248 */
        let mut accumulator = Object::default();
        let _t2 = downstream.combiner()?;
        let _t3: Object = Collectors::mapMerger(Clone::clone(&_t2))?;
        let mut merger: Object = _t3;
        let mut mangledFactory: Object = mapFactory;
        let _t4 = downstream.characteristics()?;
        let _t5 = _t4.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))?;
        if _t5 {
            return Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&mangledFactory), Clone::clone(&accumulator), Clone::clone(&merger), Clone::clone(&Collectors::CH_ID()))?));
        }
        let _t6 = downstream.finisher()?;
        let mut downstreamFinisher: Object = _t6;
        /* TODO: invokedynamic 254 */
        let mut finisher = Object::default();
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&mangledFactory), Clone::clone(&accumulator), Clone::clone(&merger), Clone::clone(&finisher), Clone::clone(&Collectors::CH_NOID()))?))
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;Ljava/util/List<TT;>;>;>;"))]
    pub fn groupingByConcurrent_functi(classifier: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TD;>;>;"))]
    pub fn groupingByConcurrent_functi_collec(classifier: Object, downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;A:Ljava/lang/Object;D:Ljava/lang/Object;M::Ljava/util/concurrent/ConcurrentMap<TK;TD;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Supplier<TM;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*TM;>;"))]
    pub fn groupingByConcurrent_functi_suppli_collec(classifier: Object, mapFactory: Object, downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.groupingByConcurrent:(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<Ljava/lang/Boolean;Ljava/util/List<TT;>;>;>;"))]
    // java: partitioningBy(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;
    pub fn partitioningBy_predic(mut predicate: Object) -> Result<Collector<Object, Object, Object>> {
        let _t0: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t1: Collector<Object, Object, Object> = Collectors::partitioningBy_predic_collec(Clone::clone(&predicate), Clone::clone(&_t0))?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;D:Ljava/lang/Object;A:Ljava/lang/Object;>(Ljava/util/function/Predicate<-TT;>;Ljava/util/stream/Collector<-TT;TA;TD;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<Ljava/lang/Boolean;TD;>;>;"))]
    // java: partitioningBy(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn partitioningBy_predic_collec(mut predicate: Object, mut downstream: Collector<Object, Object, Object>) -> Result<Collector<Object, Object, Object>> {
        let _t0 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t0;
        /* TODO: invokedynamic 276 */
        let mut accumulator = Object::default();
        let _t1 = downstream.combiner()?;
        let mut op: Object = _t1;
        /* TODO: invokedynamic 279 */
        let mut merger = Object::default();
        /* TODO: invokedynamic 280 */
        let mut supplier = Object::default();
        let _t2 = downstream.characteristics()?;
        let _t3 = _t2.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))?;
        if _t3 {
            return Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_set(Clone::clone(&supplier), Clone::clone(&accumulator), Clone::clone(&merger), Clone::clone(&Collectors::CH_ID()))?));
        }
        /* TODO: invokedynamic 283 */
        let mut finisher = Object::default();
        Ok(<_ as Into<Collector<Object, Object, Object>>>::into(Collectors_CollectorImpl::<Object, Object, Object>::new_suppli_bicons_binary_functi_set(Clone::clone(&supplier), Clone::clone(&accumulator), Clone::clone(&merger), Clone::clone(&finisher), Clone::clone(&Collectors::CH_NOID()))?))
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;"))]
    pub fn toMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;"))]
    pub fn toUnmodifiableMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toUnmodifiableMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;"))]
    pub fn toMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/Map<TK;TU;>;>;"))]
    pub fn toUnmodifiableMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toUnmodifiableMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;M::Ljava/util/Map<TK;TU;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;Ljava/util/function/Supplier<TM;>;)Ljava/util/stream/Collector<TT;*TM;>;"))]
    pub fn toMap_functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>;"))]
    pub fn toConcurrentMap_functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/Collector<TT;*Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>;"))]
    pub fn toConcurrentMap_functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;K:Ljava/lang/Object;U:Ljava/lang/Object;M::Ljava/util/concurrent/ConcurrentMap<TK;TU;>;>(Ljava/util/function/Function<-TT;+TK;>;Ljava/util/function/Function<-TT;+TU;>;Ljava/util/function/BinaryOperator<TU;>;Ljava/util/function/Supplier<TM;>;)Ljava/util/stream/Collector<TT;*TM;>;"))]
    pub fn toConcurrentMap_functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.toConcurrentMap:(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "summarizingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/IntSummaryStatistics;>;"))]
    pub fn summarizingInt(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.summarizingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "summarizingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/LongSummaryStatistics;>;"))]
    pub fn summarizingLong(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.summarizingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "summarizingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/Collector<TT;*Ljava/util/DoubleSummaryStatistics;>;"))]
    pub fn summarizingDouble(mapper: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.summarizingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "teeing", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;R1:Ljava/lang/Object;R2:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;*TR1;>;Ljava/util/stream/Collector<-TT;*TR2;>;Ljava/util/function/BiFunction<-TR1;-TR2;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;"))]
    pub fn teeing(downstream1: Collector<Object, Object, Object>, downstream2: Collector<Object, Object, Object>, merger: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.teeing:(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "teeing0", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A1:Ljava/lang/Object;A2:Ljava/lang/Object;R1:Ljava/lang/Object;R2:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;TA1;TR1;>;Ljava/util/stream/Collector<-TT;TA2;TR2;>;Ljava/util/function/BiFunction<-TR1;-TR2;TR;>;)Ljava/util/stream/Collector<TT;*TR;>;"))]
    pub fn teeing0(downstream1: Collector<Object, Object, Object>, downstream2: Collector<Object, Object, Object>, merger: Object) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collectors.teeing0:(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;")
    }
}
