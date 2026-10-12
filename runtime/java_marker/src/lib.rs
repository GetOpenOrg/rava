//! 标记层基础（S7 计划 §9.9 K4）：与签名无关的运行时基础设施。
//!
//! 本 crate 承载 `java_class!` 标记层（`<stem>_m.rs`，K6 起由生成器发射到本 crate）依赖的全部
//! 手写真源：对象引用计数指针 `obj_ref`、同步模型原语 `sync_model`、类 / 字段描述符
//! （`class_desc` / `field_desc`）、对象句柄（`handle`）、异常载体（`error` 的标记面）、
//! 数组系统（`array`）、`java/lang/Object` 三件（`object` / `object_ext` / `object_impl`）与
//! Java 浮点排版（`java_fmt`）。不依赖任何声明层 crate，可置于声明链之下（`java_runtime`
//! 反向依赖本 crate，见 §9.9 K4 阶段 3）。
//!
//! 行为面依赖（异常对象构造、根类方法体、监视器、类镜像表、元数据名字、栈界检查、释放
//! 计深、安全点、字节序原语等）经 `unsafe extern "Rust"` 链接期声明转交行为层
//! （`java_runtime` 侧 `marker_hooks.rs` 的 `#[export_name]` 定义，签名取自同一类型 →
//! 漂移即链接失败），与 `rava_layer` 拆层同一机制。引用生成类型（`java/lang/Class` /
//! `String` 等）的签名按标记文件形态书写——这些类型只在本 crate 的 scratch 构建中存在，
//! 故真源 standalone `cargo check` 不是绿色口径（§9.9 K4 阶段 3：以 scratch 全构建 +
//! 宏单测 + generator check 为口径）。
//!
//! `__process_static!` / `__DynFn!` 两个宏随 sync_model 落在本 crate 根，`java_runtime`
//! 再导出以保持 `crate::__process_static!` 等旧路径。

pub mod array;
pub mod class_desc;
pub mod error;
pub mod field_desc;
pub mod handle;
pub mod java_fmt;
pub mod marker_hooks;
pub mod obj_ref;
pub mod sync_model;
// `java/`（java/lang/Object 三件）不在真源 lib.rs 声明：scratch 中该包树由生成标记文件与
// 手写文件共同构成（mod.rs 由 mod 树按磁盘生成，lib.rs 由 complete_lib_rs 补顶层声明）；
// 真源 standalone 编译不覆盖 java/（本 crate 的绿色口径是 scratch 全构建，见模块头注）。

/// JVM null 检查（ifnull / ifnonnull 与 athrow 判空辅助，自 java_runtime lib.rs 随
/// `JvmError::from` 的判空迁入）：`Object` 载体的 vtable `is_jvm_null()` 是唯一判定
///（S-3.1 后统一），非 Object 载体恒 false。java_runtime 按原路径再导出。
#[inline(always)]
pub fn _is_jnull<T: 'static>(val: &T) -> bool {
    if let Some(obj) = (val as &dyn std::any::Any).downcast_ref::<Object>() {
        obj.0.is_jvm_null()
    } else {
        false
    }
}

/// 类型变量操作数的 null 判定（ifnull / ifnonnull，FS-M8；自 java_runtime lib.rs 随迁）：
/// `T` 实例化为任意引用载体（wrapper / 数组 / 接口载体 / Object）时都经 `Into<Object>`
/// 落到 vtable 的 `is_jvm_null` 唯一判定。`_is_jnull` 只识别 `Object` 载体，对
/// `T = String` 等实例化恒假。
#[inline]
pub fn _is_jnull_ref<T: Clone + Into<java::lang::Object>>(val: &T) -> bool {
    Into::<java::lang::Object>::into(Clone::clone(val)).0.is_jvm_null()
}

/// prelude：标记文件头 `use crate::prelude::*;` 的名字面（§9.9 K4 阶段 3）。只含标记层
/// 基础（与 `java_class!` 标记展开引用的名字一一对应）；方法 trait（`__Methods`）与
/// vtable trait（`__VTable`）不在其中——标记层不含方法面（宏单测
/// `marker_layer_is_signature_free` 守护）。`jvm_ext` / `jvm_native` 供随迁的
/// `object_ext` / `object_impl`（生成标记文件不用宏属性，多留不碍）。
pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::array::JArray;
    pub use crate::class_desc::__ClassDesc;
    pub use crate::error::{JvmError, Result};
    pub use crate::field_desc::{__FieldDesc, __FieldKind, __RefAccess};
    pub use crate::handle::{__Handle, __IfaceRef};
    pub use crate::java::lang::{Object, ObjectVTable, JvmRef, __TypedNull};
    pub use crate::java::lang::{__class_from_object, __erased_view, __iface_missing, __PartsFn};
    pub use crate::obj_ref::__Obj;
    pub use crate::sync_model::{__AnyRef, __PrimCell, __RefField, __RefSlot, __Shared, __ThreadSafe};
    pub use crate::java_fmt::{java_fmt_char, java_fmt_f32, java_fmt_f64};
    pub use crate::_is_jnull;
    pub use rava_macros::{jvm_ext, jvm_native};
}
