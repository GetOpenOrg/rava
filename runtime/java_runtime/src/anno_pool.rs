//! 类级注解原始字节 + 注解引用的稀疏常量池（FS-R R4b，方案 `docs/plans/2026-09-27-reflection-metadata-table.md` §2.6）。
//!
//! 数据流：codegen 保留 RuntimeVisibleAnnotations 原始属性体，并收集其（及字段 / 方法注解、
//! AnnotationDefault）引用的常量池条目 → java_class! 属性 `raw_annotations` / `anno_cpool` →
//! build.rs 造表（OUT_DIR/class_anno_table.rs）→ 本模块查询面。消费方：Class.getRawAnnotations、
//! ConstantPool natives（constant_pool_impl.rs）；解析本身是翻译的 JDK AnnotationParser。

mod table {
    include!(concat!(env!("OUT_DIR"), "/class_anno_table.rs"));
}

pub use table::CpVal;

/// 类级 RuntimeVisibleAnnotations 原始属性体（空 = 无）。
pub fn class_annotations(class_slash: &str) -> &'static [u8] {
    table::CLASS_ANNO.iter()
        .find(|(n, _, _)| *n == class_slash)
        .map(|(_, raw, _)| *raw)
        .unwrap_or(&[])
}

/// 稀疏常量池条目（原索引）。
pub fn cp_entry(class_slash: &str, index: i32) -> Option<&'static CpVal> {
    table::CLASS_ANNO.iter()
        .find(|(n, _, _)| *n == class_slash)
        .and_then(|(_, _, cp)| cp.iter().find(|(i, _)| *i == index).map(|(_, v)| v))
}
