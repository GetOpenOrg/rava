#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$NaturalOrder",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator",
    access      = "final",
    source      = "Arrays.java",
))]
pub struct Arrays_NaturalOrder;

impl Arrays_NaturalOrder {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/Arrays$NaturalOrder.<init>:()V")
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, first: Object, second: Object) -> Result<i32> {
        panic!("stub: java/util/Arrays$NaturalOrder.compare:(Ljava/lang/Object;Ljava/lang/Object;)I")
    }
}
