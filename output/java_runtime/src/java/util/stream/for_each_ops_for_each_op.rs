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
    #[binary_name       = "java/util/stream/ForEachOps$ForEachOp"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/stream/TerminalOp,java/util/stream/TerminalSink"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/stream/TerminalOp<TT;Ljava/lang/Void;>;Ljava/util/stream/TerminalSink<TT;Ljava/lang/Void;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ForEachOps.java"]
    #[inner_classes     = "java/util/stream/ForEachOps$ForEachOp:java/util/stream/ForEachOps:ForEachOp:1032;java/util/stream/ForEachOps$ForEachOrderedTask:java/util/stream/ForEachOps:ForEachOrderedTask:24;java/util/stream/ForEachOps$ForEachTask:java/util/stream/ForEachOps:ForEachTask:24;java/util/stream/ForEachOps$ForEachOp$OfDouble:java/util/stream/ForEachOps$ForEachOp:OfDouble:24;java/util/stream/ForEachOps$ForEachOp$OfLong:java/util/stream/ForEachOps$ForEachOp:OfLong:24;java/util/stream/ForEachOps$ForEachOp$OfInt:java/util/stream/ForEachOps$ForEachOp:OfInt:24;java/util/stream/ForEachOps$ForEachOp$OfRef:java/util/stream/ForEachOps$ForEachOp:OfRef:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ForEachOps$ForEachOp;java/util/stream/TerminalOp;java/util/stream/TerminalSink"]

    pub struct ForEachOps_ForEachOp<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "ordered", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub ordered: bool,
    }

    impl<T> ForEachOps_ForEachOp<T> {
        #[java_method(name = "<init>", descriptor = "(Z)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut ordered: bool) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_ordered(ordered);
            Ok(this)
        }

        #[java_method(name = "getOpFlags", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOpFlags(&self) -> Result<i32> {
            panic!("stub: java/util/stream/ForEachOps$ForEachOp.getOpFlags:()I")
        }

        #[java_method(name = "evaluateSequential", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Void;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<S:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TS;>;)Ljava/lang/Void;")]
        pub fn evaluateSequential(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps$ForEachOp.evaluateSequential:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Void;")
        }

        #[java_method(name = "evaluateParallel", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Void;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<S:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TT;>;Ljava/util/Spliterator<TS;>;)Ljava/lang/Void;")]
        pub fn evaluateParallel(&self, helper: PipelineHelper<T>, spliterator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps$ForEachOp.evaluateParallel:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;)Ljava/lang/Void;")
        }

        #[java_method(name = "get", descriptor = "()Ljava/lang/Void;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ForEachOps$ForEachOp.get:()Ljava/lang/Void;")
        }
    }
}
