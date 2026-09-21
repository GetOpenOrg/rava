use crate::prelude::*;
use super::arrays_support::ArraysSupport;

impl ArraysSupport {
    /// mismatch([B I [B I I)：两段字节区间首个不等元素的相对下标，全等返回 -1。
    /// JDK 以 Unsafe 向量化比较实现，可观察结果即逐元素比较。
    #[jvm_boundary]
    pub fn mismatch_arr_b_i_arr_b_i_i(a: JArray<i8>, a_from_index: i32, b: JArray<i8>,
                                      b_from_index: i32, length: i32) -> Result<i32> {
        for i in 0..length {
            if a.get(a_from_index + i)? != b.get(b_from_index + i)? {
                return Ok(i);
            }
        }
        Ok(-1)
    }

    /// mismatch([B [B I)：同上，两数组均自下标 0 起。
    #[jvm_boundary]
    pub fn mismatch_arr_b_arr_b_i(a: JArray<i8>, b: JArray<i8>, length: i32) -> Result<i32> {
        Self::mismatch_arr_b_i_arr_b_i_i(a, 0, b, 0, length)
    }

    /// mismatch([I [I I)：两段 int 区间首个不等元素的相对下标，全等返回 -1。
    /// JDK 21 方法体（javap 核对）：首元素快筛 → vectorizedMismatch 向量化前缀
    /// （命返回 i≥0，否则 ~i=已证相等元素数，换算 `length - (~i)` 续扫）→
    /// 标量尾扫 → `iconst_m1`。向量 intrinsic 的可观察结果即逐元素比较，
    /// 与 byte 版同族，直接逐元素实现。
    #[jvm_boundary]
    pub fn mismatch_arr_i_arr_i_i(a: JArray<i32>, b: JArray<i32>, length: i32) -> Result<i32> {
        for i in 0..length {
            if a.get(i)? != b.get(i)? {
                return Ok(i);
            }
        }
        Ok(-1)
    }

    #[jvm_native]
    pub fn newLength(old_length: i32, min_growth: i32, pref_growth: i32) -> Result<i32> {
        let pref_length = old_length.wrapping_add(min_growth.max(pref_growth));
        if pref_length > 0 && pref_length <= (i32::MAX - 8) {
            Ok(pref_length)
        } else {
            let min_length = old_length.wrapping_add(min_growth);
            if min_length < 0 {
                Err(JvmError::out_of_memory(&format!(
                    "Required length exceeds implementation limit: {} + {}", old_length, min_growth)))
            } else if min_length <= (i32::MAX - 8) {
                Ok(i32::MAX - 8)
            } else {
                Ok(min_length)
            }
        }
    }

    /// @IntrinsicCandidate vectorizedHashCode：按 basicType（JVM T_* 常量）解释数组元素，
    /// 计算 31 进制多项式哈希（与 JDK 标量回退路径逐位一致，i32 回绕）。
    #[jvm_native(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn vectorizedHashCode(array: Object, from_index: i32, length: i32,
                              initial_value: i32, basic_type: i32) -> Result<i32> {
        const T_BOOLEAN: i32 = 4;
        const T_CHAR: i32 = 5;
        const T_BYTE: i32 = 8;
        const T_SHORT: i32 = 9;
        const T_INT: i32 = 10;
        let end = from_index + length;
        let mut result = initial_value;
        let step = |acc: i32, v: i32| acc.wrapping_mul(31).wrapping_add(v);
        match basic_type {
            T_BOOLEAN => {
                // 无符号字节（StringLatin1 以 T_BOOLEAN 表示 value[i] & 0xff）
                let a = array.downcast::<JArray<i8>>();
                for i in from_index..end { result = step(result, (a.get(i)? as u8) as i32); }
            }
            T_BYTE => {
                let a = array.downcast::<JArray<i8>>();
                for i in from_index..end { result = step(result, a.get(i)? as i32); }
            }
            T_CHAR => {
                // StringUTF16 传入的是 byte[]，每个 char 占 2 字节（本机小端）
                if let Some(a) = array.0.as_any().downcast_ref::<JArray<u16>>() {
                    for i in from_index..end { result = step(result, a.get(i)? as i32); }
                } else {
                    let a = array.downcast::<JArray<i8>>();
                    for i in from_index..end {
                        let lo = (a.get(i * 2)? as u8) as i32;
                        let hi = (a.get(i * 2 + 1)? as u8) as i32;
                        result = step(result, lo | (hi << 8));
                    }
                }
            }
            T_SHORT => {
                let a = array.downcast::<JArray<i16>>();
                for i in from_index..end { result = step(result, a.get(i)? as i32); }
            }
            T_INT => {
                let a = array.downcast::<JArray<i32>>();
                for i in from_index..end { result = step(result, a.get(i)?); }
            }
            _ => return Err(crate::java::lang::IllegalArgumentException::new_str(String::from(
                format!("unrecognized basic type: {}", basic_type)))?.into()),
        }
        Ok(result)
    }
}
