//! 引擎：结果查询与诊断输出（folds、实例化集、`--flows`、分派点）。

use super::*;

impl<'a> Engine<'a> {
    // ── 结果查询 ────────────────────────────────────────────────────────────

    /// 折叠点导出（folds v2）：不可达指令区间、不进入的异常处理器、死 catch 表项、折叠为常量的读取点。
    /// 按方法标签排序；只含有折叠内容的字节码方法
    /// 同一成员的各克隆合并：任一克隆可达即可达，常量须在其可达的全部克隆里一致
    pub fn folds(&self) -> Vec<Fold> {
        let mut groups: IndexMap<&MemberRef, Vec<(usize, Option<Rc<Analysis>>)>> = IndexMap::default();
        for (i, mn) in self.methods.values().enumerate() {
            if mn.kind == Kind::Bytecode {
                let a = if self.is_concrete(i) { self.concrete_analysis(&mn.key) } else { mn.analysis.clone() };
                groups.entry(&mn.key).or_default().push((i, a));
            }
        }
        let um = self.unmodeled();
        let mut out = Vec::new();
        for (key, group) in groups {
            let clones: Vec<usize> = group.iter().map(|(i, _)| *i).collect();
            let Some(all) = group.into_iter().map(|(_, a)| a).collect::<Option<Vec<_>>>() else { continue };
            if all.iter().any(|a| a.conservative) {
                continue;
            }
            let Some(cf) = self.h.class(&key.owner) else { continue };
            let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { continue };
            let mut f = fold_of(key.to_string(), code, &all);
            // 具体轨迹里抛出异常的调用：其后继不可达（活调用顺序落入死区），按不返回的调用点导出
            let mut thrown: Vec<u32> = Vec::new();
            if clones.iter().any(|&i| self.is_concrete(i)) {
                let index: HashMap<u32, &classfile::Insn> = code.insns.iter().map(|x| (x.offset, x)).collect();
                thrown = f.violations.iter().copied().filter(|pc| index.get(pc).is_some_and(|x| matches!(x.operand, classfile::Operand::Method(..)))).collect();
                f.violations.retain(|pc| !thrown.contains(pc));
            }
            self.init_reads(&key.owner, code, &mut f);
            self.dead_catches(code, &mut f);
            f.null_recv = self.null_recv(&clones, &um);
            self.noreturn_calls(code, &all, &thrown, &mut f);
            f.props = self.prop_folds(&f, &all);
            f.direct_calls = self.direct_calls(&clones, &all, code, &f);
            // 自检：活指令顺序落入 dead_pcs（folds 规则禁止），出现即分析缺陷
            if !f.violations.is_empty() {
                eprintln!("[closure] folds 自检违约：{} @{:?}", f.method, f.violations);
            }
            if !f.dead_pcs.is_empty() || !f.dead_handlers.is_empty() || !f.dead_catches.is_empty() || !f.consts.is_empty() || !f.null_recv.is_empty() || !f.noreturn_calls.is_empty() || !f.direct_calls.is_empty() {
                out.push(f);
            }
        }
        out.sort_by(|a, b| a.method.cmp(&b.method));
        out
    }

    /// 直连反射调用点：每个到达该点的克隆都按直连处理且特化入口相同（未分析的克隆不导出），
    /// 且该点不按 null_recv / noreturn / 常量导出，也不落在死区（dead_pcs / noreturn_dead_pcs：如前序调用定论不返回）
    fn direct_calls(&self, clones: &[usize], all: &[Rc<Analysis>], code: &classfile::Code, f: &Fold) -> Vec<(u32, MemberRef)> {
        let pcs: BTreeSet<u32> = self.rdirect.keys().filter(|(i, _)| clones.contains(i)).map(|(_, pc)| *pc).collect();
        let dead = |pc: u32| f.dead_pcs.iter().chain(&f.noreturn_dead_pcs).any(|&(s, e)| s <= pc && pc < e);
        let mut out = Vec::new();
        for pc in pcs {
            if dead(pc) || f.null_recv.contains(&pc) || f.noreturn_calls.contains(&pc) || f.consts.iter().any(|c| c.0 == pc) {
                continue;
            }
            let Ok(idx) = code.insns.binary_search_by_key(&pc, |x| x.offset) else { continue };
            let mut helper: Option<&MemberRef> = None;
            let mut ok = true;
            for (&i, a) in clones.iter().zip(all) {
                if !a.reachable.get(idx).copied().unwrap_or(false) {
                    continue;
                }
                match (self.direct_call_of(i, pc), helper) {
                    (Some(h), None) => helper = Some(h),
                    (Some(h), Some(prev)) if h == prev => {}
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            if let (true, Some(h)) = (ok, helper) {
                out.push((pc, h.clone()));
            }
        }
        out
    }

    /// 折叠所用的系统属性表（清单 `[facts.system_properties]`）：运行时初始属性表与之同源
    pub fn sysprops(&self) -> &crate::manifest::SysProps {
        &self.man.sysprops
    }

    /// 折叠常量里来自系统属性读取的调用点
    fn prop_folds(&self, f: &Fold, all: &[Rc<Analysis>]) -> Vec<u32> {
        f.consts
            .iter()
            .filter(|c| {
                all.iter().flat_map(|a| a.events.iter()).any(|(pc, e)| {
                    *pc == c.0 && matches!(e, Event::Invoke { opcode, mref, iface, .. } if self.ctx.read_spec(None, *opcode, mref, *iface, None).is_some())
                })
            })
            .map(|c| c.0)
            .collect()
    }

    pub fn instantiated(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .g
            .iter()
            .filter(|i| !self.lambdas.contains_key(i) && !self.hwobjs.contains_key(i))
            .map(|i| self.names[*i as usize].to_string())
            .filter(|n| !n.starts_with('['))
            .collect();
        v.sort();
        v
    }

    /// 字段折叠状态：(按字段句柄写字段的入口可达, 全部字段不折叠)
    pub fn field_handles(&self) -> (bool, bool) {
        (self.fwriter_live, self.ctx.fopen_all.get())
    }

    /// 标记已流到句柄写入口的枚举口径（`类` / `*` = 推不出；可序列化字段口径加 `:serial`）
    pub fn field_writer_cause(&self) -> Option<&str> {
        self.fwriter_cause.as_deref()
    }

    /// 句柄存取不再按来源标记建模的原因（字段句柄取得入口经非字节码调用点可达，见 `field_access.rs`）
    pub fn field_access_untrusted(&self) -> Option<&str> {
        self.fa_untrusted.as_deref()
    }

    pub fn field_handle_released(&self) -> Vec<String> {
        self.fh_released.iter().map(|(s, c)| format!("{}{}", c.as_deref().unwrap_or("*"), if *s { ":serial" } else { "" })).collect()
    }

    pub fn lambda_count(&self) -> usize {
        self.lambdas.len()
    }

    pub(super) fn set_str(&self, s: &TypeSet) -> String {
        let mut v: Vec<String> = if s.classes.len() > 6 {
            vec![format!("{} 个类", s.classes.len())]
        } else {
            s.classes.iter().map(|i| self.names[i as usize].to_string()).collect()
        };
        v.extend(s.open.iter().map(|i| format!("open({})", self.names[i as usize])));
        v.join(", ")
    }

    pub(super) fn hub_label(&self, h: u32) -> String {
        let hub = &self.hubs[h as usize];
        let (o, n, d) = hub.site.key();
        match hub.open {
            Some(x) => format!("{o}.{n}:{d} on open({})", self.names[x as usize]),
            None => format!("{o}.{n}:{d} on {} 个接收者", hub.set.as_ref().map_or(hub.recvs.len(), |s| s.len())),
        }
    }

    pub(super) fn node_str(&self, n: Node) -> String {
        match n {
            Node::P(m, i) => format!("P{i} {}", self.ctx_label(m)),
            Node::R(m) => format!("R {}", self.ctx_label(m)),
            Node::S(m, o) if o == POOL => format!("pool {}", self.ctx_label(m)),
            Node::S(m, o) if o == PROD => format!("prod {}", self.ctx_label(m)),
            Node::S(m, o) if o == CALLER => format!("callers {}", self.ctx_label(m)),
            Node::S(m, o) if o & CATCH != 0 => format!("catch@{} {}", o & !CATCH, self.ctx_label(m)),
            Node::S(m, o) => format!("@{o} {}", self.ctx_label(m)),
            Node::F(f) => format!("field {}", self.field_label(f)),
            Node::U(f) => format!("field? {}", self.field_label(f)),
            Node::O(o, f) => format!("field {} of {}", self.field_label(f), self.names[o as usize]),
            Node::E(x, p) => format!("elements[{}] {}", if p == 0 { "偶" } else { "奇" }, self.names[x as usize]),
            Node::Array => "array".into(),
            Node::Esc => "escape".into(),
            Node::K(g) => format!("keyed-gate {}", self.kgate_label(g)),
            Node::NR(r) => format!("by-name receivers {}", self.name_recv_label(r)),
            Node::RP(c) => format!("reflect-call 实参池 {}", reflect_call::channel_name(c)),
            Node::RN(c) => format!("reflect-call 实参池（去冗余） {}", reflect_call::channel_name(c)),
            Node::RA(c) => format!("reflect-call 实参数组 {}", reflect_call::channel_name(c)),
            Node::HP(h, i) => format!("hub 实参{i} {}", self.hub_label(h)),
            Node::HR(h) => format!("hub 返回 {}", self.hub_label(h)),
            Node::G(g) => {
                let (slot, n, put) = self.gathers[g as usize];
                match slot {
                    gather::Slot::Field(fi) => format!("field {} {} {n} objects", self.field_label(fi), if put { "into" } else { "of" }),
                    gather::Slot::Elem(p) => format!("elements[{}] of {n} arrays", if p == 0 { "偶" } else { "奇" }),
                }
            }
            Node::A(s, i) | Node::W(s, i) => {
                let (m, off, t) = self.hw_sites[s as usize];
                let k = if matches!(n, Node::A(..)) { "实参" } else { "写入" };
                format!("{k}{i} {}@{off} → {}", self.ctx_label(m), self.methods[t].key)
            }
        }
    }

    /// 类型流诊断：方法（标签含 `pat`）的形参 / 返回节点的类型集，以及流入它们的来源节点
    pub fn flows_of(&self, pat: &str) -> Vec<String> {
        if let Some(r) = self.diag_open(pat) {
            return r;
        }
        let mut out = Vec::new();
        if let Some(q) = pat.strip_prefix("elem:") {
            let mut es: Vec<Node> = self.graph.keys().filter(|n| matches!(n, Node::E(x, _) if self.names[*x as usize].contains(q))).copied().collect();
            es.sort_by_key(|n| format!("{n:?}"));
            for n in es {
                let s = self.graph.get(&n).cloned().unwrap_or_default();
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(&s)));
                for (src, edges) in &self.graph.flow_list() {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.graph.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.filter_label(*f), self.set_str(&s)));
                        }
                    }
                }
            }
            return out;
        }
        // 上下文值集诊断：`@ctxsets:<方法键子串>`——匹配方法的上下文克隆数，及按形参值集元素数之和排序的前 25 个克隆
        // （各形参 / 返回值的元素数、调用方数与前 4 个调用方）
        if let Some(q) = pat.strip_prefix("@ctxsets:") {
            let mut v: Vec<(usize, String)> = Vec::new();
            let mut n = 0usize;
            for (i, mn) in self.methods.values().enumerate() {
                if !mn.key.to_string().contains(q) {
                    continue;
                }
                n += 1;
                let size = |e: &Self, nd: Node| e.graph.get(&nd).map_or(0, |s| s.classes.len() + s.open.len());
                let ps: Vec<usize> = (0..mn.ptypes.len()).map(|j| size(self, Node::P(i, j as u16))).collect();
                let r = size(self, Node::R(i));
                let cs: Vec<usize> = self.callers.get(&i).into_iter().flatten().copied().collect();
                let cl: Vec<String> = cs.iter().take(4).map(|&c| self.ctx_label(c)).collect();
                let tot = ps.iter().sum::<usize>() + r;
                v.push((tot, format!("  {tot}\tP {ps:?} R {r} 调用方 {} {cl:?}\t{}", cs.len(), self.ctx_label(i))));
            }
            v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            out.push(format!("@ctxsets {q}: {n} 个克隆"));
            out.extend(v.into_iter().take(25).map(|x| x.1));
            return out;
        }
        // 污染路径诊断：`@path:<节点子串>|<类名>`——从匹配节点沿流边反向，经含该类的节点走到源头（最短路径）
        if let Some((np, cls)) = pat.strip_prefix("@path:").and_then(|v| v.split_once('|')) {
            let (open, cls) = match cls.strip_prefix("open:") {
                Some(c) => (true, c),
                None => (false, cls),
            };
            let Some(&cid) = self.ids.get(cls) else { return vec![format!("无此类：{cls}")] };
            let has = |x: &Node| self.graph.get(x).is_some_and(|s| if open { s.open.contains(&cid) } else { s.classes.contains(&cid) });
            let mut rev: HashMap<Node, Vec<Node>> = HashMap::default();
            for (src, edges) in &self.graph.flow_list() {
                for (dst, _) in edges {
                    rev.entry(*dst).or_default().push(*src);
                }
            }
            let starts: Vec<Node> = self.graph.keys().filter(|n| has(n) && self.node_str(**n).contains(np)).copied().collect();
            let mut prev: HashMap<Node, Option<Node>> = HashMap::default();
            let mut q: VecDeque<Node> = VecDeque::new();
            for n in starts.into_iter().take(1) {
                prev.insert(n, None);
                q.push_back(n);
            }
            let mut last = None;
            while let Some(n) = q.pop_front() {
                last = Some(n);
                // 最近的引入点（没有含该类的前驱）即源头
                if !rev.get(&n).is_some_and(|v| v.iter().any(|p| has(p))) {
                    break;
                }
                let mut ps: Vec<Node> = rev.get(&n).map(|v| v.iter().filter(|p| has(p) && !prev.contains_key(*p)).copied().collect()).unwrap_or_default();
                ps.sort_by_key(|p| format!("{p:?}"));
                for p in ps {
                    prev.insert(p, Some(n));
                    q.push_back(p);
                }
            }
            // 源头（或最远节点）往回打印到起点
            let mut cur = last;
            while let Some(n) = cur {
                let sz = self.graph.get(&n).map_or(0, |s| s.classes.len());
                out.push(format!("  {} (|{sz}|)", self.node_str(n)));
                cur = prev.get(&n).copied().flatten();
            }
            return out;
        }
        // open 统计诊断：`@openstat`——各 open 类型：含它的节点数、引入点数、按 G 展开的类数
        if pat == "@openstat" {
            let mut fed: HashSet<(Node, u32)> = HashSet::default();
            for (src, edges) in &self.graph.flow_list() {
                if let Some(ss) = self.graph.get(src) {
                    for o in &ss.open {
                        for (dst, _) in edges {
                            fed.insert((*dst, o));
                        }
                    }
                }
            }
            let mut stat: HashMap<u32, (usize, Vec<String>)> = HashMap::default();
            for (n, ss) in self.graph.iter() {
                for o in &ss.open {
                    let e = stat.entry(o).or_default();
                    e.0 += 1;
                    if !fed.contains(&(*n, o)) {
                        e.1.push(self.node_str(*n));
                    }
                }
            }
            let g: Vec<u32> = self.g.iter().copied().collect();
            let mut v: Vec<(usize, String)> = Vec::new();
            for (o, (cnt, intro)) in stat {
                let on = &self.names[o as usize];
                let exp = g.iter().filter(|&&x| &*self.names[x as usize] == &**on || self.h.is_subtype(&self.names[x as usize], on)).count();
                let mut intro = intro;
                intro.sort();
                let head: Vec<String> = intro.iter().take(4).map(|x| x.chars().take(140).collect()).collect();
                v.push((exp * cnt, format!("  {} 节点 {cnt} 展开 {exp} 引入 {}：{}", self.names[o as usize], intro.len(), head.join(" ｜ "))));
            }
            v.sort_by(|a, b| b.0.cmp(&a.0));
            return v.into_iter().take(40).map(|x| x.1).collect();
        }
        // open 源头诊断：`@opens:<类型>`——含 open(类型)、但没有任何含同一 open 的前驱的节点（open 的引入点）
        // 来源诊断：`@opens:<类型>` 为 open(类型) 的引入点，`@srcs:<类>` 为含该类（非 open）的引入点（直接播种、无同类前驱的节点）
        let src_q = pat.strip_prefix("@opens:").map(|q| (q, true)).or_else(|| pat.strip_prefix("@srcs:").map(|q| (q, false)));
        if let Some((q, open)) = src_q {
            let Some(&cid) = self.ids.get(q) else { return vec![format!("无此类：{q}")] };
            let has = |x: &Node| self.graph.get(x).is_some_and(|s| if open { s.open.contains(&cid) } else { s.classes.contains(&cid) });
            let mut fed: HashSet<Node> = HashSet::default();
            for (src, edges) in &self.graph.flow_list() {
                if has(src) {
                    for (dst, _) in edges {
                        fed.insert(*dst);
                    }
                }
            }
            let mut v: Vec<String> = self.graph.keys().filter(|n| has(n) && !fed.contains(*n)).map(|n| format!("  {}", self.node_str(*n))).collect();
            v.sort();
            return v;
        }
        // 字段不折叠来源诊断：`@fopen:<字段名子串>`——全局开关、按名放开、按键放开、手写写入中匹配者
        if let Some(q) = pat.strip_prefix("@fopen:") {
            let c = &self.ctx;
            out.push(format!("@fopen all={} deser={}", c.fopen_all.get(), c.deser.get()));
            let mut v: Vec<String> = c.fopen_names.borrow().iter().filter(|n| n.contains(q)).map(|n| {
                let via = self.fopen_name_via.get(n).copied().flatten().map_or("站点外".into(), |(m, off)| format!("{}@{off}", self.method_label(m)));
                format!("  name {n} ← {via}")
            }).collect();
            v.extend(c.fopen.borrow().iter().map(|k| k.to_string()).filter(|k| k.contains(q)).map(|k| format!("  key {k}")));
            v.extend(c.fhw.borrow().iter().map(|k| k.to_string()).filter(|k| k.contains(q)).map(|k| format!("  hw {k}")));
            v.extend(c.fhw_names.borrow().iter().filter(|n| n.contains(q)).map(|n| format!("  hwname {n}")));
            v.sort();
            out.extend(v);
            return out;
        }
        // 前驱诊断：`@in:<节点标签>`——标签恰为该串的节点的流入边源，按源节点所在方法 / 字段归并计数（前 60）
        if let Some(q) = pat.strip_prefix("@in:") {
            let mut by: HashMap<String, usize> = HashMap::default();
            for (src, edges) in &self.graph.flow_list() {
                if edges.iter().any(|(d, _)| self.node_str(*d) == q) {
                    let l = self.node_str(*src);
                    let k = l.split(" #").next().unwrap_or(&l).to_string();
                    *by.entry(k).or_default() += 1;
                }
            }
            let mut v: Vec<(String, usize)> = by.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            out.push(format!("@in {q}: {} 个源分组", v.len()));
            out.extend(v.into_iter().take(60).map(|(k, n)| format!("  {n}\t{k}")));
            return out;
        }
        // 常量诊断：`@vals:<方法标签子串>`——匹配方法节点（前 12 个）的形参常量、静态调用点答复表、节点 / 成员返回值与常量事件
        if let Some(q) = pat.strip_prefix("@vals:") {
            let ms: Vec<usize> = (0..self.methods.len()).filter(|&m| self.method_label(m).contains(q)).take(12).collect();
            for m in ms {
                let key = &self.methods[m].key;
                out.push(format!("  {}", self.method_label(m)));
                out.push(format!("    pvals {:?}", self.pvals.get(&m)));
                let cs: Vec<String> = self.callers.get(&m).into_iter().flatten().take(12).map(|&c| self.method_label(c)).collect();
                out.push(format!("    callers {cs:?}"));
                out.push(format!("    sites {:?}", self.site_used.get(&m)));
                out.push(format!("    sret {:?} nret {:?} rvals {:?}", self.sret.get(&m), self.nret.get(&m), self.ctx.rvals.borrow().get(key)));
                if let Some((mb, unwrapped)) = self.caller_diag(m) {
                    let s = self.set_of(Node::S(mb, CALLER));
                    let el: Vec<String> = s.classes.iter().take(24).map(|x| match self.mirrors.get(&x) {
                        Some(&t) => format!("{}", self.names[t as usize]),
                        None => format!("?{}", self.names[x as usize]),
                    }).collect();
                    out.push(format!("    caller_set n={} open={} unwrapped={} {el:?}", s.classes.len(), s.open.len(), unwrapped));
                }
                if let Some(a) = self.site_analysis(m) {
                    let evs: Vec<String> = a.events.iter().filter_map(|(o, e)| match e {
                        absint::Event::Const { value, .. } => Some(format!("@{o}={value:?}")),
                        absint::Event::Return(v) => Some(format!("@{o} ret {v:?}")),
                        absint::Event::Invoke { opcode, mref, iface, args } => {
                            // 本调用点按常量实参求值的结果（诊断用，不登记依赖）
                            let t = self.ctx.call_info(*opcode, mref, *iface).target.clone();
                            let ev = t.as_ref().map(|t| self.ctx.const_eval(None, t, args));
                            let tr = match (&t, &ev) {
                                (Some(t), Some(None)) if args.iter().any(|a| a.obj().is_some()) => format!(" trace[{}]", self.ctx.ceval_trace(t, args)),
                                _ => String::new(),
                            };
                            // 按对象返回值（实例调用）：目标的对象值 / 通配值
                            // 按对象返回值（实例调用）：同名同描述符各目标的对象值 / 通配值
                            let orv = if *opcode != classfile::op::INVOKESTATIC {
                                let same = |k: &MemberRef| k.name == mref.name && k.desc == mref.desc;
                                let ov: Vec<String> = self.ctx.orvals.borrow().iter().filter(|(k, _)| same(k)).flat_map(|(k, ov)| {
                                    ov.iter().take(6).map(move |(o, v)| format!("{}:{}={v:?}", k.owner, self.names[*o as usize]))
                                }).take(12).collect();
                                let ow: Vec<String> = self.ctx.orwild.borrow().iter().filter(|(k, _)| same(k)).map(|(k, v)| format!("{}={v:?}", k.owner)).take(6).collect();
                                let objs: Vec<String> = self.set_of(Node::P(m, 0)).classes.iter().filter(|x| self.objs.contains_key(x)).take(6).map(|o| self.names[o as usize].to_string()).collect();
                                format!(" orv {ov:?} orw {ow:?} this {objs:?}")
                            } else {
                                String::new()
                            };
                            let dv = if t.is_none() { self.ctx.deval_diag(m, key, *opcode, *o, mref, args) } else { String::new() };
                            Some(format!("@{o} {}{args:?} ceval {ev:?}{tr}{orv}{dv}", mref.name))
                        }
                        absint::Event::Field { mref, .. } => self.ctx.field_info(mref).map(|fi| {
                            let fopen = self.ctx.fopen.borrow().contains(&fi.key) || self.ctx.fopen_names.borrow().contains(&fi.key.name);
                            format!("@{o} field {} open {} fopen {fopen} hw {}", fi.key, fi.open, self.ctx.hw_written(&fi))
                        }),
                        _ => None,
                    }).collect();
                    out.push(format!("    conservative {} events {}", a.conservative, evs.join(" ")));
                }
            }
            return out;
        }
        // 服务查找诊断：`@svcunk`——服务 Class 实参所指未知的查找站点
        if pat == "@svcunk" {
            for &(m, off) in &self.seeds.services.unknown_sites {
                out.push(format!("  {}@{off}", self.methods[m].key));
            }
            return out;
        }
        // 值集明细诊断：`@set:<节点子串>`——匹配节点（前 8 个）的值集按成员类别列出（分配点数组 / 抽象对象 / 其余类 / open）
        if let Some(q) = pat.strip_prefix("@set:") {
            let mut ns: Vec<Node> = self.graph.keys().filter(|n| self.node_str(**n).contains(q)).copied().collect();
            ns.sort_by_key(|n| format!("{n:?}"));
            for n in ns.into_iter().take(8) {
                let s = self.graph.get(&n).cloned().unwrap_or_default();
                let (mut arr, mut obj, mut other) = (0, 0, Vec::new());
                for c in s.classes.iter() {
                    if self.arrays.contains_key(&c) {
                        arr += 1;
                    } else if self.objs.contains_key(&c) {
                        obj += 1;
                    } else {
                        other.push(self.names[c as usize].to_string());
                    }
                }
                let opens: Vec<String> = s.open.iter().map(|o| self.names[o as usize].to_string()).collect();
                out.push(format!("  {}：数组分配点 {arr}、抽象对象 {obj}、open {opens:?}、其余 {}：{}", self.node_str(n), other.len(), other.iter().take(40).cloned().collect::<Vec<_>>().join(" ")));
            }
            return out;
        }
        // 抽象对象构成诊断：`@objstat`——抽象对象 / 数组分配点按（类型, 来源）计数（来源：映像逐对象 / 字节码分配点），
        // 另计各类型的逃逸数与按 G 计的成员数（前 60）
        if pat == "@objstat" {
            let mut by: HashMap<(u32, bool, bool), (usize, usize)> = HashMap::default();
            for (&id, &t) in self.objs.iter().chain(self.arrays.iter()) {
                let name = &self.names[id as usize];
                let img = name.contains("@image");
                let e = by.entry((t, self.arrays.contains_key(&id), img)).or_default();
                e.0 += 1;
                e.1 += usize::from(self.escaped.contains(&id));
            }
            let (mut ti, mut tb) = (0, 0);
            let mut v: Vec<(usize, String)> = Vec::new();
            for ((t, arr, img), (n, esc)) in by {
                if img {
                    ti += n;
                } else {
                    tb += n;
                }
                let kind = if img { "映像" } else { "字节码" };
                let shape = if arr { "数组" } else { "对象" };
                v.push((n, format!("  {n}\t逃逸 {esc}\t{kind}{shape}\t{}", self.names[t as usize])));
            }
            v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            out.push(format!("@objstat 映像 {ti}、字节码 {tb}、G {}", self.g.len()));
            out.extend(v.into_iter().take(60).map(|x| x.1));
            return out;
        }
        // 抽象对象明细诊断：`@objs:<节点子串>`——匹配节点（前 4 个）值集里的抽象对象 / 数组分配点名（前 40，标逃逸）
        if let Some(q) = pat.strip_prefix("@objs:") {
            let mut ns: Vec<Node> = self.graph.keys().filter(|n| self.node_str(**n).contains(q)).copied().collect();
            ns.sort_by_key(|n| format!("{n:?}"));
            for n in ns.into_iter().take(4) {
                let s = self.graph.get(&n).cloned().unwrap_or_default();
                out.push(format!("  {}：", self.node_str(n)));
                for c in s.classes.iter().filter(|c| self.objs.contains_key(c) || self.arrays.contains_key(c)).take(40) {
                    let esc = if self.escaped.contains(&c) { " [逃逸]" } else { "" };
                    out.push(format!("    {}{esc}", self.names[c as usize]));
                }
            }
            return out;
        }
        // 按对象字段值诊断：`@ofield:<字段键子串>`——通配值 owild 与各抽象对象的值（`obj_fields.rs`）；
        // 映像对象另列该字段在映像中的原始值与对象的占位 / 延迟标记
        if let Some(q) = pat.strip_prefix("@ofield:") {
            let mut out: Vec<String> = Vec::new();
            let owild = self.ctx.owild.borrow();
            let ovals = self.ctx.ovals.borrow();
            let mut keys: Vec<&MemberRef> = ovals.keys().chain(owild.keys()).filter(|k| k.to_string().contains(q)).collect();
            keys.sort();
            keys.dedup();
            for k in keys.into_iter().take(8) {
                out.push(format!("  {k}：owild {:?}", owild.get(k)));
                let mut vs: Vec<(String, String)> = ovals.get(k).into_iter().flatten().map(|(&o, v)| (self.names[o as usize].to_string(), format!("{v:?}"))).collect();
                vs.sort();
                for (n, v) in vs.into_iter().take(60) {
                    let mut img = String::new();
                    if let (Some(i), Some(s)) = (n.rsplit_once("@image").and_then(|(_, i)| i.parse::<usize>().ok()), self.img.as_ref()) {
                        if let Some(x) = s.data.objs.get(i) {
                            let raw = match &x.body {
                                crate::image::IBody::Inst(fs) => fs.iter().find(|(d, f, _)| *d == k.owner && *f == k.name).map(|e| format!("{:?}", e.2)),
                                crate::image::IBody::Arr(_) => None,
                            };
                            img = format!(" 映像原值 {raw:?} 占位 {} 延迟 {}", x.placeholder, x.deferred.is_some());
                        }
                    }
                    out.push(format!("    {n} = {v}{img}"));
                }
            }
            return out;
        }
        // 方法节点序号诊断：`@m:<序号>`（数组分配点名里的方法序号）
        if let Some(i) = pat.strip_prefix("@m:").and_then(|v| v.parse::<usize>().ok()) {
            return vec![format!("  {i} = {}", self.ctx_label(i))];
        }
        // 调用方诊断：`@callers:<方法子串>`——列出 NOCTX 方法本体的全部调用点
        if let Some(q) = pat.strip_prefix("@callers:") {
            let mut v: Vec<String> = Vec::new();
            for (&(m, off), ts) in &self.dispatch {
                for &t in ts.iter() {
                    if self.methods[t].ctx == NOCTX && self.method_label(t).contains(q) {
                        v.push(format!("  {} ← {} @{off}", self.method_label(t), self.ctx_label(m)));
                    }
                }
            }
            v.sort();
            return v;
        }
        // 汇合点诊断：值集 ≥ N 的节点中，由小值集（< N）来源直接汇入的类最多者（污染的起始汇点）
        if let Some(n) = pat.strip_prefix("@merge:").and_then(|v| v.parse::<usize>().ok()) {
            let size = |x: &Node| self.graph.get(x).map_or(0, |s| s.classes.len());
            let mut inc: HashMap<Node, (IdSet, usize)> = HashMap::default();
            for (src, edges) in &self.graph.flow_list() {
                if size(src) >= n {
                    continue;
                }
                for (dst, _) in edges {
                    if size(dst) >= n {
                        let e = inc.entry(*dst).or_default();
                        e.1 += 1;
                        if let Some(s) = self.graph.get(src) {
                            for c in s.classes.iter() {
                                e.0.insert(c);
                            }
                        }
                    }
                }
            }
            let mut v: Vec<_> = inc.into_iter().collect();
            v.sort_by_key(|(k, (c, _))| (std::cmp::Reverse(c.len()), format!("{k:?}")));
            for (k, (c, k2)) in v.into_iter().take(40) {
                out.push(format!("  {} 类 / {} 来源 → {} (|{}|)", c.len(), k2, self.node_str(k), size(&k)));
            }
            return out;
        }
        // 反射调用实参池诊断：`@rcall`——各通道实参池 / 实参数组的值集
        if pat == "@rcall" {
            let mut v: Vec<Node> = self.graph.keys().filter(|n| matches!(n, Node::RA(_) | Node::RP(_) | Node::RN(_))).copied().collect();
            v.sort_by_key(|n| format!("{n:?}"));
            for n in v {
                let s = self.graph.get(&n).cloned().unwrap_or_default();
                out.push(format!("  {} (|{}| + open {}) = {{{}}}", self.node_str(n), s.classes.len(), s.open.len(), self.set_str(&s)));
            }
            return out;
        }
        // 未收窄的按偏移写入站点诊断：`@hwopen`——写入目标实参含 open 的站点（目标 / 写入值规模与 open 类型）
        if pat == "@hvn" {
            let mut v = self.hvn_report();
            v.extend(self.tau_report());
            return v;
        }
        if pat == "@hwopen" {
            let mut v: Vec<String> = Vec::new();
            for (s, &(m, off, t)) in self.hw_sites.iter().enumerate() {
                let s = s as u32;
                if self.hw_offsets.get(&s).copied().flatten().is_some() {
                    continue;
                }
                let Some(ws) = self.hw_writes.get(&t) else { continue };
                for (j, w) in ws.iter().enumerate() {
                    if !w.as_ref().is_some_and(|w| w.fields) {
                        continue;
                    }
                    let a = self.graph.get(&Node::A(s, j as u16)).cloned().unwrap_or_default();
                    let wv = self.graph.get(&Node::W(s, j as u16)).cloned().unwrap_or_default();
                    let opens: Vec<String> = a.open.iter().map(|o| self.names[o as usize].to_string()).collect();
                    v.push(format!("  {}@{off} → {} 目标 |{}| open {:?} 写入 |{}|+{}", self.ctx_label(m), self.methods[t].key, a.classes.len(), opens, wv.classes.len(), wv.open.len()));
                }
            }
            v.sort();
            return v;
        }
        if pat == "@array" {
            let s = self.graph.get(&Node::Array).cloned().unwrap_or_default();
            out.push(format!("  array = {{{}}}", self.set_str(&s)));
            for (src, edges) in &self.graph.flow_list() {
                if edges.iter().any(|(d, _)| *d == Node::Array) {
                    let s = self.graph.get(src).cloned().unwrap_or_default();
                    out.push(format!("  array ← {} {{{}}}", self.node_str(*src), self.set_str(&s)));
                }
            }
            return out;
        }
        for (i, mn) in self.methods.values().enumerate() {
            if !mn.key.to_string().contains(pat) {
                continue;
            }
            out.push(self.ctx_label(i));
            let mut nodes: Vec<Node> = (0..mn.ptypes.len()).map(|j| Node::P(i, j as u16)).collect();
            nodes.push(Node::R(i));
            // 站点与该方法分配的数组元素
            let mut extra: Vec<Node> = self
                .graph
                .keys()
                .filter(|n| match **n {
                    Node::S(j, _) => j == i,
                    Node::E(x, _) => self.names[x as usize].contains(&format!("@{i}:")),
                    _ => false,
                })
                .copied()
                .collect();
            extra.sort_by_key(|n| format!("{n:?}"));
            nodes.extend(extra);
            for n in nodes {
                let Some(s) = self.graph.get(&n) else { continue };
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(s)));
                for (src, edges) in &self.graph.flow_list() {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.graph.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.filter_label(*f), self.set_str(&s)));
                        }
                    }
                }
            }
        }
        out
    }

    pub fn method_label(&self, i: usize) -> String {
        self.methods[i].key.to_string()
    }

    /// 带克隆上下文的方法标签（诊断用：`--why` 溯源链逐节点标出克隆上下文）
    pub(crate) fn ctx_label(&self, i: usize) -> String {
        match self.methods[i].ctx {
            NOCTX => self.method_label(i),
            c => format!("{} #{}", self.method_label(i), self.names[c as usize]),
        }
    }

    pub(super) fn field_label(&self, f: usize) -> String {
        self.fields.get_index(f).map(|(k, _)| k.to_string()).unwrap_or_default()
    }

    /// 方法节点按成员去重（克隆只是分析内部的上下文区分；输出按成员）。手写实现对象的伪方法不是 Java 成员，不输出
    pub fn method_nodes(&self) -> impl Iterator<Item = &MNode> {
        self.methods
            .values()
            .enumerate()
            .filter(|(i, m)| self.mbase[&m.key] == *i && !self.is_pseudo_method(*i))
            .map(|(_, m)| m)
    }

    /// 输出序的类表：按类名排序，与处理次序无关（计划 2026-09-30-closure-analyzer-performance.md §二 不变量）
    pub fn class_entries(&self) -> Vec<(&String, &ClassNode)> {
        let mut v: Vec<_> = self.classes.iter().collect();
        v.sort_unstable_by(|a, b| a.0.cmp(b.0));
        v
    }

    /// 输出序的方法表：按成员标识（`类.名:描述符`）排序
    pub fn method_entries(&self) -> Vec<&MNode> {
        let mut v: Vec<(String, &MNode)> = self.method_nodes().map(|m| (m.key.to_string(), m)).collect();
        v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        v.into_iter().map(|(_, m)| m).collect()
    }

    /// 输出序的类初始化集合：按类名排序
    pub fn clinit_list(&self) -> Vec<&String> {
        let mut v: Vec<&String> = self.inited.keys().collect();
        v.sort_unstable();
        v
    }

    pub fn method_count(&self) -> usize {
        self.method_nodes().count()
    }

    /// 经虚分派到达的实现（按成员合并克隆，标签排序）：全部活虚调用点（字节码 invokevirtual /
    /// invokeinterface，含单目标及按非虚处理的 final 方法 / final 类调用点；手写层回调；枢纽中转）的目标之并，
    /// 以及 VM 反射虚调用选中的实现。
    /// 生成器据此只为被派发到的实现占 vtable 槽（C3 第 5 项）；手写实现对象的方法不是 Java 方法，不输出
    pub fn dispatched(&self) -> Vec<String> {
        let canon: Vec<usize> = self.methods.values().map(|m| self.mbase[&m.key]).collect();
        let mut ids: BTreeSet<usize> = BTreeSet::new();
        for site in self.recv_sites.iter().chain(self.direct_virtual_sites.iter()).chain(self.hub_sites.keys()) {
            ids.extend(self.dispatch.get(site).into_iter().flatten().filter(|t| !self.is_pseudo_method(**t)).map(|&t| canon[t]));
        }
        let mut memo: HashMap<u32, Rc<[usize]>> = HashMap::default();
        for hs in self.hub_sites.values() {
            for &h in hs {
                ids.extend(self.hub_targets_canon(h, &canon, &mut memo).iter().copied());
            }
        }
        ids.extend(self.vm_targets.iter().map(|&t| canon[t]));
        let mut out: Vec<String> = ids.into_iter().map(|i| self.method_label(i)).collect();
        out.sort_unstable();
        out.dedup();
        out
    }

    /// 调用点分派（按成员合并克隆）：调用方法标签@偏移 → 目标方法标签。
    /// 先按成员代表序号（`mbase`）聚合、枢纽目标链逐枢纽记忆，最后才格式化标签
    pub fn dispatch_sites(&self) -> BTreeMap<(String, u32), BTreeSet<String>> {
        let canon: Vec<usize> = self.methods.values().map(|m| self.mbase[&m.key]).collect();
        let mut ids: HashMap<(usize, u32), BTreeSet<usize>> = HashMap::default();
        for ((m, off), ts) in &self.dispatch {
            if self.is_pseudo_method(*m) {
                continue;
            }
            let e = ids.entry((canon[*m], *off)).or_default();
            e.extend(ts.iter().filter(|t| !self.is_pseudo_method(**t)).map(|&t| canon[t]));
        }
        let mut memo: HashMap<u32, Rc<[usize]>> = HashMap::default();
        for ((m, off), hs) in &self.hub_sites {
            let e = ids.entry((canon[*m], *off)).or_default();
            for &h in hs {
                let ts = self.hub_targets_canon(h, &canon, &mut memo);
                e.extend(ts.iter().copied());
            }
        }
        let mut labels: HashMap<usize, Rc<str>> = HashMap::default();
        let mut label = |i: usize| labels.entry(i).or_insert_with(|| self.method_label(i).into()).clone();
        let mut out: BTreeMap<(String, u32), BTreeSet<String>> = BTreeMap::new();
        for ((m, off), ts) in ids {
            let e = out.entry((label(m).to_string(), off)).or_default();
            e.extend(ts.into_iter().map(|t| label(t).to_string()));
        }
        out
    }

    /// 枢纽（含父链）的目标，按成员代表序号去重；逐枢纽记忆（父链迭代展开，不递归）
    fn hub_targets_canon(&self, h: u32, canon: &[usize], memo: &mut HashMap<u32, Rc<[usize]>>) -> Rc<[usize]> {
        let mut chain = Vec::new();
        let mut cur = Some(h);
        let mut base: Rc<[usize]> = Rc::from(Vec::new());
        while let Some(c) = cur {
            if let Some(r) = memo.get(&c) {
                base = r.clone();
                break;
            }
            chain.push(c);
            cur = self.hubs[c as usize].parent;
        }
        for c in chain.into_iter().rev() {
            let mut v: Vec<usize> = self.hubs[c as usize].plain.iter().map(|&t| canon[t]).collect();
            v.extend(base.iter().copied());
            v.sort_unstable();
            v.dedup();
            base = v.into();
            memo.insert(c, base.clone());
        }
        base
    }
}
