// Ergonomic API for HashMap<K, V> (T39)
// @jvm_class: java/util/HashMap
// @jvm_rename: put -> put_obj, get -> get_obj

impl<K: Clone + Default + Into<Object> + From<Object> + 'static,
     V: Clone + Default + Into<Object> + From<Object> + 'static> HashMap<K, V>
{
    /// Java: new HashMap<K,V>() — 类型安全构造器
    pub fn new_typed() -> Result<HashMap<K, V>> {
        let base = HashMap::<Object, Object>::new()?;
        Ok(HashMap {
            _super: base._super,
            table: base.table,
            entrySet: base.entrySet,
            size: base.size,
            modCount: base.modCount,
            threshold: base.threshold,
            loadFactor: base.loadFactor,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Java: map.put(key, value)  — 无需 .into()
    pub fn put(&self, key: K, value: V) -> Result<Object> {
        self.put_obj(key.into(), value.into())
    }

    /// Java: V v = map.get(key)  — 返回类型安全的 V，无需 downcast
    pub fn get(&self, key: K) -> Result<V> {
        Ok(V::from(self.get_obj(key.into())?))
    }

    /// Java: map.containsKey(key)  — 类型安全 key 查找（snake_case）
    pub fn contains_key(&self, key: K) -> Result<bool> {
        self.containsKey(key.into())
    }
}
