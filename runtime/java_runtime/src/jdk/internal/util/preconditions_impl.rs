use crate::prelude::*;
use super::preconditions::Preconditions;

// 内部边界类：越界格式化器（BiFunction）在手写层退化为「异常种类」标记对象，
// check* 方法按标记构造与 JDK 一致的异常类与消息。
const FORMATTER_SIOOBE: i32 = 1;
const FORMATTER_AIOOBE: i32 = 2;
const FORMATTER_IOOBE: i32 = 3;

fn out_of_bounds(oobef: &Object, message: std::string::String) -> JvmError {
    match oobef.0.as_any().downcast_ref::<i32>() {
        Some(&FORMATTER_SIOOBE) => JvmError::string_index_out_of_bounds(message),
        Some(&FORMATTER_AIOOBE) => JvmError::array_index_out_of_bounds_message(message),
        _ => JvmError::index_out_of_bounds(message),
    }
}

impl Preconditions {
    #[jvm_boundary]
    pub fn SIOOBE_FORMATTER() -> Result<Object> {
        Ok(Object::from(FORMATTER_SIOOBE))
    }

    #[jvm_boundary]
    pub fn AIOOBE_FORMATTER() -> Result<Object> {
        Ok(Object::from(FORMATTER_AIOOBE))
    }

    #[jvm_boundary]
    pub fn IOOBE_FORMATTER() -> Result<Object> {
        Ok(Object::from(FORMATTER_IOOBE))
    }

    /// `outOfBoundsExceptionFormatter(Function<String, X>)`：由异常工厂派生的格式化器。
    /// 手写层以异常种类标记承载（check* 按标记构造异常）；消费方 Buffer.<clinit> 的
    /// IOOBE_FORMATTER（工厂 IndexOutOfBoundsException::new）→ IOOBE 标记。
    #[jvm_boundary]
    pub fn outOfBoundsExceptionFormatter(_f: crate::java::util::function::Function<Object, Object>)
        -> Result<crate::java::util::function::BiFunction<Object, Object, Object>> {
        Ok(From::from(Object::from(FORMATTER_IOOBE)))
    }

    #[jvm_boundary]
    pub fn checkIndex_i_i_bifunction(index: i32, length: i32, oobef: Object) -> Result<i32> {
        if index < 0 || index >= length {
            return Err(out_of_bounds(&oobef, format!(
                "Index {} out of bounds for length {}", index, length)));
        }
        Ok(index)
    }

    #[jvm_boundary]
    pub fn checkFromToIndex_i_i_i_bifunction(fromIndex: i32, toIndex: i32, length: i32, oobef: Object) -> Result<i32> {
        if fromIndex < 0 || fromIndex > toIndex || toIndex > length {
            return Err(out_of_bounds(&oobef, format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, toIndex, length)));
        }
        Ok(fromIndex)
    }

    #[jvm_boundary]
    pub fn checkFromIndexSize_i_i_i_bifunction(fromIndex: i32, size: i32, length: i32, oobef: Object) -> Result<i32> {
        if (length | fromIndex | size) < 0 || size > length - fromIndex {
            return Err(out_of_bounds(&oobef, format!(
                "Range [{}, {} + {}) out of bounds for length {}", fromIndex, fromIndex, size, length)));
        }
        Ok(fromIndex)
    }

    #[jvm_boundary]
    pub fn checkIndex_l_l_bifunction(index: i64, length: i64, oobef: Object) -> Result<i64> {
        if index < 0 || index >= length {
            return Err(out_of_bounds(&oobef, format!(
                "Index {} out of bounds for length {}", index, length)));
        }
        Ok(index)
    }

    #[jvm_boundary]
    pub fn checkFromToIndex_l_l_l_bifunction(fromIndex: i64, toIndex: i64, length: i64, oobef: Object) -> Result<i64> {
        if fromIndex < 0 || fromIndex > toIndex || toIndex > length {
            return Err(out_of_bounds(&oobef, format!(
                "Range [{}, {}) out of bounds for length {}", fromIndex, toIndex, length)));
        }
        Ok(fromIndex)
    }

    #[jvm_boundary]
    pub fn checkFromIndexSize_l_l_l_bifunction(fromIndex: i64, size: i64, length: i64, oobef: Object) -> Result<i64> {
        if fromIndex < 0 || size < 0 || size > length - fromIndex {
            return Err(out_of_bounds(&oobef, format!(
                "Range [{}, {} + {}) out of bounds for length {}", fromIndex, fromIndex, size, length)));
        }
        Ok(fromIndex)
    }
}
