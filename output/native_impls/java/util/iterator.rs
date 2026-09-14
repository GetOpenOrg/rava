use java_runtime::prelude::*;
use super::*;

// Iterator 注入字段：迭代状态（Vec 共享引用 + 当前位置）
/// @field _data: JField<Rc<RefCell<Vec<Object>>>>
/// @field _pos:  JField<i32>

impl<E: Clone + 'static> super::Iterator<E> {
    pub fn hasNext(&self) -> Result<bool> {
        Ok(self._pos.get() < self._data.get().borrow().len() as i32)
    }

    pub fn next(&self) -> Result<Object> {
        let pos = self._pos.get() as usize;
        let data = self._data.get();
        let v = data.borrow().get(pos).cloned()
            .ok_or(JvmError::Custom("NoSuchElementException".into()))?;
        self._pos.set(self._pos.get() + 1);
        Ok(v)
    }
}
