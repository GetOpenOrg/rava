use java_runtime::prelude::*;
use super::*;

impl Double {
    #[jvm_native]
    pub fn doubleToRawLongBits(value: f64) -> Result<i64> {
        Ok(value.to_bits() as i64)
    }

    #[jvm_native]
    pub fn longBitsToDouble(bits: i64) -> Result<f64> {
        Ok(f64::from_bits(bits as u64))
    }
}
