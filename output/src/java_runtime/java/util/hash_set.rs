// java.util.HashSet<E> 存根（E=i32）
use std::collections::HashSet;
pub fn new() -> HashSet<i32> { HashSet::new() }
pub fn add(set: &mut HashSet<i32>, v: i32) -> bool  { set.insert(v) }
pub fn contains(set: &HashSet<i32>, v: i32) -> bool { set.contains(&v) }
pub fn remove(set: &mut HashSet<i32>, v: i32) -> bool { set.remove(&v) }
pub fn size(set: &HashSet<i32>) -> i32               { set.len() as i32 }
pub fn is_empty(set: &HashSet<i32>) -> bool          { set.is_empty() }
