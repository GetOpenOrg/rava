//! 引擎：值映射槽——封存值映射字段（`sealed.rs::map_seal`）读取结果的名字候选。
//!
//! 封存判定只看字节码（嵌套内全部访问，不读事实）：映射对象不逃逸、写字段只有新建点写入或 null、读字段的值只作
//! readers / writers / keeps 的接收者（或传给只读的辅助方法）。读取结果的候选因此是两部分写入值的并：
//! - 新建点写入（「新建 → 构造 → 写入 → put 字段」）：在访问方法的独立分析里按拆段规则求值（与旧口径相同）；
//! - 经读字段写入（`this.m.put(k, v)` 一类）：写入值可来自写入方法的形参（如 `charset(名, 类名, 别名)` 以常量实参
//!   被各调用点写入），独立分析求不出。引擎处理每个写入调用点时把写入值登记到该字段的值映射槽（`PSlot::V`）：
//!   字面量直接并入、写入方形参沿子集边取形参常量集、其余值登记为槽的输入（读者在写入方帧里按拼接段求值）。
//!   只有引擎实际处理到的写入点才登记——不可达方法里的写入在运行期不执行；写入方重分析、形参常量集增长时读者重跑
//!   （与 String 字段槽同一套机制，见 `pstrs.rs`），结果只并不减，与处理先后无关。
//!
//! 钩子对任一 private 字段的读取结果上的写入都登记（不先做封存判定，登记本身无副作用）；读者只在字段封存时取槽。
//! 字段可经字节码外途径写入（`field_open`）时不给候选。

use super::class_lookup::{event_at, is_invoke, site_of, Gap};
use super::name_eval::Frame;
use super::sealed::{flatten, is_field};
use super::*;
use classfile::op::{GETFIELD, GETSTATIC, INVOKESTATIC};

impl<'a> Engine<'a> {
    /// 写入点钩子：方法 m 偏移 off 的调用是值映射写入入口、接收者恰为 private 字段的读取结果时，写入值并入该字段的值映射槽
    pub(super) fn map_slot_write(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) {
        if opcode == INVOKESTATIC || args.len() != 3 {
            return;
        }
        if !self.man.names.value_maps().writers.contains(&format!("{}:{}", mref.name, mref.desc)) {
            return;
        }
        let Some(o) = site_of(&args[0]) else { return };
        let Some(a) = self.methods[m].analysis.clone() else { return };
        let Some(Event::Field { opcode: GETFIELD | GETSTATIC, mref: f, .. }) = event_at(&a, o, is_field) else { return };
        let Some(fi) = self.ctx.field_info(f) else { return };
        if fi.access & acc::PRIVATE == 0 {
            return;
        }
        let n = self.field_node(fi.key.clone());
        self.pstr_map_put(m, off, n, &args[2]);
    }

    /// 值（可经一次 checkcast）是封存值映射字段经 readers 的读取结果时，其名字候选与是否推得出；否则 None。
    /// 有经读字段的写入时需在站点内求值（读者登记），不在站点内为 None
    pub(super) fn map_field_names(&mut self, a: &Analysis, v: &V, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let mut o = site_of(v)?;
        if let Some(Event::CheckCast(_, Some(inner))) = event_at(a, o, |e| matches!(e, Event::CheckCast(..))) {
            o = site_of(inner)?;
        }
        let Event::Invoke { opcode, mref, args, .. } = event_at(a, o, is_invoke)? else { return None };
        if *opcode == INVOKESTATIC || !self.man.names.value_maps().readers.contains(&format!("{}:{}", mref.name, mref.desc)) {
            return None;
        }
        let Event::Field { opcode: GETFIELD | GETSTATIC, mref: f, .. } = event_at(a, site_of(args.first()?)?, is_field)? else {
            return None;
        };
        let f = f.clone();
        let (accs, vals, read_writes) = self.map_seal(&f)?;
        let mut out = BTreeSet::new();
        for (i, v) in vals {
            if v == V::Null {
                continue;
            }
            let fr = Frame { m: None, a: &accs[i].a, owner: &accs[i].owner, up: None };
            let parts = self.name_parts(&fr, &v, Gap::Fail, 0)?;
            out.extend(flatten(&parts)?);
        }
        if !read_writes {
            return Some((out, true));
        }
        let fi = self.ctx.field_info(&f)?;
        if self.ctx.field_open(&fi) {
            return None;
        }
        let n = self.field_node(fi.key.clone());
        let (names, complete) = self.map_slot_names(n, depth)?;
        out.extend(names);
        Some((out, complete))
    }
}
