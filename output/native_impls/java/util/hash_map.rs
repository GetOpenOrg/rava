/// @synthetic
pub fn new_default() -> Result<HashMap<Object, Object>> {
    Ok(HashMap::default())
}

/// java/util/HashMap.size:()I
pub fn size<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>) -> Result<i32> {
    Ok(0)
}

/// java/util/HashMap.isEmpty:()Z
pub fn isEmpty<K: Clone + 'static, V: Clone + 'static>(_this: &HashMap<K, V>) -> Result<bool> {
    Ok(true)
}
