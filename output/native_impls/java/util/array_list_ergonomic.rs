// Ergonomic API for ArrayList<E> (T39)
// @jvm_class: java/util/ArrayList
// @jvm_rename: get -> get_obj, contains -> contains_obj
//
// JVM 原始方法重命名后，ergonomic 层使用 Java 原始方法名：
//   add(E)        → 内部调用 add_obj(e.into())
//   get(i32)->E   → 内部调用 get_obj(i)，再 E::from(obj)
//   contains(E)   → 内部调用 contains_obj(e.into())
//   iter_typed()  → 返回 Vec<E> 供 for 循环
//
// new_typed() 是 new() 的类型安全版本（用于 ergonomic 场景）

impl<E: Clone + Default + Into<Object> + From<Object> + 'static> ArrayList<E> {
    /// Java: new ArrayList<E>() — 类型安全构造器
    pub fn new_typed() -> Result<ArrayList<E>> {
        let base = ArrayList::<Object>::new()?;
        Ok(ArrayList {
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
