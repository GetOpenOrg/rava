//! 发射层（P5a）：类文件骨架与 Cargo workspace 工程发射。
//!
//! 输入是 [`input::EmitInput`]（闭包事实 + registry + 清单），输出是 scratch 下的
//! `java_runtime/src/**` 生成类文件、包 mod 树、`user/` crate 与 workspace 根。方法体经
//! [`body::MethodBodyEmitter`] 取得：[`method_bodies::MethodBodies`] 以 P4c `method` crate 实现（P5b）。
//!
//! 结构：
//! - [`ctx`]：只读发射上下文 [`ctx::EmitCtx`] 与每项目可变状态 [`ctx::ProjectState`]
//!   （Python 模块级全局账本的显式化）；
//! - [`project`]：布局、落盘（手写文件永不覆写）、mod 树、main.rs / Cargo.toml、overlay；
//! - [`class_writer`]：单类文件文本；
//! - [`phase2`]：全部类文本生成后的第二阶段收尾（接口实现 / upcast / 继承成员插入位填充）。
//!
//! # C3 接入点（levels / dispatch / folds 消费，P5 切换后在本 crate 实施）
//! - **levels**：`class_writer` 的类块头 / struct 发射处按类的 level 选择 L1 不透明类型形态
//!   （无字段、无方法体，仅类型身份）；`project::layout` 的 JDK 布局按 level 过滤生成集。
//! - **dispatch**：vtable 槽发射（步骤 (d) 的 vtable / 继承段）按闭包导出的 `dispatch` 事实
//!   决定槽位集合，替代「祖先声明全部虚方法」的保守集合。
//! - **folds**：方法体侧消费（`EmitInput::code` 已给出折叠后的规范化方法体）；发射层只在
//!   `<clinit>` 静态字段初值提取处消费常量折叠结果。

pub mod audit;
pub mod body;
pub mod class_writer;
pub mod ctx;
pub mod emission;
pub mod error;
pub mod fallback;
pub mod imports;
pub mod lang;
pub mod method_bodies;
pub mod par;
pub mod perf;
pub mod phase2;
pub mod precheck;
pub mod project;
pub mod sam;
pub mod scan;
pub mod text;
pub mod vtable;
pub mod vtable_prune;

pub use body::{BodyEffects, BodyError, BodyOutput, BodyRequest, MethodBodyEmitter};
pub use ctx::{EmitCtx, EmitOptions, ProjectState};
pub use emission::ClassEmission;
pub use error::{EmitError, Result};
pub use method_bodies::MethodBodies;
pub use project::{prepare_scratch, scan_gaps, write_project};
