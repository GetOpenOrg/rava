//! java.lang.String 同构类型。
//! 遮蔽 Rust 的 std::string::String，是预期行为。
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct String(std::string::String);

impl String {
    pub fn new() -> Self { String(std::string::String::new()) }
    pub fn from(s: &str) -> Self { String(s.to_owned()) }
    pub fn from_owned(s: std::string::String) -> Self { String(s) }
    pub fn append(&mut self, s: &String) { self.0.push_str(&s.0); }
    pub fn append_str(&mut self, s: &str)  { self.0.push_str(s); }
    pub fn length(&self) -> i32  { self.0.len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn to_std(&self) -> &str  { &self.0 }
    pub fn concat(&self, other: &String) -> String {
        String(format!("{}{}", self.0, other.0))
    }
    pub fn trim(&self) -> String { String(self.0.trim().to_owned()) }
    pub fn to_upper_case(&self) -> String { String(self.0.to_uppercase()) }
    pub fn to_lower_case(&self) -> String { String(self.0.to_lowercase()) }
    pub fn contains_str(&self, s: &str) -> bool { self.0.contains(s) }
    pub fn char_at(&self, i: i32) -> u16 {
        self.0.chars().nth(i as usize).unwrap_or('\0') as u16
    }
    pub fn substring(&self, start: i32) -> String {
        String(self.0.chars().skip(start as usize).collect())
    }
    pub fn substring_end(&self, start: i32, end: i32) -> String {
        String(self.0.chars().skip(start as usize).take((end - start) as usize).collect())
    }
    pub fn index_of_str(&self, s: &str) -> i32 {
        self.0.find(s).map(|i| i as i32).unwrap_or(-1)
    }
    pub fn replace_str(&self, old: &str, new: &str) -> String {
        String(self.0.replace(old, new))
    }
    pub fn value_of_i32(v: i32)  -> String { String(v.to_string()) }
    pub fn value_of_i64(v: i64)  -> String { String(v.to_string()) }
    pub fn value_of_f64(v: f64)  -> String { String(v.to_string()) }
    pub fn value_of_bool(v: bool) -> String { String(v.to_string()) }
    pub fn parse_int(&self)  -> i32 { self.0.parse::<i32>().unwrap_or(0) }
    pub fn parse_long(&self) -> i64 { self.0.parse::<i64>().unwrap_or(0) }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self { String(s.to_owned()) }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self { String(s) }
}

impl From<i32>  for String { fn from(v: i32)  -> Self { String(v.to_string()) } }
impl From<i64>  for String { fn from(v: i64)  -> Self { String(v.to_string()) } }
impl From<f64>  for String { fn from(v: f64)  -> Self { String(v.to_string()) } }
impl From<bool> for String { fn from(v: bool) -> Self { String(v.to_string()) } }
