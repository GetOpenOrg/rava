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
    // java: <init>(Z)V
    pub fn new__z(value: bool) -> Result<Self> {
        let this = Self { value: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { value: Field::new(false) };
        let _t0: bool = Boolean::parseBoolean(s)?;
        /* invokespecial Method java/lang/Boolean.<init>:(Z)V */
        Ok(this)
    }

    // java: parseBoolean(Ljava/lang/String;)Z
    pub fn parseBoolean(s: String) -> Result<bool> {
        let _t0 = String::from("true").equalsIgnoreCase(s)?;
        Ok(_t0)
    }

    // java: booleanValue()Z
    pub fn booleanValue(&self) -> Result<bool> {
        let this = self;
        Ok(this.value.get())
    }

    // java: valueOf(Z)Ljava/lang/Boolean;
    // java: valueOf(Z)Ljava/lang/Boolean;
    pub fn valueOf__z(b: bool) -> Result<bool> {
        Ok(Boolean::FALSE())
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Boolean;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Boolean;
    pub fn valueOf__str(s: String) -> Result<bool> {
        let _t0: bool = Boolean::parseBoolean(s)?;
        Ok(Boolean::FALSE())
    }

    // java: toString(Z)Ljava/lang/String;
    // java: toString(Z)Ljava/lang/String;
    pub fn toString__z(b: bool) -> Result<String> {
        Ok(String::from_owned(format!("{}", b)))
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::from_owned(format!("{}", this.value.get())))
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Boolean::hashCode__z(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(Z)I
    // java: hashCode(Z)I
    pub fn hashCode__z(value: bool) -> Result<i32> {
        Ok(1237i32)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(this.value.get() == obj);
        Ok(0i32)
    }

    // java: getBoolean(Ljava/lang/String;)Z
    pub fn getBoolean(name: String) -> Result<bool> {
        let mut result: i32 = 0i32;
        let _t0: String = System::getProperty(name)?;
        let _t1: bool = Boolean::parseBoolean(_t0)?;
        result = _t1;
        let mut local_2: i32 = todo!("stack underflow");
        Ok(result)
    }

    // java: compareTo(Ljava/lang/Boolean;)I
    pub fn compareTo(&self, b: bool) -> Result<i32> {
        let this = self;
        let _t0: i32 = Boolean::compare(this.value.get(), b.value.get())?;
        Ok(_t0)
    }

    // java: compare(ZZ)I
    pub fn compare(x: bool, y: bool) -> Result<i32> {
        Ok(x==0i32)
    }

    // java: logicalAnd(ZZ)Z
    pub fn logicalAnd(a: bool, b: bool) -> Result<bool> {
        Ok(b!=0i32)
    }

    // java: logicalOr(ZZ)Z
    pub fn logicalOr(a: bool, b: bool) -> Result<bool> {
        Ok(b!=0i32)
    }

    // java: logicalXor(ZZ)Z
    pub fn logicalXor(a: bool, b: bool) -> Result<bool> {
        Ok((a^b))
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(ConstantDescs::FALSE())?;
        Ok(_t0)
    }
}
