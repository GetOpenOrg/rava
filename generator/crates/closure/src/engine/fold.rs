//! 折叠点（folds v1）：不可达区间、死处理器与常量折叠的导出。

use super::*;

// ── 折叠点（folds v1）────────────────────────────────────────────────────────

/// 一个方法的折叠点（计划 §7.3「折叠点导出」）
pub struct Fold {
    pub method: String,
    /// 不可达指令的半开区间 [start, end)，两端落在指令起点（或代码末尾），有序、不重叠、相邻合并
    pub dead_pcs: Vec<(u32, u32)>,
    /// 不进入的异常处理器（起点 pc）：try 区间全部不可达，或 catch 类型从不被实例化
    pub dead_handlers: Vec<u32>,
    /// (pc, 指令, 常量值, 类型描述符)
    pub consts: Vec<(u32, u8, V, String)>,
    /// 接收者恒为 null 的活虚调用点（invokevirtual / invokeinterface）：全部克隆里接收者值集都没有对象，
    /// 分析器不给目标入链；执行即 NullPointerException，发射层不得按调用翻译（否则撞到闭包外的存根）
    pub null_recv: Vec<u32>,
    /// 违反「活的非跳转指令落到死区」约定的 pc（应恒为空）
    pub violations: Vec<u32>,
}

pub(super) fn fold_of(method: String, code: &classfile::Code, all: &[Rc<Analysis>]) -> Fold {
    use classfile::insn::Operand;
    use classfile::op;
    let insns = &code.insns;
    let reachable: Vec<bool> = (0..insns.len()).map(|i| all.iter().any(|a| a.reachable[i])).collect();
    let mut dead_pcs: Vec<(u32, u32)> = Vec::new();
    for (i, x) in insns.iter().enumerate() {
        if reachable[i] {
            continue;
        }
        let end = insns.get(i + 1).map_or(code.code_len, |n| n.offset);
        match dead_pcs.last_mut() {
            Some(last) if last.1 == x.offset => last.1 = end,
            _ => dead_pcs.push((x.offset, end)),
        }
    }
    let index: HashMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
    let live_at = |pc: u32| index.get(&pc).is_some_and(|&i| reachable[i]);
    let mut dead_handlers: Vec<u32> = code.exception_table.iter().map(|h| h.handler).filter(|&h| !live_at(h)).collect();
    dead_handlers.sort();
    dead_handlers.dedup();
    let mut violations = Vec::new();
    for (i, x) in insns.iter().enumerate() {
        // 跳转（含条件跳转 / ifnull / ifnonnull / goto_w / jsr）、switch、return、athrow 之后允许是死区
        let falls = !matches!(x.opcode, 0x99..=0xab | 0xc6..=0xc9 | op::ATHROW) && !(op::IRETURN..=op::RETURN).contains(&x.opcode);
        if reachable[i] && falls && insns.get(i + 1).is_some_and(|_| !reachable[i + 1]) {
            violations.push(x.offset);
        }
    }
    // 各克隆的常量点（同一 pc 取首个事件）
    let per: Vec<HashMap<u32, (u8, &V)>> = all
        .iter()
        .map(|a| {
            let mut m = HashMap::default();
            for (pc, e) in &a.events {
                if let Event::Const { opcode, value } = e {
                    m.entry(*pc).or_insert((*opcode, value));
                }
            }
            m
        })
        .collect();
    let mut consts: Vec<(u32, u8, V, String)> = Vec::new();
    for (pc, e) in all.iter().flat_map(|a| a.events.iter()) {
        let Event::Const { opcode, value } = e else { continue };
        let Some(&i) = index.get(pc) else { continue };
        let agree = all.iter().zip(&per).all(|(a, p)| !a.reachable[i] || p.get(pc) == Some(&(*opcode, value)));
        if !agree {
            continue;
        }
        let ins = &insns[i];
        let ty = match &ins.operand {
            Operand::Field(f) => f.desc.clone(),
            Operand::Method(m, _) => match m.desc.rfind(')') {
                Some(p) => m.desc[p + 1..].to_string(),
                None => continue,
            },
            _ => continue,
        };
        if consts.iter().any(|c| c.0 == *pc) {
            continue;
        }
        consts.push((*pc, *opcode, value.clone(), ty));
    }
    consts.sort_by_key(|c| c.0);
    Fold { method, dead_pcs, dead_handlers, consts, null_recv: Vec::new(), violations }
}

impl Engine<'_> {
    /// 字节码调用点 (方法, 偏移) 有接收者到达（逐个派发 / 非虚接边 / 枢纽展开过接收者）
    pub(super) fn site_has_recv(&self, m: usize, off: u32) -> bool {
        if self.recv_sites.contains(&(m, off)) || self.dispatch.get(&(m, off)).is_some_and(|t| !t.is_empty()) {
            return true;
        }
        self.hub_sites.get(&(m, off)).is_some_and(|hs| {
            hs.iter().any(|&h| {
                let mut cur = Some(h);
                while let Some(c) = cur {
                    if !self.hubs[c as usize].recvs.is_empty() {
                        return true;
                    }
                    cur = self.hubs[c as usize].parent;
                }
                false
            })
        })
    }

    /// 成员各克隆（方法节点序号）的接收者恒为 null 的活虚调用点：任一克隆有接收者即不算
    pub(super) fn null_recv(&self, clones: &[usize]) -> Vec<u32> {
        let mut hit: BTreeMap<u32, bool> = BTreeMap::new();
        for &i in clones {
            let Some(a) = &self.methods[i].analysis else { continue };
            for (pc, e) in &a.events {
                if let Event::Invoke { opcode: classfile::op::INVOKEVIRTUAL | classfile::op::INVOKEINTERFACE, .. } = e {
                    *hit.entry(*pc).or_default() |= self.site_has_recv(i, *pc);
                }
            }
        }
        hit.into_iter().filter(|(_, h)| !h).map(|(pc, _)| pc).collect()
    }
}
