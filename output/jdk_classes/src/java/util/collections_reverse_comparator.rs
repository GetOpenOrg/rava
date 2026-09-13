#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$ReverseComparator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_ReverseComparator;

impl Collections_ReverseComparator {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/Collections$ReverseComparator.<init>:()V")
    }

    // java: compare(Ljava/lang/Comparable;Ljava/lang/Comparable;)I
    pub fn compare(&self, c1: Object, c2: Object) -> Result<i32> {
        panic!("stub: java/util/Collections$ReverseComparator.compare:(Ljava/lang/Comparable;Ljava/lang/Comparable;)I")
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$ReverseComparator.readResolve:()Ljava/lang/Object;")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/Collections$ReverseComparator.reversed:()Ljava/util/Comparator;")
    }
}
