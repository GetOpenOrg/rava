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
        todo!("abstract java/lang/Double.toString")
    }

    // java: toHexString(D)Ljava/lang/String;
    pub fn toHexString(d: f64) -> Result<String> {
        todo!("abstract java/lang/Double.toHexString")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Double;
    pub fn valueOf__str(s: String) -> Result<f64> {
        todo!("abstract java/lang/Double.valueOf")
    }

    // java: valueOf(D)Ljava/lang/Double;
    pub fn valueOf__d(d: f64) -> Result<f64> {
        todo!("abstract java/lang/Double.valueOf")
    }

    // java: parseDouble(Ljava/lang/String;)D
    pub fn parseDouble(s: String) -> Result<f64> {
        todo!("abstract java/lang/Double.parseDouble")
    }

    // java: isNaN(D)Z
    pub fn isNaN__d(v: f64) -> Result<bool> {
        todo!("abstract java/lang/Double.isNaN")
    }

    // java: isInfinite(D)Z
    pub fn isInfinite__d(v: f64) -> Result<bool> {
        todo!("abstract java/lang/Double.isInfinite")
    }

    // java: isFinite(D)Z
    pub fn isFinite(d: f64) -> Result<bool> {
        todo!("abstract java/lang/Double.isFinite")
    }

    // java: <init>(D)V
    pub fn new__d(&self, value: f64) -> Result<()> {
        todo!("abstract java/lang/Double.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Double.<init>")
    }

    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        todo!("abstract java/lang/Double.isNaN")
    }

    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        todo!("abstract java/lang/Double.isInfinite")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Double.toString")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Double.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Double.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Double.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Double.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Double.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Double.doubleValue")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Double.hashCode")
    }

    // java: hashCode(D)I
    pub fn hashCode__d(value: f64) -> Result<i32> {
        todo!("abstract java/lang/Double.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Double.equals")
    }

    // java: doubleToLongBits(D)J
    pub fn doubleToLongBits(value: f64) -> Result<i64> {
        todo!("abstract java/lang/Double.doubleToLongBits")
    }

    // java: doubleToRawLongBits(D)J
    pub fn doubleToRawLongBits(arg0: f64) -> Result<i64> {
        todo!("native java/lang/Double.doubleToRawLongBits")
    }

    // java: longBitsToDouble(J)D
    pub fn longBitsToDouble(arg0: i64) -> Result<f64> {
        todo!("native java/lang/Double.longBitsToDouble")
    }

    // java: compareTo(Ljava/lang/Double;)I
    pub fn compareTo(&self, anotherDouble: f64) -> Result<i32> {
        todo!("abstract java/lang/Double.compareTo")
    }

    // java: compare(DD)I
    pub fn compare(d1: f64, arg1: f64) -> Result<i32> {
        todo!("abstract java/lang/Double.compare")
    }

    // java: sum(DD)D
    pub fn sum(a: f64, arg1: f64) -> Result<f64> {
        todo!("abstract java/lang/Double.sum")
    }

    // java: max(DD)D
    pub fn max(a: f64, arg1: f64) -> Result<f64> {
        todo!("abstract java/lang/Double.max")
    }

    // java: min(DD)D
    pub fn min(a: f64, arg1: f64) -> Result<f64> {
        todo!("abstract java/lang/Double.min")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Double.describeConstable")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<f64> {
        todo!("abstract java/lang/Double.resolveConstantDesc")
    }
}
