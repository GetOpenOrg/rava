#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/stream/Collector",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Collector.java",
    inner_classes     = "java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409;java/util/stream/Collectors$CollectorImpl:java/util/stream/Collectors:CollectorImpl:24",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Collector<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static>(std::marker::PhantomData<(T, A, R,)>);

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> Collector<T, A, R> {
    #[cfg_attr(any(), java_method(name = "supplier", descriptor = "()Ljava/util/function/Supplier;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/function/Supplier<TA;>;"))]
    pub fn supplier(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.supplier:()Ljava/util/function/Supplier;")
    }

    #[cfg_attr(any(), java_method(name = "accumulator", descriptor = "()Ljava/util/function/BiConsumer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/function/BiConsumer<TA;TT;>;"))]
    pub fn accumulator(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.accumulator:()Ljava/util/function/BiConsumer;")
    }

    #[cfg_attr(any(), java_method(name = "combiner", descriptor = "()Ljava/util/function/BinaryOperator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/function/BinaryOperator<TA;>;"))]
    pub fn combiner(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.combiner:()Ljava/util/function/BinaryOperator;")
    }

    #[cfg_attr(any(), java_method(name = "finisher", descriptor = "()Ljava/util/function/Function;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/function/Function<TA;TR;>;"))]
    pub fn finisher(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.finisher:()Ljava/util/function/Function;")
    }

    #[cfg_attr(any(), java_method(name = "characteristics", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/stream/Collector$Characteristics;>;"))]
    pub fn characteristics(&self) -> Result<Set<Object>> {
        panic!("stub: java/util/stream/Collector.characteristics:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TR;>;Ljava/util/function/BiConsumer<TR;TT;>;Ljava/util/function/BinaryOperator<TR;>;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector<TT;TR;TR;>;"))]
    pub fn of_suppli_bicons_binary_arr_col(supplier: Object, accumulator: Object, combiner: Object, characteristics: Rc<RefCell<Vec<Collector_Characteristics>>>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collector.of:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;A:Ljava/lang/Object;R:Ljava/lang/Object;>(Ljava/util/function/Supplier<TA;>;Ljava/util/function/BiConsumer<TA;TT;>;Ljava/util/function/BinaryOperator<TA;>;Ljava/util/function/Function<TA;TR;>;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector<TT;TA;TR;>;"))]
    pub fn of_suppli_bicons_binary_functi_arr_col(supplier: Object, accumulator: Object, combiner: Object, finisher: Object, characteristics: Rc<RefCell<Vec<Collector_Characteristics>>>) -> Result<Collector<Object, Object, Object>> {
        panic!("stub: java/util/stream/Collector.of:(Ljava/util/function/Supplier;Ljava/util/function/BiConsumer;Ljava/util/function/BinaryOperator;Ljava/util/function/Function;[Ljava/util/stream/Collector$Characteristics;)Ljava/util/stream/Collector;")
    }
}
