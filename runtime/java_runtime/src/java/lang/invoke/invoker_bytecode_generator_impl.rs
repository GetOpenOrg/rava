//! `java/lang/invoke/InvokerBytecodeGenerator` 手写伴生：VM 边界类（vm_boundary.txt），
//! MH-native（docs/plans/2026-09-26-mh-native.md §二-2）。
//!
//! JDK 在 `LambdaForm.compileToBytecode` / `prepare` 里经本类把 LambdaForm 编译成隐藏类字节码
//!（ASM 生成 + `Lookup.defineHiddenClass`）。原生二进制不能在运行期定义类；句柄调用由
//! `MethodHandle.invokeBasic` 的原生 LambdaForm 解释器承载（method_handle_impl.rs），不经
//! `vmentry`。故编译入口返回 null（vmentry 留空、isCompiled 置位，形态不再重复编译），
//! 静态可调用性判定（仅断言消费）恒 true。整类截断 ASM 生成链。

use crate::prelude::*;
use super::invoker_bytecode_generator::implref::InvokerBytecodeGenerator;
use super::{LambdaForm, LambdaForm_Name, LambdaForm_NamedFunction, MemberName, MethodType};

impl InvokerBytecodeGenerator {
    /// `generateCustomizedCode(LambdaForm, MethodType)`：不生成字节码 → null vmentry。
    #[jvm_boundary]
    pub fn generateCustomizedCode(_form: LambdaForm, _invokerType: MethodType) -> Result<MemberName> {
        Ok(MemberName::default())
    }

    /// `generateLambdaFormInterpreterEntryPoint(MethodType)`：解释入口同样由原生解释器承载 → null。
    #[jvm_boundary]
    pub fn generateLambdaFormInterpreterEntryPoint(_mt: MethodType) -> Result<MemberName> {
        Ok(MemberName::default())
    }

    /// `lookupPregenerated(LambdaForm, MethodType)`：预生成 Holder 入口同样不经 vmentry → null。
    #[jvm_boundary]
    pub fn lookupPregenerated(_form: LambdaForm, _invokerType: MethodType) -> Result<MemberName> {
        Ok(MemberName::default())
    }

    /// `isStaticallyInvocable(NamedFunction...)`：仅断言消费（解释器对全部成员可调）→ true。
    #[jvm_boundary]
    pub fn isStaticallyInvocable_arr_lambdaform_namedfunction(_functions: JArray<LambdaForm_NamedFunction>) -> Result<bool> {
        Ok(true)
    }

    #[jvm_boundary]
    pub fn isStaticallyInvocable_lambdaform_name(_name: LambdaForm_Name) -> Result<bool> {
        Ok(true)
    }

    #[jvm_boundary]
    pub fn isStaticallyInvocable_membername(_member: MemberName) -> Result<bool> {
        Ok(true)
    }
}
