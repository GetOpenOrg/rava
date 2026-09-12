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
    // java: toString(B)Ljava/lang/String;
    pub fn toString__b(b: i8) -> Result<String> {
        let _t0: String = Integer::toString__i(b)?;
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this;
        let _t1: Object = DynamicConstantDesc::ofNamed(ConstantDescs::BSM_EXPLICIT_CAST(), String::from("_"), ConstantDescs::CD_byte(), &_arr0)?;
        let _t2: Object = Optional::of(_t1)?;
        Ok(_t2)
    }

    // java: valueOf(B)Ljava/lang/Byte;
    // java: valueOf(B)Ljava/lang/Byte;
    pub fn valueOf__b(b: i8) -> Result<Object> {
        let mut offset: i32 = 128i32;
        Ok(Byte_ByteCache::cache()[(b).wrapping_add(128i32) as usize].clone())
    }

    // java: parseByte(Ljava/lang/String;I)B
    // java: parseByte(Ljava/lang/String;I)B
    pub fn parseByte__str_i(s: String, radix: i32) -> Result<i8> {
        let _t0: i32 = Integer::parseInt__str_i(s, radix)?;
        let mut i: i32 = _t0;
        String::new().append(&String::from("Value out of range. Value:\""))?;
        String::new().append(&s)?;
        String::new().append(&String::from("\" Radix:"))?;
        String::new().append(&radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: i2b  */
        Ok(i)
    }

    // java: parseByte(Ljava/lang/String;)B
    // java: parseByte(Ljava/lang/String;)B
    pub fn parseByte__str(s: String) -> Result<i8> {
        let _t0: i8 = Byte::parseByte__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Byte;
    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Byte;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        let _t0: i8 = Byte::parseByte__str_i(s, radix)?;
        let _t1: Object = Byte::valueOf__b(_t0)?;
        Ok(_t1)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Byte;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn valueOf__str(s: String) -> Result<Object> {
        let _t0: Object = Byte::valueOf__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Byte;
    pub fn decode(nm: String) -> Result<Object> {
        let _t0: i32 = Integer::decode(nm)?;
        let mut i: i32 = _t0;
        String::new().append(&String::from("Value"))?;
        String::new().append(&i)?;
        String::new().append(&String::from("out of range from input"))?;
        String::new().append(&nm)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: i2b  */
        let _t1: Object = Byte::valueOf__b(i)?;
        Ok(_t1)
    }

    // java: <init>(B)V
    // java: <init>(B)V
    pub fn new__b(value: i8) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: i8 = Byte::parseByte__str_i(s, 10i32)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        Ok(this.value.get())
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
        /* TODO: i2s  */
        Ok(this.value.get())
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        Ok(this.value.get())
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        Ok((this.value.get() as i64))
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        Ok((this.value.get() as f32))
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        Ok((this.value.get() as f64))
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Integer::toString__i(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Byte::hashCode__b(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(B)I
    // java: hashCode(B)I
    pub fn hashCode__b(value: i8) -> Result<i32> {
        Ok(value)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(this.value.get() == obj);
        Ok(0i32)
    }

    // java: compareTo(Ljava/lang/Byte;)I
    pub fn compareTo(&self, anotherByte: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = Byte::compare(this.value.get(), anotherByte.value.get())?;
        Ok(_t0)
    }

    // java: compare(BB)I
    pub fn compare(x: i8, y: i8) -> Result<i32> {
        Ok((x).wrapping_sub(y))
    }

    // java: compareUnsigned(BB)I
    pub fn compareUnsigned(x: i8, y: i8) -> Result<i32> {
        let _t0: i32 = Byte::toUnsignedInt(x)?;
        let _t1: i32 = Byte::toUnsignedInt(y)?;
        Ok((_t0).wrapping_sub(_t1))
    }

    // java: toUnsignedInt(B)I
    pub fn toUnsignedInt(x: i8) -> Result<i32> {
        Ok((x&255i32))
    }

    // java: toUnsignedLong(B)J
    pub fn toUnsignedLong(x: i8) -> Result<i64> {
        /* TODO: land  */
        Ok(255i64)
    }
}
