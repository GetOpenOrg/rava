/// Field<T>：实例字段封装，支持通过不可变引用修改（Java 字段语义）。
pub struct Field<T>(std::cell::RefCell<T>);

impl<T: Clone> Field<T> {
    pub fn new(v: T) -> Self { Field(std::cell::RefCell::new(v)) }
    pub fn get(&self) -> T  { self.0.borrow().clone() }
    pub fn set(&self, v: T) { *self.0.borrow_mut() = v; }
}

impl<T: Default + Clone> Default for Field<T> {
    fn default() -> Self { Field::new(T::default()) }
}
