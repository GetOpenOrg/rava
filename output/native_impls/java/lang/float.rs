use java_runtime::prelude::*;
use super::*;

impl super::Float {
    pub fn floatToRawIntBits(value: f32) -> Result<i32> {
        Ok(value.to_bits() as i32)
    }

    pub fn intBitsToFloat(bits: i32) -> Result<f32> {
        Ok(f32::from_bits(bits as u32))
    }
}
