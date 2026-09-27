//! `java/lang/invoke/InvokerBytecodeGenerator` 手写伴生：VM 边界类（vm_boundary.txt），
//! MH-native（docs/plans/2026-09-26-mh-native.md §二-2）。
//!
//! JDK 在 `LambdaForm.compileToBytecode` / `prepare` 里经本类把 LambdaForm 编译成隐藏类字节码
//!（ASM 生成 + `Lookup.defineHiddenClass`）。原生二进制不能在运行期定义类；句柄调用由
//! `MethodHandle.invokeBasic` 的原生 LambdaForm 解释器承载（method_handle_impl.rs），不经
//! `vmentry`。但 `vmentry` 非空是 LambdaForm 的「已就绪」判据：`prepare()` 见 null 即新建
//! 解释形态（createBlankForType → constantZero → createFormsFor → SimpleMethodHandle.make
//! → MethodHandle.<init> → prepare），createFormsFor 缓存写回之前重入自身 → 无限递归
//!（RecordsSerializationTest 栈溢出）。故编译 / 解释入口返回**解释入口 MemberName**：
//! `LambdaForm.interpretWithArguments` 按调用类型的未解析引用（等价 HotSpot 的
//! `interpret_<sig>` 解释入口，执行仍由原生解释器承载）。静态可调用性判定（仅断言消费）
//! 恒 true。整类截断 ASM 生成链。

use crate::prelude::*;
use super::invoker_bytecode_generator::implref::InvokerBytecodeGenerator;
use super::{LambdaForm, LambdaForm_Name, LambdaForm_NamedFunction, MemberName, MethodType};

impl InvokerBytecodeGenerator {
    /// `generateCustomizedCode(LambdaForm, MethodType)`：不生成字节码 → 解释入口 vmentry。
    #[jvm_boundary(upcalls = "java/lang/invoke/MemberName.<init>:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;B)V")]
    pub fn generateCustomizedCode(_form: LambdaForm, invokerType: MethodType) -> Result<MemberName> {
        interpreter_entry(invokerType)
    }

    /// `generateLambdaFormInterpreterEntryPoint(MethodType)`：解释入口由原生解释器承载。
    #[jvm_boundary(upcalls = "java/lang/invoke/MemberName.<init>:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;B)V")]
    pub fn generateLambdaFormInterpreterEntryPoint(mt: MethodType) -> Result<MemberName> {
        interpreter_entry(mt)
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

/// 解释入口 MemberName：`LambdaForm.interpretWithArguments`，REF_invokeStatic，类型为调用类型
/// （未解析——原生解释器不经 vmentry 取目标，引用只承载「已就绪」语义与类型信息）。
fn interpreter_entry(mt: MethodType) -> Result<MemberName> {
    const REF_INVOKE_STATIC: i8 = 6;
    MemberName::new_class_str_methodtype_b(
        crate::java::lang::Class::for_class(crate::java::lang::String::from("java/lang/invoke/LambdaForm")),
        crate::java::lang::String::from("interpretWithArguments"),
        mt,
        REF_INVOKE_STATIC,
    )
}
