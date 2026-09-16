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
    #[binary_name       = "java/util/stream/IntStream"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/stream/BaseStream"]
    #[access            = "public"]
    #[modifiers         = "abstract interface"]
    #[generic_signature = "Ljava/lang/Object;Ljava/util/stream/BaseStream<Ljava/lang/Integer;Ljava/util/stream/IntStream;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IntStream.java"]
    #[inner_classes     = "java/util/stream/IntStream$IntMapMultiConsumer:java/util/stream/IntStream:IntMapMultiConsumer:1545;java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfInt:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:Taking:24;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:Dropping:24;java/util/stream/Streams$IntStreamBuilderImpl:java/util/stream/Streams:IntStreamBuilderImpl:24;java/util/stream/IntStream$1:::0;java/util/stream/IntStream$2:::0;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:java/util/stream/StreamSpliterators:InfiniteSupplyingSpliterator:1032;java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator$OfInt:java/util/stream/StreamSpliterators$InfiniteSupplyingSpliterator:OfInt:24;java/util/stream/Streams$RangeIntSpliterator:java/util/stream/Streams:RangeIntSpliterator:24;java/util/stream/Streams$ConcatSpliterator:java/util/stream/Streams:ConcatSpliterator:1032;java/util/stream/Streams$ConcatSpliterator$OfInt:java/util/stream/Streams$ConcatSpliterator:OfInt:8;java/util/PrimitiveIterator$OfInt:java/util/PrimitiveIterator:OfInt:1545;java/util/stream/SpinedBuffer$OfInt:java/util/stream/SpinedBuffer:OfInt:8;java/util/stream/IntStream$Builder:java/util/stream/IntStream:Builder:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = true]

    pub struct IntStream;
}
