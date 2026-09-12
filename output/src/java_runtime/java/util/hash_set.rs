//! java.util.HashSet<T> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashSet<T>(Rc<RefCell<std::collections::HashSet<T>>>);

impl<T: Clone + Eq + std::hash::Hash + 'static> HashSet<T> {
    pub fn new() -> Result<Self> {
        Ok(HashSet(Rc::new(RefCell::new(std::collections::HashSet::new()))))
    }
    pub fn add(&self, v: T) -> Result<bool> { Ok(self.0.borrow_mut().insert(v)) }
    pub fn contains(&self, v: T) -> Result<bool> {
        Ok(self.0.borrow().contains(&v))
    }
    pub fn remove(&self, v: T) -> Result<bool> {
        Ok(self.0.borrow_mut().remove(&v))
    }
    pub fn size(&self) -> Result<i32>  { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
}

impl<T> Clone for HashSet<T> {
    fn clone(&self) -> Self { HashSet(self.0.clone()) }
}
