use crate::prelude::*;
use super::event::Event;

// jdk.internal.event.Event（JFR 事件的 java.base 占位基类）：原生二进制无 JFR，
// 事件恒未启用。按调用链按需实现，其余保持存根。

impl Event {
    /// `isEnabled()Z`：java.base 层占位实现恒 false（JFR 启用时重定义事件类；
    /// 无 JFR 即不可用）。JDK25 `Thread.beforeSleep` 据此跳过 time / begin，
    /// 返回 null 使 afterSleep 不 commit。经 vtable 槽位供子类事件继承。
    #[jvm_boundary]
    pub fn __impl_isEnabled(&self) -> Result<bool> {
        Ok(false)
    }

    /// `shouldCommit()Z`：事件未启用 → false。`ObjectInputStream.filterCheck` 的
    /// DeserializationEvent 等据此跳过字段赋值与 commit（SerializableDemo 揭出）。
    #[jvm_boundary]
    pub fn __impl_shouldCommit(&self) -> Result<bool> {
        Ok(false)
    }

    /// `begin()V` / `end()V` / `commit()V`：计时与提交在未启用事件上是空操作（JDK 占位层同义）。
    #[jvm_boundary]
    pub fn __impl_begin(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_end(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn __impl_commit(&self) -> Result<()> {
        Ok(())
    }

    /// `set(int, Object)V`：按下标写字段（JFR 生成子类覆盖）；占位层空操作。
    #[jvm_boundary]
    pub fn __impl_set(&self, _index: i32, _value: Object) -> Result<()> {
        Ok(())
    }
}
