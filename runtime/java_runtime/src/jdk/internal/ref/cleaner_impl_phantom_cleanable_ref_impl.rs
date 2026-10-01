//! `jdk/internal/ref/CleanerImpl$PhantomCleanableRef` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! `Cleaner.register(obj, action)` 的登记对象。本运行时无 GC / 幻象引用入队（见
//! phantom_cleanable_impl.rs）：不入 Cleaner 链表，动作只在显式 `clean()` 时执行一次。

use crate::prelude::*;
use super::cleaner_impl_phantom_cleanable_ref::CleanerImpl_PhantomCleanableRef;
use crate::java::lang::r#ref::Cleaner;

impl CleanerImpl_PhantomCleanableRef {
    /// `<init>(Object obj, Cleaner cleaner, Runnable action)`：记录清理动作。
    #[jvm_boundary]
    pub fn new_obj_cleaner_runnable(obj: Object, cleaner: Cleaner, action: Object) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on_obj_cleaner_runnable(this, obj, cleaner, action)
    }

    #[doc(hidden)]
    pub fn __init_on_obj_cleaner_runnable(this: Self, _obj: Object, _cleaner: Cleaner, action: Object) -> Result<Self> {
        this.__set_action(action.into());
        Ok(this)
    }

    /// `performCleanup()`：运行登记的动作（JDK：`action.run()`）。
    #[jvm_boundary]
    pub fn __impl_performCleanup(&self) -> Result<()> {
        crate::java::lang::Runnable::from(self.__get_action()).run()
    }
}
