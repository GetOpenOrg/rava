use crate::prelude::*;
use super::*;
use super::__DoubleHw;

impl __DoubleHw for Double {
    #[jvm_native]
    fn doubleToRawLongBits(value: f64) -> Result<i64> {
        Ok(value.to_bits() as i64)
    }

    #[jvm_native]
    fn longBitsToDouble(bits: i64) -> Result<f64> {
        Ok(f64::from_bits(bits as u64))
    }
}
