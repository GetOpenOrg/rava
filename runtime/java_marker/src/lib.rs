//! 标记层基础（S7 计划 §9.9 K4）：与签名无关的运行时基础设施。
//!
//! 本 crate 是独立 `java_marker` crate 的「标记层基础」部分：只承载与 Java 行为无关的
//! 基础设施（对象引用计数指针 `obj_ref`、同步模型原语 `sync_model`），不依赖任何
//! JDK / 声明层 crate，可置于声明链之下（`java_runtime` 反向依赖本 crate）。行为层
//! 依赖（区域整组释放的登记钩子、监视器侧表摘除）经 `unsafe extern "Rust"` 链接期
//! 声明转交行为层（`java_runtime` 侧同名符号定义，与 `rava_layer` 拆层同一机制）。
//!
//! K6 的类标记文件（`<stem>_m.rs`）后续整体搬入本 crate（届时只剩文件搬移与路径
//! 前缀，见 §9.9 K6 偏差注记）。`__process_static!` / `__DynFn!` 两个宏随 sync_model
//! 落在本 crate 根，`java_runtime` 再导出以保持 `crate::__process_static!` 等旧路径。

pub mod obj_ref;
pub mod sync_model;
