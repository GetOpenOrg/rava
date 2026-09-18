use crate::prelude::*;
use super::arrays::Arrays;

impl Arrays {
    #[jvm_native]
    pub fn copyOf_arr_obj_i(original: JArray<Object>, newLength: i32) -> Result<JArray<Object>> {
        let orig_len = original.len();
        let copy_len = orig_len.min(newLength);
        let result = JArray::<Object>::new(newLength);
        for i in 0..copy_len {
            result.set(i, original.get(i));
        }
        Ok(result)
    }
}
