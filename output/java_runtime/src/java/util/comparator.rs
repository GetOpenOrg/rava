#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/Comparator",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Comparator.java",
    inner_classes     = "java/util/Comparators$NaturalOrderComparator:java/util/Comparators:NaturalOrderComparator:16408;java/util/Comparators$NullComparator:java/util/Comparators:NullComparator:24;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Comparator<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Comparator<T> {
    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TT;TT;)I"))]
    pub fn compare(&self, arg0: T, arg1: T) -> Result<i32> {
        panic!("stub: java/util/Comparator.compare:(Ljava/lang/Object;Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Comparator.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "reversed", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Comparator<TT;>;"))]
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/Comparator.reversed:()Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparing", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparing_compar(&self, other: Comparator<T>) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparing", descriptor = "(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;Ljava/util/Comparator<-TU;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparing_functi_compar(&self, keyExtractor: Object, keyComparator: Comparator<Object>) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparing", descriptor = "(Ljava/util/function/Function;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U::Ljava/lang/Comparable<-TU;>;>(Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparing_functi(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparingInt(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparingLong(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "thenComparingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn thenComparingDouble(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "reverseOrder", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>()Ljava/util/Comparator<TT;>;"))]
    pub fn reverseOrder() -> Result<Object> {
        panic!("stub: java/util/Comparator.reverseOrder:()Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "naturalOrder", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>()Ljava/util/Comparator<TT;>;"))]
    pub fn naturalOrder() -> Result<Object> {
        panic!("stub: java/util/Comparator.naturalOrder:()Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "nullsFirst", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn nullsFirst(comparator: Comparator<T>) -> Result<Object> {
        panic!("stub: java/util/Comparator.nullsFirst:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "nullsLast", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn nullsLast(comparator: Comparator<T>) -> Result<Object> {
        panic!("stub: java/util/Comparator.nullsLast:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "comparing", descriptor = "(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;Ljava/util/Comparator<-TU;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn comparing_functi_compar(keyExtractor: Object, keyComparator: Comparator<Object>) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "comparing", descriptor = "(Ljava/util/function/Function;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U::Ljava/lang/Comparable<-TU;>;>(Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn comparing_functi(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "comparingInt", descriptor = "(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToIntFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn comparingInt(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "comparingLong", descriptor = "(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToLongFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn comparingLong(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
    }

    #[cfg_attr(any(), java_method(name = "comparingDouble", descriptor = "(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/ToDoubleFunction<-TT;>;)Ljava/util/Comparator<TT;>;"))]
    pub fn comparingDouble(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
    }
}
