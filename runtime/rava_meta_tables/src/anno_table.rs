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

pub(crate) fn render_class_anno_table(g: &mut Group, entries: &BTreeMap<String, (Vec<u8>, String)>) {
    // CLASS_ANNO 行：类, 注解原始字节, [(常量池下标, 标签, 值)]——Class.getRawAnnotations、ConstantPool natives。
    // 标签：0 U（池串）/ 1 W（长度 + UTF-16 码元）/ 2 I / 3 J（zigzag）/ 4 F / 5 D（位模式）
    let (s, p) = g.table("CLASS_ANNO");
    for (name, (raw, cp)) in entries {
        let mut ents: Vec<(i32, &str, &str)> = Vec::new();
        for e in cp.split(';').filter(|x| !x.is_empty()) {
            let mut it = e.splitn(3, ':');
            let (Some(idx), Some(k), Some(v)) = (it.next(), it.next(), it.next()) else { continue };
            let Ok(idx) = idx.parse::<i32>() else { continue };
            if matches!(k, "U" | "W" | "I" | "J" | "F" | "D") {
                ents.push((idx, k, v));
            }
        }
        s.str(p, name);
        s.bytes(p, raw);
        s.len(ents.len());
        for (idx, k, v) in ents {
            s.i32(idx);
            match k {
                "U" => {
                    s.u32(0);
                    s.str(p, &String::from_utf8_lossy(&hex_bytes(v)));
                }
                // 含孤立代理项的字符串：UTF-16 码元（每码元 4 位 hex）原样承载
                "W" => {
                    s.u32(1);
                    let units: Vec<u32> = v.as_bytes().chunks(4)
                        .map(|c| u32::from_str_radix(std::str::from_utf8(c).unwrap_or("0"), 16).unwrap_or(0)).collect();
                    s.len(units.len());
                    for u in units {
                        s.u32(u);
                    }
                }
                "I" => {
                    s.u32(2);
                    s.i32(v.parse().unwrap_or(0));
                }
                "J" => {
                    s.u32(3);
                    s.i64(v.parse().unwrap_or(0));
                }
                "F" => {
                    s.u32(4);
                    s.u32(u32::from_str_radix(v, 16).unwrap_or(0));
                }
                _ => {
                    s.u32(5);
                    s.u64(u64::from_str_radix(v, 16).unwrap_or(0));
                }
            }
        }
    }
}
