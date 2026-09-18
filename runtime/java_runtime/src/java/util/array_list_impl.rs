use crate::prelude::*;
use super::array_list::ArrayList;

impl<E: Clone + Default + 'static + From<Object> + Into<Object>> ArrayList<E> {
    #[jvm_native]
    pub fn add_obj_arr_obj_i(&self, e: E, elementData_arg: JArray<Object>, s: i32) -> Result<()> {
        let this = self;
        let mut elementData = elementData_arg;
        if s == elementData.len() {
            elementData = this.grow()?;
        }
        let needed = s + 1;
        if elementData.len() < needed {
            let new_data = JArray::<Object>::new(needed);
            for i in 0..elementData.len() {
                new_data.set(i, elementData.get(i));
            }
            this.__set_elementData(Clone::clone(&new_data));
            elementData = new_data;
        }
        elementData.set(s, e.into());
        this.__set_size(s.wrapping_add(1i32));
        Ok(())
    }

    #[jvm_native]
    pub fn elementData(&self, index: i32) -> Result<Object> {
        Ok(self.__get_elementData().get(index))
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
