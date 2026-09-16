use crate::prelude::*;
use super::arrays_support::ArraysSupport;

impl ArraysSupport {
    #[jvm_native]
    pub fn newLength(old_length: i32, min_growth: i32, pref_growth: i32) -> Result<i32> {
        let pref_length = old_length.wrapping_add(min_growth.max(pref_growth));
        if pref_length > 0 && pref_length <= (i32::MAX - 8) {
            Ok(pref_length)
        } else {
            let min_length = old_length.wrapping_add(min_growth);
            if min_length < 0 {
                Err(JvmError::Custom("OutOfMemoryError: array size".to_owned()))
            } else if min_length <= (i32::MAX - 8) {
                Ok(i32::MAX - 8)
            } else {
                Ok(min_length)
            }
        }
    }
}
