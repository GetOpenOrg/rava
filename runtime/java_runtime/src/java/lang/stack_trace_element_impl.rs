//! `java.lang.StackTraceElement` 的 native 层（S-19 #5 栈回溯数据面）。
//!
//! `initStackTraceElements`（JDK native）：把 `Throwable.backtrace` 载体展开为
//! 栈帧元素数组——载体即共置 `throwable_impl.rs` 捕获的 [`StackTraceFrames`]
//!（真实 Rust 栈，帧数真实、内容近似——compatibility.md「栈回溯」行的已声明
//! 边界）。数组本身（长度 = depth）由 Java 侧 `StackTraceElement.of` 分配，
//! 本 native 只填字段。
//!
//! `__impl_computeFormat`（翻译体手写覆盖）：Java 体经 `declaringClassObject`
//! 查询 class-loader / module 元数据（`Class.getClassLoader0` / `getModule`），
//! 该元数据面在原生二进制不存在；观测效果（format 省略位）在此等价于
//! loader 非 Builtin、module 非 java.base-hashed——format = 0，无可见差
//!（本侧 classLoaderName / moduleName / moduleVersion 本就保持 null）。

use crate::prelude::*;
use super::*;
use super::throwable_impl::StackTraceFrames;

impl StackTraceElement {
    /// native initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V：
    /// 按 `fillInStackTrace` 捕获的 Rust 栈帧填充元素字段。载体形态不符
    ///（非本数据面写入，如反序列化路径）时元素保持默认值，不失败。
    #[jvm_native]
    pub fn initStackTraceElements(
        mut stack_trace: JArray<StackTraceElement>,
        backtrace: Object,
        depth: i32,
    ) -> Result<()> {
        let frames = backtrace
            .0
            .as_any()
            .downcast_ref::<Rc<StackTraceFrames>>()
            .cloned();
        for i in 0..depth {
            let mut element = stack_trace.get(i)?;
            if let Some(entry) = frames.as_ref().and_then(|f| f.entries.get(i as usize)) {
                element.__set_declaringClass(String::from(entry.declaring_class.as_str()));
                element.__set_methodName(String::from(entry.method_name.as_str()));
                if let Some(file_name) = &entry.file_name {
                    element.__set_fileName(String::from(file_name.as_str()));
                }
                element.__set_lineNumber(entry.line_number);
            }
            stack_trace.set(i, element)?;
        }
        Ok(())
    }

    /// computeFormat 翻译体的手写覆盖（`__impl_` 机制）：format = 0、清
    /// declaringClassObject，与 Java 体在无 class-loader/module 元数据下的
    /// 可观测行为等价。
    pub fn __impl_computeFormat(&self) -> Result<()> {
        self.__set_format(0);
        self.__set_declaringClassObject(Default::default());
        Ok(())
    }
}
