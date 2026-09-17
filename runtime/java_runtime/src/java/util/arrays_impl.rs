use crate::prelude::*;
use super::arrays::Arrays;

impl Arrays {
    // Arrays.copyOf(Object[], int) 翻译结果依赖 getClass() 在数组类型上的调用，
    // 但 Rc<RefCell<Vec<Object>>> 无 getClass()，生成代码会产生 Object::default() 占位
    // 导致 downcast::<Class>() 崩溃。此处直接分配新 Vec 并按 min(orig_len, newLength) 复制。
    #[jvm_native]
    pub fn copyOf_arr_obj_i(original: Rc<RefCell<Vec<Object>>>, newLength: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
        let orig_len = original.borrow().len();
        let copy_len = orig_len.min(newLength as usize);
        let mut copy: Vec<Object> = vec![Object::default(); newLength as usize];
        for i in 0..copy_len {
            copy[i] = original.borrow()[i].clone();
        }
        Ok(Rc::new(RefCell::new(copy)))
    }
}
