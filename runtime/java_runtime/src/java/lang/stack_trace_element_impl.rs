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

    /// native `initStackTraceElement(StackTraceElement, StackFrameInfo)`：HotSpot
    /// `java_lang_StackFrameInfo::to_stack_trace_element` → `java_lang_StackTraceElement::fill_in`。
    /// 方法取自 StackFrameInfo.memberName（栈遍历数据面写入的 clazz / name / flags）：填
    /// declaringClassObject、declaringClass、methodName；fileName = 声明类的 SourceFile 属性（java_meta
    /// 表，无该属性为 null）；lineNumber：native 方法 -2，否则按 memberName 的 (类, 名, type) 取行表中的
    /// LineNumberTable，由 StackFrameInfo.bci 定行（`Method::line_number_from_bci`，无表 -1）。模块 / 类加载器名与 initStackTraceElements 同口径不填。
    /// 形参以 `Into<Object>` 接收：StackFrameInfo 只在 StackFrameTraverser 路径进入生成范围，本文件不以
    /// 类型名引用它。
    #[jvm_native]
    pub fn initStackTraceElement<S: Into<Object>>(mut element: StackTraceElement, info: S) -> Result<()> {
        const ACC_NATIVE: i32 = 0x100;
        let info: Object = info.into();
        if info.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let member = info.0.__unsafe_ref_get("memberName")
            .unwrap_or_else(|| panic!("stub: java/lang/StackFrameInfo.memberName 无按名协议"));
        let field = |name: &str| member.0.__unsafe_ref_get(name)
            .unwrap_or_else(|| panic!("stub: java/lang/invoke/MemberName.{} 无按名协议", name));
        let clazz = field("clazz").try_cast::<Class>("java/lang/Class")?;
        let method_name = field("name").try_cast::<String>("java/lang/String")?;
        let flags = member.0.__unsafe_int_cell("flags").map_or(0, |c| c.get());
        let binary = format!("{}", clazz.__get_name());
        let slash = binary.replace('.', "/");
        element.__set_declaringClassObject(Clone::clone(&clazz));
        element.__set_declaringClass(String::from(binary.as_str()));
        let name = format!("{}", method_name);
        element.__set_methodName(method_name);
        if let Some((_, file)) = crate::meta::class_source_file().iter().find(|(c, _)| *c == slash) {
            element.__set_fileName(String::from(*file));
        }
        let line = if flags & ACC_NATIVE != 0 {
            -2
        } else {
            // type 由数据面写为描述符串；Java 侧可能已把它展开为 MethodType，此时按 (类, 名) 唯一对位
            let descriptor = field("type_").try_cast::<String>("java/lang/String").ok().map(|d| format!("{}", d));
            let bci = info.0.__unsafe_int_cell("bci").map_or(0, |c| c.get());
            crate::vm_stack::line_number_from_bci(&slash, &name, descriptor.as_deref(), bci)
        };
        element.__set_lineNumber(line);
        Ok(())
    }
}
