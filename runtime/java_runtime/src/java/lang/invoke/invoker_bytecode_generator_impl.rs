//! `java/lang/invoke/InvokerBytecodeGenerator` 手写伴生：类 2 运行模型替换（vm_intrinsics.toml
//! [[intrinsic]] class_definition 登记），MH-native（docs/plans/2026-09-26-mh-native.md §二-2）。
//!
//! JDK 在 `LambdaForm.compileToBytecode` / `prepare` 里经本类把 LambdaForm 编译成隐藏类字节码
//!（ASM 生成 + `Lookup.defineHiddenClass`）。原生二进制不能在运行期定义类；句柄调用由
//! `MethodHandle.invokeBasic` 的原生 LambdaForm 解释器承载（method_handle_impl.rs），不经
//! `vmentry`。但 `vmentry` 非空是 LambdaForm 的「已就绪」判据：`prepare()` 见 null 即新建
//! 解释形态（createBlankForType → constantZero → createFormsFor → SimpleMethodHandle.make
//! → MethodHandle.<init> → prepare），createFormsFor 缓存写回之前重入自身 → 无限递归
//!（RecordsSerializationTest 栈溢出）。故编译 / 解释入口返回**解释入口 MemberName**：
//! `LambdaForm.interpretWithArguments` 按调用类型的未解析引用（等价 HotSpot 的
//! `interpret_<sig>` 解释入口，执行仍由原生解释器承载）。只手写这两个类定义点；
//! `lookupPregenerated` / `isStaticallyInvocable` 按字节码翻译（唯一调用面是上述生成链与
//! 引导类 assert，后者经 `$assertionsDisabled` 折叠不入闭包，a3-X1）。

use crate::prelude::*;
use super::invoker_bytecode_generator::InvokerBytecodeGenerator;
use super::{LambdaForm, MemberName, MethodType};

impl InvokerBytecodeGenerator {
    /// `generateCustomizedCode(LambdaForm, MethodType)`：不生成字节码 → 解释入口 vmentry。
    #[jvm_native]
    pub fn generateCustomizedCode(_form: LambdaForm, invokerType: MethodType) -> Result<MemberName> {
        interpreter_entry(invokerType)
    }

    /// `generateLambdaFormInterpreterEntryPoint(MethodType)`：解释入口由原生解释器承载。
    #[jvm_native]
    pub fn generateLambdaFormInterpreterEntryPoint(mt: MethodType) -> Result<MemberName> {
        interpreter_entry(mt)
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
