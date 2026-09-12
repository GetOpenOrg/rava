#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/stream/StreamSupport",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "StreamSupport.java",
))]
pub struct StreamSupport;

impl StreamSupport {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "stream", descriptor = "(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;", access = "public static"))]
    // java: stream(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;
    pub fn stream__splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(spliterator)?;
        let _t1: i32 = StreamOpFlag::fromCharacteristics(spliterator)?;
        Ok(ReferencePipeline_Head::new(spliterator, _t1, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "stream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/Stream;", access = "public static"))]
    // java: stream(Ljava/util/function/Supplier;IZ)Ljava/util/stream/Stream;
    pub fn stream__suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(supplier)?;
        let _t1: i32 = StreamOpFlag::fromCharacteristics(characteristics)?;
        Ok(ReferencePipeline_Head::new(supplier, _t1, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "intStream", descriptor = "(Ljava/util/Spliterator$OfInt;Z)Ljava/util/stream/IntStream;", access = "public static"))]
    // java: intStream(Ljava/util/Spliterator$OfInt;Z)Ljava/util/stream/IntStream;
    pub fn intStream__splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(spliterator)?;
        Ok(IntPipeline_Head::new(spliterator, _t0, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "intStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/IntStream;", access = "public static"))]
    // java: intStream(Ljava/util/function/Supplier;IZ)Ljava/util/stream/IntStream;
    pub fn intStream__suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(characteristics)?;
        Ok(IntPipeline_Head::new(supplier, _t0, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "longStream", descriptor = "(Ljava/util/Spliterator$OfLong;Z)Ljava/util/stream/LongStream;", access = "public static"))]
    // java: longStream(Ljava/util/Spliterator$OfLong;Z)Ljava/util/stream/LongStream;
    pub fn longStream__splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(spliterator)?;
        Ok(LongPipeline_Head::new(spliterator, _t0, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "longStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/LongStream;", access = "public static"))]
    // java: longStream(Ljava/util/function/Supplier;IZ)Ljava/util/stream/LongStream;
    pub fn longStream__suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(characteristics)?;
        Ok(LongPipeline_Head::new(supplier, _t0, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "doubleStream", descriptor = "(Ljava/util/Spliterator$OfDouble;Z)Ljava/util/stream/DoubleStream;", access = "public static"))]
    // java: doubleStream(Ljava/util/Spliterator$OfDouble;Z)Ljava/util/stream/DoubleStream;
    pub fn doubleStream__splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(spliterator)?;
        Ok(DoublePipeline_Head::new(spliterator, _t0, parallel)?)
    }

    #[cfg_attr(any(), java_method(name = "doubleStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/DoubleStream;", access = "public static"))]
    // java: doubleStream(Ljava/util/function/Supplier;IZ)Ljava/util/stream/DoubleStream;
    pub fn doubleStream__suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
        let _t0: i32 = StreamOpFlag::fromCharacteristics(characteristics)?;
        Ok(DoublePipeline_Head::new(supplier, _t0, parallel)?)
    }
}
