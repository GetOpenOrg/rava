#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Byte",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable",
    access      = "public final",
    source      = "Byte.java",
))]
pub struct Byte {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "B", access = "private final"))]
    pub value: Field<i8>,
}

impl Byte {
    // java: toString(B)Ljava/lang/String;
    pub fn toString__b(b: i8) -> Result<String> {
        panic!("stub: java/lang/Byte.toString:(B)Ljava/lang/String;")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Byte.describeConstable:()Ljava/util/Optional;")
    }

    // java: valueOf(B)Ljava/lang/Byte;
    pub fn valueOf__b(b: i8) -> Result<Object> {
        panic!("stub: java/lang/Byte.valueOf:(B)Ljava/lang/Byte;")
    }

    // java: parseByte(Ljava/lang/String;I)B
    pub fn parseByte__str_i(s: String, radix: i32) -> Result<i8> {
        panic!("stub: java/lang/Byte.parseByte:(Ljava/lang/String;I)B")
    }

    // java: parseByte(Ljava/lang/String;)B
    pub fn parseByte__str(s: String) -> Result<i8> {
        panic!("stub: java/lang/Byte.parseByte:(Ljava/lang/String;)B")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Byte;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        panic!("stub: java/lang/Byte.valueOf:(Ljava/lang/String;I)Ljava/lang/Byte;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn valueOf__str(s: String) -> Result<Object> {
        panic!("stub: java/lang/Byte.valueOf:(Ljava/lang/String;)Ljava/lang/Byte;")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn decode(nm: String) -> Result<Object> {
        panic!("stub: java/lang/Byte.decode:(Ljava/lang/String;)Ljava/lang/Byte;")
    }

    // java: <init>(B)V
    pub fn new__b(&self, value: i8) -> Result<()> {
        panic!("stub: java/lang/Byte.<init>:(B)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Byte.<init>:(Ljava/lang/String;)V")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Byte.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Byte.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Byte.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Byte.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Byte.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Byte.doubleValue:()D")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Byte.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Byte.hashCode:()I")
    }

    // java: hashCode(B)I
    pub fn hashCode__b(value: i8) -> Result<i32> {
        panic!("stub: java/lang/Byte.hashCode:(B)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Byte.equals:(Ljava/lang/Object;)Z")
    }

    // java: compareTo(Ljava/lang/Byte;)I
    pub fn compareTo(&self, anotherByte: Object) -> Result<i32> {
        panic!("stub: java/lang/Byte.compareTo:(Ljava/lang/Byte;)I")
    }

    // java: compare(BB)I
    pub fn compare(x: i8, y: i8) -> Result<i32> {
        panic!("stub: java/lang/Byte.compare:(BB)I")
    }

    // java: compareUnsigned(BB)I
    pub fn compareUnsigned(x: i8, y: i8) -> Result<i32> {
        panic!("stub: java/lang/Byte.compareUnsigned:(BB)I")
    }

    // java: toUnsignedInt(B)I
    pub fn toUnsignedInt(x: i8) -> Result<i32> {
        panic!("stub: java/lang/Byte.toUnsignedInt:(B)I")
    }

    // java: toUnsignedLong(B)J
    pub fn toUnsignedLong(x: i8) -> Result<i64> {
        panic!("stub: java/lang/Byte.toUnsignedLong:(B)J")
    }
}
