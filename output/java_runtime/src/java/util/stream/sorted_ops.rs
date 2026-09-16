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
    #[binary_name       = "java/util/stream/SortedOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "SortedOps.java"]
    #[inner_classes     = "java/util/stream/SortedOps$OfRef:java/util/stream/SortedOps:OfRef:26;java/util/stream/SortedOps$OfInt:java/util/stream/SortedOps:OfInt:26;java/util/stream/SortedOps$OfLong:java/util/stream/SortedOps:OfLong:26;java/util/stream/SortedOps$OfDouble:java/util/stream/SortedOps:OfDouble:26;java/util/stream/SortedOps$DoubleSortingSink:java/util/stream/SortedOps:DoubleSortingSink:26;java/util/stream/SortedOps$SizedDoubleSortingSink:java/util/stream/SortedOps:SizedDoubleSortingSink:26;java/util/stream/SortedOps$AbstractDoubleSortingSink:java/util/stream/SortedOps:AbstractDoubleSortingSink:1034;java/util/stream/SortedOps$LongSortingSink:java/util/stream/SortedOps:LongSortingSink:26;java/util/stream/SortedOps$SizedLongSortingSink:java/util/stream/SortedOps:SizedLongSortingSink:26;java/util/stream/SortedOps$AbstractLongSortingSink:java/util/stream/SortedOps:AbstractLongSortingSink:1034;java/util/stream/SortedOps$IntSortingSink:java/util/stream/SortedOps:IntSortingSink:26;java/util/stream/SortedOps$SizedIntSortingSink:java/util/stream/SortedOps:SizedIntSortingSink:26;java/util/stream/SortedOps$AbstractIntSortingSink:java/util/stream/SortedOps:AbstractIntSortingSink:1034;java/util/stream/SortedOps$RefSortingSink:java/util/stream/SortedOps:RefSortingSink:26;java/util/stream/SortedOps$SizedRefSortingSink:java/util/stream/SortedOps:SizedRefSortingSink:26;java/util/stream/SortedOps$AbstractRefSortingSink:java/util/stream/SortedOps:AbstractRefSortingSink:1034"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/SortedOps"]

    pub struct SortedOps;

    impl SortedOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/SortedOps.<init>:()V")
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*TT;*>;)Ljava/util/stream/Stream<TT;>;")]
        // java: makeRef(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/Stream;
        pub fn makeRef_abstra(mut upstream: AbstractPipeline<Object, Object, Object>) -> Result<Object> {
            Ok(Object::from_any(SortedOps_OfRef::<Object>::new_abstra(Clone::clone(&upstream))?.clone()))
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/stream/AbstractPipeline;Ljava/util/Comparator;)Ljava/util/stream/Stream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*TT;*>;Ljava/util/Comparator<-TT;>;)Ljava/util/stream/Stream<TT;>;")]
        pub fn makeRef_abstra_compar(upstream: AbstractPipeline<Object, Object, Object>, comparator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps.makeRef:(Ljava/util/stream/AbstractPipeline;Ljava/util/Comparator;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/IntStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Integer;*>;)Ljava/util/stream/IntStream;")]
        pub fn makeInt(upstream: AbstractPipeline<Object, Object, Object>) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps.makeInt:(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/LongStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Long;*>;)Ljava/util/stream/LongStream;")]
        pub fn makeLong(upstream: AbstractPipeline<Object, Object, Object>) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps.makeLong:(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/DoubleStream;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/stream/AbstractPipeline<*Ljava/lang/Double;*>;)Ljava/util/stream/DoubleStream;")]
        pub fn makeDouble(upstream: AbstractPipeline<Object, Object, Object>) -> Result<Object> {
            panic!("stub: java/util/stream/SortedOps.makeDouble:(Ljava/util/stream/AbstractPipeline;)Ljava/util/stream/DoubleStream;")
        }
    }
}
