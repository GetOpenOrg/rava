//! `java.lang.Throwable` 的 native 层 + 栈回溯数据面（S-19 #5，FS-E1）。
//!
//! ## 语义：帧内容与 JVM 一致（类 / 方法 / 源文件 / Java 行号）
//!
//! `fillInStackTrace` 的帧取自栈遍历数据面 `crate::vm_stack::capture_java_frames`（与
//! getCallerClass / getClassContext / StackWalker 同一来源）：真实 Rust 栈的帧位置查发射层行表
//!（方法体语句行尾的 `// line N` 标记，即 class 文件的 LineNumberTable；手写 native 帧行号 -2）。
//! 只在 fillInStackTrace 时查表，运行期无影子栈等额外开销。
//!
//! 隐藏规则同 HotSpot `java_lang_Throwable::fill_in_stack_trace`：先跳过 `fillInStackTrace`
//! 帧，再跳过本异常类及其超类的 `<init>` 帧。帧数上限 1024（`MaxJavaStackTraceDepth` 默认值）。
//!
//! ## 数据面（与 JDK 协议状态对齐）
//!
//! HotSpot 的 native `fillInStackTrace(int)` 把回溯写入 `backtrace` 字段并置
//! `depth`；`getOurStackTrace`（Java 侧）据 `backtrace != null` 经
//! `StackTraceElement.of(backtrace, depth)` 展开为数组。本侧同构：
//! `backtrace` 字段承载 [`StackTraceFrames`]（Rc 负载体，经 `Object::from_any`
//! 擦除存储），`depth` 承载帧数；展开由共置 `stack_trace_element_impl.rs` 的
//! `initStackTraceElements` 消费。

use crate::prelude::*;
use super::*;

/// 帧数截断上限：对齐 JVM `MaxJavaStackTraceDepth` 默认值（1024）。
const MAX_FRAMES: usize = 1024;

/// `fillInStackTrace` 捕获的栈帧载体（`Throwable.backtrace` 字段的负载体）。
/// `Rc` 共享：Throwable 按字段克隆（Clone 语义）时不复制帧数据。
pub(crate) struct StackTraceFrames {
    pub(crate) entries: Vec<StackFrameEntry>,
}

/// 单个 Java 帧。
pub(crate) struct StackFrameEntry {
    /// 声明类 binary name，点分形态（`StackTraceElement.declaringClass`）
    pub(crate) declaring_class: std::string::String,
    /// 方法名（构造器 `<init>`、类初始化 `<clinit>`）
    pub(crate) method_name: std::string::String,
    /// SourceFile 属性；缺失为 None → 呈 "Unknown Source"
    pub(crate) file_name: Option<std::string::String>,
    /// Java 行号；LineNumberTable 缺失为 -1（JDK「不可得」约定），native 方法 -2
    pub(crate) line_number: i32,
}

/// 捕获当前栈并恢复 Java 帧，按 JVM 规则隐藏捕获点与本异常类构造器链。
fn capture_java_frames(exception_class: &str) -> Rc<StackTraceFrames> {
    let mut entries: Vec<StackFrameEntry> = crate::vm_stack::capture_java_frames()
        .into_iter()
        .map(|f| StackFrameEntry {
            declaring_class: f.class.replace('/', "."),
            method_name: f.method.name.to_owned(),
            file_name: f.source.map(str::to_owned),
            line_number: f.line,
        })
        .collect();
    let supers: &[&str] = crate::meta::class_hierarchy()
        .iter()
        .find(|(n, _)| *n == exception_class)
        .map(|(_, s)| *s)
        .unwrap_or(&[]);
    let is_own_init = |e: &StackFrameEntry| {
        e.method_name == "<init>"
            && (e.declaring_class.replace('.', "/") == exception_class
                || supers.iter().any(|s| *s == e.declaring_class.replace('.', "/")))
    };
    let skip_fill = entries.iter().take_while(|e| e.method_name == "fillInStackTrace").count();
    let skip_init = entries[skip_fill..].iter().take_while(|e| is_own_init(e)).count();
    entries.drain(..skip_fill + skip_init);
    entries.truncate(MAX_FRAMES);
    Rc::new(StackTraceFrames { entries })
}

impl Throwable {
    /// native fillInStackTrace(int)：捕获构造点的 Java 栈帧，写 `backtrace`（载体）与
    /// `depth`（帧数）两字段后返回自身（`fillInStackTrace() == this` 的 Java 语义由 wrapper 保持）。
    #[jvm_native]
    pub fn fillInStackTrace_i(&self, _dummy: i32) -> Result<Throwable> {
        let frames = capture_java_frames(Object::from(Clone::clone(self)).0.__class_name());
        self.__set_depth(frames.entries.len() as i32);
        self.__set_backtrace(Object::from_any(frames));
        Ok(Clone::clone(self))
    }
}
