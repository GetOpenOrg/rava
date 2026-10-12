use crate::prelude::*;
use super::*;
use super::__FloatHw;

impl __FloatHw for Float {
    #[jvm_native]
    fn floatToRawIntBits(value: f32) -> Result<i32> {
        Ok(value.to_bits() as i32)
    }

    #[jvm_native]
    fn intBitsToFloat(bits: i32) -> Result<f32> {
        Ok(f32::from_bits(bits as u32))
    }
}
