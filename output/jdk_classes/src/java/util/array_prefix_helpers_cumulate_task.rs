#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayPrefixHelpers$CumulateTask",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArrayPrefixHelpers.java",
))]
pub struct ArrayPrefixHelpers_CumulateTask<T> {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub array: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "function", descriptor = "Ljava/util/function/BinaryOperator;", access = "final"))]
    pub function: Field<Object>,
    #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/ArrayPrefixHelpers$CumulateTask;"))]
    pub left: Field<Object>,
    #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/ArrayPrefixHelpers$CumulateTask;"))]
    pub right: Field<Object>,
    #[cfg_attr(any(), java_field(name = "in", descriptor = "Ljava/lang/Object;"))]
    pub in_: Field<Object>,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/lang/Object;"))]
    pub out: Field<Object>,
    #[cfg_attr(any(), java_field(name = "lo", descriptor = "I", access = "final"))]
    pub lo: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi", descriptor = "I", access = "final"))]
    pub hi: Field<i32>,
    #[cfg_attr(any(), java_field(name = "origin", descriptor = "I", access = "final"))]
    pub origin: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I", access = "final"))]
    pub threshold: Field<i32>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> ArrayPrefixHelpers_CumulateTask<T> {
    // java: <init>(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;II)V
    pub fn new__arrayp_binary_arr_obj_i_i(&self, parent: Object, function: Object, array: Vec<Object>, lo: i32, hi: i32) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$CumulateTask.<init>")
    }

    // java: <init>(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;IIIII)V
    pub fn new__arrayp_binary_arr_obj_i_i_i_i_i(&self, parent: Object, function: Object, array: Vec<Object>, origin: i32, fence: i32, threshold: i32, lo: i32, hi: i32) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$CumulateTask.<init>")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$CumulateTask.compute")
    }
}
