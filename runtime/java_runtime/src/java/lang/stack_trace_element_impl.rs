//! `java.lang.StackTraceElement` 的 native 层（S-19 #5 栈回溯数据面）。
//!
//! `initStackTraceElements`（JDK native）：把 `Throwable.backtrace` 载体展开为
//! 栈帧元素数组——载体即共置 `throwable_impl.rs` 捕获的 [`StackTraceFrames`]
//!（真实 Rust 栈，帧数真实、内容近似——compatibility.md「栈回溯」行的已声明
//! 边界）。数组本身（长度 = depth）由 Java 侧 `StackTraceElement.of` 分配，
//! 本 native 只填字段。
//!
//! 与 HotSpot `java_lang_StackTraceElement::fill_in` 同：一并填 `declaringClassObject`，
//! 供翻译体 `computeFormat` 查询 class-loader / module（`Class.getClassLoader0` /
//! `getModule`：无名模块、不在 boot layer → format 省略位与 JVM 对用户类的判定一致）。

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
            // VM 恒填声明类对象（computeFormat 据此查询；帧数据缺失时以占位名承载）
            let decl = frames.as_ref().and_then(|f| f.entries.get(i as usize))
                .map(|e| e.declaring_class.replace('.', "/"))
                .unwrap_or_else(|| "<unknown>".to_owned());
            element.__set_declaringClassObject(super::Class::for_class(String::from(decl.as_str())));
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
}
