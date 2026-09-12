#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Boolean",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable,java/lang/Comparable,java/lang/constant/Constable",
    access      = "public final",
    source      = "Boolean.java",
))]
pub struct Boolean {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "Z", access = "private final"))]
    pub value: Field<bool>,
}

impl Boolean {
    // java: <init>(Z)V
    pub fn new__z(&self, value: bool) -> Result<()> {
        todo!("abstract java/lang/Boolean.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Boolean.<init>")
    }

    // java: parseBoolean(Ljava/lang/String;)Z
    pub fn parseBoolean(s: String) -> Result<bool> {
        todo!("abstract java/lang/Boolean.parseBoolean")
    }

    // java: booleanValue()Z
    pub fn booleanValue(&self) -> Result<bool> {
        todo!("abstract java/lang/Boolean.booleanValue")
    }

    // java: valueOf(Z)Ljava/lang/Boolean;
    pub fn valueOf__z(b: bool) -> Result<bool> {
        todo!("abstract java/lang/Boolean.valueOf")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Boolean;
    pub fn valueOf__str(s: String) -> Result<bool> {
        todo!("abstract java/lang/Boolean.valueOf")
    }

    // java: toString(Z)Ljava/lang/String;
    pub fn toString__z(b: bool) -> Result<String> {
        todo!("abstract java/lang/Boolean.toString")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Boolean.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Boolean.hashCode")
    }

    // java: hashCode(Z)I
    pub fn hashCode__z(value: bool) -> Result<i32> {
        todo!("abstract java/lang/Boolean.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Boolean.equals")
    }

    // java: getBoolean(Ljava/lang/String;)Z
    pub fn getBoolean(name: String) -> Result<bool> {
        todo!("abstract java/lang/Boolean.getBoolean")
    }

    // java: compareTo(Ljava/lang/Boolean;)I
    pub fn compareTo(&self, b: bool) -> Result<i32> {
        todo!("abstract java/lang/Boolean.compareTo")
    }

    // java: compare(ZZ)I
    pub fn compare(x: bool, y: bool) -> Result<i32> {
        todo!("abstract java/lang/Boolean.compare")
    }

    // java: logicalAnd(ZZ)Z
    pub fn logicalAnd(a: bool, b: bool) -> Result<bool> {
        todo!("abstract java/lang/Boolean.logicalAnd")
    }

    // java: logicalOr(ZZ)Z
    pub fn logicalOr(a: bool, b: bool) -> Result<bool> {
        todo!("abstract java/lang/Boolean.logicalOr")
    }

    // java: logicalXor(ZZ)Z
    pub fn logicalXor(a: bool, b: bool) -> Result<bool> {
        todo!("abstract java/lang/Boolean.logicalXor")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Boolean.describeConstable")
    }
}
