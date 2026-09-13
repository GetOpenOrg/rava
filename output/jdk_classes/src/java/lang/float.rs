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
        panic!("stub: java/lang/Float.toString:(F)Ljava/lang/String;")
    }

    // java: toHexString(F)Ljava/lang/String;
    pub fn toHexString(f: f32) -> Result<String> {
        panic!("stub: java/lang/Float.toHexString:(F)Ljava/lang/String;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Float;
    pub fn valueOf__str(s: String) -> Result<Object> {
        panic!("stub: java/lang/Float.valueOf:(Ljava/lang/String;)Ljava/lang/Float;")
    }

    // java: valueOf(F)Ljava/lang/Float;
    pub fn valueOf__f(f: f32) -> Result<Object> {
        panic!("stub: java/lang/Float.valueOf:(F)Ljava/lang/Float;")
    }

    // java: parseFloat(Ljava/lang/String;)F
    pub fn parseFloat(s: String) -> Result<f32> {
        panic!("stub: java/lang/Float.parseFloat:(Ljava/lang/String;)F")
    }

    // java: isNaN(F)Z
    pub fn isNaN__f(v: f32) -> Result<bool> {
        panic!("stub: java/lang/Float.isNaN:(F)Z")
    }

    // java: isInfinite(F)Z
    pub fn isInfinite__f(v: f32) -> Result<bool> {
        panic!("stub: java/lang/Float.isInfinite:(F)Z")
    }

    // java: isFinite(F)Z
    pub fn isFinite(f: f32) -> Result<bool> {
        panic!("stub: java/lang/Float.isFinite:(F)Z")
    }

    // java: <init>(F)V
    pub fn new__f(&self, value: f32) -> Result<()> {
        panic!("stub: java/lang/Float.<init>:(F)V")
    }

    // java: <init>(D)V
    pub fn new__d(&self, value: f64) -> Result<()> {
        panic!("stub: java/lang/Float.<init>:(D)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Float.<init>:(Ljava/lang/String;)V")
    }

    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        panic!("stub: java/lang/Float.isNaN:()Z")
    }

    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        panic!("stub: java/lang/Float.isInfinite:()Z")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Float.toString:()Ljava/lang/String;")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Float.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Float.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Float.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Float.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Float.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Float.doubleValue:()D")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Float.hashCode:()I")
    }

    // java: hashCode(F)I
    pub fn hashCode__f(value: f32) -> Result<i32> {
        panic!("stub: java/lang/Float.hashCode:(F)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Float.equals:(Ljava/lang/Object;)Z")
    }

    // java: floatToIntBits(F)I
    pub fn floatToIntBits(value: f32) -> Result<i32> {
        panic!("stub: java/lang/Float.floatToIntBits:(F)I")
    }

    // java: floatToRawIntBits(F)I
    pub fn floatToRawIntBits(arg0: f32) -> Result<i32> {
        panic!("native: java/lang/Float.floatToRawIntBits:(F)I")
    }

    // java: intBitsToFloat(I)F
    pub fn intBitsToFloat(arg0: i32) -> Result<f32> {
        panic!("native: java/lang/Float.intBitsToFloat:(I)F")
    }

    // java: float16ToFloat(S)F
    pub fn float16ToFloat(floatBinary16: i16) -> Result<f32> {
        panic!("stub: java/lang/Float.float16ToFloat:(S)F")
    }

    // java: floatToFloat16(F)S
    pub fn floatToFloat16(f: f32) -> Result<i16> {
        panic!("stub: java/lang/Float.floatToFloat16:(F)S")
    }

    // java: compareTo(Ljava/lang/Float;)I
    pub fn compareTo(&self, anotherFloat: Object) -> Result<i32> {
        panic!("stub: java/lang/Float.compareTo:(Ljava/lang/Float;)I")
    }

    // java: compare(FF)I
    pub fn compare(f1: f32, f2: f32) -> Result<i32> {
        panic!("stub: java/lang/Float.compare:(FF)I")
    }

    // java: sum(FF)F
    pub fn sum(a: f32, b: f32) -> Result<f32> {
        panic!("stub: java/lang/Float.sum:(FF)F")
    }

    // java: max(FF)F
    pub fn max(a: f32, b: f32) -> Result<f32> {
        panic!("stub: java/lang/Float.max:(FF)F")
    }

    // java: min(FF)F
    pub fn min(a: f32, b: f32) -> Result<f32> {
        panic!("stub: java/lang/Float.min:(FF)F")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Float.describeConstable:()Ljava/util/Optional;")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Object> {
        panic!("stub: java/lang/Float.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;")
    }
}
