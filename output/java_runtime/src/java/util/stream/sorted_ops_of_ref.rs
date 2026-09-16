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

impl<T: Clone + Default + 'static> From<SortedOps_OfRef<T>> for ReferencePipeline_StatefulOp<T, Object> {
    fn from(v: SortedOps_OfRef<T>) -> ReferencePipeline_StatefulOp<T, Object> { v.__into_super() }
}

impl<T: Clone + Default + 'static> From<SortedOps_OfRef<T>> for ReferencePipeline<T, Object> {
    fn from(v: SortedOps_OfRef<T>) -> ReferencePipeline<T, Object> { v.__into_super().__into_super() }
}

impl<T: Clone + Default + 'static> From<SortedOps_OfRef<T>> for AbstractPipeline<T, Object, Object> {
    fn from(v: SortedOps_OfRef<T>) -> AbstractPipeline<T, Object, Object> { v.__into_super().__into_super().__into_super() }
}

impl<T: Clone + Default + 'static> From<SortedOps_OfRef<T>> for PipelineHelper<T> {
    fn from(v: SortedOps_OfRef<T>) -> PipelineHelper<T> { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/SortedOps$OfRef"]
    #[super_class       = "java/util/stream/ReferencePipeline$StatefulOp"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/util/stream/ReferencePipeline$StatefulOp<TT;TT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "SortedOps.java"]
    #[inner_classes     = "java/util/stream/ReferencePipeline$StatefulOp:java/util/stream/ReferencePipeline:StatefulOp:1032;java/util/stream/SortedOps$OfRef:java/util/stream/SortedOps:OfRef:26;java/util/stream/SortedOps$SizedRefSortingSink:java/util/stream/SortedOps:SizedRefSortingSink:26;java/util/stream/SortedOps$RefSortingSink:java/util/stream/SortedOps:RefSortingSink:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline_StatefulOp<T, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$StatefulOp;java/util/stream/SortedOps$OfRef;java/util/stream/Stream"]

    pub struct SortedOps_OfRef<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "isNaturalSort", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub isNaturalSort: bool,
        #[cfg_attr(any(), java_field(name = "comparator", descriptor = "Ljava/util/Comparator;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Comparator<-TT;>;"))]
        pub comparator: Object,
    }

    impl<T> SortedOps_OfRef<T> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TT;*>;)V")]
        // java: <init>(Ljava/util/stream/AbstractPipeline;)V
        pub fn new_abstra(mut upstream: AbstractPipeline<Object, T, Object>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ReferencePipeline_StatefulOp::new(Clone::clone(&upstream), Clone::clone(&StreamShape::REFERENCE()), (StreamOpFlag::IS_ORDERED()|StreamOpFlag::IS_SORTED()))?);
            this.__set_isNaturalSort((1i32 != 0i32));
            let _t0: Object = Comparator::<Object>::naturalOrder()?;
            let mut comp: Object = _t0;
            this.__set_comparator(Clone::clone(&comp));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/Comparator;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TT;*>;Ljava/util/Comparator<-TT;>;)V")]
        pub fn new_abstra_compar(upstream: AbstractPipeline<Object, T, Object>, comparator: Object) -> Result<Self> {
            panic!("stub: java/util/stream/SortedOps$OfRef.<init>:(Ljava/util/stream/AbstractPipeline;Ljava/util/Comparator;)V")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TT;>;)Ljava/util/stream/Sink<TT;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps$OfRef.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "opEvaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;Ljava/util/function/IntFunction<[TT;>;)Ljava/util/stream/Node<TT;>;")]
        pub fn opEvaluateParallel(&self, helper: PipelineHelper<T>, spliterator: Object, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps$OfRef.opEvaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }
    }
}
