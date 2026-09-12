//! java.util.ArrayList<T> 同构类型（内部用 Rc<RefCell<Vec<T>>>）
use crate::java_runtime::error::{JvmError, Result};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ArrayList<T>(Rc<RefCell<Vec<T>>>);

impl<T: Clone + 'static> ArrayList<T> {
    pub fn new() -> Result<Self> {
        Ok(ArrayList(Rc::new(RefCell::new(Vec::new()))))
    }
    pub fn add(&self, v: T) -> Result<bool> {
        self.0.borrow_mut().push(v);
        Ok(true)
    }
    pub fn get(&self, i: i32) -> Result<T> {
        self.0.borrow().get(i as usize).cloned()
            .ok_or(JvmError::ArrayIndexOutOfBoundsException(i))
    }
    pub fn set_at(&self, i: i32, v: T) -> Result<T> {
        let mut b = self.0.borrow_mut();
        let old = b.get(i as usize).cloned()
            .ok_or(JvmError::ArrayIndexOutOfBoundsException(i))?;
        b[i as usize] = v;
        Ok(old)
    }
    pub fn size(&self) -> Result<i32> { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
    pub fn remove_at(&self, i: i32) -> Result<()> {
        self.0.borrow_mut().remove(i as usize);
        Ok(())
    }
    pub fn clear(&self) -> Result<()> { self.0.borrow_mut().clear(); Ok(()) }
    pub fn contains(&self, v: T) -> Result<bool> where T: PartialEq {
        Ok(self.0.borrow().contains(&v))
    }
    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        for item in self.0.borrow().iter() { f(item); }
    }
}

impl<T> Clone for ArrayList<T> {
    fn clone(&self) -> Self { ArrayList(self.0.clone()) }
}
