fn _to_bytes(s: &std::string::String) -> Vec<i8> {
    s.as_bytes().iter().map(|&b| b as i8).collect()
}

fn _from_bytes(bytes: &[i8]) -> std::string::String {
    let ubytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
    std::string::String::from_utf8_lossy(&ubytes).into_owned()
}

/// @synthetic
pub fn from_owned(s: std::string::String) -> String {
    let result = String::default();
    result.value.set(_to_bytes(&s));
    result
}

/// @synthetic
pub fn append(_this: &mut String, s: &String) -> Result<()> {
    let self_bytes = _this.value.get();
    let s_bytes = s.value.get();
    let mut self_str = _from_bytes(&self_bytes);
    let s_str = _from_bytes(&s_bytes);
    self_str.push_str(&s_str);
    _this.value.set(_to_bytes(&self_str));
    Ok(())
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.value.get();
        write!(f, "{}", _from_bytes(&bytes))
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        from_owned(s.to_owned())
    }
}
