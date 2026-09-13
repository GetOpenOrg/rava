/// @synthetic
pub fn new_default() -> Result<HashSet<Object>> {
    Ok(HashSet::default())
}

/// java/util/HashSet.size:()I
pub fn size<E: Clone + 'static>(_this: &HashSet<E>) -> Result<i32> {
    Ok(0)
}

/// java/util/HashSet.isEmpty:()Z
pub fn isEmpty<E: Clone + 'static>(_this: &HashSet<E>) -> Result<bool> {
    Ok(true)
}
