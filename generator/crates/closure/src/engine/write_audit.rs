//! 引擎：字段折叠审计（`--flows @foldfields`）。
//!
//! 字段读折叠的前提是「全部写入来源都已建模」。本诊断列出活方法读过、且当前按常量折叠的每个字段，
//! 按值的来历分三类，供逐条核对写入来源是否完整：
//! - `clinit`：static final，值取自 `<clinit>` 唯一一次赋值（VM 在 `<clinit>` 之后改写的常量在此暴露）
//! - `writes`：值集由可达字节码写入（及初值）合成
//! - `initial`：没有任何已建模的写入，按初值（0 / null）折叠——未建模写入（VM 注入 / 手写 / 反射 /
//!   Unsafe）只会落在这一类或 `writes` 里，是审计重点

use super::*;

impl Engine<'_> {
    pub(super) fn fold_fields(&self) -> Vec<String> {
        let keys: BTreeSet<MemberRef> = self.ctx.fdeps.borrow().keys().cloned().collect();
        let mut out = Vec::new();
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for k in keys {
            let Some(fi) = self.ctx.field_info(&k) else { continue };
            if self.ctx.field_open(&fi) || fi.constant.is_some() {
                continue;
            }
            let (class, v) = if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
                ("clinit", self.ctx.static_const(None, &fi.key, None))
            } else {
                match self.ctx.fvals.borrow().get(&fi.key) {
                    Some(pv) => ("writes", pv.value()),
                    None => ("initial", facts::default_pv(&fi.key.desc).value()),
                }
            };
            let Some(v) = v else { continue };
            *counts.entry(class).or_default() += 1;
            out.push(format!("  {class:8} {}.{}:{} = {v:?}", fi.key.owner, fi.key.name, fi.key.desc));
        }
        out.sort();
        out.insert(0, format!("  折叠字段 {}（{counts:?}）", out.len()));
        out
    }
}

impl Engine<'_> {
    /// 按名反射写入形状（形参含 Class 或接收者是 Class、且有 String 形参）里名字不是常量的活调用点
    /// （`--flows @bynamesites`）。名字来自本方法形参的（`param`）由调用方的常量补齐；其余（`unknown`）
    /// 若最终落到「名字 → 字段身份」的原语，该字段的写入不会被放开——逐条核对
    pub(super) fn byname_sites(&self) -> Vec<String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        for mn in self.methods.values() {
            let Some(a) = mn.analysis.as_ref() else { continue };
            for (off, e) in &a.events {
                let Event::Invoke { opcode, mref, args, .. } = e else { continue };
                let Some(md) = parse_method(&mref.desc) else { continue };
                let is_class = |p: &FieldType| matches!(p, FieldType::Object(c) if c == CLASS);
                let class_recv = *opcode != classfile::op::INVOKESTATIC && mref.owner == CLASS;
                if !class_recv && !md.params.iter().any(is_class) {
                    continue;
                }
                let base = usize::from(*opcode != classfile::op::INVOKESTATIC);
                for (i, p) in md.params.iter().enumerate() {
                    if !matches!(p, FieldType::Object(c) if c == STRING) {
                        continue;
                    }
                    let Some(v) = args.get(base + i) else { continue };
                    if matches!(v, V::Str(_) | V::Null) {
                        continue;
                    }
                    let from_param = v.srcs().iter().any(|s| matches!(s, Src::Param(_)));
                    let tag = if from_param { "param" } else { "unknown" };
                    out.insert(format!("  {tag:7} {}@{off} → {mref}", mn.key));
                }
            }
        }
        let mut v: Vec<String> = out.into_iter().collect();
        v.insert(0, format!("  {} 个名字非常量的按名调用点", v.len()));
        v
    }
}
