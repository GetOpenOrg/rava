use crate::prelude::*;
use super::string::String;
use super::string_builder::StringBuilder;

impl String {
    #[jvm_native]
    pub fn new_sb(builder: StringBuilder) -> Result<Self> {
        let coder = builder._super.coder.get();
        let count = builder._super.count.get() as usize;
        let raw = builder._super.value.get();
        let raw_bytes = raw.borrow();
        let byte_len = if coder == 0 { count } else { count * 2 };
        let trimmed: Vec<i8> = raw_bytes[..byte_len.min(raw_bytes.len())].to_vec();
        let mut this = Self::default();
        this.value.set(Rc::new(RefCell::new(trimmed)));
        this.coder.set(coder);
        Ok(this)
    }

    #[jvm_ext]
    pub fn from_owned(s: std::string::String) -> Self {
        let bytes: Vec<i8> = s.into_bytes().into_iter().map(|b| b as i8).collect();
        let mut inst = String::default();
        inst.value.set(Rc::new(RefCell::new(bytes)));
        inst.coder.set(0i8);
        inst
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rc = self.value.get();
        let bytes = rc.borrow();
        if self.coder.get() == 0 {
            let s: std::string::String = bytes.iter().map(|&b| b as u8 as char).collect();
            write!(f, "{}", s)
        } else {
            let u16s: Vec<u16> = bytes.chunks(2)
                .map(|c| u16::from_le_bytes([c[0] as u8, *c.get(1).unwrap_or(&0) as u8]))
                .collect();
            write!(f, "{}", std::string::String::from_utf16_lossy(&u16s))
        }
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self { String::from_owned(s.to_owned()) }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self { String::from_owned(s) }
}
