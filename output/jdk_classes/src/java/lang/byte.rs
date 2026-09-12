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
        todo!("abstract java/lang/Byte.toString")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Byte.describeConstable")
    }

    // java: valueOf(B)Ljava/lang/Byte;
    pub fn valueOf__b(b: i8) -> Result<Object> {
        todo!("abstract java/lang/Byte.valueOf")
    }

    // java: parseByte(Ljava/lang/String;I)B
    pub fn parseByte__str_i(s: String, radix: i32) -> Result<i8> {
        todo!("abstract java/lang/Byte.parseByte")
    }

    // java: parseByte(Ljava/lang/String;)B
    pub fn parseByte__str(s: String) -> Result<i8> {
        todo!("abstract java/lang/Byte.parseByte")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Byte;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        todo!("abstract java/lang/Byte.valueOf")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn valueOf__str(s: String) -> Result<Object> {
        todo!("abstract java/lang/Byte.valueOf")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn decode(nm: String) -> Result<Object> {
        todo!("abstract java/lang/Byte.decode")
    }

    // java: <init>(B)V
    pub fn new__b(&self, value: i8) -> Result<()> {
        todo!("abstract java/lang/Byte.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Byte.<init>")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Byte.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Byte.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Byte.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Byte.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Byte.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Byte.doubleValue")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Byte.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Byte.hashCode")
    }

    // java: hashCode(B)I
    pub fn hashCode__b(value: i8) -> Result<i32> {
        todo!("abstract java/lang/Byte.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Byte.equals")
    }

    // java: compareTo(Ljava/lang/Byte;)I
    pub fn compareTo(&self, anotherByte: Object) -> Result<i32> {
        todo!("abstract java/lang/Byte.compareTo")
    }

    // java: compare(BB)I
    pub fn compare(x: i8, y: i8) -> Result<i32> {
        todo!("abstract java/lang/Byte.compare")
    }

    // java: compareUnsigned(BB)I
    pub fn compareUnsigned(x: i8, y: i8) -> Result<i32> {
        todo!("abstract java/lang/Byte.compareUnsigned")
    }

    // java: toUnsignedInt(B)I
    pub fn toUnsignedInt(x: i8) -> Result<i32> {
        todo!("abstract java/lang/Byte.toUnsignedInt")
    }

    // java: toUnsignedLong(B)J
    pub fn toUnsignedLong(x: i8) -> Result<i64> {
        todo!("abstract java/lang/Byte.toUnsignedLong")
    }
}
