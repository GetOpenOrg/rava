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
    #[binary_name       = "java/util/stream/WhileOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "WhileOps.java"]
    #[inner_classes     = "java/util/stream/WhileOps$1:::0;java/util/stream/WhileOps$2:::0;java/util/stream/WhileOps$3:::0;java/util/stream/WhileOps$4:::0;java/util/stream/WhileOps$1Op::Op:0;java/util/stream/WhileOps$2Op::Op:0;java/util/stream/WhileOps$3Op::Op:0;java/util/stream/WhileOps$4Op::Op:0;java/util/stream/WhileOps$DropWhileTask:java/util/stream/WhileOps:DropWhileTask:26;java/util/stream/WhileOps$TakeWhileTask:java/util/stream/WhileOps:TakeWhileTask:26;java/util/stream/WhileOps$UnorderedWhileSpliterator:java/util/stream/WhileOps:UnorderedWhileSpliterator:1032;java/util/stream/WhileOps$DropWhileSink:java/util/stream/WhileOps:DropWhileSink:1544;java/util/stream/WhileOps$DropWhileOp:java/util/stream/WhileOps:DropWhileOp:1544;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfDouble:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble:Dropping:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfDouble:Taking:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfLong:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong:Dropping:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfLong:Taking:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfInt:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:Dropping:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfInt:Taking:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:java/util/stream/WhileOps$UnorderedWhileSpliterator:OfRef:1032;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Dropping:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Dropping:24;java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef$Taking:java/util/stream/WhileOps$UnorderedWhileSpliterator$OfRef:Taking:24;java/util/stream/WhileOps$4Op$1OpSink::OpSink:0;java/util/stream/WhileOps$3Op$1OpSink::OpSink:0;java/util/stream/WhileOps$2Op$1OpSink::OpSink:0;java/util/stream/WhileOps$1Op$1OpSink::OpSink:0;java/util/stream/WhileOps$4$1:::0;java/util/stream/WhileOps$3$1:::0;java/util/stream/WhileOps$2$1:::0;java/util/stream/WhileOps$1$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/WhileOps"]

    pub struct WhileOps;

    impl WhileOps {
        #[cfg_attr(any(), java_field(name = "TAKE_FLAGS", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: TAKE_FLAGS:I
        pub fn TAKE_FLAGS() -> i32 {
            panic!("stub: java/util/stream/WhileOps.TAKE_FLAGS:I")
        }

        #[cfg_attr(any(), java_field(name = "DROP_FLAGS", descriptor = "I", access = "package", modifiers = "static final", is_static = true))]
        // static field: DROP_FLAGS:I
        pub fn DROP_FLAGS() -> i32 {
            panic!("stub: java/util/stream/WhileOps.DROP_FLAGS:I")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/WhileOps.<init>:()V")
        }

        #[java_method(name = "makeTakeWhileRef", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*TT;*>;Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Stream<TT;>;")]
        pub fn makeTakeWhileRef(mut upstream: AbstractPipeline<Object, Object, Object>, mut predicate: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&predicate))?;
            Ok(Object::from_any(WhileOps_1::new(Clone::clone(&upstream), Clone::clone(&StreamShape::REFERENCE()), WhileOps::TAKE_FLAGS(), Clone::clone(&predicate))?.clone()))
        }

        #[java_method(name = "makeTakeWhileInt", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Integer;*>;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")]
        pub fn makeTakeWhileInt(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeTakeWhileInt:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "makeTakeWhileLong", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Long;*>;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;")]
        pub fn makeTakeWhileLong(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeTakeWhileLong:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "makeTakeWhileDouble", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Double;*>;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;")]
        pub fn makeTakeWhileDouble(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeTakeWhileDouble:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "makeDropWhileRef", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*TT;*>;Ljava/util/function/Predicate<-TT;>;)Ljava/util/stream/Stream<TT;>;")]
        pub fn makeDropWhileRef(mut upstream: AbstractPipeline<Object, Object, Object>, mut predicate: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&predicate))?;
            Ok(Object::from_any(WhileOps_1Op::new(Clone::clone(&upstream), Clone::clone(&StreamShape::REFERENCE()), WhileOps::DROP_FLAGS(), Clone::clone(&predicate))?.clone()))
        }

        #[java_method(name = "makeDropWhileInt", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Integer;*>;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")]
        pub fn makeDropWhileInt(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeDropWhileInt:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "makeDropWhileLong", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Long;*>;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;")]
        pub fn makeDropWhileLong(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeDropWhileLong:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/LongPredicate;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "makeDropWhileDouble", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Double;*>;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;")]
        pub fn makeDropWhileDouble(upstream: AbstractPipeline<Object, Object, Object>, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/WhileOps.makeDropWhileDouble:(Ljava/util/stream/AbstractPipeline;Ljava/util/function/DoublePredicate;)Ljava/util/stream/DoubleStream;")
        }
    }
}
