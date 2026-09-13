// Ergonomic API for ArrayList<E> (T39)
// Requires E: Into<Object> + From<Object> so callers don't need .into() / .downcast()
// add(E) / get_item(i32)->E / contains_e(E) are the clean Java-like names
// (get_item instead of get because the JVM-exact get(i32)->Object is already present)

impl<E: Clone + Into<Object> + From<Object> + 'static> ArrayList<E> {
    /// Java: list.add(e)  — no .into() needed
    pub fn add(&self, e: E) -> Result<bool> {
        self.add__obj(e.into())
    }

    /// Java: E e = list.get(i)  — returns typed E, no .downcast() needed
    pub fn get_item(&self, index: i32) -> Result<E> {
        Ok(E::from(self.get(index)?))
    }

    /// Java: list.contains(e)  — typed contains
    pub fn contains_e(&self, e: E) -> Result<bool> {
        self.contains(e.into())
    }

    /// Java: for (E e : list)  — collect to Vec<E> for iteration
    pub fn iter_typed(&self) -> std::vec::Vec<E> {
        let rc = self.elementData.get();
        let cloned: std::vec::Vec<Object> = rc.borrow().clone();
        cloned.into_iter().map(|o| E::from(o)).collect()
    }
}
