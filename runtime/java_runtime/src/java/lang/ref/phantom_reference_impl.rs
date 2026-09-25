//! `java/lang/ref/PhantomReference` 手写伴生：native 方法。
//!
//! JDK 中 PhantomReference 覆盖 `refersTo0` 为独立 native（虚引用 `get()` 恒 null，
//! 但 `refersTo` 仍可判定 referent 身份）。运行时无 GC（见 reference_impl.rs），
//! referent 仅经 `clear()` 置空——语义与 `Reference.refersTo0` 同：引用身份比较。

use crate::prelude::*;
use super::phantom_reference::PhantomReference;

impl<T: Clone + Default + 'static + From<Object> + Into<Object>> PhantomReference<T> {
    /// native `refersTo0(Object)`：referent 引用身份比较（acmp 语义，null 同样成立）。
    /// 消费方：Reference.refersTo 经 refersToImpl 虚分派到本覆盖（TestReferenceTypes）。
    #[jvm_native]
    pub fn refersTo0(&self, obj: Object) -> Result<bool> {
        Ok(Into::<Object>::into(self.__get_referent()) == obj)
    }
}
