//! 引擎：手写体按名写入的接收者取自按名读时，按接收者值集解出被写字段。
//!
//! `let m = o.0.__unsafe_ref_get("g")?; m.0.__unsafe_ref_set("f", v)`：接收者 m 是 o 上字段 g 的内容。
//! g 的读取接进接收者汇集节点 [`Node::NR`]（o 是形参时按其值集逐个对象取 g，否则全部名为 g 的字段），汇集节点
//! 新增的每个类型 X 上解出字段 f（含超类）：只该字段登记手写写入、不折叠，写入值接进该字段——不再把全部名为 f 的
//! 字段按手写写入处理。X 上无此字段时手写体按名写失败即 stub panic，不贡献；X 为非 final 的 open 类型时子类型可能
//! 另声明同名字段，退回按名处理。

use super::*;

/// 一处接收者取自按名读的手写按名写入
#[derive(Clone, PartialEq, Eq)]
pub(super) struct NameWrite {
    m: usize,
    host: Rc<str>,
    fa: FieldAccess,
}

/// 接收者汇集节点表：(按名读的接收者节点（None = 未知）, 字段名 g) → 序号
#[derive(Default)]
pub(super) struct NameRecvs {
    at: HashMap<(Option<Node>, String), u32>,
    keys: Vec<(Option<Node>, String)>,
    writes: HashMap<u32, Vec<NameWrite>>,
}

impl<'a> Engine<'a> {
    /// 手写按名写入的接收者取自按名读：登记到接收者汇集节点并按其当前值集接入，返回 true；否则 false
    pub(super) fn hw_name_write(&mut self, m: usize, host: &str, fa: &FieldAccess) -> bool {
        let Some(src) = fa.recv_src.clone() else { return false };
        if !fa.write || fa.recv.is_some() {
            return false;
        }
        let from = fa.recv_src_param.filter(|&k| self.methods[m].ptypes.get(k as usize).is_some_and(|t| t.is_some())).map(|k| Node::P(m, k));
        let key = (from, src.clone());
        let (id, fresh) = match self.name_recvs.at.get(&key) {
            Some(&id) => (id, false),
            None => {
                let id = self.name_recvs.keys.len() as u32;
                self.name_recvs.at.insert(key.clone(), id);
                self.name_recvs.keys.push(key);
                (id, true)
            }
        };
        let w = NameWrite { m, host: Rc::from(host), fa: fa.clone() };
        let ws = self.name_recvs.writes.entry(id).or_default();
        if ws.contains(&w) {
            return true;
        }
        ws.push(w.clone());
        let hub = Node::NR(id);
        if fresh {
            let obj = self.id(OBJECT);
            match from {
                Some(p) => self.name_read(p, &src, hub, obj),
                None => self.hw_copy_by_name(&src, hub, obj),
            }
        }
        let cur = self.graph.get(&hub).cloned().unwrap_or_default();
        self.name_write_apply(&[w], &cur);
        true
    }

    /// 接收者汇集节点新增 delta：各登记写入在新增类型上解出字段并接入
    pub(super) fn name_write_objs(&mut self, id: u32, delta: &TypeSet) {
        let Some(ws) = self.name_recvs.writes.get(&id).cloned() else { return };
        self.name_write_apply(&ws, delta);
    }

    fn name_write_apply(&mut self, ws: &[NameWrite], delta: &TypeSet) {
        let class = self.id(CLASS);
        let mut xs: Vec<(u32, Option<u32>)> = Vec::new();
        let mut wide = false;
        for x in delta.classes.iter().filter(|x| !self.arrays.contains_key(x)) {
            xs.push(match (self.objs.get(&x), self.mirrors.contains_key(&x)) {
                (Some(&c), _) => (c, Some(x)),
                (None, true) => (class, None),
                (None, false) => (x, None),
            });
        }
        for o in delta.open.iter() {
            let fin = self.h.class(&self.names[o as usize].clone()).is_some_and(|cf| cf.access & acc::FINAL != 0 && !cf.is_interface());
            if fin {
                xs.push((o, None));
            } else {
                wide = true;
            }
        }
        for w in ws {
            for &(cls, obj) in &xs {
                let cname = self.names[cls as usize].clone();
                let Some((decl, desc)) = self.field_by_name(&cname, &w.fa.field) else { continue };
                let key = MemberRef { owner: decl, name: w.fa.field.clone(), desc };
                self.hw_written.insert(key.clone());
                self.hw_open_field(&key);
                let Some(tid) = parse_field(&key.desc).and_then(|t| self.ptype(&t)) else { continue };
                let fi = self.field_node(key);
                let to = match obj {
                    Some(o) => self.obj_field(o, fi, tid),
                    None => Node::U(fi),
                };
                self.hw_write_value(w.m, &w.host, &w.fa, to, tid);
            }
            if wide {
                self.hw_field_by_name(w.m, &w.fa);
            }
        }
    }

    /// 接收者汇集节点的显示名（诊断）
    pub(super) fn name_recv_label(&self, id: u32) -> String {
        let (from, g) = &self.name_recvs.keys[id as usize];
        match from {
            Some(n) => format!("{} 上按名读的 {g}", self.node_str(*n)),
            None => format!("按名读的 {g}（接收者未知）"),
        }
    }
}
