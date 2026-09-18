// Java 数组类型封装：Rc<RefCell<Vec<T>>> 的可读层包装
// get/set/len 隐藏 borrow_mut()，让生成代码保持 Java 语义可读性
// 调用方只需 array.get(i)、array.set(i, v)、array.len()，无需接触 RefCell API

use std::cell::RefCell;
use std::rc::Rc;

/// Java 数组 newtype。封装 Rc<RefCell<Vec<T>>>，对外提供 Java 语义的下标访问。
///
/// Java 数组是语言原语（T[]），不属于任何 Java 包，此类型是纯 Rust 侧封装。
///
/// 生成代码替换示例：
///   替换前：arr.borrow()[i as usize].clone()
///   替换后：arr.get(i)
#[derive(Clone, Debug)]
pub struct Array<T>(Rc<RefCell<Vec<T>>>);

impl<T: Clone + Default + 'static> Array<T> {
    /// 创建长度为 len 的数组，元素初始化为类型默认值（对应 Java newarray/anewarray）
    pub fn new(len: i32) -> Self {
        Array(Rc::new(RefCell::new(vec![T::default(); len as usize])))
    }

    /// 读取下标 i 的元素（对应 Java iaload/aaload 等）
    pub fn get(&self, i: i32) -> T {
        self.0.borrow()[i as usize].clone()
    }

    /// 写入下标 i 的元素（对应 Java iastore/aastore 等）
    pub fn set(&self, i: i32, v: T) {
        self.0.borrow_mut()[i as usize] = v;
    }

    /// 数组长度（对应 Java arraylength 字节码）
    pub fn len(&self) -> i32 {
        self.0.borrow().len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

impl<T> From<Vec<T>> for Array<T> {
    /// 从 Vec<T> 构造，用于字面量数组初始化（对应 Java 数组初始化器）
    fn from(v: Vec<T>) -> Self {
        Array(Rc::new(RefCell::new(v)))
    }
}
