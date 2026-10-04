//! 档案侧反射元数据表（类层次 / 直接父类 / 字段 / 方法 / 修饰符 / record / 类文件级 / 嵌套 /
//! 直接超接口 / 类级注解 / 模块服务 / VM 初始系统属性 / 栈帧行表）。
//!
//! 表源码由发射层写入 scratch 的 `closure_input/`（与本 crate 同级），只含档案侧（JDK 与 lib crate）的类：
//! 同一档案下与用户程序无关，本 crate 属于档案。每个 static 以 `__java_meta_<表名>` 符号导出，由
//! java_runtime::meta 的同名 extern 声明读取；用户类的行生成在用户 crate，启动时登记
//!（`java_runtime::meta::register_user`）。本 crate 只承载数据，不含逻辑。

use java_runtime::meta::{CpVal, FieldMeta, MethodMeta, NestMeta};

// 反射元数据表：发射层扫描档案侧生成文件渲染（rava_meta_tables）
include!("../../closure_input/meta_tables.rs");
// 闭包派生表（模块服务 / VM 初始系统属性）
include!("../../closure_input/closure_tables.rs");
// Java 栈帧行表（FS-E1）：发射层扫描落盘文本的行标记
include!("../../closure_input/line_tables.rs");
