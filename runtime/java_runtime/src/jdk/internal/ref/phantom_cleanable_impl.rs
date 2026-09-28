//! `jdk/internal/ref/PhantomCleanable` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 语义：Cleanable 登记到 Cleaner 的双向链表，referent 幻象可达时由 Cleaner 线程出队并执行
//! `performCleanup`。本运行时无 GC / 引用处理（FS-G2 / FS-G4）：对象经引用计数常驻、幻象引用永不
//! 入队、清理动作永不触发——登记与注销的可观测语义都是无操作。清理的唯一路径是显式 close
//!（FileDescriptor.close0 等）。

use crate::prelude::*;
use super::phantom_cleanable::PhantomCleanable;
use crate::java::lang::r#ref::Cleaner;

impl<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe> PhantomCleanable<T> {
    /// `<init>(Object referent, Cleaner cleaner)`：不入 Cleaner 链表（见模块说明）。
    #[jvm_boundary]
    pub fn new_obj_cleaner(referent: T, cleaner: Cleaner) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on_obj_cleaner(this, referent, cleaner)
    }

    #[doc(hidden)]
    pub fn __init_on_obj_cleaner(this: Self, _referent: T, _cleaner: Cleaner) -> Result<Self> {
        Ok(this)
    }

    /// `clear()`：从链表摘除并清 referent——未入链表、无 referent 追踪，无操作。
    #[jvm_boundary]
    pub fn __impl_clear(&self) -> Result<()> {
        Ok(())
    }
}
