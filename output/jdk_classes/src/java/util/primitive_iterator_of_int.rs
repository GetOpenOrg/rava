#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PrimitiveIterator$OfInt",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator",
    access      = "public abstract",
    source      = "PrimitiveIterator.java",
))]
pub struct PrimitiveIterator_OfInt;

impl PrimitiveIterator_OfInt {
    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        todo!("abstract java/util/PrimitiveIterator$OfInt.nextInt")
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining__intcon(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/PrimitiveIterator$OfInt.forEachRemaining")
    }

    // java: next()Ljava/lang/Integer;
    pub fn next(&self) -> Result<i32> {
        todo!("abstract java/util/PrimitiveIterator$OfInt.next")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining__consum(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/PrimitiveIterator$OfInt.forEachRemaining")
    }
}
