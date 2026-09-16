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

impl From<ReferencePipeline_3> for ReferencePipeline_StatelessOp<Object, Object> {
    fn from(v: ReferencePipeline_3) -> ReferencePipeline_StatelessOp<Object, Object> { v.__into_super() }
}

impl From<ReferencePipeline_3> for ReferencePipeline<Object, Object> {
    fn from(v: ReferencePipeline_3) -> ReferencePipeline<Object, Object> { v.__into_super().__into_super() }
}

impl From<ReferencePipeline_3> for AbstractPipeline<Object, Object, Object> {
    fn from(v: ReferencePipeline_3) -> AbstractPipeline<Object, Object, Object> { v.__into_super().__into_super().__into_super() }
}

impl From<ReferencePipeline_3> for PipelineHelper<Object> {
    fn from(v: ReferencePipeline_3) -> PipelineHelper<Object> { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/ReferencePipeline$3"]
    #[super_class       = "java/util/stream/ReferencePipeline$StatelessOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/util/stream/ReferencePipeline$StatelessOp<TP_OUT;TR;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReferencePipeline.java"]
    #[inner_classes     = "java/util/stream/ReferencePipeline$3:::0;java/util/stream/ReferencePipeline$StatelessOp:java/util/stream/ReferencePipeline:StatelessOp:1032;java/util/stream/ReferencePipeline$3$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline_StatelessOp<Object, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$3;java/util/stream/ReferencePipeline$StatelessOp;java/util/stream/Stream"]

    pub struct ReferencePipeline_3 {
        #[cfg_attr(any(), java_field(name = "val$mapper", descriptor = "Ljava/util/function/Function;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub val_mapper: Object,
    }

    impl ReferencePipeline_3 {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/ReferencePipeline;Ljava/util/stream/AbstractPipeline;Ljava/util/stream/StreamShape;ILjava/util/function/Function;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32784;:0;:0;:0;:4112")]
        pub fn new(mut this_0: ReferencePipeline<Object, Object>, mut upstream: AbstractPipeline<Object, Object, Object>, mut inputShape: StreamShape, mut opFlags: i32, mut arg_4: Object) -> Result<Self> {
            let mut this = Self::default();
            this.__set_val_mapper(Clone::clone(&arg_4));
            this = Self::__new_with_super(ReferencePipeline_StatelessOp::new(Clone::clone(&upstream), Clone::clone(&inputShape), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TR;>;)Ljava/util/stream/Sink<TP_OUT;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline$3.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }
    }
}
