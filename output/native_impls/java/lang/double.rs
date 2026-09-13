/// java/lang/Double.doubleToRawLongBits:(D)J
pub fn doubleToRawLongBits(value: f64) -> Result<i64> {
    Ok(value.to_bits() as i64)
}

/// java/lang/Double.longBitsToDouble:(J)D
pub fn longBitsToDouble(bits: i64) -> Result<f64> {
    Ok(f64::from_bits(bits as u64))
}
