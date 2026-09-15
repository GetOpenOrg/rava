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
    binary_name       = "java/util/stream/Stream",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/stream/BaseStream",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/stream/BaseStream<TT;Ljava/util/stream/Stream<TT;>;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Stream.java",
    inner_classes     = "java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Taking:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Dropping:24;java/util/stream/Streams$StreamBuilderImpl:java/util/stream/Streams:StreamBuilderImpl:24;java/util/stream/Stream$1:::0;java/util/stream/Stream$2:::0;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:java/util/stream/StreamSpliterators:InfiniteSupplyingSpliterator:1032;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfRef:java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:OfRef:24;java/util/stream/Streams$ConcatSpliterator:java/util/stream/Streams:ConcatSpliterator:1032;java/util/stream/Streams$ConcatSpliterator$OfRef:java/util/stream/Streams$ConcatSpliterator:OfRef:8;java/util/stream/SpinedBuffer$OfDouble:java/util/stream/SpinedBuffer:OfDouble:8;java/util/Spliterator$OfDouble:java/util/Spliterator:OfDouble:1545;java/util/stream/SpinedBuffer$OfLong:java/util/stream/SpinedBuffer:OfLong:8;java/util/Spliterator$OfLong:java/util/Spliterator:OfLong:1545;java/util/stream/SpinedBuffer$OfInt:java/util/stream/SpinedBuffer:OfInt:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/stream/Stream$Builder:java/util/stream/Stream:Builder:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Stream<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Stream<T> {
    #[cfg_attr(any(), java_method(name = "filter", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn filter(&self, arg0: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.filter:(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "map", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TR;>;)Ljava/util/stream/Stream<TR;>;"))]
    pub fn map(&self, arg0: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.map:(Ljava/util/function/Function;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "mapToInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/IntStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/stream/IntStream;"))]
    pub fn mapToInt(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapToInt:(Ljava/util/function/ToIntFunction;)Ljava/util/stream/IntStream;")
    }

    #[cfg_attr(any(), java_method(name = "mapToLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/stream/LongStream;"))]
    pub fn mapToLong(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapToLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;")
    }

    #[cfg_attr(any(), java_method(name = "mapToDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/stream/DoubleStream;"))]
    pub fn mapToDouble(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapToDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;")
    }

    #[cfg_attr(any(), java_method(name = "flatMap", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+Ljava/util/stream/Stream<+TR;>;>;)Ljava/util/stream/Stream<TR;>;"))]
    pub fn flatMap(&self, arg0: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.flatMap:(Ljava/util/function/Function;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "flatMapToInt", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/IntStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TT;+Ljava/util/stream/IntStream;>;)Ljava/util/stream/IntStream;"))]
    pub fn flatMapToInt(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.flatMapToInt:(Ljava/util/function/Function;)Ljava/util/stream/IntStream;")
    }

    #[cfg_attr(any(), java_method(name = "flatMapToLong", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/LongStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TT;+Ljava/util/stream/LongStream;>;)Ljava/util/stream/LongStream;"))]
    pub fn flatMapToLong(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.flatMapToLong:(Ljava/util/function/Function;)Ljava/util/stream/LongStream;")
    }

    #[cfg_attr(any(), java_method(name = "flatMapToDouble", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TT;+Ljava/util/stream/DoubleStream;>;)Ljava/util/stream/DoubleStream;"))]
    pub fn flatMapToDouble(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.flatMapToDouble:(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;")
    }

    #[cfg_attr(any(), java_method(name = "mapMulti", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/BiConsumer<-TT;-Ljava/util/function/Consumer<TR;>;>;)Ljava/util/stream/Stream<TR;>;"))]
    pub fn mapMulti(&self, mapper: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.mapMulti:(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToInt", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TT;-Ljava/util/function/IntConsumer;>;)Ljava/util/stream/IntStream;"))]
    pub fn mapMultiToInt(&self, mapper: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapMultiToInt:(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;")
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToLong", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TT;-Ljava/util/function/LongConsumer;>;)Ljava/util/stream/LongStream;"))]
    pub fn mapMultiToLong(&self, mapper: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapMultiToLong:(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;")
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToDouble", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TT;-Ljava/util/function/DoubleConsumer;>;)Ljava/util/stream/DoubleStream;"))]
    pub fn mapMultiToDouble(&self, mapper: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.mapMultiToDouble:(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;")
    }

    #[cfg_attr(any(), java_method(name = "distinct", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TT;>;"))]
    pub fn distinct(&self) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.distinct:()Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "sorted", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TT;>;"))]
    pub fn sorted(&self) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.sorted:()Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "sorted", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn sorted_compar(&self, arg0: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.sorted:(Ljava/util/Comparator;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "peek", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn peek(&self, arg0: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.peek:(Ljava/util/function/Consumer;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "limit", descriptor = "(J)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(J)Ljava/util/stream/Stream<TT;>;"))]
    pub fn limit(&self, arg0: i64) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.limit:(J)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "skip", descriptor = "(J)Ljava/util/stream/Stream;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(J)Ljava/util/stream/Stream<TT;>;"))]
    pub fn skip(&self, arg0: i64) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.skip:(J)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "takeWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn takeWhile(&self, predicate: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.takeWhile:(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "dropWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn dropWhile(&self, predicate: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.dropWhile:(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V"))]
    pub fn forEach(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/stream/Stream.forEach:(Ljava/util/function/Consumer;)V")
    }

    #[cfg_attr(any(), java_method(name = "forEachOrdered", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V"))]
    pub fn forEachOrdered(&self, arg0: Object) -> Result<()> {
        panic!("stub: java/util/stream/Stream.forEachOrdered:(Ljava/util/function/Consumer;)V")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/stream/Stream.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<A:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TA;>;)[TA;"))]
    pub fn toArray_intfun(&self, arg0: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/stream/Stream.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT;Ljava/util/function/BinaryOperator<TT;>;)TT;"))]
    pub fn reduce_obj_binary(&self, arg0: Object, arg1: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.reduce:(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "reduce", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/Optional;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/Optional<TT;>;"))]
    pub fn reduce_binary(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.reduce:(Ljava/util/function/BinaryOperator;)Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(TU;Ljava/util/function/BiFunction<TU;-TT;TU;>;Ljava/util/function/BinaryOperator<TU;>;)TU;"))]
    pub fn reduce_obj_bifunc_binary(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.reduce:(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "collect", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/BiConsumer<TR;-TT;>;Ljava/util/function/BiConsumer<TR;TR;>;)TR;"))]
    pub fn collect_suppli_bicons_bicons(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.collect:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "collect", descriptor = "(Ljava/util/stream/Collector;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;A:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;TA;TR;>;)TR;"))]
    pub fn collect_collec(&self, arg0: Collector<Object, Object, Object>) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.collect:(Ljava/util/stream/Collector;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toList", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<TT;>;"))]
    pub fn toList(&self) -> Result<List<Object>> {
        panic!("stub: java/util/stream/Stream.toList:()Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TT;>;)Ljava/util/Optional<TT;>;"))]
    pub fn min(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.min:(Ljava/util/Comparator;)Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TT;>;)Ljava/util/Optional<TT;>;"))]
    pub fn max(&self, arg0: Object) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.max:(Ljava/util/Comparator;)Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "count", descriptor = "()J", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn count(&self) -> Result<i64> {
        panic!("stub: java/util/stream/Stream.count:()J")
    }

    #[cfg_attr(any(), java_method(name = "anyMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Z"))]
    pub fn anyMatch(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/stream/Stream.anyMatch:(Ljava/util/function/Predicate;)Z")
    }

    #[cfg_attr(any(), java_method(name = "allMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Z"))]
    pub fn allMatch(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/stream/Stream.allMatch:(Ljava/util/function/Predicate;)Z")
    }

    #[cfg_attr(any(), java_method(name = "noneMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Z"))]
    pub fn noneMatch(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/stream/Stream.noneMatch:(Ljava/util/function/Predicate;)Z")
    }

    #[cfg_attr(any(), java_method(name = "findFirst", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Optional<TT;>;"))]
    pub fn findFirst(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.findFirst:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "findAny", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Optional<TT;>;"))]
    pub fn findAny(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Stream.findAny:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "builder", descriptor = "()Ljava/util/stream/Stream$Builder;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Stream$Builder<TT;>;"))]
    pub fn builder() -> Result<Object> {
        panic!("stub: java/util/stream/Stream.builder:()Ljava/util/stream/Stream$Builder;")
    }

    #[cfg_attr(any(), java_method(name = "empty", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/Stream<TT;>;"))]
    pub fn empty() -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.empty:()Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn of_obj(t: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.of:(Ljava/lang/Object;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "ofNullable", descriptor = "(Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn ofNullable(t: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.ofNullable:(Ljava/lang/Object;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "([Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn of_arr_obj(values: Rc<RefCell<Vec<Object>>>) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.of:([Ljava/lang/Object;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "iterate", descriptor = "(Ljava/lang/Object;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/UnaryOperator<TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn iterate_obj_unaryo(seed: Object, f: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.iterate:(Ljava/lang/Object;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "iterate", descriptor = "(Ljava/lang/Object;Ljava/util/function/Predicate;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;Ljava/util/function/Predicate<-TT;>;Ljava/util/function/UnaryOperator<TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn iterate_obj_predic_unaryo(seed: Object, hasNext: Object, next: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.iterate:(Ljava/lang/Object;Ljava/util/function/Predicate;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "generate", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Supplier<+TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn generate(s: Object) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.generate:(Ljava/util/function/Supplier;)Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "concat", descriptor = "(Ljava/util/stream/Stream;Ljava/util/stream/Stream;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/Stream<+TT;>;Ljava/util/stream/Stream<+TT;>;)Ljava/util/stream/Stream<TT;>;"))]
    pub fn concat(a: Stream<Object>, b: Stream<Object>) -> Result<Stream<Object>> {
        panic!("stub: java/util/stream/Stream.concat:(Ljava/util/stream/Stream;Ljava/util/stream/Stream;)Ljava/util/stream/Stream;")
    }
}
