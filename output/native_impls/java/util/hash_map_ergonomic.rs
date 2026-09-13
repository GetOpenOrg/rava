// Ergonomic API for HashMap<K, V> (T39)
// Requires K/V: Into<Object> + From<Object>

impl<K: Clone + Into<Object> + From<Object> + 'static,
     V: Clone + Into<Object> + From<Object> + 'static> HashMap<K, V>
{
    /// Java: map.put(key, value)  — no .into() needed
    pub fn put_kv(&self, key: K, value: V) -> Result<Object> {
        self.put(key.into(), value.into())
    }

    /// Java: V v = map.get(key)  — returns typed V, no .downcast() needed
    pub fn get_value(&self, key: K) -> Result<V> {
        Ok(V::from(self.get(key.into())?))
    }

    /// Java: map.containsKey(key)  — typed key lookup
    pub fn contains_key_e(&self, key: K) -> Result<bool> {
        self.containsKey(key.into())
    }
}
