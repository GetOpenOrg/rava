#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparators$NaturalOrderComparator",
    super_class = "java/lang/Enum",
    interfaces  = "java/util/Comparator",
    access      = "final",
    source      = "Comparators.java",
))]
pub struct Comparators_NaturalOrderComparator;

impl Comparators_NaturalOrderComparator {
    // java: values()[Ljava/util/Comparators$NaturalOrderComparator;
    pub fn values() -> Result<Vec<Object>> {
        panic!("stub: java/util/Comparators$NaturalOrderComparator.values:()[Ljava/util/Comparators$NaturalOrderComparator;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/util/Comparators$NaturalOrderComparator;
    pub fn valueOf(name: String) -> Result<Object> {
        panic!("stub: java/util/Comparators$NaturalOrderComparator.valueOf:(Ljava/lang/String;)Ljava/util/Comparators$NaturalOrderComparator;")
    }

    // java: <init>(Ljava/lang/String;I)V
    pub fn new(&self, arg0: String, arg1: i32) -> Result<()> {
        panic!("stub: java/util/Comparators$NaturalOrderComparator.<init>:(Ljava/lang/String;I)V")
    }

    // java: compare(Ljava/lang/Comparable;Ljava/lang/Comparable;)I
    pub fn compare(&self, c1: Object, c2: Object) -> Result<i32> {
        panic!("stub: java/util/Comparators$NaturalOrderComparator.compare:(Ljava/lang/Comparable;Ljava/lang/Comparable;)I")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        panic!("stub: java/util/Comparators$NaturalOrderComparator.reversed:()Ljava/util/Comparator;")
    }
}
