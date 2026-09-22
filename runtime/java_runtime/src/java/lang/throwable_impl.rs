//! `java.lang.Throwable` 的 native 层 + 栈回溯数据面（S-19 #5）。
//!
//! ## 语义档位：帧数真实、内容近似（compatibility.md「栈回溯」行）
//!
//! `fillInStackTrace` 经 `std::backtrace::Backtrace`（Rust 1.65+ 标准库，无新依赖）
//! 捕获构造点的**真实 Rust 栈**：帧数（depth）为捕获到的真实帧数（截断上限 1024，
//! 对齐 JVM `MaxJavaStackTraceDepth` 默认值），`getStackTrace().length` 语义成立；
//! 帧内容（className/methodName/fileName/lineNumber）取自 Rust 栈符号，与 Java
//! 帧不一致——属已声明的近似等价边界。
//!
//! ## 数据面（与 JDK 协议状态对齐）
//!
//! HotSpot 的 native `fillInStackTrace(int)` 把回溯写入 `backtrace` 字段并置
//! `depth`；`getOurStackTrace`（Java 侧）据 `backtrace != null` 经
//! `StackTraceElement.of(backtrace, depth)` 展开为数组。本侧同构：
//! `backtrace` 字段承载 [`StackTraceFrames`]（Rc 负载体，经 `Object::from_any`
//! 擦除存储），`depth` 承载帧数；展开由共置 `stack_trace_element_impl.rs` 的
//! `initStackTraceElements` 消费。捕获时机 = Throwable 构造（JVM 语义：构造器
//! 调 `fillInStackTrace()`，显式补充调用重捕获）。

use crate::prelude::*;
use super::*;

/// 帧数截断上限：对齐 JVM `MaxJavaStackTraceDepth` 默认值（1024）。
/// 防深 Rust 栈（转译代码调用链深）无界展开。
const MAX_FRAMES: usize = 1024;

/// `fillInStackTrace` 捕获的栈帧载体（`Throwable.backtrace` 字段的负载体）。
/// `Rc` 共享：Throwable 按字段克隆（Clone 语义）时不复制帧数据。
pub(crate) struct StackTraceFrames {
    pub(crate) entries: Vec<StackFrameEntry>,
}

/// 单帧：Rust 栈符号拆解结果（内容为近似等价边界，数量真实）。
pub(crate) struct StackFrameEntry {
    /// Rust 符号路径（`::` 已归一为 `.`），如 `java_runtime.java.lang.Throwable`
    pub(crate) declaring_class: std::string::String,
    /// Rust 符号末段（方法名），无 `::` 分隔时为完整符号
    pub(crate) method_name: std::string::String,
    /// `at` 行的文件路径；缺 `at` 行（如未解析符号）为 None → 呈 "Unknown Source"
    pub(crate) file_name: Option<std::string::String>,
    /// `at` 行的行号；缺为 -1（JDK「不可得」约定）
    pub(crate) line_number: i32,
}

/// 捕获内部管线帧判定。仅修剪**连续前缀**（吃到首个非管线帧为止），对齐 JVM
/// 可见栈回溯的隐藏规则（HotSpot：`fillInStackTrace` 与异常构造器链不进入
/// Java 可见栈，首帧 = 构造点所在方法）：
///   - std 捕获机制自身：`std::backtrace::` 与内部实现路径 `std::backtrace_rs`
///    （Display 的实测拼写，含 `<std::backtrace::Backtrace>::create`）；
///   - 本文件的捕获函数与 `fillInStackTrace` 调用链（native 体 / `__impl_` 体 /
///     vtable 分派 / wrapper，帧名均含 `fillInStackTrace`）；
///   - Throwable 构造器链的翻译形态（`new` / `new_*` / `__init_on*` 包装）——
///     JVM 对 `Throwable` 子类构造器帧同样隐藏。
/// 前缀语义使过度匹配无害：匹配只发生在捕获点正下方的连续窗口内；全管线栈
/// 退化为保留最后 1 帧（见调用点 guard）。
fn is_capture_plumbing(symbol: &str) -> bool {
    symbol.contains("std::backtrace")
        || symbol.contains("capture_rust_frames")
        || symbol.contains("fillInStackTrace")
        || symbol.contains("::new")
        || symbol.contains("::__init_")
}

/// 捕获并解析当前 Rust 栈。解析基于 `Backtrace` 的 Display 输出（std 稳定面
/// 上帧信息的唯一可得形态），格式漂移时退化为单帧占位（长度语义不破）。
fn capture_rust_frames() -> Rc<StackTraceFrames> {
    let text = std::format!("{}", std::backtrace::Backtrace::force_capture());

    // Display 形态：`   {index}: {symbol}` 行，可随 `             at {file}:{line}:{col}` 行
    let mut symbols: Vec<(std::string::String, Option<std::string::String>, i32)> = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        let Some((index, symbol)) = trimmed.split_once(": ") else { continue };
        if index.is_empty() || !index.bytes().all(|b| b.is_ascii_digit()) {
            continue; // "note:" 等非帧行
        }
        // 紧随的 "at" 行（可缺省；多行内联 at 只取首行）
        let mut file_name: Option<std::string::String> = None;
        let mut line_number = -1i32;
        if let Some(next) = lines.peek() {
            let next = next.trim_start();
            if let Some(location) = next.strip_prefix("at ") {
                lines.next();
                // `path/file.rs:{line}:{col}`：自右取列、行，余下为文件路径
                let parts: Vec<&str> = location.rsplitn(3, ':').collect();
                if parts.len() == 3 {
                    if let (Ok(col), Ok(ln)) = (parts[0].parse::<u32>(), parts[1].parse::<i32>()) {
                        let _ = col;
                        line_number = ln;
                        file_name = Some(parts[2].to_owned());
                    }
                }
            }
        }
        symbols.push((symbol.to_owned(), file_name, line_number));
        if symbols.len() >= MAX_FRAMES {
            break;
        }
    }

    // 修剪捕获管线前缀帧（保留至少 1 帧：格式漂移 / 全管线栈的退化保护）
    let start = symbols
        .iter()
        .position(|(sym, _, _)| !is_capture_plumbing(sym))
        .unwrap_or(symbols.len().saturating_sub(1))
        .min(symbols.len().saturating_sub(1));
    let entries = if symbols.is_empty() {
        Vec::new()
    } else {
        symbols[start..]
            .iter()
            .map(|(symbol, file_name, line_number)| {
                let (declaring_class, method_name) = match symbol.rsplit_once("::") {
                    Some((path, method)) => (path.replace("::", "."), method.to_owned()),
                    None => ("<rust>".to_owned(), symbol.clone()),
                };
                StackFrameEntry {
                    declaring_class,
                    method_name,
                    file_name: file_name.clone(),
                    line_number: *line_number,
                }
            })
            .collect()
    };
    // 退化保护：解析全空（Display 格式漂移）时单帧占位，length > 0 语义不破
    let entries = if entries.is_empty() {
        vec![StackFrameEntry {
            declaring_class: "<rust>".to_owned(),
            method_name: "<unresolved>".to_owned(),
            file_name: None,
            line_number: -1,
        }]
    } else {
        entries
    };

    Rc::new(StackTraceFrames { entries })
}

impl Throwable {
    /// native fillInStackTrace(int)：捕获构造点的真实 Rust 栈（帧数真实非零，
    /// 内容为 Rust 栈形态——近似等价边界）。写 `backtrace`（载体）与 `depth`
    ///（帧数）两字段后返回自身（`fillInStackTrace() == this` 的 Java 语义由
    /// wrapper 保持）。
    #[jvm_native]
    pub fn fillInStackTrace_i(&self, _dummy: i32) -> Result<Throwable> {
        let frames = capture_rust_frames();
        self.__set_depth(frames.entries.len() as i32);
        self.__set_backtrace(Object::from_any(frames));
        Ok(Clone::clone(self))
    }
}
