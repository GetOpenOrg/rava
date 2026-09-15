use crate::prelude::*;
use super::arrays::Arrays;

impl Arrays {
    /// copyOf(Object[], int) — 跳过 Java 侧的 original.getClass() 反射调用，
    /// 直接创建新 Vec 并复制元素（类型始终是 Object[]）。
    #[jvm_native]
    pub fn copyOf_arr_obj_i(original: Rc<RefCell<Vec<Object>>>, new_length: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
        let new_len = new_length.max(0) as usize;
        let old = original.borrow();
        let copy_len = old.len().min(new_len);
        let mut new_vec: Vec<Object> = vec![Default::default(); new_len];
        new_vec[..copy_len].clone_from_slice(&old[..copy_len]);
        Ok(Rc::new(RefCell::new(new_vec)))
    }
}
