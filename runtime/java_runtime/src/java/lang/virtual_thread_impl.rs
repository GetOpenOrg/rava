//! `java/lang/VirtualThread` 的 ACC_NATIVE（类 1，计划 §21.8 a3-T4）。其余方法（含 `<clinit>` 的调度器
//! `ForkJoinPool` / `CarrierThread` 与延时调度器 `UNPARKER`）按字节码翻译：虚拟线程经
//! `VirtualThread.runContinuation` → `Continuation.run` 挂载到载体线程上执行，`park` / `sleep` 经
//! `Continuation.yield` 卸载（有栈协程，`jdk/internal/vm/continuation_impl.rs`），被 pin 时按字节码
//! `parkOnCarrierThread` 在载体上阻塞。

use crate::java::lang::VirtualThread__Methods as _;
use crate::prelude::*;
use super::virtual_thread::VirtualThread;
use super::__VirtualThreadHw;

impl __VirtualThreadHw for VirtualThread {
    /// native `registerNatives()`：JNI 注册（notifyJvmti* 等 JVMTI 通知）；无 JVMTI——no-op。
    #[jvm_native]
    fn registerNatives() -> Result<()> {
        Ok(())
    }

    // JVMTI 虚拟线程事件通知（mount / unmount / start / end / 隐藏帧）：原生二进制无 JVMTI
    // 代理，事件无接收方——no-op（HotSpot 在无 JVMTI 环境下同为空操作）。

    #[jvm_native]
    fn notifyJvmtiStart(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    fn notifyJvmtiEnd(&self) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    fn notifyJvmtiMount(&self, _hide: bool) -> Result<()> {
        Ok(())
    }

    #[jvm_native]
    fn notifyJvmtiUnmount(&self, _hide: bool) -> Result<()> {
        Ok(())
    }

    /// JDK 21：隐藏帧通知（JDK 25 已删）
    #[cfg(not(jdk_ge_25))]
    #[jvm_native]
    fn notifyJvmtiHideFrames(&self, _hide: bool) -> Result<()> {
        Ok(())
    }

    /// JDK 25：挂起禁用通知（JVMTI）——no-op
    #[cfg(jdk_ge_25)]
    #[jvm_native]
    fn notifyJvmtiDisableSuspend(_enter: bool) -> Result<()> {
        Ok(())
    }

    /// JDK 25：pin 住时的 JFR `VirtualThreadPinned` 事件；无 JFR 记录器——no-op
    #[cfg(jdk_ge_25)]
    #[jvm_native]
    fn postPinnedEvent(_op: String) -> Result<()> {
        Ok(())
    }

    /// JDK 25：取 VM 维护的「监视器释放后待解阻」虚拟线程链表（`unblocker` 守护线程循环调用，链表空时
    /// HotSpot 在此阻塞）。本模型虚拟线程在 synchronized 上被 pin 于载体线程阻塞、不经 VM 挂到等待链，
    /// 链表恒空——按 HotSpot 空表语义永久阻塞调用线程（守护线程，不影响进程退出）
    #[cfg(jdk_ge_25)]
    #[jvm_native]
    fn takeVirtualThreadListToUnblock() -> Result<VirtualThread> {
        loop {
            std::thread::park();
        }
    }
}
