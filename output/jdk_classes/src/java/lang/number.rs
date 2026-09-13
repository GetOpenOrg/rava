#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Number",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public abstract",
    source      = "Number.java",
))]
pub struct Number;

impl Number {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/Number.<init>:()V")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Number.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Number.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Number.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Number.doubleValue:()D")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Number.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Number.shortValue:()S")
    }
}
