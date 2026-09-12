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
        todo!("abstract java/lang/Short.toString")
    }

    // java: parseShort(Ljava/lang/String;I)S
    pub fn parseShort__str_i(s: String, radix: i32) -> Result<i16> {
        todo!("abstract java/lang/Short.parseShort")
    }

    // java: parseShort(Ljava/lang/String;)S
    pub fn parseShort__str(s: String) -> Result<i16> {
        todo!("abstract java/lang/Short.parseShort")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Short;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        todo!("abstract java/lang/Short.valueOf")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Short;
    pub fn valueOf__str(s: String) -> Result<Object> {
        todo!("abstract java/lang/Short.valueOf")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Short.describeConstable")
    }

    // java: valueOf(S)Ljava/lang/Short;
    pub fn valueOf__s(s: i16) -> Result<Object> {
        todo!("abstract java/lang/Short.valueOf")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Short;
    pub fn decode(nm: String) -> Result<Object> {
        todo!("abstract java/lang/Short.decode")
    }

    // java: <init>(S)V
    pub fn new__s(&self, value: i16) -> Result<()> {
        todo!("abstract java/lang/Short.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Short.<init>")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Short.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Short.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Short.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Short.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Short.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Short.doubleValue")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Short.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Short.hashCode")
    }

    // java: hashCode(S)I
    pub fn hashCode__s(value: i16) -> Result<i32> {
        todo!("abstract java/lang/Short.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Short.equals")
    }

    // java: compareTo(Ljava/lang/Short;)I
    pub fn compareTo(&self, anotherShort: Object) -> Result<i32> {
        todo!("abstract java/lang/Short.compareTo")
    }

    // java: compare(SS)I
    pub fn compare(x: i16, y: i16) -> Result<i32> {
        todo!("abstract java/lang/Short.compare")
    }

    // java: compareUnsigned(SS)I
    pub fn compareUnsigned(x: i16, y: i16) -> Result<i32> {
        todo!("abstract java/lang/Short.compareUnsigned")
    }

    // java: reverseBytes(S)S
    pub fn reverseBytes(i: i16) -> Result<i16> {
        todo!("abstract java/lang/Short.reverseBytes")
    }

    // java: toUnsignedInt(S)I
    pub fn toUnsignedInt(x: i16) -> Result<i32> {
        todo!("abstract java/lang/Short.toUnsignedInt")
    }

    // java: toUnsignedLong(S)J
    pub fn toUnsignedLong(x: i16) -> Result<i64> {
        todo!("abstract java/lang/Short.toUnsignedLong")
    }
}
