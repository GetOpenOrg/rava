#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PrimitiveIterator$OfDouble",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator",
    access      = "public abstract",
    source      = "PrimitiveIterator.java",
))]
pub struct PrimitiveIterator_OfDouble;

impl PrimitiveIterator_OfDouble {
    // java: nextDouble()D
    pub fn nextDouble(&self) -> Result<f64> {
        todo!("abstract java/util/PrimitiveIterator$OfDouble.nextDouble")
    }

    // java: forEachRemaining(Ljava/util/function/DoubleConsumer;)V
    pub fn forEachRemaining__double(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/PrimitiveIterator$OfDouble.forEachRemaining")
    }

    // java: next()Ljava/lang/Double;
    pub fn next(&self) -> Result<f64> {
        todo!("abstract java/util/PrimitiveIterator$OfDouble.next")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining__consum(&self, action: Object) -> Result<()> {
        todo!("abstract java/util/PrimitiveIterator$OfDouble.forEachRemaining")
    }
}
