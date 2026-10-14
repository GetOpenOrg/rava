//! 标记层基础的行为钩子定义（§9.9 K4）：`java_marker` 中 `unsafe extern "Rust"` 声明的
//! 对侧定义（`#[export_name]`）。签名与声明取自同一类型（`java_marker::obj_ref::group::Ops`），
//! 漂移即链接失败。最终二进制链接两侧（与 `rava_layer` 拆层同一机制）。

/// 区域整组释放的分配登记（`obj_ref` 分配路径调用）：见 `region::__on_alloc`。
#[export_name = "__rava_marker_region_on_alloc"]
pub fn __region_on_alloc(value: *mut u8, ops: &'static java_marker::obj_ref::group::Ops) -> bool {
    crate::region::__on_alloc(value, ops)
}

/// 对象曾进入监视器，释放最后一个强引用时摘身份侧表条目：见 `monitor::forget`。
#[export_name = "__rava_marker_monitor_forget"]
pub fn __monitor_forget(identity: usize) {
    crate::monitor::forget(identity)
}
