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
    #[binary_name       = "java/util/stream/MatchOps$MatchOp"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/stream/TerminalOp"]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/stream/TerminalOp<TT;Ljava/lang/Boolean;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "MatchOps.java"]
    #[inner_classes     = "java/util/stream/MatchOps$MatchOp:java/util/stream/MatchOps:MatchOp:26;java/util/stream/MatchOps$MatchKind:java/util/stream/MatchOps:MatchKind:16408;java/util/stream/MatchOps$BooleanTerminalSink:java/util/stream/MatchOps:BooleanTerminalSink:1034;java/util/stream/MatchOps$MatchTask:java/util/stream/MatchOps:MatchTask:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/MatchOps$MatchOp;java/util/stream/TerminalOp"]

    pub struct MatchOps_MatchOp<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "inputShape", descriptor = "Ljava/util/stream/StreamShape;", access = "private", modifiers = "final", is_static = false))]
        pub inputShape: StreamShape,
        #[cfg_attr(any(), java_field(name = "matchKind", descriptor = "Ljava/util/stream/MatchOps$MatchKind;", access = "package", modifiers = "final", is_static = false))]
        pub matchKind: MatchOps_MatchKind,
        #[cfg_attr(any(), java_field(name = "sinkSupplier", descriptor = "Ljava/util/function/Supplier;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/function/Supplier<Ljava/util/stream/MatchOps$BooleanTerminalSink<TT;>;>;"))]
        pub sinkSupplier: Object,
    }

    impl<T> MatchOps_MatchOp<T> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/StreamShape;Ljava/util/stream/MatchOps$MatchKind;Ljava/util/function/Supplier;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/StreamShape;Ljava/util/stream/MatchOps$MatchKind;Ljava/util/function/Supplier<Ljava/util/stream/MatchOps$BooleanTerminalSink<TT;>;>;)V")]
        pub fn new(mut shape: StreamShape, mut matchKind: MatchOps_MatchKind, mut sinkSupplier: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_inputShape(Clone::clone(&shape));
            this.__set_matchKind(Clone::clone(&matchKind));
            this.__set_sinkSupplier(Clone::clone(&sinkSupplier));
            Ok(this)
        }

        #[java_method(name = "getOpFlags", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOpFlags(&self) -> Result<i32> {
            panic!("stub: java/util/stream/MatchOps$MatchOp.getOpFlags:()I")
        }

        #[java_method(name = "inputShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inputShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/MatchOps$MatchOp.inputShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "evaluateSequential", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Boolean;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<S:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TS;>;)Ljava/lang/Boolean;")]
        pub fn evaluateSequential(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<bool> {
            panic!("stub: java/util/stream/MatchOps$MatchOp.evaluateSequential:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Boolean;")
        }

        #[java_method(name = "evaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Boolean;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<S:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TS;>;)Ljava/lang/Boolean;")]
        pub fn evaluateParallel(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<bool> {
            panic!("stub: java/util/stream/MatchOps$MatchOp.evaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Boolean;")
        }
    }
}
