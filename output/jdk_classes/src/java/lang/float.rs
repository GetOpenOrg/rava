#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Float",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Float.java",
))]
pub struct Float {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "F", access = "private final"))]
    pub value: Field<f32>,
}

impl Float {
    // java: toString(F)Ljava/lang/String;
    // java: toString(F)Ljava/lang/String;
    pub fn toString__f(f: f32) -> Result<String> {
        let _t0: String = FloatToDecimal::toString(f)?;
        Ok(_t0)
    }

    // java: toHexString(F)Ljava/lang/String;
    pub fn toHexString(f: f32) -> Result<String> {
        let _t0: f32 = (f).abs();
        /* TODO: fcmpg  */
        /* TODO: fcmpl  */
        let _t1: f64 = ((f as f64)).abs();
        let _t2: String = Double::toHexString(_t1)?;
        let mut s: String = _t2;
        let _t3 = s.replaceFirst(String::from("p-1022$"), String::from("p-126"))?;
        return Ok(_t3);
        let _t4: String = Double::toHexString((f as f64))?;
        Ok(_t4)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Float;
    // java: valueOf(Ljava/lang/String;)Ljava/lang/Float;
    pub fn valueOf__str(s: String) -> Result<Object> {
        let _t0: f32 = Float::parseFloat(s)?;
        Ok(Float::new(_t0)?)
    }

    // java: valueOf(F)Ljava/lang/Float;
    // java: valueOf(F)Ljava/lang/Float;
    pub fn valueOf__f(f: f32) -> Result<Object> {
        Ok(Float::new(f)?)
    }

    // java: parseFloat(Ljava/lang/String;)F
    pub fn parseFloat(s: String) -> Result<f32> {
        let _t0: f32 = FloatingDecimal::parseFloat(s)?;
        Ok(_t0)
    }

    // java: isNaN(F)Z
    // java: isNaN(F)Z
    pub fn isNaN__f(v: f32) -> Result<bool> {
        /* TODO: fcmpl  */
        Ok(v!=0i32)
    }

    // java: isInfinite(F)Z
    // java: isInfinite(F)Z
    pub fn isInfinite__f(v: f32) -> Result<bool> {
        let _t0: f32 = (v).abs();
        /* TODO: fcmpl  */
        Ok(3.4028234663852886e+38f32>0i32)
    }

    // java: isFinite(F)Z
    pub fn isFinite(f: f32) -> Result<bool> {
        let _t0: f32 = (f).abs();
        /* TODO: fcmpg  */
        Ok(3.4028234663852886e+38f32<=0i32)
    }

    // java: <init>(F)V
    // java: <init>(F)V
    pub fn new__f(value: f32) -> Result<Self> {
        let this = Self { value: Field::new(0.0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set(value);
        Ok(this)
    }

    // java: <init>(D)V
    // java: <init>(D)V
    pub fn new__d(value: f64) -> Result<Self> {
        let this = Self { value: Field::new(0.0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        this.value.set((value as f32));
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { value: Field::new(0.0) };
        /* invokespecial Method java/lang/Number.<init>:()V */
        let _t0: f32 = Float::parseFloat(s)?;
        this.value.set(_t0);
        Ok(this)
    }

    // java: isNaN()Z
    // java: isNaN()Z
    pub fn isNaN(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = Float::isNaN__f(this.value.get())?;
        Ok(_t0)
    }

    // java: isInfinite()Z
    // java: isInfinite()Z
    pub fn isInfinite(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = Float::isInfinite__f(this.value.get())?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = Float::toString__f(this.value.get())?;
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
        /* TODO: f2l  */
        Ok(this.value.get())
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        let this = self;
        Ok(this.value.get())
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        let this = self;
        Ok((this.value.get() as f64))
    }

    // java: hashCode()I
    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Float::hashCode__f(this.value.get())?;
        Ok(_t0)
    }

    // java: hashCode(F)I
    // java: hashCode(F)I
    pub fn hashCode__f(value: f32) -> Result<i32> {
        let _t0: i32 = Float::floatToIntBits(value)?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        let _t0: i32 = Float::floatToIntBits(obj.value.get())?;
        let _t1: i32 = Float::floatToIntBits(this.value.get())?;
        Ok(_t0 == _t1)
    }

    // java: floatToIntBits(F)I
    pub fn floatToIntBits(value: f32) -> Result<i32> {
        let _t0: bool = Float::isNaN__f(value)?;
        let _t1: i32 = Float::floatToRawIntBits(value)?;
        return Ok(_t1);
        Ok(2143289344i32)
    }

    // java: floatToRawIntBits(F)I
    pub fn floatToRawIntBits(arg0: f32) -> Result<i32> {
        todo!("native java/lang/Float.floatToRawIntBits")
    }

    // java: intBitsToFloat(I)F
    pub fn intBitsToFloat(arg0: i32) -> Result<f32> {
        todo!("native java/lang/Float.intBitsToFloat")
    }

    // java: float16ToFloat(S)F
    pub fn float16ToFloat(floatBinary16: i16) -> Result<f32> {
        let mut bin16arg: i32 = floatBinary16;
        let mut bin16SignBit: i32 = (32768i32&bin16arg);
        let mut bin16ExpBits: i32 = (31744i32&bin16arg);
        let mut bin16SignifBits: i32 = (1023i32&bin16arg);
        let mut SIGNIF_SHIFT: i32 = 13i32;
        let mut sign: f32 = 1f32;
        let mut bin16Exp: i32 = ((bin16ExpBits>>((10i32&0x1f)))).wrapping_sub(15i32);
        return Ok((sign*(5.960464477539063e-08f32*(bin16SignifBits as f32))));
        let _t0: f32 = Float::intBitsToFloat((((bin16SignBit<<(16i32&0x1f))|2139095040i32)|(bin16SignifBits<<(13i32&0x1f))))?;
        return Ok(_t0);
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut floatExpBits: i32 = ((bin16Exp).wrapping_add(127i32)<<(23i32&0x1f));
        let _t1: f32 = Float::intBitsToFloat((((bin16SignBit<<(16i32&0x1f))|floatExpBits)|(bin16SignifBits<<(13i32&0x1f))))?;
        Ok(_t1)
    }

    // java: floatToFloat16(F)S
    pub fn floatToFloat16(f: f32) -> Result<i16> {
        let _t0: i32 = Float::floatToRawIntBits(f)?;
        let mut doppel: i32 = _t0;
        /* TODO: i2s  */
        let mut sign_bit: i32 = ((doppel&-2147483648i32)>>((16i32&0x1f)));
        let _t1: bool = Float::isNaN__f(f)?;
        /* TODO: i2s  */
        return Ok(((((sign_bit|31744i32)|((doppel&8380416i32)>>((13i32&0x1f))))|((doppel&8176i32)>>((4i32&0x1f))))|(doppel&15i32)));
        let _t2: f32 = (f).abs();
        let mut abs_f: f32 = _t2;
        /* TODO: fcmpl  */
        /* TODO: i2s  */
        return Ok((sign_bit|31744i32));
        /* TODO: fcmpg  */
        return Ok(sign_bit);
        let _t3: i32 = (f).abs();
        let mut exp: i32 = _t3;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut expdelta: i32 = 0i32;
        let mut msb: i32 = 0i32;
        expdelta = (-14i32).wrapping_sub(exp);
        exp = -15i32;
        msb = 8388608i32;
        let mut f_signif_bits: i32 = ((doppel&8388607i32)|msb);
        /* TODO: i2s  */
        let mut signif_bits: i32 = (f_signif_bits>>(((13i32).wrapping_add(expdelta)&0x1f)));
        let mut lsb: i32 = (f_signif_bits&(1i32<<((13i32).wrapping_add(expdelta)&0x1f)));
        let mut round: i32 = (f_signif_bits&(1i32<<((12i32).wrapping_add(expdelta)&0x1f)));
        let mut sticky: i32 = (f_signif_bits&((1i32<<((12i32).wrapping_add(expdelta)&0x1f))).wrapping_sub(1i32));
        /* TODO: i2s  */
        signif_bits = (signif_bits).wrapping_add(1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: i2s  */
        Ok((sign_bit|(((exp).wrapping_add(15i32)<<(10i32&0x1f))).wrapping_add(signif_bits)))
    }

    // java: compareTo(Ljava/lang/Float;)I
    pub fn compareTo(&self, anotherFloat: Object) -> Result<i32> {
        let this = self;
        let _t0: i32 = Float::compare(this.value.get(), anotherFloat.value.get())?;
        Ok(_t0)
    }

    // java: compare(FF)I
    pub fn compare(f1: f32, f2: f32) -> Result<i32> {
        /* TODO: fcmpg  */
        return Ok(-1i32);
        /* TODO: fcmpl  */
        return Ok(1i32);
        let _t0: i32 = Float::floatToIntBits(f1)?;
        let mut thisBits: i32 = _t0;
        let _t1: i32 = Float::floatToIntBits(f2)?;
        let mut anotherBits: i32 = _t1;
        Ok(thisBits >= anotherBits)
    }

    // java: sum(FF)F
    pub fn sum(a: f32, b: f32) -> Result<f32> {
        Ok((a+b))
    }

    // java: max(FF)F
    pub fn max(a: f32, b: f32) -> Result<f32> {
        let _t0: f32 = (a).max(b);
        Ok(_t0)
    }

    // java: min(FF)F
    pub fn min(a: f32, b: f32) -> Result<f32> {
        let _t0: f32 = (a).min(b);
        Ok(_t0)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Optional::of(this)?;
        Ok(_t0)
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Object> {
        let this = self;
        Ok(this)
    }
}
