use crate::prelude::*;
use super::array_list::ArrayList;

/// 将 E 存入 Vec<Object>，避免 E=Object 时的二次包装。
/// Java 泛型类型擦除后，E 在运行时 = Object，直接存引用即可。
fn e_to_object<E: Clone + 'static>(e: E) -> Object {
    let any_val: &dyn std::any::Any = &e;
    if let Some(obj) = any_val.downcast_ref::<Object>() {
        obj.clone()
    } else {
        Object::from_any(e)
    }
}

impl<E: Clone + Default + 'static> ArrayList<E> {
    #[jvm_native]
    pub fn add_obj_arr_obj_i(&self, e: E, elementData_arg: Rc<RefCell<Vec<Object>>>, s: i32) -> Result<()> {
        let this = self;
        let mut elementData = elementData_arg;
        if s == elementData.borrow().len() as i32 {
            elementData = this.grow()?;
        }
        let needed = (s + 1) as usize;
        if elementData.borrow().len() < needed {
            elementData.borrow_mut().resize(needed, Object::default());
        }
        elementData.borrow_mut()[s as usize] = e_to_object(e);
        this.__set_size(s.wrapping_add(1i32));
        Ok(())
    }

    #[jvm_native]
    pub fn elementData(&self, index: i32) -> Result<Object> {
        let data = self.__get_elementData();
        let data = data.borrow();
        Ok(data.get(index as usize).cloned().unwrap_or_default())
    }

    #[jvm_native]
    pub fn get(&self, index: i32) -> Result<Object> {
        let size = self.__get_size();
        if index < 0 || index >= size {
            return Err(JvmError::Custom(format!(
                "Index: {}, Size: {}", index, size
            )));
        }
        self.elementData(index)
    }
}
