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

impl<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static, S: Clone + Default + 'static> From<AbstractPipeline<E_IN, E_OUT, S>> for PipelineHelper<E_IN> {
    fn from(v: AbstractPipeline<E_IN, E_OUT, S>) -> PipelineHelper<E_IN> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/AbstractPipeline"]
    #[super_class       = "java/util/stream/PipelineHelper"]
    #[interfaces        = "java/util/stream/BaseStream"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E_IN:Ljava/lang/Object;E_OUT:Ljava/lang/Object;S::Ljava/util/stream/BaseStream<TE_OUT;TS;>;>Ljava/util/stream/PipelineHelper<TE_OUT;>;Ljava/util/stream/BaseStream<TE_OUT;TS;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractPipeline.java"]
    #[inner_classes     = "java/util/stream/Node$Builder:java/util/stream/Node:Builder:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "PipelineHelper<E_IN>"]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper"]

    pub struct AbstractPipeline<E_IN: Clone + Default + 'static, E_OUT: Clone + Default + 'static, S: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "sourceStage", descriptor = "Ljava/util/stream/AbstractPipeline;", access = "private", modifiers = "final", is_static = false))]
        pub sourceStage: AbstractPipeline<Object, Object, Object>,
        #[cfg_attr(any(), java_field(name = "previousStage", descriptor = "Ljava/util/stream/AbstractPipeline;", access = "private", modifiers = "final", is_static = false))]
        pub previousStage: AbstractPipeline<Object, Object, Object>,
        #[cfg_attr(any(), java_field(name = "sourceOrOpFlags", descriptor = "I", access = "protected", modifiers = "final", is_static = false))]
        pub sourceOrOpFlags: i32,
        #[cfg_attr(any(), java_field(name = "nextStage", descriptor = "Ljava/util/stream/AbstractPipeline;", access = "private", modifiers = "", is_static = false))]
        pub nextStage: AbstractPipeline<Object, Object, Object>,
        #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub depth: i32,
        #[cfg_attr(any(), java_field(name = "combinedFlags", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub combinedFlags: i32,
        #[cfg_attr(any(), java_field(name = "sourceSpliterator", descriptor = "Ljava/util/Spliterator;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Spliterator<*>;"))]
        pub sourceSpliterator: Object,
        #[cfg_attr(any(), java_field(name = "sourceSupplier", descriptor = "Ljava/util/function/Supplier;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/function/Supplier<+Ljava/util/Spliterator<*>;>;"))]
        pub sourceSupplier: Object,
        #[cfg_attr(any(), java_field(name = "linkedOrConsumed", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub linkedOrConsumed: bool,
        #[cfg_attr(any(), java_field(name = "sourceAnyStateful", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub sourceAnyStateful: bool,
        #[cfg_attr(any(), java_field(name = "sourceCloseAction", descriptor = "Ljava/lang/Runnable;", access = "private", modifiers = "", is_static = false))]
        pub sourceCloseAction: Object,
        #[cfg_attr(any(), java_field(name = "parallel", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub parallel: bool,
    }

    impl<E_IN, E_OUT, S> AbstractPipeline<E_IN, E_OUT, S> {
        #[cfg_attr(any(), java_field(name = "MSG_STREAM_LINKED", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "stream has already been operated upon or closed"))]
        // static field: MSG_STREAM_LINKED:Ljava/lang/String;
        pub fn MSG_STREAM_LINKED() -> String {
            String::from("stream has already been operated upon or closed")
        }

        #[cfg_attr(any(), java_field(name = "MSG_CONSUMED", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "source already consumed or closed"))]
        // static field: MSG_CONSUMED:Ljava/lang/String;
        pub fn MSG_CONSUMED() -> String {
            String::from("source already consumed or closed")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<*>;>;IZ)V")]
        pub fn new_suppli_i_z(source: Object, sourceFlags: i32, parallel: bool) -> Result<Self> {
            panic!("stub: java/util/stream/AbstractPipeline.<init>:(Ljava/util/function/Supplier;IZ)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<*>;IZ)V")]
        // java: <init>(Ljava/util/Spliterator;IZ)V
        pub fn new_splite_i_z(mut source: Object, mut sourceFlags: i32, mut parallel: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(PipelineHelper::new()?);
            this.__set_previousStage(Default::default());
            this.__set_sourceSpliterator(Clone::clone(&source));
            this.__set_sourceStage(Clone::clone(&this));
            this.__set_sourceOrOpFlags((sourceFlags&StreamOpFlag::STREAM_MASK()));
            this.__set_combinedFlags((((this.__get_sourceOrOpFlags()<<(1i32&0x1f))^-1i32)&StreamOpFlag::INITIAL_OPS_VALUE()));
            this.__set_depth(0i32);
            this.__set_parallel(parallel);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TE_IN;*>;I)V")]
        // java: <init>(Ljava/util/stream/AbstractPipeline;I)V
        pub fn new_abstra_i(mut previousStage: AbstractPipeline<Object, E_IN, Object>, mut opFlags: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(PipelineHelper::new()?);
            if previousStage.__get_linkedOrConsumed() {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            previousStage.__set_linkedOrConsumed((1i32 != 0i32));
            previousStage.__set_nextStage(this);
            this.__set_previousStage(Clone::clone(&previousStage));
            this.__set_sourceOrOpFlags((opFlags&StreamOpFlag::OP_MASK()));
            let _t0: i32 = StreamOpFlag::combineOpFlags(opFlags, previousStage.__get_combinedFlags())?;
            this.__set_combinedFlags(_t0);
            this.__set_sourceStage(Clone::clone(&previousStage.__get_sourceStage()));
            let _t1 = this.opIsStateful()?;
            if _t1 {
                this.__get_sourceStage().__set_sourceAnyStateful((1i32 != 0i32));
            }
            this.__set_depth((previousStage.__get_depth()).wrapping_add(1i32));
            Ok(this)
        }

        #[java_method(name = "evaluate", descriptor = "(Ljava/util/stream/TerminalOp;)Ljava/lang/Object;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/stream/TerminalOp<TE_OUT;TR;>;)TR;")]
        pub fn evaluate_termin(&self, terminalOp: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.evaluate:(Ljava/util/stream/TerminalOp;)Ljava/lang/Object;")
        }

        #[java_method(name = "evaluateToArrayNode", descriptor = "(Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/IntFunction<[TE_OUT;>;)Ljava/util/stream/Node<TE_OUT;>;")]
        pub fn evaluateToArrayNode(&self, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.evaluateToArrayNode:(Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "sourceStageSpliterator", descriptor = "()Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn sourceStageSpliterator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.sourceStageSpliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "sequential", descriptor = "()Ljava/util/stream/BaseStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TS;")]
        pub fn sequential(&self) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.sequential:()Ljava/util/stream/BaseStream;")
        }

        #[java_method(name = "parallel", descriptor = "()Ljava/util/stream/BaseStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TS;")]
        pub fn parallel(&self) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.parallel:()Ljava/util/stream/BaseStream;")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/util/stream/AbstractPipeline.close:()V")
        }

        #[java_method(name = "onClose", descriptor = "(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Runnable;)TS;")]
        pub fn onClose(&self, closeHandler: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.onClose:(Ljava/lang/Runnable;)Ljava/util/stream/BaseStream;")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "isParallel", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isParallel(&self) -> Result<bool> {
            panic!("stub: java/util/stream/AbstractPipeline.isParallel:()Z")
        }

        #[java_method(name = "getStreamFlags", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStreamFlags(&self) -> Result<i32> {
            panic!("stub: java/util/stream/AbstractPipeline.getStreamFlags:()I")
        }

        #[java_method(name = "sourceSpliterator", descriptor = "(I)Ljava/util/Spliterator;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/Spliterator<*>;")]
        pub fn sourceSpliterator(&self, terminalFlags: i32) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.sourceSpliterator:(I)Ljava/util/Spliterator;")
        }

        #[java_method(name = "getSourceShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSourceShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/AbstractPipeline.getSourceShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "exactOutputSizeIfKnown", descriptor = "(Ljava/util/Spliterator;)J", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;)J")]
        pub fn exactOutputSizeIfKnown(&self, spliterator: Object) -> Result<i64> {
            panic!("stub: java/util/stream/AbstractPipeline.exactOutputSizeIfKnown:(Ljava/util/Spliterator;)J")
        }

        #[java_method(name = "exactOutputSize", descriptor = "(J)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exactOutputSize(&self, previousSize: i64) -> Result<i64> {
            panic!("stub: java/util/stream/AbstractPipeline.exactOutputSize:(J)J")
        }

        #[java_method(name = "wrapAndCopyInto", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Ljava/util/stream/Sink;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;S::Ljava/util/stream/Sink<TE_OUT;>;>(TS;Ljava/util/Spliterator<TP_IN;>;)TS;")]
        pub fn wrapAndCopyInto(&self, sink: S, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.wrapAndCopyInto:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "copyInto", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TP_IN;>;Ljava/util/Spliterator<TP_IN;>;)V")]
        pub fn copyInto(&self, wrappedSink: Object, spliterator: Object) -> Result<()> {
            panic!("stub: java/util/stream/AbstractPipeline.copyInto:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)V")
        }

        #[java_method(name = "copyIntoWithCancel", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TP_IN;>;Ljava/util/Spliterator<TP_IN;>;)Z")]
        pub fn copyIntoWithCancel(&self, wrappedSink: Object, spliterator: Object) -> Result<bool> {
            panic!("stub: java/util/stream/AbstractPipeline.copyIntoWithCancel:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Z")
        }

        #[java_method(name = "getStreamAndOpFlags", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStreamAndOpFlags(&self) -> Result<i32> {
            panic!("stub: java/util/stream/AbstractPipeline.getStreamAndOpFlags:()I")
        }

        #[java_method(name = "isOrdered", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOrdered(&self) -> Result<bool> {
            panic!("stub: java/util/stream/AbstractPipeline.isOrdered:()Z")
        }

        #[java_method(name = "wrapSink", descriptor = "(Ljava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TE_OUT;>;)Ljava/util/stream/Sink<TP_IN;>;")]
        pub fn wrapSink(&self, sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.wrapSink:(Ljava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "wrapSpliterator", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn wrapSpliterator(&self, sourceSpliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.wrapSpliterator:(Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "evaluate", descriptor = "(Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;ZLjava/util/function/IntFunction<[TE_OUT;>;)Ljava/util/stream/Node<TE_OUT;>;")]
        pub fn evaluate_splite_z_intfun(&self, spliterator: Object, flatten: bool, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.evaluate:(Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "getOutputShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getOutputShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/AbstractPipeline.getOutputShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "evaluateToNode", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TE_OUT;>;Ljava/util/Spliterator<TP_IN;>;ZLjava/util/function/IntFunction<[TE_OUT;>;)Ljava/util/stream/Node<TE_OUT;>;")]
        pub fn evaluateToNode(&self, arg0: PipelineHelper<E_OUT>, arg1: Object, arg2: bool, arg3: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.evaluateToNode:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "wrap", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TE_OUT;>;Ljava/util/function/Supplier<Ljava/util/Spliterator<TP_IN;>;>;Z)Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn wrap(&self, arg0: PipelineHelper<E_OUT>, arg1: Object, arg2: bool) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.wrap:(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;")
        }

        #[java_method(name = "lazySpliterator", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/Spliterator;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<TE_OUT;>;>;)Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn lazySpliterator(&self, arg0: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.lazySpliterator:(Ljava/util/function/Supplier;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEachWithCancel", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TE_OUT;>;Ljava/util/stream/Sink<TE_OUT;>;)Z")]
        pub fn forEachWithCancel(&self, arg0: Object, arg1: Object) -> Result<bool> {
            panic!("stub: java/util/stream/AbstractPipeline.forEachWithCancel:(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z")
        }

        #[java_method(name = "makeNodeBuilder", descriptor = "(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(JLjava/util/function/IntFunction<[TE_OUT;>;)Ljava/util/stream/Node$Builder<TE_OUT;>;")]
        pub fn makeNodeBuilder(&self, arg0: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.makeNodeBuilder:(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;")
        }

        #[java_method(name = "opIsStateful", descriptor = "()Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn opIsStateful(&self) -> Result<bool> {
            panic!("stub: java/util/stream/AbstractPipeline.opIsStateful:()Z")
        }

        #[java_method(name = "opWrapSink", descriptor = "(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(ILjava/util/stream/Sink<TE_OUT;>;)Ljava/util/stream/Sink<TE_IN;>;")]
        pub fn opWrapSink(&self, arg0: i32, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.opWrapSink:(ILjava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "opEvaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TE_OUT;>;Ljava/util/Spliterator<TP_IN;>;Ljava/util/function/IntFunction<[TE_OUT;>;)Ljava/util/stream/Node<TE_OUT;>;")]
        pub fn opEvaluateParallel(&self, helper: PipelineHelper<E_OUT>, spliterator: Object, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.opEvaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;Ljava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "opEvaluateParallelLazy", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TE_OUT;>;Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TE_OUT;>;")]
        pub fn opEvaluateParallelLazy(&self, helper: PipelineHelper<E_OUT>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/AbstractPipeline.opEvaluateParallelLazy:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }
    }
}
