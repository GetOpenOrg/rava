//! `jdk/internal/reflect/Reflection` 手写伴生：内部边界类，按调用链按需实现
//! （K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use super::reflection::Reflection;
use crate::java::lang::Class;

/// Rust 符号路径 → Java 声明类 binary name（斜线形态）。
///
/// 翻译方法的符号形态（std Backtrace Display）：
/// `<java_runtime::java::util::concurrent::atomic::atomic_reference::AtomicReference>::__clinit`
/// —— `java_runtime::` 前缀后是包的 snake 模块段、类型段、方法段（可能带
/// `::<泛参>` / `::{closure}` 尾巴）。规则：
/// - 泛参与闭包尾巴（首个 `<` 起）截断；
/// - 从右向左跳过方法/函数段（小写或下划线开头、空段），首个大写开头段 = 类型段；
/// - 类型段的 `_` 是内部类 `$` 分隔（Java 类名不含下划线，宏对嵌套类即此命名）；
/// - 包段的单词式关键字转义（尾随一个 `_`，如 `unsafe_`）剥掉下划线。
/// 解析不出（非 java_runtime 帧 / 形态不符）返回 None。
fn _java_class_of_symbol(symbol: &str) -> Option<std::string::String> {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];
    let s = symbol.trim_start_matches('<').trim_end_matches('>');
    let start = s.find("java_runtime::")?;
    let path = s[start..].split('<').next()?;
    let segs: Vec<&str> = path.split("::").collect();
    // 从右向左找类型段（首个大写开头段；小写/下划线开头为方法或辅助函数段）
    let mut idx = segs.len();
    while idx > 0 {
        let seg = segs[idx - 1];
        match seg.chars().next() {
            Some(c) if c.is_uppercase() => break,
            _ => idx -= 1,
        }
    }
    if idx == 0 {
        return None;
    }
    let type_name = segs[idx - 1].replace('_', "$");
    let mut pkg: Vec<std::string::String> = Vec::new();
    for seg in &segs[1..idx - 1] {
        if seg.is_empty() {
            return None;
        }
        // 关键字转义模块（尾随 _）还原
        let stripped = seg.strip_suffix('_').unwrap_or(seg);
        if stripped.len() + 1 == seg.len() && KEYWORDS.contains(&stripped) {
            pkg.push(stripped.to_owned());
        } else {
            pkg.push((*seg).to_owned());
        }
    }
    if pkg.is_empty() {
        return None;
    }
    Some(format!("{}/{}", pkg.join("/"), type_name))
}

/// 捕获当前 Rust 栈的符号帧序列（std Backtrace Display 行 `   N: symbol`；
/// `at file:line` 续行与序号列无关，跳过）。返回逐帧的 Java 声明类（解析不出
/// 为 None）。
fn _capture_frame_classes() -> Vec<Option<std::string::String>> {
    let text = std::format!("{}", std::backtrace::Backtrace::force_capture());
    let mut classes: Vec<Option<std::string::String>> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let Some((idx, symbol)) = trimmed.split_once(": ") else { continue };
        if idx.trim().parse::<u32>().is_err() {
            continue; // `at file:line` 续行
        }
        classes.push(_java_class_of_symbol(symbol.trim()));
    }
    classes
}

impl Reflection {
    /// native `getCallerClass()`：`@CallerSensitive`——返回「调用 getCallerClass
    /// 的方法」的调用者声明类（JDK javadoc：ignoring frames associated with
    /// java.lang.reflect.Method.invoke）。
    ///
    /// 帧源为 std::backtrace 的真实 Rust 栈（栈回溯数据面，compatibility.md
    /// 「栈回溯」近似等价边界——与 throwable_impl 的 fillInStackTrace 同族；
    /// 逐字节的 Java 帧元数据在原生二进制不存在）。跳帧规则：
    ///   1. 定位本方法帧（声明类 jdk.internal.reflect.Reflection）；
    ///   2. 其后第一帧组 = `@CallerSensitive` 声明者（如 MethodHandles.lookup）
    ///      ——同一 Java 方法的多个 Rust 帧（含解析不出类的辅助帧）按类名
    ///      聚合，整组跳过；
    ///   3. 首个声明类不同的帧即调用者的调用者，返回其 Class。
    /// 无可用帧（main 之下 / 形态不符）返回 null Class——调用方按 JDK 语义
    /// 处理（MethodHandles.lookup 抛 IllegalCallerException("no caller frame")）。
    #[jvm_native]
    pub fn getCallerClass() -> Result<Class> {
        const SELF_CLASS: &str = "jdk/internal/reflect/Reflection";
        let frames = _capture_frame_classes();
        // 1. 定位本方法帧
        let Some(mut i) = frames.iter().position(|c| c.as_deref() == Some(SELF_CLASS)) else {
            return Ok(Class::default());
        };
        // 2. 调用者帧组的类（@CallerSensitive 声明者）
        i += 1;
        let Some(caller) = frames.get(i).and_then(|c| c.clone()) else {
            return Ok(Class::default());
        };
        // 3. 跳过同类（与解析失败跟随前帧）的帧，取首个异类帧
        i += 1;
        while i < frames.len() {
            match &frames[i] {
                Some(c) if c != &caller => {
                    return Ok(Class::for_class(String::from(c.as_str())));
                }
                _ => i += 1,
            }
        }
        Ok(Class::default())
    }
}
