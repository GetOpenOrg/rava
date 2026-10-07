//! 候选门发现与事实收集（引擎侧）：沿各类的首达溯源链（与 `--why` 同口径）上溯，统计每个消费型节点
//! （方法体、调用点）下游经过的类数；同时给任一候选收集分类事实。
//!
//! 消费型限定（`docs/plans/2026-10-01-c1d-closure-bloat.md` §7）：切构造器 / 类初始化 / 含字段写入的方法体、
//! 或切 new / 字段写入点，会让字段值集变空、按初值折叠，结果非单调——这些节点不作候选。
//! 调用点只取 invokevirtual / invokeinterface / invokestatic / 非构造的 invokespecial。

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use classfile::{op, Operand};

use super::super::{Engine, From, Kind, Via, HUB_MIN};
use super::classify::Facts;

/// 首达树上的候选门：(方法代表序号, 调用点偏移；None = 方法体)
pub(super) type TreeKey = (usize, Option<u32>);

#[derive(Debug, Clone, Default)]
pub(super) struct TreeGate {
    /// 首达链经过本门的类数
    pub classes: usize,
    /// 本门到根的最短步数
    pub depth: usize,
    /// 示例下游类（首个经过本门的类）
    pub example: String,
}

const MAX_STEPS: usize = 400;

impl Engine<'_> {
    /// 溯源一步：(方法代表序号与偏移 | None, 下一条溯源)
    fn via_step(&self, v: &Via) -> (Option<TreeKey>, Option<Via>) {
        match &v.from {
            From::Root(_) => (None, None),
            From::Method(i) => (Some((self.mbase[&self.methods[*i].key], v.off)), Some(self.methods[*i].via.clone())),
            From::Class(c) => {
                let next = match (v.kind, self.inited.get(c)) {
                    ("clinit" | "super-init" | "iface-init", Some(iv)) => Some(iv.clone()),
                    _ => self.classes.get(c).map(|n| n.via.clone()),
                };
                (None, next)
            }
        }
    }

    /// 首达树：各候选门经过的类数（只计合格的消费型节点）
    pub(super) fn gate_tree(&self) -> HashMap<TreeKey, TreeGate> {
        let mut out: HashMap<TreeKey, TreeGate> = HashMap::new();
        let mut elig: HashMap<TreeKey, bool> = HashMap::new();
        for (cls, node) in &self.classes {
            let mut path: Vec<TreeKey> = Vec::new();
            let mut seen: HashSet<TreeKey> = HashSet::new();
            let mut cur = Some(node.via.clone());
            let mut steps = 0;
            while let Some(v) = cur {
                let (k, next) = self.via_step(&v);
                if let Some((m, off)) = k {
                    for key in [Some((m, None)), off.map(|o| (m, Some(o)))].into_iter().flatten() {
                        if seen.insert(key) && *elig.entry(key).or_insert_with(|| self.gate_eligible(key)) {
                            path.push(key);
                        }
                    }
                }
                cur = next;
                steps += 1;
                if steps > MAX_STEPS {
                    break;
                }
            }
            let len = path.len();
            for (pos, key) in path.into_iter().enumerate() {
                let g = out.entry(key).or_insert_with(|| TreeGate { depth: usize::MAX, example: cls.clone(), ..Default::default() });
                g.classes += 1;
                g.depth = g.depth.min(len - pos);
            }
        }
        out
    }

    /// 方法代表序号的字节码（无则 None）
    fn gate_code(&self, m: usize) -> Option<std::sync::Arc<classfile::ClassFile>> {
        let key = &self.methods[m].key;
        self.cp.get(&key.owner).filter(|cf| cf.method(&key.name, &key.desc).is_some_and(|x| x.code.is_some()))
    }

    /// 候选是否消费型（可安全切除）
    pub(super) fn gate_eligible(&self, (m, off): TreeKey) -> bool {
        let n = &self.methods[m];
        if n.kind != Kind::Bytecode || self.is_pseudo_method(m) {
            return false;
        }
        let Some(cf) = self.gate_code(m) else { return false };
        let Some(code) = cf.method(&n.key.name, &n.key.desc).and_then(|x| x.code.as_ref()) else { return false };
        match off {
            None => {
                !matches!(n.key.name.as_str(), "<init>" | "<clinit>")
                    && !matches!(n.via.from, From::Root(_))
                    && !code.insns.iter().any(|x| matches!(x.opcode, op::PUTFIELD | op::PUTSTATIC))
            }
            Some(o) => code.insns.iter().find(|x| x.offset == o).is_some_and(|x| match &x.operand {
                Operand::Method(r, _) => match x.opcode {
                    op::INVOKEVIRTUAL | op::INVOKEINTERFACE | op::INVOKESTATIC => true,
                    op::INVOKESPECIAL => r.name != "<init>",
                    _ => false,
                },
                _ => false,
            }),
        }
    }

    /// 切除条目：`类.方法:描述符` 或 `类.方法:描述符@偏移`
    pub(super) fn gate_id(&self, (m, off): TreeKey) -> String {
        match off {
            None => self.methods[m].key.to_string(),
            Some(o) => format!("{}@{o}", self.methods[m].key),
        }
    }

    /// 自门向根的首达链（人读行，至多 `limit` 行）与溯源种类
    pub(super) fn gate_chain(&self, start: &Via, limit: usize) -> (Vec<String>, Vec<&'static str>) {
        let mut lines = Vec::new();
        let mut kinds = Vec::new();
        let mut cur = Some(start.clone());
        while let Some(v) = cur {
            kinds.push(v.kind);
            if lines.len() < limit {
                let off = v.off.map(|o| format!("@{o}")).unwrap_or_default();
                lines.push(match &v.from {
                    From::Root(s) => format!("[{}] 根 {s}", v.kind),
                    From::Method(i) => format!("[{}] {}{off}", v.kind, self.ctx_label(*i)),
                    From::Class(c) => format!("[{}] 类 {c}", v.kind),
                });
            }
            cur = self.via_step(&v).1;
            if kinds.len() > MAX_STEPS {
                lines.push("…（截断）".into());
                break;
            }
        }
        if kinds.len() > limit {
            lines.push(format!("…（共 {} 步）", kinds.len()));
        }
        (lines, kinds)
    }

    /// 下游首达节点按溯源种类计数：(方法代表序号, 偏移) → 种类 → 个数（偏移 None 的项是方法体整体）
    pub(super) fn gate_child_kinds(&self) -> HashMap<TreeKey, BTreeMap<&'static str, usize>> {
        let mut out: HashMap<TreeKey, BTreeMap<&'static str, usize>> = HashMap::new();
        let vias = self.classes.values().map(|c| &c.via).chain(self.methods.values().map(|m| &m.via));
        for v in vias {
            if let From::Method(i) = v.from {
                let m = self.mbase[&self.methods[i].key];
                *out.entry((m, None)).or_default().entry(v.kind).or_default() += 1;
                if let Some(o) = v.off {
                    *out.entry((m, Some(o))).or_default().entry(v.kind).or_default() += 1;
                }
            }
        }
        out
    }

    /// 分类事实
    pub(super) fn gate_facts(
        &self,
        (m, off): TreeKey,
        dispatch: &BTreeMap<(String, u32), BTreeSet<String>>,
        child_kinds: &HashMap<TreeKey, BTreeMap<&'static str, usize>>,
    ) -> Facts {
        let n = &self.methods[m];
        let cf = self.gate_code(m);
        let code = cf.as_ref().and_then(|cf| cf.method(&n.key.name, &n.key.desc)).and_then(|x| x.code.as_ref());
        let target = off.and_then(|o| code?.insns.iter().find(|x| x.offset == o)).and_then(|x| match &x.operand {
            Operand::Method(r, _) => Some(r.clone()),
            _ => None,
        });
        let reads_sysprop = code.is_some_and(|c| {
            c.insns.iter().any(|x| match &x.operand {
                Operand::Method(r, iface) => self.ctx.read_spec(None, x.opcode, r, *iface, None).is_some(),
                _ => false,
            })
        });
        let (fanout, open_hub) = match off {
            None => (0, false),
            Some(o) => {
                let fan = dispatch.get(&(n.key.to_string(), o)).map_or(0, |s| s.len());
                let open = self
                    .hub_sites
                    .iter()
                    .filter(|((i, so), _)| *so == o && self.mbase[&self.methods[*i].key] == m)
                    .flat_map(|(_, hs)| hs.iter())
                    .any(|&h| self.hubs[h as usize].open.is_some());
                (fan, open)
            }
        };
        let (_, chain_kinds) = self.gate_chain(&n.via, 0);
        let in_clinit = n.key.name == "<clinit>" || chain_kinds.iter().take(3).any(|k| matches!(*k, "clinit" | "super-init" | "iface-init"));
        let hints = &self.man.gate_hints;
        Facts {
            hint_owner: hints.category_of(&n.key.owner).map(|(c, e)| (c, e.to_string())),
            hint_target: target.as_ref().and_then(|t| hints.category_of(&t.owner)).map(|(c, e)| (c, e.to_string())),
            vm_boundary: self.man.is_vm_boundary(&n.key.owner),
            chain_kinds,
            child_kinds: child_kinds.get(&(m, off)).cloned().unwrap_or_default(),
            fanout,
            open_hub,
            reads_sysprop,
            in_clinit,
            hub_min: HUB_MIN,
        }
    }
}

