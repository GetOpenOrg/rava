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

impl<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static> From<ReferencePipeline_Head<E_IN, E_OUT>> for ReferencePipeline<E_IN, E_OUT> {
    fn from(v: ReferencePipeline_Head<E_IN, E_OUT>) -> ReferencePipeline<E_IN, E_OUT> { v.__into_super() }
}

impl<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static> From<ReferencePipeline_Head<E_IN, E_OUT>> for AbstractPipeline<E_IN, E_OUT, Object> {
    fn from(v: ReferencePipeline_Head<E_IN, E_OUT>) -> AbstractPipeline<E_IN, E_OUT, Object> { v.__into_super().__into_super() }
}

impl<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static> From<ReferencePipeline_Head<E_IN, E_OUT>> for PipelineHelper<E_IN> {
    fn from(v: ReferencePipeline_Head<E_IN, E_OUT>) -> PipelineHelper<E_IN> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/ReferencePipeline$Head"]
    #[super_class       = "java/util/stream/ReferencePipeline"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E_IN:Ljava/lang/Object;E_OUT:Ljava/lang/Object;>Ljava/util/stream/ReferencePipeline<TE_IN;TE_OUT;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReferencePipeline.java"]
    #[inner_classes     = "java/util/stream/ReferencePipeline$Head:java/util/stream/ReferencePipeline:Head:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ReferencePipeline<E_IN, E_OUT>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/ReferencePipeline$Head;java/util/stream/Stream"]

    pub struct ReferencePipeline_Head<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static>;

    impl<E_IN, E_OUT> ReferencePipeline_Head<E_IN, E_OUT> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<*>;>;IZ)V")]
        pub fn new_suppli_i_z(source: Object, sourceFlags: i32, parallel: bool) -> Result<Self> {
            panic!("stub: java/util/stream/ReferencePipeline$Head.<init>:(Ljava/util/function/Supplier;IZ)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<*>;IZ)V")]
        // java: <init>(Ljava/util/Spliterator;IZ)V
        pub fn new_splite_i_z(mut source: Object, mut sourceFlags: i32, mut parallel: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ReferencePipeline::new_splite_i_z(Clone::clone(&source), sourceFlags, parallel)?);
            Ok(this)
        }

        #[java_method(name = "opIsStateful", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn opIsStateful(&self) -> Result<bool> {
            panic!("stub: java/util/stream/ReferencePipeline$Head.opIsStateful:()Z")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TE_OUT;>;)Ljava/util/stream/Sink<TE_IN;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline$Head.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE_OUT;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/ReferencePipeline$Head.forEach:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "forEachOrdered", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE_OUT;>;)V")]
        pub fn forEachOrdered(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/ReferencePipeline$Head.forEachOrdered:(Ljava/util/function/Consumer;)V")
        }
    }
}
