// Java 数组类型封装：Rc<RefCell<Vec<T>>> 的可读层包装
// get/set/len 隐藏 borrow_mut()，让生成代码保持 Java 语义可读性
// 调用方只需 array.get(i)、array.set(i, v)、array.len()，无需接触 RefCell API

use std::cell::RefCell;
use std::rc::Rc;

/// Java 数组 newtype。封装 Rc<RefCell<Vec<T>>>，对外提供 Java 语义的下标访问。
/// Clone 共享底层 Rc（Java 数组是引用类型，赋值不复制内容）。
#[derive(Clone, Debug)]
pub struct JArray<T>(Rc<RefCell<Vec<T>>>);

impl<T: Clone + Default + 'static> Default for JArray<T> {
    fn default() -> Self { JArray::new(0) }
}

impl<T: PartialEq + Clone + 'static> PartialEq for JArray<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Clone + Default + 'static> JArray<T> {
    /// 创建长度为 len 的数组，元素初始化为类型默认值（对应 Java newarray/anewarray）
    pub fn new(len: i32) -> Self {
        JArray(Rc::new(RefCell::new(vec![T::default(); len as usize])))
    }

    /// 读取下标 i 的元素（对应 Java iaload/aaload 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *aload）。
    pub fn get(&self, i: i32) -> crate::error::Result<T> {
        let data = self.0.borrow();
        if i < 0 || i as usize >= data.len() {
            return Err(crate::error::JvmError::array_index_out_of_bounds(i, data.len() as i32));
        }
        Ok(data[i as usize].clone())
    }

    /// 写入下标 i 的元素（对应 Java iastore/aastore 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *astore）。
    pub fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        let mut data = self.0.borrow_mut();
        if i < 0 || i as usize >= data.len() {
            return Err(crate::error::JvmError::array_index_out_of_bounds(i, data.len() as i32));
        }
        data[i as usize] = v;
        Ok(())
    }

    /// 元素快照（手写 VM 层批量读取用，不经过逐元素边界检查）
    pub fn to_vec(&self) -> Vec<T> {
        self.0.borrow().clone()
    }

    /// 数组长度（对应 Java arraylength 字节码）
    pub fn len(&self) -> i32 {
        self.0.borrow().len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

impl<T> From<Vec<T>> for JArray<T> {
    /// 从 Vec<T> 构造，用于字面量数组初始化（对应 Java 数组初始化器）
    fn from(v: Vec<T>) -> Self {
        JArray(Rc::new(RefCell::new(v)))
    }
}
