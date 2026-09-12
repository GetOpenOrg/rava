#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Float",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Float.java",
))]
pub struct Float {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "F", access = "private final"))]
    pub value: Field<f32>,
}

impl Float {
    // java: toString(F)Ljava/lang/String;
    pub fn toString__f(f: f32) -> Result<String> {
        todo!("abstract java/lang/Float.toString")
    }

    // java: toHexString(F)Ljava/lang/String;
    pub fn toHexString(f: f32) -> Result<String> {
        todo!("abstract java/lang/Float.toHexString")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Float;
    pub fn valueOf__str(s: String) -> Result<Object> {
        todo!("abstract java/lang/Float.valueOf")
    }

    // java: valueOf(F)Ljava/lang/Float;
    pub fn valueOf__f(f: f32) -> Result<Object> {
        todo!("abstract java/lang/Float.valueOf")
    }

    // java: parseFloat(Ljava/lang/String;)F
    pub fn parseFloat(s: String) -> Result<f32> {
        todo!("abstract java/lang/Float.parseFloat")
    }

    // java: isNaN(F)Z
    pub fn isNaN__f(v: f32) -> Result<bool> {
        todo!("abstract java/lang/Float.isNaN")
    }

    // java: isInfinite(F)Z
    pub fn isInfinite__f(v: f32) -> Result<bool> {
        todo!("abstract java/lang/Float.isInfinite")
    }

    // java: isFinite(F)Z
    pub fn isFinite(f: f32) -> Result<bool> {
        todo!("abstract java/lang/Float.isFinite")
    }

    // java: <init>(F)V
    pub fn new__f(&self, value: f32) -> Result<()> {
        todo!("abstract java/lang/Float.<init>")
    }

    // java: <init>(D)V
    pub fn new__d(&self, value: f64) -> Result<()> {
        todo!("abstract java/lang/Float.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Float.<init>")
    }

    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        todo!("abstract java/lang/Float.isNaN")
    }

    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        todo!("abstract java/lang/Float.isInfinite")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Float.toString")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Float.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Float.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Float.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Float.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Float.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Float.doubleValue")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Float.hashCode")
    }

    // java: hashCode(F)I
    pub fn hashCode__f(value: f32) -> Result<i32> {
        todo!("abstract java/lang/Float.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Float.equals")
    }

    // java: floatToIntBits(F)I
    pub fn floatToIntBits(value: f32) -> Result<i32> {
        todo!("abstract java/lang/Float.floatToIntBits")
    }

    // java: floatToRawIntBits(F)I
    pub fn floatToRawIntBits(arg0: f32) -> Result<i32> {
        todo!("native java/lang/Float.floatToRawIntBits")
    }

    // java: intBitsToFloat(I)F
    pub fn intBitsToFloat(arg0: i32) -> Result<f32> {
        todo!("native java/lang/Float.intBitsToFloat")
    }

    // java: float16ToFloat(S)F
    pub fn float16ToFloat(floatBinary16: i16) -> Result<f32> {
        todo!("abstract java/lang/Float.float16ToFloat")
    }

    // java: floatToFloat16(F)S
    pub fn floatToFloat16(f: f32) -> Result<i16> {
        todo!("abstract java/lang/Float.floatToFloat16")
    }

    // java: compareTo(Ljava/lang/Float;)I
    pub fn compareTo(&self, anotherFloat: Object) -> Result<i32> {
        todo!("abstract java/lang/Float.compareTo")
    }

    // java: compare(FF)I
    pub fn compare(f1: f32, f2: f32) -> Result<i32> {
        todo!("abstract java/lang/Float.compare")
    }

    // java: sum(FF)F
    pub fn sum(a: f32, b: f32) -> Result<f32> {
        todo!("abstract java/lang/Float.sum")
    }

    // java: max(FF)F
    pub fn max(a: f32, b: f32) -> Result<f32> {
        todo!("abstract java/lang/Float.max")
    }

    // java: min(FF)F
    pub fn min(a: f32, b: f32) -> Result<f32> {
        todo!("abstract java/lang/Float.min")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Float.describeConstable")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Object> {
        todo!("abstract java/lang/Float.resolveConstantDesc")
    }
}
