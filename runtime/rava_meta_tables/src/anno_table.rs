//! 类级注解原始字节 + 注解引用的稀疏常量池表（FS-R R4b）。

use super::*;

/// 类级注解原始字节 + 稀疏常量池（FS-R R4b）：数据源 `raw_annotations` / `anno_cpool`
/// 类属性（classfile.encode_anno_cpool 编码）。消费方：Class.getRawAnnotations /
/// ConstantPool natives（getUTF8At0 / getIntAt0 …，按原常量池索引）。
pub(crate) fn scan_class_annos(texts: &[&str]) -> BTreeMap<String, (Vec<u8>, String)> {
    let mut result: BTreeMap<String, (Vec<u8>, String)> = BTreeMap::new();
    for content in texts.iter().copied() {
        let mut current = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(name) = extract_attr_padded(trimmed, "binary_name") {
                current = name;
                continue;
            }
            if current.is_empty() || !trimmed.starts_with("#[") { continue; }
            if let Some(h) = extract_attr_padded(trimmed, "raw_annotations") {
                if trimmed.starts_with("#[raw_annotations") {
                    result.entry(current.clone()).or_default().0 = hex_bytes(&h);
                }
            }
            if let Some(cp) = extract_attr_padded(trimmed, "anno_cpool") {
                result.entry(current.clone()).or_default().1 = cp;
            }
        }
    }
    result
}

pub(crate) fn render_class_anno_table(entries: &BTreeMap<String, (Vec<u8>, String)>) -> String {
    let mut out = String::from(
        "// 由生成器（rava_meta_tables）生成：类级注解原始字节 + 注解引用的稀疏常量池（FS-R R4b）。
         // 消费方：Class.getRawAnnotations、ConstantPool natives。请勿手改。


         #[export_name = \"__java_meta_CLASS_ANNO\"] pub static CLASS_ANNO: &[(&str, &[u8], &[(i32, CpVal)])] = &[
",
    );
    for (name, (raw, cp)) in entries {
        let mut ents: Vec<String> = Vec::new();
        for e in cp.split(';').filter(|x| !x.is_empty()) {
            let mut it = e.splitn(3, ':');
            let (Some(idx), Some(k), Some(v)) = (it.next(), it.next(), it.next()) else { continue };
            let val = match k {
                "U" => format!("CpVal::U({:?})", std::string::String::from_utf8_lossy(&hex_bytes(v))),
                // 含孤立代理项的字符串：UTF-16 码元（每码元 4 位 hex）原样承载
                "W" => format!(
                    "CpVal::W(&[{}])",
                    v.as_bytes().chunks(4).map(|c| format!("0x{}", std::str::from_utf8(c).unwrap_or("0"))).collect::<Vec<_>>().join(", ")
                ),
                "I" => format!("CpVal::I({}i32)", v),
                "J" => format!("CpVal::J({}i64)", v),
                "F" => format!("CpVal::F(f32::from_bits(0x{}))", v),
                "D" => format!("CpVal::D(f64::from_bits(0x{}))", v),
                _ => continue,
            };
            ents.push(format!("({}, {})", idx, val));
        }
        out.push_str(&format!("    ({:?}, &{:?}, &[{}]),\n", name, raw, ents.join(", ")));
    }
    out.push_str("];\n");
    out
}
