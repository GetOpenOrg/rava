//! 手写层（runtime/java_runtime/src 共置 `_impl.rs` / `_ext.rs`）的语法级扫描（syn）。
//!
//! 走真实语法树（取代已删除的 Python 正则扫描 native_upcalls.py），抽取：
//! - `pub fn` 名（成员由手写体提供的判定）；
//! - 手写体分配：`let mut x = T::default(); x._init_not_null();`（单独的 `T::default()` 是 Java null）
//!   与构造调用 `T::new*(…)`；沿同文件 fn 调用传递闭包；
//! - 调用表达式的名字与实参 / 接收者类型（语法推断，见 [`TypedCall`]）：引擎据此反解 Rust→Java 回调边
//!  （手写层不声明回调），实参按调用点精确接入，推断不出时退回手写方法的值池。
//!
//! 成员匹配：Rust fn 名 = Java 名或 `名_<重载后缀>`；虚方法体前缀 `__impl_`；构造器 `<init>` ↔ `new`。

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use classfile::MemberRef;

mod generic_fns;
mod guards;
mod hooks;
mod objects;
mod scan;
mod stype;
mod syntax;
mod macro_fn_lint;
mod thread_local_lint;
mod type_refs;
mod units;
mod vm_writes;
pub use type_refs::MODULE_SUFFIXES;
pub use units::CRATE_ROOT;
use scan::{close_transitive, collect_uses, prelude_uses, scan_file, FileFns};
use syntax::path_segs;

/// 生成类文件的标记（手写共置文件恒不含限定宏调用）
pub const GENERATED_MARK: &str = "rava_macros::java_class";
const SUFFIXES: [&str; 2] = ["_impl.rs", "_ext.rs"];
const CTOR_RUST: &str = "new";
/// prelude 导出的 Java 根类型的 Rust 名（`Object::from(x)` 是保持身份的上转；字符串字面量产出 String）
const OBJECT_RUST: &str = "Object";
const STRING_RUST: &str = "String";
const VIRTUAL_PREFIX: &str = "__impl_";
/// Java 类型的 vtable trait 名后缀（`X__VTable`，rava_macros 生成）
const VTABLE_SUFFIX: &str = "__VTable";
/// 字段访问器前缀（rava_macros 生成）
const SET_PREFIX: &str = "__set_";
const GET_PREFIX: &str = "__get_";
/// java_class! 为 static 字段生成的写访问器前缀（`T::set_<字段>(v)`）
const STATIC_SET_PREFIX: &str = "set_";
/// ObjectVTable 的实例字段按名协议（object.rs）：首参为字段名字面量时即该名字段的读 / 写
const BY_NAME_WRITES: &[&str] = &["__unsafe_ref_set", "__unsafe_ref_update", "__unsafe_int_set"];
const BY_NAME_READS: &[&str] = &["__unsafe_ref_get", "__unsafe_int_cell", "__unsafe_long_cell", "__unsafe_bool_cell"];
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use", "where", "while", "abstract",
    "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// 生成类的类初始化入口名（`T::__class_init()`：JVMS §5.5 主动初始化 T）
pub(crate) const CLASS_INIT_RUST: &str = "__class_init";

/// 回调目标（由手写体调用点推断）：方法、静态字段或类初始化
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Upcall {
    Method(MemberRef),
    Field(MemberRef),
    Init(String),
}

/// Rust 类型路径（分段）→ binary name 候选（由调用方按类路径验证存在）
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeRef(pub Vec<String>);

/// 手写体里的一次调用：`recv.name(args)` 或 `Path::name(args)`。
///
/// 实参类型只从保持对象身份的形态推断——分配根（`T::new*` / `T::from(非 Java 值)` / 字符串字面量 /
/// `T::default()` + `_init_not_null`）经 `Object::from`、`clone`、`?`、`try_cast`、`into`、局部 `let`
/// 传递；其余一律 None（未知），由引擎退回值池。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedCall {
    pub name: String,
    /// 路径调用的类型段（`T::new` 的 `T`）
    pub path_ty: Option<TypeRef>,
    /// 方法调用的接收者类型：外层 None = 路径调用；内层 None = 未知
    pub recv: Option<Option<TypeRef>>,
    pub args: Vec<Option<TypeRef>>,
    /// 接收者是本 fn 内 `let x = T::new*(…)` 绑定、此后未被遮蔽的不可变局部变量：
    /// 实际接收者只能是该方法手写体新建的 `T` 对象
    pub fresh: Option<TypeRef>,
    /// 方法调用接收者的静态类型（语法推断；推断回调目标用）
    pub srecv: Option<SType>,
    /// 各实参的字符串字面量值（见 `syntax::str_lit`；非字面量为 None）
    pub lits: Vec<Option<String>>,
    /// 接收者是本 fn 的 `self` / `self.0`，即被调 Java 方法自身的接收者（经同文件被调 fn 传递来的调用不算）
    pub on_self: bool,
}

/// 接收者的静态类型（语法推断）：具名类型 / `T::m(…)` 的返回类型 / `x.__get_f()` 的字段类型 /
/// `x.m(…)` 的返回类型（`x` 静态类型上 Rust 名为 `m` 的方法）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SType {
    Named(TypeRef),
    Ret(TypeRef, String),
    Field(Box<SType>, String),
    Call(Box<SType>, String),
    /// Java binary name 给出的类型：类名判定守卫区域内的接收者（`v.0.__class_name() == "c"` 等，见 `guards.rs`）
    Java(String),
}

/// 手写体里的字段访问器调用：`recv.__set_<字段>(v)` / `recv.__get_<字段>()`，及 static 字段写访问器
/// `T::set_<字段>(v)`（字段名即 Java 名）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccess {
    pub field: String,
    pub write: bool,
    /// 接收者静态类型；推断不出为 None（引擎按字段名在所有声明类上保守处理）
    pub recv: Option<SType>,
    /// 写入值的动态类型（同 [`TypedCall`] 实参规则）
    pub value: Option<TypeRef>,
    /// 写入值是本文件辅助 fn 的返回、该 fn 只收标量形参（Rust 基本类型 / `str`，见 `stype::scalar_arg_fns`）：
    /// 返回值不可能是任何实参，只能来自被调 fn 的产出（分配、回调结果、静态读取），引擎取手写体产出而非值池
    pub value_fresh: bool,
    /// 接收者是本 fn 的 `self`（即被调 Java 方法的接收者；经同文件被调 fn 传递来的访问不算）
    pub on_self: bool,
    /// 写入值是被调 Java 方法的接收者：本 fn 的 `self` 经保持身份的转换（`Clone::clone` / `.clone()` / 引用 / `Object::from`）
    /// 写入；经同文件被调 fn 传递时，只在调用链全程以 `self` 为接收者（`self.f(…)`）时成立
    pub value_self: bool,
    /// static 写访问器路径调用 `T::set_<字段>(v)`：`recv` 为 `T`；T 上无此字段时是同名的手写辅助函数，不算访问
    pub path: bool,
    /// 写入值取自按名读（`o.0.__unsafe_ref_get("f")`，经 unwrap / 转换 / `?` / 局部 let 传递）的字段名 `f`：
    /// 值即读取接收者上该字段的内容（转换只约束类型，不产生新对象；语法推得的转换目标类型不作写入值）
    pub value_src: Option<String>,
    /// 按名读的接收者是本 fn 的形参：其序号（含接收者，即被调 Java 方法的形参序号）。经同文件被调 fn 传递来的访问不算；
    /// None 时接收者未知，同名字段全部作来源
    pub value_src_param: Option<u16>,
    /// 按名写入（`o.0.__unsafe_ref_set("f", v)`）的接收者 `o` 本身取自按名读（`o = r.0.__unsafe_ref_get("g")`，经解包 / 转换 /
    /// 局部 let 传递）的字段名 `g`：接收者即 `r` 上该字段的内容，引擎按其值集逐个类型解出被写字段（见 `engine/hw_name_write.rs`）
    pub recv_src: Option<String>,
    /// `recv_src` 按名读的接收者 `r` 是本 fn 的形参：其序号（含接收者）；None 时 `r` 未知，同名字段全部作来源
    pub recv_src_param: Option<u16>,
}

#[derive(Debug, Default, Clone)]
pub struct FnInfo {
    pub is_pub: bool,
    /// 调用点（含同文件被调 fn 的传递闭包）
    pub calls: Vec<TypedCall>,
    /// 宏调用内出现的标识符（syn 不展开宏：同名回调可能藏在宏里 → 按未知处理）
    pub opaque: HashSet<String>,
    /// 分配的类型（含同文件被调 fn 的传递闭包）
    pub allocs: BTreeSet<TypeRef>,
    /// 构造调用（类型, Rust 构造器名 `new` / `new_<后缀>`），同样传递闭包
    pub ctors: BTreeSet<(TypeRef, String)>,
    /// 字段访问器调用，同样传递闭包
    pub fields: Vec<FieldAccess>,
    /// 取得引用元素数组视图（`JArray<引用>`、`__view_into`、`array_store_object` 等，含宏内与同文件被调 fn；
    /// 基本元素视图 `JArray<i8>` 等不计）：没有这种视图的手写体不可能改写实参数组的引用元素
    pub array_access: bool,
    /// 引用的手写实现对象（本文件的 struct 名 `S`，或经 `use` 引入的 `…::<类>_impl::S`），同样传递闭包：
    /// 手写体可能在此新建该对象并交给建模代码
    pub objects: BTreeSet<TypeRef>,
}

/// 手写实现对象：手写文件里实现 Java 类型 vtable trait（`impl X__VTable for S`）的本地 struct。
/// 它是运行期真实存在的 Java 接收者（手写边界方法把它当作 `X` 交出），方法体即各 trait impl 的 fn
#[derive(Debug, Default)]
pub struct HwObject {
    /// 实现的 Java 类型（trait 路径去 `__VTable`，按 use 表展开）
    pub supers: BTreeSet<TypeRef>,
    /// trait impl 的 fn 名 → 信息（沿本对象与同文件 fn 调用传递闭包）
    pub fns: HashMap<String, FnInfo>,
}

/// 一个类的共置手写文件汇总
#[derive(Debug, Default)]
pub struct ClassHw {
    pub files: Vec<PathBuf>,
    /// fn 名 → 信息（同名 fn 合并）
    pub fns: HashMap<String, FnInfo>,
    /// 手写文件（共置 `_impl` / `_ext` 与整体手写 `<snake>.rs`）的编译期类型路径
    pub type_refs: BTreeSet<TypeRef>,
    /// 手写实现对象（struct 名 → 对象）；其 trait impl fn 不并入 `fns`
    pub objects: BTreeMap<String, HwObject>,
    /// 文件定义的类型名（只对模块单元填写：路径调用 `模块::T::f` 的定位）
    pub types: BTreeSet<String>,
    /// 顶层 impl 块关联 fn 的返回类型：(impl self 类型全路径, fn 名) → 返回类型全路径（剥 `Result` / `Option`）
    pub rets: stype::LocalRets,
}

/// 成员（Java 名）对应的手写体汇总
#[derive(Debug, Default)]
pub struct MemberHw {
    pub provided: bool,
    pub allocs: BTreeSet<TypeRef>,
    pub ctors: BTreeSet<(TypeRef, String)>,
    pub calls: Vec<TypedCall>,
    pub opaque: HashSet<String>,
    pub fields: Vec<FieldAccess>,
    pub array_access: bool,
    pub objects: BTreeSet<TypeRef>,
    /// 命中的 fn 名（溯源）
    pub fns: Vec<String>,
}

impl MemberHw {
    /// 并入一个命中的 fn
    pub fn absorb(&mut self, name: &str, f: &FnInfo) {
        self.provided |= f.is_pub;
        self.allocs.extend(f.allocs.iter().cloned());
        self.ctors.extend(f.ctors.iter().cloned());
        self.calls.extend(f.calls.iter().cloned());
        self.opaque.extend(f.opaque.iter().cloned());
        self.fields.extend(f.fields.iter().cloned());
        self.array_access |= f.array_access;
        self.objects.extend(f.objects.iter().cloned());
        self.fns.push(name.to_string());
    }
}

pub struct Handwritten {
    src: PathBuf,
    /// `lib.rs` 的 `mod prelude` 导出（手写文件 `use crate::prelude::*` 引入）
    prelude: HashMap<String, Vec<String>>,
    cache: RefCell<HashMap<String, Rc<ClassHw>>>,
    /// 模块单元（见 `units.rs`；首次使用时载入）
    units: RefCell<Option<Rc<BTreeMap<String, Rc<ClassHw>>>>>,
    /// 共置手写文件的顶层 fn 返回类型（文件模块路径 → 返回表；模块路径调用 `super::x_impl::f` 首次解析时载入）
    file_rets: RefCell<HashMap<String, Rc<stype::LocalRets>>>,
    abbrev: HashMap<String, String>,
    pub errors: RefCell<Vec<String>>,
}

/// 类的手写文件（共置 `_impl` / `_ext` 与整体手写 `<snake>.rs`）中的编译期类型路径；解析错误追加到 `errors`
fn class_type_refs(src: &Path, prelude: &HashMap<String, Vec<String>>, cls: &str, errors: &mut Vec<String>) -> BTreeSet<TypeRef> {
    let (pkg, simple) = cls.rsplit_once('/').unwrap_or(("", cls));
    let mut out = BTreeSet::new();
    let stem = to_snake(simple);
    for suf in SUFFIXES.iter().chain([".rs"].iter()) {
        // 类名以 Impl / Ext 结尾时 `<snake>.rs` 是同包类 X 的共置手写（`x_impl.rs`），不是本类的整体手写
        // （发射层布局让出该路径，见 emit `companion_clash`）
        if *suf == ".rs" && MODULE_SUFFIXES.iter().any(|x| stem.ends_with(x)) {
            continue;
        }
        let path = src.join(pkg).join(format!("{stem}{suf}"));
        let Ok(content) = std::fs::read_to_string(&path) else { continue };
        match type_refs::scan(&content, prelude) {
            Ok(t) => out.extend(t),
            Err(e) => errors.push(format!("{}：{e}", path.display())),
        }
    }
    out
}

/// 只取手写文件类型路径的只读扫描器（无缓存、可跨线程共享）：与 [`Handwritten::class`] 的
/// `type_refs` 同源同值，供落盘阶段按目录并行判定共置手写的模块依赖
pub struct HwTypeRefs {
    src: PathBuf,
    prelude: HashMap<String, Vec<String>>,
}

impl HwTypeRefs {
    /// `runtime_dir` = runtime/java_runtime
    pub fn new(runtime_dir: &Path) -> Self {
        HwTypeRefs { src: runtime_dir.join("src"), prelude: load_prelude(runtime_dir) }
    }

    /// 同 `Handwritten::class(cls).type_refs`（解析错误忽略）
    pub fn class(&self, cls: &str) -> BTreeSet<TypeRef> {
        class_type_refs(&self.src, &self.prelude, cls, &mut Vec::new())
    }
}

fn load_prelude(runtime_dir: &Path) -> HashMap<String, Vec<String>> {
    std::fs::read_to_string(runtime_dir.join("src/lib.rs"))
        .ok()
        .and_then(|c| syn::parse_file(&c).ok())
        .map(|f| prelude_uses(&f))
        .unwrap_or_default()
}

pub fn to_snake(name: &str) -> String {
    let s: Vec<char> = name.replace('$', "_").chars().collect();
    // ([A-Z]+)([A-Z][a-z]) → \1_\2 ；([a-z\d])([A-Z]) → \1_\2
    let mut out = String::new();
    for i in 0..s.len() {
        let c = s[i];
        if i > 0 && c.is_ascii_uppercase() {
            let p = s[i - 1];
            let next_lower = s.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if p.is_ascii_lowercase() || p.is_ascii_digit() || (p.is_ascii_uppercase() && next_lower) {
                out.push('_');
            }
        }
        out.push(c);
    }
    let out = out.to_lowercase();
    if RUST_KEYWORDS.contains(&out.as_str()) {
        out + "_"
    } else {
        out
    }
}

pub fn member_matches(fn_name: &str, member: &str) -> bool {
    let rust = if member == "<init>" { CTOR_RUST } else { member };
    let f = fn_name.strip_prefix(VIRTUAL_PREFIX).unwrap_or(fn_name);
    f == rust || f.strip_prefix(rust).is_some_and(|r| r.starts_with('_'))
}

impl Handwritten {
    /// `runtime_dir` = runtime/java_runtime
    pub fn new(runtime_dir: &Path) -> Self {
        let mut abbrev = HashMap::new();
        if let Ok(s) = std::fs::read_to_string(runtime_dir.join("overload_abbrev.txt")) {
            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut it = line.split_whitespace();
                if let (Some(k), Some(v)) = (it.next(), it.next()) {
                    abbrev.insert(k.to_string(), v.to_string());
                }
            }
        }
        let prelude = load_prelude(runtime_dir);
        Handwritten {
            src: runtime_dir.join("src"),
            prelude,
            cache: RefCell::new(HashMap::new()),
            units: RefCell::new(None),
            file_rets: RefCell::new(HashMap::new()),
            abbrev,
            errors: RefCell::new(Vec::new()),
        }
    }

    pub fn class(&self, cls: &str) -> Rc<ClassHw> {
        if let Some(c) = self.cache.borrow().get(cls) {
            return c.clone();
        }
        let hw = Rc::new(self.load(cls));
        self.cache.borrow_mut().insert(cls.to_string(), hw.clone());
        hw
    }

    fn load(&self, cls: &str) -> ClassHw {
        let (pkg, simple) = cls.rsplit_once('/').unwrap_or(("", cls));
        let mut hw = ClassHw::default();
        let mut raw = FileFns::default();
        for suf in SUFFIXES {
            let path = self.src.join(pkg).join(format!("{}{suf}", to_snake(simple)));
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            // 自动生成的类文件碰巧以 _impl.rs 结尾（类名含 Impl / Ext）：生成类文件恒含限定宏调用
            if content.contains(GENERATED_MARK) {
                continue;
            }
            match syn::parse_file(&content) {
                Ok(file) => scan_file(&file, &self.prelude, &mut raw),
                Err(e) => self.errors.borrow_mut().push(format!("{}：{e}", path.display())),
            }
            hw.files.push(path);
        }
        hw.type_refs = class_type_refs(&self.src, &self.prelude, cls, &mut self.errors.borrow_mut());
        close_transitive(&mut raw.fns, &raw.calls, &raw.nonself);
        hw.objects = objects::close(&raw);
        hw.rets = std::mem::take(&mut raw.rets);
        hw.fns = raw.fns;
        hw
    }

    /// 成员 `member`（Java 名；构造器 `<init>`）对应的手写体
    pub fn member(&self, cls: &str, member: &str) -> MemberHw {
        let hw = self.class(cls);
        let mut out = MemberHw::default();
        let mut names: Vec<&String> = hw.fns.keys().filter(|f| member_matches(f, member)).collect();
        names.sort();
        for n in names {
            out.absorb(n, &hw.fns[n]);
        }
        out
    }

    /// 手写实现对象 `obj`（`cls` 的手写文件里）按 fn 名汇总的手写体
    pub fn object_member(&self, cls: &str, obj: &str, fns: &[String]) -> MemberHw {
        let hw = self.class(cls);
        let mut out = MemberHw::default();
        if let Some(o) = hw.objects.get(obj) {
            for n in fns {
                if let Some(f) = o.fns.get(n) {
                    out.absorb(n, f);
                }
            }
        }
        out
    }

    /// 类有共置手写文件（含至少一个 pub fn）
    pub fn has_impls(&self, cls: &str) -> bool {
        self.class(cls).fns.values().any(|f| f.is_pub)
    }

    /// Rust 构造器名：`new` + 描述符重载后缀（`ty::type_map::mangle_name` 同规则）
    pub fn ctor_name(&self, desc: &str) -> String {
        let suffix = self.descriptor_suffix(desc);
        if suffix.is_empty() {
            CTOR_RUST.to_string()
        } else {
            format!("{CTOR_RUST}_{suffix}")
        }
    }

    fn short(&self, cls: &str) -> String {
        let s = cls.rsplit('/').next().unwrap_or(cls).to_lowercase().replace('$', "_");
        self.abbrev.get(&s).cloned().unwrap_or(s)
    }

    pub fn descriptor_suffix(&self, desc: &str) -> String {
        let Some(params) = desc.strip_prefix('(').and_then(|d| d.split(')').next()) else { return String::new() };
        let b = params.as_bytes();
        let mut parts = Vec::new();
        let mut i = 0;
        let prim = |c: u8| match c {
            b'I' => Some("i"),
            b'J' => Some("l"),
            b'Z' => Some("z"),
            b'B' => Some("b"),
            b'S' => Some("s"),
            b'F' => Some("f"),
            b'D' => Some("d"),
            b'C' => Some("c"),
            _ => None,
        };
        while i < b.len() {
            let c = b[i];
            if let Some(p) = prim(c) {
                parts.push(p.to_string());
                i += 1;
            } else if c == b'L' {
                let Some(end) = params[i..].find(';').map(|e| e + i) else { break };
                parts.push(self.short(&params[i + 1..end]));
                i = end + 1;
            } else if c == b'[' {
                let mut j = i + 1;
                while j < b.len() && b[j] == b'[' {
                    j += 1;
                }
                if j >= b.len() {
                    break;
                }
                if b[j] == b'L' {
                    let Some(end) = params[j..].find(';').map(|e| e + j) else { break };
                    parts.push(format!("{}{}", "arr_".repeat(j - i), self.short(&params[j + 1..end])));
                    i = end + 1;
                } else {
                    parts.push(format!("{}{}", "arr_".repeat(j - i), prim(b[j]).unwrap_or("x")));
                    i = j + 1;
                }
            } else {
                i += 1;
            }
        }
        parts.join("_")
    }

    /// 类型路径 → binary name 候选（`Self` → 宿主类；单段按 use 表或同包；`_` 可能是 `$`）
    pub fn resolve_type(&self, host: &str, t: &TypeRef) -> Vec<String> {
        resolve_type(host, t)
    }
}

/// 手写文件的 use 表：本地名 → 完整路径段（解析失败为空）
pub fn file_uses(content: &str) -> HashMap<String, Vec<String>> {
    let mut out = HashMap::new();
    if let Ok(file) = syn::parse_file(content) {
        for item in &file.items {
            if let syn::Item::Use(u) = item {
                collect_uses(&u.tree, &mut Vec::new(), &mut out);
            }
        }
    }
    out
}

/// 类型路径 → binary name 候选（`host` 为手写文件的宿主类；见 [`Handwritten::resolve_type`]）
pub fn resolve_type(host: &str, t: &TypeRef) -> Vec<String> {
    let pkg: Vec<&str> = host.rsplit_once('/').map_or(vec![], |(p, _)| p.split('/').collect());
    let segs = &t.0;
    if segs.len() == 1 && segs[0] == "Self" {
        return vec![host.to_string()];
    }
    let ty = segs.last().cloned().unwrap_or_default();
    let (base, rest): (Vec<String>, &[String]) = match segs.first().map(String::as_str) {
        // 共置手写是包模块的子模块：首个 `super` = 宿主包，其后每个 `super` 上溯一级
        Some("super") => {
            let k = segs.iter().take_while(|s| *s == "super").count();
            (pkg[..pkg.len().saturating_sub(k - 1)].iter().map(|s| s.to_string()).collect(), &segs[k..])
        }
        Some("crate") => (vec![], &segs[1..]),
        _ if segs.len() == 1 => (pkg.iter().map(|s| s.to_string()).collect(), &segs[..]),
        _ => (vec![], &segs[..]),
    };
    let snake = to_snake(&ty);
    let mut full = base;
    for m in &rest[..rest.len().saturating_sub(1)] {
        let m = m.strip_prefix("r#").unwrap_or(m);
        if m != "self" {
            full.push(m.to_string());
        }
    }
    // 末段模块与类型同名：可能是类文件模块（`stream_decoder::StreamDecoder`），也可能是包
    // （`charset::Charset`）——两种前缀都给出
    let mut prefixes = vec![full.clone()];
    if full.last() == Some(&snake) {
        full.pop();
        prefixes.insert(0, full);
    }
    let mut out = Vec::new();
    for mut p in prefixes {
        if p.is_empty() {
            p = pkg.iter().map(|s| s.to_string()).collect();
        }
        let prefix = p.join("/");
        out.extend(dollar_variants(&ty).into_iter().map(|v| if prefix.is_empty() { v } else { format!("{prefix}/{v}") }));
    }
    out
}

/// `A_B_C` 的 `_` ↔ `$` 组合（嵌套类的 Rust 名把 `$` 换成 `_`）；原名优先
fn dollar_variants(ty: &str) -> Vec<String> {
    let idx: Vec<usize> = ty.match_indices('_').map(|m| m.0).filter(|i| *i > 0).collect();
    if idx.len() > 4 {
        return vec![ty.to_string()];
    }
    let mut out = Vec::new();
    for mask in 0..(1u32 << idx.len()) {
        let mut b = ty.as_bytes().to_vec();
        for (k, i) in idx.iter().enumerate() {
            if mask & (1 << k) != 0 {
                b[*i] = b'$';
            }
        }
        out.push(String::from_utf8(b).unwrap_or_default());
    }
    out
}
