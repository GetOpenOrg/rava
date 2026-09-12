//! java.util.HashSet<T> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashSet<T>(Rc<RefCell<std::collections::HashSet<T>>>);

impl<T: Clone + Eq + std::hash::Hash + 'static> HashSet<T> {
    pub fn new() -> Result<Self> {
        Ok(HashSet(Rc::new(RefCell::new(std::collections::HashSet::new()))))
    }
    pub fn add(&self, v: T) -> bool  { self.0.borrow_mut().insert(v) }
    pub fn contains(&self, v: &T) -> bool { self.0.borrow().contains(v) }
    pub fn remove(&self, v: &T) -> bool   { self.0.borrow_mut().remove(v) }
    pub fn size(&self) -> i32  { self.0.borrow().len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.borrow().is_empty() }
}

impl<T> Clone for HashSet<T> {
    fn clone(&self) -> Self { HashSet(self.0.clone()) }
}
