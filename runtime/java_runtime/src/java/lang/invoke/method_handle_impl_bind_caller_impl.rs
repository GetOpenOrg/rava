//! `java/lang/invoke/MethodHandleImpl$BindCaller` 的运行期类定义点（方案
//! `docs/plans/2026-10-02-c1d-reflect-narrow.md` §3.3「CallerSensitive 方法句柄路径」）。
//!
//! JDK 对无 @CallerSensitiveAdapter 的 CS 方法以 ASM 模板（`generateInvokerTemplate`，
//! `BindCaller.<clinit>` 生成一次）为每个调用者类定义隐藏巢成员类 `H$$InjectedInvoker/0x…`。
//! 原生二进制无运行期类定义：注入类由 `crate::injected_invoker` 按宿主登记，其静态方法取
//! VM 支持类 `InjectedInvokerDyn`（字节码翻译）；模板字节唯一消费方是本类定义点，
//! 不再生成。vm_intrinsics.toml 登记。

use crate::prelude::*;
use super::method_handle_impl_bind_caller::MethodHandleImpl_BindCaller;
use crate::java::lang::Class;

impl MethodHandleImpl_BindCaller {
    /// `makeInjectedInvoker(Class targetClass)`：定义（登记）调用者类的注入调用器隐藏类并返回。
    #[jvm_native]
    pub fn makeInjectedInvoker(targetClass: Class) -> Result<Class> {
        let host = format!("{}", targetClass.__get_name()).replace('.', "/");
        Ok(Class::for_class(String::from(crate::injected_invoker::define(&host))))
    }

    /// `generateInvokerTemplate()`：模板类字节只供 `makeInjectedInvoker` 定义隐藏类，该定义点
    /// 由 VM 承载 → 空模板（ASM 生成链不执行）。
    #[jvm_native]
    pub fn generateInvokerTemplate() -> Result<JArray<i8>> {
        Ok(JArray::from(Vec::<i8>::new()))
    }
}
