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

impl<P_IN: Clone + Default + 'static, P_OUT: Clone + Default + 'static> From<ReferencePipeline<P_IN, P_OUT>> for AbstractPipeline<P_IN, P_OUT, Object> {
    fn from(v: ReferencePipeline<P_IN, P_OUT>) -> AbstractPipeline<P_IN, P_OUT, Object> { v.__into_super() }
}

impl<P_IN: Clone + Default + 'static, P_OUT: Clone + Default + 'static> From<ReferencePipeline<P_IN, P_OUT>> for PipelineHelper<P_IN> {
    fn from(v: ReferencePipeline<P_IN, P_OUT>) -> PipelineHelper<P_IN> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/ReferencePipeline"]
    #[super_class       = "java/util/stream/AbstractPipeline"]
    #[interfaces        = "java/util/stream/Stream"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<P_IN:Ljava/lang/Object;P_OUT:Ljava/lang/Object;>Ljava/util/stream/AbstractPipeline<TP_IN;TP_OUT;Ljava/util/stream/Stream<TP_OUT;>;>;Ljava/util/stream/Stream<TP_OUT;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReferencePipeline.java"]
    #[inner_classes     = "java/util/stream/StreamSpliterators$WrappingSpliterator:java/util/stream/StreamSpliterators:WrappingSpliterator:24;java/util/stream/StreamSpliterators$DelegatingSpliterator:java/util/stream/StreamSpliterators:DelegatingSpliterator:8;java/util/stream/Node$Builder:java/util/stream/Node:Builder:1545;java/util/stream/ReferencePipeline$1:::0;java/util/stream/ReferencePipeline$2:::0;java/util/stream/ReferencePipeline$3:::0;java/util/stream/ReferencePipeline$4:::0;java/util/stream/ReferencePipeline$5:::0;java/util/stream/ReferencePipeline$6:::0;java/util/stream/ReferencePipeline$7:::0;java/util/stream/ReferencePipeline$8:::0;java/util/stream/ReferencePipeline$9:::0;java/util/stream/ReferencePipeline$10:::0;java/util/stream/ReferencePipeline$11:::0;java/util/stream/ReferencePipeline$12:::0;java/util/stream/ReferencePipeline$13:::0;java/util/stream/ReferencePipeline$14:::0;java/util/stream/ReferencePipeline$15:::0;java/util/stream/MatchOps$MatchKind:java/util/stream/MatchOps:MatchKind:16408;java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/util/stream/ReferencePipeline$StatefulOp:java/util/stream/ReferencePipeline:StatefulOp:1032;java/util/stream/ReferencePipeline$StatelessOp:java/util/stream/ReferencePipeline:StatelessOp:1032;java/util/stream/ReferencePipeline$Head:java/util/stream/ReferencePipeline:Head:8;java/util/stream/ReferencePipeline$15$1:::0;java/util/stream/ReferencePipeline$14$1:::0;java/util/stream/ReferencePipeline$13$1:::0;java/util/stream/ReferencePipeline$12$1:::0;java/util/stream/ReferencePipeline$11$1:::0;java/util/stream/ReferencePipeline$10$1:::0;java/util/stream/ReferencePipeline$9$1:::0;java/util/stream/ReferencePipeline$8$1:::0;java/util/stream/ReferencePipeline$7$1:::0;java/util/stream/ReferencePipeline$6$1:::0;java/util/stream/ReferencePipeline$5$1:::0;java/util/stream/ReferencePipeline$4$1:::0;java/util/stream/ReferencePipeline$3$1:::0;java/util/stream/ReferencePipeline$2$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractPipeline<P_IN, P_OUT, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/PipelineHelper;java/util/stream/ReferencePipeline;java/util/stream/Stream"]

    pub struct ReferencePipeline<P_IN: Clone + Default + 'static, P_OUT: Clone + Default + 'static>;

    impl<P_IN, P_OUT> ReferencePipeline<P_IN, P_OUT> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<*>;>;IZ)V")]
        pub fn new_suppli_i_z(source: Object, sourceFlags: i32, parallel: bool) -> Result<Self> {
            panic!("stub: java/util/stream/ReferencePipeline.<init>:(Ljava/util/function/Supplier;IZ)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<*>;IZ)V")]
        // java: <init>(Ljava/util/Spliterator;IZ)V
        pub fn new_splite_i_z(mut source: Object, mut sourceFlags: i32, mut parallel: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractPipeline::new_splite_i_z(Clone::clone(&source), sourceFlags, parallel)?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TP_IN;*>;I)V")]
        // java: <init>(Ljava/util/stream/AbstractPipeline;I)V
        pub fn new_abstra_i(mut upstream: AbstractPipeline<Object, P_IN, Object>, mut opFlags: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractPipeline::new_abstra_i(Clone::clone(&upstream), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "getOutputShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOutputShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/ReferencePipeline.getOutputShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "evaluateToNode", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TP_OUT;>;Ljava/util/Spliterator<TP_IN;>;ZLjava/util/function/IntFunction<[TP_OUT;>;)Ljava/util/stream/Node<TP_OUT;>;")]
        pub fn evaluateToNode(&self, helper: PipelineHelper<P_OUT>, spliterator: Object, flattenTree: bool, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.evaluateToNode:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "wrap", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<TP_OUT;>;Ljava/util/function/Supplier<Ljava/util/Spliterator<TP_IN;>;>;Z)Ljava/util/Spliterator<TP_OUT;>;")]
        pub fn wrap(&self, ph: PipelineHelper<P_OUT>, supplier: Object, isParallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.wrap:(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;")
        }

        #[java_method(name = "lazySpliterator", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<TP_OUT;>;>;)Ljava/util/Spliterator<TP_OUT;>;")]
        pub fn lazySpliterator(&self, supplier: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.lazySpliterator:(Ljava/util/function/Supplier;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEachWithCancel", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<TP_OUT;>;Ljava/util/stream/Sink<TP_OUT;>;)Z")]
        pub fn forEachWithCancel(&self, spliterator: Object, sink: Object) -> Result<bool> {
            panic!("stub: java/util/stream/ReferencePipeline.forEachWithCancel:(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z")
        }

        #[java_method(name = "makeNodeBuilder", descriptor = "(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/IntFunction<[TP_OUT;>;)Ljava/util/stream/Node$Builder<TP_OUT;>;")]
        pub fn makeNodeBuilder(&self, exactSizeIfKnown: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.makeNodeBuilder:(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TP_OUT;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "unordered", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn unordered(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.unordered:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "filter", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn filter(&self, mut predicate: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&predicate))?;
            Ok(Object::from_any(ReferencePipeline_2::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::REFERENCE()), StreamOpFlag::NOT_SIZED(), Clone::clone(&predicate))?.clone()))
        }

        #[java_method(name = "map", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-TP_OUT;+TR;>;)Ljava/util/stream/Stream<TR;>;")]
        pub fn map(&self, mut mapper: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&mapper))?;
            Ok(Object::from_any(ReferencePipeline_3::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::REFERENCE()), (StreamOpFlag::NOT_SORTED()|StreamOpFlag::NOT_DISTINCT()), Clone::clone(&mapper))?.clone()))
        }

        #[java_method(name = "mapToInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToIntFunction<-TP_OUT;>;)Ljava/util/stream/IntStream;")]
        pub fn mapToInt(&self, mut mapper: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&mapper))?;
            Ok(Object::from_any(ReferencePipeline_4::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::REFERENCE()), (StreamOpFlag::NOT_SORTED()|StreamOpFlag::NOT_DISTINCT()), Clone::clone(&mapper))?.clone()))
        }

        #[java_method(name = "mapToLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToLongFunction<-TP_OUT;>;)Ljava/util/stream/LongStream;")]
        pub fn mapToLong(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapToLong:(Ljava/util/function/ToLongFunction;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "mapToDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToDoubleFunction<-TP_OUT;>;)Ljava/util/stream/DoubleStream;")]
        pub fn mapToDouble(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapToDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "flatMap", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-TP_OUT;+Ljava/util/stream/Stream<+TR;>;>;)Ljava/util/stream/Stream<TR;>;")]
        pub fn flatMap(&self, mut mapper: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&mapper))?;
            Ok(Object::from_any(ReferencePipeline_7::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::REFERENCE()), ((StreamOpFlag::NOT_SORTED()|StreamOpFlag::NOT_DISTINCT())|StreamOpFlag::NOT_SIZED()), Clone::clone(&mapper))?.clone()))
        }

        #[java_method(name = "flatMapToInt", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TP_OUT;+Ljava/util/stream/IntStream;>;)Ljava/util/stream/IntStream;")]
        pub fn flatMapToInt(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.flatMapToInt:(Ljava/util/function/Function;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "flatMapToDouble", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TP_OUT;+Ljava/util/stream/DoubleStream;>;)Ljava/util/stream/DoubleStream;")]
        pub fn flatMapToDouble(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.flatMapToDouble:(Ljava/util/function/Function;)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "flatMapToLong", descriptor = "(Ljava/util/function/Function;)Ljava/util/stream/LongStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Function<-TP_OUT;+Ljava/util/stream/LongStream;>;)Ljava/util/stream/LongStream;")]
        pub fn flatMapToLong(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.flatMapToLong:(Ljava/util/function/Function;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "mapMulti", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/BiConsumer<-TP_OUT;-Ljava/util/function/Consumer<TR;>;>;)Ljava/util/stream/Stream<TR;>;")]
        pub fn mapMulti(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapMulti:(Ljava/util/function/BiConsumer;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "mapMultiToInt", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TP_OUT;-Ljava/util/function/IntConsumer;>;)Ljava/util/stream/IntStream;")]
        pub fn mapMultiToInt(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapMultiToInt:(Ljava/util/function/BiConsumer;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "mapMultiToLong", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TP_OUT;-Ljava/util/function/LongConsumer;>;)Ljava/util/stream/LongStream;")]
        pub fn mapMultiToLong(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapMultiToLong:(Ljava/util/function/BiConsumer;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "mapMultiToDouble", descriptor = "(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BiConsumer<-TP_OUT;-Ljava/util/function/DoubleConsumer;>;)Ljava/util/stream/DoubleStream;")]
        pub fn mapMultiToDouble(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.mapMultiToDouble:(Ljava/util/function/BiConsumer;)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "peek", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TP_OUT;>;)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn peek(&self, mut action: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&action))?;
            Ok(Object::from_any(ReferencePipeline_15::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::REFERENCE()), 0i32, Clone::clone(&action))?.clone()))
        }

        #[java_method(name = "distinct", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn distinct(&self) -> Result<Object> {
            let this = self;
            let _t0: ReferencePipeline<Object, Object> = DistinctOps::makeRef(Clone::clone(&this).into())?;
            Ok(Object::from_any(_t0.clone()))
        }

        #[java_method(name = "sorted", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TP_OUT;>;")]
        // java: sorted()Ljava/util/stream/Stream;
        pub fn sorted(&self) -> Result<Object> {
            let this = self;
            let _t0: Object = SortedOps::makeRef_abstra(Clone::clone(&this).into())?;
            Ok(_t0)
        }

        #[java_method(name = "sorted", descriptor = "(Ljava/util/Comparator;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TP_OUT;>;)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn sorted_compar(&self, comparator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.sorted:(Ljava/util/Comparator;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "limit", descriptor = "(J)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(J)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn limit(&self, mut maxSize: i64) -> Result<Object> {
            let this = self;
            if (((maxSize>(0i64)) as i32-((maxSize)<(0i64)) as i32)<0) {
                let _t0: String = Long::toString_l(maxSize)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: Object = SliceOps::makeRef(Clone::clone(&this).into(), 0i64, maxSize)?;
            Ok(_t0)
        }

        #[java_method(name = "skip", descriptor = "(J)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(J)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn skip(&self, n: i64) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.skip:(J)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "takeWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn takeWhile(&self, mut predicate: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = WhileOps::makeTakeWhileRef(Clone::clone(&this).into(), Clone::clone(&predicate))?;
            Ok(_t0)
        }

        #[java_method(name = "dropWhile", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Ljava/util/stream/Stream<TP_OUT;>;")]
        pub fn dropWhile(&self, mut predicate: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = WhileOps::makeDropWhileRef(Clone::clone(&this).into(), Clone::clone(&predicate))?;
            Ok(_t0)
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TP_OUT;>;)V")]
        pub fn forEach(&self, mut action: Object) -> Result<()> {
            let this = self;
            let _t0: Object = ForEachOps::makeRef(Clone::clone(&action), (0i32 != 0i32))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok(())
        }

        #[java_method(name = "forEachOrdered", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TP_OUT;>;)V")]
        pub fn forEachOrdered(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/ReferencePipeline.forEachOrdered:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TA;>;)[TA;")]
        pub fn toArray_intfun(&self, generator: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/stream/ReferencePipeline.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/stream/ReferencePipeline.toArray:()[Ljava/lang/Object;")
        }

        #[java_method(name = "toList", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<TP_OUT;>;")]
        pub fn toList(&self) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.toList:()Ljava/util/List;")
        }

        #[java_method(name = "anyMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Z")]
        pub fn anyMatch(&self, mut predicate: Object) -> Result<bool> {
            let this = self;
            let _t0: Object = MatchOps::makeRef(Clone::clone(&predicate), Clone::clone(&MatchOps_MatchKind::ANY()))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok((_t1).downcast::<bool>())
        }

        #[java_method(name = "allMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Z")]
        pub fn allMatch(&self, mut predicate: Object) -> Result<bool> {
            let this = self;
            let _t0: Object = MatchOps::makeRef(Clone::clone(&predicate), Clone::clone(&MatchOps_MatchKind::ALL()))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok((_t1).downcast::<bool>())
        }

        #[java_method(name = "noneMatch", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TP_OUT;>;)Z")]
        pub fn noneMatch(&self, mut predicate: Object) -> Result<bool> {
            let this = self;
            let _t0: Object = MatchOps::makeRef(Clone::clone(&predicate), Clone::clone(&MatchOps_MatchKind::NONE()))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok((_t1).downcast::<bool>())
        }

        #[java_method(name = "findFirst", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<TP_OUT;>;")]
        pub fn findFirst(&self) -> Result<Optional<P_OUT>> {
            let this = self;
            let _t0: Object = FindOps::makeRef((1i32 != 0i32))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok(Default::default())
        }

        #[java_method(name = "findAny", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<TP_OUT;>;")]
        pub fn findAny(&self) -> Result<Optional<Object>> {
            panic!("stub: java/util/stream/ReferencePipeline.findAny:()Ljava/util/Optional;")
        }

        #[java_method(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TP_OUT;Ljava/util/function/BinaryOperator<TP_OUT;>;)TP_OUT;")]
        // java: reduce(Ljava/lang/Object;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;
        pub fn reduce_obj_binary(&self, mut identity: P_OUT, mut accumulator: Object) -> Result<P_OUT> {
            let this = self;
            let _t0: Object = ReduceOps::makeRef_obj_bifunc_binary(Clone::clone(&identity), Clone::clone(&accumulator), Clone::clone(&accumulator))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok(panic!("null"))
        }

        #[java_method(name = "reduce", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/BinaryOperator<TP_OUT;>;)Ljava/util/Optional<TP_OUT;>;")]
        // java: reduce(Ljava/util/function/BinaryOperator;)Ljava/util/Optional;
        pub fn reduce_binary(&self, mut accumulator: Object) -> Result<Optional<P_OUT>> {
            let this = self;
            let _t0: Object = ReduceOps::makeRef_binary(Clone::clone(&accumulator))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok(Default::default())
        }

        #[java_method(name = "reduce", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(TR;Ljava/util/function/BiFunction<TR;-TP_OUT;TR;>;Ljava/util/function/BinaryOperator<TR;>;)TR;")]
        pub fn reduce_obj_bifunc_binary(&self, identity: Object, accumulator: Object, combiner: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.reduce:(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/lang/Object;")
        }

        #[java_method(name = "collect", descriptor = "(Ljava/util/stream/Collector;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;A:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TP_OUT;TA;TR;>;)TR;")]
        // java: collect(Ljava/util/stream/Collector;)Ljava/lang/Object;
        pub fn collect_collec(&self, mut collector: Object) -> Result<Object> {
            let this = self;
            let _t0 = this.__super().isParallel()?;
        let mut container: Object = Default::default();
            if _t0 {
                let _vdispatch1: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.characteristics()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch2: bool = if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<Object>() { _d.contains(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else if let Some(__f) = _vdispatch1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(Collector_Characteristics::CONCURRENT().clone()))? } else { Default::default() };
        container = Default::default();
                if _vdispatch2 {
                    let _t3 = this.__super().isOrdered()?;
                    let _vdispatch4: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.characteristics()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                    let _vdispatch5: bool = if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<HashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(_d) = _vdispatch4.0.as_any().downcast_ref::<Object>() { _d.contains(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else if let Some(__f) = _vdispatch4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(Collector_Characteristics::UNORDERED().clone()))? } else { Default::default() };
                    if _vdispatch5 {
                        let _vdispatch6: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.supplier()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.supplier()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        let _vdispatch7: Object = if let Some(__f) = _vdispatch6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        container = _vdispatch7;
                        let _vdispatch8: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.accumulator()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.accumulator()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        let mut accumulator = (_vdispatch8).downcast::<BiConsumer<Object, P_OUT>>();
                        let __lam_cap309_0 = container;
                        let __lam_cap309_1 = accumulator;
                        let __lam_309: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<()> { ReferencePipeline::lambda_collect_1(__lam_cap309_0.clone(), __lam_cap309_1.clone(), _la0) });
                        this.forEach(Clone::clone(&Object::from_any(__lam_309)))?;
                    } else {
                        let _t6: Object = ReduceOps::makeRef_collec(Clone::clone(&collector))?;
                        let _t7 = this.__super().evaluate(Clone::clone(&_t6))?;
                        container = _t7;
                    }
                } else {
                    let _t3: Object = ReduceOps::makeRef_collec(Clone::clone(&collector))?;
                    let _t4 = this.__super().evaluate(Clone::clone(&_t3))?;
                    container = _t4;
                }
            } else {
                let _t1: Object = ReduceOps::makeRef_collec(Clone::clone(&collector))?;
                let _t2 = this.__super().evaluate(Clone::clone(&_t1))?;
                container = _t2;
            }
            let _vdispatch1: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.characteristics()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.characteristics()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let _vdispatch2: bool = if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<HashSet<Object>>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(_d) = _vdispatch1.0.as_any().downcast_ref::<Object>() { _d.contains(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else if let Some(__f) = _vdispatch1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(Collector_Characteristics::IDENTITY_FINISH().clone()))? } else { Default::default() };
            let mut _merged5: Object;
            if _vdispatch2 {
                _merged5 = container;
            } else {
                let _vdispatch3: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.finisher()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.finisher()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _vdispatch4: Object = if let Some(__f) = _vdispatch3.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&container))? } else { Default::default() };
                _merged5 = _vdispatch4;
            }
            Ok(_merged5)
        }

        #[java_method(name = "collect", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/BiConsumer<TR;-TP_OUT;>;Ljava/util/function/BiConsumer<TR;TR;>;)TR;")]
        pub fn collect_suppli_bicons_bicons(&self, supplier: Object, accumulator: Object, combiner: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReferencePipeline.collect:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;")
        }

        #[java_method(name = "max", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TP_OUT;>;)Ljava/util/Optional<TP_OUT;>;")]
        pub fn max(&self, mut comparator: Object) -> Result<Optional<P_OUT>> {
            let this = self;
            let _t0: Object = BinaryOperator::<Object>::maxBy(Clone::clone(&comparator))?;
            let _t1 = this.reduce_binary(Clone::clone(&_t0))?;
            Ok(Default::default())
        }

        #[java_method(name = "min", descriptor = "(Ljava/util/Comparator;)Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TP_OUT;>;)Ljava/util/Optional<TP_OUT;>;")]
        pub fn min(&self, mut comparator: Object) -> Result<Optional<P_OUT>> {
            let this = self;
            let _t0: Object = BinaryOperator::<Object>::minBy(Clone::clone(&comparator))?;
            let _t1 = this.reduce_binary(Clone::clone(&_t0))?;
            Ok(Default::default())
        }

        #[java_method(name = "count", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn count(&self) -> Result<i64> {
            let this = self;
            let _t0: Object = ReduceOps::makeRefCounting()?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok((_t1).downcast::<i64>())
        }
    }
}
