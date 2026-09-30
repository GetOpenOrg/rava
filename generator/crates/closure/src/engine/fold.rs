//! 折叠点（folds v2）：不可达区间、死处理器、死 catch 表项与常量折叠的导出。

use super::*;

// ── 折叠点（folds v2）────────────────────────────────────────────────────────

/// 一个方法的折叠点（计划 §7.3「折叠点导出」）
pub struct Fold {
    pub method: String,
    /// 不可达指令的半开区间 [start, end)，两端落在指令起点（或代码末尾），有序、不重叠、相邻合并
    pub dead_pcs: Vec<(u32, u32)>,
    /// 不进入的异常处理器（起点 pc）：try 区间全部不可达，或 catch 类型从不被实例化
    pub dead_handlers: Vec<u32>,
    /// 删除的异常表项：catch 类型不在闭包类集合内（从不被加载，处理器不经它进入）；
    /// 处理器本身仍活（另有活表项），全部表项都死的处理器并入 dead_handlers、不在此列
    pub dead_catches: Vec<DeadCatch>,
    /// (pc, 指令, 常量值, 类型描述符)
    pub consts: Vec<(u32, u8, V, String)>,
    /// 接收者恒为 null 的活虚调用点（invokevirtual / invokeinterface）：全部克隆里接收者值集都没有对象，
    /// 分析器不给目标入链；执行即 NullPointerException，发射层不得按调用翻译（否则撞到闭包外的存根）
    pub null_recv: Vec<u32>,
    /// 常量来自系统属性读取折叠的调用点（consts 的子集；统计用，不导出）
    pub props: Vec<u32>,
    /// 违反「活的非跳转指令落到死区」约定的 pc（应恒为空）
    pub violations: Vec<u32>,
}

/// 死 catch 表项（按异常表原值，消费方逐字段匹配删除）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeadCatch {
    pub start: u32,
    pub end: u32,
    pub handler: u32,
    pub catch_type: String,
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
        if !exportable(value, &ty) || consts.iter().any(|c| c.0 == *pc) {
            continue;
        }
        consts.push((*pc, *opcode, value.clone(), ty));
    }
    consts.sort_by_key(|c| c.0);
    Fold { method, dead_pcs, dead_handlers, dead_catches: Vec::new(), consts, null_recv: Vec::new(), props: Vec::new(), violations }
}

/// 常量值能否按 folds 约定导出到该读取点：整型族 ↔ 整数、J ↔ long、String 槽 ↔ 字符串、引用槽 ↔ null。
/// 其余组合不导出（消费方按读取点类型换成装载指令）：
/// - 字符串值经声明为非 String 的返回 / 字段（`requireNonNull(s, …)` 返回 Object）到达——换成 `ldc`
///   会改变该点的静态类型；
/// - 类字面量值（V::Class）、奇偶值（V::Par）等没有装载指令等价物的抽象值。
fn exportable(v: &V, ty: &str) -> bool {
    match v {
        V::Int(_) => matches!(ty, "Z" | "B" | "C" | "S" | "I"),
        V::Long(_) => ty == "J",
        V::Str(_) => ty.strip_prefix('L').and_then(|t| t.strip_suffix(';')) == Some(crate::absint::STRING),
        V::Null => ty.starts_with('L') || ty.starts_with('['),
        _ => false,
    }
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

    /// 死 catch 表项：catch 类型不在闭包类集合内。catch-any 从不算死；
    /// 处理器的全部表项都死 → 处理器并入 dead_handlers（此时不再逐项列出）
    pub(super) fn dead_catches(&self, code: &classfile::Code, f: &mut Fold) {
        let dead = |t: &Option<String>| t.as_ref().is_some_and(|c| !self.classes.contains_key(c.as_str()));
        let mut by_handler: BTreeMap<u32, bool> = BTreeMap::new();
        for h in &code.exception_table {
            *by_handler.entry(h.handler).or_insert(true) &= dead(&h.catch_type);
        }
        for (h, all_dead) in by_handler {
            if all_dead && !f.dead_handlers.contains(&h) {
                f.dead_handlers.push(h);
            }
        }
        f.dead_handlers.sort();
        for h in &code.exception_table {
            if dead(&h.catch_type) && f.dead_handlers.binary_search(&h.handler).is_err() {
                let catch_type = h.catch_type.clone().unwrap_or_default();
                f.dead_catches.push(DeadCatch { start: h.start, end: h.end, handler: h.handler, catch_type });
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exportable_matches_slot_type() {
        let s = V::Str(Rc::from("UTF-16BE"));
        let string_slot = format!("L{};", crate::absint::STRING);
        assert!(exportable(&s, &string_slot));
        // 经 Object 返回的字符串（声明为基类型的恒等返回）不导出
        assert!(!exportable(&s, "Lx/Base;"));
        assert!(exportable(&V::Int(1), "Z"));
        assert!(!exportable(&V::Int(1), "J"));
        assert!(exportable(&V::Long(1), "J"));
        assert!(exportable(&V::Null, "[I"));
        assert!(!exportable(&V::Null, "I"));
        assert!(!exportable(&V::Par(true), "I"));
        assert!(!exportable(&V::Class(Rc::from("x/Y"), 0), "Lx/Mirror;"));
    }
}
