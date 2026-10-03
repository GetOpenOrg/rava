//! `java/lang/VirtualThread` 的 ACC_NATIVE（类 1，计划 §21.8 a3-T4）。其余方法（含 `<clinit>` 的调度器
//! `ForkJoinPool` / `CarrierThread` 与延时调度器 `UNPARKER`）按字节码翻译：虚拟线程经
//! `VirtualThread.runContinuation` → `Continuation.run` 挂载到载体线程上执行，`park` / `sleep` 经
//! `Continuation.yield` 卸载（有栈协程，`jdk/internal/vm/continuation_impl.rs`），被 pin 时按字节码
//! `parkOnCarrierThread` 在载体上阻塞。

use crate::prelude::*;
use super::virtual_thread::VirtualThread;

impl VirtualThread {
    /// native `registerNatives()`：JNI 注册（notifyJvmti* 等 JVMTI 通知）；无 JVMTI——no-op。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    // JVMTI 虚拟线程事件通知（mount / unmount / start / end / 隐藏帧）：原生二进制无 JVMTI
    // 代理，事件无接收方——no-op（HotSpot 在无 JVMTI 环境下同为空操作）。

    #[jvm_native]
    pub fn notifyJvmtiStart(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn notifyJvmtiEnd(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn notifyJvmtiMount(&self, _hide: bool) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn notifyJvmtiUnmount(&self, _hide: bool) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    pub fn notifyJvmtiHideFrames(&self, _hide: bool) -> Result<()> {
        Ok(())
    }
}
