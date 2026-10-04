//! 引用收集与 `use` 行生成（← `emitter/import_gen.py`）。
//!
//! - [`referenced`]：本类文件需引用的类型集合（`collect_referenced`）；
//! - [`cross`]：结构化引用在文件作用域的预认领（含 `__VTable` / `__base` 派生名登记）与 crate 定向；
//! - [`fill`]：文件作用域认领记录 → 导入块；
//! - [`refs`]：描述符 / 签名文本的类引用抽取。

pub mod cross;
pub mod referenced;
pub mod fill;
pub mod refs;

pub use cross::{claim_structural, CrateRoute, CrossInput};
pub use fill::{import_lines, ImportSite};
pub use referenced::collect_referenced;
