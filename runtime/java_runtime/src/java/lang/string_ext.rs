use crate::prelude::*;
use super::string::String;

impl String {
    #[jvm_ext]
    pub fn from_owned(s: std::string::String) -> Self {
        let bytes: Vec<i8> = s.into_bytes().into_iter().map(|b| b as i8).collect();
        let mut inst = String::default();
        inst._init_not_null();
        inst.__set_value(JArray::from(bytes));
        inst.__set_coder(0i8);
        inst
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.__get_value();
        let len = val.len();
        if self.__get_coder() == 0i8 {
            let s: std::string::String = (0..len).map(|i| val.get(i) as u8 as char).collect();
            write!(f, "{}", s)
        } else {
            let u16s: Vec<u16> = (0..len as usize / 2).map(|i| {
                let b0 = val.get(i as i32 * 2) as u8;
                let b1 = if (i as i32 * 2 + 1) < len { val.get(i as i32 * 2 + 1) as u8 } else { 0 };
                u16::from_le_bytes([b0, b1])
            }).collect();
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
