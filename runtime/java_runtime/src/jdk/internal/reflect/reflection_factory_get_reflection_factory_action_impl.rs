//! `jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction` 手写伴生：
//! `AccessibleObject` 类初始化经 `AccessController.doPrivileged` 消费本 action。
//!
//! JDK 语义面：`<init>` 空体；`run()` 返回 `ReflectionFactory.getReflectionFactory()`
//! 的单例（伴生实现，见 reflection_factory_impl.rs）。

use crate::prelude::*;
use super::reflection_factory::ReflectionFactory;
use super::reflection_factory_get_reflection_factory_action::ReflectionFactory_GetReflectionFactoryAction;

impl ReflectionFactory_GetReflectionFactoryAction {
    /// `<init>()V`：空构造（默认形态 + 非空落置）。
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `run()`（虚方法，经 wrapper 钩子 `__impl_run` 执行）：返回反射工厂单例。
    pub fn __impl_run(&self) -> Result<ReflectionFactory> {
        ReflectionFactory::getReflectionFactory()
    }
}
