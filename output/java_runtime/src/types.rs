/// JField<T>：实例字段封装，支持通过不可变引用修改（Java 字段语义）。
pub struct JField<T>(std::cell::RefCell<T>);

impl<T: Clone> JField<T> {
    pub fn new(v: T) -> Self { JField(std::cell::RefCell::new(v)) }
    pub fn get(&self) -> T  { self.0.borrow().clone() }
    pub fn set(&self, v: T) { *self.0.borrow_mut() = v; }
}

impl<T: Clone> Clone for JField<T> {
    fn clone(&self) -> Self { JField::new(self.get()) }
}

impl<T: Default + Clone> Default for JField<T> {
    fn default() -> Self { JField::new(T::default()) }
}
