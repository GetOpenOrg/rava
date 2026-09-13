#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparator",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Comparator.java",
))]
pub struct Comparator<T>(std::marker::PhantomData<T>);

impl<T: Clone + 'static> Comparator<T> {
    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, arg0: Object, arg1: Object) -> Result<i32> {
        panic!("stub: java/util/Comparator.compare:(Ljava/lang/Object;Ljava/lang/Object;)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Comparator.equals:(Ljava/lang/Object;)Z")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/Comparator.reversed:()Ljava/util/Comparator;")
    }

    // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing__compar(&self, other: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: thenComparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing__functi_compar(&self, keyExtractor: Object, keyComparator: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: thenComparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    pub fn thenComparing__functi(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
    }

    // java: thenComparingInt(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;
    pub fn thenComparingInt(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
    }

    // java: thenComparingLong(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;
    pub fn thenComparingLong(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
    }

    // java: thenComparingDouble(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;
    pub fn thenComparingDouble(&self, keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.thenComparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
    }

    // java: reverseOrder()Ljava/util/Comparator;
    pub fn reverseOrder() -> Result<Object> {
        panic!("stub: java/util/Comparator.reverseOrder:()Ljava/util/Comparator;")
    }

    // java: naturalOrder()Ljava/util/Comparator;
    pub fn naturalOrder() -> Result<Object> {
        panic!("stub: java/util/Comparator.naturalOrder:()Ljava/util/Comparator;")
    }

    // java: nullsFirst(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn nullsFirst(comparator: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.nullsFirst:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: nullsLast(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn nullsLast(comparator: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.nullsLast:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: comparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn comparing__functi_compar(keyExtractor: Object, keyComparator: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparing:(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: comparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    pub fn comparing__functi(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparing:(Ljava/util/function/Function;)Ljava/util/Comparator;")
    }

    // java: comparingInt(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;
    pub fn comparingInt(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingInt:(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;")
    }

    // java: comparingLong(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;
    pub fn comparingLong(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingLong:(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;")
    }

    // java: comparingDouble(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;
    pub fn comparingDouble(keyExtractor: Object) -> Result<Object> {
        panic!("stub: java/util/Comparator.comparingDouble:(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;")
    }
}
