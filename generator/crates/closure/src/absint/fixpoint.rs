//! 方法级驱动：入口状态、数据流不动点与事件产出两阶段（见模块文档）、保守模式。

use super::*;

/// 方法入口状态：this + 形参（按描述符类型，来源 = 形参序号）
fn entry_state(owner: &str, desc: &str, is_static: bool, max_locals: u16, param: impl Fn(u16) -> Option<V>) -> Option<State> {
    let md = parse_method(desc)?;
    let mut locals = Vec::with_capacity(max_locals as usize);
    if !is_static {
        // 接收者：求值器给出的非空常量（字符串 / 类字面量）按其值；引导映像对象（身份确定、final 字段已知）
        // 按该对象（进入方法体即非空）；否则为属主类型的非空引用
        let this = param(0).and_then(|v| match v {
            V::Ref { obj: Some(o), .. } if matches!(*o, Obj::Image(..)) => {
                Some(V::Ref { ty: Some(Rc::from(owner)), nonnull: true, src: src1(Src::Param(0)), obj: Some(o) })
            }
            V::Ref { .. } => None,
            v => (v.nonnull() == Some(true)).then_some(v),
        });
        locals.push(this.map_or_else(|| V::Ref { ty: Some(Rc::from(owner)), nonnull: true, src: src1(Src::Param(0)), obj: None }, |v| v.rebased(Src::Param(0))));
    }
    let base = u16::from(!is_static);
    for (i, p) in md.params.iter().enumerate() {
        let k = base + i as u16;
        // 形参常量格的「非空引用」不带类型：按描述符补上
        let v = match param(k).map(|v| v.rebased(Src::Param(k))) {
            Some(V::Ref { ty: None, nonnull, src, obj }) => match value_of(p, Src::Param(k)) {
                V::Ref { ty, .. } => V::Ref { ty, nonnull, src, obj },
                _ => V::Ref { ty: None, nonnull, src, obj },
            },
            Some(v) => v,
            None => value_of(p, Src::Param(k)),
        };
        locals.push(v);
        if p.slots() == 2 {
            locals.push(V::Hi);
        }
    }
    if locals.len() > max_locals as usize {
        return None;
    }
    locals.resize(max_locals as usize, V::Top);
    Some(State { locals, stack: Vec::new(), finals: Vec::new(), nnf: Vec::new(), inits: Vec::new() })
}

fn conservative(code: &Code) -> Analysis {
    let mut events = Vec::new();
    for ins in &code.insns {
        // 保守模式：值全部未知，事件照常产出（实参按 Top）
        let e = match (&ins.operand, ins.opcode) {
            (Operand::Method(m, iface), o) => {
                let n = parse_method(&m.desc).map_or(0, |d| d.params.len()) + usize::from(o != op::INVOKESTATIC);
                Some(Event::Invoke { opcode: o, mref: m.clone(), iface: *iface, args: vec![V::Top; n] })
            }
            (Operand::InvokeDynamic { bsm, name, desc, .. }, _) => {
                let n = parse_method(desc).map_or(0, |d| d.params.len());
                Some(Event::Indy { bsm: *bsm, name: name.clone(), desc: desc.clone(), args: vec![V::Top; n] })
            }
            (Operand::Field(f), o) => {
                let recv = matches!(o, op::GETFIELD | op::PUTFIELD).then_some(V::Top);
                Some(Event::Field { opcode: o, mref: f.clone(), recv, value: None })
            }
            (Operand::Class(c), op::NEW) => Some(Event::New(c.clone())),
            (Operand::Class(c), op::ANEWARRAY) => Some(Event::NewArray(format!("[L{c};"), false)),
            (Operand::Class(c), op::CHECKCAST) => Some(Event::CheckCast(c.clone(), None)),
            (Operand::Class(c), op::INSTANCEOF) => Some(Event::InstanceOf(c.clone(), None)),
            (Operand::MultiANewArray(c, _), _) => Some(Event::NewArray(c.clone(), false)),
            (Operand::Ldc(c), _) => Some(Event::Ldc(c.clone())),
            (_, 0x32) => Some(Event::ArrayLoad { array: V::Top, index: V::Top }),
            (_, 0x53) => Some(Event::ArrayStore { array: V::Top, index: V::Top, value: V::Top }),
            (_, op::ATHROW) => Some(Event::Throw(V::Top)),
            (_, 0xac..=0xb1) => Some(Event::Return(V::Top)),
            _ => None,
        };
        if let Some(e) = e {
            events.push((ins.offset, e));
        }
    }
    for h in &code.exception_table {
        events.push((h.handler, Event::Catch(h.catch_type.clone())));
    }
    events.sort_by_key(|e| e.0);
    Analysis { reachable: vec![true; code.insns.len()], events, pending_types: vec![], mirror_assumed: vec![], mirror_field_assumed: vec![], site_mirror_assumed: false, conservative: true, cfg: Rc::new(cfg::Cfg::build(code)), selector_params: 0 }
}

/// 分析一个方法体
pub fn analyze<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O) -> Analysis {
    match run(owner, desc, is_static, code, oracle, false) {
        Some((a, _)) => a,
        None => conservative(code),
    }
}

/// 构造器的确定初始化摘要（见 `init.rs`）；无法建模（保守模式）→ None
pub fn analyze_init<O: Oracle>(owner: &str, desc: &str, code: &Code, oracle: &O) -> Option<InitSum> {
    run(owner, desc, false, code, oracle, true)?.1
}

pub(super) fn run<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O, track: bool) -> Option<(Analysis, Option<InitSum>)> {
    let insns = &code.insns;
    let n = insns.len();
    // 指令按偏移升序：偏移 → 下标用二分（免逐次分析建表）
    let at = |off: u32| insns.binary_search_by_key(&off, |x| x.offset).ok();

    // 基本块首指令
    let mut leader = vec![false; n];
    if n == 0 {
        return None;
    }
    leader[0] = true;
    for (i, ins) in insns.iter().enumerate() {
        let targets: Vec<u32> = match &ins.operand {
            Operand::Branch(t) => vec![*t],
            Operand::TableSwitch { default, targets, .. } => targets.iter().copied().chain([*default]).collect(),
            Operand::LookupSwitch { default, pairs } => pairs.iter().map(|p| p.1).chain([*default]).collect(),
            _ => vec![],
        };
        for t in targets {
            leader[at(t)?] = true;
        }
        if (!targets_empty(&ins.operand) || classfile::insn::is_terminal(ins.opcode)) && i + 1 < n {
            leader[i + 1] = true;
        }
    }
    for h in &code.exception_table {
        leader[at(h.handler)?] = true;
    }

    let mut entry: BTreeMap<usize, State> = BTreeMap::new();
    entry.insert(0, entry_state(owner, desc, is_static, code.max_locals, |i| oracle.param(i))?);
    let mut work: Vec<usize> = vec![0];
    let mut reachable = vec![false; n];
    let mut handler_on = vec![false; code.exception_table.len()];
    // 各指令所在的 try 区间（处理器下标）；处理器入口局部变量 = 区间内各指令前状态的并集
    let cover: Vec<Vec<usize>> = insns
        .iter()
        .map(|x| {
            code.exception_table
                .iter()
                .enumerate()
                .filter(|(_, h)| x.offset >= h.start && x.offset < h.end)
                .map(|(hi, _)| hi)
                .collect()
        })
        .collect();
    let mut hlocals: Vec<Option<Vec<V>>> = vec![None; code.exception_table.len()];
    let mut interp = Interp { oracle, emit: None, assumed: vec![], field_assumed: vec![], site_assumed: false, selects: 0 };
    let mut tr = track.then(init::Track::default);

    let merge = |entry: &mut BTreeMap<usize, State>, work: &mut Vec<usize>, i: usize, st: &State| -> Option<()> {
        match entry.get_mut(&i) {
            Some(old) => {
                if old.join(st).ok()? {
                    work.push(i);
                }
            }
            None => {
                entry.insert(i, st.clone());
                work.push(i);
            }
        }
        Some(())
    };

    // 键判定收窄（键读取方法 / 字符串相等判定取自清单事实）
    let key_test = |i: usize, st: &State, pre: Option<&[V]>| {
        narrow::key_test_narrow(insns, &leader, i, st, pre, |m| oracle.key_getter(m), |m| oracle.string_equality(m))
    };
    loop {
        while let Some(l) = work.pop() {
            let mut st = entry[&l].clone();
            let mut i = l;
            // 前一条指令（字符串相等判定）调用前的两个操作数（键判定收窄用，见 `narrow.rs`）
            let mut pre: Option<Vec<V>> = None;
            loop {
                reachable[i] = true;
                for &hi in &cover[i] {
                    match &mut hlocals[hi] {
                        Some(hl) => {
                            for (a, b) in hl.iter_mut().zip(st.locals.iter()) {
                                *a = a.join(b);
                            }
                        }
                        slot => *slot = Some(st.locals.clone()),
                    }
                }
                let ins = &insns[i];
                let cur = narrow::eq_operands(insns, &leader, i, &st, |m| oracle.string_equality(m).is_some());
                if let Some(t) = &mut tr {
                    t.pre(oracle, &mut st, ins, false);
                }
                let fl = interp.step(&mut st, ins).ok()?;
                narrow::final_reread(insns, &leader, i, &mut st);
                match fl {
                    Flow::Next => {
                        pre = cur;
                        if i + 1 >= n {
                            return None;
                        }
                        if leader[i + 1] {
                            merge(&mut entry, &mut work, i + 1, &st)?;
                            break;
                        }
                        i += 1;
                    }
                    Flow::Cond(t, k) => {
                        // instanceof 判定成立的一侧收窄被测局部变量
                        let narrow = narrow::instanceof_narrow(insns, &leader, i, &st)
                            .or_else(|| narrow::mirror_sub_narrow(insns, &leader, i, &st, |m| interp.oracle.mirror_subtype_test(m)))
                            .or_else(|| key_test(i, &st, pre.as_deref()));
                        // ifnull / ifnonnull 两侧收窄被测局部变量的可空性；前后缀判定成立一侧收窄形状
                        let nulls = narrow::null_narrow(insns, &leader, i, &st)
                            .or_else(|| strs::affix_narrow(insns, &leader, i, &st, |m| interp.oracle.str_kind(op::INVOKEVIRTUAL, m, false)));
                        let ffield = narrow::final_null_test(insns, &leader, i, |f| interp.oracle.final_field(f));
                        let edge = |taken: bool| {
                            let mut s2 = match (&narrow, &nulls) {
                                (Some(nw), _) => {
                                    let mut s2 = st.clone();
                                    s2.locals[nw.slot] = nw.side(taken).clone();
                                    s2
                                }
                                (None, Some((k, on_taken, on_next))) => {
                                    let mut s2 = st.clone();
                                    s2.locals[*k] = if taken { on_taken.clone() } else { on_next.clone() };
                                    s2
                                }
                                (None, None) => st.clone(),
                            };
                            if let Some((k, f, nn_taken)) = &ffield {
                                if taken == *nn_taken && !s2.nnf.iter().any(|(j, g)| j == k && g == f) {
                                    s2.nnf.push((*k, f.clone()));
                                }
                            }
                            s2
                        };
                        if k != Some(false) {
                            merge(&mut entry, &mut work, at(t)?, &edge(true))?;
                        }
                        if k != Some(true) {
                            if i + 1 >= n {
                                return None;
                            }
                            merge(&mut entry, &mut work, i + 1, &edge(false))?;
                        }
                        break;
                    }
                    Flow::Goto(t) => {
                        merge(&mut entry, &mut work, at(t)?, &st)?;
                        break;
                    }
                    Flow::Switch(ts) => {
                        for t in ts {
                            merge(&mut entry, &mut work, at(t)?, &st)?;
                        }
                        break;
                    }
                    Flow::End => break,
                }
            }
        }
        // 异常处理器：try 区间有可达指令且 catch 类型存活 → 进入（局部变量 = 区间内前状态并集，
        // 栈 = 异常对象）；已进入的处理器随区间状态增长重新合并
        for (hi, h) in code.exception_table.iter().enumerate() {
            let Some(hl) = &hlocals[hi] else { continue };
            if !handler_on[hi] {
                if let Some(ct) = &h.catch_type {
                    if !oracle.type_live(ct) {
                        continue;
                    }
                }
                handler_on[hi] = true;
            }
            let ty: Rc<str> = Rc::from(h.catch_type.as_deref().unwrap_or("java/lang/Throwable"));
            let mut hl = hl.clone();
            strs::strip_builders(&mut hl);
            let st = State {
                locals: hl,
                stack: vec![V::Ref { ty: Some(ty), nonnull: true, src: src1(Src::Catch(h.handler)), obj: None }],
                finals: Vec::new(),
                nnf: Vec::new(),
                inits: Vec::new(),
            };
            merge(&mut entry, &mut work, at(h.handler)?, &st)?;
        }
        if work.is_empty() {
            break;
        }
    }

    // 第二阶段：按不动点入口状态产出事件
    let mut events = Vec::new();
    interp.emit = Some(&mut events);
    for (&l, st0) in &entry {
        let mut st = st0.clone();
        let mut i = l;
        let mut pre: Option<Vec<V>> = None;
        loop {
            let ins = &insns[i];
            let cur = narrow::eq_operands(insns, &leader, i, &st, |m| oracle.string_equality(m).is_some());
            if let Some(t) = &mut tr {
                t.pre(oracle, &mut st, ins, true);
            }
            let fl = interp.step(&mut st, ins).ok()?;
            narrow::final_reread(insns, &leader, i, &mut st);
            match fl {
                Flow::Next if i + 1 < n && !leader[i + 1] => {
                    pre = cur;
                    i += 1;
                    continue;
                }
                // 收窄值以事件给出的一侧可达（instanceof 不成立一侧、类镜像子类型判定成立一侧）：发其来源事件
                Flow::Cond(_, k) => {
                    let narrow = narrow::instanceof_narrow(insns, &leader, i, &st)
                        .or_else(|| narrow::mirror_sub_narrow(insns, &leader, i, &st, |m| interp.oracle.mirror_subtype_test(m)))
                        .or_else(|| key_test(i, &st, pre.as_deref()));
                    if let Some(nw) = narrow {
                        if k != Some(!nw.event_when) {
                            interp.ev(nw.event.0, nw.event.1);
                        }
                    }
                    break;
                }
                _ => break,
            }
        }
    }
    let mut mirror_assumed = std::mem::take(&mut interp.assumed);
    let mut mirror_field_assumed = std::mem::take(&mut interp.field_assumed);
    mirror_field_assumed.sort();
    mirror_field_assumed.dedup();
    let selector_params = interp.selects;
    let site_mirror_assumed = interp.site_assumed;
    drop(interp);
    mirror_assumed.sort();
    let mut pending_types: Vec<String> = insns
        .iter()
        .enumerate()
        .filter(|(i, x)| reachable[*i] && x.opcode == op::INSTANCEOF)
        .filter_map(|(_, x)| match &x.operand {
            Operand::Class(c) if !c.starts_with('[') && !oracle.type_live(c) => Some(c.clone()),
            _ => None,
        })
        .collect();
    for (hi, h) in code.exception_table.iter().enumerate() {
        if handler_on[hi] {
            events.push((h.handler, Event::Catch(h.catch_type.clone())));
        } else if let Some(ct) = &h.catch_type {
            let live_range = insns.iter().enumerate().any(|(i, x)| reachable[i] && x.offset >= h.start && x.offset < h.end);
            if live_range {
                pending_types.push(ct.clone());
            }
        }
    }
    events.sort_by_key(|e| e.0);
    pending_types.sort();
    pending_types.dedup();
    let a = Analysis { reachable, events, pending_types, mirror_assumed, mirror_field_assumed, site_mirror_assumed, conservative: false, cfg: Rc::new(cfg::Cfg::build(code)), selector_params };
    Some((a, tr.map(init::Track::finish)))
}

fn targets_empty(o: &Operand) -> bool {
    !matches!(o, Operand::Branch(_) | Operand::TableSwitch { .. } | Operand::LookupSwitch { .. })
}
