// java.lang.Object 基础行为
pub fn hash_code<T: std::hash::Hash>(v: &T) -> i32 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    v.hash(&mut h);
    h.finish() as i32
}
pub fn equals<T: PartialEq>(a: &T, b: &T) -> bool { a == b }
pub fn to_string_i32(v: i32) -> String { v.to_string() }
pub fn to_string_i64(v: i64) -> String { v.to_string() }
pub fn to_string_f64(v: f64) -> String { v.to_string() }
pub fn to_string_bool(v: bool) -> String { v.to_string() }
