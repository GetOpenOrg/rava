#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PrimitiveIterator$OfLong",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator",
    access      = "public abstract",
    source      = "PrimitiveIterator.java",
))]
pub struct PrimitiveIterator_OfLong;

impl PrimitiveIterator_OfLong {
    // java: nextLong()J
    pub fn nextLong(&self) -> Result<i64> {
        panic!("stub: java/util/PrimitiveIterator$OfLong.nextLong:()J")
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining__longco(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/PrimitiveIterator$OfLong.forEachRemaining:(Ljava/util/function/LongConsumer;)V")
    }

    // java: next()Ljava/lang/Long;
    pub fn next(&self) -> Result<i64> {
        panic!("stub: java/util/PrimitiveIterator$OfLong.next:()Ljava/lang/Long;")
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining__consum(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/PrimitiveIterator$OfLong.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
