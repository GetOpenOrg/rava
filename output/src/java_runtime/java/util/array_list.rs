// java.util.ArrayList<E> 存根（元素类型泛化为 i32）
pub fn new() -> Vec<i32> { Vec::new() }
pub fn add(list: &mut Vec<i32>, v: i32)        { list.push(v); }
pub fn get(list: &[i32], idx: i32) -> i32       { list[idx as usize] }
pub fn set(list: &mut Vec<i32>, idx: i32, v: i32) { list[idx as usize] = v; }
pub fn size(list: &[i32]) -> i32                 { list.len() as i32 }
pub fn remove(list: &mut Vec<i32>, idx: i32)    { list.remove(idx as usize); }
pub fn contains(list: &[i32], v: i32) -> bool   { list.contains(&v) }
pub fn clear(list: &mut Vec<i32>)               { list.clear(); }
pub fn is_empty(list: &[i32]) -> bool           { list.is_empty() }
