//! `java/lang/ref/Reference` 手写伴生：native 方法。
//!
//! 运行时无 GC（Rc 引用计数），引用对象的 referent 恒为强可达——
//! 从不被清除，也不入队。JDK 语义在此模型下的等价面：
//!   - `refersTo0(obj)`：referent 与 obj 同一对象（`==` 引用身份，null 同样成立）；
//!     即 JDK `refersTo` 的规范定义 `get() == obj` 在 referent 未被清除时的取值；
//!   - `clear0()`：显式清除是唯一的 referent 置空途径（GC 清除不发生）。

use crate::prelude::*;
use super::reference::Reference;

impl<T: Clone + Default + 'static + From<Object> + Into<Object>> Reference<T> {
    /// native `refersTo0(Object)`：referent 引用身份比较（Object `==` 即 acmp 语义）。
    /// 消费方：ClassCache 的软引用缓存槽（ObjectStreamClass.lookup 链）。
    #[jvm_native]
    pub fn refersTo0(&self, obj: Object) -> Result<bool> {
        Ok(Into::<Object>::into(self.__get_referent()) == obj)
    }

    /// native `clear0()`：清除 referent（此后 get() 返回 null；不入队——JDK
    /// clear 同样不触发入队）。消费方：ClassValue/ClassCache 缓存条目替换路径。
    #[jvm_native]
    pub fn clear0(&self) -> Result<()> {
        self.__set_referent(T::default());
        Ok(())
    }
}
