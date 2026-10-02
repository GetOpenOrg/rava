//! 反射元数据表（类层次 / 直接父类 / 字段 / 方法 / 修饰符 / record / 类文件级 / 嵌套 /
//! 直接超接口 / 类级注解 / 模块服务）。表由构建脚本从整个 workspace（java_runtime、user、lib crate）
//! 的 java_class! 属性生成，每个 static 以 `__java_meta_<表名>` 符号导出，由
//! java_runtime::meta 的同名 extern 声明读取。本 crate 只承载数据，不含逻辑。

use java_runtime::meta::{CpVal, FieldMeta, MethodMeta, NestMeta};

include!(concat!(env!("OUT_DIR"), "/hierarchy_table.rs"));
include!(concat!(env!("OUT_DIR"), "/direct_super_table.rs"));
include!(concat!(env!("OUT_DIR"), "/field_table.rs"));
include!(concat!(env!("OUT_DIR"), "/method_table.rs"));
include!(concat!(env!("OUT_DIR"), "/modifiers_table.rs"));
include!(concat!(env!("OUT_DIR"), "/class_meta_table.rs"));
include!(concat!(env!("OUT_DIR"), "/record_table.rs"));
include!(concat!(env!("OUT_DIR"), "/nest_table.rs"));
include!(concat!(env!("OUT_DIR"), "/interfaces_table.rs"));
include!(concat!(env!("OUT_DIR"), "/class_anno_table.rs"));
// 闭包派生表（模块服务 / VM 初始系统属性）：发射层每次构建写入 scratch 的 closure_input/（与本 crate 同级）
include!("../../closure_input/closure_tables.rs");
// Java 栈帧行表（FS-E1）：发射层扫描落盘文本的行标记写入 closure_input/
include!("../../closure_input/line_tables.rs");
