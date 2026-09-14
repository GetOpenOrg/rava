/// JField<T>：实例字段封装，支持通过不可变引用修改（Java 字段语义）。
/// 使用 Box<RefCell<T>> 确保堆分配，使循环引用结构（如 Reference→ReferenceQueue→Reference）
/// 可以确定大小，符合 Java 所有对象均为堆引用的语义。
pub struct JField<T>(Box<std::cell::RefCell<T>>);

impl<T: Clone> JField<T> {
    pub fn new(v: T) -> Self { JField(Box::new(std::cell::RefCell::new(v))) }
    pub fn get(&self) -> T  { self.0.borrow().clone() }
    pub fn set(&self, v: T) { *self.0.borrow_mut() = v; }
}

impl<T: Clone> Clone for JField<T> {
    fn clone(&self) -> Self { JField::new(self.get()) }
}

impl<T: Default + Clone> Default for JField<T> {
    fn default() -> Self { JField::new(T::default()) }
}
