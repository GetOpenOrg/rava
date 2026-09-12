#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArraysParallelSortHelpers$FJObject$Sorter",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArraysParallelSortHelpers.java",
))]
pub struct ArraysParallelSortHelpers_FJObject_Sorter<T> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub a: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "w", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub w: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "base", descriptor = "I", access = "final"))]
    pub base: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "wbase", descriptor = "I", access = "final"))]
    pub wbase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "gran", descriptor = "I", access = "final"))]
    pub gran: Field<i32>,
    #[cfg_attr(any(), java_field(name = "comparator", descriptor = "Ljava/util/Comparator;"))]
    pub comparator: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> ArraysParallelSortHelpers_FJObject_Sorter<T> {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V
    pub fn new(&self, par: Object, a: Vec<Object>, w: Vec<Object>, base: i32, size: i32, wbase: i32, gran: i32, comparator: Object) -> Result<()> {
        todo!("abstract java/util/ArraysParallelSortHelpers$FJObject$Sorter.<init>")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        todo!("abstract java/util/ArraysParallelSortHelpers$FJObject$Sorter.compute")
    }
}
