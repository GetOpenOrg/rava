use crate::prelude::*;
use super::arrays::Arrays;

impl Arrays {
    #[jvm_native]
    pub fn copyOf_arr_obj_i(original: JArray<Object>, newLength: i32) -> Result<JArray<Object>> {
        let orig_len = original.len()?;
        // 负 newLength 抛 NegativeArraySizeException（Java 语义，经 try_new）
        let result = JArray::<Object>::try_new(newLength)?;
        let copy_len = orig_len.min(newLength);
        for i in 0..copy_len {
            result.set(i, original.get(i)?)?;
        }
        Ok(result)
    }
}
