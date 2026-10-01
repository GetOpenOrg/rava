//! 发射层所需的补充属性（纯增量；[`crate::parse`] 的结构不变）。
//!
//! 闭包分析不需要、而类文件发射需要逐字节复述的属性，在这里对同一份 `.class` 字节
//! 二次解析得到：
//! - 局部变量表（LVT / LVTT，形参名与局部变量声明）；
//! - 行号表（LineNumberTable，Java 栈帧的行号来源）；
//! - 注解原始字节（RuntimeVisibleAnnotations / RuntimeVisibleParameterAnnotations /
//!   AnnotationDefault）与其引用的稀疏常量池；
//! - `Deprecated` 属性；
//! - Record 组件的 Signature。
//!
//! 字段 / 方法按类文件声明序与 [`crate::ClassFile`] 的 `fields` / `methods` 一一对应。

use std::collections::{BTreeMap, BTreeSet};

use crate::constant::{ConstantPool, CpEntry};
use crate::reader::Reader;
use crate::Error;

/// 局部变量表条目（LVT 一条 + 同 (slot, start, name) 的 LVTT 签名）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVar {
    pub slot: u16,
    pub start: u16,
    pub len: u16,
    pub name: String,
    pub desc: String,
    /// LVTT 泛型签名（无 → 空串）
    pub signature: String,
}

/// LVTT 原始条目
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTypeEntry {
    pub name: String,
    pub signature: String,
    pub slot: u16,
    pub start: u16,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FieldExtras {
    pub deprecated: bool,
    /// RuntimeVisibleAnnotations 属性体（无 → 空）
    pub raw_annotations: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MethodExtras {
    pub deprecated: bool,
    pub raw_annotations: Vec<u8>,
    pub raw_param_annotations: Vec<u8>,
    pub raw_annotation_default: Vec<u8>,
    /// LVT 条目（属性内出现序）
    pub local_vars: Vec<LocalVar>,
    /// LVTT 条目（属性内出现序）
    pub local_type_entries: Vec<LocalTypeEntry>,
    /// 行号表 (起始 pc, 源行)，按 pc 升序（多个 LineNumberTable 属性合并）
    pub line_numbers: Vec<(u16, u16)>,
}

impl MethodExtras {
    /// pc 所在的源行：起始 pc ≤ `pc` 的最后一条（JVMS §4.7.12；表空或 pc 在首条之前 → None）
    pub fn line_at(&self, pc: u32) -> Option<u16> {
        let i = self.line_numbers.partition_point(|&(start, _)| u32::from(start) <= pc);
        i.checked_sub(1).map(|i| self.line_numbers[i].1)
    }

    /// slot → 代表名：作用域最长的 LVT 条目（等长取先出现者）
    pub fn local_names(&self) -> BTreeMap<u16, String> {
        let mut best: BTreeMap<u16, (u16, &str)> = BTreeMap::new();
        for v in &self.local_vars {
            match best.get(&v.slot) {
                Some((len, _)) if v.len <= *len => {}
                _ => {
                    best.insert(v.slot, (v.len, &v.name));
                }
            }
        }
        best.into_iter().map(|(s, (_, n))| (s, n.to_string())).collect()
    }

    /// slot → (LVTT 签名, 起始 pc)：LVTT 名与该 slot 任一 LVT 名一致的首条
    pub fn local_types(&self) -> BTreeMap<u16, (String, u16)> {
        let mut names: BTreeMap<u16, BTreeSet<&str>> = BTreeMap::new();
        for v in &self.local_vars {
            names.entry(v.slot).or_default().insert(&v.name);
        }
        let mut out = BTreeMap::new();
        for e in &self.local_type_entries {
            if out.contains_key(&e.slot) {
                continue;
            }
            if names.get(&e.slot).is_some_and(|s| s.contains(e.name.as_str())) {
                out.insert(e.slot, (e.signature.clone(), e.start));
            }
        }
        out
    }
}

/// 注解引用的常量池条目（稀疏表，原索引不变）
#[derive(Debug, Clone, PartialEq)]
pub enum AnnoConst {
    Utf8(String),
    /// 含孤立代理项的字符串（UTF-16 码元原样保留；合法文本一律走 `Utf8`）
    Utf16(Vec<u16>),
    Int(i32),
    Long(i64),
    /// IEEE 754 位模式
    Float(u32),
    Double(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordComponent {
    pub name: String,
    pub desc: String,
    /// 组件 Signature（无 → 空串）
    pub signature: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClassExtras {
    pub deprecated: bool,
    pub raw_annotations: Vec<u8>,
    /// 全部注解属性（类 / 字段 / 方法）引用的常量池条目，按索引排序
    pub anno_cpool: BTreeMap<u16, AnnoConst>,
    pub record_components: Vec<RecordComponent>,
    pub fields: Vec<FieldExtras>,
    pub methods: Vec<MethodExtras>,
}

enum AnnoForm {
    Annos,
    Params,
    Default,
}

/// 收集注解属性体引用的常量池索引（格式错误时保留已收集部分）
fn anno_refs(data: &[u8], form: AnnoForm, refs: &mut BTreeSet<u16>) {
    fn ev(r: &mut Reader, refs: &mut BTreeSet<u16>) -> Result<(), Error> {
        let tag = r.u1()?;
        match tag {
            b'B' | b'C' | b'I' | b'S' | b'Z' | b'J' | b'F' | b'D' | b's' | b'c' => {
                refs.insert(r.u2()?);
            }
            b'e' => {
                refs.insert(r.u2()?);
                refs.insert(r.u2()?);
            }
            b'@' => anno(r, refs)?,
            b'[' => {
                for _ in 0..r.u2()? {
                    ev(r, refs)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn anno(r: &mut Reader, refs: &mut BTreeSet<u16>) -> Result<(), Error> {
        refs.insert(r.u2()?);
        for _ in 0..r.u2()? {
            refs.insert(r.u2()?);
            ev(r, refs)?;
        }
        Ok(())
    }
    let mut r = Reader::new(data);
    let _ = (|| -> Result<(), Error> {
        match form {
            AnnoForm::Annos => {
                for _ in 0..r.u2()? {
                    anno(&mut r, refs)?;
                }
            }
            AnnoForm::Params => {
                for _ in 0..r.u1()? {
                    for _ in 0..r.u2()? {
                        anno(&mut r, refs)?;
                    }
                }
            }
            AnnoForm::Default => ev(&mut r, refs)?,
        }
        Ok(())
    })();
}

fn attribute<'a>(r: &mut Reader<'a>, pool: &'a ConstantPool) -> Result<(&'a str, &'a [u8]), Error> {
    let name = pool.utf8(r.u2()?)?;
    let len = r.u4()? as usize;
    Ok((name, r.bytes(len)?))
}

fn code_locals(body: &[u8], pool: &ConstantPool, m: &mut MethodExtras) -> Result<(), Error> {
    let mut r = Reader::new(body);
    r.skip(4)?;
    let len = r.u4()? as usize;
    r.skip(len)?;
    let n_exc = r.u2()? as usize;
    r.skip(n_exc * 8)?;
    let mut lvtt: Vec<LocalTypeEntry> = Vec::new();
    let mut lvt: Vec<(u16, u16, String, u16, String)> = Vec::new();
    for _ in 0..r.u2()? {
        let (name, sub) = attribute(&mut r, pool)?;
        if name == "LineNumberTable" {
            let mut sr = Reader::new(sub);
            for _ in 0..sr.u2()? {
                let pc = sr.u2()?;
                m.line_numbers.push((pc, sr.u2()?));
            }
            continue;
        }
        let is_lvt = name == "LocalVariableTable";
        if !is_lvt && name != "LocalVariableTypeTable" {
            continue;
        }
        let mut sr = Reader::new(sub);
        for _ in 0..sr.u2()? {
            let start = sr.u2()?;
            let vlen = sr.u2()?;
            let vname = pool.utf8(sr.u2()?)?.to_string();
            let d = pool.utf8(sr.u2()?)?.to_string();
            let slot = sr.u2()?;
            if is_lvt {
                lvt.push((start, vlen, vname, slot, d));
            } else {
                lvtt.push(LocalTypeEntry { name: vname, signature: d, slot, start });
            }
        }
    }
    m.local_vars = lvt
        .into_iter()
        .map(|(start, len, name, slot, desc)| {
            let signature = lvtt
                .iter()
                .rev()
                .find(|e| e.slot == slot && e.start == start && e.name == name)
                .map(|e| e.signature.clone())
                .unwrap_or_default();
            LocalVar { slot, start, len, name, desc, signature }
        })
        .collect();
    m.local_type_entries = lvtt;
    m.line_numbers.sort_by_key(|&(pc, _)| pc);
    Ok(())
}

fn anno_const(pool: &ConstantPool, idx: u16) -> Option<AnnoConst> {
    Some(match pool.get(idx).ok()? {
        CpEntry::Utf8(_, Some(units)) => AnnoConst::Utf16(units.to_vec()),
        CpEntry::Utf8(s, None) => AnnoConst::Utf8(s.clone()),
        CpEntry::Integer(v) => AnnoConst::Int(*v),
        CpEntry::Long(v) => AnnoConst::Long(*v),
        CpEntry::Float(v) => AnnoConst::Float(*v),
        CpEntry::Double(v) => AnnoConst::Double(*v),
        _ => return None,
    })
}

/// 二次解析 `.class` 字节，取发射层补充属性
pub fn parse_extras(data: &[u8]) -> Result<ClassExtras, Error> {
    let mut r = Reader::new(data);
    if r.u4()? != 0xCAFE_BABE {
        return Err(Error::BadMagic);
    }
    r.skip(4)?;
    let pool = ConstantPool::parse(&mut r)?;
    r.skip(6)?;
    let n_if = r.u2()? as usize;
    r.skip(n_if * 2)?;
    let mut refs: BTreeSet<u16> = BTreeSet::new();
    let mut out = ClassExtras::default();
    for _ in 0..r.u2()? {
        r.skip(6)?;
        let mut f = FieldExtras::default();
        for _ in 0..r.u2()? {
            let (name, body) = attribute(&mut r, &pool)?;
            match name {
                "Deprecated" => f.deprecated = true,
                "RuntimeVisibleAnnotations" => {
                    anno_refs(body, AnnoForm::Annos, &mut refs);
                    f.raw_annotations = body.to_vec();
                }
                _ => {}
            }
        }
        out.fields.push(f);
    }
    for _ in 0..r.u2()? {
        r.skip(6)?;
        let mut m = MethodExtras::default();
        for _ in 0..r.u2()? {
            let (name, body) = attribute(&mut r, &pool)?;
            match name {
                "Code" => code_locals(body, &pool, &mut m)?,
                "Deprecated" => m.deprecated = true,
                "RuntimeVisibleAnnotations" => {
                    anno_refs(body, AnnoForm::Annos, &mut refs);
                    m.raw_annotations = body.to_vec();
                }
                "RuntimeVisibleParameterAnnotations" => {
                    anno_refs(body, AnnoForm::Params, &mut refs);
                    m.raw_param_annotations = body.to_vec();
                }
                "AnnotationDefault" => {
                    anno_refs(body, AnnoForm::Default, &mut refs);
                    m.raw_annotation_default = body.to_vec();
                }
                _ => {}
            }
        }
        out.methods.push(m);
    }
    for _ in 0..r.u2()? {
        let (name, body) = attribute(&mut r, &pool)?;
        match name {
            "Deprecated" => out.deprecated = true,
            "RuntimeVisibleAnnotations" => {
                anno_refs(body, AnnoForm::Annos, &mut refs);
                out.raw_annotations = body.to_vec();
            }
            "Record" => {
                let mut rr = Reader::new(body);
                for _ in 0..rr.u2()? {
                    let cname = pool.utf8(rr.u2()?)?.to_string();
                    let cdesc = pool.utf8(rr.u2()?)?.to_string();
                    let mut sig = String::new();
                    for _ in 0..rr.u2()? {
                        let (an, ab) = attribute(&mut rr, &pool)?;
                        if an == "Signature" {
                            sig = pool.utf8(Reader::new(ab).u2()?)?.to_string();
                        }
                    }
                    out.record_components.push(RecordComponent { name: cname, desc: cdesc, signature: sig });
                }
            }
            _ => {}
        }
    }
    out.anno_cpool = refs.into_iter().filter_map(|i| anno_const(&pool, i).map(|c| (i, c))).collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lv(slot: u16, start: u16, len: u16, name: &str) -> LocalVar {
        LocalVar { slot, start, len, name: name.into(), desc: "I".into(), signature: String::new() }
    }

    #[test]
    fn local_names_longest_scope_first_wins() {
        let m = MethodExtras {
            local_vars: vec![lv(1, 0, 5, "a"), lv(1, 6, 9, "b"), lv(1, 20, 9, "c"), lv(2, 0, 1, "d")],
            ..Default::default()
        };
        let names = m.local_names();
        assert_eq!(names[&1], "b");
        assert_eq!(names[&2], "d");
    }

    #[test]
    fn local_types_match_any_lvt_name() {
        let m = MethodExtras {
            local_vars: vec![lv(3, 0, 50, "x"), lv(3, 60, 5, "tab")],
            local_type_entries: vec![
                LocalTypeEntry { name: "zz".into(), signature: "S0".into(), slot: 3, start: 0 },
                LocalTypeEntry { name: "tab".into(), signature: "S1".into(), slot: 3, start: 60 },
                LocalTypeEntry { name: "x".into(), signature: "S2".into(), slot: 3, start: 0 },
            ],
            ..Default::default()
        };
        assert_eq!(m.local_types()[&3], ("S1".to_string(), 60));
    }

    #[test]
    fn line_at_takes_last_entry_not_after_pc() {
        let m = MethodExtras { line_numbers: vec![(0, 10), (4, 11), (9, 13)], ..Default::default() };
        assert_eq!(m.line_at(0), Some(10));
        assert_eq!(m.line_at(3), Some(10));
        assert_eq!(m.line_at(4), Some(11));
        assert_eq!(m.line_at(100), Some(13));
        let late = MethodExtras { line_numbers: vec![(2, 5)], ..Default::default() };
        assert_eq!(late.line_at(1), None);
        assert_eq!(MethodExtras::default().line_at(0), None);
    }

    #[test]
    fn anno_const_keeps_lone_surrogate_units() {
        // 常量池：#1 = Utf8 "\uD800"（Modified UTF-8 三字节），#2 = Utf8 "ab"
        let bytes = [0x00, 0x03, 0x01, 0x00, 0x03, 0xED, 0xA0, 0x80, 0x01, 0x00, 0x02, b'a', b'b'];
        let pool = ConstantPool::parse(&mut Reader::new(&bytes)).unwrap();
        assert_eq!(anno_const(&pool, 1), Some(AnnoConst::Utf16(vec![0xD800])));
        assert_eq!(anno_const(&pool, 2), Some(AnnoConst::Utf8("ab".into())));
    }
}
