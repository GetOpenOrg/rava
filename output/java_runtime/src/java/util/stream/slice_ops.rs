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
    #[binary_name       = "java/util/stream/SliceOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "SliceOps.java"]
    #[inner_classes     = "java/util/stream/SliceOps$5:::4104;java/util/stream/StreamSpliterators$SliceSpliterator:java/util/stream/StreamSpliterators:SliceSpliterator:1032;java/util/stream/StreamSpliterators$SliceSpliterator$OfRef:java/util/stream/StreamSpliterators$SliceSpliterator:OfRef:24;java/util/stream/StreamSpliterators$SliceSpliterator$OfInt:java/util/stream/StreamSpliterators$SliceSpliterator:OfInt:24;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/stream/StreamSpliterators$SliceSpliterator$OfLong:java/util/stream/StreamSpliterators$SliceSpliterator:OfLong:24;java/util/Spliterator$OfLong:java/util/Spliterator:OfLong:1545;java/util/stream/StreamSpliterators$SliceSpliterator$OfDouble:java/util/stream/StreamSpliterators$SliceSpliterator:OfDouble:24;java/util/Spliterator$OfDouble:java/util/Spliterator:OfDouble:1545;java/util/stream/SliceOps$1:::0;java/util/stream/SliceOps$2:::0;java/util/stream/SliceOps$3:::0;java/util/stream/SliceOps$4:::0;java/util/stream/SliceOps$SliceTask:java/util/stream/SliceOps:SliceTask:26;java/util/stream/SliceOps$4$1:::0;java/util/stream/SliceOps$3$1:::0;java/util/stream/SliceOps$2$1:::0;java/util/stream/SliceOps$1$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/SliceOps"]

    pub struct SliceOps;

    impl SliceOps {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/SliceOps.<init>:()V")
        }

        #[java_method(name = "calcSize", descriptor = "(JJJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn calcSize(size: i64, arg1: i64, skip: i64) -> Result<i64> {
            panic!("stub: java/util/stream/SliceOps.calcSize:(JJJ)J")
        }

        #[java_method(name = "calcSliceFence", descriptor = "(JJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn calcSliceFence(skip: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/stream/SliceOps.calcSliceFence:(JJ)J")
        }

        #[java_method(name = "sliceSpliterator", descriptor = "(Ljava/util/stream/StreamShape;Ljava/util/Spliterator;JJ)Ljava/util/Spliterator;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/StreamShape;Ljava/util/Spliterator<TP_IN;>;JJ)Ljava/util/Spliterator<TP_IN;>;")]
        pub fn sliceSpliterator(shape: StreamShape, s: Object, skip: i64, arg3: i64) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps.sliceSpliterator:(Ljava/util/stream/StreamShape;Ljava/util/Spliterator;JJ)Ljava/util/Spliterator;")
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*TT;*>;JJ)Ljava/util/stream/Stream<TT;>;")]
        pub fn makeRef(mut upstream: AbstractPipeline<Object, Object, Object>, mut skip: i64, mut limit: i64) -> Result<Object> {
            if (((skip>(0i64)) as i32-((skip)<(0i64)) as i32)<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Skip must be non-negative: ")))?;
                let _t1 = _t0.append_l(skip)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut normalizedLimit = (if (((limit>(0i64)) as i32-((limit)<(0i64)) as i32)>=0) { limit } else { 9223372036854775807i64 });
            let _t0: i32 = SliceOps::flags(limit)?;
            Ok(Object::from_any(SliceOps_1::new(Clone::clone(&upstream), Clone::clone(&StreamShape::REFERENCE()), _t0, skip, normalizedLimit, limit)?.clone()))
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/IntStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Integer;*>;JJ)Ljava/util/stream/IntStream;")]
        pub fn makeInt(upstream: AbstractPipeline<Object, Object, Object>, skip: i64, arg2: i64) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps.makeInt:(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/LongStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Long;*>;JJ)Ljava/util/stream/LongStream;")]
        pub fn makeLong(upstream: AbstractPipeline<Object, Object, Object>, skip: i64, arg2: i64) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps.makeLong:(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Double;*>;JJ)Ljava/util/stream/DoubleStream;")]
        pub fn makeDouble(upstream: AbstractPipeline<Object, Object, Object>, skip: i64, arg2: i64) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps.makeDouble:(Ljava/util/stream/AbstractPipeline;JJ)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "flags", descriptor = "(J)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flags(mut limit: i64) -> Result<i32> {
            Ok((StreamOpFlag::IS_SIZE_ADJUSTING()|(if (((limit>(-1i64)) as i32-((limit)<(-1i64)) as i32)!=0) { StreamOpFlag::IS_SHORT_CIRCUIT() } else { 0i32 })))
        }
    }
}
