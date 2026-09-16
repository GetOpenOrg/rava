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
    #[binary_name       = "java/util/stream/Streams$RangeIntSpliterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Spliterator$OfInt"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Streams.java"]
    #[inner_classes     = "java/util/stream/Streams$RangeIntSpliterator:java/util/stream/Streams:RangeIntSpliterator:24;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/Spliterator$OfPrimitive:java/util/Spliterator:OfPrimitive:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Spliterator$OfInt;java/util/stream/Streams$RangeIntSpliterator"]

    pub struct Streams_RangeIntSpliterator {
        #[cfg_attr(any(), java_field(name = "from", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub from: i32,
        #[cfg_attr(any(), java_field(name = "upTo", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub upTo: i32,
        #[cfg_attr(any(), java_field(name = "last", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub last: i32,
    }

    impl Streams_RangeIntSpliterator {
        #[cfg_attr(any(), java_field(name = "BALANCED_SPLIT_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16777216"))]
        // static field: BALANCED_SPLIT_THRESHOLD:I
        pub fn BALANCED_SPLIT_THRESHOLD() -> i32 {
            16777216
        }

        #[cfg_attr(any(), java_field(name = "RIGHT_BALANCED_SPLIT_RATIO", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: RIGHT_BALANCED_SPLIT_RATIO:I
        pub fn RIGHT_BALANCED_SPLIT_RATIO() -> i32 {
            8
        }

        #[java_method(name = "<init>", descriptor = "(IIZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IIZ)V
        pub fn new_i_i_z(mut from: i32, mut upTo: i32, mut closed: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Streams_RangeIntSpliterator::new_i_i_i(from, upTo, (!(!(closed)) as i32))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(III)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(III)V
        pub fn new_i_i_i(mut from: i32, mut upTo: i32, mut last: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_from(from);
            this.__set_upTo(upTo);
            this.__set_last(last);
            Ok(this)
        }

        #[java_method(name = "tryAdvance", descriptor = "(Ljava/util/function/IntConsumer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryAdvance(&self, consumer: Object) -> Result<bool> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.tryAdvance:(Ljava/util/function/IntConsumer;)Z")
        }

        #[java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/IntConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forEachRemaining(&self, consumer: Object) -> Result<()> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.forEachRemaining:(Ljava/util/function/IntConsumer;)V")
        }

        #[java_method(name = "estimateSize", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn estimateSize(&self) -> Result<i64> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.estimateSize:()J")
        }

        #[java_method(name = "characteristics", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn characteristics(&self) -> Result<i32> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.characteristics:()I")
        }

        #[java_method(name = "getComparator", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<-Ljava/lang/Integer;>;")]
        pub fn getComparator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.getComparator:()Ljava/util/Comparator;")
        }

        #[java_method(name = "trySplit", descriptor = "()Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySplit(&self) -> Result<Object> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.trySplit:()Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "splitPoint", descriptor = "(J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn splitPoint(&self, size: i64) -> Result<i32> {
            panic!("stub: java/util/stream/Streams$RangeIntSpliterator.splitPoint:(J)I")
        }
    }
}
