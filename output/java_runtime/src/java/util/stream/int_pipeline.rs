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

impl<E_IN: Clone + Default + 'static> From<IntPipeline<E_IN>> for AbstractPipeline<E_IN, Object, Object> {
    fn from(v: IntPipeline<E_IN>) -> AbstractPipeline<E_IN, Object, Object> { v.__into_super() }
}

impl<E_IN: Clone + Default + 'static> From<IntPipeline<E_IN>> for PipelineHelper<E_IN> {
    fn from(v: IntPipeline<E_IN>) -> PipelineHelper<E_IN> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/IntPipeline"]
    #[super_class       = "java/util/stream/AbstractPipeline"]
    #[interfaces        = "java/util/stream/IntStream"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E_IN:Ljava/lang/Object;>Ljava/util/stream/AbstractPipeline<TE_IN;Ljava/lang/Integer;Ljava/util/stream/IntStream;>;Ljava/util/stream/IntStream;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IntPipeline.java"]
    #[inner_classes     = "java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/stream/Node$OfInt:java/util/stream/Node:OfInt:1545;java/util/stream/StreamSpliterators$IntWrappingSpliterator:java/util/stream/StreamSpliterators:IntWrappingSpliterator:24;java/util/stream/StreamSpliterators$DelegatingSpliterator:java/util/stream/StreamSpliterators:DelegatingSpliterator:8;java/util/stream/StreamSpliterators$DelegatingSpliterator$OfInt:java/util/stream/StreamSpliterators$DelegatingSpliterator:OfInt:24;java/util/stream/Node$Builder:java/util/stream/Node:Builder:1545;java/util/stream/Node$Builder$OfInt:java/util/stream/Node$Builder:OfInt:1545;java/util/stream/IntPipeline$1:::0;java/util/PrimitiveIterator$OfInt:java/util/PrimitiveIterator:OfInt:1545;java/util/stream/IntPipeline$2:::0;java/util/stream/IntPipeline$3:::0;java/util/stream/IntPipeline$4:::0;java/util/stream/IntPipeline$5:::0;java/util/stream/IntPipeline$6:::0;java/util/stream/IntPipeline$7:::0;java/util/stream/IntPipeline$8:::0;java/util/stream/IntStream$IntMapMultiConsumer:java/util/stream/IntStream:IntMapMultiConsumer:1545;java/util/stream/IntPipeline$9:::0;java/util/stream/IntPipeline$10:::0;java/util/stream/IntPipeline$11:::0;java/util/stream/MatchOps$MatchKind:java/util/stream/MatchOps:MatchKind:16408;java/util/stream/IntPipeline$StatefulOp:java/util/stream/IntPipeline:StatefulOp:1032;java/util/stream/IntPipeline$StatelessOp:java/util/stream/IntPipeline:StatelessOp:1032;java/util/stream/IntPipeline$Head:java/util/stream/IntPipeline:Head:8;java/util/stream/IntPipeline$11$1:::0;java/util/stream/IntPipeline$10$1:::0;java/util/stream/IntPipeline$8$1:::0;java/util/stream/IntPipeline$7$1:::0;java/util/stream/IntPipeline$6$1:::0;java/util/stream/IntPipeline$5$1:::0;java/util/stream/IntPipeline$4$1:::0;java/util/stream/IntPipeline$3$1:::0;java/util/stream/IntPipeline$2$1:::0;java/util/stream/IntPipeline$1$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractPipeline<E_IN, Object, Object>"]
    #[superclass_fields(sourceStage: AbstractPipeline<Object, Object, Object>, previousStage: AbstractPipeline<Object, Object, Object>, sourceOrOpFlags: i32, nextStage: AbstractPipeline<Object, Object, Object>, depth: i32, combinedFlags: i32, sourceSpliterator: Object, sourceSupplier: Object, linkedOrConsumed: bool, sourceAnyStateful: bool, sourceCloseAction: Object, parallel: bool)]
    #[all_supertypes    = "java/lang/Object;java/util/stream/AbstractPipeline;java/util/stream/BaseStream;java/util/stream/IntPipeline;java/util/stream/IntStream;java/util/stream/PipelineHelper"]

    pub struct IntPipeline<E_IN: Clone + Default + 'static>;

    impl<E_IN> IntPipeline<E_IN> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/function/Supplier;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<Ljava/lang/Integer;>;>;IZ)V")]
        pub fn new_suppli_i_z(source: Object, sourceFlags: i32, parallel: bool) -> Result<Self> {
            panic!("stub: java/util/stream/IntPipeline.<init>:(Ljava/util/function/Supplier;IZ)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Spliterator;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<Ljava/lang/Integer;>;IZ)V")]
        // java: <init>(Ljava/util/Spliterator;IZ)V
        pub fn new_splite_i_z(mut source: Object, mut sourceFlags: i32, mut parallel: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractPipeline::new_splite_i_z(Clone::clone(&source), sourceFlags, parallel)?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/stream/AbstractPipeline;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/AbstractPipeline<*TE_IN;*>;I)V")]
        // java: <init>(Ljava/util/stream/AbstractPipeline;I)V
        pub fn new_abstra_i(mut upstream: AbstractPipeline<Object, E_IN, Object>, mut opFlags: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractPipeline::new_abstra_i(Clone::clone(&upstream), opFlags)?);
            Ok(this)
        }

        #[java_method(name = "adapt", descriptor = "(Ljava/util/stream/Sink;)Ljava/util/function/IntConsumer;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/stream/Sink<Ljava/lang/Integer;>;)Ljava/util/function/IntConsumer;")]
        pub fn adapt_sink(sink: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.adapt:(Ljava/util/stream/Sink;)Ljava/util/function/IntConsumer;")
        }

        #[java_method(name = "adapt", descriptor = "(Ljava/util/Spliterator;)Ljava/util/Spliterator$OfInt;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<Ljava/lang/Integer;>;)Ljava/util/Spliterator$OfInt;")]
        pub fn adapt_splite(s: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.adapt:(Ljava/util/Spliterator;)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "getOutputShape", descriptor = "()Ljava/util/stream/StreamShape;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOutputShape(&self) -> Result<StreamShape> {
            panic!("stub: java/util/stream/IntPipeline.getOutputShape:()Ljava/util/stream/StreamShape;")
        }

        #[java_method(name = "evaluateToNode", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<Ljava/lang/Integer;>;Ljava/util/Spliterator<TP_IN;>;ZLjava/util/function/IntFunction<[Ljava/lang/Integer;>;)Ljava/util/stream/Node<Ljava/lang/Integer;>;")]
        pub fn evaluateToNode(&self, helper: PipelineHelper<i32>, spliterator: Object, flattenTree: bool, generator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.evaluateToNode:(Ljava/util/stream/PipelineHelper;Ljava/util/Spliterator;ZLjava/util/function/IntFunction;)Ljava/util/stream/Node;")
        }

        #[java_method(name = "wrap", descriptor = "(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P_IN:Ljava/lang/Object;>(Ljava/util/stream/PipelineHelper<Ljava/lang/Integer;>;Ljava/util/function/Supplier<Ljava/util/Spliterator<TP_IN;>;>;Z)Ljava/util/Spliterator<Ljava/lang/Integer;>;")]
        pub fn wrap(&self, ph: PipelineHelper<i32>, supplier: Object, isParallel: bool) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.wrap:(Ljava/util/stream/PipelineHelper;Ljava/util/function/Supplier;Z)Ljava/util/Spliterator;")
        }

        #[java_method(name = "lazySpliterator", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/Spliterator$OfInt;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Spliterator<Ljava/lang/Integer;>;>;)Ljava/util/Spliterator$OfInt;")]
        pub fn lazySpliterator(&self, supplier: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.lazySpliterator:(Ljava/util/function/Supplier;)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "forEachWithCancel", descriptor = "(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Spliterator<Ljava/lang/Integer;>;Ljava/util/stream/Sink<Ljava/lang/Integer;>;)Z")]
        pub fn forEachWithCancel(&self, spliterator: Object, sink: Object) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline.forEachWithCancel:(Ljava/util/Spliterator;Ljava/util/stream/Sink;)Z")
        }

        #[java_method(name = "makeNodeBuilder", descriptor = "(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/IntFunction<[Ljava/lang/Integer;>;)Ljava/util/stream/Node$Builder<Ljava/lang/Integer;>;")]
        pub fn makeNodeBuilder(&self, exactSizeIfKnown: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.makeNodeBuilder:(JLjava/util/function/IntFunction;)Ljava/util/stream/Node$Builder;")
        }

        #[java_method(name = "mapToObj", descriptor = "(Ljava/util/function/IntFunction;I)Ljava/util/stream/Stream;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/IntFunction<+TU;>;I)Ljava/util/stream/Stream<TU;>;")]
        // java: mapToObj(Ljava/util/function/IntFunction;I)Ljava/util/stream/Stream;
        pub fn mapToObj_intfun_i(&self, mut mapper: Object, mut opFlags: i32) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(IntPipeline_1::new(Clone::clone(this), Clone::clone(&this).into(), Clone::clone(&StreamShape::INT_VALUE()), opFlags, Clone::clone(&mapper))?.clone()))
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/PrimitiveIterator$OfInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.iterator:()Ljava/util/PrimitiveIterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.spliterator:()Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "asLongStream", descriptor = "()Ljava/util/stream/LongStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asLongStream(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.asLongStream:()Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "asDoubleStream", descriptor = "()Ljava/util/stream/DoubleStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asDoubleStream(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.asDoubleStream:()Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "boxed", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/lang/Integer;>;")]
        pub fn boxed(&self) -> Result<Object> {
            let this = self;
            let __lam_118: std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: i32| -> crate::error::Result<Object> { Integer::valueOf(_la0) });
            let _t0 = this.mapToObj_intfun_i(Clone::clone(&Object::from_any(__lam_118)), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "map", descriptor = "(Ljava/util/function/IntUnaryOperator;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn map(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.map:(Ljava/util/function/IntUnaryOperator;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "mapToObj", descriptor = "(Ljava/util/function/IntFunction;)Ljava/util/stream/Stream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/IntFunction<+TU;>;)Ljava/util/stream/Stream<TU;>;")]
        pub fn mapToObj_intfun(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.mapToObj:(Ljava/util/function/IntFunction;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "mapToLong", descriptor = "(Ljava/util/function/IntToLongFunction;)Ljava/util/stream/LongStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mapToLong(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.mapToLong:(Ljava/util/function/IntToLongFunction;)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "mapToDouble", descriptor = "(Ljava/util/function/IntToDoubleFunction;)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mapToDouble(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.mapToDouble:(Ljava/util/function/IntToDoubleFunction;)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "flatMap", descriptor = "(Ljava/util/function/IntFunction;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/IntFunction<+Ljava/util/stream/IntStream;>;)Ljava/util/stream/IntStream;")]
        pub fn flatMap(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.flatMap:(Ljava/util/function/IntFunction;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "mapMulti", descriptor = "(Ljava/util/stream/IntStream$IntMapMultiConsumer;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mapMulti(&self, mapper: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.mapMulti:(Ljava/util/stream/IntStream$IntMapMultiConsumer;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "unordered", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unordered(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.unordered:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "filter", descriptor = "(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn filter(&self, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.filter:(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "peek", descriptor = "(Ljava/util/function/IntConsumer;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn peek(&self, action: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.peek:(Ljava/util/function/IntConsumer;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "limit", descriptor = "(J)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn limit(&self, maxSize: i64) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.limit:(J)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "skip", descriptor = "(J)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn skip(&self, n: i64) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.skip:(J)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "takeWhile", descriptor = "(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn takeWhile(&self, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.takeWhile:(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "dropWhile", descriptor = "(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dropWhile(&self, predicate: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.dropWhile:(Ljava/util/function/IntPredicate;)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "sorted", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sorted(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.sorted:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "distinct", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn distinct(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.distinct:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/IntConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/IntPipeline.forEach:(Ljava/util/function/IntConsumer;)V")
        }

        #[java_method(name = "forEachOrdered", descriptor = "(Ljava/util/function/IntConsumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forEachOrdered(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/stream/IntPipeline.forEachOrdered:(Ljava/util/function/IntConsumer;)V")
        }

        #[java_method(name = "sum", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sum(&self) -> Result<i32> {
            let this = self;
            let __lam_237: std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<i32>> = std::rc::Rc::new(move |_la0: i32, _la1: i32| -> crate::error::Result<i32> { Integer::sum(_la0, _la1) });
            let _t0 = this.reduce_i_intbin(0i32, Clone::clone(&Object::from_any(__lam_237)))?;
            Ok(_t0)
        }

        #[java_method(name = "min", descriptor = "()Ljava/util/OptionalInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.min:()Ljava/util/OptionalInt;")
        }

        #[java_method(name = "max", descriptor = "()Ljava/util/OptionalInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.max:()Ljava/util/OptionalInt;")
        }

        #[java_method(name = "count", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn count(&self) -> Result<i64> {
            panic!("stub: java/util/stream/IntPipeline.count:()J")
        }

        #[java_method(name = "average", descriptor = "()Ljava/util/OptionalDouble;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn average(&self) -> Result<OptionalDouble> {
            let this = self;
            let __lam_259: std::rc::Rc<dyn Fn() -> crate::error::Result<Object>> = std::rc::Rc::new(move || -> crate::error::Result<Object> { IntPipeline::lambda_average_1() });
            let __lam_263: std::rc::Rc<dyn Fn(Object, i32) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: i32| -> crate::error::Result<()> { IntPipeline::lambda_average_2(_la0, _la1) });
            let __lam_266: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<()>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<()> { IntPipeline::lambda_average_3(_la0, _la1) });
            let _t0 = this.collect(Clone::clone(&Object::from_any(__lam_259)), Clone::clone(&Object::from_any(__lam_263)), Clone::clone(&Object::from_any(__lam_266)))?;
            let mut avg = (_t0).downcast::<Rc<RefCell<Vec<i64>>>>();
            let mut _merged2: OptionalDouble;
            if (((avg.borrow()[0i32 as usize]>(0i64)) as i32-((avg.borrow()[0i32 as usize])<(0i64)) as i32)>0) {
                let _t1: OptionalDouble = OptionalDouble::of(((avg.borrow()[1i32 as usize] as f64)/(avg.borrow()[0i32 as usize] as f64)))?;
                _merged2 = _t1;
            } else {
                let _t1: OptionalDouble = OptionalDouble::empty()?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "summaryStatistics", descriptor = "()Ljava/util/IntSummaryStatistics;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn summaryStatistics(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.summaryStatistics:()Ljava/util/IntSummaryStatistics;")
        }

        #[java_method(name = "reduce", descriptor = "(ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: reduce(ILjava/util/function/IntBinaryOperator;)I
        pub fn reduce_i_intbin(&self, mut identity: i32, mut op: Object) -> Result<i32> {
            let this = self;
            let _t0: Object = ReduceOps::makeInt_i_intbin(identity, Clone::clone(&op))?;
            let _t1 = this.__super().evaluate(Clone::clone(&_t0))?;
            Ok((_t1).downcast::<i32>())
        }

        #[java_method(name = "reduce", descriptor = "(Ljava/util/function/IntBinaryOperator;)Ljava/util/OptionalInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reduce_intbin(&self, op: Object) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.reduce:(Ljava/util/function/IntBinaryOperator;)Ljava/util/OptionalInt;")
        }

        #[java_method(name = "collect", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/ObjIntConsumer;Ljava/util/function/BiConsumer;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/ObjIntConsumer<TR;>;Ljava/util/function/BiConsumer<TR;TR;>;)TR;")]
        pub fn collect(&self, mut supplier: Object, mut accumulator: Object, mut combiner: Object) -> Result<Object> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&combiner))?;
            let __lam_cap304_0 = combiner;
            let __lam_304: std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object, _la1: Object| -> crate::error::Result<Object> { IntPipeline::lambda_collect_4(__lam_cap304_0.clone(), _la0, _la1) });
            let mut operator = Object::from_any(__lam_304);
            let _t1: Object = ReduceOps::makeInt_suppli_objint_binary(Clone::clone(&supplier), Clone::clone(&accumulator), Clone::clone(&operator))?;
            let _t2 = this.__super().evaluate(Clone::clone(&_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "anyMatch", descriptor = "(Ljava/util/function/IntPredicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn anyMatch(&self, predicate: Object) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline.anyMatch:(Ljava/util/function/IntPredicate;)Z")
        }

        #[java_method(name = "allMatch", descriptor = "(Ljava/util/function/IntPredicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn allMatch(&self, predicate: Object) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline.allMatch:(Ljava/util/function/IntPredicate;)Z")
        }

        #[java_method(name = "noneMatch", descriptor = "(Ljava/util/function/IntPredicate;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn noneMatch(&self, predicate: Object) -> Result<bool> {
            panic!("stub: java/util/stream/IntPipeline.noneMatch:(Ljava/util/function/IntPredicate;)Z")
        }

        #[java_method(name = "findFirst", descriptor = "()Ljava/util/OptionalInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findFirst(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.findFirst:()Ljava/util/OptionalInt;")
        }

        #[java_method(name = "findAny", descriptor = "()Ljava/util/OptionalInt;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findAny(&self) -> Result<Object> {
            panic!("stub: java/util/stream/IntPipeline.findAny:()Ljava/util/OptionalInt;")
        }

        #[java_method(name = "toArray", descriptor = "()[I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/util/stream/IntPipeline.toArray:()[I")
        }
    }
}
