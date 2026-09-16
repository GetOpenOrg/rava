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
    #[binary_name       = "java/util/stream/StreamSupport"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StreamSupport.java"]
    #[inner_classes     = "java/util/stream/ReferencePipeline$Head:java/util/stream/ReferencePipeline:Head:8;java/util/stream/IntPipeline$Head:java/util/stream/IntPipeline:Head:8;java/util/stream/LongPipeline$Head:java/util/stream/LongPipeline:Head:8;java/util/stream/DoublePipeline$Head:java/util/stream/DoublePipeline:Head:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/Spliterator$OfLong:java/util/Spliterator:OfLong:1545;java/util/Spliterator$OfDouble:java/util/Spliterator:OfDouble:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/StreamSupport"]

    pub struct StreamSupport;

    impl StreamSupport {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/StreamSupport.<init>:()V")
        }

        #[java_method(name = "stream", descriptor = "(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Spliterator<TT;>;Z)Ljava/util/stream/Stream<TT;>;")]
        // java: stream(Ljava/util/Spliterator;Z)Ljava/util/stream/Stream;
        pub fn stream_splite_z(mut spliterator: Object, mut parallel: bool) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&spliterator))?;
            let _t1: i32 = StreamOpFlag::fromCharacteristics_splite(Clone::clone(&spliterator))?;
            Ok(Object::from_any(ReferencePipeline_Head::<Object, Object>::new_splite_i_z(Clone::clone(&spliterator), _t1, parallel)?.clone()))
        }

        #[java_method(name = "stream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/Supplier<+Ljava/util/Spliterator<TT;>;>;IZ)Ljava/util/stream/Stream<TT;>;")]
        pub fn stream_suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.stream:(Ljava/util/function/Supplier;IZ)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "intStream", descriptor = "(Ljava/util/Spliterator$OfInt;Z)Ljava/util/stream/IntStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: intStream(Ljava/util/Spliterator$OfInt;Z)Ljava/util/stream/IntStream;
        pub fn intStream_splite_z(mut spliterator: Object, mut parallel: bool) -> Result<Object> {
            let _t0: i32 = StreamOpFlag::fromCharacteristics_splite(Clone::clone(&spliterator))?;
            Ok(Object::from_any(IntPipeline_Head::<Object>::new_splite_i_z(Clone::clone(&spliterator), _t0, parallel)?.clone()))
        }

        #[java_method(name = "intStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/IntStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator$OfInt;>;IZ)Ljava/util/stream/IntStream;")]
        pub fn intStream_suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.intStream:(Ljava/util/function/Supplier;IZ)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "longStream", descriptor = "(Ljava/util/Spliterator$OfLong;Z)Ljava/util/stream/LongStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longStream_splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.longStream:(Ljava/util/Spliterator$OfLong;Z)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/LongStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator$OfLong;>;IZ)Ljava/util/stream/LongStream;")]
        pub fn longStream_suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.longStream:(Ljava/util/function/Supplier;IZ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "doubleStream", descriptor = "(Ljava/util/Spliterator$OfDouble;Z)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleStream_splite_z(spliterator: Object, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.doubleStream:(Ljava/util/Spliterator$OfDouble;Z)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubleStream", descriptor = "(Ljava/util/function/Supplier;IZ)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator$OfDouble;>;IZ)Ljava/util/stream/DoubleStream;")]
        pub fn doubleStream_suppli_i_z(supplier: Object, characteristics: i32, parallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/StreamSupport.doubleStream:(Ljava/util/function/Supplier;IZ)Ljava/util/stream/DoubleStream;")
        }
    }
}
