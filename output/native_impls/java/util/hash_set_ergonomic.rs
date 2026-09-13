// Ergonomic API for HashSet<E> (T39)
// Requires E: Into<Object> + From<Object>

impl<E: Clone + Into<Object> + From<Object> + 'static> HashSet<E> {
    /// Java: set.add(e)  — no .into() needed
    pub fn add_e(&self, e: E) -> Result<bool> {
        self.add(e.into())
    }

    /// Java: set.contains(e)  — typed lookup, no .into() needed
    pub fn contains_e(&self, e: E) -> Result<bool> {
        self.contains(e.into())
    }
}
