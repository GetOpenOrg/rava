#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/stream/Collectors",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Collectors.java",
))]
pub struct Collectors;

impl Collectors {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "duplicateKeyException", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/IllegalStateException;", access = "private static"))]
    pub fn duplicateKeyException(k: Object, u: Object, v: Object) -> Result<Object> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(3i32 as usize);
        _arr0[0i32 as usize] = k;
        _arr0[1i32 as usize] = u;
        _arr0[2i32 as usize] = v;
        let _t1: String = String::format(String::from("Duplicate key %s (attempted merging values %s and %s)"), &_arr0)?;
        Ok(IllegalStateException::new(_t1)?)
    }

    #[cfg_attr(any(), java_method(name = "uniqKeysMapMerger", descriptor = "()Ljava/util/function/BinaryOperator;", access = "private static"))]
    pub fn uniqKeysMapMerger() -> Result<Object> {
        /* TODO: invokedynamic 20 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "uniqKeysMapAccumulator", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/function/BiConsumer;", access = "private static"))]
    pub fn uniqKeysMapAccumulator(keyMapper: Object, valueMapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 24 */
        Ok(valueMapper)
    }

    #[cfg_attr(any(), java_method(name = "castingIdentity", descriptor = "()Ljava/util/function/Function;", access = "private static"))]
    pub fn castingIdentity() -> Result<Object> {
        /* TODO: invokedynamic 28 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toCollection", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn toCollection(collectionFactory: Object) -> Result<Object> {
        /* TODO: invokedynamic 33 */
        /* TODO: invokedynamic 36 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toList", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    pub fn toList() -> Result<Object> {
        /* TODO: invokedynamic 46 */
        /* TODO: invokedynamic 50 */
        /* TODO: invokedynamic 51 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableList", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    pub fn toUnmodifiableList() -> Result<Object> {
        /* TODO: invokedynamic 46 */
        /* TODO: invokedynamic 50 */
        /* TODO: invokedynamic 52 */
        /* TODO: invokedynamic 53 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toSet", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    pub fn toSet() -> Result<Object> {
        /* TODO: invokedynamic 60 */
        /* TODO: invokedynamic 61 */
        /* TODO: invokedynamic 62 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableSet", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    pub fn toUnmodifiableSet() -> Result<Object> {
        /* TODO: invokedynamic 60 */
        /* TODO: invokedynamic 61 */
        /* TODO: invokedynamic 66 */
        /* TODO: invokedynamic 67 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    // java: joining()Ljava/util/stream/Collector;
    pub fn joining() -> Result<Object> {
        /* TODO: invokedynamic 71 */
        /* TODO: invokedynamic 72 */
        /* TODO: invokedynamic 73 */
        /* TODO: invokedynamic 74 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: joining(Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
    pub fn joining__seq(delimiter: Object) -> Result<Object> {
        let _t0: Object = Collectors::joining(delimiter, String::from(""), String::from(""))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "joining", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: joining(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/util/stream/Collector;
    pub fn joining__seq_seq_seq(delimiter: Object, prefix: Object, suffix: Object) -> Result<Object> {
        /* TODO: invokedynamic 81 */
        /* TODO: invokedynamic 84 */
        /* TODO: invokedynamic 85 */
        /* TODO: invokedynamic 86 */
        let mut _obj0: Collectors_CollectorImpl = Collectors_CollectorImpl::new(Collectors$CollectorImpl::new(), delimiter, prefix, suffix, Collectors::CH_NOID())?;
        Ok(_obj0)
    }

    #[cfg_attr(any(), java_method(name = "mapMerger", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/function/BinaryOperator;", access = "private static"))]
    pub fn mapMerger(mergeFunction: Object) -> Result<Object> {
        /* TODO: invokedynamic 87 */
        Ok(mergeFunction)
    }

    #[cfg_attr(any(), java_method(name = "mapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn mapping(mapper: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t0;
        let _t1 = downstream.supplier()?;
        /* TODO: invokedynamic 98 */
        let _t2 = downstream.combiner()?;
        let _t3 = downstream.finisher()?;
        let _t4 = downstream.characteristics()?;
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(Collectors$CollectorImpl::new())
    }

    #[cfg_attr(any(), java_method(name = "flatMapping", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn flatMapping(mapper: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t0;
        let _t1 = downstream.supplier()?;
        /* TODO: invokedynamic 111 */
        let _t2 = downstream.combiner()?;
        let _t3 = downstream.finisher()?;
        let _t4 = downstream.characteristics()?;
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(Collectors$CollectorImpl::new())
    }

    #[cfg_attr(any(), java_method(name = "filtering", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn filtering(predicate: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t0;
        let _t1 = downstream.supplier()?;
        /* TODO: invokedynamic 114 */
        let _t2 = downstream.combiner()?;
        let _t3 = downstream.finisher()?;
        let _t4 = downstream.characteristics()?;
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(Collectors$CollectorImpl::new())
    }

    #[cfg_attr(any(), java_method(name = "collectingAndThen", descriptor = "(Ljava/util/stream/Collector;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn collectingAndThen(downstream: Object, finisher: Object) -> Result<Object> {
        let _t0 = downstream.characteristics()?;
        let mut characteristics: Object = _t0;
        let _t1 = characteristics.contains(Collector$Characteristics::IDENTITY_FINISH())?;
        let _t2 = characteristics.size()?;
        characteristics = Collectors::CH_NOID();
        let _t3: Object = EnumSet::copyOf(characteristics)?;
        characteristics = _t3;
        let _t4 = characteristics.remove(Collector$Characteristics::IDENTITY_FINISH())?;
        let _t5: Object = Collections::unmodifiableSet(characteristics)?;
        characteristics = _t5;
        let _t6 = downstream.supplier()?;
        let _t7 = downstream.accumulator()?;
        let _t8 = downstream.combiner()?;
        let _t9 = downstream.finisher()?;
        let _t10 = _t9.andThen(finisher)?;
        Ok(Collectors_CollectorImpl::new(_t6, _t7, _t8, _t10, characteristics)?)
    }

    #[cfg_attr(any(), java_method(name = "counting", descriptor = "()Ljava/util/stream/Collector;", access = "public static"))]
    pub fn counting() -> Result<Object> {
        /* TODO: invokedynamic 154 */
        let _t0: Object = Collectors::summingLong(todo!("stack underflow"))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "minBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn minBy(comparator: Object) -> Result<Object> {
        let _t0: Object = BinaryOperator::minBy(comparator)?;
        let _t1: Object = Collectors::reducing(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "maxBy", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn maxBy(comparator: Object) -> Result<Object> {
        let _t0: Object = BinaryOperator::maxBy(comparator)?;
        let _t1: Object = Collectors::reducing(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "summingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summingInt(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 175 */
        /* TODO: invokedynamic 176 */
        /* TODO: invokedynamic 179 */
        /* TODO: invokedynamic 180 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "summingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summingLong(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 181 */
        /* TODO: invokedynamic 182 */
        /* TODO: invokedynamic 185 */
        /* TODO: invokedynamic 186 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "summingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summingDouble(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 187 */
        /* TODO: invokedynamic 188 */
        /* TODO: invokedynamic 191 */
        /* TODO: invokedynamic 192 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "sumWithCompensation", descriptor = "([DD)[D", access = "static"))]
    pub fn sumWithCompensation(intermediateSum: &[f64], value: f64) -> Result<Vec<f64>> {
        let mut tmp: f64 = (value-intermediateSum[1i32 as usize]);
        let mut sum: f64 = intermediateSum[0i32 as usize];
        let mut velvel: f64 = (sum+tmp);
        intermediateSum[1i32 as usize] = ((velvel-sum)-tmp);
        intermediateSum[0i32 as usize] = velvel;
        Ok(intermediateSum)
    }

    #[cfg_attr(any(), java_method(name = "computeFinalSum", descriptor = "([D)D", access = "static"))]
    pub fn computeFinalSum(summands: &[f64]) -> Result<f64> {
        let mut tmp: f64 = (summands[0i32 as usize]-summands[1i32 as usize]);
        let mut simpleSum: f64 = summands[((summands.len() as i32)).wrapping_sub(1i32) as usize];
        let _t0: bool = Double::isNaN(tmp)?;
        let _t1: bool = Double::isInfinite(simpleSum)?;
        return Ok(simpleSum);
        Ok(tmp)
    }

    #[cfg_attr(any(), java_method(name = "averagingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn averagingInt(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 202 */
        /* TODO: invokedynamic 203 */
        /* TODO: invokedynamic 204 */
        /* TODO: invokedynamic 205 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "averagingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn averagingLong(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 206 */
        /* TODO: invokedynamic 207 */
        /* TODO: invokedynamic 208 */
        /* TODO: invokedynamic 209 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "averagingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn averagingDouble(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 210 */
        /* TODO: invokedynamic 211 */
        /* TODO: invokedynamic 212 */
        /* TODO: invokedynamic 213 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: reducing(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn reducing__obj_binary(identity: Object, op: Object) -> Result<Object> {
        let _t0: Object = Collectors::boxSupplier(identity)?;
        /* TODO: invokedynamic 218 */
        /* TODO: invokedynamic 221 */
        /* TODO: invokedynamic 222 */
        let mut _obj1: Collectors_CollectorImpl = Collectors_CollectorImpl::new(Collectors$CollectorImpl::new(), _t0, op, op, Collectors::CH_NOID())?;
        Ok(_obj1)
    }

    #[cfg_attr(any(), java_method(name = "boxSupplier", descriptor = "(Ljava/lang/Object;)Ljava/util/function/Supplier;", access = "private static"))]
    pub fn boxSupplier(identity: Object) -> Result<Object> {
        /* TODO: invokedynamic 223 */
        Ok(identity)
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: reducing(Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn reducing__binary(op: Object) -> Result<Object> {
        /* TODO: invokedynamic 225 */
        /* TODO: invokedynamic 228 */
        /* TODO: invokedynamic 229 */
        /* TODO: invokedynamic 230 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "reducing", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: reducing(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn reducing__obj_functi_binary(identity: Object, mapper: Object, op: Object) -> Result<Object> {
        let _t0: Object = Collectors::boxSupplier(identity)?;
        /* TODO: invokedynamic 231 */
        /* TODO: invokedynamic 234 */
        /* TODO: invokedynamic 235 */
        Ok(Collectors_CollectorImpl::new(_t0, op, mapper, op, Collectors::CH_NOID())?)
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingBy(Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn groupingBy__functi(classifier: Object) -> Result<Object> {
        let _t0: Object = Collectors::toList()?;
        let _t1: Object = Collectors::groupingBy(classifier, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingBy(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingBy__functi_collec(classifier: Object, downstream: Object) -> Result<Object> {
        /* TODO: invokedynamic 244 */
        let _t0: Object = Collectors::groupingBy(todo!("stack underflow"), classifier, downstream)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "groupingBy", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingBy(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingBy__functi_suppli_collec(classifier: Object, mapFactory: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.supplier()?;
        let mut downstreamSupplier: Object = _t0;
        let _t1 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t1;
        /* TODO: invokedynamic 248 */
        let mut accumulator: Object = downstreamAccumulator;
        let _t2 = downstream.combiner()?;
        let _t3: Object = Collectors::mapMerger(_t2)?;
        let mut merger: Object = _t3;
        let mut mangledFactory: Object = mapFactory;
        let _t4 = downstream.characteristics()?;
        let _t5 = _t4.contains(Collector$Characteristics::IDENTITY_FINISH())?;
        return Ok(Collectors_CollectorImpl::new(mangledFactory, accumulator, merger, Collectors::CH_ID())?);
        let _t6 = downstream.finisher()?;
        let mut downstreamFinisher: Object = _t6;
        /* TODO: invokedynamic 254 */
        let mut finisher: Object = downstreamFinisher;
        Ok(Collectors_CollectorImpl::new(mangledFactory, accumulator, merger, finisher, Collectors::CH_NOID())?)
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingByConcurrent(Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn groupingByConcurrent__functi(classifier: Object) -> Result<Object> {
        /* TODO: invokedynamic 256 */
        let _t0: Object = Collectors::toList()?;
        let _t1: Object = Collectors::groupingByConcurrent(todo!("stack underflow"), classifier, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingByConcurrent(Ljava/util/function/Function;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingByConcurrent__functi_collec(classifier: Object, downstream: Object) -> Result<Object> {
        /* TODO: invokedynamic 256 */
        let _t0: Object = Collectors::groupingByConcurrent(todo!("stack underflow"), classifier, downstream)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "groupingByConcurrent", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: groupingByConcurrent(Ljava/util/function/Function;Ljava/util/function/Supplier;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn groupingByConcurrent__functi_suppli_collec(classifier: Object, mapFactory: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.supplier()?;
        let mut downstreamSupplier: Object = _t0;
        let _t1 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t1;
        let _t2 = downstream.combiner()?;
        let _t3: Object = Collectors::mapMerger(_t2)?;
        let mut merger: Object = _t3;
        let mut mangledFactory: Object = mapFactory;
        let _t4 = downstream.characteristics()?;
        let _t5 = _t4.contains(Collector$Characteristics::CONCURRENT())?;
        /* TODO: invokedynamic 263 */
        let mut accumulator: Object = downstreamAccumulator;
        /* TODO: invokedynamic 264 */
        accumulator = downstreamAccumulator;
        let _t6 = downstream.characteristics()?;
        let _t7 = _t6.contains(Collector$Characteristics::IDENTITY_FINISH())?;
        return Ok(Collectors_CollectorImpl::new(mangledFactory, accumulator, merger, Collectors::CH_CONCURRENT_ID())?);
        let _t8 = downstream.finisher()?;
        let mut downstreamFinisher: Object = _t8;
        /* TODO: invokedynamic 268 */
        let mut finisher: Object = downstreamFinisher;
        Ok(Collectors_CollectorImpl::new(mangledFactory, accumulator, merger, finisher, Collectors::CH_CONCURRENT_NOID())?)
    }

    #[cfg_attr(any(), java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: partitioningBy(Ljava/util/function/Predicate;)Ljava/util/stream/Collector;
    pub fn partitioningBy__predic(predicate: Object) -> Result<Object> {
        let _t0: Object = Collectors::toList()?;
        let _t1: Object = Collectors::partitioningBy(predicate, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "partitioningBy", descriptor = "(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: partitioningBy(Ljava/util/function/Predicate;Ljava/util/stream/Collector;)Ljava/util/stream/Collector;
    pub fn partitioningBy__predic_collec(predicate: Object, downstream: Object) -> Result<Object> {
        let _t0 = downstream.accumulator()?;
        let mut downstreamAccumulator: Object = _t0;
        /* TODO: invokedynamic 276 */
        let mut accumulator: Object = predicate;
        let _t1 = downstream.combiner()?;
        let mut op: Object = _t1;
        /* TODO: invokedynamic 279 */
        let mut merger: Object = op;
        /* TODO: invokedynamic 280 */
        let mut supplier: Object = downstream;
        let _t2 = downstream.characteristics()?;
        let _t3 = _t2.contains(Collector$Characteristics::IDENTITY_FINISH())?;
        return Ok(Collectors_CollectorImpl::new(supplier, accumulator, merger, Collectors::CH_ID())?);
        /* TODO: invokedynamic 283 */
        let mut finisher: Object = downstream;
        Ok(Collectors_CollectorImpl::new(supplier, accumulator, merger, finisher, Collectors::CH_NOID())?)
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toMap(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn toMap__functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 244 */
        let _t0: Object = Collectors::uniqKeysMapAccumulator(keyMapper, valueMapper)?;
        let _t1: Object = Collectors::uniqKeysMapMerger()?;
        let mut _obj2: Collectors_CollectorImpl = Collectors_CollectorImpl::new(Collectors$CollectorImpl::new(), _t0, _t1, Collectors::CH_ID())?;
        Ok(_obj2)
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toUnmodifiableMap(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn toUnmodifiableMap__functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(keyMapper, String::from("keyMapper"))?;
        let _t1: Object = Objects::requireNonNull(valueMapper, String::from("valueMapper"))?;
        let _t2: Object = Collectors::toMap(keyMapper, valueMapper)?;
        /* TODO: invokedynamic 306 */
        let _t3: Object = Collectors::collectingAndThen(todo!("stack underflow"), _t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toMap(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn toMap__functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
        /* TODO: invokedynamic 244 */
        let _t0: Object = Collectors::toMap(todo!("stack underflow"), keyMapper, valueMapper, mergeFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toUnmodifiableMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toUnmodifiableMap(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn toUnmodifiableMap__functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(keyMapper, String::from("keyMapper"))?;
        let _t1: Object = Objects::requireNonNull(valueMapper, String::from("valueMapper"))?;
        let _t2: Object = Objects::requireNonNull(mergeFunction, String::from("mergeFunction"))?;
        /* TODO: invokedynamic 316 */
        let _t3: Object = Collectors::toMap(todo!("stack underflow"), keyMapper, valueMapper, mergeFunction)?;
        /* TODO: invokedynamic 317 */
        let _t4: Object = Collectors::collectingAndThen(todo!("stack underflow"), _t3)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "toMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toMap(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;
    pub fn toMap__functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Object> {
        /* TODO: invokedynamic 318 */
        let mut accumulator: Object = mergeFunction;
        let _t0: Object = Collectors::mapMerger(mergeFunction)?;
        Ok(Collectors_CollectorImpl::new(mapFactory, accumulator, _t0, Collectors::CH_ID())?)
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toConcurrentMap(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/util/stream/Collector;
    pub fn toConcurrentMap__functi_functi(keyMapper: Object, valueMapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 321 */
        let _t0: Object = Collectors::uniqKeysMapAccumulator(keyMapper, valueMapper)?;
        let _t1: Object = Collectors::uniqKeysMapMerger()?;
        let mut _obj2: Collectors_CollectorImpl = Collectors_CollectorImpl::new(Collectors$CollectorImpl::new(), _t0, _t1, Collectors::CH_CONCURRENT_ID())?;
        Ok(_obj2)
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toConcurrentMap(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;)Ljava/util/stream/Collector;
    pub fn toConcurrentMap__functi_functi_binary(keyMapper: Object, valueMapper: Object, mergeFunction: Object) -> Result<Object> {
        /* TODO: invokedynamic 256 */
        let _t0: Object = Collectors::toConcurrentMap(todo!("stack underflow"), keyMapper, valueMapper, mergeFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toConcurrentMap", descriptor = "(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;", access = "public static"))]
    // java: toConcurrentMap(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/util/function/BinaryOperator;Ljava/util/function/Supplier;)Ljava/util/stream/Collector;
    pub fn toConcurrentMap__functi_functi_binary_suppli(keyMapper: Object, valueMapper: Object, mergeFunction: Object, mapFactory: Object) -> Result<Object> {
        /* TODO: invokedynamic 325 */
        let mut accumulator: Object = mergeFunction;
        let _t0: Object = Collectors::mapMerger(mergeFunction)?;
        Ok(Collectors_CollectorImpl::new(mapFactory, accumulator, _t0, Collectors::CH_CONCURRENT_ID())?)
    }

    #[cfg_attr(any(), java_method(name = "summarizingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summarizingInt(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 326 */
        /* TODO: invokedynamic 327 */
        /* TODO: invokedynamic 328 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "summarizingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summarizingLong(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 329 */
        /* TODO: invokedynamic 330 */
        /* TODO: invokedynamic 331 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "summarizingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn summarizingDouble(mapper: Object) -> Result<Object> {
        /* TODO: invokedynamic 332 */
        /* TODO: invokedynamic 333 */
        /* TODO: invokedynamic 334 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/Set;)V */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "teeing", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "public static"))]
    pub fn teeing(downstream1: Object, downstream2: Object, merger: Object) -> Result<Object> {
        let _t0: Object = Collectors::teeing0(downstream1, downstream2, merger)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "teeing0", descriptor = "(Ljava/util/stream/Collector;Ljava/util/stream/Collector;Ljava/util/function/BiFunction;)Ljava/util/stream/Collector;", access = "private static"))]
    pub fn teeing0(downstream1: Object, downstream2: Object, merger: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(downstream1, String::from("downstream1"))?;
        let _t1: Object = Objects::requireNonNull(downstream2, String::from("downstream2"))?;
        let _t2: Object = Objects::requireNonNull(merger, String::from("merger"))?;
        let _t3 = downstream1.supplier()?;
        let _t4: Object = Objects::requireNonNull(_t3, String::from("downstream1 supplier"))?;
        let mut c1Supplier: Object = _t4;
        let _t5 = downstream2.supplier()?;
        let _t6: Object = Objects::requireNonNull(_t5, String::from("downstream2 supplier"))?;
        let mut c2Supplier: Object = _t6;
        let _t7 = downstream1.accumulator()?;
        let _t8: Object = Objects::requireNonNull(_t7, String::from("downstream1 accumulator"))?;
        let mut c1Accumulator: Object = _t8;
        let _t9 = downstream2.accumulator()?;
        let _t10: Object = Objects::requireNonNull(_t9, String::from("downstream2 accumulator"))?;
        let mut c2Accumulator: Object = _t10;
        let _t11 = downstream1.combiner()?;
        let _t12: Object = Objects::requireNonNull(_t11, String::from("downstream1 combiner"))?;
        let mut c1Combiner: Object = _t12;
        let _t13 = downstream2.combiner()?;
        let _t14: Object = Objects::requireNonNull(_t13, String::from("downstream2 combiner"))?;
        let mut c2Combiner: Object = _t14;
        let _t15 = downstream1.finisher()?;
        let _t16: Object = Objects::requireNonNull(_t15, String::from("downstream1 finisher"))?;
        let mut c1Finisher: Object = _t16;
        let _t17 = downstream2.finisher()?;
        let _t18: Object = Objects::requireNonNull(_t17, String::from("downstream2 finisher"))?;
        let mut c2Finisher: Object = _t18;
        let _t19 = downstream1.characteristics()?;
        let mut c1Characteristics: Object = _t19;
        let _t20 = downstream2.characteristics()?;
        let mut c2Characteristics: Object = _t20;
        let _t21 = Collectors::CH_ID().containsAll(c1Characteristics)?;
        let _t22 = Collectors::CH_ID().containsAll(c2Characteristics)?;
        let mut characteristics: Object = Collectors::CH_NOID();
        let _t23: Object = EnumSet::noneOf(118i32)?;
        let mut c: Object = _t23;
        let _t24 = c.addAll(c1Characteristics)?;
        let _t25 = c.retainAll(c2Characteristics)?;
        let _t26 = c.remove(Collector$Characteristics::IDENTITY_FINISH())?;
        let _t27: Object = Collections::unmodifiableSet(c)?;
        characteristics = _t27;
        /* TODO: invokedynamic 380 */
        /* TODO: invokedynamic 383 */
        /* TODO: invokedynamic 384 */
        /* TODO: invokedynamic 385 */
        /* invokespecial Method java/util/stream/Collectors$CollectorImpl.<init>:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;Ljava/util/Set;)V */
        Ok(c2Accumulator)
    }
}
