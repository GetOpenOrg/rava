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
    #[binary_name       = "java/util/stream/ReduceOps$ReduceOp"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/stream/TerminalOp"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;R:Ljava/lang/Object;S::Ljava/util/stream/ReduceOps$AccumulatingSink<TT;TR;TS;>;>Ljava/lang/Object;Ljava/util/stream/TerminalOp<TT;TR;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReduceOps.java"]
    #[inner_classes     = "java/util/stream/ReduceOps$ReduceOp:java/util/stream/ReduceOps:ReduceOp:1034;java/util/stream/ReduceOps$AccumulatingSink:java/util/stream/ReduceOps:AccumulatingSink:1546;java/util/stream/ReduceOps$ReduceTask:java/util/stream/ReduceOps:ReduceTask:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ReduceOps$ReduceOp;java/util/stream/TerminalOp"]

    pub struct ReduceOps_ReduceOp<T: Clone + Default + 'static, R: Clone + Default + 'static, S: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "inputShape", descriptor = "Ljava/util/stream/StreamShape;", access = "private", modifiers = "final", is_static = false))]
        pub inputShape: StreamShape,
    }

    impl<T, R, S> ReduceOps_ReduceOp<T, R, S> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/StreamShape;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut shape: StreamShape) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_inputShape(Clone::clone(&shape));
            Ok(this)
        }

        #[java_method(name = "makeSink", descriptor = "()Ljava/util/stream/ReduceOps$AccumulatingSink;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()TS;")]
        pub fn makeSink(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps$ReduceOp.makeSink:()Ljava/util/stream/ReduceOps$AccumulatingSink;")
        }

        #[java_method(name = "inputShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inputShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/ReduceOps$ReduceOp.inputShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "evaluateSequential", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)TR;")]
        pub fn evaluateSequential(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps$ReduceOp.evaluateSequential:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Object;")
        }

        #[java_method(name = "evaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TP_IN;>;)TR;")]
        pub fn evaluateParallel(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps$ReduceOp.evaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Object;")
        }
    }
}
