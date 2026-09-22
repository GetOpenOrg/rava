//! `jdk/internal/reflect/Reflection` 手写伴生：内部边界类，按调用链按需实现
//! （K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use super::reflection::Reflection;
use crate::java::lang::Class;
use crate::java::util::Set;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// 字段过滤登记表：声明类 binary name（斜线形态）→ 对反射字段枚举隐藏
    /// 的字段名集合。`registerFieldsToFilter` 写入；`Class.getDeclaredFields`
    /// （复数形态，class_impl.rs）消费——JDK 的 fieldFilterMap 协议：隐藏类
    /// 实现细节字段（如 MethodHandles$Lookup 的 lookupClass / allowedModes），
    /// 对单字段查询（getDeclaredField）不生效。运行时对象为 Rc 单线程形态，
    /// 登记表随之线程内（与 unsafe__impl 的字段偏移登记表同族）。
    static FIELD_FILTERS: RefCell<HashMap<std::string::String, Vec<std::string::String>>> =
        RefCell::new(HashMap::new());
}

/// 反射族内部：binary name 的登记过滤字段名集（未登记 → None）。消费方：
/// Class.getDeclaredFields（复数形态，按名过滤）。
pub(crate) fn __field_filter(binary_name: &str) -> Option<Vec<std::string::String>> {
    FIELD_FILTERS.with(|t| t.borrow().get(binary_name).cloned())
}

/// Rust 符号路径 → Java 声明类 binary name（斜线形态）。
///
/// 翻译方法的符号形态（std Backtrace Display，TestAtomics scratch 实测）：
/// - 包裹体形态：`<java_runtime::jdk::internal::reflect::reflection::Reflection>::getCallerClass`
///   （inherent 方法按类型的规范化路径符号化，`_impl.rs` 伴生方法亦然）；
/// - 泛型尾巴形态：`...::AtomicReference::<Object>::__clinit` / `...::{closure#0}`。
/// 解析规则：
/// - 取首个 `<` 到其后首个 `>` 的内部为路径体（泛型实参尾巴一并截断），
///   无 `<` 按裸路径；旧的「剥首 `<` 剥尾 `>`」会把 `>` 留在类型段里
///   （`Reflection>`）导致永不命中——本批修正；
/// - 从右向左跳过方法/函数段（小写或下划线开头、空段），首个大写开头段 = 类型段；
/// - 类型段的 `_` 是内部类 `$` 分隔（Java 类名不含下划线，宏对嵌套类即此命名）；
/// - 包段末段的「类文件 stem」（snake(类简单名)，如 atomic_reference / reflection /
///   method_handles）不是 Java 包——codegen 每类一文件，类型自身模块名恰是 stem，
///   归一化比较（去 `_`/`$` 后小写相等）命中即剥掉；不命中（非 java_runtime 形态）保留；
/// - 包段的单词式关键字转义（尾随一个 `_`，如 `unsafe_`）剥掉下划线。
/// 解析不出（非 java_runtime 帧 / 形态不符）返回 None。
fn _java_class_of_symbol(symbol: &str) -> Option<std::string::String> {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];
    // 路径体：`<...>` 包裹体取内部（首个 `>` 前截断，泛型实参一并丢弃），
    // 其后的 `::method` 段对类解析无意义；无包裹体按裸路径。
    let path: &str = if let Some(open) = symbol.find('<') {
        let rest = &symbol[open + 1..];
        let end = rest.find('>').unwrap_or(rest.len());
        &rest[..end]
    } else {
        symbol
    };
    let start = path.find("java_runtime::")?;
    let path = path[start..].split('<').next()?;
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
    // 末段若是类型自身文件的 stem（snake(类简单名)）则剥掉——它不是 Java 包段
    if let Some(last) = pkg.last() {
        let norm = |s: &str| -> std::string::String {
            s.chars().filter(|c| *c != '_' && *c != '$').flat_map(|c| c.to_lowercase()).collect()
        };
        if norm(last) == norm(&type_name) {
            pkg.pop();
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

    /// static synchronized `registerFieldsToFilter(Class, Set)`：登记对
    /// `getDeclaredFields`（复数形态）隐藏的字段名（JDK 用途：内部实现字段
    /// 不进反射枚举）。参数形态取擦除 Object（调用点 codegen 的泛型擦除
    /// 传参形态），内部还原 Set；元素按 String 收敛（协议消费方全部传
    /// String 名单），非 String 元素忽略（JDK 泛型签名即 Set<String>）。
    /// 重复登记取后者（JDK newMap.put 语义）。null 类按 JDK 抛 NPE。
    pub fn registerFieldsToFilter(containingClass: Class, fieldNames: Object) -> Result<()> {
        if containingClass.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let key = format!("{}", containingClass.__get_name()).replace('.', "/");
        let set: Set<Object> = Set::<Object>::from(Clone::clone(&fieldNames));
        let mut names: Vec<std::string::String> = Vec::new();
        let mut it = set.iterator()?;
        while it.hasNext()? {
            let e = it.next()?;
            if let Ok(s) = Object::from(e).try_cast::<crate::java::lang::String>("java/lang/String") {
                names.push(format!("{}", s));
            }
        }
        FIELD_FILTERS.with(|t| {
            t.borrow_mut().insert(key, names);
        });
        Ok(())
    }
}
