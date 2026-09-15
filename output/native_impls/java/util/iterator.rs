use java_runtime::prelude::*;
use super::*;

// TODO(K-1/K-4): Iterator 是 Java 接口，_data/_pos 是当前实现的临时状态。
// 正确架构：状态应在具体实现类（如 ArrayList$Itr），不在接口上。
// 当前保留 impl 方法，等待具体迭代器类实现后重构。

impl<E: Clone + Default + 'static> super::Iterator<E> {
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
