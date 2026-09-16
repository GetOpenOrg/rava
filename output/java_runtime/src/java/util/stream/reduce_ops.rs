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
    #[binary_name       = "java/util/stream/ReduceOps"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReduceOps.java"]
    #[inner_classes     = "java/util/stream/ReduceOps$1:::0;java/util/stream/ReduceOps$2:::0;java/util/stream/ReduceOps$3:::0;java/util/stream/ReduceOps$4:::0;java/util/stream/ReduceOps$5:::0;java/util/stream/ReduceOps$6:::0;java/util/stream/ReduceOps$7:::0;java/util/stream/ReduceOps$8:::0;java/util/stream/ReduceOps$9:::0;java/util/stream/ReduceOps$10:::0;java/util/stream/ReduceOps$11:::0;java/util/stream/ReduceOps$12:::0;java/util/stream/ReduceOps$13:::0;java/util/stream/ReduceOps$14:::0;java/util/stream/ReduceOps$15:::0;java/util/stream/ReduceOps$16:::0;java/util/stream/ReduceOps$17:::0;java/util/stream/ReduceOps$ReduceTask:java/util/stream/ReduceOps:ReduceTask:26;java/util/stream/ReduceOps$ReduceOp:java/util/stream/ReduceOps:ReduceOp:1034;java/util/stream/ReduceOps$Box:java/util/stream/ReduceOps:Box:1034;java/util/stream/ReduceOps$AccumulatingSink:java/util/stream/ReduceOps:AccumulatingSink:1546;java/util/stream/ReduceOps$CountingSink:java/util/stream/ReduceOps:CountingSink:1032;java/util/stream/ReduceOps$13ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$12ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$11ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$10ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$9ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$8ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$7ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$6ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$5ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$4ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$3ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$2ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$1ReducingSink::ReducingSink:0;java/util/stream/ReduceOps$CountingSink$OfDouble:java/util/stream/ReduceOps$CountingSink:OfDouble:24;java/util/stream/ReduceOps$CountingSink$OfLong:java/util/stream/ReduceOps$CountingSink:OfLong:24;java/util/stream/ReduceOps$CountingSink$OfInt:java/util/stream/ReduceOps$CountingSink:OfInt:24;java/util/stream/ReduceOps$CountingSink$OfRef:java/util/stream/ReduceOps$CountingSink:OfRef:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/stream/ReduceOps"]

    pub struct ReduceOps;

    impl ReduceOps {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/stream/ReduceOps.<init>:()V")
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>(TU;Ljava/util/function/BiFunction<TU;-TT;TU;>;Ljava/util/function/BinaryOperator<TU;>;)Ljava/util/stream/TerminalOp<TT;TU;>;")]
        // java: makeRef(Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;
        pub fn makeRef_obj_bifunc_binary(mut seed: Object, mut reducer: Object, mut combiner: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&reducer))?;
            let _t1: Object = Objects::requireNonNull_obj(Clone::clone(&combiner))?;
            Ok(Object::from_any(ReduceOps_1::new(Clone::clone(&StreamShape::REFERENCE()), Clone::clone(&combiner), Clone::clone(&reducer), Clone::clone(&seed))?.clone()))
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/BinaryOperator<TT;>;)Ljava/util/stream/TerminalOp<TT;Ljava/util/Optional<TT;>;>;")]
        // java: makeRef(Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;
        pub fn makeRef_binary(mut operator: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&operator))?;
            Ok(Object::from_any(ReduceOps_2::new(Clone::clone(&StreamShape::REFERENCE()), Clone::clone(&operator))?.clone()))
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/stream/Collector;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;I:Ljava/lang/Object;>(Ljava/util/stream/Collector<-TT;TI;*>;)Ljava/util/stream/TerminalOp<TT;TI;>;")]
        // java: makeRef(Ljava/util/stream/Collector;)Ljava/util/stream/TerminalOp;
        pub fn makeRef_collec(mut collector: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&collector))?;
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.supplier()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.supplier()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut supplier: Object = _vdispatch1;
            let _vdispatch2: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.accumulator()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.accumulator()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut accumulator = (_vdispatch2).downcast::<BiConsumer<Object, Object>>();
            let _vdispatch3: Object = if let Some(_d) = collector.0.as_any().downcast_ref::<Collectors_CollectorImpl<Object, Object, Object>>() { _d.combiner()? } else if let Some(_d) = collector.0.as_any().downcast_ref::<Object>() { _d.combiner()? } else if let Some(__f) = collector.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut combiner: Object = _vdispatch3;
            Ok(Object::from_any(ReduceOps_3::new(Clone::clone(&StreamShape::REFERENCE()), Clone::clone(&combiner), Object::from_any(accumulator.clone()), Clone::clone(&supplier), Clone::clone(&collector))?.clone()))
        }

        #[java_method(name = "makeRef", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/BiConsumer<TR;-TT;>;Ljava/util/function/BiConsumer<TR;TR;>;)Ljava/util/stream/TerminalOp<TT;TR;>;")]
        pub fn makeRef_suppli_bicons_bicons(seedFactory: Object, accumulator: Object, reducer: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeRef:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BiConsumer;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeRefCounting", descriptor = "()Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/stream/TerminalOp<TT;Ljava/lang/Long;>;")]
        pub fn makeRefCounting() -> Result<Object> {
            Ok(Object::from_any(ReduceOps_5::new(Clone::clone(&StreamShape::REFERENCE()))?.clone()))
        }

        #[java_method(name = "makeInt", descriptor = "(ILjava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/lang/Integer;>;")]
        // java: makeInt(ILjava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp;
        pub fn makeInt_i_intbin(mut identity: i32, mut operator: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&operator))?;
            Ok(Object::from_any(ReduceOps_6::new(Clone::clone(&StreamShape::INT_VALUE()), Clone::clone(&operator), identity)?.clone()))
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/util/OptionalInt;>;")]
        pub fn makeInt_intbin(operator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeInt:(Ljava/util/function/IntBinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeInt", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/ObjIntConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/ObjIntConsumer<TR;>;Ljava/util/function/BinaryOperator<TR;>;)Ljava/util/stream/TerminalOp<Ljava/lang/Integer;TR;>;")]
        // java: makeInt(Ljava/util/function/Supplier;Ljava/util/function/ObjIntConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;
        pub fn makeInt_suppli_objint_binary(mut supplier: Object, mut accumulator: Object, mut combiner: Object) -> Result<Object> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&supplier))?;
            let _t1: Object = Objects::requireNonNull_obj(Clone::clone(&accumulator))?;
            let _t2: Object = Objects::requireNonNull_obj(Clone::clone(&combiner))?;
            Ok(Object::from_any(ReduceOps_8::new(Clone::clone(&StreamShape::INT_VALUE()), Clone::clone(&combiner), Clone::clone(&accumulator), Clone::clone(&supplier))?.clone()))
        }

        #[java_method(name = "makeIntCounting", descriptor = "()Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/TerminalOp<Ljava/lang/Integer;Ljava/lang/Long;>;")]
        pub fn makeIntCounting() -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeIntCounting:()Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(JLjava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(JLjava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/lang/Long;>;")]
        pub fn makeLong_l_longbi(identity: i64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeLong:(JLjava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/util/OptionalLong;>;")]
        pub fn makeLong_longbi(operator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeLong:(Ljava/util/function/LongBinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLong", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/ObjLongConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/ObjLongConsumer<TR;>;Ljava/util/function/BinaryOperator<TR;>;)Ljava/util/stream/TerminalOp<Ljava/lang/Long;TR;>;")]
        pub fn makeLong_suppli_objlon_binary(supplier: Object, accumulator: Object, combiner: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeLong:(Ljava/util/function/Supplier;Ljava/util/function/ObjLongConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeLongCounting", descriptor = "()Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/TerminalOp<Ljava/lang/Long;Ljava/lang/Long;>;")]
        pub fn makeLongCounting() -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeLongCounting:()Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(DLjava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(DLjava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/lang/Double;>;")]
        pub fn makeDouble_d_double(identity: f64, arg1: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeDouble:(DLjava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/util/OptionalDouble;>;")]
        pub fn makeDouble_double(operator: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeDouble:(Ljava/util/function/DoubleBinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDouble", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/ObjDoubleConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/ObjDoubleConsumer<TR;>;Ljava/util/function/BinaryOperator<TR;>;)Ljava/util/stream/TerminalOp<Ljava/lang/Double;TR;>;")]
        pub fn makeDouble_suppli_objdou_binary(supplier: Object, accumulator: Object, combiner: Object) -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeDouble:(Ljava/util/function/Supplier;Ljava/util/function/ObjDoubleConsumer;Ljava/util/function/BinaryOperator;)Ljava/util/stream/TerminalOp;")
        }

        #[java_method(name = "makeDoubleCounting", descriptor = "()Ljava/util/stream/TerminalOp;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/TerminalOp<Ljava/lang/Double;Ljava/lang/Long;>;")]
        pub fn makeDoubleCounting() -> Result<Object> {
            panic!("stub: java/util/stream/ReduceOps.makeDoubleCounting:()Ljava/util/stream/TerminalOp;")
        }
    }
}
