#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Double",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Double.java",
))]
pub struct Double {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "D", access = "private final"))]
    pub value: Field<f64>,
}

impl Double {
    // java: toString(D)Ljava/lang/String;
    // java: toString(D)Ljava/lang/String;
    pub fn toString__d(d: f64) -> Result<String> {
        let _t0: String = DoubleToDecimal::toString(d)?;
        Ok(_t0)
    }

    // java: toHexString(D)Ljava/lang/String;
    pub fn toHexString(d: f64) -> Result<String> {
        let _t0: bool = Double::isFinite(d)?;
        let _t1: String = Double::toString__d(d)?;
        return Ok(_t1);
        let mut answer: String = String::new();
        let _t2: f64 = (1f64).abs();
        /* TODO: dcmpl  */
        answer.append(&String::from("-"))?;
        answer.append(&String::from("0x"))?;
        let _t3: f64 = (d).abs();
        d = _t3;
        /* TODO: dcmpl  */
        answer.append(&String::from("0.0p0"))?;
        /* TODO: dcmpg  */
        let mut subnormal: i32 = 2.2250738585072014e-308f64<0i32;
        let _t4: i64 = Double::doubleToLongBits(d)?;
        /* TODO: land  */
        /* TODO: lor  */
        let mut signifBits: i64 = 1152921504606846976i64;
        String::from("0.").append(&String::from("1."))?;
        let _ = String::from("0.");
        let _t5: String = Long::toHexString(signifBits)?;
        let _t6 = _t5.substring(3i32, 16i32)?;
        let mut signif: String = _t6;
        let _t7 = signif.equals(String::from("0000000000000"))?;
        let _t8 = signif.replaceFirst(String::from("0{1,12}$"), String::from(""))?;
        String::from("0").append(&_t8)?;
        let _ = String::from("0");
        answer.append(&112i32)?;
        let _t9: i32 = (d).abs();
        -1022i32.append(&_t9)?;
        Ok(answer)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Double;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Double;
    pub fn valueOf__str(s: String) -> Result<f64> {
        let _t0: f64 = Double::parseDouble(s)?;
        Ok(Double::new(_t0)?)
    }

    // java: valueOf(D)Ljava/lang/Double;
    // java: valueOf(D)Ljava/lang/Double;
    pub fn valueOf__d(d: f64) -> Result<f64> {
        Ok(Double::new(d)?)
    }

    // java: parseDouble(Ljava/lang/String;)D
    pub fn parseDouble(s: String) -> Result<f64> {
        let _t0: f64 = FloatingDecimal::parseDouble(s)?;
        Ok(_t0)
    }

    // java: isNaN(D)Z
    // java: isNaN(D)Z
    pub fn isNaN__d(v: f64) -> Result<bool> {
        /* TODO: dcmpl  */
        Ok(v!=0i32)
    }

    // java: isInfinite(D)Z
    // java: isInfinite(D)Z
    pub fn isInfinite__d(v: f64) -> Result<bool> {
        let _t0: f64 = (v).abs();
        /* TODO: dcmpl  */
        Ok(1.7976931348623157e+308f64>0i32)
    }

    // java: isFinite(D)Z
    pub fn isFinite(d: f64) -> Result<bool> {
        let _t0: f64 = (d).abs();
        /* TODO: dcmpg  */
        Ok(1.7976931348623157e+308f64<=0i32)
    }

    // java: <init>(D)V
    // java: <init>(D)V
    pub fn new__d(value: f64) -> Result<Self> {
        let this = Self { value: Field::new(0.0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { value: Field::new(0.0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: f64 = Double::parseDouble(s)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: isNaN()Z
    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = Double::isNaN__d(this.value.get())?;
        Ok(_t0)
    }

    // java: isInfinite()Z
    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = Double::isInfinite__d(this.value.get())?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Double::toString__d(this.value.get())?;
        Ok(_t0)
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        let this = self;
        /* TODO: i2b  */
        Ok((this.value.get() as i32))
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        let this = self;
        /* TODO: i2s  */
        Ok((this.value.get() as i32))
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        let this = self;
        Ok((this.value.get() as i32))
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        let this = self;
        /* TODO: d2l  */
        Ok(this.value.get())
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        Ok((this.value.get() as f32))
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        Ok(this.value.get())
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Double::hashCode__d(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(D)I
    // java: hashCode(D)I
    pub fn hashCode__d(value: f64) -> Result<i32> {
        let _t0: i64 = Double::doubleToLongBits(value)?;
        let _t1: i32 = Long::hashCode__l(_t0)?;
        Ok(_t1)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        let _t0: i64 = Double::doubleToLongBits(obj.value.get())?;
        let _t1: i64 = Double::doubleToLongBits(this.value.get())?;
        /* TODO: lcmp  */
        Ok(_t1==0i32)
    }

    // java: doubleToLongBits(D)J
    pub fn doubleToLongBits(value: f64) -> Result<i64> {
        let _t0: bool = Double::isNaN__d(value)?;
        let _t1: i64 = Double::doubleToRawLongBits(value)?;
        return Ok(_t1);
        Ok(9221120237041090560i64)
    }

    // java: doubleToRawLongBits(D)J
    pub fn doubleToRawLongBits(arg0: f64) -> Result<i64> {
        todo!("native java/lang/Double.doubleToRawLongBits")
    }

    // java: longBitsToDouble(J)D
    pub fn longBitsToDouble(arg0: i64) -> Result<f64> {
        todo!("native java/lang/Double.longBitsToDouble")
    }

    // java: compareTo(Ljava/lang/Double;)I
    pub fn compareTo(&self, anotherDouble: f64) -> Result<i32> {
        let this = self;
        let _t0: i32 = Double::compare(this.value.get(), anotherDouble.value.get())?;
        Ok(_t0)
    }

    // java: compare(DD)I
    pub fn compare(d1: f64, arg_1: f64) -> Result<i32> {
        /* TODO: dcmpg  */
        return Ok(-1i32);
        /* TODO: dcmpl  */
        return Ok(1i32);
        let _t0: i64 = Double::doubleToLongBits(d1)?;
        let mut thisBits: i64 = _t0;
        let _t1: i64 = Double::doubleToLongBits(local_2)?;
        let mut anotherBits: i64 = _t1;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        Ok(anotherBits>=0i32)
    }

    // java: sum(DD)D
    pub fn sum(a: f64, arg_1: f64) -> Result<f64> {
        Ok((a+local_2))
    }

    // java: max(DD)D
    pub fn max(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = (a).max(local_2);
        Ok(_t0)
    }

    // java: min(DD)D
    pub fn min(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = (a).min(local_2);
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(this)?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<f64> {
        let this = self;
        Ok(this)
    }
}
