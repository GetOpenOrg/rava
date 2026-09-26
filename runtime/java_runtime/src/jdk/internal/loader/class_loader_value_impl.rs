//! `jdk/internal/loader/ClassLoaderValue` 手写伴生：`ClassValue`/`ClassCache`
//! 映射层（ObjectStreamClass 的 localDescs 缓存链）触达的无参构造。

use crate::prelude::*;
use super::class_loader_value::ClassLoaderValue;

impl<V: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe> ClassLoaderValue<V> {
    /// `<init>()V`：空构造（JDK 源 `public ClassLoaderValue() {}`——泛型形参
    /// 仅存在于类型位置，默认形态即完整构造）。
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }
}
