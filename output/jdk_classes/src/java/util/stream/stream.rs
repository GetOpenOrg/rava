#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/stream/Stream",
    super_class = "java/lang/Object",
    interfaces  = "java/util/stream/BaseStream",
    access      = "public abstract",
    source      = "Stream.java",
))]
pub struct Stream<T>;

impl<T: Clone + 'static> Stream<T> {
    #[cfg_attr(any(), java_native(name = "filter", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn filter(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.filter")
    }

    #[cfg_attr(any(), java_native(name = "map", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn map(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.map")
    }

    #[cfg_attr(any(), java_native(name = "mapToInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/IntStream;", access = "public abstract"))]
    pub fn mapToInt(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.mapToInt")
    }

    #[cfg_attr(any(), java_native(name = "mapToLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;", access = "public abstract"))]
    pub fn mapToLong(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.mapToLong")
    }

    #[cfg_attr(any(), java_native(name = "mapToDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;", access = "public abstract"))]
    pub fn mapToDouble(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.mapToDouble")
    }

    #[cfg_attr(any(), java_native(name = "flatMap", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn flatMap(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.flatMap")
    }

    #[cfg_attr(any(), java_native(name = "flatMapToInt", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/IntStream;", access = "public abstract"))]
    pub fn flatMapToInt(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.flatMapToInt")
    }

    #[cfg_attr(any(), java_native(name = "flatMapToLong", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/LongStream;", access = "public abstract"))]
    pub fn flatMapToLong(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.flatMapToLong")
    }

    #[cfg_attr(any(), java_native(name = "flatMapToDouble", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;", access = "public abstract"))]
    pub fn flatMapToDouble(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.flatMapToDouble")
    }

    #[cfg_attr(any(), java_method(name = "mapMulti", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;", access = "public"))]
    pub fn mapMulti(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mapper)?;
        /* TODO: invokedynamic 7 */
        let _t1 = this.flatMap(mapper)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToInt", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;", access = "public"))]
    pub fn mapMultiToInt(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mapper)?;
        /* TODO: invokedynamic 17 */
        let _t1 = this.flatMapToInt(mapper)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToLong", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;", access = "public"))]
    pub fn mapMultiToLong(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mapper)?;
        /* TODO: invokedynamic 22 */
        let _t1 = this.flatMapToLong(mapper)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "mapMultiToDouble", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;", access = "public"))]
    pub fn mapMultiToDouble(&self, mapper: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mapper)?;
        /* TODO: invokedynamic 27 */
        let _t1 = this.flatMapToDouble(mapper)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_native(name = "distinct", descriptor = "()Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn distinct(&self) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.distinct")
    }

    #[cfg_attr(any(), java_native(name = "sorted", descriptor = "()Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn sorted(&self) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.sorted")
    }

    #[cfg_attr(any(), java_native(name = "sorted", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn sorted__compar(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.sorted")
    }

    #[cfg_attr(any(), java_native(name = "peek", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn peek(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.peek")
    }

    #[cfg_attr(any(), java_native(name = "limit", descriptor = "(J)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn limit(&self, arg0: i64) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.limit")
    }

    #[cfg_attr(any(), java_native(name = "skip", descriptor = "(J)Ljava/util/stream/Stream;", access = "public abstract"))]
    pub fn skip(&self, arg0: i64) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.skip")
    }

    #[cfg_attr(any(), java_method(name = "takeWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public"))]
    pub fn takeWhile(&self, predicate: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(predicate)?;
        let _t1 = this.spliterator()?;
        let _t2 = this.isParallel()?;
        let _t3: Object = StreamSupport::stream(WhileOps_UnorderedWhileSpliterator_OfRef_Taking::new(_t1, 1i32, predicate)?, _t2)?;
        /* TODO: invokedynamic 52 */
        let _t4 = _t3.onClose(this)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "dropWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public"))]
    pub fn dropWhile(&self, predicate: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(predicate)?;
        let _t1 = this.spliterator()?;
        let _t2 = this.isParallel()?;
        let _t3: Object = StreamSupport::stream(WhileOps_UnorderedWhileSpliterator_OfRef_Dropping::new(_t1, 1i32, predicate)?, _t2)?;
        /* TODO: invokedynamic 52 */
        let _t4 = _t3.onClose(this)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_native(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public abstract"))]
    pub fn forEach(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/stream/Stream.forEach")
    }

    #[cfg_attr(any(), java_native(name = "forEachOrdered", descriptor = "(Ljava/util/function/Consumer;)V", access = "public abstract"))]
    pub fn forEachOrdered(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/stream/Stream.forEachOrdered")
    }

    #[cfg_attr(any(), java_native(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public abstract"))]
    pub fn toArray(&self) -> Result<Vec<Object>> {
        todo!("abstract java/util/stream/Stream.toArray")
    }

    #[cfg_attr(any(), java_native(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public abstract"))]
    pub fn toArray__intfun(&self, arg0: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/stream/Stream.toArray")
    }

    #[cfg_attr(any(), java_native(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn reduce__obj_binary(&self, arg0: Object, arg1: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.reduce")
    }

    #[cfg_attr(any(), java_native(name = "reduce", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/Optional;", access = "public abstract"))]
    pub fn reduce__binary(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.reduce")
    }

    #[cfg_attr(any(), java_native(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn reduce__obj_bifunc_binary(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.reduce")
    }

    #[cfg_attr(any(), java_native(name = "collect", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn collect__suppli_bicons_bicons(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.collect")
    }

    #[cfg_attr(any(), java_native(name = "collect", descriptor = "(Ljava/util/stream/Collector;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn collect__collec(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.collect")
    }

    #[cfg_attr(any(), java_method(name = "toList", descriptor = "()Ljava/util/List;", access = "public"))]
    pub fn toList(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.toArray()?;
        let _t1: Object = Arrays::asList(&_t0)?;
        let _t2: Object = Collections::unmodifiableList(ArrayList::<_>::new()?)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "min", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public abstract"))]
    pub fn min(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.min")
    }

    #[cfg_attr(any(), java_native(name = "max", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public abstract"))]
    pub fn max(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.max")
    }

    #[cfg_attr(any(), java_native(name = "count", descriptor = "()J", access = "public abstract"))]
    pub fn count(&self) -> Result<i64> {
        todo!("abstract java/util/stream/Stream.count")
    }

    #[cfg_attr(any(), java_native(name = "anyMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public abstract"))]
    pub fn anyMatch(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/stream/Stream.anyMatch")
    }

    #[cfg_attr(any(), java_native(name = "allMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public abstract"))]
    pub fn allMatch(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/stream/Stream.allMatch")
    }

    #[cfg_attr(any(), java_native(name = "noneMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public abstract"))]
    pub fn noneMatch(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/stream/Stream.noneMatch")
    }

    #[cfg_attr(any(), java_native(name = "findFirst", descriptor = "()Ljava/util/Optional;", access = "public abstract"))]
    pub fn findFirst(&self) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.findFirst")
    }

    #[cfg_attr(any(), java_native(name = "findAny", descriptor = "()Ljava/util/Optional;", access = "public abstract"))]
    pub fn findAny(&self) -> Result<Object> {
        todo!("abstract java/util/stream/Stream.findAny")
    }

    #[cfg_attr(any(), java_method(name = "builder", descriptor = "()Ljava/util/stream/Stream$Builder;", access = "public static"))]
    pub fn builder() -> Result<Object> {
        Ok(Streams_StreamBuilderImpl::new()?)
    }

    #[cfg_attr(any(), java_method(name = "empty", descriptor = "()Ljava/util/stream/Stream;", access = "public static"))]
    pub fn empty() -> Result<Object> {
        let _t0: Object = Spliterators::emptySpliterator()?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public static"))]
    // java: of(Ljava/lang/Object;)Ljava/util/stream/Stream;
    pub fn of__obj(t: T) -> Result<Object> {
        let _t0: Object = StreamSupport::stream(Streams_StreamBuilderImpl::new(t)?, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "ofNullable", descriptor = "(Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public static"))]
    pub fn ofNullable(t: T) -> Result<Object> {
        let _t0: Object = Stream::empty()?;
        let _t1: Object = StreamSupport::stream(Streams_StreamBuilderImpl::new(t)?, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "([Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public static"))]
    // java: of([Ljava/lang/Object;)Ljava/util/stream/Stream;
    pub fn of__arr_obj(values: &[Object]) -> Result<Object> {
        let _t0: Object = Arrays::stream(&values)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "iterate", descriptor = "(Ljava/lang/Object;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;", access = "public static"))]
    // java: iterate(Ljava/lang/Object;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;
    pub fn iterate__obj_unaryo(seed: T, f: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(f)?;
        let mut spliterator: Stream_1 = Stream_1::new(9223372036854775807i64, 1040i32, f, seed)?;
        let _t1: Object = StreamSupport::stream(spliterator, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "iterate", descriptor = "(Ljava/lang/Object;Ljava/util/function/Predicate;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;", access = "public static"))]
    // java: iterate(Ljava/lang/Object;Ljava/util/function/Predicate;Ljava/util/function/UnaryOperator;)Ljava/util/stream/Stream;
    pub fn iterate__obj_predic_unaryo(seed: T, hasNext: Object, next: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(next)?;
        let _t1: Object = Objects::requireNonNull(hasNext)?;
        let mut spliterator: Stream_2 = Stream_2::new(9223372036854775807i64, 1040i32, next, seed, hasNext)?;
        let _t2: Object = StreamSupport::stream(spliterator, 0i32)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "generate", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/stream/Stream;", access = "public static"))]
    pub fn generate(s: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(s)?;
        let _t1: Object = StreamSupport::stream(StreamSpliterators_InfiniteSupplyingSpliterator_OfRef::new(9223372036854775807i64, s)?, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "concat", descriptor = "(Ljava/util/stream/Stream;Ljava/util/stream/Stream;)Ljava/util/stream/Stream;", access = "public static"))]
    pub fn concat(a: Object, b: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(a)?;
        let _t1: Object = Objects::requireNonNull(b)?;
        let _t2 = a.spliterator()?;
        let _t3 = b.spliterator()?;
        let mut split: Streams_ConcatSpliterator_OfRef = Streams_ConcatSpliterator_OfRef::new(_t2, _t3)?;
        let _t4 = a.isParallel()?;
        let _t5 = b.isParallel()?;
        let _t6: Object = StreamSupport::stream(_t4, _t5!=0i32)?;
        let mut stream: Object = _t6;
        let _t7: Object = Streams::composedClose(a, b)?;
        let _t8 = stream.onClose(_t7)?;
        Ok(_t8)
    }
}
