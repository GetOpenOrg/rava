//! `sun/security/jca/GetInstance$Instance` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 消费方：`Security.getImpl`（翻译字节码）以 `Object[] {impl, provider}` 交给
//! `AlgorithmParameters.getInstance` 等 engine 类。

use crate::prelude::*;
use super::get_instance_instance::GetInstance_Instance;

impl GetInstance_Instance {
    /// `toArray()`：`new Object[] { impl, provider }`（JDK 同序）。
    #[jvm_boundary]
    pub fn toArray(&self) -> Result<JArray<Object>> {
        Ok(JArray::from(vec![self.__get_impl_(), Object::from(self.__get_provider())]))
    }
}
