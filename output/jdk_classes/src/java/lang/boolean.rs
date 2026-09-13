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
        panic!("stub: java/lang/Boolean.<init>:(Z)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Boolean.<init>:(Ljava/lang/String;)V")
    }

    // java: parseBoolean(Ljava/lang/String;)Z
    pub fn parseBoolean(s: String) -> Result<bool> {
        panic!("stub: java/lang/Boolean.parseBoolean:(Ljava/lang/String;)Z")
    }

    // java: booleanValue()Z
    pub fn booleanValue(&self) -> Result<bool> {
        panic!("stub: java/lang/Boolean.booleanValue:()Z")
    }

    // java: valueOf(Z)Ljava/lang/Boolean;
    pub fn valueOf__z(b: bool) -> Result<bool> {
        panic!("stub: java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Boolean;
    pub fn valueOf__str(s: String) -> Result<bool> {
        panic!("stub: java/lang/Boolean.valueOf:(Ljava/lang/String;)Ljava/lang/Boolean;")
    }

    // java: toString(Z)Ljava/lang/String;
    pub fn toString__z(b: bool) -> Result<String> {
        panic!("stub: java/lang/Boolean.toString:(Z)Ljava/lang/String;")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Boolean.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Boolean.hashCode:()I")
    }

    // java: hashCode(Z)I
    pub fn hashCode__z(value: bool) -> Result<i32> {
        panic!("stub: java/lang/Boolean.hashCode:(Z)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Boolean.equals:(Ljava/lang/Object;)Z")
    }

    // java: getBoolean(Ljava/lang/String;)Z
    pub fn getBoolean(name: String) -> Result<bool> {
        panic!("stub: java/lang/Boolean.getBoolean:(Ljava/lang/String;)Z")
    }

    // java: compareTo(Ljava/lang/Boolean;)I
    pub fn compareTo(&self, b: bool) -> Result<i32> {
        panic!("stub: java/lang/Boolean.compareTo:(Ljava/lang/Boolean;)I")
    }

    // java: compare(ZZ)I
    pub fn compare(x: bool, y: bool) -> Result<i32> {
        panic!("stub: java/lang/Boolean.compare:(ZZ)I")
    }

    // java: logicalAnd(ZZ)Z
    pub fn logicalAnd(a: bool, b: bool) -> Result<bool> {
        panic!("stub: java/lang/Boolean.logicalAnd:(ZZ)Z")
    }

    // java: logicalOr(ZZ)Z
    pub fn logicalOr(a: bool, b: bool) -> Result<bool> {
        panic!("stub: java/lang/Boolean.logicalOr:(ZZ)Z")
    }

    // java: logicalXor(ZZ)Z
    pub fn logicalXor(a: bool, b: bool) -> Result<bool> {
        panic!("stub: java/lang/Boolean.logicalXor:(ZZ)Z")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Boolean.describeConstable:()Ljava/util/Optional;")
    }
}
