#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayPrefixHelpers$LongCumulateTask",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArrayPrefixHelpers.java",
))]
pub struct ArrayPrefixHelpers_LongCumulateTask {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[J", access = "final"))]
    pub array: Field<Vec<i64>>,
    #[cfg_attr(any(), java_field(name = "function", descriptor = "Ljava/util/function/LongBinaryOperator;", access = "final"))]
    pub function: Field<Object>,
    #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/ArrayPrefixHelpers$LongCumulateTask;"))]
    pub left: Field<Object>,
    #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/ArrayPrefixHelpers$LongCumulateTask;"))]
    pub right: Field<Object>,
    #[cfg_attr(any(), java_field(name = "in", descriptor = "J"))]
    pub in_: Field<i64>,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "J"))]
    pub out: Field<i64>,
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
}

impl ArrayPrefixHelpers_LongCumulateTask {
    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JII)V
    pub fn new__arrayp_longbi_arr_l_i_i(&self, parent: Object, function: Object, array: Vec<i64>, lo: i32, hi: i32) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$LongCumulateTask.<init>")
    }

    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JIIIII)V
    pub fn new__arrayp_longbi_arr_l_i_i_i_i_i(&self, parent: Object, function: Object, array: Vec<i64>, origin: i32, fence: i32, threshold: i32, lo: i32, hi: i32) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$LongCumulateTask.<init>")
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        todo!("abstract java/util/ArrayPrefixHelpers$LongCumulateTask.compute")
    }
}
