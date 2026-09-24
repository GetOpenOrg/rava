//! `jdk/internal/reflect/ReflectionFactory` 手写伴生：JUnit Runner 路径的
//! `AccessibleObject` 类初始化触达（`setAccessible` → clinit 的
//! `doPrivileged(GetReflectionFactoryAction)` 链）。
//!
//! JDK 21 语义面：`<init>()V` 为私有空体；`soleInstance` 由 clinit 内联初始化的
//! 单例；`getReflectionFactory()` 即返回该单例。本伴生按需实现这三点——
//! `newMethodAccessor` / `newFieldAccessor` 等 accessor 工厂族不在翻译侧
//! 反射协议（L3 `__reflect_dispatch`）的路径上，保持 panic 存根。

use crate::prelude::*;
use super::reflection_factory::ReflectionFactory;

impl ReflectionFactory {
    /// `<init>()V`：私有空构造（JDK 21 `private ReflectionFactory() {}`）。
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// static `getReflectionFactory()`：返回 clinit 单例 `soleInstance`。
    /// 生成侧 clinit 不在本闭包调用链上（静态字段初始化未发射），单例在此
    /// 惰性落置——`thread_local` 持有底层 `Object`（运行时单线程模型，
    /// Object 非 Sync），跨调用同一对象身份（JVM `soleInstance` 的可观测语义）。
    pub fn getReflectionFactory() -> Result<ReflectionFactory> {
        thread_local! {
            static SOLE_INSTANCE: RefCell<Option<Object>> = const { RefCell::new(None) };
        }
        let obj = SOLE_INSTANCE.with(|slot| {
            let mut slot = slot.borrow_mut();
            slot.get_or_insert_with(|| {
                // 构造为空体（不可失败）；Err 分支退回 null 单例仅为闭合类型
                match ReflectionFactory::new() {
                    Ok(rf) => Object::from(rf),
                    Err(_) => Object::default(),
                }
            })
            .clone()
        });
        Ok(ReflectionFactory::from(obj))
    }
}
