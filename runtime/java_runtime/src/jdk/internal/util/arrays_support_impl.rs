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

    /// @IntrinsicCandidate vectorizedHashCode：按 basicType（JVM T_* 常量）解释数组元素，
    /// 计算 31 进制多项式哈希（与 JDK 标量回退路径逐位一致，i32 回绕）。
    #[jvm_native]
    pub fn vectorizedHashCode(array: Object, from_index: i32, length: i32,
                              initial_value: i32, basic_type: i32) -> Result<i32> {
        const T_BOOLEAN: i32 = 4;
        const T_CHAR: i32 = 5;
        const T_BYTE: i32 = 8;
        const T_SHORT: i32 = 9;
        const T_INT: i32 = 10;
        let end = from_index + length;
        let mut result = initial_value;
        let mut step = |v: i32| result = result.wrapping_mul(31).wrapping_add(v);
        match basic_type {
            T_BOOLEAN => {
                // 无符号字节（StringLatin1 以 T_BOOLEAN 表示 value[i] & 0xff）
                let a = array.downcast::<JArray<i8>>();
                for i in from_index..end { step((a.get(i) as u8) as i32); }
            }
            T_BYTE => {
                let a = array.downcast::<JArray<i8>>();
                for i in from_index..end { step(a.get(i) as i32); }
            }
            T_CHAR => {
                // StringUTF16 传入的是 byte[]，每个 char 占 2 字节（本机小端）
                if let Some(a) = array.0.as_any().downcast_ref::<JArray<u16>>() {
                    for i in from_index..end { step(a.get(i) as i32); }
                } else {
                    let a = array.downcast::<JArray<i8>>();
                    for i in from_index..end {
                        let lo = (a.get(i * 2) as u8) as i32;
                        let hi = (a.get(i * 2 + 1) as u8) as i32;
                        step(lo | (hi << 8));
                    }
                }
            }
            T_SHORT => {
                let a = array.downcast::<JArray<i16>>();
                for i in from_index..end { step(a.get(i) as i32); }
            }
            T_INT => {
                let a = array.downcast::<JArray<i32>>();
                for i in from_index..end { step(a.get(i)); }
            }
            _ => return Err(JvmError::Custom(format!(
                "IllegalArgumentException: unrecognized basic type: {}", basic_type))),
        }
        Ok(result)
    }
}
