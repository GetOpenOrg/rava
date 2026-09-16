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

impl<E_IN: Clone + Default + 'static> From<IntPipeline_StatelessOp<E_IN>> for IntPipeline<E_IN> {
    fn from(v: IntPipeline_StatelessOp<E_IN>) -> IntPipeline<E_IN> { v.__into_super() }
}

impl<E_IN: Clone + Default + 'static> From<IntPipeline_StatelessOp<E_IN>> for AbstractPipeline<E_IN, Object, Object> {
    fn from(v: IntPipeline_StatelessOp<E_IN>) -> AbstractPipeline<E_IN, Object, Object> { v.__into_super().__into_super() }
}

impl<E_IN: Clone + Default + 'static> From<IntPipeline_StatelessOp<E_IN>> for PipelineHelper<E_IN> {
    fn from(v: IntPipeline_StatelessOp<E_IN>) -> PipelineHelper<E_IN> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/IntPipeline$StatelessOp"]
    #[super_class       = "java/util/stream/IntPipeline"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E_IN:Ljava/lang/Object;>Ljava/util/stream/IntPipeline<TE_IN;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IntPipeline.java"]
    #[inner_classes     = "java/util/stream/IntPipeline$StatelessOp:java/util/stream/IntPipeline:StatelessOp:1032;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/PrimitiveIterator$OfInt:java/util/PrimitiveIterator:OfInt:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IntPipeline<E_IN>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/IntPipeline;java/util/stream/IntPipeline$StatelessOp;java/util/stream/IntStream;java/util/stream/PipelineHelper"]

    pub struct IntPipeline_StatelessOp<E_IN: Clone + Default + 'static>;

    impl<E_IN> IntPipeline_StatelessOp<E_IN> {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/stream/StreamShape;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TE_IN;*>;Ljava/util/stream/StreamShape;I)V")]
        pub fn new(mut upstream: AbstractPipeline<Object, E_IN, Object>, mut inputShape: StreamShape, mut opFlags: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IntPipeline::new_abstra_i(Clone::clone(&upstream), opFlags)?);
            let _t0 = upstream.getOutputShape()?;
            if Object::from_any(_t0.clone()) != Object::from_any(inputShape.clone()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(this)
        }

        #[java_method(name = "opIsStateful", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn opIsStateful(&self) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline$StatelessOp.opIsStateful:()Z")
        }
    }
}
