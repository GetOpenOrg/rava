/// java/lang/Float.floatToRawIntBits:(F)I
pub fn floatToRawIntBits(value: f32) -> i32 {
    value.to_bits() as i32
}

/// java/lang/Float.intBitsToFloat:(I)F
pub fn intBitsToFloat(bits: i32) -> f32 {
    f32::from_bits(bits as u32)
}
