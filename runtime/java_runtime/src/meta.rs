//! 反射元数据表：元素类型与查询入口。
//!
//! 表本体由 java_meta crate 的构建脚本从整个 workspace 的 java_class! 属性生成（覆盖用户类与
//! lib 类），每个表以 `__java_meta_<表名>` 符号导出；本模块以同名 extern 声明读取。java_meta
//! 依赖本 crate（取本模块的元素类型），本 crate 不依赖 java_meta——用户类变化只重编 java_meta。
//! 符号在最终链接时解析：user crate 以 `use java_meta as _;` 把 java_meta 纳入链接。
//! 方案：docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S1。

/// 单个声明字段的元数据。modifiers 为 java.lang.reflect.Modifier 位集；
/// constant 为 ConstantValue 属性的整数值（仅整型常量收录）。
pub struct FieldMeta {
    pub name:       &'static str,
    pub descriptor: &'static str,
    pub modifiers:  i32,
    pub is_static:  bool,
    pub constant:   Option<i64>,
    pub annotations: &'static [u8],
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
    pub inherited:   bool,
}

/// 嵌套元数据（InnerClasses 本类条目 + EnclosingMethod）。
pub struct NestMeta {
    pub outer:      &'static str,
    pub simple:     &'static str,
    pub self_entry: bool,
    pub enclosing:  Option<(&'static str, &'static str, &'static str)>,
}

/// 注解引用的稀疏常量池条目值。
pub enum CpVal { U(&'static str), W(&'static [u16]), I(i32), J(i64), F(f32), D(f64) }

type Names = &'static [&'static str];

extern "Rust" {
    #[link_name = "__java_meta_CLASS_HIERARCHY"]
    static CLASS_HIERARCHY: &'static [(&'static str, Names)];
    #[link_name = "__java_meta_CLASS_DIRECT_SUPER"]
    static CLASS_DIRECT_SUPER: &'static [(&'static str, &'static str)];
    #[link_name = "__java_meta_CLASS_FIELDS"]
    static CLASS_FIELDS: &'static [(&'static str, &'static [FieldMeta])];
    #[link_name = "__java_meta_CLASS_METHODS"]
    static CLASS_METHODS: &'static [(&'static str, &'static [MethodMeta])];
    #[link_name = "__java_meta_CLASS_MODIFIERS"]
    static CLASS_MODIFIERS: &'static [(&'static str, i32)];
    #[link_name = "__java_meta_CLASS_NEST"]
    static CLASS_NEST: &'static [(&'static str, NestMeta)];
    #[link_name = "__java_meta_CLASS_INTERFACES"]
    static CLASS_INTERFACES: &'static [(&'static str, Names)];
    #[link_name = "__java_meta_CLASS_ANNO"]
    static CLASS_ANNO: &'static [(&'static str, &'static [u8], &'static [(i32, CpVal)])];
    #[link_name = "__java_meta_CLINIT_CLASSES"]
    static CLINIT_CLASSES: Names;
    #[link_name = "__java_meta_PERMITTED_SUBCLASSES"]
    static PERMITTED_SUBCLASSES: &'static [(&'static str, Names)];
    #[link_name = "__java_meta_RECORD_CLASSES"]
    static RECORD_CLASSES: Names;
    #[link_name = "__java_meta_RECORD_COMPONENTS"]
    static RECORD_COMPONENTS: &'static [(&'static str, &'static [(&'static str, &'static str, &'static str)])];
    #[link_name = "__java_meta_MODULE_SERVICES"]
    static MODULE_SERVICES: &'static [(&'static str, &'static str)];
    #[link_name = "__java_meta_VM_CONST_PROPERTIES"]
    static VM_CONST_PROPERTIES: &'static [(&'static str, &'static str)];
    #[link_name = "__java_meta_VM_DYNAMIC_PROPERTIES"]
    static VM_DYNAMIC_PROPERTIES: &'static [&'static str];
}

// SAFETY（以下各函数同）：符号由 java_meta 以完全相同的类型定义为不可变 static，
// 初始化于编译期，读取无数据竞争。

/// 类 → 全部超类型（含自身）。
pub fn class_hierarchy() -> &'static [(&'static str, Names)] { unsafe { CLASS_HIERARCHY } }
/// 类 → 直接父类（接口缺席）。
pub fn class_direct_super() -> &'static [(&'static str, &'static str)] { unsafe { CLASS_DIRECT_SUPER } }
/// 类 → 声明字段（声明序 = slot）。
pub fn class_fields() -> &'static [(&'static str, &'static [FieldMeta])] { unsafe { CLASS_FIELDS } }
/// 类 → 声明方法（声明序 = slot）。
pub fn class_methods() -> &'static [(&'static str, &'static [MethodMeta])] { unsafe { CLASS_METHODS } }
/// 类 → Modifier 位集。
pub fn class_modifiers() -> &'static [(&'static str, i32)] { unsafe { CLASS_MODIFIERS } }
/// 类 → 嵌套元数据。
pub fn class_nest() -> &'static [(&'static str, NestMeta)] { unsafe { CLASS_NEST } }
/// 类 → 直接超接口。
pub fn class_interfaces() -> &'static [(&'static str, Names)] { unsafe { CLASS_INTERFACES } }
/// 类 → 类级注解原始字节 + 稀疏常量池。
pub fn class_anno() -> &'static [(&'static str, &'static [u8], &'static [(i32, CpVal)])] { unsafe { CLASS_ANNO } }
/// 含 <clinit> 的类集。
pub fn clinit_classes() -> Names { unsafe { CLINIT_CLASSES } }
/// sealed 类 → 许可子类型。
pub fn permitted_subclasses() -> &'static [(&'static str, Names)] { unsafe { PERMITTED_SUBCLASSES } }
/// record 类集。
pub fn record_classes() -> Names { unsafe { RECORD_CLASSES } }
/// record 类 → 分量（名、描述符、泛型签名）。
pub fn record_components() -> &'static [(&'static str, &'static [(&'static str, &'static str, &'static str)])] {
    unsafe { RECORD_COMPONENTS }
}
/// 模块服务 (服务, provider)：闭包事实 seeds.module_services（发射层写入 java_meta），事实序。
pub fn module_services() -> &'static [(&'static str, &'static str)] { unsafe { MODULE_SERVICES } }
/// VM 初始系统属性的常量键（键, 值）：闭包事实 system_properties.values，与分析器折叠同源。
pub fn vm_const_properties() -> &'static [(&'static str, &'static str)] { unsafe { VM_CONST_PROPERTIES } }
/// VM 初始系统属性的动态键（由手写层取宿主值）：闭包事实 system_properties.dynamic。
pub fn vm_dynamic_properties() -> &'static [&'static str] { unsafe { VM_DYNAMIC_PROPERTIES } }
