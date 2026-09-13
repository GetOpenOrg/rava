/// @synthetic
pub fn new_default() -> Result<HashMap> {
    Ok(HashMap::default())
}

/// java/util/HashMap.size:()I
pub fn size(_this: &HashMap) -> Result<i32> {
    Ok(0)
}

/// java/util/HashMap.isEmpty:()Z
pub fn isEmpty(_this: &HashMap) -> Result<bool> {
    Ok(true)
}
