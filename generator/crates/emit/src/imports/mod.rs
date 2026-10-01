//! 引用收集与 `use` 行生成（← `emitter/import_gen.py`）。
//!
//! - [`referenced`]：本类文件需引用的类型集合（`collect_referenced`）；
//! - [`cross`]：跨类导入（引用记录按 binary 生成，含 `__VTable` / `__base` 导入）；
//! - [`scan`]：按发射文本补导（`scan_used_vtable_imports` / `scan_supplementary_iface_imports`）；
//! - [`base_fn`]：`__base` 函数命名所需的 invokespecial 解析（instr 辅助的私有最小移植）；
//! - [`refs`]：描述符 / 签名文本的类引用抽取。

pub mod cross;
pub mod referenced;
pub mod refs;
pub mod scan;

pub use cross::{cross_imports, CrateRoute, CrossInput, Prefix};
pub use referenced::collect_referenced;
pub use scan::{supplementary_iface_imports, used_vtable_imports};
