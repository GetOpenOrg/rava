#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Number",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Serializable",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Number.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Number;

impl Number {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-8742448824652078965"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -8742448824652078965i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Number.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Number.intValue:()I")
    }

    #[cfg_attr(any(), java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Number.longValue:()J")
    }

    #[cfg_attr(any(), java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Number.floatValue:()F")
    }

    #[cfg_attr(any(), java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Number.doubleValue:()D")
    }

    #[cfg_attr(any(), java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Number.byteValue:()B")
    }

    #[cfg_attr(any(), java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Number.shortValue:()S")
    }
}
