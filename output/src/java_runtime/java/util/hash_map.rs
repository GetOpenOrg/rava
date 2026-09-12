// java.util.HashMap<K,V> 存根（K=String, V=i32）
use std::collections::HashMap;
pub fn new() -> HashMap<String, i32> { HashMap::new() }
pub fn put(map: &mut HashMap<String, i32>, k: String, v: i32) -> Option<i32> {
    map.insert(k, v)
}
pub fn get(map: &HashMap<String, i32>, k: &str) -> i32 {
    map.get(k).copied().unwrap_or(0)
}
pub fn contains_key(map: &HashMap<String, i32>, k: &str) -> bool { map.contains_key(k) }
pub fn remove(map: &mut HashMap<String, i32>, k: &str)           { map.remove(k); }
pub fn size(map: &HashMap<String, i32>) -> i32                    { map.len() as i32 }
pub fn is_empty(map: &HashMap<String, i32>) -> bool               { map.is_empty() }
