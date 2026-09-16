use crate::prelude::*;
use super::string::String;

impl String {
    #[jvm_ext]
    pub fn from_owned(s: std::string::String) -> Self {
        let bytes: Vec<i8> = s.into_bytes().into_iter().map(|b| b as i8).collect();
        let mut inst = String::default();
        inst.__set_value(Rc::new(RefCell::new(bytes)));
        inst.__set_coder(0i8);
        inst
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rc = self.__get_value();
        let bytes = rc.borrow();
        if self.__get_coder() == 0 {
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
