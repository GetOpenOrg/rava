//! `java.lang.SecurityManager` 的 native 层（类 ③：VM 栈遍历的落地语义）。
//!
//! `getClassContext` = HotSpot `JVM_GetClassContext`：自 getClassContext 帧起逐帧（vframeStream
//! 的 security_next）取方法持有类，排除 native 帧与「安全栈遍历忽略」的帧
//!（`Method::is_ignored_by_security_stack_walk`：Method.invoke、MethodAccessorImpl 子类的方法、
//! MH 内建与 `@LambdaForm.Compiled` 帧）。getClassContext 自身是 native 帧，故结果从其调用者起。
//! 帧源见 `crate::vm_stack`。

use crate::prelude::*;
use super::*;

fn _internal_error(msg: &str) -> JvmError {
    match crate::java::lang::InternalError::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// HotSpot `Method::is_ignored_by_security_stack_walk`。
fn _ignored_by_security_walk(f: &crate::vm_stack::JavaFrame) -> bool {
    (f.class == "java/lang/reflect/Method" && f.method.name == "invoke")
        || f.class_extends("jdk/internal/reflect/MethodAccessorImpl")
        || crate::anno_pool::has_annotation(&f.class, f.method.annotations, "Ljava/lang/invoke/LambdaForm$Compiled;")
}

impl SecurityManager {
    /// native `getClassContext()`：调用栈上各帧的持有类（自调用者起）。
    #[jvm_native]
    pub fn getClassContext(&self) -> Result<JArray<Class>> {
        let frames = crate::vm_stack::capture_java_frames();
        // vframeStream 自 getClassContext 帧起（其上为本数据面自身的 Rust 帧，不成 Java 帧）
        let Some(start) = frames.iter()
            .position(|f| f.class == "java/lang/SecurityManager" && f.method.name == "getClassContext")
        else {
            return Err(_internal_error("JVM_GetClassContext must only be called from SecurityManager.getClassContext"));
        };
        let classes: Vec<Class> = frames[start..].iter()
            .filter(|f| !f.is_native() && !_ignored_by_security_walk(f))
            .map(|f| Class::for_class(String::from(f.class.as_str())))
            .collect();
        Ok(JArray::from(classes))
    }
}
