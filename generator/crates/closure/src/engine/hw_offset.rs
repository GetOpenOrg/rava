//! 引擎：按偏移读写的手写调用点（Unsafe 引用读写）的符号偏移收窄。
//!
//! 偏移实参是符号偏移（`V::Offset`：按名取得的实例字段偏移，经 static final 字段传递）时，调用点只触及
//! 所指字段：写入值只进该字段、读取只取该字段，不再撒到目标对象的全部引用字段。调用方方法重分析后
//! 偏移实参不再是同一符号偏移时，站点放宽为全部引用字段并按当前实参集重放（只放宽不收窄，单调）。

use super::*;

impl<'a> Engine<'a> {
    /// 调用点 (m, off) 的偏移实参（序号含接收者）所指字段节点：调用方当前分析给出符号偏移时
    fn site_offset(&mut self, m: usize, off: u32, idx: Option<usize>) -> Option<usize> {
        let idx = idx?;
        let a = self.methods[m].applied.clone()?;
        let Some(Event::Invoke { args, .. }) = class_lookup::event_at(&a, off, class_lookup::is_invoke) else { return None };
        let V::Offset(f) = args.get(idx)? else { return None };
        Some(self.field_node((**f).clone()))
    }

    /// 登记 / 复核站点 s 的触及字段；偏移实参变化即放宽并按当前实参集重放写入与读取
    pub(super) fn site_restrict(&mut self, s: u32, m: usize, off: u32, idx: Option<usize>) {
        let r = self.site_offset(m, off, idx);
        match self.hw_offsets.get(&s) {
            None => {
                self.hw_offsets.insert(s, r);
            }
            Some(&cur) if cur.is_none() || cur == r => {}
            Some(_) => {
                self.hw_offsets.insert(s, None);
                let (_, _, t) = self.hw_sites[s as usize];
                let ws = self.hw_writes(t);
                for (j, w) in ws.iter().enumerate() {
                    if w.as_ref().is_some_and(|w| w.fields) {
                        let cur = self.set_of(Node::A(s, j as u16));
                        self.hw_site_fields(s, j as u16, &cur);
                    }
                }
                if let Some(&(i, _, _)) = self.hw_reads.get(&s) {
                    let cur = self.set_of(Node::A(s, i));
                    self.memory_read(s, &cur);
                }
            }
        }
    }

    /// 站点 s 收窄到的字段：(字段节点, 声明类, 字段类型)；未收窄 / 基本类型字段为 None
    pub(super) fn site_field(&mut self, s: u32) -> Option<(usize, u32, u32)> {
        let fi = self.hw_offsets.get(&s).copied().flatten()?;
        let key = self.fields.get_index(fi)?.0.clone();
        let tid = parse_field(&key.desc).and_then(|t| self.ptype(&t))?;
        Some((fi, self.id(&key.owner), tid))
    }

    /// 收窄站点的对象分量：值集各元素若是声明类的子类型，给出其上该字段的节点（抽象对象取对象分量，
    /// 其余取 `plain`：写入用未知接收者写入节点、读取用字段并集视图）
    pub(super) fn site_field_nodes(&mut self, (fi, decl, tid): (usize, u32, u32), delta: &TypeSet, plain: Node) -> Vec<Node> {
        let class = self.id(CLASS);
        let mut out = Vec::new();
        let xs: Vec<u32> = delta.classes.iter().filter(|x| !self.arrays.contains_key(x)).collect();
        for x in xs {
            let (cls, is_obj) = match (self.objs.get(&x), self.mirrors.contains_key(&x)) {
                (Some(&c), _) => (c, true),
                (None, true) => (class, false),
                (None, false) => (x, false),
            };
            if self.sub(cls, decl) {
                out.push(if is_obj { self.obj_field(x, fi, tid) } else { plain });
            }
        }
        let os: Vec<u32> = delta.open.iter().collect();
        if os.into_iter().any(|o| self.sub(o, decl) || self.sub(decl, o)) {
            out.push(plain);
        }
        out.dedup();
        out
    }

    /// 方法是声明了内存效果（数组 / 字段写入或内存读取）的手写方法
    pub(super) fn declares_memory(&self, m: usize) -> bool {
        if !matches!(self.methods[m].kind, Kind::Handwritten(_)) {
            return false;
        }
        let k = self.methods[m].key.to_string();
        self.man.array_writes(&k).is_some() || self.man.memory_read(&k).is_some()
    }
}
