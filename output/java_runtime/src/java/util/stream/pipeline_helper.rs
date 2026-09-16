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
    #[binary_name       = "java/util/stream/PipelineHelper"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<P_OUT:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "PipelineHelper.java"]
    #[inner_classes     = "java/util/stream/Node$Builder:java/util/stream/Node:Builder:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/PipelineHelper"]

    pub struct PipelineHelper<P_OUT: Clone + Default + 'static>;

    impl<P_OUT> PipelineHelper<P_OUT> {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "getSourceShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getSourceShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/PipelineHelper.getSourceShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "getStreamAndOpFlags", descriptor = "()I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getStreamAndOpFlags(&self) -> Result<i32> {
            panic!("stub: java/util/stream/PipelineHelper.getStreamAndOpFlags:()I")
        }

        #[java_method(name = "exactOutputSizeIfKnown", descriptor = "(Ljava/util/Spliterator;)J", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;)J")]
        pub fn exactOutputSizeIfKnown(&self, arg0: Object) -> Result<i64> {
            panic!("stub: java/util/stream/PipelineHelper.exactOutputSizeIfKnown:(Ljava/util/Spliterator;)J")
        }

        #[java_method(name = "wrapAndCopyInto", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Ljava/util/stream/Sink;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;S::Ljava/util/stream/Sink<TP_OUT;>;>(TS;Ljava/util/Spliterator<TP_IN;>;)TS;")]
        pub fn wrapAndCopyInto(&self, arg0: Object, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/PipelineHelper.wrapAndCopyInto:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "copyInto", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TP_IN;>;Ljava/util/Spliterator<TP_IN;>;)V")]
        pub fn copyInto(&self, arg0: Object, arg1: Object) -> Result<()> {
            panic!("stub: java/util/stream/PipelineHelper.copyInto:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)V")
        }

        #[java_method(name = "copyIntoWithCancel", descriptor = "(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TP_IN;>;Ljava/util/Spliterator<TP_IN;>;)Z")]
        pub fn copyIntoWithCancel(&self, arg0: Object, arg1: Object) -> Result<bool> {
            panic!("stub: java/util/stream/PipelineHelper.copyIntoWithCancel:(Ljava/util/stream/Sink;Ljava/util/Spliterator;)Z")
        }

        #[java_method(name = "wrapSink", descriptor = "(Ljava/util/stream/Sink;)Ljava/util/stream/Sink;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/Sink<TP_OUT;>;)Ljava/util/stream/Sink<TP_IN;>;")]
        pub fn wrapSink(&self, arg0: Object) -> Result<Object> {
            panic!("stub: java/util/stream/PipelineHelper.wrapSink:(Ljava/util/stream/Sink;)Ljava/util/stream/Sink;")
        }

        #[java_method(name = "wrapSpliterator", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Spliterator;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;)Ljava/util/Spliterator<TP_OUT;>;")]
        pub fn wrapSpliterator(&self, arg0: Object) -> Result<Object> {
            panic!("stub: java/util/stream/PipelineHelper.wrapSpliterator:(Ljava/util/Spliterator;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "makeNodeBuilder", descriptor = "(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(JLjava/util/function/IntFunction<[TP_OUT;>;)Ljava/util/stream/Node$Builder<TP_OUT;>;")]
        pub fn makeNodeBuilder(&self, arg0: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/PipelineHelper.makeNodeBuilder:(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;")
        }

        #[java_method(name = "evaluate", descriptor = "(Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/Spliterator<TP_IN;>;ZLjava/util/function/IntFunction<[TP_OUT;>;)Ljava/util/stream/Node<TP_OUT;>;")]
        pub fn evaluate(&self, arg0: Object, arg1: bool, arg2: Object) -> Result<Object> {
            panic!("stub: java/util/stream/PipelineHelper.evaluate:(Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }
    }
}
