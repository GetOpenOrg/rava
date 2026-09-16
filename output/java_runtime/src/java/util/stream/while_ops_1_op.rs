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

impl From<WhileOps_1Op> for ReferencePipeline_StatefulOp<Object, Object> {
    fn from(v: WhileOps_1Op) -> ReferencePipeline_StatefulOp<Object, Object> { v.__into_super() }
}

impl From<WhileOps_1Op> for ReferencePipeline<Object, Object> {
    fn from(v: WhileOps_1Op) -> ReferencePipeline<Object, Object> { v.__into_super().__into_super() }
}

impl From<WhileOps_1Op> for AbstractPipeline<Object, Object, Object> {
    fn from(v: WhileOps_1Op) -> AbstractPipeline<Object, Object, Object> { v.__into_super().__into_super().__into_super() }
}

impl From<WhileOps_1Op> for PipelineHelper<Object> {
    fn from(v: WhileOps_1Op) -> PipelineHelper<Object> { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/WhileOps$1Op"]
    #[super_class       = "java/util/stream/ReferencePipeline$StatefulOp"]
    #[interfaces        = "java/util/stream/WhileOps$DropWhileOp"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/stream/ReferencePipeline$StatefulOp<TT;TT;>;Ljava/util/stream/WhileOps$DropWhileOp<TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "WhileOps.java"]
    #[inner_classes     = "java/util/stream/WhileOps$1Op::Op:0;java/util/stream/ReferencePipeline$StatefulOp:java/util/stream/ReferencePipeline:StatefulOp:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Dropping:24;java/util/stream/WhileOps$DropWhileTask:java/util/stream/WhileOps:DropWhileTask:26;java/util/stream/WhileOps$DropWhileSink:java/util/stream/WhileOps:DropWhileSink:1544;java/util/stream/WhileOps$1Op$1OpSink::OpSink:0;java/util/stream/WhileOps$DropWhileOp:java/util/stream/WhileOps:DropWhileOp:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline_StatefulOp<Object, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$StatefulOp;java/util/stream/Stream;java/util/stream/WhileOps$1Op;java/util/stream/WhileOps$DropWhileOp"]

    pub struct WhileOps_1Op {
        #[cfg_attr(any(), java_field(name = "val$predicate", descriptor = "Ljava/util/function/Predicate;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_predicate: Object,
    }

    impl WhileOps_1Op {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/stream/StreamShape;ILjava/util/function/Predicate;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TT;*>;Ljava/util/stream/StreamShape;I)V")]
        pub fn new(mut upstream: AbstractPipeline<Object, Object, Object>, mut inputShape: StreamShape, mut opFlags: i32, mut arg_3: Object) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_predicate(Clone::clone(&arg_3));
            this = Self::__new_with_super(ReferencePipeline_StatefulOp::new(Clone::clone(&upstream), Clone::clone(&inputShape), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "opEvaluateParallelLazy", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TT;>;")]
        pub fn opEvaluateParallelLazy(&self, helper: PipelineHelper<Object>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$1Op.opEvaluateParallelLazy:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "opEvaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;Ljava/util/function/IntFunction<[TT;>;)Ljava/util/stream/Node<TT;>;")]
        pub fn opEvaluateParallel(&self, helper: PipelineHelper<Object>, spliterator: Object, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$1Op.opEvaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TT;>;)Ljava/util/stream/Sink<TT;>;")]
        pub fn opWrapSink_i_sink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$1Op.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "opWrapSink", descriptor = "(Ljava/util/stream/Sink;Z)Ljava/util/stream/WhileOps$DropWhileSink;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/Sink<TT;>;Z)Ljava/util/stream/WhileOps$DropWhileSink<TT;>;")]
        pub fn opWrapSink_sink_z(&self, sink: Object, retainAndCountDroppedElements: bool) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps$1Op.opWrapSink:(Ljava/util/stream/Sink;Z)Ljava/util/stream/WhileOps$DropWhileSink;")
        }
    }
}
