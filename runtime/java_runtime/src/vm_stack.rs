//! 栈遍历数据面（运行时基础设施）：真实 Rust 栈 → Java 帧序列，全部栈帧消费方的唯一来源。
//!
//! HotSpot 的 vframeStream 逐帧给出 (Method*, bci)；原生二进制的等价物是链接期预解析的地址表
//!（[`crate::pc_map`]）：返回地址 → 该物理帧内联链上的 Java 帧（帧归属类, 方法元数据, Java 行）。
//! 帧方法的修饰位与注解随地址表发射（[`FrameMethod`]），不读成员表。
//! 消费方：Throwable.fillInStackTrace、Reflection.getCallerClass、SecurityManager.getClassContext、
//! StackWalker（StackStreamFactory$AbstractStackWalker.callStackWalk / fetchStackFrames）。
//!
//! 成帧规则（链接器包装 `rava-link` 建表时执行，判据为发射层行表，不解析符号形态）：
//! - 帧归属方法体的声明类（HotSpot 帧的 method holder）：复制进本类的接口 default / 超类虚方法体
//!   归 `declared_by`；继承转发外壳无行标记，不成帧；
//! - 宏生成的派发入口、vtable impl、`new` 分配外壳以调用点 span 落在块外或方法序言（Java 行 0），
//!   不成帧——帧落在实际执行方法体的 Rust 帧上，直接递归每层一帧；
//! - 手写方法体在伴生 `_impl.rs`，按生成文件的登记成帧（native 行号 -2，其余 -1）；
//! - 闭包帧（DWARF 函数名含 `{closure`）不成帧：原位闭包（`__caller_sensitive`、`try_new_with`）与外层
//!   方法同一行，外层帧即该 Java 帧；延迟闭包（lambda 代理）的位置是创建点，不在执行栈上。

/// StackWalker 锚定的一条帧流：快照与下一个待检视帧的下标（存于执行上下文块，`exec_context::ExecState`）。
pub(crate) struct AnchoredWalk {
    pub(crate) frames: Vec<JavaFrame>,
    pub(crate) cursor: usize,
}

/// 帧方法的元数据（行表方法项解码）：HotSpot 帧 Method* 上栈遍历消费方读取的部分
#[derive(Clone, Copy)]
pub struct FrameMethod {
    pub name: &'static str,
    pub descriptor: &'static str,
    /// java.lang.reflect.Modifier 位集
    pub modifiers: i32,
    pub is_static: bool,
    pub is_native: bool,
    /// RuntimeVisibleAnnotations 原始属性体
    pub annotations: &'static [u8],
}

/// 一个 Java 帧：方法持有类（binary name，斜线形态）、方法元数据与源位置。
#[derive(Clone)]
pub struct JavaFrame {
    pub class: &'static str,
    pub method: FrameMethod,
    /// SourceFile 属性；缺失为 None
    pub source: Option<&'static str>,
    /// Java 行号（LineNumberTable）；不可得 -1，native 方法 -2（`StackTraceElement` 约定）
    pub line: i32,
    /// 字节码下标：该行在 LineNumberTable 中的首个 start_pc（帧位置的精度是语句行）；无行号 0
    pub bci: i32,
}

impl JavaFrame {
    pub fn is_native(&self) -> bool { self.method.is_native }

    /// HotSpot `Method::is_hidden`：`@jdk.internal.vm.annotation.Hidden` 注解的方法。隐藏类（lambda
    /// 代理等）的方法在本模型是闭包帧，已不成帧。
    pub fn is_hidden(&self) -> bool {
        crate::anno_pool::has_annotation(self.class, self.method.annotations, "Ljdk/internal/vm/annotation/Hidden;")
    }

    /// HotSpot `Method::caller_sensitive`：`@jdk.internal.reflect.CallerSensitive` 注解的方法。
    pub fn is_caller_sensitive(&self) -> bool {
        crate::anno_pool::has_annotation(self.class, self.method.annotations, "Ljdk/internal/reflect/CallerSensitive;")
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
        let class_flags = class_access_flags(self.class);
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

    /// HotSpot `Method::is_ignored_by_security_stack_walk`：Method.invoke、MethodAccessorImpl 子类的
    /// 方法、`@LambdaForm.Compiled` 帧（getCallerClass / getClassContext 跳过）。
    pub fn is_ignored_by_security_stack_walk(&self) -> bool {
        (self.class == "java/lang/reflect/Method" && self.method.name == "invoke")
            || self.class_extends("jdk/internal/reflect/MethodAccessorImpl")
            || crate::anno_pool::has_annotation(self.class, self.method.annotations, "Ljava/lang/invoke/LambdaForm$Compiled;")
    }

    /// 声明类是 `ancestor` 或其（任意深度）子类。
    pub fn class_extends(&self, ancestor: &str) -> bool {
        class_extends(self.class, ancestor)
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

/// 捕获当前线程的 Java 帧序列（自栈顶向下）：返回地址逐个查地址表（`pc_map`），每个物理帧给出其内联链上
/// 的 Java 帧（自内向外）。
pub fn capture_java_frames() -> Vec<JavaFrame> {
    let map = crate::pc_map::table();
    crate::pc_map::return_addresses()
        .into_iter()
        .flat_map(|pc| crate::pc_map::frames_at(pc).iter())
        .map(|&(method, line)| java_frame(&map.methods[method as usize], line))
        .collect()
}

/// 地址表的帧项 → Java 帧
fn java_frame(m: &crate::meta::LineMethod, line: i32) -> JavaFrame {
    let &(class, name, descriptor, source, flags, annotations) = m;
    let method = FrameMethod {
        name,
        descriptor,
        modifiers: (flags & 0xFFFF) as i32,
        is_static: flags & (1 << 16) != 0,
        is_native: flags & (1 << 17) != 0,
        annotations,
    };
    // 手写体无 Java 行（-1）时 bci 取 -1：StackWalker 按 bci 定行同样得 -1，与 Throwable 栈一致
    let bci = match line {
        l if l > 0 => bci_of_line(class, name, descriptor, l as u16),
        -1 => -1,
        _ => 0,
    };
    JavaFrame { class, method, source: (!source.is_empty()).then_some(source), line, bci }
}

/// 方法的 LineNumberTable（行表方法项；无表 → None）。描述符缺省时按 (类, 名) 唯一对位。
fn line_number_table(class: &str, name: &str, descriptor: Option<&str>) -> Option<&'static [(u16, u16)]> {
    let all = crate::meta::line_numbers();
    match descriptor {
        Some(d) => all.binary_search_by(|e| (e.0, e.1, e.2).cmp(&(class, name, d))).ok().map(|at| all[at].3),
        None => {
            let from = all.partition_point(|e| (e.0, e.1) < (class, name));
            match all[from..].iter().take_while(|e| e.0 == class && e.1 == name).collect::<Vec<_>>()[..] {
                [only] => Some(only.3),
                _ => None,
            }
        }
    }
}

/// 行的首个 start_pc
fn bci_of_line(class: &str, name: &str, descriptor: &str, line: u16) -> i32 {
    line_number_table(class, name, Some(descriptor))
        .and_then(|t| t.iter().find(|(_, l)| *l == line))
        .map_or(0, |(pc, _)| *pc as i32)
}

/// HotSpot `Method::line_number_from_bci`：start_pc 不大于 bci 的最后一项的行；无 LineNumberTable → -1。
pub fn line_number_from_bci(class: &str, name: &str, descriptor: Option<&str>, bci: i32) -> i32 {
    let Some(table) = line_number_table(class, name, descriptor) else { return -1 };
    let at = table.partition_point(|(pc, _)| (*pc as i32) <= bci);
    at.checked_sub(1).map_or(-1, |i| table[i].1 as i32)
}
