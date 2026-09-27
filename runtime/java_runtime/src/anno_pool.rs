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

/// RuntimeVisibleAnnotations 原始属性体中是否含类型描述符 `type_desc` 的注解（按类 `class_slash`
/// 的稀疏常量池解析 type_index）。JVMS §4.7.16 结构遍历，只读类型、跳过元素值。
pub fn has_annotation(class_slash: &str, raw: &[u8], type_desc: &str) -> bool {
    struct R<'a> { b: &'a [u8], i: usize }
    impl R<'_> {
        fn u1(&mut self) -> Option<u8> { let v = *self.b.get(self.i)?; self.i += 1; Some(v) }
        fn u2(&mut self) -> Option<u16> { Some(((self.u1()? as u16) << 8) | self.u1()? as u16) }
        fn skip_value(&mut self) -> Option<()> {
            match self.u1()? {
                b'e' => { self.u2()?; self.u2()?; }
                b'@' => { self.skip_anno()?; }
                b'[' => { for _ in 0..self.u2()? { self.skip_value()?; } }
                _ => { self.u2()?; }
            }
            Some(())
        }
        fn skip_anno(&mut self) -> Option<u16> {
            let ty = self.u2()?;
            for _ in 0..self.u2()? { self.u2()?; self.skip_value()?; }
            Some(ty)
        }
    }
    let mut r = R { b: raw, i: 0 };
    let Some(n) = r.u2() else { return false };
    for _ in 0..n {
        let Some(ty) = r.skip_anno() else { return false };
        if let Some(CpVal::U(s)) = cp_entry(class_slash, ty as i32) {
            if *s == type_desc {
                return true;
            }
        }
    }
    false
}
