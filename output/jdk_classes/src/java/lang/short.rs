#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Short",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable",
    access      = "public final",
    source      = "Short.java",
))]
pub struct Short {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "S", access = "private final"))]
    pub value: Field<i16>,
}

impl Short {
    // java: toString(S)Ljava/lang/String;
    pub fn toString__s(s: i16) -> Result<String> {
        panic!("stub: java/lang/Short.toString:(S)Ljava/lang/String;")
    }

    // java: parseShort(Ljava/lang/String;I)S
    pub fn parseShort__str_i(s: String, radix: i32) -> Result<i16> {
        panic!("stub: java/lang/Short.parseShort:(Ljava/lang/String;I)S")
    }

    // java: parseShort(Ljava/lang/String;)S
    pub fn parseShort__str(s: String) -> Result<i16> {
        panic!("stub: java/lang/Short.parseShort:(Ljava/lang/String;)S")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Short;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        panic!("stub: java/lang/Short.valueOf:(Ljava/lang/String;I)Ljava/lang/Short;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Short;
    pub fn valueOf__str(s: String) -> Result<Object> {
        panic!("stub: java/lang/Short.valueOf:(Ljava/lang/String;)Ljava/lang/Short;")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Short.describeConstable:()Ljava/util/Optional;")
    }

    // java: valueOf(S)Ljava/lang/Short;
    pub fn valueOf__s(s: i16) -> Result<Object> {
        panic!("stub: java/lang/Short.valueOf:(S)Ljava/lang/Short;")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Short;
    pub fn decode(nm: String) -> Result<Object> {
        panic!("stub: java/lang/Short.decode:(Ljava/lang/String;)Ljava/lang/Short;")
    }

    // java: <init>(S)V
    pub fn new__s(&self, value: i16) -> Result<()> {
        panic!("stub: java/lang/Short.<init>:(S)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Short.<init>:(Ljava/lang/String;)V")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Short.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Short.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Short.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Short.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Short.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Short.doubleValue:()D")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Short.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Short.hashCode:()I")
    }

    // java: hashCode(S)I
    pub fn hashCode__s(value: i16) -> Result<i32> {
        panic!("stub: java/lang/Short.hashCode:(S)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Short.equals:(Ljava/lang/Object;)Z")
    }

    // java: compareTo(Ljava/lang/Short;)I
    pub fn compareTo(&self, anotherShort: Object) -> Result<i32> {
        panic!("stub: java/lang/Short.compareTo:(Ljava/lang/Short;)I")
    }

    // java: compare(SS)I
    pub fn compare(x: i16, y: i16) -> Result<i32> {
        panic!("stub: java/lang/Short.compare:(SS)I")
    }

    // java: compareUnsigned(SS)I
    pub fn compareUnsigned(x: i16, y: i16) -> Result<i32> {
        panic!("stub: java/lang/Short.compareUnsigned:(SS)I")
    }

    // java: reverseBytes(S)S
    pub fn reverseBytes(i: i16) -> Result<i16> {
        panic!("stub: java/lang/Short.reverseBytes:(S)S")
    }

    // java: toUnsignedInt(S)I
    pub fn toUnsignedInt(x: i16) -> Result<i32> {
        panic!("stub: java/lang/Short.toUnsignedInt:(S)I")
    }

    // java: toUnsignedLong(S)J
    pub fn toUnsignedLong(x: i16) -> Result<i64> {
        panic!("stub: java/lang/Short.toUnsignedLong:(S)J")
    }
}
