use crate::prelude::*;
use super::preconditions::Preconditions;

impl Preconditions {
    #[jvm_native]
    pub fn checkIndex_i_i_bifunc(index: i32, length: i32, _oobef: Object) -> Result<i32> {
        if index < 0 || index >= length {
            return Err(JvmError::Custom(format!(
                "Index {} out of bounds for length {}", index, length
            )));
        }
        Ok(index)
    }

    #[jvm_native]
    pub fn checkFromToIndex_i_i_i_bifunc(fromIndex: i32, toIndex: i32, length: i32, _oobef: Object) -> Result<i32> {
        if fromIndex < 0 || fromIndex > toIndex || toIndex > length {
            return Err(JvmError::Custom(format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, toIndex, length
            )));
        }
        Ok(fromIndex)
    }

    #[jvm_native]
    pub fn checkFromIndexSize_i_i_i_bifunc(fromIndex: i32, size: i32, length: i32, _oobef: Object) -> Result<i32> {
        if (length | fromIndex | size) < 0 || size > length - fromIndex {
            return Err(JvmError::Custom(format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, fromIndex + size, length
            )));
        }
        Ok(fromIndex)
    }

    #[jvm_native]
    pub fn checkIndex_l_l_bifunc(index: i64, length: i64, _oobef: Object) -> Result<i64> {
        if index < 0 || index >= length {
            return Err(JvmError::Custom(format!(
                "Index {} out of bounds for length {}", index, length
            )));
        }
        Ok(index)
    }

    #[jvm_native]
    pub fn checkFromToIndex_l_l_l_bifunc(fromIndex: i64, toIndex: i64, length: i64, _oobef: Object) -> Result<i64> {
        if fromIndex < 0 || fromIndex > toIndex || toIndex > length {
            return Err(JvmError::Custom(format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, toIndex, length
            )));
        }
        Ok(fromIndex)
    }

    #[jvm_native]
    pub fn checkFromIndexSize_l_l_l_bifunc(fromIndex: i64, size: i64, length: i64, _oobef: Object) -> Result<i64> {
        if fromIndex < 0 || size < 0 || size > length - fromIndex {
            return Err(JvmError::Custom(format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, fromIndex + size, length
            )));
        }
        Ok(fromIndex)
    }
}
