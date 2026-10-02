//! 栈遍历数据面（运行时基础设施）：真实 Rust 栈 → Java 帧序列。
//!
//! HotSpot 的 vframeStream 逐帧给出 (Method*, bci)；原生二进制没有 Java 帧元数据，帧源为
//! std::backtrace 的真实 Rust 栈（compatibility.md「栈回溯」边界，与 fillInStackTrace 同族）：
//! 符号 → 声明类（java_class_of_symbol）→ 方法段对到 java_meta 方法表的 (name, descriptor)。
//! 消费方：Reflection.getCallerClass、SecurityManager.getClassContext、StackWalker
//!（StackStreamFactory$AbstractStackWalker.callStackWalk / fetchStackFrames）。
//!
//! 帧归并：翻译的一个 Java 方法在 Rust 栈上可展开为多帧（派发包装 → `__impl_` 方法体、trait
//! 限定形态）。同一 (类, 方法) 的连续 Rust 帧归为一个 Java 帧，组内同一符号再次出现即新帧
//!（直接递归每层一个符号帧）。闭包帧（lambda 代理调用、方法体内的局部闭包）不是 Java 帧，
//! 其外层方法帧另行出现；未对上方法表的辅助函数帧同样跳过。

use crate::meta::MethodMeta;
use std::collections::HashMap;

/// 一个 Java 帧：声明类（binary name，斜线形态）与方法元数据。
#[derive(Clone)]
pub struct JavaFrame {
    pub class: std::string::String,
    pub method: &'static MethodMeta,
}

impl JavaFrame {
    pub fn is_native(&self) -> bool { self.method.is_native }

    /// HotSpot `Method::is_hidden`：`@jdk.internal.vm.annotation.Hidden` 注解的方法。隐藏类（lambda
    /// 代理等）的方法在本模型是闭包帧，已不成帧。
    pub fn is_hidden(&self) -> bool {
        crate::anno_pool::has_annotation(&self.class, self.method.annotations, "Ljdk/internal/vm/annotation/Hidden;")
    }

    /// HotSpot `Method::caller_sensitive`：`@jdk.internal.reflect.CallerSensitive` 注解的方法。
    pub fn is_caller_sensitive(&self) -> bool {
        crate::anno_pool::has_annotation(&self.class, self.method.annotations, "Ljdk/internal/reflect/CallerSensitive;")
    }

    /// HotSpot `MethodHandles::init_method_MemberName` 对本帧方法给出的 MemberName.flags：
    /// 可识别方法修饰位 | 类别位 | reference kind << 24 | CALLER_SENSITIVE。分派类别同
    /// `CallInfo(Method*)`：可静态绑定（static / private / final 方法 / final 类 / 构造器）→ 直接调用
    ///（static → REF_invokeStatic，其余 → REF_invokeSpecial，构造器另记 IS_CONSTRUCTOR）；声明类是接口 →
    /// REF_invokeInterface；否则 REF_invokeVirtual。
    pub fn member_name_flags(&self) -> i32 {
        const RECOGNIZED_METHOD_MODIFIERS: i32 = 0x1DFF;
        const ACC_PRIVATE: i32 = 0x2;
        const ACC_FINAL: i32 = 0x10;
        const ACC_INTERFACE: i32 = 0x200;
        const MN_IS_METHOD: i32 = 0x0001_0000;
        const MN_IS_CONSTRUCTOR: i32 = 0x0002_0000;
        const MN_CALLER_SENSITIVE: i32 = 0x0010_0000;
        const REF_INVOKE_VIRTUAL: i32 = 5;
        const REF_INVOKE_STATIC: i32 = 6;
        const REF_INVOKE_SPECIAL: i32 = 7;
        const REF_INVOKE_INTERFACE: i32 = 9;
        let mods = self.method.modifiers;
        let class_flags = class_access_flags(&self.class);
        let initializer = self.method.name == "<init>";
        let statically_bound = self.method.is_static || initializer
            || mods & (ACC_PRIVATE | ACC_FINAL) != 0 || class_flags & ACC_FINAL != 0;
        let kind = if statically_bound {
            if self.method.is_static {
                MN_IS_METHOD | (REF_INVOKE_STATIC << 24)
            } else if initializer {
                MN_IS_CONSTRUCTOR | (REF_INVOKE_SPECIAL << 24)
            } else {
                MN_IS_METHOD | (REF_INVOKE_SPECIAL << 24)
            }
        } else if class_flags & ACC_INTERFACE != 0 {
            MN_IS_METHOD | (REF_INVOKE_INTERFACE << 24)
        } else {
            MN_IS_METHOD | (REF_INVOKE_VIRTUAL << 24)
        };
        let cs = if self.is_caller_sensitive() { MN_CALLER_SENSITIVE } else { 0 };
        (mods & RECOGNIZED_METHOD_MODIFIERS) | kind | cs
    }

    /// HotSpot `Method::external_name`：`<返回类型> <类>.<方法>(<形参类型,…>)`（Java 源码形态的类型名）。
    pub fn external_name(&self) -> std::string::String {
        let desc = self.method.descriptor;
        let (params, ret) = desc.strip_prefix('(').and_then(|d| d.split_once(')')).unwrap_or(("", desc));
        let mut names = Vec::new();
        let mut rest = params;
        while !rest.is_empty() {
            let (name, tail) = external_type(rest);
            names.push(name);
            rest = tail;
        }
        format!("{} {}.{}({})", external_type(ret).0, self.class.replace('/', "."), self.method.name, names.join(", "))
    }

    /// 声明类是 `ancestor` 或其（任意深度）子类。
    pub fn class_extends(&self, ancestor: &str) -> bool {
        class_extends(&self.class, ancestor)
    }
}

/// `class` 是 `ancestor` 或其（任意深度）子类（java_meta 直接父类表逐级上溯）。
pub fn class_extends(class: &str, ancestor: &str) -> bool {
    let mut cur = class;
    for _ in 0..64 {
        if cur == ancestor {
            return true;
        }
        match direct_super(cur) {
            Some(s) => cur = s,
            None => return false,
        }
    }
    false
}

/// 类文件 access_flags 原值（java_meta 表，无表项 → 0）。
pub fn class_access_flags(class: &str) -> i32 {
    crate::meta::class_access_flags().iter().find(|(c, _)| *c == class).map_or(0, |(_, f)| *f)
}

/// 描述符首个字段类型 → (Java 源码形态名, 余下描述符)。
fn external_type(desc: &str) -> (std::string::String, &str) {
    let dims = desc.bytes().take_while(|b| *b == b'[').count();
    let body = &desc[dims..];
    let (base, rest): (std::string::String, &str) = match body.as_bytes().first() {
        Some(b'L') => {
            let end = body.find(';').unwrap_or(body.len() - 1);
            (body[1..end].replace('/', "."), &body[end + 1..])
        }
        Some(&c) => {
            let name = match c {
                b'B' => "byte", b'C' => "char", b'D' => "double", b'F' => "float", b'I' => "int",
                b'J' => "long", b'S' => "short", b'Z' => "boolean", _ => "void",
            };
            (name.to_owned(), &body[1..])
        }
        None => (std::string::String::new(), ""),
    };
    (format!("{}{}", base, "[]".repeat(dims)), rest)
}

/// 直接父类（java_meta 表，无表项 → None）。
pub fn direct_super(class: &str) -> Option<&'static str> {
    crate::meta::class_direct_super().iter().find(|(c, _)| *c == class).map(|(_, s)| *s)
}

/// 捕获当前线程的 Java 帧序列（自栈顶向下）。
pub fn capture_java_frames() -> Vec<JavaFrame> {
    let mut frames: Vec<JavaFrame> = Vec::new();
    let mut group: Vec<std::string::String> = Vec::new(); // 当前 Java 帧已归入的 Rust 符号
    for symbol in capture_symbols() {
        let Some(parsed) = parse_symbol(&symbol) else { continue };
        if parsed.closure {
            continue;
        }
        let Some(seg) = parsed.method.as_deref() else { continue };
        let Some(method) = java_method_of(&parsed.class, seg) else { continue };
        let same = frames.last().is_some_and(|f| f.class == parsed.class && std::ptr::eq(f.method, method));
        if same && !group.contains(&symbol) {
            group.push(symbol);
            continue;
        }
        group.clear();
        group.push(symbol);
        frames.push(JavaFrame { class: parsed.class, method });
    }
    frames
}

/// 逐帧的 Java 声明类（解析不出为 None），不做方法对位与归并（getCallerClass 的帧组口径）。
pub fn capture_frame_classes() -> Vec<Option<std::string::String>> {
    capture_symbols().iter().map(|s| java_class_of_symbol(s)).collect()
}

/// std Backtrace Display 的帧符号行（`   N: symbol`；`at file:line` 续行跳过）。
fn capture_symbols() -> Vec<std::string::String> {
    let text = std::format!("{}", std::backtrace::Backtrace::force_capture());
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let Some((idx, symbol)) = trimmed.split_once(": ") else { continue };
        if idx.trim().parse::<u32>().is_err() {
            continue;
        }
        out.push(symbol.trim().to_owned());
    }
    out
}

/// 声明类的方法表（非继承副本），按类名索引（首次使用时建表）。
fn methods_of(class: &str) -> Option<&'static [MethodMeta]> {
    static INDEX: std::sync::OnceLock<HashMap<&'static str, &'static [MethodMeta]>> = std::sync::OnceLock::new();
    INDEX.get_or_init(|| crate::meta::class_methods().iter().map(|(c, m)| (*c, *m)).collect())
        .get(class).copied()
}

/// Rust 方法段 → Java 方法。生成器命名：`<Java 名>`（`$` → `_`）或 `<Java 名>_<重载后缀>`；
/// 构造器为 `new` / `new_<后缀>` 与 `__init*`（在已分配对象上执行构造体），类初始化为
/// `__clinit`；`__impl_` 前缀为派发包装后的方法体。同名重载按描述符后缀对位，对不上取首个。
fn java_method_of(class: &str, seg: &str) -> Option<&'static MethodMeta> {
    let methods = methods_of(class)?;
    let seg = seg.strip_prefix("__impl_").unwrap_or(seg);
    let (java_name, suffix): (&str, &str) = if seg == "__clinit" {
        ("<clinit>", "")
    } else if seg == "new" || seg.starts_with("__init") {
        ("<init>", "")
    } else if let Some(rest) = seg.strip_prefix("new_") {
        ("<init>", rest)
    } else {
        // 最长的 Java 名前缀（Java 名可含 `_`，如 `lambda$main$0` → `lambda_main_0`）
        let mut best: Option<(&'static str, usize)> = None;
        for m in methods.iter().filter(|m| !m.inherited) {
            let mangled = m.name.replace('$', "_");
            let hit = seg == mangled || (seg.starts_with(mangled.as_str()) && seg.as_bytes().get(mangled.len()) == Some(&b'_'));
            if hit && best.map_or(true, |(_, l)| mangled.len() > l) {
                best = Some((m.name, mangled.len()));
            }
        }
        let (name, len) = best?;
        (name, seg.get(len + 1..).unwrap_or(""))
    };
    let mut candidates = methods.iter().filter(|m| !m.inherited && m.name == java_name);
    let first = candidates.next()?;
    if suffix.is_empty() || descriptor_suffix(first.descriptor) == suffix {
        return Some(first);
    }
    candidates.find(|m| descriptor_suffix(m.descriptor) == suffix).or(Some(first))
}

/// 描述符参数 → 重载后缀（基本类型单字母、类取小写简单名、数组加 `arr_`）。生成器另按清单缩写
///（如 String → str），缩写未知时对不上，退回首个同名方法。
fn descriptor_suffix(descriptor: &str) -> std::string::String {
    let params = descriptor.strip_prefix('(').and_then(|d| d.split_once(')')).map(|(p, _)| p).unwrap_or("");
    let prim = |c: u8| -> Option<&'static str> {
        Some(match c { b'I' => "i", b'J' => "l", b'Z' => "z", b'B' => "b", b'S' => "s", b'F' => "f", b'D' => "d", b'C' => "c", _ => return None })
    };
    let class = |body: &str| -> std::string::String {
        let short = body.rsplit('/').next().unwrap_or(body).to_lowercase().replace('$', "_");
        match short.as_str() { "object" => "obj".to_owned(), "string" => "str".to_owned(), _ => short }
    };
    let b = params.as_bytes();
    let mut parts: Vec<std::string::String> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let mut arr = false;
        while i < b.len() && b[i] == b'[' { arr = true; i += 1; }
        let part = match b.get(i) {
            Some(b'L') => {
                let end = params[i..].find(';').map(|p| i + p).unwrap_or(b.len());
                let p = class(&params[i + 1..end]);
                i = end + 1;
                p
            }
            Some(&c) => { i += 1; prim(c).unwrap_or("x").to_owned() }
            None => break,
        };
        parts.push(if arr { format!("arr_{part}") } else { part });
    }
    parts.join("_")
}

/// Rust 符号路径 → Java 声明类 binary name（斜线形态）。
///
/// 翻译方法的符号形态（std Backtrace Display，随 rustc 版本而异）：
/// - 包裹体形态：`<java_runtime::jdk::internal::reflect::reflection::Reflection>::getCallerClass`
///   / `<Type as Trait>::m`（inherent / trait 方法的限定 Self 形态）；
/// - 泛型形态两种：`...::AtomicReference::<Object>::__clinit`（rustc 1.98 实测）与
///   `...::AtomicReference<V>::__clinit`（rustc 1.94 实测，泛型组紧贴类型段）；
/// - 闭包尾巴：`...::{closure#0}`。
/// 解析规则：
/// - 跨模块 impl 组 `<impl T>` / `<impl Tr for T>` 取实现类型（impl_self_type），
///   限定 Self 包裹体取 Self 类型（unwrap_qualified_self），再删除全部成对
///   尖括号泛型组（strip_generic_groups）——与泛型打印形态无关。旧实现取
///   「首个 `<` 到首个 `>`」，在 1.94 形态下取到泛型形参 `V`，泛型类帧全部
///   解析失败 → getCallerClass 越过调用者返回 null → MethodHandles.lookup
///   抛 IllegalCallerException（TestAtomics 服务器侧 EIIE 根因）；
/// - 从右向左跳过方法/函数段（小写或下划线开头、空段），首个大写开头段 = 类型段；
/// - 类型段的 `_` 是内部类 `$` 分隔（Java 类名不含下划线，宏对嵌套类即此命名）；
/// - 包段末段的「类文件 stem」（snake(类简单名)，如 atomic_reference / reflection /
///   method_handles）不是 Java 包——codegen 每类一文件，类型自身模块名恰是 stem，
///   归一化比较（去 `_`/`$` 后小写相等）命中即剥掉；不命中（非 java_runtime 形态）保留；
/// - 包段的单词式关键字转义（尾随一个 `_`，如 `unsafe_`）剥掉下划线。
/// - 用户 crate 帧（`<bin>::<包段…>::<stem>::Class::method`）同一规则解析（首段为 crate 根）。
/// 解析不出（标准库 / 依赖帧、形态不符）返回 None。
pub(crate) fn java_class_of_symbol(symbol: &str) -> Option<std::string::String> {
    parse_symbol(symbol).map(|p| p.class)
}

/// 符号解析结果：声明类 + 类型段之后的首个方法段 + 是否闭包帧。
struct ParsedSymbol {
    class: std::string::String,
    method: Option<std::string::String>,
    closure: bool,
}

fn parse_symbol(symbol: &str) -> Option<ParsedSymbol> {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];
    let path = strip_generic_groups(&unwrap_qualified_self(&impl_self_type(symbol)));
    // 翻译类所在 crate：JDK 类在 java_runtime，用户类在用户 crate（bin 名为 crate 根，
    // 模块路径 = 包段 + 类文件 stem，与 java_runtime 同一布局）。Rust 标准库与第三方
    // 依赖帧不是 Java 帧。
    const NON_JAVA_CRATES: &[&str] = &[
        "std", "core", "alloc", "parking_lot", "parking_lot_core", "lock_api", "backtrace",
        "rustc_demangle", "gimli", "addr2line", "rava_macros",
    ];
    let in_runtime = path.contains("java_runtime::");
    let start = match path.find("java_runtime::") {
        Some(s) => s,
        None => {
            let first = path.split("::").next()?;
            if first.is_empty() || !first.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                || NON_JAVA_CRATES.contains(&first) {
                return None;
            }
            0
        }
    };
    let segs: Vec<&str> = path[start..].split("::").filter(|s| !s.is_empty()).collect();
    // 从右向左找类型段（首个大写开头段；小写/下划线开头为方法或辅助函数段）
    let mut idx = segs.len();
    while idx > 0 {
        let seg = segs[idx - 1];
        match seg.chars().next() {
            Some(c) if c.is_uppercase() => break,
            _ => idx -= 1,
        }
    }
    if idx < 2 {
        return None; // 至少「crate 根 + 类型段」
    }
    // 方法段从原符号取：包裹体 / impl 组解析只保留 Self 类型，方法段在组外
    let tail = strip_generic_groups(symbol);
    let tail_segs: Vec<&str> = tail.split("::").filter(|s| !s.is_empty()).collect();
    let closure = tail_segs.iter().any(|s| s.starts_with("{closure"));
    let method = tail_segs.iter().rev().find(|s| !s.starts_with('{')).map(|s| (*s).to_owned());
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
        // 无名包：只对用户 crate 成立（JDK 类恒在具名包）
        return if in_runtime { None } else { Some(ParsedSymbol { class: type_name, method, closure }) };
    }
    Some(ParsedSymbol { class: format!("{}/{}", pkg.join("/"), type_name), method, closure })
}

/// 跨模块 impl 形态 `mod::<impl Type<T>>::m` / `mod::<impl Trait for Type>::m`
/// （`_impl.rs` 伴生方法与宏展开的 trait impl 即此形态，rustc 1.94 实测）→
/// 实现类型的绝对路径 `Type<T>`：取深度 0 处 `<impl ` 组的内部，trait impl
/// 取深度 1 的 ` for ` 之后。非此形态原样返回。
fn impl_self_type(symbol: &str) -> std::string::String {
    let Some(open) = symbol.find("<impl ") else { return symbol.to_owned() };
    // 须是深度 0 的组（泛型实参内部的 `<impl` 不是 Self 路径）
    if symbol[..open].matches('<').count() != symbol[..open].matches('>').count() {
        return symbol.to_owned();
    }
    let body = &symbol[open + 1..];
    let mut depth = 1usize;
    let mut end = body.len();
    let mut for_at: Option<usize> = None;
    let bytes = body.as_bytes();
    for (i, c) in body.char_indices() {
        match c {
            '<' => depth += 1,
            '>' if i > 0 && bytes[i - 1] == b'-' => {}
            '>' => {
                depth -= 1;
                if depth == 0 { end = i; break; }
            }
            _ if depth == 1 && for_at.is_none() && body[i..].starts_with(" for ") => for_at = Some(i + 5),
            _ => {}
        }
    }
    let inner = &body[..end];
    match for_at {
        Some(k) if k <= end => inner[k..].to_owned(),
        _ => inner.trim_start_matches("impl ").to_owned(),
    }
}

/// 限定 Self 包裹体 `<Type as Trait>::m` / `<Type>::m` → `Type`（成对尖括号
/// 深度计数定位包裹体终点；` as ` 只在深度 1 处切分，泛型实参内的 ` as ` 不误切）。
/// 非包裹形态原样返回。
fn unwrap_qualified_self(symbol: &str) -> std::string::String {
    if !symbol.starts_with('<') {
        return symbol.to_owned();
    }
    let mut depth = 0usize;
    let mut out = std::string::String::new();
    let bytes = symbol.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '<' => { depth += 1; if depth > 1 { out.push(c); } }
            // 函数指针类型的 `->` 不是尖括号
            '>' if i > 0 && bytes[i - 1] == b'-' => out.push(c),
            '>' => {
                depth -= 1;
                if depth == 0 { break; }
                out.push(c);
            }
            _ if depth == 1 && symbol[i..].starts_with(" as ") => break,
            _ => out.push(c),
        }
        i += 1;
    }
    out
}

/// 删除全部成对尖括号泛型组（含嵌套）：兼容 std Backtrace 的两种泛型打印——
/// `Type<T>::m`（rustc 1.94 实测）与 `Type::<T>::m`（1.98 实测）；删后遗留的
/// 空 `::` 段由调用方过滤。
fn strip_generic_groups(path: &str) -> std::string::String {
    let mut depth = 0usize;
    let mut out = std::string::String::new();
    let mut prev = '\0';
    for c in path.chars() {
        match c {
            '<' => depth += 1,
            '>' if prev == '-' => { if depth == 0 { out.push(c); } }
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
        prev = c;
    }
    out
}
