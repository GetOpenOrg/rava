//! 编译期逃逸分析与区域整组释放规划（无 GC 第一步，`docs/plans/2026-10-07-no-gc-memory-model.md` §4.5）。
//!
//! 对用户方法里的语句段求证：段内分配的对象在段结束后不被段外引用。成立且这些对象成环时，发射层
//! 用 `java_scope!` 包住该段，运行时在段的正常出口整组释放（环也一并回收）。不做追踪式 GC、不做运行期环检测。

mod analysis;
mod cycle;
mod flow;
mod indy;
mod plan;
mod step;
mod val;
mod world;

pub use plan::plan_regions;
pub use world::{World, WorldCache};
