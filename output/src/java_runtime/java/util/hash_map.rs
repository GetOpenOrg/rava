//! java.util.HashMap<K,V> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashMap<K, V>(Rc<RefCell<std::collections::HashMap<K, V>>>);

impl<K: Clone + Eq + std::hash::Hash + 'static, V: Clone + 'static> HashMap<K, V> {
    pub fn new() -> Result<Self> {
        Ok(HashMap(Rc::new(RefCell::new(std::collections::HashMap::new()))))
    }
    pub fn put(&self, k: K, v: V) -> Result<Option<V>> {
        Ok(self.0.borrow_mut().insert(k, v))
    }
    pub fn get(&self, k: K) -> Result<V> where V: Default {
        Ok(self.0.borrow().get(&k).cloned().unwrap_or_default())
    }
    pub fn get_or_default(&self, k: K, d: V) -> Result<V> {
        Ok(self.0.borrow().get(&k).cloned().unwrap_or(d))
    }
    pub fn contains_key(&self, k: K) -> Result<bool> {
        Ok(self.0.borrow().contains_key(&k))
    }
    /// Java camelCase alias for contains_key
    #[allow(non_snake_case)]
    pub fn containsKey(&self, k: K) -> Result<bool> {
        self.contains_key(k)
    }
    pub fn remove(&self, k: K) -> Result<Option<V>> {
        Ok(self.0.borrow_mut().remove(&k))
    }
    pub fn size(&self) -> Result<i32>  { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
}

impl<K, V> Clone for HashMap<K, V> {
    fn clone(&self) -> Self { HashMap(self.0.clone()) }
}
