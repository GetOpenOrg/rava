//! 反射元数据表：元素类型与查询入口。
//!
//! 表分两侧（docs/plans/2026-10-01-cross-test-compile-reuse.md §6.4）：
//! - 档案侧（JDK 与 lib crate 的类）：java_meta crate 承载，每个表以 `__java_meta_<表名>` 符号导出，
//!   本模块以同名 extern 声明读取。本 crate 不依赖 java_meta；
//!   符号在最终链接时解析：user crate 以 `use java_meta as _;` 把 java_meta 纳入链接
//!  （方案 docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S1）。
//! - 用户侧（用户类）：生成在用户 crate（[`UserMeta`]），入口在一切 VM 活动之前 [`register_user`] 登记。
//!
//! 查询入口返回两侧合并后的表（首次查询时解码并合并，按类名有序；未登记用户侧时即档案侧表本身）。
//! 表在二进制中是字符串池 + 字节流（二进制体积 B1(c)），解码见 [`crate::meta_codec`]。

use crate::meta_codec;

/// 单个声明字段的元数据。modifiers 为 java.lang.reflect.Modifier 位集；
/// constant 为 ConstantValue 属性的整数值（仅整型常量收录）。
pub struct FieldMeta {
    pub name:       &'static str,
    pub descriptor: &'static str,
    pub modifiers:  i32,
    pub is_static:  bool,
    pub constant:   Option<i64>,
    pub annotations: &'static [u8],
    /// Signature 属性（泛型签名）；无则空串
    pub signature:  &'static str,
}

/// 单个声明方法的元数据；方法身份键是 (name, descriptor) 二元组（重载语义）。
pub struct MethodMeta {
    pub name:        &'static str,
    pub descriptor:  &'static str,
    pub modifiers:   i32,
    pub is_static:   bool,
    pub is_native:   bool,
    pub is_abstract: bool,
    pub exceptions:  &'static [&'static str],
    pub annotations: &'static [u8],
    pub param_annotations: &'static [u8],
    pub annotation_default: &'static [u8],
    /// Signature 属性（泛型签名）；无则空串
    pub signature:   &'static str,
    pub inherited:   bool,
    /// 复制进本类的方法体（接口 default / 未覆盖的超类虚方法）的声明类型（binary name）；本类声明 / 继承转发行为空串
    pub declared_by:  &'static str,
}

/// 嵌套元数据（InnerClasses 本类条目 + EnclosingMethod）。
#[derive(Clone, Copy)]
pub struct NestMeta {
    pub outer:      &'static str,
    pub simple:     &'static str,
    pub self_entry: bool,
    pub enclosing:  Option<(&'static str, &'static str, &'static str)>,
    /// 本类声明的成员类（InnerClasses 中 outer 为本类的条目，属性序）
    pub members:    &'static [&'static str],
}

/// 注解引用的稀疏常量池条目值。
pub enum CpVal { U(&'static str), W(&'static [u16]), I(i32), J(i64), F(f32), D(f64) }

type Names = &'static [&'static str];
/// 帧方法项 (帧归属类, 方法名, 描述符, 源文件, 标志字, RuntimeVisibleAnnotations 原始属性体)：
/// 标志字 = Modifier 位集（低 16 位）| static << 16 | native << 17。帧方法元数据只来自地址表（`pc_map`），不读成员表
pub type LineMethod = (&'static str, &'static str, &'static str, &'static str, u32, &'static [u8]);
/// 帧方法的 LineNumberTable：(类, 方法名, 描述符, [(start_pc, 行)])，按 (类, 名, 描述符) 升序
pub type LineNumbers = (&'static str, &'static str, &'static str, &'static [(u16, u16)]);
extern "Rust" {
    #[link_name = "__java_meta_META_POOL"]
    static META_POOL: &'static [u8];
    #[link_name = "__java_meta_CLOSURE_POOL"]
    static CLOSURE_POOL: &'static [u8];
    #[link_name = "__java_meta_LINE_POOL"]
    static LINE_POOL: &'static [u8];
    #[link_name = "__java_meta_CLASS_HIERARCHY"]
    static CLASS_HIERARCHY: &'static [u8];
    #[link_name = "__java_meta_CLASS_DIRECT_SUPER"]
    static CLASS_DIRECT_SUPER: &'static [u8];
    #[link_name = "__java_meta_CLASS_FIELDS"]
    static CLASS_FIELDS: &'static [u8];
    #[link_name = "__java_meta_CLASS_METHODS"]
    static CLASS_METHODS: &'static [u8];
    #[link_name = "__java_meta_CLASS_MODIFIERS"]
    static CLASS_MODIFIERS: &'static [u8];
    #[link_name = "__java_meta_CLASS_NEST"]
    static CLASS_NEST: &'static [u8];
    #[link_name = "__java_meta_CLASS_INTERFACES"]
    static CLASS_INTERFACES: &'static [u8];
    #[link_name = "__java_meta_CLASS_ANNO"]
    static CLASS_ANNO: &'static [u8];
    #[link_name = "__java_meta_CLINIT_CLASSES"]
    static CLINIT_CLASSES: &'static [u8];
    #[link_name = "__java_meta_HIDDEN_CLASSES"]
    static HIDDEN_CLASSES: &'static [u8];
    #[link_name = "__java_meta_PERMITTED_SUBCLASSES"]
    static PERMITTED_SUBCLASSES: &'static [u8];
    #[link_name = "__java_meta_NEST_MEMBERS"]
    static NEST_MEMBERS: &'static [u8];
    #[link_name = "__java_meta_CLASS_ACCESS_FLAGS"]
    static CLASS_ACCESS_FLAGS: &'static [u8];
    #[link_name = "__java_meta_CLASS_SOURCE_FILE"]
    static CLASS_SOURCE_FILE: &'static [u8];
    #[link_name = "__java_meta_CLASS_DEFINING_LOADER"]
    static CLASS_DEFINING_LOADER: &'static [u8];
    #[link_name = "__java_meta_RECORD_CLASSES"]
    static RECORD_CLASSES: &'static [u8];
    #[link_name = "__java_meta_RECORD_COMPONENTS"]
    static RECORD_COMPONENTS: &'static [u8];
    #[link_name = "__java_meta_MODULE_SERVICES"]
    static MODULE_SERVICES: &'static [u8];
    #[link_name = "__java_meta_VM_CONST_PROPERTIES"]
    static VM_CONST_PROPERTIES: &'static [u8];
    #[link_name = "__java_meta_VM_DYNAMIC_PROPERTIES"]
    static VM_DYNAMIC_PROPERTIES: &'static [u8];
    #[link_name = "__java_meta_LINE_NUMBERS"]
    static LINE_NUMBERS: &'static [u8];
}

/// 用户类的反射元数据行（用户 crate 生成；表形与档案侧同名表一致：池 + 字节流，见 [`crate::meta_codec`]）
pub struct UserMeta {
    pub meta_pool: &'static [u8],
    pub class_hierarchy: &'static [u8],
    pub class_direct_super: &'static [u8],
    pub class_fields: &'static [u8],
    pub class_methods: &'static [u8],
    pub class_modifiers: &'static [u8],
    pub class_nest: &'static [u8],
    pub class_interfaces: &'static [u8],
    pub class_anno: &'static [u8],
    pub clinit_classes: &'static [u8],
    pub hidden_classes: &'static [u8],
    pub permitted_subclasses: &'static [u8],
    pub nest_members: &'static [u8],
    pub class_access_flags: &'static [u8],
    pub class_source_file: &'static [u8],
    pub class_defining_loader: &'static [u8],
    pub record_classes: &'static [u8],
    pub record_components: &'static [u8],
    pub closure_pool: &'static [u8],
    pub module_services: &'static [u8],
    pub line_pool: &'static [u8],
    pub line_numbers: &'static [u8],
}

static USER_META: std::sync::OnceLock<&'static UserMeta> = std::sync::OnceLock::new();

/// 登记用户侧元数据行（入口 `main` 首句，先于一切查询；重复登记忽略）
pub fn register_user(meta: &'static UserMeta) {
    let _ = USER_META.set(meta);
}

/// 档案侧表 + 用户侧行（各自首次查询时解码）：用户侧为空时即档案侧表，否则合并后按 `order` 稳定排序（进程内一次）
fn merged<T: Copy + 'static>(
    cell: &'static std::sync::OnceLock<&'static [T]>,
    archive: fn() -> &'static [T],
    user: fn(&'static UserMeta) -> &'static [T],
    order: Option<fn(&T, &T) -> std::cmp::Ordering>,
) -> &'static [T] {
    cell.get_or_init(|| {
        let archive = archive();
        match USER_META.get().map(|m| user(m)) {
            None | Some([]) => archive,
            Some(rows) => {
                let mut v = Vec::with_capacity(archive.len() + rows.len());
                v.extend_from_slice(archive);
                v.extend_from_slice(rows);
                if let Some(order) = order {
                    v.sort_by(order);
                }
                Vec::leak(v)
            }
        }
    })
}

// 表组读取器：三组（反射元数据 / 闭包派生表 / 行表）各一个池，两侧各自建池索引（进程内一次）。
// SAFETY（以下读 extern 池的函数与各查询入口同）：符号由 java_meta 以完全相同的类型定义为不可变 static，
// 初始化于编译期，读取无数据竞争。
fn archive_meta(table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, unsafe { META_POOL }), table)
}
fn archive_closure(table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, unsafe { CLOSURE_POOL }), table)
}
fn archive_line(table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, unsafe { LINE_POOL }), table)
}
fn user_meta(m: &'static UserMeta, table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, m.meta_pool), table)
}
fn user_closure(m: &'static UserMeta, table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, m.closure_pool), table)
}
fn user_line(m: &'static UserMeta, table: &'static [u8]) -> meta_codec::Reader {
    static POOL: std::sync::OnceLock<Vec<&'static [u8]>> = std::sync::OnceLock::new();
    meta_codec::Reader::new(meta_codec::pool_index(&POOL, m.line_pool), table)
}

// 合并表查询入口一律写成普通 fn（不经 macro_rules! 生成）：闭包分析器按 syn 扫描手写模块函数，
// 宏展开出的 fn 对其不可见，跨文件调用边会丢失（2026-10-04 种子不确定回归）。

/// 类 → 全部超类型（含自身）。
pub fn class_hierarchy() -> &'static [(&'static str, Names)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, Names)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::name_lists(archive_meta(unsafe { CLASS_HIERARCHY })), |m| meta_codec::name_lists(user_meta(m, m.class_hierarchy)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类的全部超类型（含自身；层次表按名有序，二分查找）；表外类 → 空。
pub fn supertypes(class: &str) -> Names {
    let table = class_hierarchy();
    table.binary_search_by(|(n, _)| (*n).cmp(class)).map_or(&[], |i| table[i].1)
}
/// `class` 的实例是否为 `of` 的实例（JVMS §6.5 checkcast / instanceof 的类型判定，按层次表）。
pub fn is_subtype_of(class: &str, of: &str) -> bool {
    supertypes(class).contains(&of)
}
/// 类 → 直接父类（接口缺席）。
pub fn class_direct_super() -> &'static [(&'static str, &'static str)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static str)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::pairs(archive_meta(unsafe { CLASS_DIRECT_SUPER })), |m| meta_codec::pairs(user_meta(m, m.class_direct_super)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 声明字段（声明序 = slot）。
pub fn class_fields() -> &'static [(&'static str, &'static [FieldMeta])] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static [FieldMeta])]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::fields(archive_meta(unsafe { CLASS_FIELDS })), |m| meta_codec::fields(user_meta(m, m.class_fields)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 声明方法（声明序 = slot）。
pub fn class_methods() -> &'static [(&'static str, &'static [MethodMeta])] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static [MethodMeta])]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::methods(archive_meta(unsafe { CLASS_METHODS })), |m| meta_codec::methods(user_meta(m, m.class_methods)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → Modifier 位集。
pub fn class_modifiers() -> &'static [(&'static str, i32)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, i32)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::ints(archive_meta(unsafe { CLASS_MODIFIERS })), |m| meta_codec::ints(user_meta(m, m.class_modifiers)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 嵌套元数据。
pub fn class_nest() -> &'static [(&'static str, NestMeta)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, NestMeta)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::nest(archive_meta(unsafe { CLASS_NEST })), |m| meta_codec::nest(user_meta(m, m.class_nest)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 直接超接口。
pub fn class_interfaces() -> &'static [(&'static str, Names)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, Names)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::name_lists(archive_meta(unsafe { CLASS_INTERFACES })), |m| meta_codec::name_lists(user_meta(m, m.class_interfaces)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 类级注解原始字节 + 稀疏常量池。
pub fn class_anno() -> &'static [(&'static str, &'static [u8], &'static [(i32, CpVal)])] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static [u8], &'static [(i32, CpVal)])]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::class_anno(archive_meta(unsafe { CLASS_ANNO })), |m| meta_codec::class_anno(user_meta(m, m.class_anno)), Some(|a, b| a.0.cmp(b.0)))
}
/// 含 <clinit> 的类集。
pub fn clinit_classes() -> &'static [&'static str] {
    static CELL: std::sync::OnceLock<&'static [&'static str]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::names(archive_meta(unsafe { CLINIT_CLASSES })), |m| meta_codec::names(user_meta(m, m.clinit_classes)), Some(|a, b| a.cmp(b)))
}
fn hidden_classes() -> &'static [&'static str] {
    static CELL: std::sync::OnceLock<&'static [&'static str]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::names(archive_meta(unsafe { HIDDEN_CLASSES })), |m| meta_codec::names(user_meta(m, m.hidden_classes)), Some(|a, b| a.cmp(b)))
}
fn class_defining_loader_table() -> &'static [(&'static str, &'static str)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static str)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::pairs(archive_meta(unsafe { CLASS_DEFINING_LOADER })), |m| meta_codec::pairs(user_meta(m, m.class_defining_loader)), Some(|a, b| a.0.cmp(b.0)))
}
/// 隐藏类集：lambda 调用点隐藏类（生成器 `hidden_class!` 声明，按名有序）与运行期登记的
/// @CallerSensitive 注入调用器（`injected_invoker`）。
pub fn is_hidden_class(class: &str) -> bool {
    hidden_classes().binary_search(&class).is_ok() || crate::injected_invoker::is_injected(class)
}
/// 内部名（`/` 分隔）→ Class.getName 形式：普通类全部 `/` 换 `.`；隐藏类只换调用者类部分，
/// 保留 `/0x…` 后缀（JVM 隐藏类名 `p.C$$Lambda/0x…`）。
pub fn java_name(class: &str) -> std::string::String {
    match class.rsplit_once('/') {
        Some((host, suffix)) if is_hidden_class(class) => format!("{}/{suffix}", host.replace('/', ".")),
        _ => class.replace('/', "."),
    }
}
/// sealed 类 → 许可子类型。
pub fn permitted_subclasses() -> &'static [(&'static str, Names)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, Names)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::name_lists(archive_meta(unsafe { PERMITTED_SUBCLASSES })), |m| meta_codec::name_lists(user_meta(m, m.permitted_subclasses)), Some(|a, b| a.0.cmp(b.0)))
}
/// 嵌套宿主 → NestMembers 属性所列成员。
pub fn nest_members() -> &'static [(&'static str, Names)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, Names)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::name_lists(archive_meta(unsafe { NEST_MEMBERS })), |m| meta_codec::name_lists(user_meta(m, m.nest_members)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 类文件 access_flags 原值（JVM_ACC_WRITTEN_FLAGS 掩码内）。
pub fn class_access_flags() -> &'static [(&'static str, i32)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, i32)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::ints(archive_meta(unsafe { CLASS_ACCESS_FLAGS })), |m| meta_codec::ints(user_meta(m, m.class_access_flags)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → SourceFile 属性值（无该属性的类不在表中）。
pub fn class_source_file() -> &'static [(&'static str, &'static str)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static str)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::pairs(archive_meta(unsafe { CLASS_SOURCE_FILE })), |m| meta_codec::pairs(user_meta(m, m.class_source_file)), Some(|a, b| a.0.cmp(b.0)))
}
/// 类 → 定义加载器（`app` / `platform`；引导加载器的类不在表中），按类名有序。
/// 注入调用器（`injected_invoker`）取宿主的定义加载器（JDK 以宿主的 Lookup 定义该隐藏类）。
pub fn class_defining_loader(class: &str) -> Option<&'static str> {
    let t = class_defining_loader_table();
    match t.binary_search_by(|(c, _)| (*c).cmp(class)) {
        Ok(i) => Some(t[i].1),
        Err(_) => crate::injected_invoker::host_of(class).and_then(|h| class_defining_loader(&h)),
    }
}
/// record 类集。
pub fn record_classes() -> &'static [&'static str] {
    static CELL: std::sync::OnceLock<&'static [&'static str]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::names(archive_meta(unsafe { RECORD_CLASSES })), |m| meta_codec::names(user_meta(m, m.record_classes)), Some(|a, b| a.cmp(b)))
}
/// record 类 → 分量（名、描述符、泛型签名）。
pub fn record_components() -> &'static [(&'static str, &'static [(&'static str, &'static str, &'static str)])] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static [(&'static str, &'static str, &'static str)])]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::record_components(archive_meta(unsafe { RECORD_COMPONENTS })), |m| meta_codec::record_components(user_meta(m, m.record_components)), Some(|a, b| a.0.cmp(b.0)))
}
/// 模块服务 (服务, provider)：闭包事实 seeds.module_services（发射层写入 java_meta），事实序。
pub fn module_services() -> &'static [(&'static str, &'static str)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static str)]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::pairs(archive_closure(unsafe { MODULE_SERVICES })), |m| meta_codec::pairs(user_closure(m, m.module_services)), None)
}
/// VM 初始系统属性的常量键（键, 值）：闭包事实 system_properties.values，与分析器折叠同源。
pub fn vm_const_properties() -> &'static [(&'static str, &'static str)] {
    static CELL: std::sync::OnceLock<&'static [(&'static str, &'static str)]> = std::sync::OnceLock::new();
    CELL.get_or_init(|| meta_codec::pairs(archive_closure(unsafe { VM_CONST_PROPERTIES })))
}
/// VM 初始系统属性的动态键（由手写层取宿主值）：闭包事实 system_properties.dynamic。
pub fn vm_dynamic_properties() -> &'static [&'static str] {
    static CELL: std::sync::OnceLock<&'static [&'static str]> = std::sync::OnceLock::new();
    CELL.get_or_init(|| meta_codec::names(archive_closure(unsafe { VM_DYNAMIC_PROPERTIES })))
}
/// 行表中各 Java 方法的 LineNumberTable（StackFrameInfo bci ↔ 行号）。
pub fn line_numbers() -> &'static [LineNumbers] {
    static CELL: std::sync::OnceLock<&'static [LineNumbers]> = std::sync::OnceLock::new();
    merged(&CELL, || meta_codec::line_numbers(archive_line(unsafe { LINE_NUMBERS })), |m| meta_codec::line_numbers(user_line(m, m.line_numbers)), Some(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2))))
}
