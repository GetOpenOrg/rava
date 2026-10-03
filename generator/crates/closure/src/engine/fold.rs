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
    /// 定论不返回的活调用点：唯一目标为字节码方法，全部节点已分析且没有任何返回路径（见 `noreturn.rs`）。
    /// 分析在定论阶段按值未知继续分析其后的代码（folds 规则 7），这里单独给出；发射层可在调用后终止控制流
    pub noreturn_calls: Vec<u32>,
    /// 把 noreturn_calls 与 null_recv 当作控制流终点时另外不可达的区间（与 dead_pcs 不相交，格式同 dead_pcs）
    pub noreturn_dead_pcs: Vec<(u32, u32)>,
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
    Fold {
        method,
        dead_pcs,
        dead_handlers,
        dead_catches: Vec::new(),
        consts,
        null_recv: Vec::new(),
        noreturn_calls: Vec::new(),
        noreturn_dead_pcs: Vec::new(),
        props: Vec::new(),
        violations,
    }
}

/// 把 `stops` 中的调用点当作控制流终点时，原本可达（`reachable`）的指令里不再可达的区间。
/// 只沿原本可达的指令走（被常量剪掉的边不会复活）：正常后继（跳转 / switch 目标、非终结指令的顺序后继）
/// 与覆盖该指令的异常处理器（处理器原本可达即计入，保守）
pub(super) fn cut_after(code: &classfile::Code, reachable: &[bool], stops: &[u32]) -> Vec<(u32, u32)> {
    use classfile::insn::{is_terminal, Operand};
    let insns = &code.insns;
    let index: HashMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
    let mut seen = vec![false; insns.len()];
    let mut work: Vec<usize> = if reachable.first() == Some(&true) { vec![0] } else { vec![] };
    let push = |pc: u32, work: &mut Vec<usize>| {
        if let Some(&j) = index.get(&pc) {
            if reachable[j] {
                work.push(j);
            }
        }
    };
    while let Some(i) = work.pop() {
        if std::mem::replace(&mut seen[i], true) {
            continue;
        }
        let x = &insns[i];
        match &x.operand {
            Operand::Branch(t) => push(*t, &mut work),
            Operand::TableSwitch { default, targets, .. } => targets.iter().chain([default]).for_each(|t| push(*t, &mut work)),
            Operand::LookupSwitch { default, pairs } => pairs.iter().map(|p| &p.1).chain([default]).for_each(|t| push(*t, &mut work)),
            _ => {}
        }
        for h in code.exception_table.iter().filter(|h| h.start <= x.offset && x.offset < h.end) {
            push(h.handler, &mut work);
        }
        if !is_terminal(x.opcode) && !stops.contains(&x.offset) {
            if let Some(n) = insns.get(i + 1) {
                push(n.offset, &mut work);
            }
        }
    }
    let mut out: Vec<(u32, u32)> = Vec::new();
    for (i, x) in insns.iter().enumerate() {
        if !reachable[i] || seen[i] {
            continue;
        }
        let end = insns.get(i + 1).map_or(code.code_len, |n| n.offset);
        match out.last_mut() {
            Some(last) if last.1 == x.offset => last.1 = end,
            _ => out.push((x.offset, end)),
        }
    }
    out
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

    /// 可能触发类初始化的 getstatic 不导出为常量（JVMS §5.5）：字段声明类（类则连同父类链）中有不在当前类
    /// 父类链上（当前类初始化时父类链已初始化）且带 `<clinit>` 的——常量替换会丢掉这次初始化的副作用。
    /// 不可达区间照常导出，发射层按真实读取翻译（访问器触发初始化）
    pub(super) fn init_reads(&self, owner: &str, code: &classfile::Code, f: &mut Fold) {
        let mine: Vec<String> = self.h.superclasses(owner).iter().map(|c| c.name.clone()).collect();
        f.consts.retain(|c| {
            if c.1 != classfile::op::GETSTATIC {
                return true;
            }
            let Some(x) = code.insns.iter().find(|x| x.offset == c.0) else { return true };
            let classfile::Operand::Field(fr) = &x.operand else { return true };
            let Some(site) = self.h.resolve_field(&fr.owner, &fr.name, &fr.desc) else { return true };
            // 接口初始化不连带父接口；类初始化连带父类链，已在当前类父类链上的不再触发
            let chain = if site.class.is_interface() { vec![site.class.clone()] } else { self.h.superclasses(&site.class.name) };
            !chain.iter().any(|c| !mine.contains(&c.name) && c.methods.iter().any(|m| m.is_clinit()))
        });
    }

    /// 定论不返回的活调用点与因此另外不可达的区间：唯一目标为字节码方法（无清单返回事实 / 派生结果），
    /// 目标有节点、全部节点已分析，且返回常量格缺席（没有任何克隆的分析含返回点）。
    /// 截断区间把 `f.null_recv` 也当作终点（接收者恒 null 的调用只会抛 NPE）；
    /// 须在 consts / null_recv 算出之后、prop_folds 之前调用
    pub(super) fn noreturn_calls(&self, code: &classfile::Code, all: &[Rc<Analysis>], thrown: &[u32], f: &mut Fold) {
        let reachable: Vec<bool> = (0..code.insns.len()).map(|i| all.iter().any(|a| a.reachable[i])).collect();
        let nr = self.ctx.noreturn.borrow();
        let rvals = self.ctx.rvals.borrow();
        let mut stops = Vec::new();
        for (i, x) in code.insns.iter().enumerate() {
            let classfile::Operand::Method(mref, iface) = &x.operand else { continue };
            if !reachable[i] || x.opcode == classfile::op::INVOKEDYNAMIC {
                continue;
            }
            let c = self.ctx.call_info(x.opcode, mref, *iface);
            if c.fact.is_some() || c.null_to_false || self.derived_call(x.opcode, mref, *iface, &c) {
                continue;
            }
            let Some(t) = &c.target else { continue };
            if nr.settled_never(t) && !rvals.contains_key(t) {
                stops.push(x.offset);
            }
        }
        stops.extend_from_slice(thrown);
        stops.sort_unstable();
        stops.dedup();
        // null_recv 调用点的目标集为空，同样不会正常返回：一并作为截断终点（须先算出 f.null_recv）
        let mut ends: Vec<u32> = stops.iter().chain(&f.null_recv).copied().collect();
        ends.sort_unstable();
        ends.dedup();
        if !ends.is_empty() {
            f.noreturn_dead_pcs = cut_after(code, &reachable, &ends);
        }
        // 落入截断区间的终点与常量折叠点本身已不可达（被前一终点截断）：不再列出
        let cut = &f.noreturn_dead_pcs;
        let live = |pc: &u32| !cut.iter().any(|&(s, e)| s <= *pc && *pc < e);
        f.null_recv.retain(live);
        // null_recv 调用点只会抛 NPE：目标集为空时返回值格缺席而被当作常量的，不再列为常量
        let nulls = &f.null_recv;
        f.consts.retain(|c| live(&c.0) && nulls.binary_search(&c.0).is_err());
        f.noreturn_calls = stops.into_iter().filter(live).collect();
    }

    /// 调用结果由清单派生规则给出（值相等 / 字符串运算 / 系统属性读取）：不按被调字节码判定
    fn derived_call(&self, opcode: u8, m: &MemberRef, iface: bool, c: &CallInfo) -> bool {
        let named = |k: &str| self.man.is_value_equals(k) || self.man.string_op(k).is_some() || self.man.sysprops.is_holder(k);
        named(&m.to_string()) || c.target.as_ref().is_some_and(|t| named(&t.to_string())) || self.ctx.read_spec(None, opcode, m, iface, Some(c)).is_some()
    }

    /// 成员各克隆（方法节点序号）的接收者恒为 null 的活虚调用点：任一克隆有接收者、接收者恒非 null
    /// （实例方法的 this 按 JVMS 恒非 null，`new` 结果、catch 值同理——类型集为空只说明值来自未建模来源
    /// 或所在路径不执行，不说明是 null）、或接收者的值流可能缺失（来源可经流边从未建模来源到达，见
    /// `unmodeled.rs`）即不算
    pub(super) fn null_recv(&self, clones: &[usize], um: &super::unmodeled::Unmodeled) -> Vec<u32> {
        let mut hit: BTreeMap<u32, bool> = BTreeMap::new();
        for &i in clones {
            let Some(a) = &self.methods[i].analysis else { continue };
            for (pc, e) in &a.events {
                if let Event::Invoke { opcode: classfile::op::INVOKEVIRTUAL | classfile::op::INVOKEINTERFACE, mref, args, .. } = e {
                    // 属主停在 L1：非 null 值的运行时类及其全部超类型至少 L2，接收者可能来自未建模来源（含运行期
                    // 定义类的对象）的调用点属主也已升 L2（`levels.rs`）——接收者只可能是 null
                    let opaque = self.classes.get(mref.owner.as_str()).is_some_and(|c| c.level == Level::Type);
                    let h = hit.entry(*pc).or_default();
                    // 开放世界（`open_world.rs`）：属主可被用户扩展的非用户调用点不依赖本次的实例化集合
                    if !*h && self.open_site(i, &mref.owner) {
                        *h = true;
                    }
                    if !*h && !opaque {
                        *h = self.site_has_recv(i, *pc) || args.first().is_none_or(|r| r.nonnull() == Some(true) || self.recv_unmodeled(i, r, um));
                    }
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

    /// 0 nop；1 invokestatic（不返回）；4 nop；5 return。handler 5 覆盖 [0, 4)
    #[test]
    fn cut_after_stops_fallthrough_keeps_handlers() {
        use classfile::{op, Code, ExceptionEntry, Insn, Operand};
        let m = MemberRef { owner: "p/X".into(), name: "f".into(), desc: "()V".into() };
        let insns = vec![
            Insn { offset: 0, opcode: 0x00, operand: Operand::None },
            Insn { offset: 1, opcode: op::INVOKESTATIC, operand: Operand::Method(m, false) },
            Insn { offset: 4, opcode: 0x00, operand: Operand::None },
            Insn { offset: 5, opcode: op::RETURN, operand: Operand::None },
        ];
        let mut code = Code { max_stack: 1, max_locals: 0, code_len: 6, insns, exception_table: vec![] };
        let live = [true; 4];
        assert_eq!(cut_after(&code, &live, &[1]), vec![(4, 6)]);
        assert!(cut_after(&code, &live, &[]).is_empty());
        code.exception_table.push(ExceptionEntry { start: 0, end: 4, handler: 5, catch_type: None });
        assert_eq!(cut_after(&code, &live, &[1]), vec![(4, 5)]);
        // 原本不可达的指令不重复列出
        assert_eq!(cut_after(&code, &[true, true, false, true], &[1]), vec![]);
    }
}
