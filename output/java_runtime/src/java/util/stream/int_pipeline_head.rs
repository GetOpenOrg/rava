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

impl<E_IN: Clone + Default + 'static> From<IntPipeline_Head<E_IN>> for IntPipeline<E_IN> {
    fn from(v: IntPipeline_Head<E_IN>) -> IntPipeline<E_IN> { v.__into_super() }
}

impl<E_IN: Clone + Default + 'static> From<IntPipeline_Head<E_IN>> for AbstractPipeline<E_IN, Object, Object> {
    fn from(v: IntPipeline_Head<E_IN>) -> AbstractPipeline<E_IN, Object, Object> { v.__into_super().__into_super() }
}

impl<E_IN: Clone + Default + 'static> From<IntPipeline_Head<E_IN>> for PipelineHelper<E_IN> {
    fn from(v: IntPipeline_Head<E_IN>) -> PipelineHelper<E_IN> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/IntPipeline$Head"]
    #[super_class       = "java/util/stream/IntPipeline"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E_IN:Ljava/lang/Object;>Ljava/util/stream/IntPipeline<TE_IN;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IntPipeline.java"]
    #[inner_classes     = "java/util/stream/IntPipeline$Head:java/util/stream/IntPipeline:Head:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/PrimitiveIterator$OfInt:java/util/PrimitiveIterator:OfInt:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IntPipeline<E_IN>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/IntPipeline;java/util/stream/IntPipeline$Head;java/util/stream/IntStream;java/util/stream/PipelineHelper"]

    pub struct IntPipeline_Head<E_IN: Clone + Default + 'static>;

    impl<E_IN> IntPipeline_Head<E_IN> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<Ljava/lang/Integer;>;>;IZ)V")]
        pub fn new_suppli_i_z(source: Object, sourceFlags: i32, parallel: bool) -> Result<Self> {
            panic!("stub: java/util/stream/IntPipeline$Head.<init>:(Ljava/util/function/Supplier;IZ)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<Ljava/lang/Integer;>;IZ)V")]
        // java: <init>(Ljava/util/Spliterator;IZ)V
        pub fn new_splite_i_z(mut source: Object, mut sourceFlags: i32, mut parallel: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IntPipeline::new_splite_i_z(Clone::clone(&source), sourceFlags, parallel)?);
            Ok(this)
        }

        #[java_method(name = "opIsStateful", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn opIsStateful(&self) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline$Head.opIsStateful:()Z")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<Ljava/lang/Integer;>;)Ljava/util/stream/Sink<TE_IN;>;")]
        pub fn opWrapSink(&self, flags: i32, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline$Head.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/IntConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/IntPipeline$Head.forEach:(Ljava/util/function/IntConsumer;)V")
        }

        #[java_method(name = "forEachOrdered", descriptor = "(Ljava/util/function/IntConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forEachOrdered(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/IntPipeline$Head.forEachOrdered:(Ljava/util/function/IntConsumer;)V")
        }
    }
}
