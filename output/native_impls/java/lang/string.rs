use java_runtime::prelude::*;
use super::*;

fn _to_bytes(s: &std::string::String) -> Vec<i8> {
    s.as_bytes().iter().map(|&b| b as i8).collect()
}

fn _from_bytes(bytes: &[i8]) -> std::string::String {
    let ubytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
    std::string::String::from_utf8_lossy(&ubytes).into_owned()
}

impl super::String {
    /// JDK 9+ 紧凑字符串模式：始终为 true（所有 String 均为单字节 Latin1 或 UTF16 字节数组）
    pub fn COMPACT_STRINGS() -> bool { true }

    pub fn from_owned(s: std::string::String) -> super::String {
        let result = super::String::default();
        result.value.set(Rc::new(RefCell::new(_to_bytes(&s))));
        result
    }

    pub fn append(&mut self, s: &super::String) -> Result<()> {
        let self_rc = self.value.get();
        let s_rc = s.value.get();
        let mut self_str = _from_bytes(&self_rc.borrow());
        let s_str = _from_bytes(&s_rc.borrow());
        self_str.push_str(&s_str);
        self.value.set(Rc::new(RefCell::new(_to_bytes(&self_str))));
        Ok(())
    }
}

impl std::fmt::Display for super::String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rc = self.value.get();
        let s = _from_bytes(&rc.borrow());
        write!(f, "{}", s)
    }
}

impl From<&str> for super::String {
    fn from(s: &str) -> Self {
        super::String::from_owned(s.to_owned())
    }
}
