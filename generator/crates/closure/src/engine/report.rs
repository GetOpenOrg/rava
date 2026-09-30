//! 引擎：结果查询与诊断输出（folds、实例化集、`--flows`、分派点）。

use super::*;

impl<'a> Engine<'a> {
    // ── 结果查询 ────────────────────────────────────────────────────────────

    /// 折叠点导出（folds v1）：不可达指令区间、不进入的异常处理器、折叠为常量的读取点。
    /// 按方法标签排序；只含有折叠内容的字节码方法
    /// 同一成员的各克隆合并：任一克隆可达即可达，常量须在其可达的全部克隆里一致
    pub fn folds(&self) -> Vec<Fold> {
        let mut groups: IndexMap<&MemberRef, Vec<Option<Rc<Analysis>>>> = IndexMap::new();
        for mn in self.methods.values() {
            if mn.kind == Kind::Bytecode {
                groups.entry(&mn.key).or_default().push(mn.analysis.clone());
            }
        }
        let mut out = Vec::new();
        for (key, group) in groups {
            let Some(all) = group.into_iter().collect::<Option<Vec<_>>>() else { continue };
            if all.iter().any(|a| a.conservative) {
                continue;
            }
            let Some(cf) = self.h.class(&key.owner) else { continue };
            let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { continue };
            let f = fold_of(key.to_string(), code, &all);
            // 自检：活指令顺序落入 dead_pcs（v1 规则禁止），出现即分析缺陷
            if !f.violations.is_empty() {
                eprintln!("[closure] folds 自检违约：{} @{:?}", f.method, f.violations);
            }
            if !f.dead_pcs.is_empty() || !f.dead_handlers.is_empty() || !f.consts.is_empty() {
                out.push(f);
            }
        }
        out.sort_by(|a, b| a.method.cmp(&b.method));
        out
    }

    pub fn instantiated(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .g
            .iter()
            .filter(|i| !self.lambdas.contains_key(i))
            .map(|i| self.names[*i as usize].to_string())
            .filter(|n| !n.starts_with('['))
            .collect();
        v.sort();
        v
    }

    pub fn lambda_count(&self) -> usize {
        self.lambdas.len()
    }

    pub(super) fn set_str(&self, s: &TypeSet) -> String {
        let mut v: Vec<String> = if s.classes.len() > 6 {
            vec![format!("{} 个类", s.classes.len())]
        } else {
            s.classes.iter().map(|i| self.names[*i as usize].to_string()).collect()
        };
        v.extend(s.open.iter().map(|i| format!("open({})", self.names[*i as usize])));
        v.join(", ")
    }

    pub(super) fn hub_label(&self, h: u32) -> String {
        let hub = &self.hubs[h as usize];
        let (o, n, d) = hub.site.key();
        match hub.open {
            Some(x) => format!("{o}.{n}:{d} on open({})", self.names[x as usize]),
            None => format!("{o}.{n}:{d} on {} 个接收者", hub.recvs.len()),
        }
    }

    pub(super) fn node_str(&self, n: Node) -> String {
        match n {
            Node::P(m, i) => format!("P{i} {}", self.ctx_label(m)),
            Node::R(m) => format!("R {}", self.ctx_label(m)),
            Node::S(m, o) if o == POOL => format!("pool {}", self.ctx_label(m)),
            Node::S(m, o) if o == PROD => format!("prod {}", self.ctx_label(m)),
            Node::S(m, o) if o & CATCH != 0 => format!("catch@{} {}", o & !CATCH, self.ctx_label(m)),
            Node::S(m, o) => format!("@{o} {}", self.ctx_label(m)),
            Node::F(f) => format!("field {}", self.field_label(f)),
            Node::U(f) => format!("field? {}", self.field_label(f)),
            Node::O(o, f) => format!("field {} of {}", self.field_label(f), self.names[o as usize]),
            Node::E(x, p) => format!("elements[{}] {}", if p == 0 { "偶" } else { "奇" }, self.names[x as usize]),
            Node::Array => "array".into(),
            Node::Esc => "escape".into(),
            Node::HP(h, i) => format!("hub 实参{i} {}", self.hub_label(h)),
            Node::HR(h) => format!("hub 返回 {}", self.hub_label(h)),
            Node::A(s, i) | Node::W(s, i) => {
                let (m, off, t) = self.hw_sites[s as usize];
                let k = if matches!(n, Node::A(..)) { "实参" } else { "写入" };
                format!("{k}{i} {}@{off} → {}", self.ctx_label(m), self.methods[t].key)
            }
        }
    }

    /// 类型流诊断：方法（标签含 `pat`）的形参 / 返回节点的类型集，以及流入它们的来源节点
    pub fn flows_of(&self, pat: &str) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(q) = pat.strip_prefix("elem:") {
            let mut es: Vec<Node> = self.sets.keys().filter(|n| matches!(n, Node::E(x, _) if self.names[*x as usize].contains(q))).copied().collect();
            es.sort_by_key(|n| format!("{n:?}"));
            for n in es {
                let s = self.sets.get(&n).cloned().unwrap_or_default();
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(&s)));
                for (src, edges) in &self.flows {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.sets.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.names[*f as usize], self.set_str(&s)));
                        }
                    }
                }
            }
            return out;
        }
        // 污染路径诊断：`@path:<节点子串>|<类名>`——从匹配节点沿流边反向，经含该类的节点走到源头（最短路径）
        if let Some((np, cls)) = pat.strip_prefix("@path:").and_then(|v| v.split_once('|')) {
            let (open, cls) = match cls.strip_prefix("open:") {
                Some(c) => (true, c),
                None => (false, cls),
            };
            let Some(&cid) = self.ids.get(cls) else { return vec![format!("无此类：{cls}")] };
            let has = |x: &Node| self.sets.get(x).is_some_and(|s| if open { s.open.contains(&cid) } else { s.classes.contains(&cid) });
            let mut rev: HashMap<Node, Vec<Node>> = HashMap::default();
            for (src, edges) in &self.flows {
                for (dst, _) in edges {
                    rev.entry(*dst).or_default().push(*src);
                }
            }
            let starts: Vec<Node> = self.sets.keys().filter(|n| has(n) && self.node_str(**n).contains(np)).copied().collect();
            let mut prev: HashMap<Node, Option<Node>> = HashMap::default();
            let mut q: VecDeque<Node> = VecDeque::new();
            for n in starts.into_iter().take(1) {
                prev.insert(n, None);
                q.push_back(n);
            }
            let mut last = None;
            while let Some(n) = q.pop_front() {
                last = Some(n);
                let mut ps: Vec<Node> = rev.get(&n).map(|v| v.iter().filter(|p| has(p) && !prev.contains_key(*p)).copied().collect()).unwrap_or_default();
                ps.sort_by_key(|p| format!("{p:?}"));
                for p in ps {
                    prev.insert(p, Some(n));
                    q.push_back(p);
                }
            }
            // 最远的源头往回打印到起点
            let mut cur = last;
            while let Some(n) = cur {
                let sz = self.sets.get(&n).map_or(0, |s| s.classes.len());
                out.push(format!("  {} (|{sz}|)", self.node_str(n)));
                cur = prev.get(&n).copied().flatten();
            }
            return out;
        }
        // open 统计诊断：`@openstat`——各 open 类型：含它的节点数、引入点数、按 G 展开的类数
        if pat == "@openstat" {
            let mut fed: HashSet<(Node, u32)> = HashSet::default();
            for (src, edges) in &self.flows {
                if let Some(ss) = self.sets.get(src) {
                    for &o in &ss.open {
                        for (dst, _) in edges {
                            fed.insert((*dst, o));
                        }
                    }
                }
            }
            let mut stat: HashMap<u32, (usize, Vec<String>)> = HashMap::default();
            for (n, ss) in &self.sets {
                for &o in &ss.open {
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
        if let Some(q) = pat.strip_prefix("@opens:") {
            let Some(&cid) = self.ids.get(q) else { return vec![format!("无此类：{q}")] };
            let has = |x: &Node| self.sets.get(x).is_some_and(|s| s.open.contains(&cid));
            let mut fed: HashSet<Node> = HashSet::default();
            for (src, edges) in &self.flows {
                if has(src) {
                    for (dst, _) in edges {
                        fed.insert(*dst);
                    }
                }
            }
            let mut v: Vec<String> = self.sets.keys().filter(|n| has(n) && !fed.contains(*n)).map(|n| format!("  {}", self.node_str(*n))).collect();
            v.sort();
            return v;
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
            let size = |x: &Node| self.sets.get(x).map_or(0, |s| s.classes.len());
            let mut inc: HashMap<Node, (IdSet, usize)> = HashMap::default();
            for (src, edges) in &self.flows {
                if size(src) >= n {
                    continue;
                }
                for (dst, _) in edges {
                    if size(dst) >= n {
                        let e = inc.entry(*dst).or_default();
                        e.1 += 1;
                        if let Some(s) = self.sets.get(src) {
                            for c in s.classes.iter() {
                                e.0.insert(*c);
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
        if pat == "@array" {
            let s = self.sets.get(&Node::Array).cloned().unwrap_or_default();
            out.push(format!("  array = {{{}}}", self.set_str(&s)));
            for (src, edges) in &self.flows {
                if edges.iter().any(|(d, _)| *d == Node::Array) {
                    let s = self.sets.get(src).cloned().unwrap_or_default();
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
                .sets
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
                let Some(s) = self.sets.get(&n) else { continue };
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(s)));
                for (src, edges) in &self.flows {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.sets.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.names[*f as usize], self.set_str(&s)));
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

    /// 带克隆上下文的方法标签（诊断用）
    pub(super) fn ctx_label(&self, i: usize) -> String {
        match self.methods[i].ctx {
            NOCTX => self.method_label(i),
            c => format!("{} #{}", self.method_label(i), self.names[c as usize]),
        }
    }

    pub(super) fn field_label(&self, f: usize) -> String {
        self.fields.get_index(f).map(|(k, _)| k.to_string()).unwrap_or_default()
    }

    /// 方法节点按成员去重（克隆只是分析内部的上下文区分；输出按成员）
    pub fn method_nodes(&self) -> impl Iterator<Item = &MNode> {
        self.methods.values().enumerate().filter(|(i, m)| self.mbase[&m.key] == *i).map(|(_, m)| m)
    }

    pub fn method_count(&self) -> usize {
        self.mbase.len()
    }

    /// 调用点分派（按成员合并克隆）：调用方法标签@偏移 → 目标方法标签
    pub fn dispatch_sites(&self) -> BTreeMap<(String, u32), BTreeSet<String>> {
        let mut out: BTreeMap<(String, u32), BTreeSet<String>> = BTreeMap::new();
        for ((m, off), ts) in &self.dispatch {
            out.entry((self.method_label(*m), *off)).or_default().extend(ts.iter().map(|t| self.method_label(*t)));
        }
        for ((m, off), hs) in &self.hub_sites {
            let e = out.entry((self.method_label(*m), *off)).or_default();
            for &h in hs {
                e.extend(self.hub_targets(h).into_iter().map(|t| self.method_label(t)));
            }
        }
        out
    }
}
