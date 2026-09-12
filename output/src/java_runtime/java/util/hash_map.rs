//! java.util.HashMap<K,V> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashMap<K, V>(Rc<RefCell<std::collections::HashMap<K, V>>>);

impl<K: Clone + Eq + std::hash::Hash + 'static, V: Clone + 'static> HashMap<K, V> {
    pub fn new() -> Result<Self> {
        Ok(HashMap(Rc::new(RefCell::new(std::collections::HashMap::new()))))
    }
    pub fn put(&self, k: K, v: V) -> Option<V> {
        self.0.borrow_mut().insert(k, v)
    }
    pub fn get(&self, k: &K) -> Option<V> {
        self.0.borrow().get(k).cloned()
    }
    pub fn get_or_default(&self, k: &K, d: V) -> V {
        self.0.borrow().get(k).cloned().unwrap_or(d)
    }
    pub fn contains_key(&self, k: &K) -> bool { self.0.borrow().contains_key(k) }
    pub fn remove(&self, k: &K) -> Option<V>  { self.0.borrow_mut().remove(k) }
    pub fn size(&self) -> i32  { self.0.borrow().len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.borrow().is_empty() }
}

impl<K, V> Clone for HashMap<K, V> {
    fn clone(&self) -> Self { HashMap(self.0.clone()) }
}
