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

impl From<DistinctOps_1> for ReferencePipeline_StatefulOp<Object, Object> {
    fn from(v: DistinctOps_1) -> ReferencePipeline_StatefulOp<Object, Object> { v.__into_super() }
}

impl From<DistinctOps_1> for ReferencePipeline<Object, Object> {
    fn from(v: DistinctOps_1) -> ReferencePipeline<Object, Object> { v.__into_super().__into_super() }
}

impl From<DistinctOps_1> for AbstractPipeline<Object, Object, Object> {
    fn from(v: DistinctOps_1) -> AbstractPipeline<Object, Object, Object> { v.__into_super().__into_super().__into_super() }
}

impl From<DistinctOps_1> for PipelineHelper<Object> {
    fn from(v: DistinctOps_1) -> PipelineHelper<Object> { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/DistinctOps$1"]
    #[super_class       = "java/util/stream/ReferencePipeline$StatefulOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/stream/ReferencePipeline$StatefulOp<TT;TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DistinctOps.java"]
    #[inner_classes     = "java/util/stream/ReferencePipeline$StatefulOp:java/util/stream/ReferencePipeline:StatefulOp:1032;java/util/stream/DistinctOps$1:::0;java/util/concurrent/ConcurrentHashMap$KeySetView:java/util/concurrent/ConcurrentHashMap:KeySetView:25;java/util/stream/StreamSpliterators$DistinctSpliterator:java/util/stream/StreamSpliterators:DistinctSpliterator:24;java/util/stream/DistinctOps$1$1:::0;java/util/stream/DistinctOps$1$2:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline_StatefulOp<Object, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/DistinctOps$1;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$StatefulOp;java/util/stream/Stream"]

    pub struct DistinctOps_1;

    impl DistinctOps_1 {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/stream/StreamShape;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut upstream: AbstractPipeline<Object, Object, Object>, mut inputShape: StreamShape, mut opFlags: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ReferencePipeline_StatefulOp::new(Clone::clone(&upstream), Clone::clone(&inputShape), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "reduce", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/stream/Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)Ljava/util/stream/Node<TT;>;")]
        pub fn reduce(&self, helper: PipelineHelper<Object>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/DistinctOps$1.reduce:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "opEvaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;Ljava/util/function/IntFunction<[TT;>;)Ljava/util/stream/Node<TT;>;")]
        pub fn opEvaluateParallel(&self, helper: PipelineHelper<Object>, spliterator: Object, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/DistinctOps$1.opEvaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "opEvaluateParallelLazy", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TT;>;")]
        pub fn opEvaluateParallelLazy(&self, helper: PipelineHelper<Object>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/DistinctOps$1.opEvaluateParallelLazy:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TT;>;)Ljava/util/stream/Sink<TT;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/DistinctOps$1.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }
    }
}
