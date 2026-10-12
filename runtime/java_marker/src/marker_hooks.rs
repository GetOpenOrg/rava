//! 标记层基础的行为钩子声明（§9.9 K4 阶段 3）：本 crate 各模块对行为层（`java_runtime`）
//! 依赖的全部 `unsafe extern "Rust"` 链接期声明。定义在对侧 `java_runtime/src/marker_hooks.rs`
//! （`#[export_name]`）；签名取自同一类型，漂移即链接失败（与 `rava_layer` 拆层、阶段 1 的
//! region / monitor 两钩子同一机制）。`safe`：定义侧是普通安全函数，调用方照常直接调用。
//!
//! 分组（见 S7 计划 §9.9「K4 实施」阶段 3 的搬移清单）：
//! - `error_*`：VM 异常构造器（`JvmError` 的标记面构造函数，行为侧构造翻译后的异常对象）
//!   与未捕获报告描述；
//! - `release` / `stack_check`：对象释放计深（`handle::__release`）与接口分派入口的栈界检查；
//! - `object_*`：`ObjectVTable` 缺省体依赖的根类字节码方法体（equals / toString / wait×3 /
//!   finalize——行为侧包装消 Java `String` 签名依赖）；
//! - `monitor_*`：对象监视器五入口（wait_timeout / notify / notify_all / enter / exit）；
//! - `class_*` / `string_*` / `meta_java_name`：类镜像（`Class::for_class` / `__mirror_hash` /
//!   `__name_assignable` / `getName`）、Java 字符串构造、binary name 排版——签名引用生成类
//!   的按标记类型书写；
//! - `safepoint` / `native_*`：安全点钩子与字节序位形原语（数组字节视图用）。

use crate::error::{JvmError, Result};
use crate::java::lang::{Class, Object, String};
use crate::obj_ref::__Obj;

unsafe extern "Rust" {
    // ── VM 异常构造器（error.rs 标记面）───────────────────────────────────────
    #[link_name = "__rava_marker_error_null_pointer"]
    pub safe fn __error_null_pointer() -> JvmError;
    #[link_name = "__rava_marker_error_array_index_out_of_bounds"]
    pub safe fn __error_array_index_out_of_bounds(index: i32, length: i32) -> JvmError;
    #[link_name = "__rava_marker_error_array_index_out_of_bounds_message"]
    pub safe fn __error_array_index_out_of_bounds_message(message: std::string::String) -> JvmError;
    #[link_name = "__rava_marker_error_index_out_of_bounds"]
    pub safe fn __error_index_out_of_bounds(message: std::string::String) -> JvmError;
    #[link_name = "__rava_marker_error_string_index_out_of_bounds"]
    pub safe fn __error_string_index_out_of_bounds(message: std::string::String) -> JvmError;
    #[link_name = "__rava_marker_error_negative_array_size"]
    pub safe fn __error_negative_array_size(size: i32) -> JvmError;
    #[link_name = "__rava_marker_error_arithmetic"]
    pub safe fn __error_arithmetic(message: &str) -> JvmError;
    #[link_name = "__rava_marker_error_class_cast"]
    pub safe fn __error_class_cast(message: std::string::String) -> JvmError;
    #[link_name = "__rava_marker_error_array_store"]
    pub safe fn __error_array_store(value_class: &str) -> JvmError;
    #[link_name = "__rava_marker_error_clone_not_supported"]
    pub safe fn __error_clone_not_supported(binary_name: &str) -> JvmError;
    #[link_name = "__rava_marker_error_illegal_monitor_state"]
    pub safe fn __error_illegal_monitor_state(message: &str) -> JvmError;
    #[link_name = "__rava_marker_error_illegal_argument"]
    pub safe fn __error_illegal_argument(message: &str) -> JvmError;
    #[link_name = "__rava_marker_error_interrupted"]
    pub safe fn __error_interrupted(message: Option<&str>) -> JvmError;
    #[link_name = "__rava_marker_error_out_of_memory"]
    pub safe fn __error_out_of_memory(message: &str) -> JvmError;
    #[link_name = "__rava_marker_error_stack_overflow"]
    pub safe fn __error_stack_overflow() -> JvmError;
    #[link_name = "__rava_marker_error_no_class_def_found"]
    pub safe fn __error_no_class_def_found(binary_name: &str) -> JvmError;
    #[link_name = "__rava_marker_error_class_not_found_at_resolution"]
    pub safe fn __error_class_not_found_at_resolution(internal_name: &str) -> JvmError;
    #[link_name = "__rava_marker_error_in_initializer"]
    pub safe fn __error_in_initializer(cause: JvmError) -> JvmError;
    /// 未捕获异常报告的首行（`JvmError::describe` 的行为面：读异常对象的 `getMessage`）。
    #[link_name = "__rava_marker_error_describe"]
    pub safe fn __error_describe(e: &JvmError) -> std::string::String;

    // ── 释放计深 / 栈界检查（handle.rs）───────────────────────────────────────
    #[link_name = "__rava_marker_release"]
    pub safe fn __release(rc: __Obj<dyn crate::java::lang::ObjectVTable>);
    #[link_name = "__rava_marker_stack_check"]
    pub safe fn __stack_check() -> Result<()>;

    // ── 根类字节码方法体（ObjectVTable 缺省体与根类落点）──────────────────────
    #[link_name = "__rava_marker_object_equals"]
    pub safe fn __object_equals(this: &Object, other: Object) -> Result<bool>;
    #[link_name = "__rava_marker_object_to_string_std"]
    pub safe fn __object_to_string_std(this: &Object) -> Result<std::string::String>;
    #[link_name = "__rava_marker_object_to_string_java"]
    pub safe fn __object_to_string_java(this: &Object) -> Result<String>;
    #[link_name = "__rava_marker_object_wait"]
    pub safe fn __object_wait(this: &Object, millis: i64, nanos: i32) -> Result<()>;
    #[link_name = "__rava_marker_object_finalize"]
    pub safe fn __object_finalize(this: &Object) -> Result<()>;

    // ── 对象监视器（ObjectVTable 的 wait/notify/monitorenter/monitorexit）─────
    #[link_name = "__rava_marker_monitor_wait_timeout"]
    pub safe fn __monitor_wait_timeout(identity: usize, millis: i64, nanos: i32) -> Result<()>;
    #[link_name = "__rava_marker_monitor_notify"]
    pub safe fn __monitor_notify(identity: usize) -> Result<()>;
    #[link_name = "__rava_marker_monitor_notify_all"]
    pub safe fn __monitor_notify_all(identity: usize) -> Result<()>;
    #[link_name = "__rava_marker_monitor_enter"]
    pub safe fn __monitor_enter(identity: usize) -> Result<()>;
    #[link_name = "__rava_marker_monitor_exit"]
    pub safe fn __monitor_exit(identity: usize) -> Result<()>;

    // ── 类镜像 / 类名 / 元数据（ObjectVTable::getClass、数组系统、checkcast 消息）─
    #[link_name = "__rava_marker_class_for_class"]
    pub safe fn __class_for_class(binary_name: &str) -> Class;
    #[link_name = "__rava_marker_class_mirror_hash"]
    pub safe fn __class_mirror_hash(id: *const ()) -> Option<i32>;
    #[link_name = "__rava_marker_class_name"]
    pub safe fn __class_name(c: &Class) -> Result<std::string::String>;
    #[link_name = "__rava_marker_class_name_assignable"]
    pub safe fn __class_name_assignable(target: &str, source: &str) -> bool;
    #[link_name = "__rava_marker_string_from_str"]
    pub safe fn __string_from_str(s: &str) -> String;
    #[link_name = "__rava_marker_string_from_std"]
    pub safe fn __string_from_std(s: std::string::String) -> String;
    #[link_name = "__rava_marker_meta_java_name"]
    pub safe fn __meta_java_name(binary_name: &str) -> std::string::String;

    // ── 安全点 / 字节序位形（数组元素存取与字节视图）──────────────────────────
    #[link_name = "__rava_marker_safepoint"]
    pub safe fn __safepoint();
    #[link_name = "__rava_marker_native_store_bits"]
    pub safe fn __native_store_bits(dst: &mut [u8], bits: u64);
    #[link_name = "__rava_marker_native_load_bits"]
    pub safe fn __native_load_bits(src: &[u8]) -> u64;
}
