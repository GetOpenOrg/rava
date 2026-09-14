// Ergonomic API for HashSet<E> (T39)
// @jvm_class: java/util/HashSet
// @jvm_rename: add -> add_obj, contains -> contains_obj

impl<E: Clone + Default + Into<Object> + From<Object> + 'static> HashSet<E> {
    /// Java: new HashSet<E>() — 类型安全构造器
    pub fn new_typed() -> Result<HashSet<E>> {
        let base = HashSet::<Object>::new()?;
        Ok(HashSet {
            map: base.map,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Java: set.add(e)  — 无需 .into()
    pub fn add(&self, e: E) -> Result<bool> {
        self.add_obj(e.into())
    }

    /// Java: set.contains(e)  — 类型安全查找，无需 .into()
    pub fn contains(&self, e: E) -> Result<bool> {
        self.contains_obj(e.into())
    }
}
