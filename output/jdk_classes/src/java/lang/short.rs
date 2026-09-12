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
    // java: toString(S)Ljava/lang/String;
    pub fn toString__s(s: i16) -> Result<String> {
        let _t0: String = Integer::toString__i(s)?;
        Ok(_t0)
    }

    // java: parseShort(Ljava/lang/String;I)S
    // java: parseShort(Ljava/lang/String;I)S
    pub fn parseShort__str_i(s: String, radix: i32) -> Result<i16> {
        let _t0: i32 = Integer::parseInt__str_i(s, radix)?;
        let mut i: i32 = _t0;
        String::new().append(&String::from("Value out of range. Value:\""))?;
        String::new().append(&s)?;
        String::new().append(&String::from("\" Radix:"))?;
        String::new().append(&radix)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: i2s  */
        Ok(i)
    }

    // java: parseShort(Ljava/lang/String;)S
    // java: parseShort(Ljava/lang/String;)S
    pub fn parseShort__str(s: String) -> Result<i16> {
        let _t0: i16 = Short::parseShort__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Short;
    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Short;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<Object> {
        let _t0: i16 = Short::parseShort__str_i(s, radix)?;
        let _t1: Object = Short::valueOf__s(_t0)?;
        Ok(_t1)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Short;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Short;
    pub fn valueOf__str(s: String) -> Result<Object> {
        let _t0: Object = Short::valueOf__str_i(s, 10i32)?;
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this;
        let _t1: Object = DynamicConstantDesc::ofNamed(ConstantDescs::BSM_EXPLICIT_CAST(), String::from("_"), ConstantDescs::CD_short(), &_arr0)?;
        let _t2: Object = Optional::of(_t1)?;
        Ok(_t2)
    }

    // java: valueOf(S)Ljava/lang/Short;
    // java: valueOf(S)Ljava/lang/Short;
    pub fn valueOf__s(s: i16) -> Result<Object> {
        let mut offset: i32 = 128i32;
        let mut sAsInt: i32 = s;
        return Ok(Short_ShortCache::cache()[(sAsInt).wrapping_add(128i32) as usize].clone());
        Ok(Short::new(s)?)
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Short;
    pub fn decode(nm: String) -> Result<Object> {
        let _t0: i32 = Integer::decode(nm)?;
        let mut i: i32 = _t0;
        String::new().append(&String::from("Value"))?;
        String::new().append(&i)?;
        String::new().append(&String::from("out of range from input"))?;
        String::new().append(&nm)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: i2s  */
        let _t1: Object = Short::valueOf__s(i)?;
        Ok(_t1)
    }

    // java: <init>(S)V
    // java: <init>(S)V
    pub fn new__s(value: i16) -> Result<Self> {
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
        let _t0: i16 = Short::parseShort__str_i(s, 10i32)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        /* TODO: i2b  */
        Ok(this.value.get())
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
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
        let _t0: i32 = Short::hashCode__s(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(S)I
    // java: hashCode(S)I
    pub fn hashCode__s(value: i16) -> Result<i32> {
        Ok(value)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(this.value.get() == obj);
        Ok(0i32)
    }

    // java: compareTo(Ljava/lang/Short;)I
    pub fn compareTo(&self, anotherShort: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = Short::compare(this.value.get(), anotherShort.value.get())?;
        Ok(_t0)
    }

    // java: compare(SS)I
    pub fn compare(x: i16, y: i16) -> Result<i32> {
        Ok((x).wrapping_sub(y))
    }

    // java: compareUnsigned(SS)I
    pub fn compareUnsigned(x: i16, y: i16) -> Result<i32> {
        let _t0: i32 = Short::toUnsignedInt(x)?;
        let _t1: i32 = Short::toUnsignedInt(y)?;
        Ok((_t0).wrapping_sub(_t1))
    }

    // java: reverseBytes(S)S
    pub fn reverseBytes(i: i16) -> Result<i16> {
        /* TODO: i2s  */
        Ok((((i&65280i32)>>((8i32&0x1f)))|(i<<(8i32&0x1f))))
    }

    // java: toUnsignedInt(S)I
    pub fn toUnsignedInt(x: i16) -> Result<i32> {
        Ok((x&65535i32))
    }

    // java: toUnsignedLong(S)J
    pub fn toUnsignedLong(x: i16) -> Result<i64> {
        /* TODO: land  */
        Ok(65535i64)
    }
}
