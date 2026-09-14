// @jvm_class: java/util/ArrayList
// @jvm_rename: get -> get_obj, contains -> contains_obj
use java_runtime::prelude::*;
use super::*;

impl<E: Clone + Default + 'static> super::ArrayList<E> {
    pub fn new() -> Result<ArrayList<Object>> {
        Ok(ArrayList::default())
    }

    pub fn add_obj(&self, e: Object) -> Result<bool> {
        let rc = self.elementData.get();
        rc.borrow_mut().push(e);
        self.size.set(self.size.get() + 1);
        Ok(true)
    }

    pub fn size(&self) -> Result<i32> {
        Ok(self.size.get())
    }

    pub fn get_obj(&self, index: i32) -> Result<Object> {
        let rc = self.elementData.get();
        let vec = rc.borrow();
        vec.get(index as usize).cloned()
            .ok_or(JvmError::ArrayIndexOutOfBoundsException(index))
    }

    pub fn isEmpty(&self) -> Result<bool> {
        Ok(self.size.get() == 0)
    }

    pub fn clear(&self) -> Result<()> {
        let rc = self.elementData.get();
        rc.borrow_mut().clear();
        self.size.set(0);
        Ok(())
    }

    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList.iterator:()Ljava/util/Iterator;")
    }

    pub fn contains_obj(&self, o: Object) -> Result<bool> {
        let data = self.elementData.get();
        let vec = data.borrow();
        let found = vec.iter().any(|x| std::rc::Rc::ptr_eq(&x.0, &o.0));
        Ok(found)
    }
}

impl<E: Clone + Default + Into<Object> + From<Object> + 'static> super::ArrayList<E> {
    /// Java: new ArrayList<E>() — 类型安全构造器
    pub fn new_typed() -> Result<ArrayList<E>> {
        let base = ArrayList::<Object>::new()?;
        Ok(ArrayList {
            _super: Default::default(),
            elementData: base.elementData,
            size: base.size,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Java: list.add(e)  — 无需 .into()
    pub fn add(&self, e: E) -> Result<bool> {
        self.add_obj(e.into())
    }

    /// Java: E e = list.get(i)  — 返回 E，无需 downcast
    pub fn get(&self, index: i32) -> Result<E> {
        Ok(E::from(self.get_obj(index)?))
    }

    /// Java: list.contains(e)  — 类型安全 contains
    pub fn contains(&self, e: E) -> Result<bool> {
        self.contains_obj(e.into())
    }

    /// Java: for (E e : list)  — 收集为 Vec<E> 供 Rust for 循环
    pub fn iter_typed(&self) -> std::vec::Vec<E> {
        let rc = self.elementData.get();
        let cloned: std::vec::Vec<Object> = rc.borrow().clone();
        cloned.into_iter().map(|o| E::from(o)).collect()
    }
}
