#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Double",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Double.java",
))]
pub struct Double {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "D", access = "private final"))]
    pub value: Field<f64>,
}

impl Double {
    // java: toString(D)Ljava/lang/String;
    pub fn toString__d(d: f64) -> Result<String> {
        panic!("stub: java/lang/Double.toString:(D)Ljava/lang/String;")
    }

    // java: toHexString(D)Ljava/lang/String;
    pub fn toHexString(d: f64) -> Result<String> {
        panic!("stub: java/lang/Double.toHexString:(D)Ljava/lang/String;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Double;
    pub fn valueOf__str(s: String) -> Result<f64> {
        panic!("stub: java/lang/Double.valueOf:(Ljava/lang/String;)Ljava/lang/Double;")
    }

    // java: valueOf(D)Ljava/lang/Double;
    pub fn valueOf__d(d: f64) -> Result<f64> {
        panic!("stub: java/lang/Double.valueOf:(D)Ljava/lang/Double;")
    }

    // java: parseDouble(Ljava/lang/String;)D
    pub fn parseDouble(s: String) -> Result<f64> {
        panic!("stub: java/lang/Double.parseDouble:(Ljava/lang/String;)D")
    }

    // java: isNaN(D)Z
    pub fn isNaN__d(v: f64) -> Result<bool> {
        panic!("stub: java/lang/Double.isNaN:(D)Z")
    }

    // java: isInfinite(D)Z
    pub fn isInfinite__d(v: f64) -> Result<bool> {
        panic!("stub: java/lang/Double.isInfinite:(D)Z")
    }

    // java: isFinite(D)Z
    pub fn isFinite(d: f64) -> Result<bool> {
        panic!("stub: java/lang/Double.isFinite:(D)Z")
    }

    // java: <init>(D)V
    pub fn new__d(&self, value: f64) -> Result<()> {
        panic!("stub: java/lang/Double.<init>:(D)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Double.<init>:(Ljava/lang/String;)V")
    }

    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        panic!("stub: java/lang/Double.isNaN:()Z")
    }

    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        panic!("stub: java/lang/Double.isInfinite:()Z")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Double.toString:()Ljava/lang/String;")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Double.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Double.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Double.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Double.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Double.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Double.doubleValue:()D")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Double.hashCode:()I")
    }

    // java: hashCode(D)I
    pub fn hashCode__d(value: f64) -> Result<i32> {
        panic!("stub: java/lang/Double.hashCode:(D)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Double.equals:(Ljava/lang/Object;)Z")
    }

    // java: doubleToLongBits(D)J
    pub fn doubleToLongBits(value: f64) -> Result<i64> {
        panic!("stub: java/lang/Double.doubleToLongBits:(D)J")
    }

    // java: doubleToRawLongBits(D)J
    pub fn doubleToRawLongBits(arg0: f64) -> Result<i64> {
        panic!("native: java/lang/Double.doubleToRawLongBits:(D)J")
    }

    // java: longBitsToDouble(J)D
    pub fn longBitsToDouble(arg0: i64) -> Result<f64> {
        panic!("native: java/lang/Double.longBitsToDouble:(J)D")
    }

    // java: compareTo(Ljava/lang/Double;)I
    pub fn compareTo(&self, anotherDouble: f64) -> Result<i32> {
        panic!("stub: java/lang/Double.compareTo:(Ljava/lang/Double;)I")
    }

    // java: compare(DD)I
    pub fn compare(d1: f64, arg1: f64) -> Result<i32> {
        panic!("stub: java/lang/Double.compare:(DD)I")
    }

    // java: sum(DD)D
    pub fn sum(a: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Double.sum:(DD)D")
    }

    // java: max(DD)D
    pub fn max(a: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Double.max:(DD)D")
    }

    // java: min(DD)D
    pub fn min(a: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Double.min:(DD)D")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Double.describeConstable:()Ljava/util/Optional;")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<f64> {
        panic!("stub: java/lang/Double.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;")
    }
}
