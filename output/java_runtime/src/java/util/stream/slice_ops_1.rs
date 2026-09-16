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

impl From<SliceOps_1> for ReferencePipeline_StatefulOp<Object, Object> {
    fn from(v: SliceOps_1) -> ReferencePipeline_StatefulOp<Object, Object> { v.__into_super() }
}

impl From<SliceOps_1> for ReferencePipeline<Object, Object> {
    fn from(v: SliceOps_1) -> ReferencePipeline<Object, Object> { v.__into_super().__into_super() }
}

impl From<SliceOps_1> for AbstractPipeline<Object, Object, Object> {
    fn from(v: SliceOps_1) -> AbstractPipeline<Object, Object, Object> { v.__into_super().__into_super().__into_super() }
}

impl From<SliceOps_1> for PipelineHelper<Object> {
    fn from(v: SliceOps_1) -> PipelineHelper<Object> { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/SliceOps$1"]
    #[super_class       = "java/util/stream/ReferencePipeline$StatefulOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/stream/ReferencePipeline$StatefulOp<TT;TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "SliceOps.java"]
    #[inner_classes     = "java/util/stream/SliceOps$1:::0;java/util/stream/ReferencePipeline$StatefulOp:java/util/stream/ReferencePipeline:StatefulOp:1032;java/util/stream/StreamSpliterators$UnorderedSliceSpliterator:java/util/stream/StreamSpliterators:UnorderedSliceSpliterator:1032;java/util/stream/StreamSpliterators$UnorderedSliceSpliterator$OfRef:java/util/stream/StreamSpliterators$UnorderedSliceSpliterator:OfRef:24;java/util/stream/StreamSpliterators$SliceSpliterator:java/util/stream/StreamSpliterators:SliceSpliterator:1032;java/util/stream/StreamSpliterators$SliceSpliterator$OfRef:java/util/stream/StreamSpliterators$SliceSpliterator:OfRef:24;java/util/stream/SliceOps$SliceTask:java/util/stream/SliceOps:SliceTask:26;java/util/stream/SliceOps$1$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline_StatefulOp<Object, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$StatefulOp;java/util/stream/SliceOps$1;java/util/stream/Stream"]

    pub struct SliceOps_1 {
        #[cfg_attr(any(), java_field(name = "val$skip", descriptor = "J", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_skip: i64,
        #[cfg_attr(any(), java_field(name = "val$normalizedLimit", descriptor = "J", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_normalizedLimit: i64,
        #[cfg_attr(any(), java_field(name = "val$limit", descriptor = "J", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_limit: i64,
    }

    impl SliceOps_1 {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/stream/StreamShape;IJJJ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut upstream: AbstractPipeline<Object, Object, Object>, mut inputShape: StreamShape, mut opFlags: i32, mut arg_3: i64, mut arg_4: i64, mut arg_5: i64) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_skip(arg_3);
            this.__set_val_normalizedLimit(arg_4);
            this.__set_val_limit(arg_5);
            this = Self::__new_with_super(ReferencePipeline_StatefulOp::new(Clone::clone(&upstream), Clone::clone(&inputShape), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "exactOutputSize", descriptor = "(J)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exactOutputSize(&self, previousSize: i64) -> Result<i64> {
            panic!("stub: java/util/stream/SliceOps$1.exactOutputSize:(J)J")
        }

        #[java_method(name = "unorderedSkipLimitSpliterator", descriptor = "(Ljava/util/Spliterator;JJJ)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TT;>;JJJ)Ljava/util/Spliterator<TT;>;")]
        pub fn unorderedSkipLimitSpliterator(&self, s: Object, skip: i64, arg2: i64, limit: i64) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps$1.unorderedSkipLimitSpliterator:(Ljava/util/Spliterator;JJJ)Ljava/util/Spliterator;")
        }

        #[java_method(name = "opEvaluateParallelLazy", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TT;>;")]
        pub fn opEvaluateParallelLazy(&self, helper: PipelineHelper<Object>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps$1.opEvaluateParallelLazy:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "opEvaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;Ljava/util/function/IntFunction<[TT;>;)Ljava/util/stream/Node<TT;>;")]
        pub fn opEvaluateParallel(&self, helper: PipelineHelper<Object>, spliterator: Object, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps$1.opEvaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TT;>;)Ljava/util/stream/Sink<TT;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SliceOps$1.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }
    }
}
