//! `java.lang.Throwable` 的 native 层 + 栈回溯数据面（S-19 #5，FS-E1）。
//!
//! ## 语义：帧内容与 JVM 一致（类 / 方法 / 源文件 / Java 行号）
//!
//! `fillInStackTrace` 经 `std::backtrace::Backtrace` 捕获构造点的 Rust 栈，每帧带
//! `at <生成文件>:<行>`（`java_class!` 宏保留方法体 token 的原始位置）。发射层为每个生成文件
//! 写出行表（`meta::line_tables()`：Rust 行 → (类, 方法, 源文件, Java 行)，来源为方法体语句行尾
//! 的 `// line N` 标记，即 class 文件的 LineNumberTable）。本侧据帧的文件 + 行查表恢复 Java 帧；
//! 查不到的帧（运行时基础设施、std、宏生成的分派包装、方法体首个标记之前的序言）不是 Java 帧，
//! 剔除。只在 fillInStackTrace 时查表，运行期无影子栈等额外开销。
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

use std::collections::HashMap;

use crate::prelude::*;
use super::*;

/// 帧数截断上限：对齐 JVM `MaxJavaStackTraceDepth` 默认值（1024）。
const MAX_FRAMES: usize = 1024;

/// 行表中「块外 / 非 Java 方法」的方法下标（发射层 `line_tables.rs` 同值）
const NO_METHOD: u32 = u32::MAX;

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
    /// Java 行号；LineNumberTable 缺失为 -1（JDK「不可得」约定）
    pub(crate) line_number: i32,
}

/// 行表索引：scratch 相对路径 → 行表下标（首次查表时建立，线程内复用）
fn table_index() -> &'static HashMap<&'static str, usize> {
    static INDEX: std::sync::OnceLock<HashMap<&'static str, usize>> = std::sync::OnceLock::new();
    INDEX.get_or_init(|| crate::meta::line_tables().iter().enumerate().map(|(i, t)| (t.0, i)).collect())
}

/// 帧位置 → Java 帧。`file` 为回溯 `at` 行的路径（绝对、相对工作区或 `./` 前缀均可）：
/// 依次取其各 `/` 边界后缀查行表；行取「Rust 行不大于该行」的最后一行表项。
fn java_frame(file: &str, line: u32) -> Option<StackFrameEntry> {
    let index = table_index();
    let file = file.replace('\\', "/");
    let mut rest = file.as_str();
    let table = loop {
        if let Some(&i) = index.get(rest) {
            break &crate::meta::line_tables()[i];
        }
        rest = &rest[rest.find('/')? + 1..];
    };
    let (_, methods, rows) = *table;
    let at = rows.partition_point(|r| r.0 <= line).checked_sub(1)?;
    let (_, method, java_line) = rows[at];
    if method == NO_METHOD || java_line == 0 {
        return None;
    }
    let (class, name, source) = methods[method as usize];
    Some(StackFrameEntry {
        declaring_class: class.replace('/', "."),
        method_name: name.to_owned(),
        file_name: (!source.is_empty()).then(|| source.to_owned()),
        line_number: java_line as i32,
    })
}

/// 回溯 Display 解析为 (符号, 文件, 行)：`   {index}: {symbol}` 行，可随
/// `             at {file}:{line}:{col}` 行（缺省 = 未解析位置）。
fn rust_frames(text: &str) -> Vec<(&str, Option<(&str, u32)>)> {
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let Some((index, symbol)) = line.trim_start().split_once(": ") else { continue };
        if index.is_empty() || !index.bytes().all(|b| b.is_ascii_digit()) {
            continue; // "note:" 等非帧行
        }
        let mut at = None;
        if let Some(location) = lines.peek().and_then(|n| n.trim_start().strip_prefix("at ")) {
            lines.next();
            // `path/file.rs:{line}:{col}`：自右取列、行，余下为文件路径
            let parts: Vec<&str> = location.rsplitn(3, ':').collect();
            if let [_, ln, file] = parts[..] {
                at = ln.parse::<u32>().ok().map(|ln| (file, ln));
            }
        }
        out.push((symbol, at));
    }
    out
}

/// 捕获当前栈并恢复 Java 帧，按 JVM 规则隐藏捕获点与本异常类构造器链。
fn capture_java_frames(exception_class: &str) -> Rc<StackTraceFrames> {
    let text = std::format!("{}", std::backtrace::Backtrace::force_capture());
    let mut entries: Vec<StackFrameEntry> = Vec::new();
    let mut closure_of: Option<(std::string::String, std::string::String)> = None;
    for (symbol, at) in rust_frames(&text) {
        let Some(frame) = at.and_then(|(file, line)| java_frame(file, line)) else { continue };
        // 方法体内的 Rust 闭包（异常处理区段等）与其外层函数映射到同一 Java 方法：
        // 闭包帧行号准确，紧随的外层帧是同一 Java 帧，略去
        let key = (frame.declaring_class.clone(), frame.method_name.clone());
        if closure_of.take().is_some_and(|k| k == key) {
            continue;
        }
        if symbol.contains("{{closure}}") {
            closure_of = Some(key);
        }
        entries.push(frame);
    }
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
        let frames = capture_java_frames(ObjectVTable::__class_name(self));
        self.__set_depth(frames.entries.len() as i32);
        self.__set_backtrace(Object::from_any(frames));
        Ok(Clone::clone(self))
    }
}
